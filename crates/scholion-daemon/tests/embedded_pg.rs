//! Lite mode: the daemon's own Postgres when `AV_DATABASE_URL` is unset.
//!
//! Skips unless `AV_TEST_PG_INSTALL_DIR` names a PostgreSQL installation (the directory holding
//! `bin/`, e.g. `/usr/lib/postgresql/16`), so it never downloads. Postgres refuses to run as root.

use scholion_daemon::EmbeddedPg;
use sqlx::{Connection, PgConnection};

#[tokio::test]
async fn embedded_postgres_keeps_its_data_and_password_across_restarts() {
    let Some(install) = std::env::var_os("AV_TEST_PG_INSTALL_DIR").filter(|v| !v.is_empty()) else {
        println!("skip: AV_TEST_PG_INSTALL_DIR unset");
        return;
    };
    let home = tempfile::tempdir().unwrap();
    let dir = home.path().join("pg");

    let pg = EmbeddedPg::start(&dir, Some(install.clone().into()))
        .await
        .unwrap();
    let mut conn = PgConnection::connect(pg.url()).await.unwrap();
    sqlx::raw_sql("CREATE TABLE kept (n int); INSERT INTO kept VALUES (42)")
        .execute(&mut conn)
        .await
        .unwrap();
    conn.close().await.unwrap();
    let first_url = pg.url().to_owned();
    pg.stop().await.unwrap();
    assert!(
        PgConnection::connect(&first_url).await.is_err(),
        "stopped server still answers"
    );

    let pg = EmbeddedPg::start(&dir, Some(install.into())).await.unwrap();
    let mut conn = PgConnection::connect(pg.url()).await.unwrap();
    let (n,): (i32,) = sqlx::query_as("SELECT n FROM kept")
        .fetch_one(&mut conn)
        .await
        .unwrap();
    assert_eq!(n, 42);
    conn.close().await.unwrap();
    pg.stop().await.unwrap();
}

#[tokio::test]
async fn embedded_postgres_starts_after_a_crash_left_its_pid_file() {
    let Some(install) = std::env::var_os("AV_TEST_PG_INSTALL_DIR").filter(|v| !v.is_empty()) else {
        println!("skip: AV_TEST_PG_INSTALL_DIR unset");
        return;
    };
    let home = tempfile::tempdir().unwrap();
    let dir = home.path().join("pg");
    let pg = EmbeddedPg::start(&dir, Some(install.clone().into()))
        .await
        .unwrap();
    let pid_file = std::fs::read_to_string(dir.join("data/postmaster.pid")).unwrap();
    let pid = pid_file.lines().next().unwrap().trim().to_owned();
    let killed = std::process::Command::new("kill")
        .args(["-9", &pid])
        .status()
        .unwrap();
    assert!(killed.success());
    std::mem::forget(pg); // a crashed daemon runs no destructor either
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let pg = EmbeddedPg::start(&dir, Some(install.into())).await.unwrap();
    let mut conn = PgConnection::connect(pg.url()).await.unwrap();
    let (n,): (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(&mut conn)
        .await
        .unwrap();
    assert_eq!(n, 1);
    conn.close().await.unwrap();
    pg.stop().await.unwrap();
}
