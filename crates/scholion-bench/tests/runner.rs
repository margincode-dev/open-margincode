mod common;

use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use common::*;
use scholion_bench::runner::{Arm, RunConfig, run_session};
use scholion_bench::spec::{TaskSpec, load_spec};
use tempfile::TempDir;

/// A fake `claude`: checks its working directory, runs the SessionStart and PostToolUse hooks
/// from `--settings`, edits `calc.py` (unless FAKE_MODE=nofix) and prints a canned transcript.
const FAKE_CLAUDE: &str = r#"#!/bin/sh
echo "$PWD" >> "$FAKE_MARKER"
echo "$PATH" > "$FAKE_MARKER.path"
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
        av_mcp: true,
        hook_code: None,
        hook_args: vec![],
        jit_hooks: false,
        prices: None,
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
fn av_arm_without_mcp_runs_on_hooks_alone() {
    let _g = serial();
    let d = start_daemon(None);
    let mut f = fixture("fix");
    f.cfg.av_url = Some(d.url.clone());
    f.cfg.av_mcp = false;
    let o = run_session(&f.spec, Arm::Av, 1, "t11", &f.cfg).unwrap();
    assert!(o.valid, "{o:?}");
    assert_eq!(o.verify_exit, Some(0), "{o:?}");
    let mcp: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(o.run_dir.join("mcp.json")).unwrap())
            .unwrap();
    assert_eq!(mcp, serde_json::json!({"mcpServers": {}}));
    let fired = std::fs::read_to_string(o.run_dir.join("hook-count")).unwrap();
    assert_eq!(fired.lines().count(), 2, "{fired}");
}

#[test]
fn hook_code_is_passed_to_the_session_start_hook() {
    let _g = serial();
    let d = start_daemon(None);
    let mut f = fixture("fix");
    f.cfg.av_url = Some(d.url.clone());
    f.cfg.hook_code = Some("signatures".into());
    let o = run_session(&f.spec, Arm::Av, 1, "t12", &f.cfg).unwrap();
    let settings = std::fs::read_to_string(o.run_dir.join("settings.json")).unwrap();
    assert!(
        settings.contains("session-start --budget 800 --code 'signatures'"),
        "{settings}"
    );
}

