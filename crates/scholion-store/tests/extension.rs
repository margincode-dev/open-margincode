//! Migrations of code built on the daemon (an extension): they run in the extension's own schema
//! with their own `_sqlx_migrations`, so their numbers never meet the daemon's.

use scholion_core::AvError;
use scholion_store::PgStore;
use scholion_testutil::TestDb;
use sqlx::Row;

static BOARD: sqlx::migrate::Migrator = sqlx::migrate!("./tests/ext_migrations");

async fn versions(store: &PgStore, table: &str) -> Vec<i64> {
    sqlx::query(&format!("SELECT version FROM {table} ORDER BY version"))
        .fetch_all(store.pool())
        .await
        .unwrap()
        .iter()
        .map(|r| r.get::<i64, _>(0))
        .collect()
}

#[tokio::test]
async fn extension_migrations_run_in_their_own_schema() {
    let db = TestDb::new().await;
    let store = PgStore::connect(&db.url).await.unwrap();
    let daemon_before = versions(&store, "public._sqlx_migrations").await;

    store.migrate_extension("agentvcs", &BOARD).await.unwrap();

    assert_eq!(versions(&store, "agentvcs._sqlx_migrations").await, [1]);
    assert_eq!(
        versions(&store, "public._sqlx_migrations").await,
        daemon_before,
        "the daemon's migration list is untouched"
    );
    let n: i64 = sqlx::query_scalar("SELECT count(*) FROM agentvcs.board")
        .fetch_one(store.pool())
        .await
        .unwrap();
    assert_eq!(n, 0);
    let in_public: Option<String> = sqlx::query_scalar("SELECT to_regclass('public.board')::text")
        .fetch_one(store.pool())
        .await
        .unwrap();
    assert_eq!(in_public, None, "the extension's table is not in public");
}

#[tokio::test]
async fn the_daemon_and_the_extension_both_start_again_on_the_same_database() {
    let db = TestDb::new().await;
    let store = PgStore::connect(&db.url).await.unwrap();
    store.migrate_extension("agentvcs", &BOARD).await.unwrap();
    drop(store);

    // the daemon's own migrator must not see the extension's version 1 as its own
    let again = PgStore::connect(&db.url).await.unwrap();
    again.migrate_extension("agentvcs", &BOARD).await.unwrap();
    assert_eq!(versions(&again, "agentvcs._sqlx_migrations").await, [1]);
}

#[tokio::test]
async fn pooled_connections_keep_the_default_search_path() {
    let db = TestDb::new().await;
    // one connection: whatever the migration ran on is the one the next query gets
    let store = PgStore::connect_with(&db.url, 1).await.unwrap();
    store.migrate_extension("agentvcs", &BOARD).await.unwrap();
    let schema: String = sqlx::query_scalar("SELECT current_schema()::text")
        .fetch_one(store.pool())
        .await
        .unwrap();
    assert_eq!(schema, "public");
}

#[tokio::test]
async fn a_bad_schema_name_is_refused() {
    let db = TestDb::new().await;
    let store = PgStore::connect(&db.url).await.unwrap();
    for bad in [
        "",
        "public",
        "pg_board",
        "information_schema",
        "Board",
        "a-b",
        "a;b",
        "1board",
    ] {
        let err = store.migrate_extension(bad, &BOARD).await.unwrap_err();
        assert!(matches!(err, AvError::Invalid(_)), "{bad:?}: {err:?}");
    }
    let long = "a".repeat(64);
    assert!(matches!(
        store.migrate_extension(&long, &BOARD).await.unwrap_err(),
        AvError::Invalid(_)
    ));
}
