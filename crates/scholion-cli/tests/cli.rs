use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::Arc;
use std::thread::JoinHandle;

use scholion_daemon::{AppState, router};
use scholion_index::PgIndex;
use scholion_store::PgStore;
use scholion_testutil::TestDb;
use scholion_vcs::JjVcs;
use tempfile::TempDir;
use tokio::sync::oneshot;

fn git(dir: &Path, args: &[&str]) {
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
}

/// A real daemon on an ephemeral port, in its own thread/runtime; dropped = shut down cleanly.
struct Daemon {
    url: String,
    ws: PathBuf,
    src: PathBuf,
    shutdown: Option<oneshot::Sender<()>>,
    thread: Option<JoinHandle<()>>,
}

impl Drop for Daemon {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown.take() {
            let _ = tx.send(());
        }
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

fn start_daemon() -> Daemon {
    start_daemon_with(None)
}

/// A daemon that requires `token` (`AV_TOKEN`) when it is set.
fn start_daemon_with(token: Option<&str>) -> Daemon {
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
            let src = TempDir::new().unwrap();
            git(src.path(), &["init", "-q", "-b", "main"]);
            std::fs::create_dir_all(src.path().join("app")).unwrap();
            std::fs::write(
                src.path().join("app/greeter.py"),
                "class Greeter:\n    def hello(self, name):\n        return name\n",
            )
            .unwrap();
            std::fs::write(src.path().join("README.md"), "# demo\n").unwrap();
            git(src.path(), &["add", "-A"]);
            git(src.path(), &["commit", "-q", "-m", "init"]);
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
            let url = format!("http://{}", listener.local_addr().unwrap());
            ready_tx
                .send((url, ws.path().to_path_buf(), src.path().to_path_buf()))
                .unwrap();
            axum::serve(listener, router(state))
                .with_graceful_shutdown(async {
                    let _ = shutdown_rx.await;
                })
                .await
                .unwrap();
            drop((db, src, store_dir, ws));
        });
    });
    let (url, ws, src) = ready_rx.recv().expect("daemon failed to start");
    Daemon {
        url,
        ws,
        src,
        shutdown: Some(shutdown_tx),
        thread: Some(thread),
    }
}

fn av_with(url: &str, cwd: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_av"))
        .current_dir(cwd)
        .env("AV_URL", url)
        .env_remove("AV_TASK")
        .env_remove("AV_SESSION")
        .env_remove("AV_AGENT")
        .env_remove("AV_TOKEN")
        .args(args)
        .output()
        .unwrap()
}

fn av(d: &Daemon, cwd: &Path, args: &[&str]) -> Output {
    av_with(&d.url, cwd, args)
}

fn out(o: &Output) -> String {
    String::from_utf8_lossy(&o.stdout).into_owned()
}

fn err(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).into_owned()
}

fn ok(o: Output) -> String {
    assert!(
        o.status.success(),
        "exit {:?}\nstdout: {}\nstderr: {}",
        o.status.code(),
        out(&o),
        err(&o)
    );
    out(&o)
}

/// Daemon + registered repo + started task; returns (daemon, workspace dir, scratch cwd).
fn started() -> (Daemon, PathBuf, TempDir) {
    let d = start_daemon();
    let scratch = TempDir::new().unwrap();
    ok(av(
        &d,
        scratch.path(),
        &["repo", "add", "demo", d.src.to_str().unwrap()],
    ));
    let started = ok(av(
        &d,
        scratch.path(),
        &[
            "task",
            "start",
            "--repo",
            "demo",
            "--title",
            "Improve greeting",
            "--goal",
            "Make hello louder",
        ],
    ));
    let ws = d.ws.join("demo/improve-greeting");
    assert!(
        started.starts_with(&format!("workspace: {}", ws.display())),
        "{started}"
    );
    (d, ws, scratch)
}

#[test]
fn task_start_writes_av_task_file_and_prints_pack() {
    let (d, ws, scratch) = started();
    let again = ok(av(
        &d,
        scratch.path(),
        &[
            "task", "start", "--repo", "demo", "--title", "Second", "--goal", "g",
        ],
    ));
    assert!(
        again.contains("## Task") && again.contains("Second"),
        "{again}"
    );
    let file =
        std::fs::read_to_string(ws.join(".av-task")).expect(".av-task in the workspace root");
    let json: serde_json::Value = serde_json::from_str(&file).unwrap();
    assert!(
        json["task"].as_str().unwrap().len() >= 32 && !json["session"].as_str().unwrap().is_empty(),
        "{file}"
    );
}

