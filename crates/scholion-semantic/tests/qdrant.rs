use std::sync::atomic::Ordering;

use scholion_core::{
    AvError, ChangeId, ConventionId, RecordId, Semantic, SemanticHit, SemanticKind, SymbolId,
    TaskId,
};
use scholion_semantic::{
    COLLECTION_CODE_CHUNKS, COLLECTION_CONVENTIONS, COLLECTION_RECORDS, Embedder, MemoryIndex,
    QdrantIndex, ScriptedEmbedder, SemanticSearch, UpsertPoint, VectorIndex,
};
use uuid::Uuid;

fn record_hit(repo: &str, record: RecordId, superseded: bool, score: f32) -> SemanticHit {
    SemanticHit {
        kind: SemanticKind::Record,
        score,
        repo: repo.to_string(),
        task_id: Some(TaskId(Uuid::from_u128(1))),
        record_id: Some(record),
        symbol_id: None,
        convention_id: None,
        path: Some("a.py".into()),
        change_id: Some(ChangeId("abc".into())),
        superseded,
    }
}

fn cosine(left: &[f32], right: &[f32]) -> f32 {
    let mut dot = 0.0;
    let mut left_norm = 0.0;
    let mut right_norm = 0.0;
    for (x, y) in left.iter().zip(right) {
        dot += x * y;
        left_norm += x * x;
        right_norm += y * y;
    }
    let denom = left_norm.sqrt() * right_norm.sqrt();
    if denom == 0.0 { 0.0 } else { dot / denom }
}

#[tokio::test]
async fn search_filters_repo_and_superseded() {
    let embedder = ScriptedEmbedder::new(4);
    let query = "live record in repo a";
    let query_vector = embedder
        .embed(&[query.to_string()])
        .await
        .unwrap()
        .pop()
        .unwrap();
    let calls_after_setup = embedder.calls.load(Ordering::SeqCst);
    let stored = vec![1.0, 0.0, 0.0, 0.0];

    let index = MemoryIndex::new();
    let shared = index.clone();
    index.ensure(4).await.unwrap();
    let live = RecordId(Uuid::from_u128(10));
    let dead = RecordId(Uuid::from_u128(11));
    let other = RecordId(Uuid::from_u128(12));
    index
        .upsert(
            SemanticKind::Record,
            &[
                UpsertPoint {
                    entity_id: live.to_string(),
                    vector: stored.clone(),
                    payload: record_hit("a", live, false, 42.0),
                },
                UpsertPoint {
                    entity_id: dead.to_string(),
                    vector: stored.clone(),
                    payload: record_hit("a", dead, true, 99.0),
                },
                UpsertPoint {
                    entity_id: other.to_string(),
                    vector: stored.clone(),
                    payload: record_hit("b", other, false, 7.0),
                },
            ],
        )
        .await
        .unwrap();

    let search = SemanticSearch::new(embedder.clone(), index);
    let hits = search.search("a", query, 8).await.unwrap();
    assert_eq!(hits.len(), 1, "{hits:?}");
    assert_eq!(hits[0].record_id, Some(live));
    assert_eq!(hits[0].repo, "a");
    assert!(!hits[0].superseded);
    assert_eq!(hits[0].kind, SemanticKind::Record);
    let expected = cosine(&query_vector, &stored);
    assert!(
        (hits[0].score - expected).abs() < 1e-5,
        "score {} is not the query cosine {expected}",
        hits[0].score
    );
    assert!(
        (hits[0].score - 42.0).abs() > 1e-3,
        "search kept the payload score"
    );
    assert_eq!(embedder.calls.load(Ordering::SeqCst), calls_after_setup + 1);

    let shared_hits = shared
        .search(SemanticKind::Record, "a", &query_vector, 8)
        .await
        .unwrap();
    assert_eq!(shared_hits.len(), 1);
    assert_eq!(shared_hits[0].record_id, Some(live));
}

