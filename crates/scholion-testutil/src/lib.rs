//! Test helper: a throwaway PostgreSQL database per call.
//!
//! * `AV_TEST_DATABASE_URL` set (e.g. the compose Postgres): creates a fresh database on that server.
//! * otherwise: downloads and starts a user-space Postgres via `postgresql_embedded`
//!   (one server per `TestDb`, stopped on drop). Used when Docker is unavailable.

use postgresql_embedded::PostgreSQL;
use sqlx::{Connection, PgConnection};

pub struct TestDb {
    pub url: String,
    _server: Option<PostgreSQL>,
}

impl TestDb {
    pub async fn new() -> TestDb {
        let name = format!("av_test_{}", uuid::Uuid::new_v4().simple());
        if let Ok(base) = std::env::var("AV_TEST_DATABASE_URL") {
            let mut conn = PgConnection::connect(&base)
                .await
                .expect("connect AV_TEST_DATABASE_URL");
            sqlx::query(&format!("CREATE DATABASE {name}"))
                .execute(&mut conn)
                .await
                .expect("create database");
            let (prefix, _) = base
                .rsplit_once('/')
                .expect("database url must end with /dbname");
            return TestDb {
                url: format!("{prefix}/{name}"),
                _server: None,
            };
        }
        let mut pg = PostgreSQL::default();
        pg.setup()
            .await
            .expect("postgres setup (needs network on first run)");
        ensure_utc_zone(&pg.settings().installation_dir);
        pg.start().await.expect("postgres start");
        pg.create_database(&name).await.expect("create database");
        let url = pg.settings().url(&name);
        TestDb {
            url,
            _server: Some(pg),
        }
    }
}

/// The theseus Postgres bundle drops tzdata link entries (`UTC`, `GMT`, ...), and sqlx always
/// sends `TimeZone=UTC`, so restore `UTC` from the identical `UCT` file when it is missing.
fn ensure_utc_zone(installation_dir: &std::path::Path) {
    let zones = installation_dir.join("share/timezone");
    let utc = zones.join("UTC");
    if utc.exists() || !zones.is_dir() {
        return;
    }
    let src = if std::path::Path::new("/usr/share/zoneinfo/UTC").exists() {
        std::path::PathBuf::from("/usr/share/zoneinfo/UTC")
    } else {
        zones.join("UCT")
    };
    std::fs::copy(src, utc).expect("restore UTC timezone file");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn gives_a_working_empty_database() {
        let db = TestDb::new().await;
        let mut conn = PgConnection::connect(&db.url).await.unwrap();
        let (n,): (i32,) = sqlx::query_as("SELECT 41 + 1")
            .fetch_one(&mut conn)
            .await
            .unwrap();
        assert_eq!(n, 42);
    }
}