#[test]
fn hook_args_are_passed_to_the_session_start_hook() {
    let _g = serial();
    let d = start_daemon(None);
    let mut f = fixture("fix");
    f.cfg.av_url = Some(d.url.clone());
    f.cfg.hook_args = vec!["--no-map".into(), "--max-code=3".into()];
    let o = run_session(&f.spec, Arm::Av, 1, "t13", &f.cfg).unwrap();
    assert!(o.valid, "the hook accepts them: {o:?}");
    let settings = std::fs::read_to_string(o.run_dir.join("settings.json")).unwrap();
    assert!(
        settings.contains("session-start --budget 800 '--no-map' '--max-code=3'"),
        "{settings}"
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

// ---- E4 knowledge relay: what an earlier agent learned reaches each arm by its own channel ----

const RELAY_NOTES: &str = r#"
[[note]]
pair = "calc-add"
kind = "attempt"
path = "calc.py"
symbol = "add"
summary = "Swapping the operands of the subtraction does not fix add(): b - a is still a subtraction."
body = "Tried it first; the sign flips but the sum is still wrong."

[[note]]
pair = "other"
kind = "decision"
path = "README.md"
summary = "The README stays one line long."
body = "Decided with the maintainers."
"#;

fn relay_fixture(mode: &str) -> Fixture {
    let mut f = fixture(mode);
    let dir = TempDir::new().unwrap();
    std::fs::write(dir.path().join("notes.toml"), RELAY_NOTES).unwrap();
    let p = dir.path().join("calc-task.toml");
    let text = spec_toml(&f.spec.repo_path, &f.spec.base_rev, &f.fix).replace(
        "setup_cmd = \"\"\n",
        "setup_cmd = \"\"\nrelay_pool = \"notes.toml\"\nrelay_pair = \"calc-add\"\n",
    );
    std::fs::write(&p, text).unwrap();
    f.spec = load_spec(&p).unwrap();
    f
}

fn log_messages(ws: &Path) -> String {
    git(ws, &["log", "--format=%B"])
}

#[test]
fn relay_notes_are_commit_messages_in_the_git_arm() {
    let _g = serial();
    let f = relay_fixture("fix");
    assert_eq!(f.spec.relay_notes.len(), 2);
    let o = run_session(&f.spec, Arm::Baseline, 1, "r1", &f.cfg).unwrap();
    assert!(o.valid, "{o:?}");
    let ws = o.run_dir.join("ws");
    let log = log_messages(&ws);
    assert!(log.contains("Swapping the operands"), "{log}");
    assert!(log.contains("Tried it first"), "the body too: {log}");
    assert!(log.contains("README stays one line"), "{log}");
    assert_eq!(
        git(&ws, &["rev-list", "--all", "--count"]),
        "3",
        "base + one per note"
    );
    assert!(!ws.join("NOTES.md").exists());
    assert_eq!(o.verify_exit, Some(0), "the notes change no file: {o:?}");
}

#[test]
fn the_notes_arm_also_gets_every_note_in_notes_md() {
    let _g = serial();
    let f = relay_fixture("fix");
    let o = run_session(&f.spec, Arm::Notes, 1, "r2", &f.cfg).unwrap();
    assert!(o.valid, "{o:?}");
    let ws = o.run_dir.join("ws");
    let notes = std::fs::read_to_string(ws.join("NOTES.md")).unwrap();
    for s in [
        "Swapping the operands",
        "Tried it first",
        "README stays one line",
        "calc.py",
    ] {
        assert!(notes.contains(s), "{s}: {notes}");
    }
    let claude_md = std::fs::read_to_string(ws.join("CLAUDE.md")).unwrap();
    assert!(claude_md.contains("NOTES.md"), "{claude_md}");
    assert!(log_messages(&ws).contains("Swapping the operands"));
    assert!(!Arm::Notes.uses_av());
    assert_eq!(Arm::parse("notes").unwrap(), Arm::Notes);
}

const RELAY_NOTES_WITH_NOISE: &str = r#"
[[note]]
pair = "calc-add"
kind = "attempt"
path = "calc.py"
symbol = "add"
summary = "Swapping the operands of the subtraction does not fix add(): b - a is still a subtraction."
body = "Tried it first; the sign flips but the sum is still wrong."

[[note]]
pair = "noise-0001"
kind = "note"
path = "other.py"
symbol = "helper"
summary = "helper is not thread-safe; each worker needs its own instance."
body = "Documented after a flake under pytest-xdist."
noise = true
"#;

#[test]
fn noise_notes_fill_notes_md_but_skip_git_commits() {
    let _g = serial();
    let mut f = fixture("fix");
    let dir = TempDir::new().unwrap();
    std::fs::write(dir.path().join("notes.toml"), RELAY_NOTES_WITH_NOISE).unwrap();
    let p = dir.path().join("calc-task.toml");
    let text = spec_toml(&f.spec.repo_path, &f.spec.base_rev, &f.fix).replace(
        "setup_cmd = \"\"\n",
        "setup_cmd = \"\"\nrelay_pool = \"notes.toml\"\nrelay_pair = \"calc-add\"\n",
    );
    std::fs::write(&p, text).unwrap();
    f.spec = load_spec(&p).unwrap();
    assert_eq!(f.spec.relay_notes.len(), 2);
    assert!(f.spec.relay_notes.iter().any(|n| n.noise));

    let o = run_session(&f.spec, Arm::Notes, 1, "rn", &f.cfg).unwrap();
    assert!(o.valid, "{o:?}");
    let ws = o.run_dir.join("ws");
    let notes = std::fs::read_to_string(ws.join("NOTES.md")).unwrap();
    assert!(notes.contains("Swapping the operands"), "{notes}");
    assert!(
        notes.contains("not thread-safe"),
        "noise in NOTES.md: {notes}"
    );
    let log = log_messages(&ws);
    assert!(log.contains("Swapping the operands"), "{log}");
    assert!(
        !log.contains("not thread-safe"),
        "noise must not be its own commit: {log}"
    );
    assert_eq!(
        git(&ws, &["rev-list", "--all", "--count"]),
        "3",
        "base + signal commit + NOTES.md commit (no noise commit)"
    );
}

#[test]
fn the_cold_arm_gets_nothing_the_earlier_agent_learned() {
    let _g = serial();
    let f = relay_fixture("fix");
    let o = run_session(&f.spec, Arm::Cold, 1, "r3", &f.cfg).unwrap();
    assert!(o.valid, "{o:?}");
    let ws = o.run_dir.join("ws");
    let log = log_messages(&ws);
    assert!(!log.contains("Swapping the operands"), "{log}");
    assert_eq!(
        git(&ws, &["rev-list", "--all", "--count"]),
        "1",
        "base only"
    );
    assert!(!ws.join("NOTES.md").exists());
    assert!(!Arm::Cold.uses_av());
    assert_eq!(Arm::parse("cold").unwrap(), Arm::Cold);
}

#[test]
fn the_av_arm_gets_the_notes_as_an_earlier_tasks_anchored_records() {
    let _g = serial();
    let d = start_daemon(None);
    let mut f = relay_fixture("fix");
    f.cfg.av_url = Some(d.url.clone());
    let o = run_session(&f.spec, Arm::Av, 1, "r3", &f.cfg).unwrap();
    assert!(o.valid, "{o:?}");
    let ws = PathBuf::from(std::fs::read_to_string(&f.marker).unwrap().trim());
    let task = task_of(&ws);
    let t = get(&format!("{}/v1/tasks/{task}", d.url));
    let repo = t["task"]["repo"].as_str().unwrap();
    let hits = get(&format!("{}/v1/repos/{repo}/search?q=swapping", d.url));
    let rec = &hits["records"][0];
    assert_eq!(rec["kind"], "attempt", "{hits}");
    assert_ne!(
        rec["task_id"].as_str().unwrap(),
        task,
        "another task's record: {hits}"
    );
    let own = get(&format!("{}/v1/tasks/{task}/records", d.url));
    assert!(
        !own.to_string().contains("Swapping"),
        "the [[dead_end]] is not seeded on top of the relay notes: {own}"
    );
    let all = get(&format!("{}/v1/repos/{repo}/search?q=README", d.url));
    assert!(all.to_string().contains("one line long"), "{all}");
}

#[test]
fn jit_hooks_add_pre_read_and_stop_to_the_av_arms() {
    let _g = serial();
    let d = start_daemon(None);
    let mut f = fixture("fix");
    f.cfg.av_url = Some(d.url.clone());
    f.cfg.jit_hooks = true;
    let o = run_session(&f.spec, Arm::Av, 1, "r4", &f.cfg).unwrap();
    let s = std::fs::read_to_string(o.run_dir.join("settings.json")).unwrap();
    let v: serde_json::Value = serde_json::from_str(&s).unwrap();
    let pre = v["hooks"]["PreToolUse"][0]["hooks"][0]["command"]
        .as_str()
        .unwrap();
    assert!(pre.contains("hook pre-read"), "{s}");
    assert_eq!(
        v["hooks"]["PreToolUse"][0]["matcher"],
        "Read|Edit|MultiEdit"
    );
    let stop = v["hooks"]["Stop"][0]["hooks"][0]["command"]
        .as_str()
        .unwrap();
    assert!(stop.contains("hook stop"), "{s}");
    f.cfg.jit_hooks = false;
    let o = run_session(&f.spec, Arm::Av, 2, "r4", &f.cfg).unwrap();
    let s = std::fs::read_to_string(o.run_dir.join("settings.json")).unwrap();
    assert!(!s.contains("PreToolUse") && !s.contains("\"Stop\""), "{s}");
}

#[test]
fn priced_runs_do_not_pass_claudes_own_budget_cap() {
    let _g = serial();
    let mut f = fixture("fix");
    let args = f.marker.with_extension("args");
    let claude = script(
        f._tools.path(),
        "claude-args",
        &FAKE_CLAUDE.replace(
            "echo \"$PWD\" >> \"$FAKE_MARKER\"\n",
            &format!(
                "echo \"$PWD\" >> \"$FAKE_MARKER\"\necho \"$@\" > {}\n",
                args.display()
            ),
        ),
    );
    f.cfg.claude_bin = claude.to_string_lossy().into();
    f.cfg.prices = Some(scholion_bench::metrics::Prices::parse("0.3,0.3,0.03,1.2").unwrap());
    run_session(&f.spec, Arm::Baseline, 1, "r5", &f.cfg).unwrap();
    let a = std::fs::read_to_string(&args).unwrap();
    assert!(!a.contains("--max-budget-usd"), "{a}");
}

/// `f` with its spec reloaded so it prepends `dirs` to the agent's PATH.
fn with_path_prepend(mut f: Fixture, dirs: &[&Path]) -> Fixture {
    let dir = TempDir::new().unwrap();
    let p = dir.path().join("calc-task.toml");
    let list: Vec<String> = dirs
        .iter()
        .map(|d| format!("{:?}", d.display().to_string()))
        .collect();
    let text = spec_toml(&f.spec.repo_path, &f.spec.base_rev, &f.fix).replace(
        "setup_cmd = \"\"\n",
        &format!("setup_cmd = \"\"\npath_prepend = [{}]\n", list.join(", ")),
    );
    std::fs::write(&p, text).unwrap();
    f.spec = load_spec(&p).unwrap();
    f
}

#[test]
fn path_prepend_puts_dirs_first_on_the_agents_path() {
    let _g = serial();
    let (a, b) = (TempDir::new().unwrap(), TempDir::new().unwrap());
    for arm in [Arm::Baseline, Arm::Cold] {
        let f = with_path_prepend(fixture("fix"), &[a.path(), b.path()]);
        let id = format!("pp-{}", arm.as_str());
        let o = run_session(&f.spec, arm, 1, &id, &f.cfg).unwrap();
        assert!(o.valid, "{o:?}");
        let path = std::fs::read_to_string(format!("{}.path", f.marker.display())).unwrap();
        let want = format!("{}:{}:", a.path().display(), b.path().display());
        assert!(path.starts_with(&want), "{arm:?}: {path}");
        let inherited = std::env::var("PATH").unwrap();
        assert!(path.trim_end().ends_with(&inherited), "{arm:?}: {path}");
    }
}

#[test]
fn without_path_prepend_the_agents_path_is_inherited_unchanged() {
    let _g = serial();
    let f = fixture("fix");
    run_session(&f.spec, Arm::Baseline, 1, "pp-none", &f.cfg).unwrap();
    let path = std::fs::read_to_string(format!("{}.path", f.marker.display())).unwrap();
    assert_eq!(path.trim_end(), std::env::var("PATH").unwrap());
}
