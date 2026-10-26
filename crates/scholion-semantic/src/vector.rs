use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use qdrant_client::qdrant::r#match::MatchValue;
use qdrant_client::qdrant::value::Kind;
use qdrant_client::qdrant::{
    Condition, CreateCollectionBuilder, DeletePointsBuilder, Distance, FieldCondition, Filter,
    Match, PointId, PointStruct, SearchPointsBuilder, SetPayloadPointsBuilder, UpsertPointsBuilder,
    Value, VectorParamsBuilder, vectors_config,
};
use qdrant_client::{Payload, Qdrant};
use scholion_core::{
    AvError, ChangeId, ConventionId, RecordId, Result, SemanticHit, SemanticKind, SymbolId, TaskId,
};
use tokio::sync::OnceCell;
use uuid::Uuid;

pub const COLLECTION_RECORDS: &str = "records";
pub const COLLECTION_CONVENTIONS: &str = "conventions";
pub const COLLECTION_CODE_CHUNKS: &str = "code_chunks";

const COLLECTIONS: [&str; 3] = [
    COLLECTION_RECORDS,
    COLLECTION_CONVENTIONS,
    COLLECTION_CODE_CHUNKS,
];

const KINDS: [SemanticKind; 3] = [
    SemanticKind::Record,
    SemanticKind::Convention,
    SemanticKind::CodeChunk,
];

const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct UpsertPoint {
    pub entity_id: String,
    pub vector: Vec<f32>,
    pub payload: SemanticHit,
}

#[async_trait]
pub trait VectorIndex: Send + Sync {
    async fn ensure(&self, dim: usize) -> Result<()>;
    async fn upsert(&self, kind: SemanticKind, points: &[UpsertPoint]) -> Result<()>;
    /// Sets `superseded` on one point. A missing point is `Ok`.
    async fn set_superseded(&self, kind: SemanticKind, entity_id: &str) -> Result<()>;
    /// Deletes points. A missing point is `Ok`.
    async fn delete_points(&self, kind: SemanticKind, entity_ids: &[String]) -> Result<()>;
    async fn search(
        &self,
        kind: SemanticKind,
        repo: &str,
        vector: &[f32],
        limit: usize,
    ) -> Result<Vec<SemanticHit>>;
    async fn delete_repo(&self, repo: &str) -> Result<()>;
    /// Drops and recreates all three collections at `dim`. Full reindex only.
    async fn recreate(&self, dim: usize) -> Result<()>;
}

/// Lets shared trait objects (`Arc<dyn VectorIndex>`) be used wherever a `VectorIndex` is expected.
#[async_trait]
impl<T: VectorIndex + ?Sized> VectorIndex for Arc<T> {
    async fn ensure(&self, dim: usize) -> Result<()> {
        (**self).ensure(dim).await
    }
    async fn upsert(&self, kind: SemanticKind, points: &[UpsertPoint]) -> Result<()> {
        (**self).upsert(kind, points).await
    }
    async fn set_superseded(&self, kind: SemanticKind, entity_id: &str) -> Result<()> {
        (**self).set_superseded(kind, entity_id).await
    }
    async fn delete_points(&self, kind: SemanticKind, entity_ids: &[String]) -> Result<()> {
        (**self).delete_points(kind, entity_ids).await
    }
    async fn search(
        &self,
        kind: SemanticKind,
        repo: &str,
        vector: &[f32],
        limit: usize,
    ) -> Result<Vec<SemanticHit>> {
        (**self).search(kind, repo, vector, limit).await
    }
    async fn delete_repo(&self, repo: &str) -> Result<()> {
        (**self).delete_repo(repo).await
    }
    async fn recreate(&self, dim: usize) -> Result<()> {
        (**self).recreate(dim).await
    }
}

/// In-memory cosine index. Clones share the point map.
#[derive(Clone)]
pub struct MemoryIndex {
    inner: Arc<Mutex<MemoryState>>,
}