#[test]
fn context_uses_task_from_dotfile_in_cwd_and_ancestors() {
    let (d, ws, _scratch) = started();
    // `task start` already sent the Task section to this session, so it is referenced, not repeated
    let md = ok(av(&d, &ws, &["context"]));
    assert!(
        md.contains("## Task") && md.contains("sent earlier"),
        "{md}"
    );
    std::fs::create_dir_all(ws.join("app/deep")).unwrap();
    let md = ok(av(
        &d,
        &ws.join("app/deep"),
        &["context", "--budget", "1500", "--focus", "Greeter"],
    ));
    assert!(md.contains("Greeter"), "{md}");
}

#[test]
fn fresh_context_resends_everything_and_starts_a_new_session() {
    let (d, ws, _scratch) = started();
    let session_before: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(ws.join(".av-task")).unwrap()).unwrap();
    let md = ok(av(&d, &ws, &["context", "--fresh"]));
    assert!(
        md.contains("Make hello louder") && !md.contains("sent earlier"),
        "{md}"
    );
    let session_after: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(ws.join(".av-task")).unwrap()).unwrap();
    assert_ne!(session_before["session"], session_after["session"]);
    assert_eq!(session_before["task"], session_after["task"]);
    let next = ok(av(&d, &ws, &["context"]));
    assert!(
        next.contains("sent earlier"),
        "dedup resumes relative to the fresh pack:\n{next}"
    );
}

#[test]
fn record_with_anchor_flag_parses_path_and_symbol() {
    let (d, ws, _scratch) = started();
    let line = ok(av(
        &d,
        &ws,
        &[
            "record",
            "--kind",
            "attempt",
            "--summary",
            "tried caching hello",
            "--body",
            "broke tests",
            "--anchor",
            "app/greeter.py:Greeter.hello",
            "--anchor",
            "README.md",
        ],
    ));
    assert!(line.contains("rec:") && line.contains("attempt"), "{line}");
    let md = ok(av(&d, &ws, &["context", "--budget", "3000"]));
    let attempt = md
        .lines()
        .find(|l| l.contains("tried caching hello"))
        .expect("the attempt is in the pack");
    assert!(
        !attempt.contains("anchor_stale"),
        "both anchors exist: {attempt}"
    );
    assert!(
        md.contains("anchors: README.md, app/greeter.py:Greeter.hello"),
        "anchors are listed in path order:\n{md}"
    );
    let bad = av(
        &d,
        &ws,
        &["record", "--kind", "note", "--summary", &"x".repeat(201)],
    );
    assert!(
        !bad.status.success() && err(&bad).contains("201"),
        "{}",
        err(&bad)
    );
}

#[test]
fn checkpoint_reports_changed_files_and_status_shows_task() {
    let (d, ws, _scratch) = started();
    std::fs::write(
        ws.join("app/new_module.py"),
        "def brand_new():\n    return 1\n",
    )
    .unwrap();
    let cp = ok(av(&d, &ws, &["checkpoint"]));
    assert!(cp.contains("changed (1): app/new_module.py"), "{cp}");
    let st = ok(av(&d, &ws, &["status"]));
    assert!(
        st.contains("improve-greeting") && st.contains("app/new_module.py"),
        "{st}"
    );
    let md = ok(av(&d, &ws, &["context"]));
    assert!(md.contains("brand_new"), "{md}");
}

#[test]
fn submit_reports_unrecorded_symbols_as_advisory_on_stdout() {
    let (d, ws, _scratch) = started();
    std::fs::write(
        ws.join("app/new_module.py"),
        "def brand_new():\n    return 1\n",
    )
    .unwrap();
    ok(av(&d, &ws, &["checkpoint"]));
    ok(av(
        &d,
        &ws,
        &["record", "--kind", "note", "--summary", "adds a module"],
    ));
    let o = av(&d, &ws, &["submit"]);
    let stdout = ok(o.clone());
    assert!(
        stdout.contains("submitted (in_review)") && stdout.contains("advisory: 1 changed symbol"),
        "{stdout}"
    );
    assert!(
        err(&o).is_empty(),
        "a successful submit writes nothing to stderr: {}",
        err(&o)
    );
}

#[test]
fn show_diff_and_path_log_read_without_a_workspace() {
    let (d, ws, scratch) = started();
    std::fs::write(
        ws.join("app/new_module.py"),
        "def brand_new():\n    return 1\n",
    )
    .unwrap();
    ok(av(&d, &ws, &["checkpoint"]));
    // From outside any workspace: everything is read from the daemon.
    let main = ok(av(
        &d,
        scratch.path(),
        &["show", "app/greeter.py", "--repo", "demo"],
    ));
    assert!(main.contains("class Greeter"), "{main}");
    let at_task = ok(av(
        &d,
        scratch.path(),
        &[
            "show",
            "app/new_module.py",
            "--repo",
            "demo",
            "--at",
            "task:improve-greeting",
        ],
    ));
    assert!(at_task.contains("def brand_new"), "{at_task}");
    let diff = ok(av(
        &d,
        scratch.path(),
        &[
            "diff",
            "--repo",
            "demo",
            "--from",
            "main",
            "--to",
            "task:improve-greeting",
        ],
    ));
    assert!(
        diff.contains("+++ b/app/new_module.py") && diff.contains("+def brand_new"),
        "{diff}"
    );
    let log = ok(av(
        &d,
        scratch.path(),
        &["log", "--repo", "demo", "--path", "app/greeter.py"],
    ));
    assert_eq!(log.lines().count(), 1, "{log}");
}

