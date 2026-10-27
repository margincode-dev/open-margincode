#![allow(dead_code)]

use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use std::thread::JoinHandle;

use scholion_daemon::{AppState, router};
use scholion_index::PgIndex;
use scholion_store::PgStore;
use scholion_testutil::TestDb;
use scholion_vcs::JjVcs;
use tempfile::TempDir;
use tokio::sync::oneshot;

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

/// A repo with a base commit (`calc.py` with a bug, `README.md`) and a fix commit that corrects
/// `calc.py` and adds `test_calc.py`. Returns (base, fix).
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

/// A real daemon on an ephemeral port.
pub struct TestDaemon {
    pub url: String,
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
            let store = Arc::new(PgStore::connect(&db.url).await.unwrap());
            let index = Arc::new(PgIndex::new(store.pool().clone()));
            let vcs = Arc::new(JjVcs::new(
                store_dir.path().to_path_buf(),
                ws.path().to_path_buf(),
            ));
            let state = AppState::new(
                vcs,
                store,
                index,
                ws.path().to_path_buf(),
                ws.path().to_path_buf(),
            )
            .with_token(token);
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            ready_tx
                .send(format!("http://{}", listener.local_addr().unwrap()))
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
    let url = ready_rx.recv().expect("daemon failed to start");
    TestDaemon {
        url,
        shutdown: Some(shutdown_tx),
        thread: Some(thread),
    }
}