#[derive(Default)]
struct MemoryState {
    dims: HashMap<SemanticKind, usize>,
    points: HashMap<SemanticKind, HashMap<String, StoredPoint>>,
}

struct StoredPoint {
    vector: Vec<f32>,
    payload: SemanticHit,
}

impl Default for MemoryIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryIndex {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(MemoryState::default())),
        }
    }

    fn lock(&self) -> Result<std::sync::MutexGuard<'_, MemoryState>> {
        self.inner
            .lock()
            .map_err(|_| AvError::Semantic("vector index lock poisoned".into()))
    }
}

#[async_trait]
impl VectorIndex for MemoryIndex {
    async fn ensure(&self, dim: usize) -> Result<()> {
        let mut state = self.lock()?;
        for kind in KINDS {
            if let Some(stored) = state.dims.get(&kind).copied()
                && stored != dim
            {
                return Err(dim_mismatch(collection_name(kind), stored, dim));
            }
        }
        for kind in KINDS {
            state.dims.entry(kind).or_insert(dim);
            state.points.entry(kind).or_default();
        }
        Ok(())
    }

    async fn upsert(&self, kind: SemanticKind, points: &[UpsertPoint]) -> Result<()> {
        if points.is_empty() {
            return Ok(());
        }
        let mut state = self.lock()?;
        let dim = stored_dim(&state, kind)?;
        for point in points {
            if point.vector.len() != dim {
                return Err(AvError::Semantic(format!(
                    "point {} vector length is {}, collection {} dimension is {dim}",
                    point.entity_id,
                    point.vector.len(),
                    collection_name(kind)
                )));
            }
        }
        let map = state.points.entry(kind).or_default();
        for point in points {
            map.insert(
                point.entity_id.clone(),
                StoredPoint {
                    vector: point.vector.clone(),
                    payload: point.payload.clone(),
                },
            );
        }
        Ok(())
    }

    async fn set_superseded(&self, kind: SemanticKind, entity_id: &str) -> Result<()> {
        let mut state = self.lock()?;
        let Some(map) = state.points.get_mut(&kind) else {
            return Ok(());
        };
        if let Some(point) = map.get_mut(entity_id) {
            point.payload.superseded = true;
        }
        Ok(())
    }

    async fn delete_points(&self, kind: SemanticKind, entity_ids: &[String]) -> Result<()> {
        let mut state = self.lock()?;
        let Some(map) = state.points.get_mut(&kind) else {
            return Ok(());
        };
        for entity_id in entity_ids {
            map.remove(entity_id);
        }
        Ok(())
    }

    async fn search(
        &self,
        kind: SemanticKind,
        repo: &str,
        vector: &[f32],
        limit: usize,
    ) -> Result<Vec<SemanticHit>> {
        let state = self.lock()?;
        let dim = stored_dim(&state, kind)?;
        if vector.len() != dim {
            return Err(AvError::Semantic(format!(
                "query vector length is {}, collection {} dimension is {dim}",
                vector.len(),
                collection_name(kind)
            )));
        }
        if limit == 0 {
            return Ok(Vec::new());
        }
        let Some(map) = state.points.get(&kind) else {
            return Ok(Vec::new());
        };
        let mut hits: Vec<(String, SemanticHit)> = map
            .iter()
            .filter(|(_, point)| point.payload.repo == repo && !point.payload.superseded)
            .map(|(entity_id, point)| {
                let mut hit = point.payload.clone();
                hit.kind = kind;
                hit.score = cosine(&point.vector, vector);
                (entity_id.clone(), hit)
            })
            .collect();
        hits.sort_by(|left, right| {
            right
                .1
                .score
                .total_cmp(&left.1.score)
                .then_with(|| left.0.cmp(&right.0))
        });
        hits.truncate(limit);
        Ok(hits.into_iter().map(|(_, hit)| hit).collect())
    }

    async fn delete_repo(&self, repo: &str) -> Result<()> {
        let mut state = self.lock()?;
        for map in state.points.values_mut() {
            map.retain(|_, point| point.payload.repo != repo);
        }
        Ok(())
    }

