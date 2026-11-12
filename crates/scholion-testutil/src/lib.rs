//! Test helper: a throwaway PostgreSQL database per call.
//!
//! * `AV_TEST_DATABASE_URL` set (e.g. the compose Postgres): creates a fresh database on that server.
//! * else, when a PostgreSQL is installed on the machine (`installed_bin`): starts a private
//!   server from its binaries in a temporary directory (one per `TestDb`, stopped and removed
//!   on drop). Nothing is downloaded and no service is touched, so tests run offline.
//! * otherwise: downloads and starts a user-space Postgres via `postgresql_embedded`
//!   (one server per `TestDb`, stopped on drop). Used when Docker is unavailable.

mod installed;

use postgresql_embedded::PostgreSQL;
use sqlx::{Connection, PgConnection};

pub use installed::installed_bin;

pub struct TestDb {
    pub url: String,
    _server: Option<PostgreSQL>,
    /// A private server started from an installed PostgreSQL.
    _local: Option<installed::Server>,
    /// On a shared server: its URL and our database, dropped with us.
    shared: Option<(String, String)>,
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
                _local: None,
                shared: Some((base, name)),
            };
        }
        if let Some(bin) = installed_bin() {
            return TestDb::on_installed(&bin).await;
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
            _local: None,
            shared: None,
        }
    }

    /// A database on a private server started from the PostgreSQL binaries in `bin`.
    pub async fn on_installed(bin: &std::path::Path) -> TestDb {
        let server = installed::Server::start(bin).await;
        TestDb {
            url: server.url.clone(),
            _server: None,
            _local: Some(server),
            shared: None,
        }
    }
}

impl Drop for TestDb {
    /// Drops the database on a shared server, so test runs do not fill its disk. On a thread of its
    /// own: `drop` may run inside a runtime, which cannot block on another.
    fn drop(&mut self) {
        let Some((base, name)) = self.shared.take() else {
            return;
        };
        let _ = std::thread::spawn(move || {
            let Ok(rt) = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            else {
                return;
            };
            rt.block_on(async {
                if let Ok(mut conn) = PgConnection::connect(&base).await {
                    let drop = format!("DROP DATABASE IF EXISTS {name} WITH (FORCE)");
                    let _ = sqlx::query(&drop).execute(&mut conn).await;
                }
            });
        })
        .join();
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

    /// With a PostgreSQL installed, a private server is started from it: no download, and
    /// nothing is left behind when the `TestDb` goes.
    #[tokio::test]
    async fn an_installed_postgres_serves_a_private_database_and_leaves_nothing() {
        let Some(bin) = installed_bin() else {
            eprintln!("SKIPPED: no PostgreSQL is installed on this machine");
            return;
        };
        let db = TestDb::on_installed(&bin).await;
        let dir = db._local.as_ref().unwrap().dir().to_path_buf();
        assert!(dir.join("data/PG_VERSION").exists());
        let mut conn = PgConnection::connect(&db.url).await.unwrap();
        sqlx::query("CREATE TABLE t (n int)")
            .execute(&mut conn)
            .await
            .unwrap();
        // sqlx always asks for UTC, and the store's migrations need these
        let (tz,): (String,) = sqlx::query_as("SHOW TimeZone")
            .fetch_one(&mut conn)
            .await
            .unwrap();
        assert_eq!(tz, "UTC");
        drop(conn);
        // two at once do not meet
        let other = TestDb::on_installed(&bin).await;
        assert_ne!(other.url, db.url);
        let mut conn = PgConnection::connect(&other.url).await.unwrap();
        assert!(
            sqlx::query("SELECT n FROM t")
                .fetch_one(&mut conn)
                .await
                .is_err()
        );
        drop(conn);
        let url = db.url.clone();
        drop(db);
        assert!(!dir.exists(), "the server's directory is removed");
        assert!(
            PgConnection::connect(&url).await.is_err(),
            "and it is stopped"
        );
    }
}
