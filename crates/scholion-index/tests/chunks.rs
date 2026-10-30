use scholion_core::*;
use scholion_index::{CodeIndex, MAX_FILE_BYTES, PgIndex, ScopeView};
use scholion_store::PgStore;
use scholion_testutil::TestDb;
use sqlx::PgPool;
use sqlx::Row;

async fn setup() -> (TestDb, PgIndex, PgStore, PgPool) {
    let db = TestDb::new().await;
    let store = PgStore::connect(&db.url).await.unwrap();
    let pool = store.pool().clone();
    (db, PgIndex::new(pool.clone()), store, pool)
}

fn f(path: &str, src: &str) -> (String, Vec<u8>) {
    (path.to_owned(), src.as_bytes().to_vec())
}

async fn outbox_count(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT count(*) FROM outbox")
        .fetch_one(pool)
        .await
        .unwrap()
}

async fn ops(pool: &PgPool) -> Vec<(String, String)> {
    let rows = sqlx::query("SELECT op, entity_id FROM outbox ORDER BY id")
        .fetch_all(pool)
        .await
        .unwrap();
    rows.iter()
        .map(|r| (r.get::<String, _>("op"), r.get::<String, _>("entity_id")))
        .collect()
}

#[tokio::test]
async fn index_enqueues_one_upsert_and_second_pass_enqueues_nothing() {
    let (_db, idx, _store, pool) = setup().await;
    let files = vec![f("a.py", "def hello():\n    return 1\n")];
    idx.index_paths("demo", "main", files.clone())
        .await
        .unwrap();

    let rows = ops(&pool).await;
    assert_eq!(rows, vec![("upsert".into(), rows[0].1.clone())]);
    let text: String = sqlx::query_scalar("SELECT text FROM code_chunks")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(text.starts_with("a.py\nhello\n"), "{text}");
    let id: i64 = sqlx::query_scalar("SELECT symbol_id FROM code_chunks")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(idx.chunk_exists(SymbolId(id)).await.unwrap());

    let before = outbox_count(&pool).await;
    idx.index_paths("demo", "main", files).await.unwrap();
    assert_eq!(outbox_count(&pool).await, before);
}

#[tokio::test]
async fn symbol_body_over_2000_chars_is_truncated() {
    let (_db, idx, _store, pool) = setup().await;
    let src = format!("def big():\n    return \"{}\"\n", "x".repeat(5_000));
    idx.index_paths("demo", "main", vec![f("big.py", &src)])
        .await
        .unwrap();
    let text: String = sqlx::query_scalar("SELECT text FROM code_chunks")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(text.chars().count(), 2000);
    assert!(text.ends_with('…'), "{text}");
}

#[tokio::test]
async fn file_over_1mb_enqueues_no_upsert() {
    let (_db, idx, _store, pool) = setup().await;
    let huge = vec![b'x'; MAX_FILE_BYTES + 1];
    idx.index_paths("demo", "main", vec![("huge.py".into(), huge.clone())])
        .await
        .unwrap();
    assert!(ops(&pool).await.iter().all(|(op, _)| op != "upsert"));
    assert_eq!(idx.list_code_chunks(Some("demo")).await.unwrap().len(), 0);

    idx.index_paths(
        "demo",
        "main",
        vec![f("huge.py", "def small():\n    return 1\n")],
    )
    .await
    .unwrap();
    idx.index_paths("demo", "main", vec![("huge.py".into(), huge)])
        .await
        .unwrap();
    let rows = ops(&pool).await;
    let deletes: Vec<_> = rows.iter().filter(|(op, _)| op == "delete").collect();
    assert_eq!(deletes.len(), 1);
    assert!(idx.list_code_chunks(Some("demo")).await.unwrap().is_empty());
}

#[tokio::test]
async fn rewriting_a_file_enqueues_delete_then_upsert() {
    let (_db, idx, _store, pool) = setup().await;
    idx.index_paths("demo", "main", vec![f("a.py", "def old():\n    pass\n")])
        .await
        .unwrap();
    let old = idx
        .symbols_in(&ScopeView::new("demo", "main"), "a.py")
        .await
        .unwrap();
    assert_eq!(old.len(), 1);
    let old_id = old[0].id.0.to_string();

    idx.index_paths("demo", "main", vec![f("a.py", "def new():\n    pass\n")])
        .await
        .unwrap();
    let new = idx
        .symbols_in(&ScopeView::new("demo", "main"), "a.py")
        .await
        .unwrap();
    assert_eq!(new.len(), 1);
    let new_id = new[0].id.0.to_string();
    assert_ne!(old_id, new_id);

    let rows = ops(&pool).await;
    assert!(rows.iter().any(|(op, id)| op == "delete" && id == &old_id));
    assert!(rows.iter().any(|(op, id)| op == "upsert" && id == &new_id));
}