    async fn recreate(&self, dim: usize) -> Result<()> {
        let mut state = self.lock()?;
        state.dims.clear();
        state.points.clear();
        for kind in KINDS {
            state.dims.insert(kind, dim);
            state.points.insert(kind, HashMap::new());
        }
        Ok(())
    }
}

/// gRPC Qdrant client. `new` stores the URL and connects on the first call.
#[derive(Clone)]
pub struct QdrantIndex {
    url: Arc<String>,
    client: Arc<OnceCell<Qdrant>>,
}

impl QdrantIndex {
    pub fn new(grpc_url: impl Into<String>) -> Self {
        Self {
            url: Arc::new(grpc_url.into()),
            client: Arc::new(OnceCell::new()),
        }
    }

    async fn client(&self) -> Result<&Qdrant> {
        self.client
            .get_or_try_init(|| async {
                // The default compatibility check dials inside `build`.
                Qdrant::from_url(&self.url)
                    .timeout(CLIENT_TIMEOUT)
                    .skip_compatibility_check()
                    .build()
                    .map_err(qerr)
            })
            .await
    }

    async fn collection_dim(&self, name: &str) -> Result<Option<usize>> {
        let client = self.client().await?;
        if !client.collection_exists(name).await.map_err(qerr)? {
            return Ok(None);
        }
        let info = client.collection_info(name).await.map_err(qerr)?;
        let size = info
            .result
            .as_ref()
            .and_then(|info| info.config.as_ref())
            .and_then(|config| config.params.as_ref())
            .and_then(|params| params.vectors_config.as_ref())
            .and_then(vector_size)
            .ok_or_else(|| {
                AvError::Semantic(format!("collection {name} has no vector dimension"))
            })?;
        usize::try_from(size).map(Some).map_err(|_| {
            AvError::Semantic(format!("collection {name} dimension {size} does not fit"))
        })
    }

    async fn create_collection(&self, name: &str, dim: usize) -> Result<()> {
        let size = u64::try_from(dim).map_err(|_| {
            AvError::Semantic(format!(
                "collection {name} dimension {dim} does not fit in u64"
            ))
        })?;
        let client = self.client().await?;
        match client
            .create_collection(
                CreateCollectionBuilder::new(name)
                    .vectors_config(VectorParamsBuilder::new(size, Distance::Cosine)),
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(err) if is_already_exists(&err) => self.require_dim(name, dim).await,
            Err(err) => Err(qerr(err)),
        }
    }

    async fn require_dim(&self, name: &str, dim: usize) -> Result<()> {
        match self.collection_dim(name).await? {
            Some(stored) if stored == dim => Ok(()),
            Some(stored) => Err(dim_mismatch(name, stored, dim)),
            None => Err(AvError::Semantic(format!(
                "collection {name} disappeared while ensuring dimension {dim}"
            ))),
        }
    }

    async fn collection_present(&self, name: &str) -> Result<bool> {
        Ok(self.collection_dim(name).await?.is_some())
    }
}

#[async_trait]
impl VectorIndex for QdrantIndex {
    async fn ensure(&self, dim: usize) -> Result<()> {
        let mut missing = Vec::new();
        for name in COLLECTIONS {
            match self.collection_dim(name).await? {
                Some(stored) if stored != dim => return Err(dim_mismatch(name, stored, dim)),
                Some(_) => {}
                None => missing.push(name),
            }
        }
        for name in missing {
            self.create_collection(name, dim).await?;
        }
        Ok(())
    }

    async fn upsert(&self, kind: SemanticKind, points: &[UpsertPoint]) -> Result<()> {
        if points.is_empty() {
            return Ok(());
        }
        let name = collection_name(kind);
        let Some(dim) = self.collection_dim(name).await? else {
            return Err(not_ensured(name));
        };
        let mut batch = Vec::with_capacity(points.len());
        for point in points {
            if point.vector.len() != dim {
                return Err(AvError::Semantic(format!(
                    "point {} vector length is {}, collection {name} dimension is {dim}",
                    point.entity_id,
                    point.vector.len()
                )));
            }
            batch.push(PointStruct::new(
                point_id(kind, &point.entity_id)?,
                point.vector.clone(),
                payload_from_hit(&point.payload),
            ));
        }
        let client = self.client().await?;
        client
            .upsert_points(UpsertPointsBuilder::new(name, batch).wait(true))
            .await
            .map_err(qerr)?;
        Ok(())
    }

