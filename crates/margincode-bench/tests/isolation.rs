//! The agent-isolation gate: `run --agent-user` starts the agent CLI as a user that can read only
//! its own working copy. No model: a fake `claude` tries, as the second session of each arm,
//! every file-system route an earlier paid cell leaked through, and writes what it could read
//! into its own working copy.
//!
//! The isolated half needs root and the user `nobody`. Without them it does not run, and says so
//! on stderr with a line the test harness does not capture.

use std::io::Write as _;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};

use crate::common::*;
use crate::runner::{FAKE_AV, script};
use margincode_bench::isolate::{AgentUser, harness_uid};
use margincode_bench::orchestrate::{RunArgs, RunReport, run};
use margincode_bench::report::{Meta, Status};
use margincode_bench::runner::{Arm, RunConfig, Vendor};
use margincode_bench::spec::{TaskSpec, load_spec};
use tempfile::TempDir;

/// A fake `claude` that behaves like an agent looking for the answer: it runs the hooks from
/// `--settings`, fixes `calc.py`, then tries every path in `$FAKE_PROBES` (`kind|label|path`)
/// and writes `label|READ` or `label|denied|<the error>` to `probe-report.txt` in its working
/// copy. What the hooks printed and how they exited go to `$CLAUDE_CONFIG_DIR`, which is the
/// session's own in every mode.
const FAKE_AGENT: &str = r#"#!/bin/sh
settings=""
while [ $# -gt 0 ]; do
  case "$1" in --settings) settings="$2"; shift;; esac
  shift
done
out="$CLAUDE_CONFIG_DIR"
id -u > "$out/fake-uid" || exit 6
hook() {
  python3 -c "import json,sys; h=json.load(open(sys.argv[1])).get('hooks',{}).get(sys.argv[2]); print(h[0]['hooks'][0]['command'] if h else '')" "$settings" "$1"
}
cmd=$(hook SessionStart) || exit 6
if [ -n "$cmd" ]; then
  echo "{\"source\":\"startup\",\"cwd\":\"$PWD\"}" | sh -c "$cmd" > "$out/hook-start.out" 2> "$out/hook-start.err"
  echo $? > "$out/hook-start.rc"
fi
cmd=$(hook PreToolUse)
if [ -n "$cmd" ]; then
  echo "{\"tool_name\":\"Edit\",\"tool_input\":{\"file_path\":\"$PWD/calc.py\"}}" | sh -c "$cmd" > "$out/hook-pre.out" 2> "$out/hook-pre.err"
  echo $? > "$out/hook-pre.rc"
fi
printf 'def add(a, b):\n    return a + b\n' > calc.py || exit 7
if [ -n "$FAKE_LEAVE_BEHIND" ]; then
  # what an agent could leave for verify and for the next session: a link where a hidden test
  # will be written, and a process that outlives it
  ln -s "$out/stolen" test_local.py
  # and a hard link where the other hidden test will be written, its other name kept in a
  # directory of the agent's own: writing through it would hand the test to that name
  echo "the agent's" > "$out/kept" || exit 8
  rm -f test_calc.py
  ln "$out/kept" test_calc.py || exit 8
  stat -c %h test_calc.py > "$out/kept-links"
  (setsid sleep 300 > /dev/null 2>&1 &)
fi
cmd=$(hook PostToolUse)
if [ -n "$cmd" ]; then
  echo "{\"tool_input\":{\"file_path\":\"$PWD/calc.py\"}}" | sh -c "$cmd" > "$out/hook-edit.out" 2> "$out/hook-edit.err"
  echo $? > "$out/hook-edit.rc"
fi
: > probe-report.txt || exit 7
while IFS='|' read -r kind label path; do
  case "$kind" in
    read) if err=$(head -c 1 -- "$path" 2>&1 >/dev/null); then r=READ; else r="denied|$err"; fi;;
    list) if err=$(ls -- "$path" 2>&1 >/dev/null); then r=READ; else r="denied|$err"; fi;;
    *) r="denied|bad probe kind $kind";;
  esac
  printf '%s|%s\n' "$label" "$r" >> probe-report.txt
done < "$FAKE_PROBES"
echo "{\"type\":\"system\",\"subtype\":\"init\",\"cwd\":\"$PWD\"}"
echo "{\"type\":\"assistant\",\"message\":{\"id\":\"m1\",\"content\":[{\"type\":\"tool_use\",\"id\":\"t1\",\"name\":\"Edit\",\"input\":{\"file_path\":\"$PWD/calc.py\",\"old_string\":\"a - b\",\"new_string\":\"a + b\"}}],\"usage\":{\"input_tokens\":10,\"cache_creation_input_tokens\":100,\"cache_read_input_tokens\":50,\"output_tokens\":20}}}"
echo "{\"type\":\"result\",\"subtype\":\"success\",\"is_error\":false,\"num_turns\":1,\"total_cost_usd\":0.25,\"usage\":{\"input_tokens\":10,\"cache_creation_input_tokens\":100,\"cache_read_input_tokens\":50,\"output_tokens\":20}}"
"#;

const RELAY_NOTES: &str = r#"
[[note]]
pair = "calc-add"
kind = "attempt"
path = "calc.py"
symbol = "add"
summary = "Swapping the operands of the subtraction does not fix add(): b - a is still a subtraction."
body = "Tried it first; the sign flips but the sum is still wrong."
"#;

const TASK: &str = "calc-add";
const RUN: &str = "gate";
const ARMS: [Arm; 3] = [Arm::Cold, Arm::Notes, Arm::Av];

/// Straight to stderr: the test harness captures `eprintln!` of a passing test, and whether the
/// gate ran must be seen in a plain `cargo test`.
fn say(line: &str) {
    let _ = writeln!(std::io::stderr(), "{line}");
}

fn chmod(p: &Path, mode: u32) {
    std::fs::set_permissions(p, std::fs::Permissions::from_mode(mode)).unwrap();
}

/// `nobody`, when this process is root and can start a process as that user; else why not.
fn agent_user() -> Result<AgentUser, String> {
    let me = harness_uid()?;
    if me != 0 {
        return Err(format!("not root (uid {me})"));
    }
    let user = AgentUser::resolve("nobody")?;
    let switched = std::process::Command::new("true")
        .uid(user.uid)
        .gid(user.gid)
        .status();
    match switched {
        Ok(s) if s.success() => Ok(user),
        other => Err(format!("cannot start a process as nobody: {other:?}")),
    }
}

struct Probe {
    kind: &'static str,
    label: String,
    path: PathBuf,
}

struct Gate {
    _dirs: Vec<TempDir>,
    /// Holds the bench tree (`tasks/`) and the source repo: root's alone unless a test opens it.
    closed: PathBuf,
    /// Holds the tools and the work root.
    open: PathBuf,
    daemon: TestDaemon,
    specs: Vec<TaskSpec>,
    cfg: RunConfig,
    out: PathBuf,
    probes: Vec<Probe>,
    probes_file: PathBuf,
}