#[test]
fn convention_list_and_events_feed() {
    let (d, ws, scratch) = started();
    ok(av(
        &d,
        scratch.path(),
        &[
            "convention",
            "add",
            "--repo",
            "demo",
            "--scope",
            "app/**",
            "--required",
            "keep it pure",
        ],
    ));
    let list = ok(av(&d, &ws, &["convention", "list"]));
    assert!(list.contains("(app/**) (required) keep it pure"), "{list}");
    let events = ok(av(
        &d,
        scratch.path(),
        &["events", "--repo", "demo", "--since", "0"],
    ));
    assert!(
        events.contains("started") && events.contains("improve-greeting"),
        "{events}"
    );
}

#[test]
fn validate_runs_the_required_convention_checks() {
    let (d, ws, scratch) = started();
    let added = ok(av(
        &d,
        scratch.path(),
        &[
            "convention",
            "add",
            "--repo",
            "demo",
            "--scope",
            "app/**",
            "--required",
            "--check",
            "grep -q louder app/greeter.py",
            "say louder",
        ],
    ));
    assert!(added.contains("(required, check)"), "{added}");
    std::fs::write(
        ws.join("app/greeter.py"),
        "def hello():\n    return 'quiet'\n",
    )
    .unwrap();
    let o = av(&d, &ws, &["validate", "--", "true"]);
    assert!(!o.status.success(), "a failing check fails validate");
    assert!(out(&o).contains("check conv:1 FAILED"), "{}", out(&o));
    std::fs::write(
        ws.join("app/greeter.py"),
        "def hello():\n    return 'louder'\n",
    )
    .unwrap();
    let passed = ok(av(&d, &ws, &["validate", "--", "true"]));
    assert!(
        passed.contains("check conv:1 passed") && passed.contains("validation passed"),
        "{passed}"
    );
    ok(av(
        &d,
        scratch.path(),
        &["convention", "rm", "1", "--repo", "demo"],
    ));
    let list = ok(av(&d, &ws, &["convention", "list"]));
    assert!(list.trim().is_empty(), "{list}");
}

#[test]
fn runner_checks_main_and_log_shows_the_result() {
    let (d, _ws, scratch) = started();
    ok(av(
        &d,
        scratch.path(),
        &[
            "repo",
            "settings",
            "demo",
            "--check",
            "grep -q upper app/util.py",
        ],
    ));
    let refused = av(&d, scratch.path(), &["runner", "--repo", "demo", "--once"]);
    assert!(!refused.status.success(), "no --trust, no run");
    assert!(err(&refused).contains("--trust"), "{}", err(&refused));
    let work = scratch.path().join("runner-work");
    let ran = av(
        &d,
        scratch.path(),
        &[
            "runner",
            "--repo",
            "demo",
            "--trust",
            "--once",
            "--work-dir",
            work.to_str().unwrap(),
        ],
    );
    assert!(
        out(&ran).contains("FAILED"),
        "the fixture says title(), not upper(): {}",
        out(&ran)
    );
    let log = ok(av(&d, scratch.path(), &["log", "--repo", "demo"]));
    assert!(log.starts_with("✗ "), "{log}");
    let health = ok(av(&d, scratch.path(), &["health", "--repo", "demo"]));
    assert!(health.contains("red"), "{health}");
}

/// Starts a task as `dev`, writes `path`, and takes it through validate, review (by `rev`)
/// and merge. Returns the task id.
fn merge_change(d: &Daemon, scratch: &Path, title: &str, path: &str, text: &str) -> String {
    let started = ok(av_as(
        d,
        scratch,
        "dev",
        &[
            "task", "start", "--repo", "demo", "--title", title, "--goal", title,
        ],
    ));
    let ws = PathBuf::from(
        started
            .lines()
            .next()
            .unwrap()
            .trim_start_matches("workspace: "),
    );
    let id = started
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .to_owned();
    std::fs::write(ws.join(path), text).unwrap();
    ok(av_as(
        d,
        &ws,
        "dev",
        &["record", "--kind", "note", "--summary", title],
    ));
    ok(av_as(d, &ws, "dev", &["validate", "--", "true"]));
    ok(av_as(d, &ws, "dev", &["submit"]));
    ok(av_as(d, &ws, "rev", &["review", "--verdict", "approve"]));
    ok(av_as(d, &ws, "dev", &["merge"]));
    id
}