    async fn set_superseded(&self, kind: SemanticKind, entity_id: &str) -> Result<()> {
        let name = collection_name(kind);
        if !self.collection_present(name).await? {
            return Ok(());
        }
        let Some(id) = try_point_id(kind, entity_id) else {
            return Ok(());
        };
        let mut payload = Payload::new();
        payload.insert("superseded", true);
        let client = self.client().await?;
        match client
            .set_payload(
                SetPayloadPointsBuilder::new(name, payload)
                    .points_selector(vec![id])
                    .wait(true),
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(err) if is_not_found(&err) => Ok(()),
            Err(err) => Err(qerr(err)),
        }
    }

    async fn delete_points(&self, kind: SemanticKind, entity_ids: &[String]) -> Result<()> {
        if entity_ids.is_empty() {
            return Ok(());
        }
        let name = collection_name(kind);
        if !self.collection_present(name).await? {
            return Ok(());
        }
        let ids: Vec<PointId> = entity_ids
            .iter()
            .filter_map(|entity_id| try_point_id(kind, entity_id))
            .collect();
        if ids.is_empty() {
            return Ok(());
        }
        let client = self.client().await?;
        match client
            .delete_points(DeletePointsBuilder::new(name).points(ids).wait(true))
            .await
        {
            Ok(_) => Ok(()),
            Err(err) if is_not_found(&err) => Ok(()),
            Err(err) => Err(qerr(err)),
        }
    }

    async fn search(
        &self,
        kind: SemanticKind,
        repo: &str,
        vector: &[f32],
        limit: usize,
    ) -> Result<Vec<SemanticHit>> {
        if limit == 0 {
            return Ok(Vec::new());
        }
        let name = collection_name(kind);
        let Some(dim) = self.collection_dim(name).await? else {
            return Err(not_ensured(name));
        };
        if vector.len() != dim {
            return Err(AvError::Semantic(format!(
                "query vector length is {}, collection {name} dimension is {dim}",
                vector.len()
            )));
        }
        let limit = u64::try_from(limit)
            .map_err(|_| AvError::Semantic(format!("search limit {limit} does not fit in u64")))?;
        let client = self.client().await?;
        let response = client
            .search_points(
                SearchPointsBuilder::new(name, vector.to_vec(), limit)
                    .filter(repo_filter(repo))
                    .with_payload(true),
            )
            .await
            .map_err(qerr)?;
        let mut hits = Vec::with_capacity(response.result.len());
        for point in response.result {
            let hit = hit_from_payload(kind, point.score, &point.payload)?;
            if hit.repo == repo && !hit.superseded {
                hits.push(hit);
            }
        }
        Ok(hits)
    }

    async fn delete_repo(&self, repo: &str) -> Result<()> {
        let client = self.client().await?;
        let filter = Filter::must([repo_keyword(repo)]);
        for name in COLLECTIONS {
            if !client.collection_exists(name).await.map_err(qerr)? {
                continue;
            }
            match client
                .delete_points(
                    DeletePointsBuilder::new(name)
                        .points(filter.clone())
                        .wait(true),
                )
                .await
            {
                Ok(_) => {}
                Err(err) if is_not_found(&err) => {}
                Err(err) => return Err(qerr(err)),
            }
        }
        Ok(())
    }

    async fn recreate(&self, dim: usize) -> Result<()> {
        let client = self.client().await?;
        for name in COLLECTIONS {
            if !client.collection_exists(name).await.map_err(qerr)? {
                continue;
            }
            client.delete_collection(name).await.map_err(qerr)?;
        }
        for name in COLLECTIONS {
            self.create_collection(name, dim).await?;
        }
        Ok(())
    }
}

