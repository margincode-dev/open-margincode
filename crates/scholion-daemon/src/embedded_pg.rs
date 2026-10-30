//! Lite mode: with no `AV_DATABASE_URL`, the daemon runs its own user-space Postgres, so a laptop
//! needs neither Docker nor a database server. Data, password and (unless a local installation is
//! given) the Postgres binaries live under one directory, `$AV_HOME/pg` by default.

use std::path::{Path, PathBuf};
use std::time::Duration;

use postgresql_embedded::{PostgreSQL, Settings, Status, VersionReq};
use scholion_core::{AvError, Result};

/// The database the daemon uses inside its own server.
const DATABASE: &str = "scholion";
/// Same major as compose and CI; a data directory only opens with the major that created it.
const MAJOR: &str = "^16";

pub struct EmbeddedPg {
    pg: PostgreSQL,
    url: String,
}

impl EmbeddedPg {
    /// Starts (installing and initializing on first use) the server whose data is in `dir`.
    /// `install_dir` is an existing PostgreSQL installation (the directory holding `bin/`) to use
    /// instead of the bundled or downloaded one. A server left running by a crashed daemon is
    /// stopped first, so call this only after this process owns the daemon's listen address.
    pub async fn start(dir: &Path, install_dir: Option<PathBuf>) -> Result<EmbeddedPg> {
        std::fs::create_dir_all(dir).map_err(|e| failed("create", dir, e))?;
        let mut settings = Settings::new();
        settings.data_dir = dir.join("data");
        settings.password_file = dir.join("password");
        settings.temporary = false;
        settings.timeout = Some(Duration::from_secs(60));
        settings.host = "127.0.0.1".into();
        settings
            .configuration
            .insert("listen_addresses".into(), "127.0.0.1".into());
        // TCP only: a distribution's Postgres puts its socket in /var/run/postgresql, which only
        // the `postgres` user may write, and every connection here is TCP anyway.
        settings
            .configuration
            .insert("unix_socket_directories".into(), "''".into());
        match install_dir {
            Some(path) => {
                settings.installation_dir = path;
                settings.trust_installation_dir = true;
            }
            None => {
                settings.installation_dir = dir.join("install");
                settings.version = VersionReq::parse(MAJOR).expect("valid version requirement");
            }
        }
        // initdb wrote the password on first use; later starts must log in with that one.
        if let Ok(saved) = std::fs::read_to_string(&settings.password_file) {
            settings.password = saved.trim().to_owned();
        }
        let mut pg = PostgreSQL::new(settings);
        pg.setup().await.map_err(|e| failed("set up", dir, e))?;
        ensure_utc_zone(&pg.settings().installation_dir);
        if pg.status() == Status::Started {
            // A pid file from a daemon that died without stopping it: stop that server if it still
            // runs; if it does not, Postgres itself removes the stale file on start.
            let _ = pg.stop().await;
        }
        if let Err(e) = pg.start().await {
            let log = dir.join("data/start.log");
            return Err(failed(
                "start",
                dir,
                format!("{e}; {}:\n{}", log.display(), tail(&log, 10)),
            ));
        }
        if !pg
            .database_exists(DATABASE)
            .await
            .map_err(|e| failed("query", dir, e))?
        {
            pg.create_database(DATABASE)
                .await
                .map_err(|e| failed("create the database in", dir, e))?;
        }
        let url = pg.settings().url(DATABASE);
        Ok(EmbeddedPg { pg, url })
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub async fn stop(self) -> Result<()> {
        self.pg
            .stop()
            .await
            .map_err(|e| AvError::Store(format!("stop embedded postgres: {e}")))
    }
}

fn failed(what: &str, dir: &Path, e: impl std::fmt::Display) -> AvError {
    AvError::Store(format!(
        "embedded postgres: could not {what} {}: {e} (Postgres refuses to run as root; \
         AV_DATABASE_URL uses another server instead)",
        dir.display()
    ))
}

/// The last `n` lines of a log file, or nothing when it cannot be read.
fn tail(path: &Path, n: usize) -> String {
    let text = std::fs::read_to_string(path).unwrap_or_default();
    let lines: Vec<&str> = text.lines().collect();
    lines[lines.len().saturating_sub(n)..].join("\n")
}

/// The theseus Postgres bundle drops tzdata link entries (`UTC`, `GMT`, ...), and sqlx always
/// sends `TimeZone=UTC`, so restore `UTC` from the identical `UCT` file when it is missing.
fn ensure_utc_zone(installation_dir: &Path) {
    let zones = installation_dir.join("share/timezone");
    let utc = zones.join("UTC");
    if utc.exists() || !zones.is_dir() {
        return;
    }
    let src = if Path::new("/usr/share/zoneinfo/UTC").exists() {
        PathBuf::from("/usr/share/zoneinfo/UTC")
    } else {
        zones.join("UCT")
    };
    let _ = std::fs::copy(src, utc);
}