impl Gate {
    fn session_dir(&self, arm: Arm, rep: u32) -> PathBuf {
        self.cfg
            .work_root
            .join("runs")
            .join(RUN)
            .join(format!("{TASK}-{}-r{rep}", arm.as_str()))
    }

    fn av_repo_dir(&self, rep: u32) -> PathBuf {
        self.daemon.ws_root.join(format!("b{RUN}-{TASK}-av-{rep}"))
    }

    fn working_copy(&self, arm: Arm, rep: u32) -> PathBuf {
        if arm.uses_av() {
            self.av_repo_dir(rep).join(TASK)
        } else {
            self.session_dir(arm, rep).join("ws")
        }
    }
}

/// The bench tree and the source repo in a directory only root can enter (as an operator must
/// keep them), the work root and the tools where the agent user can reach them, a real daemon,
/// and the list of what the second sessions will try to read: every route the first sessions
/// and the bench tree offer, by the names the harness gives them.
fn gate(agent: Option<AgentUser>, av_bin: Option<&Path>) -> Gate {
    let closed = TempDir::new().unwrap();
    chmod(closed.path(), 0o700);
    let open = TempDir::new().unwrap();
    chmod(open.path(), 0o711);

    let repo = closed.path().join("repo");
    std::fs::create_dir(&repo).unwrap();
    let (base, _) = fixture_repo(&repo);
    // This fixture's hidden tests are its own, byte for byte: the preflight looks under the temp
    // dir for copies of them, and other tests keep working copies of the shared fixture there.
    let nonce = uuid::Uuid::new_v4();
    std::fs::write(
        repo.join("test_calc.py"),
        format!("from calc import add\nassert add(2, 3) == 5\nprint('ok')  # {nonce}\n"),
    )
    .unwrap();
    git(
        &repo,
        &["commit", "-q", "-am", "the hidden test of this fixture"],
    );
    let fix = git(&repo, &["rev-parse", "HEAD"]);
    let tasks = closed.path().join("tasks");
    std::fs::create_dir_all(tasks.join("files")).unwrap();
    std::fs::create_dir_all(tasks.join("reference")).unwrap();
    std::fs::write(
        tasks.join("files/test_local.py"),
        format!("from calc import add\nassert add(-1, 1) == 0\nprint('local ok')  # {nonce}\n"),
    )
    .unwrap();
    std::fs::write(
        tasks.join("reference/edits.py"),
        "# the reference fix: return a + b\n",
    )
    .unwrap();
    std::fs::write(tasks.join("notes.toml"), RELAY_NOTES).unwrap();
    let spec_path = tasks.join("calc-task.toml");
    let toml = spec_toml(&repo, &base, &fix)
        .replace(
            "verify_cmd = \"python3 test_calc.py\"",
            "verify_cmd = \"python3 test_calc.py && python3 test_local.py\"",
        )
        .replace(
            "setup_cmd = \"\"\n",
            "setup_cmd = \"\"\nrelay_pool = \"notes.toml\"\nrelay_pair = \"calc-add\"\n",
        )
        + "\n[[local_tests]]\npath = \"test_local.py\"\nsource = \"files/test_local.py\"\n";
    std::fs::write(&spec_path, toml).unwrap();
    let spec = load_spec(&spec_path).unwrap();
    assert_eq!(spec.local_tests.len(), 1);
    assert_eq!(spec.relay_notes.len(), 1);

    let tools = open.path().join("tools");
    std::fs::create_dir(&tools).unwrap();
    chmod(&tools, 0o755);
    let claude = script(&tools, "claude", FAKE_AGENT);
    let av = match av_bin {
        Some(real) => {
            let to = tools.join("av");
            std::fs::copy(real, &to).unwrap();
            chmod(&to, 0o755);
            to
        }
        None => script(&tools, "av", FAKE_AV),
    };

    let daemon = start_daemon(None);
    // the daemon's own directory: an operator opens it for traversal, never for listing
    chmod(&daemon.ws_root, 0o711);

    let work = open.path().join("work");
    let cfg = RunConfig {
        work_root: work.clone(),
        claude_bin: claude.to_string_lossy().into(),
        grok_bin: "grok".into(),
        vendor: Vendor::Claude,
        av_bin: av.to_string_lossy().into(),
        av_url: Some(daemon.url.clone()),
        av_token: None,
        import_dir: work.join("src"),
        import_mount: work.join("src").to_string_lossy().into(),
        allowed_tools: vec!["Read".into(), "Edit".into()],
        permission_mode: "acceptEdits".into(),
        session_cap_usd: 1.0,
        claude_home: None,
        hook_budget: 800,
        allow_no_semantic: true,
        index_wait_secs: 60,
        av_mcp: false,
        hook_code: None,
        hook_args: vec![],
        jit_hooks: false,
        prices: None,
        // the worst case: every first session's copy is still on disk for the second
        keep_work: true,
        agent_user: agent,
        av_ws_root: Some(daemon.ws_root.clone()),
    };

    let mut g = Gate {
        closed: closed.path().to_path_buf(),
        open: open.path().to_path_buf(),
        _dirs: vec![closed, open],
        daemon,
        specs: vec![spec],
        cfg,
        out: work.join("results").join(RUN),
        probes: vec![],
        probes_file: tools.join("probes"),
    };
    let mut probes = vec![];
    let mut add = |kind: &'static str, label: String, path: PathBuf| {
        probes.push(Probe { kind, label, path });
    };
    add(
        "read",
        "bench tree: hidden test".into(),
        tasks.join("files/test_local.py"),
    );
    add(
        "read",
        "bench tree: reference fix".into(),
        tasks.join("reference/edits.py"),
    );
    add("read", "bench tree: task file".into(), spec_path.clone());
    add(
        "read",
        "bench tree: relay pool".into(),
        tasks.join("notes.toml"),
    );
    add("list", "bench tree: listing".into(), tasks.clone());
    add(
        "read",
        "source repo: hidden test at the fix".into(),
        repo.join("test_calc.py"),
    );
    add(
        "list",
        "import dir: listing".into(),
        g.cfg.import_dir.clone(),
    );
    add(
        "read",
        "import dir: av r1 source".into(),
        g.cfg
            .import_dir
            .join(format!("b{RUN}-{TASK}-av-1"))
            .join("calc.py"),
    );
    add("list", "runs root: listing".into(), work.join("runs"));
    add(
        "list",
        "run dir: listing".into(),
        work.join("runs").join(RUN),
    );
    add(
        "list",
        "daemon workspace root: listing".into(),
        g.daemon.ws_root.clone(),
    );
    // the results directory: by the second sessions it holds the first sessions' rows
    add("list", "results: listing".into(), g.out.clone());
    add(
        "read",
        "results: rows.jsonl".into(),
        g.out.join("rows.jsonl"),
    );
    add("read", "results: meta.json".into(), g.out.join("meta.json"));
    for arm in ARMS {
        let a = arm.as_str();
        let s = g.session_dir(arm, 1);
        add("list", format!("{a} r1: session dir listing"), s.clone());
        for f in [
            "events.jsonl",
            "claude.stderr",
            "verify.log",
            "outcome.json",
            "settings.json",
            "mcp.json",
        ] {
            add("read", format!("{a} r1: {f}"), s.join(f));
        }
        let wc = g.working_copy(arm, 1);
        add("list", format!("{a} r1: working copy listing"), wc.clone());
        add(
            "read",
            format!("{a} r1: working copy file"),
            wc.join("calc.py"),
        );
        add(
            "read",
            format!("{a} r1: hidden test verify copied (fix_rev)"),
            wc.join("test_calc.py"),
        );
        add(
            "read",
            format!("{a} r1: hidden test verify copied (local)"),
            wc.join("test_local.py"),
        );
        match arm {
            Arm::Notes => add("read", format!("{a} r1: NOTES.md"), wc.join("NOTES.md")),
            Arm::Av => {
                add("read", format!("{a} r1: pack.md"), s.join("pack.md"));
                add("read", format!("{a} r1: hook-count"), s.join("hook-count"));
                add(
                    "list",
                    format!("{a} r1: daemon repo dir listing"),
                    g.av_repo_dir(1),
                );
            }
            _ => {}
        }
    }
    let list: String = probes
        .iter()
        .map(|p| format!("{}|{}|{}\n", p.kind, p.label, p.path.display()))
        .collect();
    std::fs::write(&g.probes_file, list).unwrap();
    chmod(&g.probes_file, 0o644);
    g.probes = probes;
    g
}