fn collection_name(kind: SemanticKind) -> &'static str {
    match kind {
        SemanticKind::Record => COLLECTION_RECORDS,
        SemanticKind::Convention => COLLECTION_CONVENTIONS,
        SemanticKind::CodeChunk => COLLECTION_CODE_CHUNKS,
    }
}

fn stored_dim(state: &MemoryState, kind: SemanticKind) -> Result<usize> {
    state
        .dims
        .get(&kind)
        .copied()
        .ok_or_else(|| not_ensured(collection_name(kind)))
}

fn not_ensured(name: &str) -> AvError {
    AvError::Semantic(format!("collection {name} is not ensured"))
}

fn dim_mismatch(name: &str, stored: usize, requested: usize) -> AvError {
    AvError::Semantic(format!(
        "collection {name} has dimension {stored}, not {requested}"
    ))
}

fn cosine(left: &[f32], right: &[f32]) -> f32 {
    let mut dot = 0.0f32;
    let mut left_norm = 0.0f32;
    let mut right_norm = 0.0f32;
    for (x, y) in left.iter().zip(right) {
        dot += x * y;
        left_norm += x * x;
        right_norm += y * y;
    }
    let denom = left_norm.sqrt() * right_norm.sqrt();
    if denom.to_bits() == 0 {
        0.0
    } else {
        dot / denom
    }
}

fn qerr(err: qdrant_client::QdrantError) -> AvError {
    AvError::Semantic(format!("qdrant: {err}"))
}

fn error_text(err: &qdrant_client::QdrantError) -> String {
    err.to_string().to_ascii_lowercase()
}

fn is_already_exists(err: &qdrant_client::QdrantError) -> bool {
    let text = error_text(err);
    text.contains("already exists") || text.contains("alreadyexists")
}

fn is_not_found(err: &qdrant_client::QdrantError) -> bool {
    let text = error_text(err);
    text.contains("not found")
        || text.contains("notfound")
        || text.contains("does not exist")
        || text.contains("doesn't exist")
}

fn point_id(kind: SemanticKind, entity_id: &str) -> Result<PointId> {
    match kind {
        SemanticKind::Record => {
            Uuid::parse_str(entity_id).map_err(|_| {
                AvError::Semantic(format!("record point id {entity_id} is not a UUID"))
            })?;
            Ok(PointId::from(entity_id))
        }
        SemanticKind::Convention | SemanticKind::CodeChunk => {
            Ok(PointId::from(entity_u64(kind, entity_id)?))
        }
    }
}

fn try_point_id(kind: SemanticKind, entity_id: &str) -> Option<PointId> {
    point_id(kind, entity_id).ok()
}

fn entity_u64(kind: SemanticKind, entity_id: &str) -> Result<u64> {
    if let Ok(id) = entity_id.parse::<u64>() {
        return Ok(id);
    }
    if let Ok(id) = entity_id.parse::<i64>() {
        return Ok(id.cast_unsigned());
    }
    Err(AvError::Semantic(format!(
        "{} point id {entity_id} is not an integer",
        collection_name(kind)
    )))
}

fn vector_size(vectors: &qdrant_client::qdrant::VectorsConfig) -> Option<u64> {
    match &vectors.config {
        Some(vectors_config::Config::Params(params)) => Some(params.size),
        Some(vectors_config::Config::ParamsMap(map)) => {
            map.map.values().next().map(|params| params.size)
        }
        None => None,
    }
}

fn null_value() -> Value {
    Value {
        kind: Some(Kind::NullValue(0)),
    }
}

fn insert_text(payload: &mut Payload, key: &str, value: Option<&str>) {
    match value {
        Some(text) => payload.insert(key, text),
        None => payload.insert(key, null_value()),
    }
}

fn insert_int(payload: &mut Payload, key: &str, value: Option<i64>) {
    match value {
        Some(value) => payload.insert(key, value),
        None => payload.insert(key, null_value()),
    }
}