#[test]
fn bisect_names_the_merge_that_broke_main_and_revert_fixes_it() {
    let d = start_daemon();
    let scratch = TempDir::new().unwrap();
    let s = scratch.path();
    ok(av(&d, s, &["repo", "add", "demo", d.src.to_str().unwrap()]));
    ok(av(
        &d,
        s,
        &[
            "repo",
            "settings",
            "demo",
            "--check",
            "! grep -q BROKEN app/util.py",
        ],
    ));
    merge_change(&d, s, "First fine change", "app/one.py", "ONE = 1\n");
    let bad = merge_change(&d, s, "Breaking change", "app/util.py", "BROKEN = True\n");
    merge_change(&d, s, "Later fine change", "app/two.py", "TWO = 2\n");

    let work = s.join("bisect-work");
    let out = ok(av(
        &d,
        s,
        &[
            "bisect",
            "--repo",
            "demo",
            "--trust",
            "--work-dir",
            work.to_str().unwrap(),
        ],
    ));
    assert!(
        out.contains("first bad") && out.contains("breaking-change"),
        "{out}"
    );
    assert!(out.contains(&format!("av revert {bad}")), "{out}");

    let why = ok(av(&d, s, &["why", "app/util.py:1", "--repo", "demo"]));
    assert!(
        why.contains("breaking-change") && why.contains("BROKEN = True"),
        "{why}"
    );

    let reverted = ok(av_as(
        &d,
        s,
        "dev",
        &["revert", &bad, "--reason", "broke the check"],
    ));
    assert!(reverted.contains("reverted"), "{reverted}");
    ok(av(
        &d,
        s,
        &[
            "runner",
            "--repo",
            "demo",
            "--trust",
            "--once",
            "--work-dir",
            work.to_str().unwrap(),
        ],
    ));
    let health = ok(av(&d, s, &["health", "--repo", "demo"]));
    assert!(health.contains("green"), "{health}");
}

#[test]
fn expand_prints_full_text_for_a_pack_id() {
    let (d, ws, _scratch) = started();
    ok(av(
        &d,
        &ws,
        &[
            "record",
            "--kind",
            "decision",
            "--summary",
            "keep hello pure",
            "--body",
            "no IO inside hello",
        ],
    ));
    let md = ok(av(&d, &ws, &["context", "--budget", "3000"]));
    let start = md.find("rec:").unwrap();
    let id: String = md[start..]
        .chars()
        .take_while(|c| !c.is_whitespace())
        .collect();
    let full = ok(av(&d, &ws, &["expand", &id]));
    assert!(
        full.contains("Make hello louder")
            || full.contains("keep hello pure")
            || full.contains("no IO"),
        "{full}"
    );
}

#[test]
fn unreachable_server_exits_nonzero_with_clear_message() {
    let scratch = TempDir::new().unwrap();
    let o = av_with(
        "http://127.0.0.1:1",
        scratch.path(),
        &["repo", "add", "x", "/nowhere"],
    );
    assert!(!o.status.success());
    let e = err(&o);
    assert!(
        e.contains("cannot reach") && e.contains("http://127.0.0.1:1") && e.contains("AV_URL"),
        "{e}"
    );
}

#[test]
fn missing_task_gives_actionable_error() {
    let d = start_daemon();
    let scratch = TempDir::new().unwrap();
    let o = av(&d, scratch.path(), &["context"]);
    assert!(!o.status.success());
    assert!(
        err(&o).contains("no task") && err(&o).contains("av task start"),
        "{}",
        err(&o)
    );
}

#[test]
fn reindex_prints_counts_and_exits_zero() {
    let (d, _ws, scratch) = started();
    let o = av(&d, scratch.path(), &["reindex", "--repo", "demo"]);
    let stdout = ok(o.clone());
    assert!(
        stdout.contains("enqueued") && stdout.contains("upserted 0"),
        "{stdout}"
    );
    assert!(err(&o).contains("av: note: semantic"), "{}", err(&o));
}

#[test]
fn reindex_unknown_server_exits_nonzero() {
    let scratch = TempDir::new().unwrap();
    let o = av_with("http://127.0.0.1:1", scratch.path(), &["reindex"]);
    assert!(!o.status.success(), "{}", out(&o));
}