/// Two sessions in a row per arm, through `orchestrate::run` as `margincode-bench run` does it.
/// The caller holds `SERIAL`.
fn two_reps(g: &Gate) -> margincode_bench::Result<RunReport> {
    sessions(g, &ARMS, 2)
}

fn sessions(g: &Gate, arms: &[Arm], reps: u32) -> margincode_bench::Result<RunReport> {
    sessions_to(g, &g.out, arms, reps)
}

/// As [`sessions`], into the results directory `out` (whose name is the run id).
fn sessions_to(
    g: &Gate,
    out: &Path,
    arms: &[Arm],
    reps: u32,
) -> margincode_bench::Result<RunReport> {
    let args = RunArgs {
        out: out.to_path_buf(),
        arms: arms.to_vec(),
        reps,
        budget_usd: 100.0,
        seed: 1,
        confirm: true,
        dry_run: false,
        est_session_usd: None,
        pilot: None,
        meta: Meta::default(),
    };
    // SAFETY: SERIAL is held.
    unsafe { std::env::set_var("FAKE_PROBES", &g.probes_file) };
    let r = run(&g.specs, &args, &g.cfg);
    unsafe { std::env::remove_var("FAKE_PROBES") };
    r
}

/// What a session's fake agent wrote into its working copy: (label, read, the error).
fn probe_report(g: &Gate, arm: Arm, rep: u32) -> Vec<(String, bool, String)> {
    let p = g.working_copy(arm, rep).join("probe-report.txt");
    let text = std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("{}: {e}", p.display()));
    text.lines()
        .map(|l| {
            let mut f = l.splitn(3, '|');
            let label = f.next().unwrap().to_owned();
            let read = f.next() == Some("READ");
            (label, read, f.next().unwrap_or_default().to_owned())
        })
        .collect()
}

fn read(p: &Path) -> String {
    std::fs::read_to_string(p).unwrap_or_else(|e| panic!("{}: {e}", p.display()))
}

/// Every session ran as `uid`, edited its own working copy, and passed verify; the av sessions'
/// hooks fired and the daemon saw the edit.
fn assert_sessions_worked(g: &Gate, r: &RunReport, uid: u32) {
    assert_sessions_of(g, r, uid, &ARMS, 2);
}

fn assert_sessions_of(g: &Gate, r: &RunReport, uid: u32, arms: &[Arm], reps: u32) {
    assert_eq!(r.exit_code, 0);
    assert_eq!(r.rows.len(), reps as usize * arms.len(), "{:?}", r.rows);
    for row in &r.rows {
        assert_eq!(row.status, Status::Ok, "{row:?}");
        assert!(row.success, "verify passed: {row:?}");
    }
    for &arm in arms {
        for rep in 1..=reps {
            let s = g.session_dir(arm, rep);
            let who = format!("{} r{rep}", arm.as_str());
            assert_eq!(
                read(&s.join("claude-config/fake-uid")).trim(),
                uid.to_string(),
                "{who}: the agent's uid"
            );
            let wc = g.working_copy(arm, rep);
            assert_eq!(
                read(&wc.join("calc.py")),
                "def add(a, b):\n    return a + b\n",
                "{who}: the agent edited its own working copy"
            );
            let outcome: serde_json::Value =
                serde_json::from_str(&read(&s.join("outcome.json"))).unwrap();
            assert_eq!(outcome["verify_exit"], 0, "{who}: {outcome}");
            assert!(
                read(&s.join("verify.log")).contains("local ok"),
                "{who}: both hidden tests ran"
            );
            if !arm.uses_av() {
                continue;
            }
            assert_eq!(
                read(&s.join("hook-count")).lines().collect::<Vec<_>>(),
                ["session-start", "post-edit"],
                "{who}: both hooks fired"
            );
            for hook in ["hook-start", "hook-edit"] {
                let err = read(&s.join(format!("claude-config/{hook}.err")));
                assert_eq!(
                    read(&s.join(format!("claude-config/{hook}.rc"))).trim(),
                    "0",
                    "{who}: {hook}: {err}"
                );
                assert_eq!(err.trim(), "", "{who}: {hook} complained");
            }
            assert!(
                read(&s.join("claude-config/hook-start.out")).contains("## Task"),
                "{who}: the session-start hook printed a pack"
            );
            let tf: serde_json::Value = serde_json::from_str(&read(&wc.join(".av-task"))).unwrap();
            let t: serde_json::Value = ureq::get(&format!(
                "{}/v1/tasks/{}",
                g.daemon.url,
                tf["task"].as_str().unwrap()
            ))
            .call()
            .unwrap()
            .into_json()
            .unwrap();
            assert!(
                t["changed"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|p| p == "calc.py"),
                "{who}: the daemon snapshotted the agent's edit: {t}"
            );
        }
    }
}