/// Field-by-field payload. `score` and `kind` are not stored. Absent fields are JSON null.
fn payload_from_hit(hit: &SemanticHit) -> Payload {
    let mut payload = Payload::new();
    payload.insert("repo", hit.repo.as_str());
    insert_text(
        &mut payload,
        "task_id",
        hit.task_id.map(|id| id.to_string()).as_deref(),
    );
    insert_text(
        &mut payload,
        "record_id",
        hit.record_id.map(|id| id.to_string()).as_deref(),
    );
    insert_int(
        &mut payload,
        "symbol_id",
        hit.symbol_id.map(|SymbolId(id)| id),
    );
    insert_int(
        &mut payload,
        "convention_id",
        hit.convention_id.map(|ConventionId(id)| id),
    );
    insert_text(&mut payload, "path", hit.path.as_deref());
    insert_text(
        &mut payload,
        "change_id",
        hit.change_id.as_ref().map(|ChangeId(id)| id.as_str()),
    );
    payload.insert("superseded", hit.superseded);
    payload
}

fn hit_from_payload(
    kind: SemanticKind,
    score: f32,
    payload: &HashMap<String, Value>,
) -> Result<SemanticHit> {
    Ok(SemanticHit {
        kind,
        score,
        repo: required_text(payload, "repo")?,
        task_id: optional_text(payload, "task_id")?
            .map(|text| parse_uuid("task_id", &text).map(TaskId))
            .transpose()?,
        record_id: optional_text(payload, "record_id")?
            .map(|text| parse_uuid("record_id", &text).map(RecordId))
            .transpose()?,
        symbol_id: optional_int(payload, "symbol_id")?.map(SymbolId),
        convention_id: optional_int(payload, "convention_id")?.map(ConventionId),
        path: optional_text(payload, "path")?,
        change_id: optional_text(payload, "change_id")?.map(ChangeId),
        superseded: required_bool(payload, "superseded")?,
    })
}

fn payload_value<'a>(payload: &'a HashMap<String, Value>, key: &str) -> Result<&'a Value> {
    payload
        .get(key)
        .ok_or_else(|| AvError::Semantic(format!("payload missing {key}")))
}

fn optional_text(payload: &HashMap<String, Value>, key: &str) -> Result<Option<String>> {
    let value = payload_value(payload, key)?;
    if value.is_null() {
        return Ok(None);
    }
    value
        .as_str()
        .cloned()
        .map(Some)
        .ok_or_else(|| AvError::Semantic(format!("payload {key} is not a string")))
}

fn required_text(payload: &HashMap<String, Value>, key: &str) -> Result<String> {
    optional_text(payload, key)?.ok_or_else(|| AvError::Semantic(format!("payload {key} is null")))
}

fn optional_int(payload: &HashMap<String, Value>, key: &str) -> Result<Option<i64>> {
    let value = payload_value(payload, key)?;
    if value.is_null() {
        return Ok(None);
    }
    value
        .as_integer()
        .map(Some)
        .ok_or_else(|| AvError::Semantic(format!("payload {key} is not an integer")))
}

fn required_bool(payload: &HashMap<String, Value>, key: &str) -> Result<bool> {
    let value = payload_value(payload, key)?;
    value
        .as_bool()
        .ok_or_else(|| AvError::Semantic(format!("payload {key} is not a bool")))
}

fn parse_uuid(key: &str, text: &str) -> Result<Uuid> {
    Uuid::parse_str(text).map_err(|err| AvError::Semantic(format!("payload {key}: {err}")))
}

fn repo_keyword(repo: &str) -> Condition {
    Condition::from(FieldCondition {
        key: "repo".into(),
        r#match: Some(Match {
            match_value: Some(MatchValue::Keyword(repo.to_string())),
        }),
        ..Default::default()
    })
}

fn repo_filter(repo: &str) -> Filter {
    Filter::must([repo_keyword(repo), Condition::matches("superseded", false)])
}