#[test]
fn context_reports_its_receipt_and_av_receipt_prints_the_manifest() {
    let (d, ws, _scratch) = started();
    let o = av(&d, &ws, &["context", "--budget", "2000", "--fresh"]);
    let stderr = err(&o);
    ok(o);
    let rcp = stderr
        .lines()
        .find_map(|l| l.strip_prefix("av: receipt "))
        .unwrap_or_else(|| panic!("no receipt line in stderr: {stderr}"))
        .trim()
        .to_owned();
    let shown = ok(av(&d, &ws, &["receipt", &rcp]));
    assert!(shown.contains(&rcp) && shown.contains("reason"), "{shown}");
    assert!(shown.contains("task"), "{shown}");
    let rec = ok(av(
        &d,
        &ws,
        &[
            "record",
            "--kind",
            "note",
            "--summary",
            "from pack",
            "--receipt",
            &rcp,
        ],
    ));
    assert!(rec.contains("rec:"), "{rec}");
}

fn av_as(d: &Daemon, cwd: &Path, agent: &str, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_av"))
        .current_dir(cwd)
        .env("AV_URL", &d.url)
        .env_remove("AV_TASK")
        .env_remove("AV_SESSION")
        .env_remove("AV_GENERATION")
        .env("AV_AGENT", agent)
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn handoff_then_claim_moves_the_workspace_to_the_new_agent() {
    let (d, ws, scratch) = started();
    std::fs::write(
        ws.join("app/greeter.py"),
        "class Greeter:\n    def hello(self, name):\n        return name + \"!\"\n",
    )
    .unwrap();
    // `av task start` ran as the default agent "agent"
    ok(av(
        &d,
        &ws,
        &[
            "record",
            "--kind",
            "decision",
            "--summary",
            "shout in hello",
            "--anchor",
            "app/greeter.py:Greeter.hello",
        ],
    ));
    let handed = ok(av(
        &d,
        &ws,
        &[
            "handoff",
            "--done",
            "hello shouts",
            "--remaining",
            "make goodbye louder",
        ],
    ));
    assert!(handed.contains("handed_off"), "{handed}");
    assert!(
        handed.contains("Remaining: make goodbye louder"),
        "{handed}"
    );
    let file: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(ws.join(".av-task")).unwrap()).unwrap();
    let (task, old_session) = (
        file["task"].as_str().unwrap().to_owned(),
        file["session"].clone(),
    );
    assert_eq!(file["generation"], 1, "{file}");

    let claimed = ok(av_as(&d, scratch.path(), "codex", &["claim", &task]));
    assert!(
        claimed.contains(&format!("workspace: {}", ws.display())),
        "{claimed}"
    );
    assert!(claimed.contains("generation: 2"), "{claimed}");
    assert!(
        claimed.contains("Remaining: make goodbye louder"),
        "{claimed}"
    );
    let file: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(ws.join(".av-task")).unwrap()).unwrap();
    assert_eq!(file["generation"], 2, "{file}");
    assert_eq!(file["agent"], "codex", "{file}");
    assert_ne!(file["session"], old_session, "a claim starts a new session");

    // the previous owner, still in the workspace, is fenced off; the claimant is not
    let stale = av(&d, &ws, &["checkpoint"]);
    assert!(!stale.status.success(), "{}", out(&stale));
    assert!(err(&stale).contains("claim lost"), "{}", err(&stale));
    ok(av_as(&d, &ws, "codex", &["checkpoint"]));
    let status = ok(av_as(&d, &ws, "codex", &["status"]));
    assert!(status.contains("owner codex · generation 2"), "{status}");
}

#[test]
fn bundle_prints_the_review_bundle_with_test_status() {
    let (d, ws, _scratch) = started();
    let pr = ok(av(&d, &ws, &["bundle", "--pr", "--budget", "1500"]));
    assert!(
        pr.contains("## Tests") && pr.contains("not run at this head"),
        "{pr}"
    );
    let handoff = ok(av(&d, &ws, &["bundle"]));
    assert!(
        handoff.contains("## Goal") && !handoff.contains("## Tests"),
        "{handoff}"
    );
}

#[test]
fn repo_backup_sets_a_remote_and_pushes_main() {
    let (d, ws, scratch) = started();
    let remote = TempDir::new().unwrap();
    git(remote.path(), &["init", "-q", "--bare", "-b", "main"]);
    let url = remote.path().to_str().unwrap();
    let set = ok(av(
        &d,
        scratch.path(),
        &["repo", "backup", "demo", "--set", url, "--push"],
    ));
    assert!(set.contains(&format!("backup: {url} · pushed ")), "{set}");
    let status = ok(av(&d, &ws, &["status"]));
    assert!(status.contains(&format!("backup: {url}")), "{status}");
    let off = ok(av(
        &d,
        scratch.path(),
        &["repo", "backup", "demo", "--clear"],
    ));
    assert_eq!(off.trim(), "backup: off");
}

