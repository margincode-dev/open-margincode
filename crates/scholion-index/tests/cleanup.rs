use scholion_core::ChangeId;
use scholion_index::{CodeIndex, PgIndex, ScopeView};
use scholion_store::PgStore;
use scholion_testutil::TestDb;
use sqlx::PgPool;

async fn setup() -> (TestDb, PgIndex, PgPool) {
    let db = TestDb::new().await;
    let pool = PgStore::connect(&db.url).await.unwrap().pool().clone();
    (db, PgIndex::new(pool.clone()), pool)
}

fn f(path: &str, src: &str) -> (String, Vec<u8>) {
    (path.to_owned(), src.as_bytes().to_vec())
}

async fn count(pool: &PgPool, sql: &str) -> i64 {
    sqlx::query_scalar(sql).fetch_one(pool).await.unwrap()
}

#[tokio::test]
async fn drop_scope_removes_symbols_but_not_parse_cache_entries() {
    let (_db, idx, pool) = setup().await;
    let files = vec![
        f("a.py", "def a():\n    b()\n"),
        f("b.py", "def b():\n    pass\n"),
    ];
    idx.index_paths("demo", "main", files.clone())
        .await
        .unwrap();
    // embedded, so it has code chunks
    idx.index_paths_at("demo", "task-1", Some(&ChangeId("h".into())), true, files)
        .await
        .unwrap();
    idx.set_scope_change("demo", "task-1", &ChangeId("h".into()))
        .await
        .unwrap();
    let chunks: i64 = sqlx::query_scalar("SELECT count(*) FROM code_chunks WHERE scope = 'task-1'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(chunks > 0);
    let cache_before = count(&pool, "SELECT count(*) FROM parse_cache").await;
    let deletes_before = count(&pool, "SELECT count(*) FROM outbox WHERE op = 'delete'").await;

    idx.drop_scope("demo", "task-1").await.unwrap();

    for table in ["symbols", "index_files", "code_chunks", "index_scopes"] {
        let n = count(
            &pool,
            &format!("SELECT count(*) FROM {table} WHERE scope = 'task-1'"),
        )
        .await;
        assert_eq!(n, 0, "{table}");
    }
    assert!(
        idx.symbols_in(&ScopeView::new("demo", "main"), "a.py")
            .await
            .unwrap()
            .len()
            == 1,
        "main untouched"
    );
    assert_eq!(
        count(&pool, "SELECT count(*) FROM parse_cache").await,
        cache_before
    );
    let deletes = count(&pool, "SELECT count(*) FROM outbox WHERE op = 'delete'").await;
    assert_eq!(
        deletes - deletes_before,
        chunks,
        "one vector delete per chunk"
    );
    // dropping a scope that does not exist is fine
    idx.drop_scope("demo", "nothing").await.unwrap();
}

#[tokio::test]
async fn prune_parse_cache_keeps_entries_any_scope_still_uses() {
    let (_db, idx, pool) = setup().await;
    idx.index_paths("demo", "main", vec![f("a.py", "def a():\n    pass\n")])
        .await
        .unwrap();
    idx.index_paths("demo", "task-1", vec![f("b.py", "def b():\n    pass\n")])
        .await
        .unwrap();
    assert_eq!(count(&pool, "SELECT count(*) FROM parse_cache").await, 2);
    assert_eq!(idx.prune_parse_cache().await.unwrap(), 0);
    idx.drop_scope("demo", "task-1").await.unwrap();
    assert_eq!(idx.prune_parse_cache().await.unwrap(), 1);
    assert_eq!(count(&pool, "SELECT count(*) FROM parse_cache").await, 1);
}

#[tokio::test]
async fn symbol_scope_names_the_repo_and_scope_of_a_symbol() {
    let (_db, idx, _pool) = setup().await;
    idx.index_paths("demo", "task-1", vec![f("a.py", "def a():\n    pass\n")])
        .await
        .unwrap();
    let id = idx
        .symbols_in(&ScopeView::new("demo", "task-1"), "a.py")
        .await
        .unwrap()[0]
        .id;
    assert_eq!(
        idx.symbol_scope(id).await.unwrap(),
        Some(("demo".to_owned(), "task-1".to_owned()))
    );
    assert_eq!(
        idx.symbol_scope(scholion_core::SymbolId(-1)).await.unwrap(),
        None
    );
}