#[tokio::test]
async fn empty_query_does_not_embed() {
    let embedder = ScriptedEmbedder::new(4);
    let search = SemanticSearch::new(embedder.clone(), MemoryIndex::new());
    let hits = search.search("a", "  ", 8).await.unwrap();
    assert!(hits.is_empty());
    assert_eq!(embedder.calls.load(Ordering::SeqCst), 0);
    let hits = search.search("a", "\n\t", 8).await.unwrap();
    assert!(hits.is_empty());
    assert_eq!(embedder.calls.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn ensure_rejects_a_dim_change() {
    assert_eq!(COLLECTION_RECORDS, "records");
    assert_eq!(COLLECTION_CONVENTIONS, "conventions");
    assert_eq!(COLLECTION_CODE_CHUNKS, "code_chunks");

    let index = MemoryIndex::new();
    index.ensure(4).await.unwrap();
    let id = RecordId(Uuid::from_u128(3));
    index
        .upsert(
            SemanticKind::Record,
            &[UpsertPoint {
                entity_id: id.to_string(),
                vector: vec![1.0, 0.0, 0.0, 0.0],
                payload: record_hit("a", id, false, 0.0),
            }],
        )
        .await
        .unwrap();

    let err = index.ensure(3).await.unwrap_err();
    let msg = err.to_string();
    assert!(matches!(err, AvError::Semantic(_)), "{msg}");
    assert!(msg.contains(COLLECTION_RECORDS), "{msg}");
    assert!(msg.contains('4') && msg.contains('3'), "{msg}");

    let hits = index
        .search(SemanticKind::Record, "a", &[1.0, 0.0, 0.0, 0.0], 5)
        .await
        .unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].record_id, Some(id));
}

#[tokio::test]
async fn set_superseded_on_a_missing_point_is_ok() {
    let index = MemoryIndex::new();
    index.ensure(4).await.unwrap();
    index
        .set_superseded(SemanticKind::Record, "missing")
        .await
        .unwrap();
    index
        .delete_points(SemanticKind::Convention, &["missing".to_string()])
        .await
        .unwrap();

    let entity_id = "7".to_string();
    let vector = vec![0.0, 1.0, 0.0, 0.0];
    let mut first = record_hit("a", RecordId(Uuid::from_u128(7)), false, 1.0);
    first.path = Some("old.py".into());
    index
        .upsert(
            SemanticKind::Record,
            &[UpsertPoint {
                entity_id: entity_id.clone(),
                vector: vector.clone(),
                payload: first,
            }],
        )
        .await
        .unwrap();

    let mut second = record_hit("a", RecordId(Uuid::from_u128(7)), false, 9.0);
    second.path = Some("new.py".into());
    second.convention_id = Some(ConventionId(4));
    index
        .upsert(
            SemanticKind::Record,
            &[UpsertPoint {
                entity_id: entity_id.clone(),
                vector: vector.clone(),
                payload: second,
            }],
        )
        .await
        .unwrap();

    let hits = index
        .search(SemanticKind::Record, "a", &vector, 8)
        .await
        .unwrap();
    assert_eq!(hits.len(), 1, "{hits:?}");
    assert_eq!(hits[0].path.as_deref(), Some("new.py"));
    assert_eq!(hits[0].convention_id, Some(ConventionId(4)));
    assert!((hits[0].score - 1.0).abs() < 1e-5, "{}", hits[0].score);
    assert!((hits[0].score - 9.0).abs() > 1e-3, "{}", hits[0].score);

    index
        .set_superseded(SemanticKind::Record, &entity_id)
        .await
        .unwrap();
    let hidden = index
        .search(SemanticKind::Record, "a", &vector, 8)
        .await
        .unwrap();
    assert!(hidden.is_empty(), "{hidden:?}");
}

#[tokio::test]
async fn qdrant_roundtrip_filters_repo_and_superseded() {
    let url = std::env::var("AV_TEST_QDRANT_URL").unwrap_or_default();
    if url.is_empty() {
        println!("skip: AV_TEST_QDRANT_URL unset");
        return;
    }

    let index = QdrantIndex::new(url);
    let repo_a = format!("av-{}", Uuid::new_v4());
    let repo_b = format!("av-{}", Uuid::new_v4());
    let result = qdrant_scenario(&index, &repo_a, &repo_b).await;
    let _ = index.delete_repo(&repo_a).await;
    let _ = index.delete_repo(&repo_b).await;
    result.unwrap();
}