#[test]
fn search_log_and_abandon() {
    let (d, ws, scratch) = started();
    ok(av(
        &d,
        &ws,
        &[
            "record",
            "--kind",
            "note",
            "--summary",
            "zebra stripes matter",
        ],
    ));
    let found = ok(av(&d, &ws, &["search", "zebra"]));
    assert!(found.contains("note: zebra stripes matter"), "{found}");
    let none = ok(av(
        &d,
        scratch.path(),
        &["search", "--repo", "demo", "giraffe"],
    ));
    assert!(none.contains("no records match"), "{none}");
    let log = ok(av(&d, &ws, &["log"]));
    assert_eq!(log.lines().count(), 1, "only the imported commit: {log}");
    assert_eq!(ok(av(&d, &ws, &["abandon"])).trim(), "abandoned");
    let again = av(&d, &ws, &["abandon"]);
    assert!(!again.status.success(), "{}", out(&again));
}

#[test]
fn hook_session_start_is_silent_without_task_file() {
    let d = start_daemon();
    let scratch = TempDir::new().unwrap();
    let o = av(&d, scratch.path(), &["hook", "session-start"]);
    assert!(o.status.success());
    assert_eq!((out(&o).as_str(), err(&o).as_str()), ("", ""));
    let o = av(&d, scratch.path(), &["hook", "post-edit"]);
    assert!(o.status.success());
    assert_eq!((out(&o).as_str(), err(&o).as_str()), ("", ""));
}

#[test]
fn hook_session_start_prints_a_small_pack() {
    let (d, ws, _scratch) = started();
    let pack = ok(av(&d, &ws, &["hook", "session-start"]));
    assert!(pack.contains("## Task") && pack.contains("/800"), "{pack}");
}

#[test]
fn hook_session_start_leaves_bodies_out_unless_asked_for_full_code() {
    let (d, ws, _scratch) = started();
    // a session that has been sent nothing yet, as for a new agent
    let mut tf: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(ws.join(".av-task")).unwrap()).unwrap();
    tf["session"] = "fresh-session".into();
    std::fs::write(ws.join(".av-task"), tf.to_string()).unwrap();
    // Claude Code reads a file before editing it, so a body in the pack would be sent twice
    let pack = ok(av(&d, &ws, &["hook", "session-start"]));
    assert!(pack.contains("def hello(self, name):"), "{pack}");
    assert!(!pack.contains("return name"), "{pack}");
    tf["session"] = "another-fresh-session".into();
    std::fs::write(ws.join(".av-task"), tf.to_string()).unwrap();
    let full = ok(av(&d, &ws, &["hook", "session-start", "--code", "full"]));
    assert!(full.contains("return name"), "{full}");
}

#[test]
fn hook_session_start_can_leave_out_the_map_and_cap_the_code_items() {
    let (d, ws, _scratch) = started();
    let mut tf: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(ws.join(".av-task")).unwrap()).unwrap();
    tf["session"] = "fresh-session".into();
    std::fs::write(ws.join(".av-task"), tf.to_string()).unwrap();
    let pack = ok(av(&d, &ws, &["hook", "session-start", "--budget", "3000"]));
    assert!(pack.contains("## Map"), "{pack}");
    assert!(pack.matches("- sym:").count() > 1, "{pack}");
    tf["session"] = "another-fresh-session".into();
    std::fs::write(ws.join(".av-task"), tf.to_string()).unwrap();
    let lean = ok(av(
        &d,
        &ws,
        &[
            "hook",
            "session-start",
            "--budget",
            "3000",
            "--no-map",
            "--max-code",
            "1",
        ],
    ));
    assert!(!lean.contains("## Map"), "{lean}");
    assert_eq!(lean.matches("- sym:").count(), 1, "{lean}");
}

#[test]
fn hook_post_edit_exits_zero_when_daemon_down() {
    let (_d, ws, _scratch) = started();
    let o = av_with("http://127.0.0.1:1", &ws, &["hook", "post-edit"]);
    assert!(o.status.success(), "{}", err(&o));
    assert_eq!(err(&o).lines().count(), 1, "one line: {}", err(&o));
    assert!(err(&o).contains("checkpoint failed"), "{}", err(&o));
}

#[test]
fn hook_post_edit_debounces_within_two_seconds() {
    let (d, ws, _scratch) = started();
    std::fs::write(ws.join("app/greeter.py"), "class Greeter:\n    pass\n").unwrap();
    ok(av(&d, &ws, &["hook", "post-edit"]));
    let status = ok(av(&d, &ws, &["status"]));
    assert!(status.contains("changed (1): app/greeter.py"), "{status}");
    std::fs::write(ws.join("README.md"), "# changed\n").unwrap();
    ok(av(&d, &ws, &["hook", "post-edit"]));
    let status = ok(av(&d, &ws, &["status"]));
    assert!(
        status.contains("changed (1):"),
        "second edit within 2 s is debounced: {status}"
    );
    std::thread::sleep(std::time::Duration::from_millis(2100));
    ok(av(&d, &ws, &["hook", "post-edit"]));
    let status = ok(av(&d, &ws, &["status"]));
    assert!(status.contains("changed (2):"), "{status}");
}