#[tokio::test]
async fn remove_paths_enqueues_deletes() {
    let (_db, idx, _store, pool) = setup().await;
    idx.index_paths("demo", "main", vec![f("a.py", "def gone():\n    pass\n")])
        .await
        .unwrap();
    let id = idx
        .symbols_in(&ScopeView::new("demo", "main"), "a.py")
        .await
        .unwrap()[0]
        .id
        .0
        .to_string();
    idx.remove_paths("demo", "main", &["a.py".into()])
        .await
        .unwrap();
    assert!(
        ops(&pool)
            .await
            .iter()
            .any(|(op, entity)| op == "delete" && entity == &id)
    );

    sqlx::query("DELETE FROM outbox")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query(
        "CREATE FUNCTION av_test_outbox_boom() RETURNS trigger LANGUAGE plpgsql AS $$
         BEGIN RAISE EXCEPTION 'boom'; END; $$",
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "CREATE TRIGGER av_test_outbox_boom BEFORE INSERT ON outbox
         FOR EACH ROW EXECUTE FUNCTION av_test_outbox_boom()",
    )
    .execute(&pool)
    .await
    .unwrap();

    let err = idx
        .index_paths("demo", "main", vec![f("b.py", "def fresh():\n    pass\n")])
        .await
        .unwrap_err();
    assert!(matches!(err, AvError::Index(_)), "{err}");
    let symbols: i64 = sqlx::query_scalar("SELECT count(*) FROM symbols")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(symbols, 0);
}

#[tokio::test]
async fn task_base_import_enqueues_no_upserts_but_checkpoint_does() {
    let (_db, idx, _store, pool) = setup().await;
    let files = vec![
        f("a.py", "def a():\n    pass\n"),
        f("b.py", "def b():\n    pass\n"),
    ];
    idx.index_paths_at("demo", "fix-x", None, false, files.clone())
        .await
        .unwrap();
    let chunks = idx.list_code_chunks(Some("demo")).await.unwrap();
    assert_eq!(chunks.len(), 2);
    assert_eq!(outbox_count(&pool).await, 0);

    idx.index_paths_at(
        "demo",
        "fix-x",
        None,
        true,
        vec![f("a.py", "def a2():\n    return 1\n")],
    )
    .await
    .unwrap();
    let rows = sqlx::query("SELECT op, path FROM outbox ORDER BY id")
        .fetch_all(&pool)
        .await
        .unwrap();
    assert!(!rows.is_empty());
    assert!(
        rows.iter()
            .all(|r| r.get::<Option<String>, _>("path").as_deref() == Some("a.py"))
    );
    assert!(rows.iter().any(|r| r.get::<String, _>("op") == "upsert"));
    assert!(
        rows.iter()
            .filter(|r| r.get::<String, _>("op") == "upsert")
            .count()
            == 1
    );

    let before = outbox_count(&pool).await;
    idx.index_paths("demo", "main", files).await.unwrap();
    let main_upserts: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM outbox o
         JOIN symbols s ON s.id::text = o.entity_id
         WHERE o.op = 'upsert' AND s.scope = 'main'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(main_upserts, 2);
    assert!(outbox_count(&pool).await >= before + 2);
}

#[tokio::test]
async fn task_scope_stores_task_id_and_change() {
    let (_db, idx, store, pool) = setup().await;
    let task = store
        .create_task(NewTask {
            repo: "demo".into(),
            title: "fix-x".into(),
            goal: "goal".into(),
            owner_agent: "claude".into(),
        })
        .await
        .unwrap();
    assert_eq!(task.slug, "fix-x");

    idx.index_paths_at(
        "demo",
        "fix-x",
        Some(&ChangeId("abc".into())),
        true,
        vec![f("a.py", "def a():\n    pass\n")],
    )
    .await
    .unwrap();
    let (task_id, change_id): (uuid::Uuid, String) =
        sqlx::query_as("SELECT task_id, change_id FROM code_chunks")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(task_id, task.id.0);
    assert_eq!(change_id, "abc");

    idx.index_paths_at(
        "demo",
        "main",
        Some(&ChangeId("main-change".into())),
        true,
        vec![f("m.py", "def m():\n    pass\n")],
    )
    .await
    .unwrap();
    let main_task: Option<uuid::Uuid> =
        sqlx::query_scalar("SELECT task_id FROM code_chunks WHERE scope = 'main'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert!(main_task.is_none());
}
