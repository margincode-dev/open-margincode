//! Plan 2 Task 6: outbox worker (TestDb + ScriptedEmbedder + MemoryIndex, no network).

use std::sync::Arc;
use std::sync::atomic::Ordering;

use scholion_core::*;
use scholion_daemon::{DrainStats, OutboxWorker};
use scholion_index::{CodeIndex, PgIndex};
use scholion_semantic::{Embedder, MemoryIndex, ScriptedEmbedder, VectorIndex};
use scholion_store::PgStore;
use scholion_testutil::TestDb;
use uuid::Uuid;

const DIM: usize = 8;

struct Env {
    _db: TestDb,
    store: Arc<PgStore>,
    index: Arc<PgIndex>,
    embedder: ScriptedEmbedder,
    memory: MemoryIndex,
    task: Task,
}

impl Env {
    async fn new() -> Env {
        let db = TestDb::new().await;
        let store = Arc::new(PgStore::connect(&db.url).await.unwrap());
        let index = Arc::new(PgIndex::new(store.pool().clone()));
        let task = store
            .create_task(NewTask {
                repo: "demo".into(),
                title: "Worker".into(),
                goal: "g".into(),
                owner_agent: "a".into(),
            })
            .await
            .unwrap();
        Env {
            _db: db,
            store,
            index,
            embedder: ScriptedEmbedder::new(DIM),
            memory: MemoryIndex::new(),
            task,
        }
    }

    fn worker(&self) -> OutboxWorker {
        OutboxWorker::new(
            self.store.clone(),
            self.index.clone(),
            Arc::new(self.embedder.clone()),
            Arc::new(self.memory.clone()),
        )
    }

    async fn record(&self, summary: &str, supersedes: Option<RecordId>) -> Record {
        self.store
            .add_record(NewRecord {
                task_id: self.task.id,
                kind: RecordKind::Note,
                summary: summary.into(),
                body: String::new(),
                change_id: None,
                supersedes,
                author_agent: "a".into(),
                anchors: vec![],
            })
            .await
            .unwrap()
    }

    /// Live (non-superseded) record hits for `text`, searched with that text's own vector.
    async fn record_hits(&self, text: &str) -> Vec<RecordId> {
        let v = self
            .embedder
            .embed(&[text.to_string()])
            .await
            .unwrap()
            .remove(0);
        // nothing upserted yet means no collection yet; ensure is idempotent at one dim
        self.memory.ensure(DIM).await.unwrap();
        self.memory
            .search(SemanticKind::Record, "demo", &v, 50)
            .await
            .unwrap()
            .into_iter()
            .filter_map(|h| h.record_id)
            .collect()
    }

    async fn queued(&self) -> i64 {
        self.store.outbox_count(None).await.unwrap()
    }
}

fn manual_upsert(kind: SemanticKind) -> NewOutbox {
    NewOutbox {
        kind,
        op: OutboxOp::Upsert,
        entity_id: "x".into(),
        repo: "demo".into(),
        task_id: None,
        record_id: None,
        symbol_id: None,
        convention_id: None,
        path: None,
        change_id: None,
        superseded: false,
        text: Some("gone".into()),
    }
}

#[tokio::test]
async fn upsert_for_missing_record_is_dropped_without_a_point() {
    let env = Env::new().await;
    let id = Uuid::new_v4();
    let mut rec = manual_upsert(SemanticKind::Record);
    rec.entity_id = id.to_string();
    rec.record_id = Some(RecordId(id));
    PgStore::enqueue(env.store.pool(), &rec).await.unwrap();
    let mut chunk = manual_upsert(SemanticKind::CodeChunk);
    chunk.entity_id = "999999".into();
    chunk.symbol_id = Some(SymbolId(999_999));
    PgStore::enqueue(env.store.pool(), &chunk).await.unwrap();

    let stats = env.worker().drain_once().await.unwrap();
    assert_eq!((stats.done, stats.failed), (2, 0));
    assert_eq!(env.queued().await, 0);
    assert_eq!(env.embedder.calls.load(Ordering::SeqCst), 0);
    assert!(env.record_hits("gone").await.is_empty());
}

#[tokio::test]
async fn wrong_dimension_does_not_upsert_and_keeps_the_row() {
    let env = Env::new().await;
    env.record("wrong dim", None).await;
    env.embedder.wrong_dim.store(true, Ordering::SeqCst);
    let stats = env.worker().drain_once().await.unwrap();
    assert!(stats.failed >= 1);
    assert_eq!(env.queued().await, 1);
    env.embedder.wrong_dim.store(false, Ordering::SeqCst);
    assert!(env.record_hits("wrong dim").await.is_empty());
}

#[tokio::test]
async fn applying_the_same_upsert_twice_leaves_one_point() {
    let env = Env::new().await;
    let rec = env.record("twice", None).await;
    let row = manual_upsert(SemanticKind::Record);
    env.worker().drain_once().await.unwrap();
    PgStore::enqueue(
        env.store.pool(),
        &NewOutbox {
            entity_id: rec.id.0.to_string(),
            record_id: Some(rec.id),
            task_id: Some(env.task.id),
            text: Some("twice".into()),
            ..row
        },
    )
    .await
    .unwrap();
    env.worker().drain_once().await.unwrap();
    assert_eq!(env.record_hits("twice").await, vec![rec.id]);
}