#[test]
fn hook_post_edit_finds_the_task_from_the_edited_file() {
    let (d, ws, scratch) = started();
    std::fs::write(ws.join("app/greeter.py"), "class Greeter:\n    pass\n").unwrap();
    let input = format!(
        r#"{{"cwd": "{}", "tool_input": {{"file_path": "{}"}}}}"#,
        scratch.path().display(),
        ws.join("app/greeter.py").display()
    );
    let mut child = Command::new(env!("CARGO_BIN_EXE_av"))
        .current_dir(scratch.path())
        .env("AV_URL", &d.url)
        .env_remove("AV_TASK")
        .env_remove("AV_AGENT")
        .args(["hook", "post-edit"])
        .stdin(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    use std::io::Write;
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    assert!(child.wait().unwrap().success());
    let status = ok(av(&d, &ws, &["status"]));
    assert!(status.contains("changed (1): app/greeter.py"), "{status}");
}

#[test]
fn cli_sends_av_token() {
    let d = start_daemon_with(Some("sekrit"));
    let scratch = TempDir::new().unwrap();
    let add = ["repo", "add", "demo", d.src.to_str().unwrap()];
    let denied = av(&d, scratch.path(), &add);
    assert!(!denied.status.success());
    assert!(err(&denied).contains("401"), "{}", err(&denied));
    let o = Command::new(env!("CARGO_BIN_EXE_av"))
        .current_dir(scratch.path())
        .env("AV_URL", &d.url)
        .env("AV_TOKEN", "sekrit")
        .args(add)
        .output()
        .unwrap();
    assert!(o.status.success(), "{}", err(&o));
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn install_sh_merges_both_hooks_and_is_idempotent() {
    if Command::new("jq").arg("--version").output().is_err() {
        eprintln!("jq not installed; skipping");
        return;
    }
    let home = TempDir::new().unwrap();
    let claude = home.path().join(".claude");
    std::fs::create_dir_all(&claude).unwrap();
    std::fs::write(
        claude.join("settings.json"),
        r#"{"model": "keep-me", "hooks": {"PostToolUse": [
             {"matcher": "Bash", "hooks": [{"type": "command", "command": "echo mine"}]}]}}"#,
    )
    .unwrap();
    let script = repo_root().join("integrations/claude-code/install.sh");
    for _ in 0..2 {
        let o = Command::new("bash")
            .arg(&script)
            .env("HOME", home.path())
            .output()
            .unwrap();
        assert!(o.status.success(), "{}", err(&o));
    }
    let settings: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(claude.join("settings.json")).unwrap())
            .unwrap();
    assert_eq!(settings["model"], "keep-me");
    let post = settings["hooks"]["PostToolUse"].as_array().unwrap();
    assert_eq!(post.len(), 2, "theirs plus ours, once: {settings}");
    assert_eq!(post[0]["hooks"][0]["command"], "echo mine");
    assert_eq!(post[1]["matcher"], "Edit|Write|MultiEdit");
    let ours = post[1]["hooks"][0]["command"].as_str().unwrap();
    assert!(Path::new(ours).is_file(), "{ours}");
    let start = settings["hooks"]["SessionStart"].as_array().unwrap();
    assert_eq!(start.len(), 1, "{settings}");
    assert!(
        start[0]["hooks"][0]["command"]
            .as_str()
            .unwrap()
            .ends_with("/av-hooks/session-start.sh")
    );
    assert!(claude.join("settings.json.bak").is_file());
}

#[test]
fn every_integration_script_parses() {
    for script in [
        "integrations/claude-code/install.sh",
        "integrations/claude-code/hooks/session-start.sh",
        "integrations/claude-code/hooks/post-edit.sh",
        "integrations/generic/av-wrap.sh",
    ] {
        let o = Command::new("bash")
            .arg("-n")
            .arg(repo_root().join(script))
            .output()
            .unwrap();
        assert!(o.status.success(), "{script}: {}", err(&o));
    }
}

#[test]
fn av_wrap_writes_context_runs_the_command_and_checkpoints() {
    let (d, ws, _scratch) = started();
    let bin = Path::new(env!("CARGO_BIN_EXE_av"))
        .parent()
        .unwrap()
        .to_owned();
    let path = format!("{}:{}", bin.display(), std::env::var("PATH").unwrap());
    let o = Command::new("bash")
        .arg(repo_root().join("integrations/generic/av-wrap.sh"))
        .args([
            "sh",
            "-c",
            r#"grep -q '## Task' "$AV_CONTEXT_FILE" && echo wrapped >> README.md && exit 3"#,
        ])
        .current_dir(&ws)
        .env("PATH", path)
        .env("AV_URL", &d.url)
        .env_remove("AV_TASK")
        .env_remove("AV_AGENT")
        .output()
        .unwrap();
    assert_eq!(
        o.status.code(),
        Some(3),
        "the command's status: {}",
        err(&o)
    );
    let status = ok(av(&d, &ws, &["status"]));
    assert!(
        status.contains("README.md"),
        "checkpointed after the command: {status}"
    );
}