/// The gate. Control first: without `--agent-user` the same fake agent reads every path, so the
/// probes are real paths and a leak is detected. Then with `--agent-user nobody`: every attempt
/// fails with a permission error, and every session still works.
#[test]
fn a_session_cannot_read_the_grading_files_or_an_earlier_session() {
    let _g = serial();
    let me = harness_uid().unwrap();

    let g = gate(None, None);
    let r = two_reps(&g).unwrap();
    assert_sessions_worked(&g, &r, me);
    let mut control_read = 0;
    for arm in ARMS {
        let report = probe_report(&g, arm, 2);
        assert_eq!(report.len(), g.probes.len());
        for (label, read, err) in &report {
            assert!(
                read,
                "control, {} r2: `{label}` must be readable without --agent-user, or the gate \
                 proves nothing about it: {err}",
                arm.as_str()
            );
        }
        control_read += report.len();
    }
    let meta = read(&g.out.join("meta.json"));
    assert!(meta.contains(r#""agent_isolation": "none""#), "{meta}");
    assert!(read(&g.out.join("report.md")).contains(" · agent isolation: none · "));
    say(&format!(
        "agent-isolation gate: control (no --agent-user, agent uid {me}): the second sessions \
         read {control_read} of {control_read} probed paths ({} paths x {} arms)",
        g.probes.len(),
        ARMS.len()
    ));
    drop(g);

    let user = match agent_user() {
        Ok(u) => u,
        Err(why) => {
            say(&format!(
                "agent-isolation gate: SKIPPED, {why}. The isolated half did NOT run: nothing \
                 here shows that --agent-user closes a route. Run the tests as root on a host \
                 with the user `nobody`."
            ));
            return;
        }
    };
    let g = gate(Some(user.clone()), None);
    // SAFETY: SERIAL is held.
    unsafe { std::env::set_var("FAKE_LEAVE_BEHIND", "1") };
    let r = two_reps(&g);
    unsafe { std::env::remove_var("FAKE_LEAVE_BEHIND") };
    let r = r.unwrap();
    let mut leaked = vec![];
    let mut tried = 0;
    for arm in ARMS {
        let report = probe_report(&g, arm, 2);
        assert_eq!(report.len(), g.probes.len(), "every route was tried");
        for (label, read, err) in report {
            tried += 1;
            if read {
                leaked.push(format!("{} r2 read `{label}`", arm.as_str()));
            } else {
                assert!(
                    err.contains("Permission denied"),
                    "{} r2: `{label}` failed for another reason than permissions: {err}",
                    arm.as_str()
                );
            }
        }
    }
    assert!(
        leaked.is_empty(),
        "{} of {tried} attempts succeeded as user {}:\n{}",
        leaked.len(),
        user.name,
        leaked.join("\n")
    );
    assert_sessions_worked(&g, &r, user.uid);
    // the failures were refusals, not missing files: root still finds every path
    for p in &g.probes {
        assert!(p.path.exists(), "`{}`: {}", p.label, p.path.display());
    }
    // what each fake agent left behind came to nothing: no process of the user is alive, and
    // the hidden test did not go through its link into a directory of its own
    assert_eq!(user.processes(), Vec::<u32>::new(), "processes left");
    for arm in ARMS {
        for rep in [1, 2] {
            let stolen = g.session_dir(arm, rep).join("claude-config/stolen");
            assert!(!stolen.exists(), "{}", stolen.display());
            let test = g.working_copy(arm, rep).join("test_local.py");
            assert!(
                std::fs::symlink_metadata(&test).unwrap().is_file(),
                "{} is a file of the harness's, not the agent's link",
                test.display()
            );
            // nor through its hard link: the agent did link a file of its own there (two names),
            // and the test is a new file, with the agent's other name left holding its own bytes
            let who = format!("{} r{rep}", arm.as_str());
            let config = g.session_dir(arm, rep).join("claude-config");
            assert_eq!(
                read(&config.join("kept-links")).trim(),
                "2",
                "{who}: the agent hard-linked its file where the hidden test goes"
            );
            let kept = config.join("kept");
            let test = g.working_copy(arm, rep).join("test_calc.py");
            let (k, t) = (
                std::fs::symlink_metadata(&kept).unwrap(),
                std::fs::symlink_metadata(&test).unwrap(),
            );
            assert!(t.is_file(), "{who}: {}", test.display());
            assert_ne!(t.ino(), k.ino(), "{who}: the hidden test is a new file");
            assert_eq!(
                (t.nlink(), k.nlink()),
                (1, 1),
                "{who}: neither file has another name"
            );
            assert_eq!(
                read(&kept),
                "the agent's\n",
                "{who}: the agent's other name"
            );
            assert!(
                read(&test).contains("assert add(2, 3) == 5"),
                "{who}: the hidden test is at its path"
            );
        }
    }
    // the results are root's alone: the directory the preflight made, and each file in it
    for path in [
        g.out.clone(),
        g.out.join("rows.jsonl"),
        g.out.join("meta.json"),
        g.out.join("report.md"),
    ] {
        let m = std::fs::metadata(&path).unwrap();
        assert_eq!(
            (m.uid(), m.mode() & 0o077),
            (0, 0),
            "{} is root's and closed to others",
            path.display()
        );
    }
    // what the sessions left is root's alone: the directories, and the transcript by itself
    for arm in ARMS {
        for rep in [1, 2] {
            let s = g.session_dir(arm, rep);
            let mut mine = vec![
                s.clone(),
                g.working_copy(arm, rep),
                s.join("events.jsonl"),
                s.join("claude.stderr"),
                s.join("home"),
                s.join("claude-config"),
            ];
            if arm.uses_av() {
                mine.extend([s.join("pack.md"), g.av_repo_dir(rep)]);
            }
            for path in mine {
                let m = std::fs::metadata(&path).unwrap();
                assert_eq!(
                    (m.uid(), m.mode() & 0o077),
                    (0, 0),
                    "{} is root's and closed to others",
                    path.display()
                );
            }
        }
    }
    let meta = read(&g.out.join("meta.json"));
    assert!(meta.contains(r#""agent_isolation": "nobody""#), "{meta}");
    assert!(read(&g.out.join("report.md")).contains(" · agent isolation: user nobody · "));
    say(&format!(
        "agent-isolation gate: RAN as root with --agent-user {} (uid {}): the second sessions \
         read 0 of {tried} probed paths ({} paths x {} arms: cold, notes, av), each refused \
         with a permission error; {} of {} sessions passed verify; the av hooks fired in both \
         av sessions; in each of the 6 sessions the agent hard-linked a file of its own where a \
         hidden test goes, and the test was written as a new file (another inode, one name)",
        user.name,
        user.uid,
        g.probes.len(),
        ARMS.len(),
        r.rows.iter().filter(|row| row.success).count(),
        r.rows.len()
    ));
}

/// The `av` binary cargo built beside this test binary (`<target>/<profile>/av`), if it did.
fn real_av() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let av = exe.parent()?.parent()?.join("av");
    av.is_file().then_some(av)
}

/// The gate's av sessions use the fake `av` every other bench test uses, so they hold wherever
/// the tests run. This one runs the real hooks as the agent user: `av hook session-start` reads
/// and rewrites `.av-task`, asks the daemon for the pack and makes the scratch directory;
/// `av hook post-edit` checkpoints. Without `--keep-work`, so the task is abandoned afterwards.
#[test]
fn the_real_av_hooks_work_as_the_agent_user() {
    let _g = serial();
    let user = match agent_user() {
        Ok(u) => u,
        Err(why) => {
            say(&format!(
                "agent-isolation, real av hooks: SKIPPED, {why}. The real hooks were NOT run as \
                 another user."
            ));
            return;
        }
    };
    let Some(av) = real_av() else {
        say(
            "agent-isolation, real av hooks: SKIPPED, no `av` binary beside the test binary \
             (`cargo test --workspace` builds it; so does `cargo build -p margincode-cli`). The \
             real hooks were NOT run as the agent user; only the fake `av` was.",
        );
        return;
    };
    let mut g = gate(Some(user.clone()), Some(&av));
    g.cfg.keep_work = false;
    let r = sessions(&g, &[Arm::Av], 1).unwrap();
    assert_sessions_of(&g, &r, user.uid, &[Arm::Av], 1);
    let s = g.session_dir(Arm::Av, 1);
    let start = read(&s.join("claude-config/hook-start.out"));
    assert!(
        start.contains("Temporary files:"),
        "the hook got its scratch directory: {start}"
    );
    let wc = g.working_copy(Arm::Av, 1);
    let tf: serde_json::Value = serde_json::from_str(&read(&wc.join(".av-task"))).unwrap();
    let t: serde_json::Value = ureq::get(&format!(
        "{}/v1/tasks/{}",
        g.daemon.url,
        tf["task"].as_str().unwrap()
    ))
    .call()
    .unwrap()
    .into_json()
    .unwrap();
    assert_eq!(t["task"]["status"], "abandoned", "{t}");
    for dir in [&s, &wc, &g.av_repo_dir(1)] {
        let m = std::fs::metadata(dir).unwrap();
        assert_eq!((m.uid(), m.mode() & 0o077), (0, 0), "{}", dir.display());
    }
    say(&format!(
        "agent-isolation, real av hooks: RAN as root with --agent-user {} (uid {}), {}: \
         session-start printed the pack and the scratch line, post-edit checkpointed, both \
         exited 0 with nothing on stderr; verify passed",
        user.name,
        user.uid,
        av.display()
    ));
}

/// A fake `av` whose pre-read hook has something to say, as the real one has about a file with
/// records.
const SAYING_AV: &str = r#"#!/bin/sh
cat > /dev/null
case "$2" in
  session-start) echo '## Task (fake pack)';;
  pre-read) echo 'a record about calc.py';;
esac
exit 0
"#;

/// `--jit-hooks` with an agent user: the pre-read hook keeps what it told the agent in
/// `pre-read.out`, and the agent user cannot write in the session directory. A hook that fails
/// there would lose the agent the records about the file it is about to read, so the hook must
/// exit 0 as that user, and the harness must still end up with the file, its own alone.
#[test]
fn the_pre_read_hook_works_as_the_agent_user_and_what_it_said_is_kept() {
    let _g = serial();
    let Some(mut g) = isolated_gate("pre-read hook (--jit-hooks)") else {
        return;
    };
    let user = g.cfg.agent_user.clone().unwrap();
    g.cfg.jit_hooks = true;
    script(&g.open.join("tools"), "av", SAYING_AV);
    let r = sessions(&g, &[Arm::Av], 1).unwrap();
    assert_eq!(r.rows.len(), 1, "{:?}", r.rows);
    assert_eq!(r.rows[0].status, Status::Ok, "{:?}", r.rows);
    assert!(r.rows[0].success, "{:?}", r.rows);
    let s = g.session_dir(Arm::Av, 1);
    assert_eq!(
        read(&s.join("claude-config/fake-uid")).trim(),
        user.uid.to_string()
    );
    let err = read(&s.join("claude-config/hook-pre.err"));
    assert_eq!(
        read(&s.join("claude-config/hook-pre.rc")).trim(),
        "0",
        "the pre-read hook failed as the agent user: {err}"
    );
    assert_eq!(err.trim(), "", "the pre-read hook complained");
    let told = read(&s.join("claude-config/hook-pre.out"));
    assert_eq!(told, "a record about calc.py\n", "what the agent was told");
    assert_eq!(
        read(&s.join("pre-read.out")),
        told,
        "the harness keeps what the hook said"
    );
    assert_eq!(
        read(&s.join("hook-count")).lines().collect::<Vec<_>>(),
        ["session-start", "pre-read", "post-edit"]
    );
    for f in ["pre-read.out", "hook-count", "home/pre-read.out", "home"] {
        let m = std::fs::metadata(s.join(f)).unwrap();
        assert_eq!(
            (m.uid(), m.mode() & 0o077),
            (0, 0),
            "{f} is root's and closed to others"
        );
    }
    say(&format!(
        "agent-isolation, pre-read hook: RAN as root with --agent-user {} (uid {}) and \
         --jit-hooks: the PreToolUse hook exited 0 with nothing on stderr, the agent got its \
         output, and <session>/pre-read.out holds it, root's and mode 600",
        user.name, user.uid
    ));
}

/// The gate's fixture for the preflight tests, or `None` (said on stderr) without root.
fn isolated_gate(test: &str) -> Option<Gate> {
    match agent_user() {
        Ok(user) => Some(gate(Some(user), None)),
        Err(why) => {
            say(&format!(
                "agent-isolation, {test}: SKIPPED, {why}. This check did NOT run."
            ));
            None
        }
    }
}

/// No session directory was made: the refusal came before the first session.
fn assert_nothing_launched(g: &Gate) {
    let run_dir = g.cfg.work_root.join("runs").join(RUN);
    let made: Vec<_> = std::fs::read_dir(&run_dir)
        .map(|d| d.flatten().map(|e| e.file_name()).collect())
        .unwrap_or_default();
    assert!(made.is_empty(), "{made:?}");
    assert!(!g.out.join("rows.jsonl").exists());
}

#[test]
fn the_preflight_refuses_a_bench_tree_the_agent_user_can_read_and_names_the_paths() {
    let _g = serial();
    let Some(g) = isolated_gate("preflight, readable bench tree") else {
        return;
    };
    // a bench tree mounted for everyone, as in the container that leaked
    chmod(&g.closed, 0o755);
    let err = two_reps(&g).err().expect("the run must be refused");
    assert!(err.contains("refusing to run"), "{err}");
    let tasks = g.closed.join("tasks");
    for line in [
        format!("enter {}", tasks.display()),
        format!("list {}", tasks.display()),
        format!("read {}", tasks.join("files/test_local.py").display()),
        format!("read {}", tasks.join("notes.toml").display()),
        format!("enter {}", g.closed.join("repo").display()),
    ] {
        assert!(err.contains(&line), "`{line}` is named in: {err}");
    }
    assert_nothing_launched(&g);

    // closed again, the same run starts
    chmod(&g.closed, 0o700);
    let r = sessions(&g, &[Arm::Cold], 1).unwrap();
    assert!(r.rows[0].success, "{:?}", r.rows);
}

#[test]
fn the_preflight_says_when_the_agent_user_cannot_reach_the_work_root() {
    let _g = serial();
    let Some(g) = isolated_gate("preflight, unreachable work root") else {
        return;
    };
    // as a work root under /root: a directory on the way that only root may enter
    chmod(&g.open, 0o700);
    let err = two_reps(&g).err().expect("the run must be refused");
    assert!(
        err.contains("cannot reach the work root")
            && err.contains(&format!("may not enter {} (mode 700)", g.open.display()))
            && err.contains("/root"),
        "{err}"
    );
    assert!(
        !g.cfg.work_root.exists(),
        "nothing is made under a work root that is then refused"
    );
}

#[test]
fn the_preflight_refuses_workspaces_and_sessions_left_open_to_the_agent_user() {
    let _g = serial();
    let Some(g) = isolated_gate("preflight, daemon workspaces and earlier sessions") else {
        return;
    };
    // the daemon's workspace root as a daemon makes it, and a workspace it kept
    chmod(&g.daemon.ws_root, 0o755);
    let kept = g.daemon.ws_root.join("bold-calc-add-av-1");
    std::fs::create_dir(&kept).unwrap();
    chmod(&kept, 0o755);
    // an earlier run made without the flag, and a session of this run's name left open
    let runs = g.cfg.work_root.join("runs");
    let earlier = runs.join("pilot");
    let session = runs.join(RUN).join("calc-add-av-r1");
    for d in [&earlier, &session] {
        std::fs::create_dir_all(d).unwrap();
        chmod(d, 0o755);
    }
    let err = two_reps(&g).err().expect("the run must be refused");
    for line in [
        format!("list {}", g.daemon.ws_root.display()),
        format!("enter {}", kept.display()),
        format!("enter {}", earlier.display()),
        format!("enter {}", session.display()),
    ] {
        assert!(err.contains(&line), "`{line}` is named in: {err}");
    }
    assert!(!g.out.join("rows.jsonl").exists());

    // without the daemon's workspace root there is nothing to check the av arms against
    let mut g = g;
    g.cfg.av_ws_root = None;
    let err = two_reps(&g).err().expect("the run must be refused");
    assert!(err.contains("--av-ws-root"), "{err}");
    // and the git arms do not need it
    for d in [&earlier, &session] {
        chmod(d, 0o700);
    }
    let r = sessions(&g, &[Arm::Cold], 1).unwrap();
    assert!(r.rows[0].success, "{:?}", r.rows);
}

/// A grading file copied to where every user reads (a temp dir) is found by its bytes, whatever
/// it is called there. An open run leaves such copies behind: the harness container that leaked
/// had the hidden test of one task in `/tmp`.
#[test]
fn the_preflight_refuses_a_copy_of_a_hidden_test_in_a_shared_directory() {
    let _g = serial();
    let Some(g) = isolated_gate("preflight, a hidden test copied to a temp dir") else {
        return;
    };
    // the tools directory is under the temp dir and open to the agent user
    let tools = g.open.join("tools");
    let from_fix = tools.join("test_calc.py");
    let renamed = tools.join("notes-to-self.txt");
    std::fs::copy(g.closed.join("repo/test_calc.py"), &from_fix).unwrap();
    std::fs::copy(g.closed.join("tasks/files/test_local.py"), &renamed).unwrap();
    for f in [&from_fix, &renamed] {
        chmod(f, 0o644);
    }
    let err = two_reps(&g).err().expect("the run must be refused");
    for f in [&from_fix, &renamed] {
        let line = format!("read {} (a copy of a hidden test of calc-add)", f.display());
        assert!(err.contains(&line), "`{line}` is named in: {err}");
    }
    assert_nothing_launched(&g);

    // closed to that user, they are no route
    for f in [&from_fix, &renamed] {
        chmod(f, 0o600);
    }
    let r = sessions(&g, &[Arm::Cold], 1).unwrap();
    assert!(r.rows[0].success, "{:?}", r.rows);
}

/// Results hold what a `cold` or `baseline` session must not see: `first-packs` writes packs
/// with the tasks' lessons, `run` the rows of earlier sessions. An `--out` that is there and
/// open is refused, and so is a results directory beside it; a missing `--out` is made root's
/// alone, with the files in it.
#[test]
fn the_preflight_refuses_results_the_agent_user_can_read() {
    let _g = serial();
    let Some(g) = isolated_gate("preflight, results directories") else {
        return;
    };
    let results = g.out.parent().unwrap().to_path_buf();
    // as `first-packs --out` and an earlier `run --out` leave them: default modes
    let packs = results.join("first-pack/n500");
    let earlier = results.join("pilot");
    for d in [&g.out, &packs.join("packs"), &earlier] {
        std::fs::create_dir_all(d).unwrap();
    }
    let lesson = packs.join("packs/calc-add-start.md");
    let report = g.out.join("report.md");
    for f in [
        &lesson,
        &packs.join("first-packs.jsonl"),
        &earlier.join("rows.jsonl"),
        &report,
    ] {
        std::fs::write(f, "").unwrap();
        chmod(f, 0o644);
    }
    for d in [
        &g.cfg.work_root,
        &results,
        &results.join("first-pack"),
        &packs,
        &packs.join("packs"),
        &earlier,
        &g.out,
    ] {
        chmod(d, 0o755);
    }
    let err = two_reps(&g).err().expect("the run must be refused");
    assert!(err.contains("refusing to run"), "{err}");
    for line in [
        format!("enter {}", g.out.display()),
        format!("list {}", g.out.display()),
        format!("read {}", report.display()),
        format!("enter {}", packs.display()),
        format!("enter {}", earlier.display()),
    ] {
        assert!(err.contains(&line), "`{line}` is named in: {err}");
    }
    assert!(err.contains("chmod 700"), "{err}");
    assert_nothing_launched(&g);

    // closed, each of them, the same run starts, and what it writes is root's alone
    for d in [&g.out, &packs, &earlier] {
        chmod(d, 0o700);
    }
    let r = sessions(&g, &[Arm::Cold], 1).unwrap();
    assert!(r.rows[0].success, "{:?}", r.rows);
    for f in ["rows.jsonl", "meta.json", "report.md"] {
        let m = std::fs::metadata(g.out.join(f)).unwrap();
        assert_eq!(
            (m.uid(), m.mode() & 0o077),
            (0, 0),
            "{f} is root's and closed to others"
        );
    }
    say(
        "agent-isolation, results directories: RAN as root with --agent-user nobody: an open \
         --out (enter, list, a file in it) and two open results directories beside it (one of \
         first-packs, one of run) were refused by name; closed, the run started and wrote \
         rows.jsonl, meta.json and report.md as root's, mode 600",
    );
}

/// The harness gives the agent a `HOME` of its own per session, but a program can ask the
/// password database for the user's home (`getpwuid`). If that directory is there and the user
/// can write to it, a session can leave its work in it for a later one.
#[test]
fn the_preflight_refuses_a_home_the_agent_user_can_write_to_or_list() {
    let _g = serial();
    let Some(mut g) = isolated_gate("preflight, the agent user's own home") else {
        return;
    };
    let user = g.cfg.agent_user.clone().unwrap();
    let resolved = user.home.clone();
    // the home `nobody` has here, when it is no directory: nothing to refuse, as in every other
    // test of this file. A directory stands in for a home that exists.
    let home = g.open.join("passwd-home");
    std::fs::create_dir(&home).unwrap();
    std::os::unix::fs::chown(&home, Some(user.uid), Some(user.gid)).unwrap();
    chmod(&home, 0o700);
    g.cfg.agent_user.as_mut().unwrap().home = Some(home.clone());
    let err = two_reps(&g).err().expect("the run must be refused");
    assert!(err.contains("refusing to run"), "{err}");
    let (write, list) = (
        format!("write to {}", home.display()),
        format!("list {}", home.display()),
    );
    for line in [&write, &list] {
        assert!(err.contains(line.as_str()), "`{line}` is named in: {err}");
    }
    assert!(err.contains("usermod -d"), "how to close it: {err}");
    assert_nothing_launched(&g);

    // root's but open to read: nothing can be left there, what is there can still be listed
    std::os::unix::fs::chown(&home, Some(0), Some(0)).unwrap();
    chmod(&home, 0o755);
    let err = two_reps(&g).err().expect("the run must be refused");
    assert!(err.contains(&list) && !err.contains(&write), "{err}");
    assert_nothing_launched(&g);

    // root's and closed, the run starts; so it does when the directory is not there at all
    chmod(&home, 0o700);
    let r = sessions(&g, &[Arm::Cold], 1).unwrap();
    assert!(r.rows[0].success, "{:?}", r.rows);
    std::fs::remove_dir(&home).unwrap();
    let r = sessions(&g, &[Arm::Notes], 1).unwrap();
    assert!(r.rows[0].success, "{:?}", r.rows);
    say(&format!(
        "agent-isolation, the agent user's home: RAN as root with --agent-user {} (uid {}), \
         whose home in the password database is {resolved:?}: a home that user owns was refused \
         (write, list), one of root's open to read was refused (list), one closed or missing \
         was accepted. The directory was set on the fixture's user, not in the password \
         database",
        user.name, user.uid
    ));
}

/// A session that stops before its agent ran (here: `setup_cmd` fails) leaves nothing open for
/// the next one.
#[test]
fn a_session_that_stops_early_is_closed_too() {
    let _g = serial();
    let Some(mut g) = isolated_gate("a session that stops early") else {
        return;
    };
    g.specs[0].setup_cmd = "echo set up > made-by-setup; exit 3".into();
    let r = sessions(&g, &[Arm::Notes], 1).unwrap();
    assert_eq!(r.rows[0].status, Status::Invalid, "{:?}", r.rows);
    let s = g.session_dir(Arm::Notes, 1);
    for dir in [s.clone(), s.join("ws")] {
        let m = std::fs::metadata(&dir).unwrap();
        assert_eq!((m.uid(), m.mode() & 0o077), (0, 0), "{}", dir.display());
    }
    assert!(s.join("ws/NOTES.md").exists(), "kept for debugging");
}

/// Not root: the flag is refused with the reason, before anything is launched. (As root this
/// path cannot be taken; the other tests take the root one.)
#[test]
fn the_flag_is_refused_when_the_harness_is_not_root() {
    let _g = serial();
    let me = harness_uid().unwrap();
    if me == 0 {
        return;
    }
    let other = AgentUser {
        name: "nobody".into(),
        uid: 65534,
        gid: 65534,
        home: None,
    };
    let g = gate(Some(other), None);
    let err = two_reps(&g).err().expect("the run must be refused");
    assert!(err.contains("must run as root"), "{err}");
    assert!(!g.out.join("rows.jsonl").exists());
}

fn mode_of(p: &Path) -> u32 {
    std::fs::metadata(p)
        .unwrap_or_else(|e| panic!("{}: {e}", p.display()))
        .mode()
        & 0o7777
}

/// The entries of a directory; none when it is not there.
fn entries_of(dir: &Path) -> Vec<PathBuf> {
    std::fs::read_dir(dir)
        .map(|d| d.flatten().map(|e| e.path()).collect())
        .unwrap_or_default()
}

/// A run the preflight refuses leaves nothing it made: no run directory under `runs/` (which the
/// agent user may enter, so the next preflight would refuse on it), no `--out`, no work root.
/// Directories that were there before are kept, with the modes they had.
#[test]
fn a_refused_preflight_leaves_nothing_the_next_one_refuses() {
    let _g = serial();
    let Some(g) = isolated_gate("a refused preflight leaves nothing behind") else {
        return;
    };
    let results = g.out.parent().unwrap().to_path_buf();
    let runs = g.cfg.work_root.join("runs");
    // refused for a reason of its own (the bench tree open to everyone), three times in a row,
    // each with a new --out, as an operator retries
    chmod(&g.closed, 0o755);
    let tasks = format!("enter {}", g.closed.join("tasks").display());
    for name in ["try1", "try2", "try3"] {
        let out = results.join(name);
        let err = sessions_to(&g, &out, &[Arm::Cold], 1)
            .err()
            .expect("the run must be refused");
        assert!(
            err.contains(&tasks),
            "{name}: refused for the bench tree: {err}"
        );
        assert!(
            !err.contains(&runs.display().to_string()),
            "{name}: refused on what an earlier try left under {}: {err}",
            runs.display()
        );
        let left = entries_of(&runs);
        assert!(
            left.is_empty(),
            "{name}: the refused run left {left:?} under {}",
            runs.display()
        );
        assert!(
            !out.exists(),
            "{name}: the refused run left its --out {}",
            out.display()
        );
        assert!(
            !g.cfg.work_root.exists(),
            "{name}: the refused run left the work root it made, {}",
            g.cfg.work_root.display()
        );
    }
    // a --out and a run directory that were there before stay, and so do their modes
    let kept_out = results.join("kept");
    let kept_run = runs.join("kept");
    for d in [&kept_out, &kept_run] {
        std::fs::create_dir_all(d).unwrap();
        chmod(d, 0o700);
    }
    let err = sessions_to(&g, &kept_out, &[Arm::Cold], 1)
        .err()
        .expect("the run must be refused");
    assert!(err.contains(&tasks), "{err}");
    for d in [&kept_out, &kept_run] {
        assert_eq!(mode_of(d), 0o700, "{} kept as it was", d.display());
    }
    // fixed, a run with a new --out starts with no manual step
    chmod(&g.closed, 0o700);
    let r = sessions_to(&g, &results.join("fixed"), &[Arm::Cold], 1)
        .unwrap_or_else(|e| panic!("after the fix: {e}"));
    assert!(r.rows[0].success, "{:?}", r.rows);
    say(
        "agent-isolation, a refused preflight: RAN as root with --agent-user nobody: three \
         refused runs (each with a new --out) left no run directory, --out or work root; a \
         --out and run directory that were there before were kept at mode 700; the fixed run \
         with a new --out started",
    );
}

/// A run that ends closes its run directory (mode 700), so the next run in the same work root,
/// with another `--out`, starts with no manual step. A resumed run opens it for its own sessions
/// and closes it again.
#[test]
fn a_finished_run_closes_its_run_directory() {
    let _g = serial();
    let Some(g) = isolated_gate("a finished run closes its run directory") else {
        return;
    };
    let user = g.cfg.agent_user.clone().unwrap();
    let runs = g.cfg.work_root.join("runs");
    let r = sessions(&g, &[Arm::Cold], 1).unwrap();
    assert!(r.rows[0].success, "{:?}", r.rows);
    assert_eq!(
        mode_of(&runs.join(RUN)),
        0o700,
        "the finished run's directory is closed"
    );
    // the next run, with another --out, in the same work root
    let next = g.out.parent().unwrap().join("next");
    let r = sessions_to(&g, &next, &[Arm::Cold], 1).unwrap_or_else(|e| panic!("the next run: {e}"));
    assert!(r.rows[0].success, "{:?}", r.rows);
    assert_eq!(mode_of(&runs.join("next")), 0o700);
    // resumed with a second rep: its session gets through the run's directory again
    let r = sessions(&g, &[Arm::Cold], 2).unwrap_or_else(|e| panic!("the resume: {e}"));
    assert_eq!(r.rows.len(), 2, "{:?}", r.rows);
    assert!(r.rows.iter().all(|row| row.success), "{:?}", r.rows);
    assert_eq!(
        read(&g.session_dir(Arm::Cold, 2).join("claude-config/fake-uid")).trim(),
        user.uid.to_string()
    );
    assert_eq!(
        mode_of(&runs.join(RUN)),
        0o700,
        "closed again after the resume"
    );
    say(
        "agent-isolation, a finished run: RAN as root with --agent-user nobody: the run's \
         directory was mode 700 after the run, a second run with another --out started with no \
         manual step, and a resume ran its session as nobody and closed the directory again",
    );
}

/// A run that stops on an error after its preflight passed (here: the same `--out` at another
/// noise level) closes its run directory too, which that preflight had opened again.
#[test]
fn a_run_that_stops_on_an_error_closes_its_run_directory() {
    let _g = serial();
    let Some(g) = isolated_gate("a run that stops on an error") else {
        return;
    };
    let run_dir = g.cfg.work_root.join("runs").join(RUN);
    let r = sessions(&g, &[Arm::Cold], 1).unwrap();
    assert!(r.rows[0].success, "{:?}", r.rows);
    let args = RunArgs {
        out: g.out.clone(),
        arms: vec![Arm::Cold],
        reps: 1,
        budget_usd: 100.0,
        seed: 1,
        confirm: true,
        dry_run: false,
        est_session_usd: None,
        pilot: None,
        meta: Meta {
            noise: Some(5),
            noise_seed: Some(1),
            ..Meta::default()
        },
    };
    let err = run(&g.specs, &args, &g.cfg)
        .err()
        .expect("another noise level in the same --out is refused");
    assert!(err.contains("one --out per noise level"), "{err}");
    assert!(
        !err.contains("refusing to run"),
        "the preflight passed: {err}"
    );
    assert_eq!(
        mode_of(&run_dir),
        0o700,
        "the run's directory is closed after the error"
    );
    say(
        "agent-isolation, a run that stops on an error: RAN as root with --agent-user nobody: \
         after a passing preflight the run stopped on an error, and its run directory was mode \
         700",
    );
}

/// An earlier run's directory the agent user can enter is refused by name, with what to do,
/// and left as it is: another run may be using it.
#[test]
fn the_preflight_names_an_open_earlier_run_directory_and_the_fix() {
    let _g = serial();
    let Some(g) = isolated_gate("preflight, an open earlier run directory") else {
        return;
    };
    let earlier = g.cfg.work_root.join("runs").join("lastnight");
    std::fs::create_dir_all(&earlier).unwrap();
    chmod(&earlier, 0o711);
    let err = sessions(&g, &[Arm::Cold], 1)
        .err()
        .expect("the run must be refused");
    for part in [
        format!("enter {}", earlier.display()),
        format!("chmod 700 {}", earlier.display()),
        "or remove it if it is empty".to_owned(),
        "another run may be using it".to_owned(),
    ] {
        assert!(err.contains(&part), "`{part}` is in: {err}");
    }
    assert_eq!(
        mode_of(&earlier),
        0o711,
        "the harness neither closes nor removes another run's directory"
    );
    chmod(&earlier, 0o700);
    let r = sessions(&g, &[Arm::Cold], 1).unwrap();
    assert!(r.rows[0].success, "{:?}", r.rows);
    say(
        "agent-isolation, an open earlier run directory: RAN as root with --agent-user nobody: \
         refused by name with `chmod 700 <dir>`, left at mode 711; closed, the run started",
    );
}

/// With an agent user the results files are mode 600 even after a resume that adds no row,
/// whatever mode they were left with.
#[test]
fn a_resume_that_adds_no_row_leaves_the_results_closed() {
    let _g = serial();
    let Some(g) = isolated_gate("results files on a resume with nothing to run") else {
        return;
    };
    let r = sessions(&g, &[Arm::Cold], 1).unwrap();
    assert!(r.rows[0].success, "{:?}", r.rows);
    let files = ["rows.jsonl", "meta.json", "report.md"].map(|f| g.out.join(f));
    for f in &files {
        assert_eq!(mode_of(f), 0o600, "{}", f.display());
        chmod(f, 0o644);
    }
    let r = sessions(&g, &[Arm::Cold], 1).unwrap();
    assert_eq!(r.rows.len(), 1, "nothing ran again: {:?}", r.rows);
    for f in &files {
        let m = std::fs::metadata(f).unwrap();
        assert_eq!(
            (m.uid(), m.mode() & 0o7777),
            (0, 0o600),
            "{} after a resume that added no row",
            f.display()
        );
    }
    say(
        "agent-isolation, results files on a resume: RAN as root with --agent-user nobody: \
         rows.jsonl, meta.json and report.md left at 644 were 600 after a resume that ran nothing",
    );
}
