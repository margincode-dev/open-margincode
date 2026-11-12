//! A private PostgreSQL server per test, started from binaries already on the machine.
//!
//! `postgresql_embedded` downloads its binaries on first use, which fails offline and behind a
//! proxy that does not allow the download. A machine with PostgreSQL installed needs none of
//! that: `initdb` into a temporary directory (about half a second), `postgres` on a free
//! local port with trust authentication, and both gone when the test ends. The installed
//! service, if one runs, is never touched.

use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use sqlx::{Connection, PgConnection};

/// The programs a server needs.
const NEEDED: [&str; 3] = ["initdb", "postgres", "pg_ctl"];

fn usable(bin: &Path) -> bool {
    NEEDED.iter().all(|p| bin.join(p).is_file())
}

/// The `bin` directory of an installed PostgreSQL, if there is one. In order:
/// `AV_TEST_PG_INSTALL_DIR` (the directory holding `bin/`; `none` means "do not look", and a
/// directory without the programs is a panic, not a silent download), `pg_config --bindir`
/// when `pg_config` is on `PATH`, then the newest of Debian's `/usr/lib/postgresql/<N>/bin`,
/// Red Hat's `/usr/pgsql-<N>/bin` and Homebrew's `postgresql@<N>/bin`.
pub fn installed_bin() -> Option<PathBuf> {
    if let Ok(dir) = std::env::var("AV_TEST_PG_INSTALL_DIR") {
        if dir == "none" {
            return None;
        }
        let bin = Path::new(&dir).join("bin");
        assert!(
            usable(&bin),
            "AV_TEST_PG_INSTALL_DIR={dir}: no initdb, postgres and pg_ctl in {}",
            bin.display()
        );
        return Some(bin);
    }
    if let Ok(out) = Command::new("pg_config").arg("--bindir").output()
        && out.status.success()
    {
        let bin = PathBuf::from(String::from_utf8_lossy(&out.stdout).trim());
        if usable(&bin) {
            return Some(bin);
        }
    }
    newest_under(&[
        ("/usr/lib/postgresql", ""),
        ("/usr", "pgsql-"),
        ("/opt/homebrew/opt", "postgresql@"),
        ("/usr/local/opt", "postgresql@"),
    ])
}

/// Among the entries `<root>/<prefix><version>` that hold a usable `bin`, the highest version.
fn newest_under(roots: &[(&str, &str)]) -> Option<PathBuf> {
    let mut found: Vec<(u32, PathBuf)> = vec![];
    for (root, prefix) in roots {
        let Ok(entries) = std::fs::read_dir(root) else {
            continue;
        };
        for e in entries.flatten() {
            let name = e.file_name().to_string_lossy().into_owned();
            let Some(version) = name.strip_prefix(prefix) else {
                continue;
            };
            let Ok(major) = version.split('.').next().unwrap_or("").parse::<u32>() else {
                continue;
            };
            let bin = e.path().join("bin");
            if usable(&bin) {
                found.push((major, bin));
            }
        }
    }
    found.sort();
    found.pop().map(|(_, bin)| bin)
}

/// The user a server runs as when the tests run as root, which PostgreSQL refuses to do:
/// `postgres` if the machine has it, else `nobody`. `(uid, gid)` from `/etc/passwd`.
#[cfg(unix)]
fn unprivileged() -> Option<(u32, u32)> {
    let passwd = std::fs::read_to_string("/etc/passwd").ok()?;
    let entry = |name: &str| {
        passwd.lines().find_map(|l| {
            let f: Vec<&str> = l.split(':').collect();
            (f.len() > 3 && f[0] == name).then(|| Some((f[2].parse().ok()?, f[3].parse().ok()?)))?
        })
    };
    entry("postgres").or_else(|| entry("nobody"))
}

#[cfg(unix)]
fn is_root() -> bool {
    use std::os::unix::fs::MetadataExt;
    // the owner of a file this process just made is its effective user
    tempfile::tempfile()
        .and_then(|f| f.metadata())
        .is_ok_and(|m| m.uid() == 0)
}

/// A running server and the directory it lives in.
pub(crate) struct Server {
    pub url: String,
    bin: PathBuf,
    dir: Option<tempfile::TempDir>,
    child: Child,
    run_as: Option<(u32, u32)>,
}

impl Server {
    pub(crate) fn dir(&self) -> &Path {
        self.dir
            .as_ref()
            .expect("a running server has its directory")
            .path()
    }

