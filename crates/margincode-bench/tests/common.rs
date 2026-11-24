#![allow(dead_code)]

use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use std::thread::JoinHandle;

use margincode_daemon::{AppState, router};
use margincode_index::PgIndex;
use margincode_store::PgStore;
use margincode_testutil::TestDb;
use margincode_vcs::JjVcs;
use tempfile::TempDir;
use tokio::sync::oneshot;

/// The one bound on any wait in a test: past it, the test hung.
pub const HANG: std::time::Duration = std::time::Duration::from_secs(120);

pub fn git(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .current_dir(dir)
        .args([
            "-c",
            "user.name=t",
            "-c",
            "user.email=t@t",
            "-c",
            "commit.gpgsign=false",
        ])
        .args(args)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).trim().to_owned()
}

pub fn fixture_repo(dir: &Path) -> (String, String) {
    git(dir, &["init", "-q", "-b", "main"]);
    std::fs::write(dir.join("calc.py"), "def add(a, b):\n    return a - b\n").unwrap();
    std::fs::write(dir.join("README.md"), "# calc\n").unwrap();
    git(dir, &["add", "-A"]);
    git(dir, &["commit", "-q", "-m", "base"]);
    let base = git(dir, &["rev-parse", "HEAD"]);
    std::fs::write(dir.join("calc.py"), "def add(a, b):\n    return a + b\n").unwrap();
    std::fs::write(
        dir.join("test_calc.py"),
        "from calc import add\nassert add(2, 3) == 5\nprint('ok')\n",
    )
    .unwrap();
    git(dir, &["add", "-A"]);
    git(dir, &["commit", "-q", "-m", "fix add"]);
    let fix = git(dir, &["rev-parse", "HEAD"]);
    (base, fix)
}

pub fn spec_toml(repo: &Path, base: &str, fix: &str) -> String {
    format!(
        r#"name = "calc-add"
repo_path = "{repo}"
base_rev = "{base}"
fix_rev = "{fix}"
goal = "add() returns the wrong result for positive numbers. Fix it."
relevant_files = ["calc.py"]
created_files = ["test_calc.py"]
hidden_tests = ["test_calc.py"]
verify_cmd = "python3 test_calc.py"
verify_timeout_secs = 20
max_turns = 12
model = "claude-sonnet-5-5"
setup_cmd = ""

[[dead_end]]
description = "Swapping the operands of the subtraction does not fix add()."
file_glob = "calc.py"
pattern = 'return b - a'
"#,
        repo = repo.display()
    )
}

/// The `av` binary cargo built beside this test binary (`<target>/<profile>/av`), if it did.
pub fn real_av() -> Option<std::path::PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let av = exe.parent()?.parent()?.join("av");
    av.is_file().then_some(av)
}

pub struct TestDaemon {
    pub url: String,
    pub ws_root: std::path::PathBuf,
    shutdown: Option<oneshot::Sender<()>>,
    thread: Option<JoinHandle<()>>,
}

impl Drop for TestDaemon {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown.take() {
            let _ = tx.send(());
        }
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

pub fn start_daemon(token: Option<&str>) -> TestDaemon {
    start_daemon_with(token, false)
}

/// `plain_records`: as a daemon run with `AV_PLAIN_RECORDS=on`.
pub fn start_daemon_with(token: Option<&str>, plain_records: bool) -> TestDaemon {
    start(token, plain_records, None)
}

/// A daemon whose workspaces are under `ws_root` (which must exist and be empty), on a fresh database.
pub fn start_daemon_in(ws_root: &Path) -> TestDaemon {
    start(None, false, Some(ws_root.to_path_buf()))
}

fn start(
    token: Option<&str>,
    plain_records: bool,
    ws_root: Option<std::path::PathBuf>,
) -> TestDaemon {
    let token = token.map(str::to_owned);
    let (ready_tx, ready_rx) = std::sync::mpsc::channel();
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    let thread = std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async move {
            let db = TestDb::new().await;
            let store_dir = TempDir::new().unwrap();
            let ws = TempDir::new().unwrap();
            let ws_root = ws_root.unwrap_or_else(|| ws.path().to_path_buf());
            let store = Arc::new(PgStore::connect(&db.url).await.unwrap());
            let index = Arc::new(PgIndex::new(store.pool().clone()));
            let vcs = Arc::new(JjVcs::new(store_dir.path().to_path_buf(), ws_root.clone()));
            let state = AppState::new(vcs, store, index, ws_root.clone(), ws_root.clone())
                .with_token(token)
                .with_plain_records(plain_records);
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            ready_tx
                .send((
                    format!("http://{}", listener.local_addr().unwrap()),
                    ws_root,
                ))
                .unwrap();
            axum::serve(listener, router(state))
                .with_graceful_shutdown(async {
                    let _ = shutdown_rx.await;
                })
                .await
                .unwrap();
            drop((db, store_dir, ws));
        });
    });
    let (url, ws_root) = ready_rx.recv().expect("daemon failed to start");
    TestDaemon {
        url,
        ws_root,
        shutdown: Some(shutdown_tx),
        thread: Some(thread),
    }
}

/// `FAKE_*` env vars are process-wide, so the tests that set them run one at a time across every module.
pub static SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub fn serial() -> std::sync::MutexGuard<'static, ()> {
    SERIAL.lock().unwrap_or_else(|e| e.into_inner())
}