#[tokio::test]
async fn redelivered_upsert_does_not_clear_superseded() {
    let env = Env::new().await;
    let old = env.record("old idea", None).await;
    env.worker().drain_once().await.unwrap();
    env.record("new idea", Some(old.id)).await;
    env.worker().drain_once().await.unwrap();
    assert!(!env.record_hits("old idea").await.contains(&old.id));
    // A stale copy of the original upsert, still saying superseded = false.
    PgStore::enqueue(
        env.store.pool(),
        &NewOutbox {
            entity_id: old.id.0.to_string(),
            record_id: Some(old.id),
            task_id: Some(env.task.id),
            text: Some("old idea".into()),
            ..manual_upsert(SemanticKind::Record)
        },
    )
    .await
    .unwrap();
    env.worker().drain_once().await.unwrap();
    assert!(
        !env.record_hits("old idea").await.contains(&old.id),
        "a redelivered upsert must not resurrect a superseded record"
    );
}

#[tokio::test]
async fn two_workers_claim_disjoint_rows() {
    let env = Env::new().await;
    for i in 0..10 {
        env.record(&format!("row {i}"), None).await;
    }
    let (a, b) = (env.worker(), env.worker());
    let (sa, sb) = tokio::join!(a.drain_once(), b.drain_once());
    let (sa, sb) = (sa.unwrap(), sb.unwrap());
    assert_eq!(sa.done + sb.done, 10);
    // each row embedded exactly once (in batches, so count texts, not calls)
    assert_eq!(env.embedder.texts.load(Ordering::SeqCst), 10);
    assert_eq!(
        env.record_hits("row 0").await.len(),
        10,
        "one point per record"
    );
}

#[tokio::test]
async fn embedder_down_keeps_the_row_until_a_later_drain() {
    let env = Env::new().await;
    let rec = env.record("later", None).await;
    env.embedder.fail.store(true, Ordering::SeqCst);
    let stats = env.worker().drain_once().await.unwrap();
    assert_eq!(stats.failed, 1);
    assert_eq!(env.queued().await, 1);
    env.embedder.fail.store(false, Ordering::SeqCst);
    sqlx_unlock(&env).await;
    let stats = env.worker().drain_once().await.unwrap();
    assert_eq!(stats.done, 1);
    assert_eq!(env.queued().await, 0);
    assert!(env.record_hits("later").await.contains(&rec.id));
}

#[tokio::test]
async fn supersede_is_applied_before_a_later_upsert_of_another_entity() {
    let env = Env::new().await;
    let a = env.record("alpha", None).await;
    env.worker().drain_once().await.unwrap();
    let b = env.record("beta", Some(a.id)).await;
    let stats = env.worker().drain_once().await.unwrap();
    assert_eq!(stats.failed, 0);
    assert!(!env.record_hits("alpha").await.contains(&a.id));
    assert!(env.record_hits("beta").await.contains(&b.id));
}

#[tokio::test]
async fn code_chunk_upsert_lands_and_delete_removes_it() {
    let env = Env::new().await;
    env.index
        .index_paths(
            "demo",
            "main",
            vec![("a.py".into(), b"def hello():\n    return 1\n".to_vec())],
        )
        .await
        .unwrap();
    env.worker().drain_once().await.unwrap();
    let text = env.index.list_code_chunks(Some("demo")).await.unwrap()[0]
        .text
        .clone();
    let v = env.embedder.embed(&[text]).await.unwrap().remove(0);
    let hits = env
        .memory
        .search(SemanticKind::CodeChunk, "demo", &v, 5)
        .await
        .unwrap();
    assert_eq!(hits.len(), 1);
    env.index
        .remove_paths("demo", "main", &["a.py".into()])
        .await
        .unwrap();
    env.worker().drain_once().await.unwrap();
    let hits = env
        .memory
        .search(SemanticKind::CodeChunk, "demo", &v, 5)
        .await
        .unwrap();
    assert!(hits.is_empty());
}

/// Move every row's lock into the past so a second drain may claim it again.
async fn sqlx_unlock(env: &Env) {
    sqlx::query("UPDATE outbox SET locked_until = now() - interval '1 second'")
        .execute(env.store.pool())
        .await
        .unwrap();
}

#[tokio::test]
async fn consecutive_upserts_are_embedded_in_one_call() {
    let env = Env::new().await;
    let recs = [
        env.record("alpha note", None).await,
        env.record("beta note", None).await,
        env.record("gamma note", None).await,
    ];
    let stats = env.worker().drain_once().await.unwrap();
    assert_eq!(stats, DrainStats { done: 3, failed: 0 });
    assert_eq!(env.embedder.calls.load(Ordering::SeqCst), 1);
    for r in &recs {
        assert!(env.record_hits(&r.summary).await.contains(&r.id));
    }
}

#[tokio::test]
async fn a_text_the_embedder_rejects_fails_only_its_own_row() {
    let env = Env::new().await;
    let good = env.record("good note", None).await;
    let _bad = env.record("bad note", None).await;
    let also_good = env.record("fine note", None).await;
    *env.embedder.poison.lock().unwrap() = Some("bad".into());
    let stats = env.worker().drain_once().await.unwrap();
    assert_eq!(stats, DrainStats { done: 2, failed: 1 });
    assert_eq!(env.queued().await, 1);
    assert!(env.record_hits("good note").await.contains(&good.id));
    assert!(env.record_hits("fine note").await.contains(&also_good.id));
}
