mod common;

use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread::JoinHandle;

use common::*;
use scholion_bench::runner::{Arm, RunConfig, run_session};
use scholion_bench::spec::{TaskSpec, load_spec};
use scholion_daemon::{AppState, router};
use scholion_index::PgIndex;
use scholion_store::PgStore;
use scholion_testutil::TestDb;
use scholion_vcs::JjVcs;
use tempfile::TempDir;
use tokio::sync::oneshot;

/// A fake `claude`: checks its working directory, runs the SessionStart and PostToolUse hooks
/// from `--settings`, edits `calc.py` (unless FAKE_MODE=nofix) and prints a canned transcript.
const FAKE_CLAUDE: &str = r#"#!/bin/sh
echo "$PWD" >> "$FAKE_MARKER"
settings=""; mcp=""
while [ $# -gt 0 ]; do
  case "$1" in --settings) settings="$2"; shift;; --mcp-config) mcp="$2"; shift;; esac
  shift
done
if [ -n "$FAKE_EXPECT_AV" ]; then
  [ -f .av-task ] || { echo "no .av-task in $PWD" >&2; exit 9; }
  grep -q SessionStart "$settings" || { echo "no hooks" >&2; exit 9; }
  grep -q X-Av-Task "$mcp" || { echo "no av mcp" >&2; exit 9; }
fi
hook() {
  python3 -c "import json,sys; h=json.load(open(sys.argv[1])).get('hooks',{}).get(sys.argv[2]); print(h[0]['hooks'][0]['command'] if h else '')" "$settings" "$1"
}
if [ "$FAKE_MODE" != nohooks ]; then
  cmd=$(hook SessionStart)
  [ -n "$cmd" ] && echo "{\"source\":\"startup\",\"cwd\":\"$PWD\"}" | sh -c "$cmd" > "$FAKE_MARKER.pack"
fi
[ "$FAKE_MODE" = sleep ] && sleep 30
if [ "$FAKE_MODE" != nofix ]; then
  printf 'def add(a, b):\n    return a + b\n' > calc.py
  if [ "$FAKE_MODE" != nohooks ]; then
    cmd=$(hook PostToolUse)
    [ -n "$cmd" ] && echo "{\"tool_input\":{\"file_path\":\"$PWD/calc.py\"}}" | sh -c "$cmd"
  fi
fi
echo "{\"type\":\"system\",\"subtype\":\"init\",\"cwd\":\"$PWD\"}"
echo "{\"type\":\"assistant\",\"message\":{\"id\":\"m1\",\"content\":[{\"type\":\"tool_use\",\"id\":\"t1\",\"name\":\"Edit\",\"input\":{\"file_path\":\"$PWD/calc.py\",\"old_string\":\"a - b\",\"new_string\":\"a + b\"}}],\"usage\":{\"input_tokens\":10,\"cache_creation_input_tokens\":100,\"cache_read_input_tokens\":50,\"output_tokens\":20}}}"
echo "{\"type\":\"result\",\"subtype\":\"success\",\"is_error\":false,\"num_turns\":1,\"total_cost_usd\":${FAKE_COST:-0.25},\"usage\":{\"input_tokens\":10,\"cache_creation_input_tokens\":100,\"cache_read_input_tokens\":50,\"output_tokens\":20}}"
"#;

/// A fake `av` for the hooks: prints a pack for session-start, nothing for post-edit.
const FAKE_AV: &str = "#!/bin/sh\ncat > /dev/null\n[ \"$2\" = session-start ] && echo '## Task (fake pack)'\nexit 0\n";

pub fn script(dir: &Path, name: &str, body: &str) -> PathBuf {
    let p = dir.join(name);
    std::fs::write(&p, body).unwrap();
    std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o755)).unwrap();
    p
}

struct Fixture {
    _repo: TempDir,
    _tools: TempDir,
    _work: TempDir,
    spec: TaskSpec,
    fix: String,
    cfg: RunConfig,
    marker: PathBuf,
}

fn fixture(mode: &str) -> Fixture {
    let repo = TempDir::new().unwrap();
    let (base, fix) = fixture_repo(repo.path());
    let specs = TempDir::new().unwrap();
    let p = specs.path().join("calc-task.toml");
    std::fs::write(&p, spec_toml(repo.path(), &base, &fix)).unwrap();
    let spec = load_spec(&p).unwrap();
    let tools = TempDir::new().unwrap();
    let claude = script(tools.path(), "claude", FAKE_CLAUDE);
    let av = script(tools.path(), "av", FAKE_AV);
    let work = TempDir::new().unwrap();
    let marker = tools.path().join("invoked");
    // SAFETY: the runner tests hold SERIAL while the fake scripts read these.
    unsafe {
        std::env::set_var("FAKE_MODE", mode);
        std::env::set_var("FAKE_MARKER", &marker);
    }
    let cfg = RunConfig {
        work_root: work.path().to_path_buf(),
        claude_bin: claude.to_string_lossy().into(),
        av_bin: av.to_string_lossy().into(),
        av_url: None,
        av_token: None,
        import_dir: work.path().join("src"),
        import_mount: work.path().join("src").to_string_lossy().into(),
        allowed_tools: vec!["Read".into(), "Edit".into()],
        permission_mode: "acceptEdits".into(),
        session_cap_usd: 1.0,
        claude_home: None,
        hook_budget: 800,
        allow_no_semantic: true,
        index_wait_secs: 60,
    };
    Fixture {
        _repo: repo,
        _tools: tools,
        _work: work,
        spec,
        fix,
        cfg,
        marker,
    }
}

