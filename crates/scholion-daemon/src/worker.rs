//! Outbox worker: drains Postgres `outbox` rows into the vector index (Plan 2 Task 6).

use std::sync::Arc;

use scholion_core::{AvError, IntentStore, OutboxOp, OutboxRow, Result, SemanticHit, SemanticKind};
use scholion_index::CodeIndex;
use scholion_semantic::{Embedder, UpsertPoint, VectorIndex};
use scholion_store::PgStore;

/// Rows claimed per `drain_once`.
const BATCH: i64 = 32;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DrainStats {
    pub done: usize,
    pub failed: usize,
}

pub struct OutboxWorker {
    store: Arc<PgStore>,
    index: Arc<dyn CodeIndex>,
    embedder: Arc<dyn Embedder>,
    vectors: Arc<dyn VectorIndex>,
}

impl OutboxWorker {
    pub fn new(
        store: Arc<PgStore>,
        index: Arc<dyn CodeIndex>,
        embedder: Arc<dyn Embedder>,
        vectors: Arc<dyn VectorIndex>,
    ) -> Self {
        Self {
            store,
            index,
            embedder,
            vectors,
        }
    }

    /// Claims up to 32 rows and applies them in ascending id. A run of consecutive upserts is
    /// embedded in one call; if that call fails, each of its rows is retried alone so one bad
    /// row cannot hold back the others. A row's error is recorded on that row (it is retried
    /// later with backoff) and does not stop the batch.
    pub async fn drain_once(&self) -> Result<DrainStats> {
        let rows = self.store.claim_outbox(BATCH).await?;
        let mut stats = DrainStats::default();
        let mut ensured = false;
        let mut run: Vec<OutboxRow> = vec![];
        for row in rows {
            if row.row.op == OutboxOp::Upsert {
                run.push(row);
                continue;
            }
            self.flush_upserts(std::mem::take(&mut run), &mut ensured, &mut stats)
                .await?;
            let applied = self.apply(&row).await;
            self.finish(row.id, applied, &mut stats).await?;
        }
        self.flush_upserts(run, &mut ensured, &mut stats).await?;
        Ok(stats)
    }

    async fn finish(&self, id: i64, applied: Result<()>, stats: &mut DrainStats) -> Result<()> {
        match applied {
            Ok(()) => {
                self.store.complete_outbox(id).await?;
                stats.done += 1;
            }
            Err(e) => {
                self.store.fail_outbox(id, &e.to_string()).await?;
                stats.failed += 1;
            }
        }
        Ok(())
    }

    async fn flush_upserts(
        &self,
        run: Vec<OutboxRow>,
        ensured: &mut bool,
        stats: &mut DrainStats,
    ) -> Result<()> {
        if run.is_empty() {
            return Ok(());
        }
        if run.len() > 1 && self.upsert_all(&run, ensured).await.is_ok() {
            for r in &run {
                self.finish(r.id, Ok(()), stats).await?;
            }
            return Ok(());
        }
        // a single row, or the batch failed: apply each row alone to find the one(s) at fault
        for r in &run {
            let applied = self.upsert_all(std::slice::from_ref(r), ensured).await;
            self.finish(r.id, applied, stats).await?;
        }
        Ok(())
    }

    /// Delete and supersede rows; upserts go through `upsert_all`.
    async fn apply(&self, r: &OutboxRow) -> Result<()> {
        let row = &r.row;
        match row.op {
            // A missing point is success for both.
            OutboxOp::Delete => {
                self.vectors
                    .delete_points(row.kind, std::slice::from_ref(&row.entity_id))
                    .await
            }
            OutboxOp::Supersede => self.vectors.set_superseded(row.kind, &row.entity_id).await,
            OutboxOp::Upsert => self.upsert_all(std::slice::from_ref(r), &mut false).await,
        }
    }

    /// Embeds the upserts whose entity still exists in one call, then writes the points.
    async fn upsert_all(&self, rows: &[OutboxRow], ensured: &mut bool) -> Result<()> {
        let mut live = vec![];
        for r in rows {
            if let Some(superseded) = self.live_state(r).await? {
                live.push((r, superseded)); // a gone entity: drop the row, embed nothing
            }
        }
        if live.is_empty() {
            return Ok(());
        }
        let texts: Vec<String> = live
            .iter()
            .map(|(r, _)| r.row.text.clone().unwrap_or_default())
            .collect();
        let vectors = self.embedder.embed(&texts).await?;
        if vectors.len() != texts.len() {
            return Err(AvError::Semantic(format!(
                "embedder returned {} vectors for {} texts",
                vectors.len(),
                texts.len()
            )));
        }
        let dim = self.embedder.dimension().await?;
        if let Some(v) = vectors.iter().find(|v| v.len() != dim) {
            return Err(AvError::Semantic(format!(
                "vector length {} is not the embedder dimension {dim}",
                v.len()
            )));
        }
        if !*ensured {
            self.vectors.ensure(dim).await?;
            *ensured = true;
        }
        // one upsert per collection, points in row order
        let mut by_kind: Vec<(SemanticKind, Vec<UpsertPoint>)> = vec![];
        for ((r, superseded), vector) in live.into_iter().zip(vectors) {
            let row = &r.row;
            let point = UpsertPoint {
                entity_id: row.entity_id.clone(),
                vector,
                payload: SemanticHit {
                    kind: row.kind,
                    score: 0.0,
                    repo: row.repo.clone(),
                    task_id: row.task_id,
                    record_id: row.record_id,
                    symbol_id: row.symbol_id,
                    convention_id: row.convention_id,
                    path: row.path.clone(),
                    change_id: row.change_id.clone(),
                    superseded,
                },
            };
            match by_kind.iter_mut().find(|(k, _)| *k == row.kind) {
                Some((_, points)) => points.push(point),
                None => by_kind.push((row.kind, vec![point])),
            }
        }
        for (kind, points) in by_kind {
            self.vectors.upsert(kind, &points).await?;
        }
        Ok(())
    }

    /// `None` when the entity no longer exists. Otherwise its current `superseded` flag, read
    /// from Postgres (never from the outbox row, which may be a stale redelivery).
    async fn live_state(&self, r: &OutboxRow) -> Result<Option<bool>> {
        let row = &r.row;
        match row.kind {
            SemanticKind::Record => {
                let Some(id) = row.record_id else {
                    return Ok(None);
                };
                match self.store.get_record(id).await {
                    Ok(rec) => Ok(Some(rec.superseded_by.is_some())),
                    Err(AvError::NotFound(_)) => Ok(None),
                    Err(e) => Err(e),
                }
            }
            SemanticKind::CodeChunk => match row.symbol_id {
                Some(id) if self.index.chunk_exists(id).await? => Ok(Some(false)),
                _ => Ok(None),
            },
            SemanticKind::Convention => {
                let Some(id) = row.convention_id else {
                    return Ok(None);
                };
                let all = self.store.list_all_conventions(Some(&row.repo)).await?;
                Ok(all.iter().any(|c| c.id == id).then_some(false))
            }
        }
    }
}