async fn qdrant_scenario(
    index: &QdrantIndex,
    repo_a: &str,
    repo_b: &str,
) -> scholion_core::Result<()> {
    index.ensure(4).await?;
    let missing = Uuid::new_v4().to_string();
    index.set_superseded(SemanticKind::Record, &missing).await?;
    index
        .delete_points(SemanticKind::CodeChunk, &[missing])
        .await?;

    let live = RecordId(Uuid::new_v4());
    let dead = RecordId(Uuid::new_v4());
    let other = RecordId(Uuid::new_v4());
    let vector = vec![1.0, 0.0, 0.0, 0.0];
    index
        .upsert(
            SemanticKind::Record,
            &[
                UpsertPoint {
                    entity_id: live.to_string(),
                    vector: vector.clone(),
                    payload: record_hit(repo_a, live, false, 42.0),
                },
                UpsertPoint {
                    entity_id: dead.to_string(),
                    vector: vector.clone(),
                    payload: record_hit(repo_a, dead, true, 99.0),
                },
                UpsertPoint {
                    entity_id: other.to_string(),
                    vector: vector.clone(),
                    payload: record_hit(repo_b, other, false, 7.0),
                },
            ],
        )
        .await?;

    let hits = index
        .search(SemanticKind::Record, repo_a, &vector, 8)
        .await?;
    assert_eq!(hits.len(), 1, "{hits:?}");
    assert_eq!(hits[0].record_id, Some(live));
    assert_eq!(hits[0].repo, repo_a);
    assert!(!hits[0].superseded);
    assert_eq!(hits[0].kind, SemanticKind::Record);
    assert_eq!(hits[0].task_id, Some(TaskId(Uuid::from_u128(1))));
    assert_eq!(hits[0].path.as_deref(), Some("a.py"));
    assert_eq!(hits[0].change_id, Some(ChangeId("abc".into())));
    assert_eq!(hits[0].symbol_id, None);
    assert_eq!(hits[0].convention_id, None);
    assert!(
        (hits[0].score - 1.0).abs() < 1e-3,
        "qdrant score {} is not cosine",
        hits[0].score
    );
    assert!((hits[0].score - 42.0).abs() > 1e-3);

    let mut replaced = record_hit(repo_a, live, false, 9.0);
    replaced.path = Some("new.py".into());
    index
        .upsert(
            SemanticKind::Record,
            &[UpsertPoint {
                entity_id: live.to_string(),
                vector: vector.clone(),
                payload: replaced,
            }],
        )
        .await?;
    let hits = index
        .search(SemanticKind::Record, repo_a, &vector, 8)
        .await?;
    assert_eq!(hits.len(), 1, "{hits:?}");
    assert_eq!(hits[0].path.as_deref(), Some("new.py"));

    index
        .set_superseded(SemanticKind::Record, &live.to_string())
        .await?;
    let hidden = index
        .search(SemanticKind::Record, repo_a, &vector, 8)
        .await?;
    assert!(hidden.is_empty(), "{hidden:?}");

    let mut chunk = record_hit(repo_a, RecordId(Uuid::nil()), false, 5.0);
    chunk.kind = SemanticKind::CodeChunk;
    chunk.record_id = None;
    chunk.symbol_id = Some(SymbolId(11));
    chunk.path = Some("chunk.py".into());
    index
        .upsert(
            SemanticKind::CodeChunk,
            &[UpsertPoint {
                entity_id: "11".to_string(),
                vector: vector.clone(),
                payload: chunk,
            }],
        )
        .await?;
    let chunks = index
        .search(SemanticKind::CodeChunk, repo_a, &vector, 8)
        .await?;
    assert_eq!(chunks.len(), 1, "{chunks:?}");
    assert_eq!(chunks[0].symbol_id, Some(SymbolId(11)));
    assert_eq!(chunks[0].kind, SemanticKind::CodeChunk);

    let mut convention = record_hit(repo_a, RecordId(Uuid::nil()), false, 5.0);
    convention.kind = SemanticKind::Convention;
    convention.record_id = None;
    convention.task_id = None;
    convention.convention_id = Some(ConventionId(4));
    convention.path = None;
    convention.change_id = None;
    index
        .upsert(
            SemanticKind::Convention,
            &[UpsertPoint {
                entity_id: "4".to_string(),
                vector,
                payload: convention,
            }],
        )
        .await?;
    let conventions = index
        .search(SemanticKind::Convention, repo_a, &[1.0, 0.0, 0.0, 0.0], 8)
        .await?;
    assert_eq!(conventions.len(), 1, "{conventions:?}");
    assert_eq!(conventions[0].convention_id, Some(ConventionId(4)));
    assert_eq!(conventions[0].task_id, None);
    assert_eq!(conventions[0].path, None);
    Ok(())
}