fn repo_state(repo: &Path) -> (String, String, String) {
    (
        git(repo, &["for-each-ref"]),
        git(repo, &["status", "--porcelain", "--ignored"]),
        git(repo, &["rev-parse", "HEAD"]),
    )
}

// The fake scripts read FAKE_* from the environment, so these tests run one at a time.
static SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn serial() -> std::sync::MutexGuard<'static, ()> {
    SERIAL.lock().unwrap_or_else(|e| e.into_inner())
}

#[test]
fn baseline_run_checks_out_base_rev_and_stores_events() {
    let _g = serial();
    let f = fixture("fix");
    let before = repo_state(&f.spec.repo_path);
    let o = run_session(&f.spec, Arm::Baseline, 1, "t1", &f.cfg).unwrap();
    assert!(o.valid, "{o:?}");
    assert_eq!(o.verify_exit, Some(0), "{o:?}");
    assert!(o.success());
    let events = std::fs::read_to_string(&o.events_path).unwrap();
    assert!(events.contains(r#""type":"result""#), "{events}");
    let ws = o.run_dir.join("ws");
    let ran_in = std::fs::read_to_string(&f.marker).unwrap();
    assert_eq!(
        Path::new(ran_in.trim()).canonicalize().unwrap(),
        ws.canonicalize().unwrap()
    );
    // a single commit, no remote, no way to the real fix
    assert_eq!(git(&ws, &["rev-list", "--all", "--count"]), "1");
    assert_eq!(git(&ws, &["remote"]), "");
    let reach = std::process::Command::new("git")
        .arg("-C")
        .arg(&ws)
        .args(["cat-file", "-e", &f.fix])
        .status()
        .unwrap();
    assert!(!reach.success(), "the fix commit must not be in the copy");
    assert_eq!(
        repo_state(&f.spec.repo_path),
        before,
        "source repo untouched"
    );
}

#[test]
fn verify_exit_code_recorded() {
    let _g = serial();
    let f = fixture("nofix");
    let o = run_session(&f.spec, Arm::Baseline, 1, "t2", &f.cfg).unwrap();
    assert!(o.valid);
    assert_eq!(o.verify_exit, Some(1), "{o:?}");
    assert!(!o.success());
}

#[test]
fn verify_timeout_kills_command_and_counts_failure() {
    let _g = serial();
    let mut f = fixture("fix");
    f.spec.verify_cmd = "sleep 30".into();
    f.spec.verify_timeout_secs = 1;
    let start = std::time::Instant::now();
    let o = run_session(&f.spec, Arm::Baseline, 1, "t3", &f.cfg).unwrap();
    assert!(start.elapsed().as_secs() < 15);
    assert!(o.verify_timed_out);
    assert_eq!(o.verify_exit, None);
    assert!(!o.success());
}

#[test]
fn session_timeout_kills_claude_and_leaves_a_truncated_stream() {
    let _g = serial();
    let mut f = fixture("sleep");
    f.spec.session_timeout_secs = 1;
    let start = std::time::Instant::now();
    let o = run_session(&f.spec, Arm::Baseline, 1, "t4", &f.cfg).unwrap();
    assert!(start.elapsed().as_secs() < 15);
    assert!(o.claude_killed);
    let events = std::fs::read_to_string(&o.events_path).unwrap();
    assert!(!events.contains("result"), "{events}");
}

#[test]
fn av_arm_aborts_before_spending_when_daemon_down() {
    let _g = serial();
    let mut f = fixture("fix");
    f.cfg.av_url = Some("http://127.0.0.1:9".into());
    let err = run_session(&f.spec, Arm::Av, 1, "t5", &f.cfg).unwrap_err();
    assert!(err.contains("not up"), "{err}");
    assert!(!f.marker.exists(), "claude must never be invoked");
}

/// A real daemon on an ephemeral port.
struct Daemon {
    url: String,
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

fn start_daemon(token: Option<&str>) -> Daemon {
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
    Daemon {
        url,
        shutdown: Some(shutdown_tx),
        thread: Some(thread),
    }
}

fn get(url: &str) -> serde_json::Value {
    ureq::get(url).call().unwrap().into_json().unwrap()
}

fn task_of(ws: &Path) -> String {
    let tf: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(ws.join(".av-task")).unwrap()).unwrap();
    tf["task"].as_str().unwrap().to_owned()
}

#[test]
fn av_arm_runs_in_the_task_workspace_with_hooks_mcp_and_seeded_dead_ends() {
    let _g = serial();
    let d = start_daemon(None);
    let mut f = fixture("fix");
    f.cfg.av_url = Some(d.url.clone());
    // SAFETY: SERIAL is held.
    unsafe { std::env::set_var("FAKE_EXPECT_AV", "1") };
    let before = repo_state(&f.spec.repo_path);
    let o = run_session(&f.spec, Arm::Av, 1, "t6", &f.cfg);
    unsafe { std::env::remove_var("FAKE_EXPECT_AV") };
    let o = o.unwrap();
    assert!(o.valid, "{o:?}");
    assert_eq!(
        o.claude_exit,
        Some(0),
        "the fake claude checks .av-task, hooks and MCP: {}",
        std::fs::read_to_string(o.run_dir.join("claude.stderr")).unwrap()
    );
    assert_eq!(o.verify_exit, Some(0), "{o:?}");
    let ws = PathBuf::from(std::fs::read_to_string(&f.marker).unwrap().trim());
    assert!(
        !ws.join(".git").exists(),
        "a jj workspace, not a git checkout"
    );
    let pack = std::fs::read_to_string(f.marker.with_extension("pack")).unwrap();
    assert!(pack.contains("## Task"), "{pack}");
    let fired = std::fs::read_to_string(o.run_dir.join("hook-count")).unwrap();
    assert_eq!(
        fired.lines().count(),
        2,
        "session-start and post-edit: {fired}"
    );
    let task = task_of(&ws);
    // the agent's session has been sent nothing yet: its first pack is in full
    let tf: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(ws.join(".av-task")).unwrap()).unwrap();
    let first = get(&format!(
        "{}/v1/tasks/{task}/context?session={}",
        d.url,
        tf["session"].as_str().unwrap()
    ));
    let md = first["markdown"].as_str().unwrap();
    assert!(
        !md.contains("sent earlier") && md.contains("Swapping"),
        "{md}"
    );
    let t = get(&format!("{}/v1/tasks/{task}", d.url));
    assert_eq!(t["task"]["pack_mode"], "full");
    assert_eq!(t["changed"], serde_json::json!(["calc.py"]), "{t}");
    let repo = t["task"]["repo"].as_str().unwrap();
    let hits = get(&format!("{}/v1/repos/{repo}/search?q=swapping", d.url));
    assert_eq!(hits["records"][0]["kind"], "attempt", "{hits}");
    let mcp = std::fs::read_to_string(o.run_dir.join("mcp.json")).unwrap();
    assert!(mcp.contains(&task) && mcp.contains("/mcp"), "{mcp}");
    assert!(o.run_dir.join("pack.md").exists());
    assert_eq!(
        repo_state(&f.spec.repo_path),
        before,
        "source repo untouched"
    );
}

#[test]
fn av_no_packs_arm_starts_a_task_only_task() {
    let _g = serial();
    let d = start_daemon(None);
    let mut f = fixture("fix");
    f.cfg.av_url = Some(d.url.clone());
    let o = run_session(&f.spec, Arm::AvNoPacks, 1, "t7", &f.cfg).unwrap();
    assert!(o.valid, "{o:?}");
    let ws = PathBuf::from(std::fs::read_to_string(&f.marker).unwrap().trim());
    let t = get(&format!("{}/v1/tasks/{}", d.url, task_of(&ws)));
    assert_eq!(t["task"]["pack_mode"], "task_only", "{t}");
    let pack = std::fs::read_to_string(o.run_dir.join("pack.md")).unwrap();
    assert!(
        pack.contains("## Task") && !pack.contains("## Map"),
        "{pack}"
    );
}

#[test]
fn av_arm_without_hook_counter_is_marked_invalid() {
    let _g = serial();
    let d = start_daemon(None);
    let mut f = fixture("nohooks");
    f.cfg.av_url = Some(d.url.clone());
    let o = run_session(&f.spec, Arm::Av, 1, "t8", &f.cfg).unwrap();
    assert!(!o.valid);
    assert!(o.invalid_reason.unwrap().contains("hooks never fired"));
}

#[test]
fn av_arms_refuse_a_daemon_without_semantic_search_by_default() {
    let _g = serial();
    let d = start_daemon(None);
    let mut f = fixture("fix");
    f.cfg.av_url = Some(d.url.clone());
    f.cfg.allow_no_semantic = false;
    let err = run_session(&f.spec, Arm::Av, 1, "t10", &f.cfg).unwrap_err();
    assert!(err.contains("semantic"), "{err}");
    assert!(!f.marker.exists());
}

#[test]
fn a_wrong_token_aborts_before_spending() {
    let _g = serial();
    let d = start_daemon(Some("right"));
    let mut f = fixture("fix");
    f.cfg.av_url = Some(d.url.clone());
    f.cfg.av_token = Some("wrong".into());
    let err = run_session(&f.spec, Arm::Av, 1, "t9", &f.cfg).unwrap_err();
    assert!(err.contains("401"), "{err}");
    assert!(!f.marker.exists());
}