    fn command(bin: &Path, program: &str, run_as: Option<(u32, u32)>) -> Command {
        let mut cmd = Command::new(bin.join(program));
        cmd.stdin(Stdio::null());
        #[cfg(unix)]
        if let Some((uid, gid)) = run_as {
            use std::os::unix::process::CommandExt;
            cmd.uid(uid).gid(gid);
        }
        #[cfg(not(unix))]
        let _ = run_as;
        cmd
    }

    pub(crate) async fn start(bin: &Path) -> Server {
        let dir = tempfile::Builder::new()
            .prefix("av-test-pg-")
            .tempdir()
            .expect("a temporary directory for the test database");
        #[cfg(unix)]
        let run_as = if is_root() {
            let user = unprivileged()
                .expect("tests run as root and /etc/passwd has no postgres or nobody user");
            // the server's user must enter and own its directory
            std::os::unix::fs::chown(dir.path(), Some(user.0), Some(user.1))
                .expect("chown the test database directory");
            Some(user)
        } else {
            None
        };
        #[cfg(not(unix))]
        let run_as = None;
        let data = dir.path().join("data");
        let out = Self::command(bin, "initdb", run_as)
            .args([
                "--no-sync",
                "-A",
                "trust",
                "-U",
                "postgres",
                "-E",
                "UTF8",
                "-D",
            ])
            .arg(&data)
            .current_dir(dir.path())
            .output()
            .expect("run initdb");
        assert!(
            out.status.success(),
            "initdb failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        let log =
            std::fs::File::create(dir.path().join("server.log")).expect("the test database's log");
        // a port that was free a moment ago; a clash shows as a server that never answers
        let port = std::net::TcpListener::bind("127.0.0.1:0")
            .and_then(|l| l.local_addr())
            .expect("a free local port")
            .port();
        let child = Self::command(bin, "postgres", run_as)
            .arg("-D")
            .arg(&data)
            .args(["-p", &port.to_string()])
            .args(["-c", "listen_addresses=127.0.0.1"])
            .args(["-c", "unix_socket_directories="])
            .args(["-c", "fsync=off"])
            .args(["-c", "synchronous_commit=off"])
            .args(["-c", "full_page_writes=off"])
            .args(["-c", "shared_buffers=16MB"])
            .args(["-c", "timezone=UTC"])
            .current_dir(dir.path())
            .stdout(Stdio::null())
            .stderr(log)
            .spawn()
            .expect("start postgres");
        let mut server = Server {
            url: format!("postgres://postgres@127.0.0.1:{port}/postgres"),
            bin: bin.to_path_buf(),
            dir: Some(dir),
            child,
            run_as,
        };
        let deadline = Instant::now() + Duration::from_secs(30);
        loop {
            if PgConnection::connect(&server.url).await.is_ok() {
                return server;
            }
            let gone = server.child.try_wait().ok().flatten().is_some();
            if gone || Instant::now() > deadline {
                let log =
                    std::fs::read_to_string(server.dir().join("server.log")).unwrap_or_default();
                panic!("the test database did not come up on port {port}:\n{log}");
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }
}

impl Drop for Server {
    /// Stops the server at once (its data is thrown away with the directory) and waits for it,
    /// so no process and no file is left.
    fn drop(&mut self) {
        let Some(dir) = self.dir.take() else {
            return;
        };
        let stopped = Self::command(&self.bin, "pg_ctl", self.run_as)
            .args(["stop", "-m", "immediate", "-w", "-D"])
            .arg(dir.path().join("data"))
            .current_dir(dir.path())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        if !stopped.is_ok_and(|s| s.success()) {
            let _ = self.child.kill();
        }
        let _ = self.child.wait();
        drop(dir);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_newest_install_with_all_three_programs_is_chosen() {
        let root = tempfile::tempdir().unwrap();
        let make = |name: &str, programs: &[&str]| {
            let bin = root.path().join(name).join("bin");
            std::fs::create_dir_all(&bin).unwrap();
            for p in programs {
                std::fs::write(bin.join(p), "").unwrap();
            }
        };
        make("14", &NEEDED);
        make("16", &NEEDED);
        make("17", &["initdb", "postgres"]); // a client-only or broken install
        make("not-a-version", &NEEDED);
        let root_str = root.path().to_str().unwrap();
        assert_eq!(
            newest_under(&[(root_str, "")]),
            Some(root.path().join("16/bin"))
        );
        assert_eq!(newest_under(&[(root_str, "pgsql-")]), None);
        assert_eq!(newest_under(&[("/no/such/dir", "")]), None);
    }
}
