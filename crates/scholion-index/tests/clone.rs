//! Copying a scope's index inside Postgres (task start) instead of re-indexing from the VCS.

use scholion_core::*;
use scholion_index::{CodeIndex, PgIndex, ScopeView};
use scholion_store::PgStore;
use scholion_testutil::TestDb;
use sqlx::PgPool;

async fn setup() -> (TestDb, PgIndex, PgPool) {
    let db = TestDb::new().await;
    let store = PgStore::connect(&db.url).await.unwrap();
    let pool = store.pool().clone();
    (db, PgIndex::new(pool.clone()), pool)
}

fn files() -> Vec<(String, Vec<u8>)> {
    vec![
        (
            "app/a.py".into(),
            b"def hello():\n    return greet()\n\n\ndef greet():\n    return 1\n".to_vec(),
        ),
        (
            "app/b.py".into(),
            b"class Greeter:\n    def wave(self):\n        return hello()\n".to_vec(),
        ),
    ]
}

type SymRow = (
    String,
    String,
    String,
    String,
    String,
    i32,
    i32,
    Vec<String>,
);

async fn symbol_rows(pool: &PgPool, scope: &str) -> Vec<SymRow> {
    sqlx::query_as(
        "SELECT path, qname, name, kind, signature, start_line, end_line, refs
         FROM symbols WHERE repo = 'demo' AND scope = $1 ORDER BY id",
    )
    .bind(scope)
    .fetch_all(pool)
    .await
    .unwrap()
}

async fn edges(pool: &PgPool, scope: &str) -> Vec<(String, String, f32)> {
    sqlx::query_as(
        "SELECT s.qname, d.qname, r.weight FROM symbol_refs r
         JOIN symbols s ON s.id = r.src JOIN symbols d ON d.id = r.dst
         WHERE s.repo = 'demo' AND s.scope = $1 ORDER BY 1, 2",
    )
    .bind(scope)
    .fetch_all(pool)
    .await
    .unwrap()
}

async fn chunks(pool: &PgPool, scope: &str) -> Vec<(String, Option<String>, String)> {
    sqlx::query_as(
        "SELECT c.path, c.change_id, c.text FROM code_chunks c JOIN symbols s ON s.id = c.symbol_id
         WHERE c.repo = 'demo' AND c.scope = $1 AND s.scope = $1 ORDER BY s.id",
    )
    .bind(scope)
    .fetch_all(pool)
    .await
    .unwrap()
}

async fn count(pool: &PgPool, sql: &str) -> i64 {
    sqlx::query_scalar(sql).fetch_one(pool).await.unwrap()
}

#[tokio::test]
async fn clone_scope_copies_symbols_edges_and_chunks_under_new_ids() {
    let (_db, idx, pool) = setup().await;
    let c = ChangeId("c1".into());
    idx.index_paths_at("demo", "main", Some(&c), true, files())
        .await
        .unwrap();

    // main's change is not recorded yet: nothing to copy from
    assert!(!idx.clone_scope("demo", "main", "t1", &c).await.unwrap());
    idx.set_scope_change("demo", "main", &c).await.unwrap();
    assert_eq!(
        idx.scope_change("demo", "main").await.unwrap(),
        Some(c.clone())
    );
    // main is at another change than the task's base
    assert!(
        !idx.clone_scope("demo", "main", "t1", &ChangeId("other".into()))
            .await
            .unwrap()
    );
    assert!(symbol_rows(&pool, "t1").await.is_empty());

    let outbox_before = count(&pool, "SELECT count(*) FROM outbox").await;
    assert!(idx.clone_scope("demo", "main", "t1", &c).await.unwrap());

    assert_eq!(
        symbol_rows(&pool, "main").await,
        symbol_rows(&pool, "t1").await
    );
    let shared_ids = count(
        &pool,
        "SELECT count(*) FROM symbols a JOIN symbols b ON a.id = b.id
         WHERE a.scope = 'main' AND b.scope = 't1'",
    )
    .await;
    assert_eq!(shared_ids, 0);
    let main_edges = edges(&pool, "main").await;
    assert!(!main_edges.is_empty());
    assert_eq!(main_edges, edges(&pool, "t1").await);
    assert_eq!(chunks(&pool, "main").await, chunks(&pool, "t1").await);
    // the copy embeds nothing (task base imports never are) and records its own change
    assert_eq!(
        count(&pool, "SELECT count(*) FROM outbox").await,
        outbox_before
    );
    assert_eq!(
        idx.scope_change("demo", "t1").await.unwrap(),
        Some(c.clone())
    );
    // the file hashes came along: re-indexing the same content is a no-op
    let stats = idx
        .index_paths_at("demo", "t1", Some(&c), false, files())
        .await
        .unwrap();
    assert_eq!(stats.unchanged, 2, "{stats:?}");
    // the copy resolves and walks like the original
    let t1 = idx
        .symbols_in(&ScopeView::new("demo", "t1"), "app/a.py")
        .await
        .unwrap();
    let hello = t1.iter().find(|s| s.symbol.qname == "hello").unwrap().id;
    let near = idx
        .neighbors(&ScopeView::new("demo", "t1"), &[hello], 1)
        .await
        .unwrap();
    assert_eq!(near.len(), 2, "{near:?}"); // greet (callee) and Greeter.wave (caller)

    // never into a scope that already has an index
    assert!(matches!(
        idx.clone_scope("demo", "main", "t1", &c).await,
        Err(AvError::Invalid(_))
    ));
}