#[test]
fn hook_session_start_after_compaction_starts_a_new_session() {
    let (d, ws, _scratch) = started();
    let session = |ws: &Path| -> String {
        let f: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(ws.join(".av-task")).unwrap()).unwrap();
        f["session"].as_str().unwrap().to_owned()
    };
    let before = session(&ws);
    let run = |source: &str| {
        let mut child = Command::new(env!("CARGO_BIN_EXE_av"))
            .current_dir(&ws)
            .env("AV_URL", &d.url)
            .env_remove("AV_SESSION")
            .args(["hook", "session-start"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        use std::io::Write;
        let input = format!(r#"{{"source": "{source}"}}"#);
        child
            .stdin
            .take()
            .unwrap()
            .write_all(input.as_bytes())
            .unwrap();
        let o = child.wait_with_output().unwrap();
        assert!(o.status.success());
        String::from_utf8_lossy(&o.stdout).into_owned()
    };
    run("resume");
    assert_eq!(session(&ws), before, "a resumed session keeps its ledger");
    let pack = run("compact");
    assert_ne!(
        session(&ws),
        before,
        "after compaction everything is sent again"
    );
    assert!(pack.contains("## Task"), "{pack}");
}

fn av_stdin(d: &Daemon, cwd: &Path, args: &[&str], stdin: &str) -> Output {
    use std::io::Write as _;
    let mut child = Command::new(env!("CARGO_BIN_EXE_av"))
        .current_dir(cwd)
        .env("AV_URL", &d.url)
        .env_remove("AV_TASK")
        .env_remove("AV_SESSION")
        .env_remove("AV_AGENT")
        .env_remove("AV_TOKEN")
        .args(args)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(stdin.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn hook_pre_read_adds_what_other_tasks_recorded_about_the_file() {
    let (d, ws, scratch) = started();
    ok(av(
        &d,
        scratch.path(),
        &[
            "task",
            "start",
            "--repo",
            "demo",
            "--title",
            "Earlier try",
            "--goal",
            "Cache names",
        ],
    ));
    let other = d.ws.join("demo/earlier-try");
    ok(av(
        &d,
        &other,
        &[
            "record",
            "--kind",
            "attempt",
            "--summary",
            "tried caching hello; greetings went stale",
            "--anchor",
            "app/greeter.py",
        ],
    ));
    let file = ws.join("app/greeter.py");
    let input = serde_json::json!({
        "hook_event_name": "PreToolUse", "tool_name": "Read",
        "tool_input": {"file_path": file}, "cwd": ws,
    })
    .to_string();
    let o = av_stdin(&d, &ws, &["hook", "pre-read"], &input);
    let v: serde_json::Value = serde_json::from_str(&ok(o)).unwrap();
    assert_eq!(v["hookSpecificOutput"]["hookEventName"], "PreToolUse");
    let ctx = v["hookSpecificOutput"]["additionalContext"]
        .as_str()
        .unwrap();
    assert!(ctx.contains("greetings went stale"), "{ctx}");
    // once per session: the same read again adds nothing
    let o = av_stdin(&d, &ws, &["hook", "pre-read"], &input);
    assert_eq!((ok(o).as_str()), "");
    // a file nobody recorded anything about adds nothing either
    let input =
        serde_json::json!({"tool_input": {"file_path": ws.join("app/util.py")}}).to_string();
    let o = av_stdin(&d, &ws, &["hook", "pre-read"], &input);
    assert_eq!((ok(o).as_str()), "");
}

#[test]
fn hook_pre_read_is_silent_outside_a_task_and_when_the_daemon_is_down() {
    let d = start_daemon();
    let scratch = TempDir::new().unwrap();
    let input =
        serde_json::json!({"tool_input": {"file_path": scratch.path().join("x.py")}}).to_string();
    let o = av_stdin(&d, scratch.path(), &["hook", "pre-read"], &input);
    assert!(o.status.success());
    assert_eq!((out(&o).as_str(), err(&o).as_str()), ("", ""));
    drop(d);
    let (_d, ws, _scratch) = started();
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_av"));
    let o = cmd
        .current_dir(&ws)
        .env("AV_URL", "http://127.0.0.1:1")
        .args(["hook", "pre-read"])
        .stdin(std::process::Stdio::null())
        .output()
        .unwrap();
    assert!(o.status.success(), "{}", err(&o));
    assert_eq!(out(&o), "");
}
