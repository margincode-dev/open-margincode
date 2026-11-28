use std::path::{Path, PathBuf};
use std::process::{Command, Output};
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
const HANG: std::time::Duration = std::time::Duration::from_secs(120);

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
        // the Stop hook's keep-busy wait is off unless a test turns it on
        .env("AV_HOOK_WAIT_SECS", "0")
        // so is the footprint (it scans $HOME), and no hook writes the real session's env file
        .env("AV_FOOTPRINT", "off")
        .env_remove("CLAUDE_ENV_FILE")
        // This process may be a Grok session. Those variables would make `av hook` act as grok.
        .env_remove("GROK_HOOK_EVENT")
        .env_remove("GROK_SESSION_ID")
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
}

/// The record `av record` printed as stored, as `av expand` shows it in full.
fn expanded(d: &Daemon, ws: &Path, stdout: &str) -> String {
    ok(av(d, ws, &["expand", &recorded_handle(stdout)]))
}

#[test]
fn record_fits_an_overlong_summary_and_keeps_the_whole_text_in_body() {
    let (d, ws, _scratch) = started();
    let long = format!(
        "Shout the greeting in hello so callers see it. {}",
        "It also keeps the name as given. ".repeat(10)
    );
    let n = long.chars().count();
    let o = av(
        &d,
        &ws,
        &[
            "record",
            "--kind",
            "note",
            "--summary",
            &long,
            "--body",
            "Why: louder.",
        ],
    );
    assert!(o.status.success(), "{}", err(&o));
    let line = format!("av: records[0]: summary was {n} chars, kept ");
    assert!(err(&o).starts_with(&line), "{}", err(&o));
    assert!(err(&o).contains("the whole text is in body"), "{}", err(&o));
    assert_eq!(err(&o).lines().count(), 1, "{}", err(&o));
    // `av expand` indents each line of the body
    let full = expanded(&d, &ws, &out(&o));
    assert!(
        full.contains(&format!("\n  {long}\n  \n  Why: louder.")),
        "{full}"
    );

    let items = format!(
        r#"[{{"kind": "decision", "summary": "fits"}},
            {{"kind": "attempt", "summary": "{}", "body": "it broke"}}]"#,
        "word ".repeat(50)
    );
    let o = av_stdin(&d, &ws, &["record", "--json", "-"], &items);
    assert!(o.status.success(), "{}", err(&o));
    assert_eq!(out(&o).matches("recorded rec:").count(), 2, "{}", out(&o));
    assert!(
        err(&o).starts_with("av: records[1]: summary was 250 chars, kept 200;"),
        "{}",
        err(&o)
    );
    let second = out(&o).lines().nth(1).unwrap().to_owned();
    let full = expanded(&d, &ws, &second);
    assert!(
        full.contains(&format!("\n  {}\n  \n  it broke", "word ".repeat(50))),
        "{full}"
    );

    std::fs::write(
        ws.join("app/greeter.py"),
        "class Greeter:\n    def hello(self, name):\n        return name.upper()\n",
    )
    .unwrap();
    let spec = format!("decision: {} @ app/greeter.py", "x".repeat(230));
    let o = av(&d, &ws, &["submit", "--record", &spec]);
    assert!(o.status.success(), "{}", err(&o));
    assert!(
        err(&o).starts_with("av: records[0]: summary was 230 chars, kept 200;"),
        "{}",
        err(&o)
    );
    assert!(out(&o).contains("recorded 1"), "{}", out(&o));

    let finding = format!("{}|app/greeter.py:Greeter.hello|", "y ".repeat(120));
    let o = av_as(
        &d,
        &ws,
        "rev",
        &[
            "review",
            "--verdict",
            "request-changes",
            "--finding",
            &finding,
        ],
    );
    assert!(o.status.success(), "{}", err(&o));
    assert!(
        err(&o).starts_with("av: findings[0]: summary was 240 chars, kept 200;"),
        "{}",
        err(&o)
    );
}

#[test]
fn submit_with_record_flags_records_and_submits_in_one_call() {
    let (d, ws, _scratch) = started();
    std::fs::write(
        ws.join("app/greeter.py"),
        "class Greeter:\n    def hello(self, name):\n        return name.upper()\n",
    )
    .unwrap();
    let stdout = ok(av(
        &d,
        &ws,
        &[
            "submit",
            "--record",
            "decision: Shout the greeting @ app/greeter.py:Greeter.hello",
        ],
    ));
    assert!(stdout.contains("submitted (in_review)"), "{stdout}");
    assert!(
        stdout.contains("av review"),
        "the daemon's next step: {stdout}"
    );
    assert!(stdout.contains("recorded 1"), "{stdout}");
    let st = ok(av(&d, &ws, &["status"]));
    assert!(st.contains("in_review"), "{st}");
    let md = ok(av(&d, &ws, &["context", "--budget", "3000"]));
    assert!(md.contains("Shout the greeting"), "{md}");
    let bad = av(&d, &ws, &["submit", "--record", "no colon here"]);
    assert!(
        !bad.status.success() && err(&bad).contains("KIND: SUMMARY @ path[:Symbol]"),
        "{}",
        err(&bad)
    );
}

fn two_records_json() -> &'static str {
    r#"[{"kind": "decision", "summary": "first record", "anchors": ["app/greeter.py:Greeter.hello"]},
        {"kind": "attempt", "summary": "second record", "body": "why", "anchors": ["README.md"]}]"#
}

#[test]
fn record_json_writes_every_record() {
    let (d, ws, _scratch) = started();
    let file = ws.join("..").join("records.json");
    std::fs::write(&file, two_records_json()).unwrap();
    let o = ok(av(&d, &ws, &["record", "--json", file.to_str().unwrap()]));
    assert_eq!(o.matches("recorded rec:").count(), 2, "{o}");
    let o = ok(av_stdin(
        &d,
        &ws,
        &["record", "--json", "-"],
        two_records_json(),
    ));
    assert_eq!(o.matches("recorded rec:").count(), 2, "{o}");
    let md = ok(av(&d, &ws, &["context", "--budget", "3000"]));
    assert!(
        md.contains("first record") && md.contains("second record"),
        "{md}"
    );
    let neither = av(&d, &ws, &["record"]);
    assert!(!neither.status.success(), "{}", err(&neither));
    let both = av(
        &d,
        &ws,
        &["record", "--json", "-", "--kind", "note", "--summary", "x"],
    );
    assert!(!both.status.success(), "{}", err(&both));
}

#[test]
fn record_json_checks_every_item_before_sending_any() {
    let (d, ws, _scratch) = started();
    let items = format!(
        r#"[{{"kind": "decision", "summary": "{}"}},
            {{"kind": "idea", "summary": "x"}}]"#,
        "first fine record ".repeat(15)
    );
    let o = av_stdin(&d, &ws, &["record", "--json", "-"], &items);
    let e = err(&o);
    assert!(
        !o.status.success() && e.contains("records[1]") && e.contains("idea"),
        "{e}"
    );
    assert!(!e.contains("whole text"), "nothing was fitted: {e}");
    let md = ok(av(&d, &ws, &["context", "--budget", "3000"]));
    assert!(!md.contains("first fine record"), "nothing was sent: {md}");
}

fn recorded_handle(line: &str) -> String {
    let at = line.find("rec:").unwrap_or_else(|| panic!("{line}"));
    line[at..at + 12].to_owned()
}

#[test]
fn record_supersedes_corrects_a_record_of_another_task() {
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
    let wrong = recorded_handle(&ok(av(
        &d,
        &other,
        &[
            "record",
            "--kind",
            "attempt",
            "--summary",
            "caching hello can never work",
            "--anchor",
            "app/greeter.py",
        ],
    )));

    let line = ok(av(
        &d,
        &ws,
        &[
            "record",
            "--kind",
            "note",
            "--summary",
            "caching hello works with a ttl",
            "--supersedes",
            &wrong,
        ],
    ));
    let fixed = recorded_handle(&line);
    assert_eq!(
        line.trim(),
        format!("recorded {fixed} (note), supersedes {wrong}")
    );
    let md = ok(av(&d, &ws, &["context", "--budget", "3000", "--fresh"]));
    assert!(md.contains("caching hello works with a ttl"), "{md}");
    assert!(md.contains("anchors: app/greeter.py"), "{md}");
    assert!(!md.contains("can never work"), "{md}");

    let batch = format!(
        r#"[{{"kind": "note", "summary": "the ttl is 60 seconds", "supersedes": "{fixed}"}}]"#
    );
    let line = ok(av_stdin(&d, &ws, &["record", "--json", "-"], &batch));
    assert!(
        line.trim()
            .ends_with(&format!("(note), supersedes {fixed}")),
        "{line}"
    );

    let again = av(
        &d,
        &ws,
        &[
            "record",
            "--kind",
            "note",
            "--summary",
            "once more",
            "--supersedes",
            &wrong,
        ],
    );
    let e = err(&again);
    assert!(
        !again.status.success() && e.contains("already superseded") && e.contains("rec:"),
        "{e}"
    );
    let not_a_handle = r#"[{"kind": "note", "summary": "fine"},
                           {"kind": "note", "summary": "x", "supersedes": 7}]"#;
    let o = av_stdin(&d, &ws, &["record", "--json", "-"], not_a_handle);
    let e = err(&o);
    assert!(
        !o.status.success() && e.contains("records[1]") && e.contains("supersedes"),
        "{e}"
    );
    let both = av(&d, &ws, &["record", "--json", "-", "--supersedes", &wrong]);
    assert!(!both.status.success(), "{}", err(&both));
}

#[test]
fn context_defaults_to_800_tokens() {
    let (d, ws, _scratch) = started();
    let o = av(&d, &ws, &["context", "--fresh"]);
    let stderr = err(&o);
    ok(o);
    let rcp = stderr
        .lines()
        .find_map(|l| l.strip_prefix("av: receipt "))
        .unwrap_or_else(|| panic!("no receipt line in stderr: {stderr}"))
        .trim()
        .to_owned();
    let shown = ok(av(&d, &ws, &["receipt", &rcp]));
    assert!(shown.contains("/800 tokens"), "{shown}");
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
fn attention_lists_an_overdue_target_as_json_and_as_lines() {
    let (d, _ws, scratch) = started();
    let quiet = ok(av(&d, scratch.path(), &["attention", "--repo", "demo"]));
    assert_eq!(
        quiet,
        "nothing needs attention
"
    );
    ok(av(
        &d,
        scratch.path(),
        &[
            "target",
            "add",
            "--repo",
            "demo",
            "--title",
            "Release 1",
            "--due",
            "2020-01-01",
        ],
    ));
    let v: serde_json::Value =
        serde_json::from_str(&ok(av(&d, scratch.path(), &["attention", "--json"]))).unwrap();
    let items = v["items"].as_array().unwrap();
    assert_eq!(items.len(), 1, "{v}");
    assert_eq!(items[0]["kind"], "overdue_target");
    assert_eq!(items[0]["severity"], 1);
    assert_eq!(items[0]["repo"], "demo");
    assert_eq!(items[0]["title"], "Release 1");
    assert!(v["generated_at"].is_string(), "{v}");
    let lines = ok(av(&d, scratch.path(), &["attention", "--repo", "demo"]));
    assert_eq!(lines.lines().count(), 1, "{lines}");
    assert!(
        lines.starts_with("1 overdue_target") && lines.contains("demo  Release 1 · due 2020-01-01"),
        "{lines}"
    );
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
fn a_person_writes_the_constitution_and_an_agent_cannot() {
    let (d, ws, scratch) = started();
    let at = |args: &[&str]| av(&d, scratch.path(), args);
    let added = ok(at(&[
        "constitution",
        "add",
        "--repo",
        "demo",
        "--title",
        "Never log a key",
        "--check",
        "make lint",
        "--reason",
        "we agreed",
        "No provider key in a log line.",
    ]));
    assert!(added.contains("art:1 added (v1)"), "{added}");
    ok(at(&[
        "constitution",
        "amend",
        "1",
        "--repo",
        "demo",
        "--no-check",
        "--reason",
        "no linter yet",
    ]));
    ok(at(&[
        "constitution",
        "add",
        "--repo",
        "demo",
        "--title",
        "Second",
        "--reason",
        "why",
        "Another rule.",
    ]));
    let repealed = ok(at(&[
        "constitution",
        "repeal",
        "2",
        "--repo",
        "demo",
        "--reason",
        "gone",
    ]));
    assert!(repealed.contains("art:2 repealed (v4)"), "{repealed}");
    // from the task's workspace, the repository is the task's
    let shown = ok(av(&d, &ws, &["constitution", "show"]));
    assert!(
        shown.starts_with(
            "constitution of demo: v4 (it outranks conventions and records)
"
        ) && shown.contains(
            "art:1 Never log a key
  No provider key in a log line.
"
        ) && !shown.contains("Second"),
        "{shown}"
    );
    let history = ok(at(&["constitution", "history", "--repo", "demo"]));
    let lines: Vec<&str> = history.lines().collect();
    assert_eq!(lines.len(), 4, "{history}");
    assert!(
        lines[1].contains("demo v2 amend art:1 by user: no linter yet"),
        "{history}"
    );
    ok(at(&[
        "--agent", "claude", "task", "start", "--repo", "demo", "--title", "Other", "--goal", "x",
    ]));
    let refused = at(&[
        "--agent",
        "claude",
        "constitution",
        "add",
        "--repo",
        "demo",
        "--title",
        "Mine",
        "--reason",
        "why",
        "text",
    ]);
    assert!(!refused.status.success());
    assert!(
        err(&refused).contains("the constitution is amended by a person"),
        "{}",
        err(&refused)
    );

    ok(at(&[
        "convention",
        "add",
        "--repo",
        "demo",
        "--scope",
        "docs/**",
        "no provider key in a log line.",
    ]));
    let list = ok(av(&d, &ws, &["convention", "list"]));
    assert!(
        list.contains("note: conv:") && list.contains("matches no file on main"),
        "{list}"
    );
    assert!(list.contains("have the same text"), "{list}");
    assert!(ok(av(&d, &ws, &["constitution", "show"])).contains("have the same text"));
}

#[test]
fn a_required_check_scoped_to_every_path_gets_a_hint() {
    let (d, _ws, scratch) = started();
    let add = |scope: &str| {
        ok(av(
            &d,
            scratch.path(),
            &[
                "convention",
                "add",
                "--repo",
                "demo",
                "--scope",
                scope,
                "--required",
                "--check",
                "true",
                "tests pass",
            ],
        ))
    };
    let out = add("**");
    assert!(out.contains("(required, check)"), "{out}");
    assert!(
        out.contains("hint: scope \"**\" matches every path"),
        "{out}"
    );
    assert!(out.contains("--scope 'crates/**'"), "{out}");
    let out = add("app/**");
    assert!(!out.contains("hint:"), "{out}");
}

/// `{crates}` is expanded before the check runs, and the line says what ran. The demo repo is
/// no cargo workspace, so it is the whole workspace (the scope rule is unit-tested in
/// `crates_scope`).
#[test]
fn validate_expands_crates_in_a_check_and_prints_what_ran() {
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
            "--check",
            "test \"{crates}\" = --workspace",
            "tests pass",
        ],
    ));
    std::fs::write(
        ws.join("app/greeter.py"),
        "def hello():\n    return 'louder'\n",
    )
    .unwrap();
    let passed = ok(av(&d, &ws, &["validate", "--", "true"]));
    assert!(
        passed.contains("check conv:1 passed (`test \"--workspace\" = --workspace`"),
        "{passed}"
    );
}

/// A `CARGO_TARGET_DIR` from the user's environment would share one build dir across task workspaces; `av validate` points it at the workspace's own `target/`.
#[test]
fn validate_builds_a_cargo_workspace_into_its_own_target_dir() {
    let (d, ws, scratch) = started();
    std::fs::write(ws.join("Cargo.toml"), "[workspace]\n").unwrap();
    std::fs::create_dir_all(ws.join("target")).unwrap(); // so both sides canonicalize
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
            "--check",
            "printf '%s|%s' \"$CARGO_TARGET_DIR\" \"${CARGO_BUILD_TARGET_DIR-unset}\" > \"$SEEN\"",
            "tests pass",
        ],
    ));
    std::fs::write(
        ws.join("app/greeter.py"),
        "def hello():\n    return 'louder'\n",
    )
    .unwrap();
    let shared = scratch.path().join("shared-target");
    let seen = scratch.path().join("seen-check");
    let seen_cmd = scratch.path().join("seen-command");
    ok(av_env(
        &d.url,
        &ws,
        &[
            ("CARGO_TARGET_DIR", shared.to_str().unwrap()),
            ("CARGO_BUILD_TARGET_DIR", shared.to_str().unwrap()),
            ("SEEN", seen.to_str().unwrap()),
            ("SEEN_CMD", seen_cmd.to_str().unwrap()),
        ],
        &[
            "validate",
            "--",
            "sh",
            "-c",
            "printf '%s|%s' \"$CARGO_TARGET_DIR\" \"${CARGO_BUILD_TARGET_DIR-unset}\" > \"$SEEN_CMD\"",
        ],
    ));
    for file in [&seen, &seen_cmd] {
        let got = std::fs::read_to_string(file).unwrap();
        let (dir, build) = got.split_once('|').unwrap();
        assert_eq!(build, "unset", "{got}");
        assert_eq!(
            Path::new(dir).canonicalize().unwrap(),
            ws.join("target").canonicalize().unwrap(),
            "{got}"
        );
    }
}

/// `sh -c "false | true"` exits 0: the warning says so, and the attestation still goes in.
#[test]
fn validate_warns_about_an_unguarded_pipeline_and_still_attests() {
    let (d, ws, scratch) = started();
    let o = av(&d, &ws, &["validate", "--", "sh", "-c", "false | true"]);
    assert!(o.status.success(), "{}", err(&o));
    assert!(out(&o).contains("validation passed"), "{}", out(&o));
    assert!(err(&o).contains("set -o pipefail"), "{}", err(&o));
    let o = av(
        &d,
        &ws,
        &[
            "validate",
            "--",
            "sh",
            "-c",
            "set -o pipefail; false | true",
        ],
    );
    assert!(!err(&o).contains("pipeline"), "{}", err(&o));
    let o = av(
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
            "t | tail",
            "tests",
        ],
    );
    assert!(
        o.status.success() && err(&o).contains("set -o pipefail"),
        "{}",
        err(&o)
    );
    std::fs::write(ws.join("app/greeter.py"), "def hello():\n    pass\n").unwrap();
    let o = av(&d, &ws, &["validate", "--", "true"]);
    assert!(
        err(&o).contains("this check pipes without pipefail"),
        "a stored check is warned about where it runs: {}",
        err(&o)
    );
}

/// The task's change since its base, as of its last checkpoint (`av validate` checkpoints first).
#[test]
fn diff_check_fails_on_trailing_whitespace_and_passes_clean() {
    let (d, ws, _scratch) = started();
    std::fs::write(ws.join("notes.md"), "fine\nspace \n").unwrap();
    ok(av(&d, &ws, &["checkpoint"]));
    let o = av(&d, &ws, &["diff", "--check"]);
    assert_eq!(o.status.code(), Some(2), "{}{}", out(&o), err(&o));
    assert!(
        out(&o).contains("notes.md:2: trailing whitespace"),
        "{}",
        out(&o)
    );
    std::fs::write(ws.join("notes.md"), "fine\nclean").unwrap();
    let bin = env!("CARGO_BIN_EXE_av");
    let o = av(&d, &ws, &["validate", "--", bin, "diff", "--check"]);
    assert!(
        out(&o).contains("notes.md:2: no newline at end of file"),
        "validate checkpoints first: {}",
        out(&o)
    );
    std::fs::write(ws.join("notes.md"), "fine\nclean\n").unwrap();
    ok(av(&d, &ws, &["checkpoint"]));
    let clean = ok(av(&d, &ws, &["diff", "--check"]));
    assert!(clean.contains("in 1 text file(s)"), "{clean}");
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
fn status_of_a_conflicted_task_says_what_is_left() {
    let d = start_daemon();
    let scratch = TempDir::new().unwrap();
    let s = scratch.path();
    ok(av(&d, s, &["repo", "add", "demo", d.src.to_str().unwrap()]));
    let started = ok(av_as(
        &d,
        s,
        "dev",
        &[
            "task", "start", "--repo", "demo", "--title", "Second", "--goal", "Second",
        ],
    ));
    let ws = PathBuf::from(
        started
            .lines()
            .next()
            .unwrap()
            .trim_start_matches("workspace: "),
    );
    merge_change(&d, s, "First", "app/util.py", "FIRST = 1\n");
    std::fs::write(ws.join("app/util.py"), "SECOND = 2\n").unwrap();
    ok(av_as(
        &d,
        &ws,
        "dev",
        &["record", "--kind", "note", "--summary", "second"],
    ));
    ok(av_as(&d, &ws, "dev", &["validate", "--", "true"]));
    ok(av_as(&d, &ws, "dev", &["submit"]));
    ok(av_as(&d, &ws, "rev", &["review", "--verdict", "approve"]));
    let merged = av_as(&d, &ws, "dev", &["merge"]);
    assert!(!merged.status.success(), "{}", out(&merged));

    let st = ok(av_as(&d, &ws, "dev", &["status"]));
    assert!(st.contains("conflicted"), "{st}");
    assert!(st.contains("next: ") && st.contains("av sync"), "{st}");
    ok(av_as(&d, &ws, "dev", &["sync"]));
    let st = ok(av_as(&d, &ws, "dev", &["status"]));
    assert!(
        st.contains("conflicts at the last merge or sync: app/util.py"),
        "{st}"
    );
    assert!(
        st.contains("next: resolve the conflict markers in app/util.py"),
        "{st}"
    );
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
fn repo_add_reports_a_failed_import() {
    let d = start_daemon();
    let scratch = TempDir::new().unwrap();
    // passes the daemon's quick look (it has a .git), then git cannot clone it
    std::fs::create_dir(scratch.path().join(".git")).unwrap();
    let src = scratch.path().to_str().unwrap();
    let o = av(&d, scratch.path(), &["repo", "add", "x", src]);
    assert!(!o.status.success());
    assert!(err(&o).contains("import of x failed"), "{}", err(&o));
}

#[test]
fn repo_add_of_a_path_the_daemon_cannot_see_says_where_it_looks() {
    let d = start_daemon();
    let scratch = TempDir::new().unwrap();
    let o = av(&d, scratch.path(), &["repo", "add", "x", "/nowhere"]);
    assert!(!o.status.success());
    let e = err(&o);
    assert!(
        e.contains("/nowhere does not exist")
            && e.contains("daemon's side")
            && e.contains("/import"),
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

    // another named agent is refused, with both names; an unnamed one cannot be told from codex's
    // own unnamed hooks, so it is let through with a warning
    let other = av_as(&d, &ws, "claude", &["checkpoint"]);
    assert!(!other.status.success(), "{}", out(&other));
    assert!(
        err(&other).contains("`codex` now owns this task; you are `claude`"),
        "{}",
        err(&other)
    );
    let unnamed = av(&d, &ws, &["checkpoint"]);
    assert!(unnamed.status.success(), "{}", err(&unnamed));
    assert!(
        err(&unnamed).contains("acting on `codex`'s task with no agent name"),
        "{}",
        err(&unnamed)
    );
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
fn repo_backup_branch_names_where_main_is_pushed() {
    let (d, ws, scratch) = started();
    let remote = TempDir::new().unwrap();
    git(remote.path(), &["init", "-q", "--bare", "-b", "main"]);
    let url = remote.path().to_str().unwrap();
    let set = ok(av(
        &d,
        scratch.path(),
        &[
            "repo", "backup", "demo", "--remote", url, "--branch", "av-main", "--push",
        ],
    ));
    assert!(
        set.contains(&format!("backup: {url} (branch av-main) · pushed ")),
        "{set}"
    );
    let status = ok(av(&d, &ws, &["status"]));
    assert!(
        status.contains(&format!("backup: {url} (branch av-main)")),
        "{status}"
    );
    let out = av(
        &d,
        scratch.path(),
        &["repo", "backup", "demo", "--branch", "x"],
    );
    assert!(!out.status.success());
    ok(av(
        &d,
        scratch.path(),
        &["repo", "backup", "demo", "--clear"],
    ));
}

#[test]
fn repo_advance_takes_main_back_from_the_remote() {
    let (d, _ws, scratch) = started();
    let remote = TempDir::new().unwrap();
    git(remote.path(), &["init", "-q", "--bare", "-b", "main"]);
    let url = remote.path().to_str().unwrap();
    let set = ok(av(
        &d,
        scratch.path(),
        &[
            "repo",
            "backup",
            "demo",
            "--set",
            url,
            "--advance",
            "--push",
        ],
    ));
    assert!(set.contains("advance from main"), "{set}");
    let same = ok(av(&d, scratch.path(), &["repo", "advance", "demo"]));
    assert!(same.trim().starts_with("main up to date at "), "{same}");

    let clone = TempDir::new().unwrap();
    git(clone.path(), &["clone", "-q", url, "."]);
    std::fs::write(
        clone.path().join("farewell.py"),
        "def bye():\n    return 1\n",
    )
    .unwrap();
    git(clone.path(), &["add", "-A"]);
    git(clone.path(), &["commit", "-q", "-m", "farewell"]);
    git(clone.path(), &["push", "-q", "origin", "main"]);
    let moved = ok(av(
        &d,
        scratch.path(),
        &[
            "repo", "advance", "demo", "--remote", url, "--branch", "main",
        ],
    ));
    assert!(moved.contains("main fast-forwarded"), "{moved}");
    let out = av(&d, scratch.path(), &["repo", "backup", "demo", "--advance"]);
    assert!(!out.status.success());
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
fn hook_session_start_asks_for_the_record_before_the_final_message() {
    let (d, ws, _scratch) = started();
    let printed = ok(av(&d, &ws, &["hook", "session-start"]));
    let at = printed
        .find("\nav: before your final message")
        .unwrap_or_else(|| panic!("{printed}"));
    assert!(
        printed.find("## Task").unwrap() < at,
        "after the pack: {printed}"
    );
    let line = printed[at + 1..].lines().next().unwrap();
    // with only `attempt` named, agents invented a kind ("fix", "change");
    // a symbol anchor alone missed a changed constant beside the function, and a file anchor
    // alone ranks lower for a later task on that function
    assert!(
        line.contains("in one call")
            && line.contains("av record --json -")
            && line.contains(&format!("≤ {} chars", margincode_core::MAX_SUMMARY_CHARS))
            && line.contains("Kinds: decision, note, attempt")
            && line.contains("\"anchors\": [\"src/app.py:changed_fn\", \"src/app.py\"]")
            && line.contains("each symbol you changed plus its file's bare path")
            && line.ends_with("Otherwise your stop is blocked to ask for it."),
        "{line}"
    );
    assert!(line.split_whitespace().count() < 60, "{line}");
    assert!(!line.contains("margincode"), "{line}");
    // the example is what `av record --json -` takes, as written
    let example = &line[line.find("[{").unwrap()..line.find("}]").unwrap() + 2];
    assert!(example.contains("\"kind\": \"decision\""), "{example}");
    let o = av_stdin(&d, &ws, &["record", "--json", "-"], example);
    assert!(o.status.success(), "{example}: {}", err(&o));
    let no_pack = av_with("http://127.0.0.1:1", &ws, &["hook", "session-start"]);
    assert!(no_pack.status.success(), "{}", err(&no_pack));
    assert!(
        !out(&no_pack).contains("before your final message"),
        "{}",
        out(&no_pack)
    );
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

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

fn last_checkpoint_ms(ws: &Path) -> Option<u64> {
    let tf: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(ws.join(".av-task")).unwrap()).unwrap();
    tf["last_checkpoint_ms"].as_u64()
}

fn set_last_checkpoint_ms(ws: &Path, ms: u64) {
    let mut tf: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(ws.join(".av-task")).unwrap()).unwrap();
    tf["last_checkpoint_ms"] = ms.into();
    std::fs::write(ws.join(".av-task"), tf.to_string()).unwrap();
}

#[test]
fn hook_post_edit_debounces_within_two_seconds() {
    let (d, ws, _scratch) = started();
    let before = now_ms();
    std::fs::write(ws.join("app/greeter.py"), "class Greeter:\n    pass\n").unwrap();
    ok(av(&d, &ws, &["hook", "post-edit"]));
    let status = ok(av(&d, &ws, &["status"]));
    assert!(status.contains("changed (1): app/greeter.py"), "{status}");
    let stamp = last_checkpoint_ms(&ws);
    assert!(stamp.is_some_and(|t| t >= before), "{stamp:?}");
    // an hour ahead, so the hook is inside the window however late it runs
    set_last_checkpoint_ms(&ws, now_ms() + 3_600_000);
    std::fs::write(ws.join("README.md"), "# changed\n").unwrap();
    ok(av(&d, &ws, &["hook", "post-edit"]));
    let status = ok(av(&d, &ws, &["status"]));
    assert!(
        status.contains("changed (1):"),
        "an edit inside the window is debounced: {status}"
    );
    let old = now_ms() - 2_100;
    set_last_checkpoint_ms(&ws, old);
    ok(av(&d, &ws, &["hook", "post-edit"]));
    let status = ok(av(&d, &ws, &["status"]));
    assert!(status.contains("changed (2):"), "{status}");
    assert_ne!(last_checkpoint_ms(&ws), Some(old));
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
        .env_remove("GROK_HOOK_EVENT")
        .env_remove("GROK_SESSION_ID")
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
fn install_sh_sets_the_stop_hook_timeout_above_the_keep_busy_wait() {
    if Command::new("jq").arg("--version").output().is_err() {
        eprintln!("jq not installed; skipping");
        return;
    }
    let script = repo_root().join("integrations/claude-code/install.sh");
    let timeout_after = |wait: Option<&str>| -> (serde_json::Value, bool) {
        let home = TempDir::new().unwrap();
        let mut cmd = Command::new("bash");
        cmd.arg(&script)
            .env("HOME", home.path())
            .env_remove("AV_HOOK_WAIT_SECS");
        if let Some(w) = wait {
            cmd.env("AV_HOOK_WAIT_SECS", w);
        }
        let o = cmd.output().unwrap();
        let settings: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(home.path().join(".claude/settings.json")).unwrap_or_default(),
        )
        .unwrap_or_default();
        (
            settings["hooks"]["Stop"][0]["hooks"][0]["timeout"].clone(),
            o.status.success(),
        )
    };
    // default wait 600 s: the hook may run 660 s
    assert_eq!(timeout_after(None), (serde_json::json!(660), true));
    assert_eq!(timeout_after(Some("30")), (serde_json::json!(90), true));
    assert_eq!(timeout_after(Some("0")), (serde_json::json!(60), true));
    let (t, ok) = timeout_after(Some("soon"));
    assert!(!ok && t.is_null(), "{t}");
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
fn av_wrap_points_cargo_at_the_workspaces_own_target_dir() {
    let (d, ws, scratch) = started();
    std::fs::write(ws.join("Cargo.toml"), "[workspace]\n").unwrap();
    std::fs::create_dir_all(ws.join("target")).unwrap();
    let bin = Path::new(env!("CARGO_BIN_EXE_av"))
        .parent()
        .unwrap()
        .to_owned();
    let path = format!("{}:{}", bin.display(), std::env::var("PATH").unwrap());
    let seen = scratch.path().join("seen");
    let o = Command::new("bash")
        .arg(repo_root().join("integrations/generic/av-wrap.sh"))
        .args([
            "sh",
            "-c",
            r#"printf '%s|%s' "$CARGO_TARGET_DIR" "${CARGO_BUILD_TARGET_DIR-unset}" > "$SEEN""#,
        ])
        .current_dir(ws.join("app"))
        .env("PATH", path)
        .env("AV_URL", &d.url)
        .env("SEEN", &seen)
        .env("CARGO_TARGET_DIR", scratch.path())
        .env("CARGO_BUILD_TARGET_DIR", scratch.path())
        .env_remove("AV_TASK")
        .env_remove("AV_AGENT")
        .output()
        .unwrap();
    assert!(o.status.success(), "{}", err(&o));
    let got = std::fs::read_to_string(&seen).unwrap();
    let (dir, build) = got.split_once('|').unwrap();
    assert_eq!(build, "unset", "{got}");
    assert_eq!(
        Path::new(dir).canonicalize().unwrap(),
        ws.join("target").canonicalize().unwrap(),
        "{got}"
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
            .env_remove("AV_AGENT")
            .env_remove("GROK_HOOK_EVENT")
            .env_remove("GROK_SESSION_ID")
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
    let compacted = session(&ws);
    assert_ne!(
        compacted, before,
        "after compaction everything is sent again"
    );
    assert!(pack.contains("## Task"), "{pack}");

    let resumed = run("resume");
    assert_eq!(session(&ws), compacted);
    assert!(resumed.contains("sent earlier"), "{resumed}");
    let fresh = run("startup");
    assert_ne!(session(&ws), compacted, "a new process starts a new ledger");
    assert!(!fresh.contains("sent earlier"), "{fresh}");
    assert!(fresh.contains("## Task"), "{fresh}");
}

fn av_stdin(d: &Daemon, cwd: &Path, args: &[&str], stdin: &str) -> Output {
    av_stdin_env(&d.url, cwd, &[], args, stdin)
}

/// `av_stdin` with extra environment; `AV_AGENT`, Grok's hook variables and the keep-busy wait are off unless `env` sets them.
fn av_stdin_env(url: &str, cwd: &Path, env: &[(&str, &str)], args: &[&str], stdin: &str) -> Output {
    use std::io::Write as _;
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_av"));
    cmd.current_dir(cwd)
        .env("AV_URL", url)
        .env_remove("AV_TASK")
        .env_remove("AV_SESSION")
        .env_remove("AV_AGENT")
        .env_remove("AV_TOKEN")
        .env_remove("GROK_HOOK_EVENT")
        .env_remove("GROK_SESSION_ID")
        .env("AV_HOOK_WAIT_SECS", "0")
        .env_remove("AV_HOOK_MAX_CONTINUES")
        .env("AV_FOOTPRINT", "off")
        .env_remove("CLAUDE_ENV_FILE")
        .env_remove("AV_FOOTPRINT_ROOTS")
        .env_remove("AV_WS_ROOT");
    for (k, v) in env {
        cmd.env(k, v);
    }
    let mut child = cmd
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
    assert!(
        ctx.contains("`av record --supersedes <id>`"),
        "says how to correct a wrong one: {ctx}"
    );
    let o = av_stdin(&d, &ws, &["hook", "pre-read"], &input);
    assert_eq!((ok(o).as_str()), "");
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

fn stop_input(ws: &Path, session: &str, active: bool) -> String {
    serde_json::json!({
        "hook_event_name": "Stop", "session_id": session, "stop_hook_active": active, "cwd": ws,
    })
    .to_string()
}

#[test]
fn hook_stop_asks_once_per_session_to_record_unexplained_changes() {
    let (d, ws, _scratch) = started();
    assert_eq!(
        ok(av_stdin(
            &d,
            &ws,
            &["hook", "stop"],
            &stop_input(&ws, "cc-0", false)
        )),
        ""
    );
    std::fs::write(
        ws.join("app/greeter.py"),
        "class Greeter:\n    def hello(self, name):\n        return name.upper()\n",
    )
    .unwrap();
    let o = ok(av_stdin(
        &d,
        &ws,
        &["hook", "stop"],
        &stop_input(&ws, "cc-1", false),
    ));
    let v: serde_json::Value = serde_json::from_str(&o).unwrap();
    assert_eq!(v["decision"], "block", "{o}");
    let reason = v["reason"].as_str().unwrap();
    assert!(reason.contains("Greeter.hello"), "{reason}");
    assert!(
        reason.contains("av submit --record") && reason.contains("av record --json"),
        "{reason}"
    );
    // asked once: the same session may stop now, and so may a Stop hook already continuing
    assert_eq!(
        ok(av_stdin(
            &d,
            &ws,
            &["hook", "stop"],
            &stop_input(&ws, "cc-1", false)
        )),
        ""
    );
    assert_eq!(
        ok(av_stdin(
            &d,
            &ws,
            &["hook", "stop"],
            &stop_input(&ws, "cc-2", true)
        )),
        ""
    );
    ok(av(
        &d,
        &ws,
        &[
            "record",
            "--kind",
            "decision",
            "--summary",
            "shout the name",
            "--anchor",
            "app/greeter.py",
        ],
    ));
    assert_eq!(
        ok(av_stdin(
            &d,
            &ws,
            &["hook", "stop"],
            &stop_input(&ws, "cc-3", false)
        )),
        ""
    );
}

/// A temp `HOME`, `TMPDIR` and `AV_HOME` with a `PATH` of one empty dir (`bin`), so a test's fake `docker` goes there and the host's real one is never seen.
struct Footprint {
    home: TempDir,
    tmp: TempDir,
    av_home: TempDir,
    bin: TempDir,
    env_file: PathBuf,
}

impl Footprint {
    fn new() -> Self {
        let home = TempDir::new().unwrap();
        let env_file = home.path().join("claude-env");
        Footprint {
            home,
            tmp: TempDir::new().unwrap(),
            av_home: TempDir::new().unwrap(),
            bin: TempDir::new().unwrap(),
            env_file,
        }
    }

    fn env<'a>(&'a self, on: &'a str) -> Vec<(&'a str, &'a str)> {
        vec![
            ("HOME", self.home.path().to_str().unwrap()),
            ("TMPDIR", self.tmp.path().to_str().unwrap()),
            ("AV_HOME", self.av_home.path().to_str().unwrap()),
            ("PATH", self.bin.path().to_str().unwrap()),
            ("CLAUDE_ENV_FILE", self.env_file.to_str().unwrap()),
            ("AV_FOOTPRINT", on),
            ("AV_FOOTPRINT_WARN_MB", "1"),
            // as if the user's shell named no target dir (this test process may have one); a
            // test that wants one pushes it after these
            ("CARGO_TARGET_DIR", ""),
            ("CARGO_BUILD_TARGET_DIR", ""),
        ]
    }

    fn snapshot(&self, session: &str) -> PathBuf {
        self.av_home
            .path()
            .join("footprint")
            .join(format!("{session}.json"))
    }

    /// A fake `docker` on the hooks' `PATH`: logs every call to `docker.log` in `bin` (one line
    /// of arguments each), answers `ps -a --size …` (the labelled scan), `images -q`,
    /// `image inspect` and `volume ls` with the files `ps`, `images`, `inspect` and `volumes` of
    /// `bin`, `ps -aq` (the session-start ids) with `ids` and `ps -a --no-trunc …` (the
    /// unlabelled net) with `all` (empty when missing; the filters are ignored, so a test writes
    /// only what docker would match), and succeeds silently on anything else (`rm`, `rmi`,
    /// `volume rm`).
    #[cfg(unix)]
    fn fake_docker(&self) {
        let bin = self.bin.path().display();
        write_script(
            &self.bin.path().join("docker"),
            &format!(
                "#!/bin/sh\nPATH=/usr/bin:/bin\necho \"$*\" >> '{bin}/docker.log'\n\
                 case \"$1 $2\" in\n\
                 \"ps -aq\") cat '{bin}/ids' 2>/dev/null ;;\n\
                 \"ps -a\") if [ \"$3\" = --no-trunc ]; then cat '{bin}/all'; \
                 else cat '{bin}/ps'; fi 2>/dev/null ;;\n\
                 \"ps \"*) cat '{bin}/ps' 2>/dev/null ;;\n\
                 \"images \"*) cat '{bin}/images' 2>/dev/null ;;\n\
                 \"image inspect\") cat '{bin}/inspect' 2>/dev/null ;;\n\
                 \"volume ls\") cat '{bin}/volumes' 2>/dev/null ;;\n\
                 esac\nexit 0\n"
            ),
        );
    }

    #[cfg(unix)]
    fn docker_log(&self) -> String {
        std::fs::read_to_string(self.bin.path().join("docker.log")).unwrap_or_default()
    }

    #[cfg(unix)]
    fn docker_answers(&self, file: &str, text: &str) {
        std::fs::write(self.bin.path().join(file), text).unwrap();
    }
}

fn task_id_of(ws: &Path) -> String {
    let tf: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(ws.join(".av-task")).unwrap()).unwrap();
    tf["task"].as_str().unwrap().to_owned()
}

fn labels_line(ws: &Path, agent: &str) -> String {
    format!(
        "export AV_DOCKER_LABELS='--label av.task={} --label av.agent={agent}'\n",
        task_id_of(ws)
    )
}

fn hook_input(ws: &Path, event: &str, session: &str) -> String {
    serde_json::json!({"hook_event_name": event, "session_id": session, "cwd": ws,
                       "source": "startup", "stop_hook_active": false})
    .to_string()
}

#[test]
fn hook_session_start_gives_the_task_a_scratch_dir_and_snapshots_the_footprint() {
    let (d, ws, _scratch) = started();
    let fp = Footprint::new();
    std::fs::create_dir_all(fp.home.path().join("old/target")).unwrap();
    std::fs::write(fp.home.path().join("old/target/a"), vec![0u8; 4096]).unwrap();
    let o = av_stdin_env(
        &d.url,
        &ws,
        &fp.env("on"),
        &["hook", "session-start"],
        &hook_input(&ws, "SessionStart", "cc-fp"),
    );
    let printed = ok(o);
    assert!(printed.contains("## Task"), "{printed}");
    assert!(
        printed
            .trim_end()
            .ends_with("Temporary files: $AV_SCRATCH (removed with the task)."),
        "{printed}"
    );
    let scratch = d.ws.join("demo/.scratch/improve-greeting");
    assert!(scratch.is_dir());
    assert_eq!(
        std::fs::read_to_string(&fp.env_file).unwrap(),
        format!("export AV_SCRATCH='{}'\n", scratch.display()) + &labels_line(&ws, "agent")
    );
    assert!(!printed.contains("AV_DOCKER_LABELS"), "{printed}");
    let snap: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(fp.snapshot("cc-fp")).unwrap()).unwrap();
    assert!(
        snap.get("container_ids").is_none(),
        "no docker, no ids: {snap}"
    );
    assert_eq!(snap["partial"], false, "{snap}");
    assert_eq!(
        snap["dirs"],
        serde_json::json!([{"path": fp.home.path().join("old/target"), "bytes": 4096}]),
        "{snap}"
    );
}

#[cfg(unix)]
#[test]
fn hook_session_start_points_cargo_at_the_current_workspaces_own_target_dir() {
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
            "Second one",
            "--goal",
            "Another",
        ],
    ));
    let ws2 = d.ws.join("demo/second-one");
    std::fs::write(ws2.join("Cargo.toml"), "[workspace]\n").unwrap();
    let fp = Footprint::new();
    let shared = scratch.path().join("shared-target");
    let mut env = fp.env("off");
    env.push(("CARGO_TARGET_DIR", shared.to_str().unwrap()));
    ok(av_stdin_env(
        &d.url,
        &ws,
        &env,
        &["hook", "session-start"],
        &hook_input(&ws, "SessionStart", "cc-target"),
    ));
    let file = std::fs::read_to_string(&fp.env_file).unwrap();
    let av_scratch = d.ws.join("demo/.scratch/improve-greeting");
    assert!(
        file.starts_with(&format!("export AV_SCRATCH='{}'\n", av_scratch.display())),
        "{file}"
    );
    let seen = |cwd: &Path| -> String {
        let o = Command::new("bash")
            .arg("-c")
            .arg(r#". "$ENV_FILE"; printf '%s|%s' "$CARGO_TARGET_DIR" "${CARGO_BUILD_TARGET_DIR-unset}""#)
            .current_dir(cwd)
            .env("ENV_FILE", &fp.env_file)
            .env("CARGO_TARGET_DIR", &shared)
            .env("CARGO_BUILD_TARGET_DIR", &shared)
            .output()
            .unwrap();
        assert!(o.status.success(), "{}", err(&o));
        out(&o)
    };
    assert_eq!(
        seen(&ws),
        format!("{}|{}", shared.display(), shared.display()),
        "no cargo workspace: nothing to protect, the caller's value stays"
    );
    assert_eq!(
        seen(&ws2.join("app")),
        format!("{}|unset", ws2.join("target").display())
    );
    let elsewhere = scratch.path();
    assert_eq!(
        seen(elsewhere),
        format!("{}|{}", shared.display(), shared.display()),
        "outside a task the caller's value stays"
    );
}

#[test]
fn hook_session_start_leaves_cargo_alone_when_no_target_dir_is_set() {
    let (d, ws, _scratch) = started();
    std::fs::write(ws.join("Cargo.toml"), "[workspace]\n").unwrap();
    let fp = Footprint::new();
    let mut env = fp.env("off");
    env.push(("CARGO_TARGET_DIR", ""));
    env.push(("CARGO_BUILD_TARGET_DIR", ""));
    ok(av_stdin_env(
        &d.url,
        &ws,
        &env,
        &["hook", "session-start"],
        &hook_input(&ws, "SessionStart", "cc-plain"),
    ));
    let av_scratch = d.ws.join("demo/.scratch/improve-greeting");
    assert_eq!(
        std::fs::read_to_string(&fp.env_file).unwrap(),
        format!("export AV_SCRATCH='{}'\n", av_scratch.display()) + &labels_line(&ws, "agent")
    );
}

#[cfg(unix)]
#[test]
fn hook_session_start_exports_docker_labels_and_says_so_when_docker_is_on_path() {
    let (d, ws, _scratch) = started();
    let fp = Footprint::new();
    fp.fake_docker();
    let mut env = fp.env("on");
    env.push(("AV_AGENT", "grok"));
    let printed = ok(av_stdin_env(
        &d.url,
        &ws,
        &env,
        &["hook", "session-start"],
        &hook_input(&ws, "SessionStart", "cc-labels"),
    ));
    let file = std::fs::read_to_string(&fp.env_file).unwrap();
    assert!(file.ends_with(&labels_line(&ws, "grok")), "{file}");
    let line = printed
        .lines()
        .find(|l| l.contains("$AV_DOCKER_LABELS"))
        .unwrap_or_else(|| panic!("{printed}"));
    assert!(
        line.starts_with("Containers: add $AV_DOCKER_LABELS to every docker run/build"),
        "{line}"
    );
    assert!(line.contains("av gc --containers"), "{line}");
    assert_eq!(fp.docker_log(), "ps -aq --no-trunc\n");
}

#[test]
fn hook_stop_blocks_once_when_a_session_left_a_big_build_outside_the_workspace() {
    let (d, ws, _scratch) = started();
    let fp = Footprint::new();
    ok(av_stdin_env(
        &d.url,
        &ws,
        &fp.env("on"),
        &["hook", "session-start"],
        &hook_input(&ws, "SessionStart", "cc-big"),
    ));
    let junk = fp.home.path().join("junk/target");
    std::fs::create_dir_all(&junk).unwrap();
    std::fs::write(junk.join("big"), vec![0u8; 2 * 1024 * 1024]).unwrap();
    let o = ok(av_stdin_env(
        &d.url,
        &ws,
        &fp.env("on"),
        &["hook", "stop"],
        &hook_input(&ws, "Stop", "cc-big"),
    ));
    let v: serde_json::Value = serde_json::from_str(&o).unwrap_or_else(|_| panic!("{o}"));
    assert_eq!(v["decision"], "block", "{o}");
    let reason = v["reason"].as_str().unwrap();
    assert!(
        reason.starts_with("av: this session added 2 MB outside any task workspace: "),
        "{reason}"
    );
    assert!(
        reason.contains(&format!("{} (2 MB)", junk.display())),
        "{reason}"
    );
    assert!(
        reason.contains(&d.ws.join("demo/.cache").display().to_string())
            && reason.contains("$AV_SCRATCH"),
        "{reason}"
    );
    let o = ok(av_stdin_env(
        &d.url,
        &ws,
        &fp.env("on"),
        &["hook", "stop"],
        &hook_input(&ws, "Stop", "cc-big"),
    ));
    assert_eq!(o, "");
    let disk: serde_json::Value = ureq::get(&format!("{}/v1/disk", d.url))
        .call()
        .unwrap()
        .into_json()
        .unwrap();
    let outside = &disk["outside"][0];
    assert_eq!(outside["session"], "cc-big", "{disk}");
    assert_eq!(
        outside["added"][0]["path"],
        junk.to_str().unwrap(),
        "{disk}"
    );
    assert!(
        junk.exists(),
        "the daemon never deletes outside its workspaces"
    );
}

#[cfg(unix)]
fn fake_task_containers(fp: &Footprint, ws: &Path) {
    let task = task_id_of(ws);
    fp.docker_answers(
        "ps",
        &format!(
            "abc123\tav-build\trunning\t12MB (virtual 1.2GB)\t{task}\n\
             def456\tav-db\texited\t0B (virtual 400MB)\t{task}\n"
        ),
    );
    fp.docker_answers("images", "sha256:aaa\n");
    fp.docker_answers(
        "inspect",
        &format!("sha256:aaa\t{task}\t1234567\tdemo/build:latest\n"),
    );
    fp.docker_answers("volumes", &format!("av-pgdata\t{task}\n"));
}

#[cfg(unix)]
#[test]
fn hook_stop_blocks_once_while_the_tasks_containers_run_and_reports_them() {
    let (d, ws, _scratch) = started();
    let fp = Footprint::new();
    fp.fake_docker();
    ok(av_stdin_env(
        &d.url,
        &ws,
        &fp.env("on"),
        &["hook", "session-start"],
        &hook_input(&ws, "SessionStart", "cc-ctr"),
    ));
    fake_task_containers(&fp, &ws);
    let o = ok(av_stdin_env(
        &d.url,
        &ws,
        &fp.env("on"),
        &["hook", "stop"],
        &hook_input(&ws, "Stop", "cc-ctr"),
    ));
    let v: serde_json::Value = serde_json::from_str(&o).unwrap_or_else(|_| panic!("{o}"));
    assert_eq!(v["decision"], "block", "{o}");
    let reason = v["reason"].as_str().unwrap();
    assert!(
        reason.starts_with("av: 1 container of this task is still running: av-build"),
        "{reason}"
    );
    assert!(
        !reason.contains("av-db"),
        "stopped ones are not the block: {reason}"
    );
    assert!(reason.contains("docker stop"), "{reason}");
    assert!(reason.contains("av gc --containers"), "{reason}");
    let task = task_id_of(&ws);
    let log = fp.docker_log();
    assert!(
        log.contains(&format!("ps -a --size --filter label=av.task={task}")),
        "{log}"
    );
    for verb in ["kill", "stop", " rm", "rmi", "prune"] {
        assert!(
            !log.contains(verb),
            "the hook never removes anything: {log}"
        );
    }
    let o = ok(av_stdin_env(
        &d.url,
        &ws,
        &fp.env("on"),
        &["hook", "stop"],
        &hook_input(&ws, "Stop", "cc-ctr"),
    ));
    assert_eq!(o, "");
    let disk: serde_json::Value = ureq::get(&format!("{}/v1/disk", d.url))
        .call()
        .unwrap()
        .into_json()
        .unwrap();
    let outside = &disk["outside"][0];
    assert_eq!(outside["session"], "cc-ctr", "{disk}");
    assert_eq!(outside["added"], serde_json::json!([]), "{disk}");
    let c = &outside["containers"];
    assert_eq!(
        c["containers"],
        serde_json::json!([
            {"id": "abc123", "name": "av-build", "state": "running", "size": "12MB (virtual 1.2GB)"},
            {"id": "def456", "name": "av-db", "state": "exited", "size": "0B (virtual 400MB)"},
        ]),
        "{disk}"
    );
    assert_eq!(
        c["images"],
        serde_json::json!([{"id": "sha256:aaa", "bytes": 1234567, "tags": ["demo/build:latest"]}]),
        "{disk}"
    );
    assert_eq!(
        c["volumes"],
        serde_json::json!([{"name": "av-pgdata"}]),
        "{disk}"
    );
    let text = ok(av(&d, &ws, &["disk"]));
    assert!(
        text.contains("containers  av-build (running, 12MB (virtual 1.2GB)), av-db (exited, 0B (virtual 400MB))"),
        "{text}"
    );
    assert!(
        text.contains("images      demo/build:latest (1 MB)"),
        "{text}"
    );
    assert!(text.contains("volumes     av-pgdata"), "{text}");
}

#[cfg(unix)]
#[test]
fn hook_stop_allows_running_containers_up_to_av_footprint_containers() {
    let (d, ws, _scratch) = started();
    let fp = Footprint::new();
    fp.fake_docker();
    let mut env = fp.env("on");
    env.push(("AV_FOOTPRINT_CONTAINERS", "1"));
    ok(av_stdin_env(
        &d.url,
        &ws,
        &env,
        &["hook", "session-start"],
        &hook_input(&ws, "SessionStart", "cc-one"),
    ));
    fake_task_containers(&fp, &ws);
    let o = av_stdin_env(
        &d.url,
        &ws,
        &env,
        &["hook", "stop"],
        &hook_input(&ws, "Stop", "cc-one"),
    );
    assert!(o.status.success(), "{}", err(&o));
    assert_eq!((out(&o).as_str(), err(&o).as_str()), ("", ""));
    let disk: serde_json::Value = ureq::get(&format!("{}/v1/disk", d.url))
        .call()
        .unwrap()
        .into_json()
        .unwrap();
    assert_eq!(
        disk["outside"][0]["containers"]["containers"][0]["name"], "av-build",
        "{disk}"
    );
}

#[cfg(unix)]
#[test]
fn hook_stop_reports_containers_started_without_labels_and_blocks_once() {
    let (d, ws, _scratch) = started();
    let fp = Footprint::new();
    fp.fake_docker();
    fp.docker_answers("ids", "old0\n");
    ok(av_stdin_env(
        &d.url,
        &ws,
        &fp.env("on"),
        &["hook", "session-start"],
        &hook_input(&ws, "SessionStart", "cc-stray"),
    ));
    let snap: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(fp.snapshot("cc-stray")).unwrap()).unwrap();
    assert_eq!(snap["container_ids"], serde_json::json!(["old0"]), "{snap}");
    assert_eq!(fp.docker_log(), "ps -aq --no-trunc\n");

    let task = task_id_of(&ws);
    fp.docker_answers(
        "ps",
        &format!("lab1\tav-build\trunning\t12MB (virtual 1.2GB)\t{task}\n"),
    );
    fp.docker_answers(
        "all",
        &format!(
            "old0\tbefore-db\trunning\t\n\
             new1\tstray-redis\trunning\t\n\
             new2\tstray-tmp\texited\t\n\
             lab1\tav-build\trunning\t{task}\n\
             oth1\tother-build\trunning\t00000000-0000-0000-0000-000000000000\n"
        ),
    );
    let o = ok(av_stdin_env(
        &d.url,
        &ws,
        &fp.env("on"),
        &["hook", "stop"],
        &hook_input(&ws, "Stop", "cc-stray"),
    ));
    let v: serde_json::Value = serde_json::from_str(&o).unwrap_or_else(|_| panic!("{o}"));
    assert_eq!(v["decision"], "block", "{o}");
    let reason = v["reason"].as_str().unwrap();
    assert!(
        reason.starts_with(
            "av: 2 containers of this task are still running: av-build, stray-redis (unlabelled)."
        ),
        "{reason}"
    );
    assert!(
        reason.contains("1 of them carries no av.task label: add $AV_DOCKER_LABELS"),
        "{reason}"
    );
    assert!(reason.contains("next time"), "{reason}");
    for never in ["before-db", "other-build", "stray-tmp"] {
        assert!(!reason.contains(never), "{never}: {reason}");
    }
    let log = fp.docker_log();
    assert!(
        log.contains("ps -a --no-trunc --format {{.ID}}"),
        "the stop lists every container: {log}"
    );
    for verb in [" rm", "kill", "stop", "rmi", "prune"] {
        assert!(
            !log.contains(verb),
            "the hook never removes anything: {log}"
        );
    }
    let o = ok(av_stdin_env(
        &d.url,
        &ws,
        &fp.env("on"),
        &["hook", "stop"],
        &hook_input(&ws, "Stop", "cc-stray"),
    ));
    assert_eq!(o, "");
    let disk: serde_json::Value = ureq::get(&format!("{}/v1/disk", d.url))
        .call()
        .unwrap()
        .into_json()
        .unwrap();
    let c = &disk["outside"][0]["containers"];
    assert_eq!(
        c["unlabelled"],
        serde_json::json!([
            {"id": "new1", "name": "stray-redis", "state": "running"},
            {"id": "new2", "name": "stray-tmp", "state": "exited"},
        ]),
        "{disk}"
    );
    let labelled: Vec<&str> = c["containers"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["name"].as_str().unwrap())
        .collect();
    assert_eq!(labelled, ["av-build"], "{disk}");
    let text = ok(av(&d, &ws, &["disk"]));
    assert!(
        text.contains("unlabelled  stray-redis (running), stray-tmp (exited)"),
        "{text}"
    );
}

#[cfg(unix)]
#[test]
fn hook_stop_counts_running_unlabelled_containers_with_the_tasks_own() {
    let (d, ws, _scratch) = started();
    let fp = Footprint::new();
    fp.fake_docker();
    let mut env = fp.env("on");
    env.push(("AV_FOOTPRINT_CONTAINERS", "1"));
    ok(av_stdin_env(
        &d.url,
        &ws,
        &env,
        &["hook", "session-start"],
        &hook_input(&ws, "SessionStart", "cc-both"),
    ));
    let task = task_id_of(&ws);
    fp.docker_answers("ps", &format!("lab1\tav-build\trunning\t\t{task}\n"));
    fp.docker_answers(
        "all",
        &format!("lab1\tav-build\trunning\t{task}\nnew1\tstray-redis\trunning\t\n"),
    );
    let o = ok(av_stdin_env(
        &d.url,
        &ws,
        &env,
        &["hook", "stop"],
        &hook_input(&ws, "Stop", "cc-both"),
    ));
    let v: serde_json::Value = serde_json::from_str(&o).unwrap_or_else(|_| panic!("{o}"));
    assert_eq!(v["decision"], "block", "{o}");
    assert!(
        v["reason"]
            .as_str()
            .unwrap()
            .starts_with("av: 2 containers of this task are still running"),
        "{o}"
    );
}

#[cfg(unix)]
#[test]
fn hook_stop_reports_no_unlabelled_containers_without_the_session_start_ids() {
    let (d, ws, _scratch) = started();
    let fp = Footprint::new();
    ok(av_stdin_env(
        &d.url,
        &ws,
        &fp.env("on"),
        &["hook", "session-start"],
        &hook_input(&ws, "SessionStart", "cc-late"),
    ));
    fp.fake_docker();
    fp.docker_answers("all", "new1\tstray-redis\trunning\t\n");
    let o = av_stdin_env(
        &d.url,
        &ws,
        &fp.env("on"),
        &["hook", "stop"],
        &hook_input(&ws, "Stop", "cc-late"),
    );
    assert!(o.status.success(), "{}", err(&o));
    assert_eq!(out(&o), "");
    assert!(
        !fp.docker_log().contains("--no-trunc --format"),
        "{}",
        fp.docker_log()
    );
}

#[cfg(unix)]
#[test]
fn gc_containers_removes_what_finished_tasks_left_and_lists_what_runs() {
    let (d, ws_open, scratch) = started();
    let open = task_id_of(&ws_open);
    let merged = merge_change(&d, scratch.path(), "Add thing", "README.md", "# thing\n");
    ok(av(
        &d,
        scratch.path(),
        &[
            "task", "start", "--repo", "demo", "--title", "Give up", "--goal", "Nothing",
        ],
    ));
    let ws_gone = d.ws.join("demo/give-up");
    let abandoned = task_id_of(&ws_gone);
    assert_eq!(ok(av(&d, &ws_gone, &["abandon"])).trim(), "abandoned");
    let unknown = "00000000-0000-0000-0000-000000000000";
    let fp = Footprint::new();
    fp.fake_docker();
    fp.docker_answers(
        "ps",
        &format!(
            "c1\tb-build\texited\t0B (virtual 1GB)\t{merged}\n\
             c2\tb-db\trunning\t\t{merged}\n\
             c3\ta-build\texited\t\t{open}\n\
             c4\tc-db\tcreated\t\t{abandoned}\n\
             c5\tghost\texited\t\t{unknown}\n\
             c6\tstray\texited\t\t\n"
        ),
    );
    fp.docker_answers("images", "sha256:b\nsha256:a\n");
    fp.docker_answers(
        "inspect",
        &format!("sha256:b\t{merged}\t1000\t\nsha256:a\t{open}\t2000\t\n"),
    );
    fp.docker_answers("volumes", &format!("b-vol\t{merged}\na-vol\t{open}\n"));

    let o = ok(av_env(
        &d.url,
        scratch.path(),
        &fp.env("off"),
        &["gc", "--dry-run", "--containers", "--json"],
    ));
    let v: serde_json::Value = serde_json::from_str(&o).unwrap_or_else(|_| panic!("{o}"));
    let c = &v["containers"];
    assert_eq!(c["dry_run"], true, "{o}");
    let removed: Vec<(&str, &str)> = c["removed"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| (r["kind"].as_str().unwrap(), r["name"].as_str().unwrap()))
        .collect();
    assert_eq!(
        removed,
        [
            ("container", "b-build"),
            ("image", "sha256:b"),
            ("volume", "b-vol"),
            ("container", "c-db"),
        ],
        "{o}"
    );
    assert_eq!(c["removed"][0]["slug"], "add-thing", "{o}");
    assert_eq!(c["removed"][3]["slug"], "give-up", "{o}");
    assert_eq!(c["running"][0]["name"], "b-db", "{o}");
    assert_eq!(
        c["left"],
        serde_json::json!({"open": 3, "unknown": 1}),
        "{o}"
    );
    let log = fp.docker_log();
    assert!(
        log.contains("images -q --no-trunc --filter label=av.task --filter dangling=true"),
        "{log}"
    );
    for verb in [" rm ", "rmi", "kill", "stop", "prune"] {
        assert!(!log.contains(verb), "a dry run removes nothing: {log}");
    }

    let text = ok(av_env(
        &d.url,
        scratch.path(),
        &fp.env("off"),
        &["gc", "--containers"],
    ));
    for line in [
        "removed container demo/add-thing  b-build (exited, 0B (virtual 1GB))",
        "removed image demo/add-thing  sha256:b (1000 B)",
        "removed volume demo/add-thing  b-vol",
        "removed container demo/give-up  c-db (created)",
        "running demo/add-thing  b-db (av never stops a container)",
        "left alone: 3 of open tasks, 1 of tasks the daemon does not know",
    ] {
        assert!(text.contains(line), "{line:?} not in\n{text}");
    }
    assert!(
        !text.contains("a-build") && !text.contains("ghost"),
        "{text}"
    );
    let log = fp.docker_log();
    for call in ["rm c1\n", "rmi sha256:b\n", "volume rm b-vol\n", "rm c4\n"] {
        assert!(log.contains(call), "{call:?} not in\n{log}");
    }
    for never in [
        "rm c2",
        "rm c3",
        "rm c5",
        "rm c6",
        "rmi sha256:a",
        "rm a-vol",
        "kill",
        "stop",
    ] {
        assert!(!log.contains(never), "{never:?} in\n{log}");
    }
    let fp2 = Footprint::new();
    let o = av_env(
        &d.url,
        scratch.path(),
        &fp2.env("off"),
        &["gc", "--containers"],
    );
    assert!(o.status.success(), "{}", err(&o));
    assert!(err(&o).contains("docker is not on PATH"), "{}", err(&o));
    assert!(out(&o).contains("freed"), "{}", out(&o));
}

#[test]
fn hook_footprint_off_writes_and_says_nothing() {
    let (d, ws, _scratch) = started();
    let fp = Footprint::new();
    ok(av_stdin_env(
        &d.url,
        &ws,
        &fp.env("off"),
        &["hook", "session-start"],
        &hook_input(&ws, "SessionStart", "cc-off"),
    ));
    assert!(!fp.av_home.path().join("footprint").exists());
    let junk = fp.home.path().join("junk/target");
    std::fs::create_dir_all(&junk).unwrap();
    std::fs::write(junk.join("big"), vec![0u8; 2 * 1024 * 1024]).unwrap();
    let o = av_stdin_env(
        &d.url,
        &ws,
        &fp.env("off"),
        &["hook", "stop"],
        &hook_input(&ws, "Stop", "cc-off"),
    );
    assert!(o.status.success());
    assert_eq!((out(&o).as_str(), err(&o).as_str()), ("", ""));
}

#[test]
fn hook_stop_is_silent_outside_a_task_and_when_the_daemon_is_down() {
    let d = start_daemon();
    let scratch = TempDir::new().unwrap();
    let o = av_stdin(
        &d,
        scratch.path(),
        &["hook", "stop"],
        &stop_input(scratch.path(), "s", false),
    );
    assert!(o.status.success());
    assert_eq!((out(&o).as_str(), err(&o).as_str()), ("", ""));
    let (_d, ws, _scratch) = started();
    let o = av_with("http://127.0.0.1:1", &ws, &["hook", "stop"]);
    assert!(o.status.success(), "{}", err(&o));
    assert_eq!(out(&o), "");
}

/// Puts one item with no tag requirements in the backlog, so any agent fits it.
fn post_item(d: &Daemon, scratch: &Path, title: &str) {
    ok(av(
        d,
        scratch,
        &[
            "post", "--repo", "demo", "--title", title, "--goal", "do it",
        ],
    ));
}

fn stop_with(d: &Daemon, ws: &Path, env: &[(&str, &str)], session: &str, active: bool) -> Output {
    av_stdin_env(
        &d.url,
        ws,
        env,
        &["hook", "stop"],
        &stop_input(ws, session, active),
    )
}

#[test]
fn hook_stop_blocks_when_work_is_queued_and_the_reason_names_it() {
    let (d, ws, scratch) = started();
    post_item(&d, scratch.path(), "Fix the README typos");
    let o = stop_with(&d, &ws, &[("AV_HOOK_WAIT_SECS", "1")], "cc-1", false);
    assert!(o.status.success(), "{}", err(&o));
    let v: serde_json::Value = serde_json::from_str(&out(&o)).unwrap();
    assert_eq!(v["decision"], "block", "{}", out(&o));
    let reason = v["reason"].as_str().unwrap();
    assert!(reason.starts_with("av: "), "{reason}");
    assert!(reason.contains("Fix the README typos"), "{reason}");
    assert!(
        reason.ends_with("call start_task with no title"),
        "{reason}"
    );
    let backlog = ok(av(&d, &ws, &["backlog"]));
    assert!(backlog.contains("Fix the README typos"), "{backlog}");
    assert!(!backlog.contains("taken"), "{backlog}");
}

#[test]
fn a_proposed_item_is_listed_with_its_why_and_accepted() {
    let (d, ws, scratch) = started();
    let posted = ok(av(
        &d,
        scratch.path(),
        &[
            "post",
            "--repo",
            "demo",
            "--title",
            "Cache packs",
            "--goal",
            "g",
            "--propose",
            "--why",
            "unsure it pays off",
        ],
    ));
    assert!(posted.starts_with("proposed"), "{posted}");
    let id = posted.split_whitespace().nth(1).unwrap().to_owned();
    let backlog = ok(av(&d, &ws, &["backlog"]));
    assert!(backlog.contains("proposed"), "{backlog}");
    assert!(
        backlog.contains("proposed by agent: unsure it pays off"),
        "{backlog}"
    );
    let o = stop_with(&d, &ws, &[("AV_HOOK_WAIT_SECS", "1")], "cc-1", false);
    assert!(o.status.success(), "{}", err(&o));
    assert_eq!(out(&o), "");

    let accepted = ok(av(&d, &ws, &["backlog", "accept", &id]));
    assert!(accepted.starts_with("accepted"), "{accepted}");
    let backlog = ok(av(&d, &ws, &["backlog", "--state", "queued"]));
    assert!(backlog.contains("Cache packs"), "{backlog}");

    let set = ok(av(
        &d,
        scratch.path(),
        &["repo", "settings", "demo", "--agent-posts", "propose"],
    ));
    assert!(set.contains("agent posts: propose"), "{set}");
}

#[test]
fn hook_stop_waits_out_an_empty_backlog_then_allows_the_stop() {
    let (d, ws, _scratch) = started();
    let t = std::time::Instant::now();
    let o = stop_with(&d, &ws, &[("AV_HOOK_WAIT_SECS", "1")], "cc-1", false);
    assert!(o.status.success(), "{}", err(&o));
    assert_eq!((out(&o).as_str(), err(&o).as_str()), ("", ""));
    assert!(
        t.elapsed() >= std::time::Duration::from_millis(900),
        "returned after {:?}, before the wait",
        t.elapsed()
    );
    // a Stop hook that is already continuing is let go too when nothing fits
    let o = stop_with(&d, &ws, &[("AV_HOOK_WAIT_SECS", "1")], "cc-1", true);
    assert!(o.status.success(), "{}", err(&o));
    assert_eq!(out(&o), "");
}

#[test]
fn hook_stop_picks_up_work_that_arrives_during_the_wait() {
    let (d, ws, scratch) = started();
    let url = d.url.clone();
    let scratch_path = scratch.path().to_path_buf();
    let poster = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(500));
        ok(av_with(
            &url,
            &scratch_path,
            &[
                "post", "--repo", "demo", "--title", "Late job", "--goal", "g",
            ],
        ));
    });
    let o = stop_with(&d, &ws, &[("AV_HOOK_WAIT_SECS", "20")], "cc-1", false);
    poster.join().unwrap();
    let v: serde_json::Value = serde_json::from_str(&out(&o)).unwrap();
    assert_eq!(v["decision"], "block", "{}{}", out(&o), err(&o));
    assert!(v["reason"].as_str().unwrap().contains("Late job"), "{v}");
}

/// The hook's write of its count after the wait must not put the previous owner's generation back, or the new owner is refused as "claim lost".
#[test]
fn a_stop_hook_that_waited_through_a_claim_leaves_the_new_owners_task_file() {
    let (d, ws, scratch) = started();
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
            "app/greeter.py",
        ],
    ));
    ok(av(
        &d,
        &ws,
        &["handoff", "--done", "half", "--remaining", "rest"],
    ));
    let task = task_id_of(&ws);
    let url = d.url.clone();
    let scratch_path = scratch.path().to_path_buf();
    let claimant = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(500));
        let claimed = Command::new(env!("CARGO_BIN_EXE_av"))
            .current_dir(&scratch_path)
            .env("AV_URL", &url)
            .env("AV_AGENT", "codex")
            .args(["claim", &task])
            .output()
            .unwrap();
        let claimed = ok(claimed);
        assert!(claimed.contains("generation: 2"), "{claimed}");
        // the hook writes its count only when it finds work
        ok(av_with(
            &url,
            &scratch_path,
            &[
                "post", "--repo", "demo", "--title", "Late job", "--goal", "g",
            ],
        ));
    });
    let o = stop_with(&d, &ws, &[("AV_HOOK_WAIT_SECS", "20")], "cc-1", false);
    claimant.join().unwrap();
    assert!(o.status.success(), "{}", err(&o));
    let file: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(ws.join(".av-task")).unwrap()).unwrap();
    assert_eq!(file["generation"], 2, "the claim's file was undone: {file}");
    assert_eq!(file["agent"], "codex", "{file}");
    // a continue it could not count would give the session a fresh budget under the new file
    assert!(
        String::from_utf8_lossy(&o.stdout).trim().is_empty(),
        "a hook that lost its claim kept the session busy: {}",
        String::from_utf8_lossy(&o.stdout)
    );
    ok(av_as(&d, &ws, "codex", &["checkpoint"]));
}

/// The cap is checked again on the file after the wait: another Stop of the same session may have counted the last continue.
#[test]
fn a_stop_hook_checks_the_cap_again_after_its_wait() {
    let (d, ws, scratch) = started();
    let url = d.url.clone();
    let scratch_path = scratch.path().to_path_buf();
    let file_path = ws.join(".av-task");
    let other_stop = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(500));
        let mut file: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&file_path).unwrap()).unwrap();
        file["continue_session"] = "cc-1".into();
        file["continues"] = 2.into();
        std::fs::write(&file_path, file.to_string()).unwrap();
        ok(av_with(
            &url,
            &scratch_path,
            &[
                "post", "--repo", "demo", "--title", "Late job", "--goal", "g",
            ],
        ));
    });
    let o = stop_with(
        &d,
        &ws,
        &[("AV_HOOK_WAIT_SECS", "20"), ("AV_HOOK_MAX_CONTINUES", "2")],
        "cc-1",
        false,
    );
    other_stop.join().unwrap();
    assert!(o.status.success(), "{}", err(&o));
    assert!(
        String::from_utf8_lossy(&o.stdout).trim().is_empty(),
        "the hook continued past the cap: {}",
        String::from_utf8_lossy(&o.stdout)
    );
    let file: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(ws.join(".av-task")).unwrap()).unwrap();
    assert_eq!(file["continues"], 2, "{file}");
}

#[test]
fn hook_stop_wait_of_zero_turns_the_feature_off() {
    let (d, ws, scratch) = started();
    post_item(&d, scratch.path(), "Fix the README typos");
    let o = stop_with(&d, &ws, &[("AV_HOOK_WAIT_SECS", "0")], "cc-1", false);
    assert!(o.status.success(), "{}", err(&o));
    assert_eq!(out(&o), "");
}

#[test]
fn hook_stop_continues_at_most_max_continues_times_per_session() {
    let (d, ws, scratch) = started();
    post_item(&d, scratch.path(), "Fix the README typos");
    let env = [("AV_HOOK_WAIT_SECS", "1"), ("AV_HOOK_MAX_CONTINUES", "2")];
    let blocked = |session: &str, active: bool| {
        let o = stop_with(&d, &ws, &env, session, active);
        assert!(o.status.success(), "{}", err(&o));
        out(&o).contains("\"block\"")
    };
    // Claude Code sets stop_hook_active once a Stop hook has continued the session
    assert!(blocked("cc-cap", false), "first continue");
    assert!(blocked("cc-cap", true), "second continue, still work");
    assert!(
        !blocked("cc-cap", true),
        "the cap of 2 is reached: let it stop"
    );
    assert!(!blocked("cc-cap", true), "and it stays reached");
    assert!(blocked("cc-other", false), "a new session starts at zero");
    let tf = std::fs::read_to_string(ws.join(".av-task")).unwrap();
    assert!(tf.contains("continues"), "{tf}");
}

#[test]
fn hook_stop_with_the_daemon_down_allows_the_stop_at_once() {
    let (_d, ws, _scratch) = started();
    let t = std::time::Instant::now();
    let o = av_stdin_env(
        "http://127.0.0.1:1",
        &ws,
        &[("AV_HOOK_WAIT_SECS", "3600")],
        &["hook", "stop"],
        &stop_input(&ws, "cc-1", false),
    );
    assert!(o.status.success(), "{}", err(&o));
    assert_eq!(out(&o), "");
    assert!(
        t.elapsed() < HANG,
        "waited {:?} for a daemon that is down",
        t.elapsed()
    );
}

#[test]
fn hook_stop_outside_a_task_stays_silent_even_with_work_queued() {
    let (d, _ws, scratch) = started();
    post_item(&d, scratch.path(), "Fix the README typos");
    let bare = TempDir::new().unwrap();
    let o = av_stdin_env(
        &d.url,
        bare.path(),
        &[("AV_HOOK_WAIT_SECS", "1")],
        &["hook", "stop"],
        &stop_input(bare.path(), "s", false),
    );
    assert!(o.status.success(), "{}", err(&o));
    assert_eq!((out(&o).as_str(), err(&o).as_str()), ("", ""));
}

#[test]
fn record_help_states_the_summary_limit() {
    let dir = tempfile::tempdir().unwrap();
    let o = av_with("http://127.0.0.1:1", dir.path(), &["record", "--help"]);
    assert!(o.status.success(), "{}", err(&o));
    let help = out(&o);
    let limit = format!("at most {} characters", margincode_core::MAX_SUMMARY_CHARS);
    let line_of = |flag: &str| {
        let at = help
            .find(flag)
            .unwrap_or_else(|| panic!("no {flag}: {help}"));
        let rest = &help[at..];
        let end = rest[2..].find("\n      --").map_or(rest.len(), |e| e + 2);
        rest[..end].to_owned()
    };
    assert!(line_of("--summary").contains(&limit), "{help}");
    assert!(line_of("--json").contains(&limit), "{help}");
}

#[test]
fn hook_stop_asks_for_records_before_it_keeps_the_agent_busy() {
    let (d, ws, scratch) = started();
    post_item(&d, scratch.path(), "Fix the README typos");
    std::fs::write(
        ws.join("app/greeter.py"),
        "class Greeter:\n    def hello(self, name):\n        return name.upper()\n",
    )
    .unwrap();
    let env = [("AV_HOOK_WAIT_SECS", "1")];
    let first = stop_with(&d, &ws, &env, "cc-1", false);
    let v: serde_json::Value = serde_json::from_str(&out(&first)).unwrap();
    assert!(
        v["reason"].as_str().unwrap().contains("no record explains"),
        "{v}"
    );
    // the limit is said before the agent writes, not by a refusal after
    assert!(
        v["reason"].as_str().unwrap().contains(&format!(
            "summary ≤ {} chars",
            margincode_core::MAX_SUMMARY_CHARS
        )),
        "{v}"
    );
    // one record call, then a one-line end, not the answer written a second time
    let reason = v["reason"].as_str().unwrap();
    assert!(
        reason.contains("in one call")
            && reason.contains("Then end your turn in a line; do not recap.")
            && reason.contains("say so and stop"),
        "{reason}"
    );
    // 408 chars for these names before it asked for a one-line end
    assert!(
        reason.chars().count() <= 408,
        "no longer than before ({} chars): {reason}",
        reason.chars().count()
    );
    let second = stop_with(&d, &ws, &env, "cc-1", true);
    let v: serde_json::Value = serde_json::from_str(&out(&second)).unwrap();
    assert!(
        v["reason"]
            .as_str()
            .unwrap()
            .contains("Fix the README typos"),
        "{v}"
    );
}

fn stop_claims(d: &Daemon, ws: &Path, env: &[(&str, &str)], args: &[&str]) -> bool {
    static CALLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let n = CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let o = av_stdin_env(
        &d.url,
        ws,
        env,
        args,
        &stop_input(ws, &format!("who-{n}"), false),
    );
    assert!(o.status.success(), "{}", err(&o));
    assert!(
        out(&o).is_empty() || out(&o).contains("\"block\""),
        "unexpected stdout {}",
        out(&o)
    );
    out(&o).contains("`claude` now owns this task")
}

#[test]
fn hook_stop_acts_as_grok_when_grok_runs_it_and_as_the_claude_default_otherwise() {
    let (d, ws, _scratch) = started_as("claude");
    assert!(
        stop_claims(&d, &ws, &[("GROK_HOOK_EVENT", "Stop")], &["hook", "stop"]),
        "GROK_HOOK_EVENT should act as grok, not the task owner"
    );
    assert!(
        stop_claims(&d, &ws, &[("GROK_SESSION_ID", "sess")], &["hook", "stop"]),
        "GROK_SESSION_ID alone should act as grok"
    );
    assert!(
        !stop_claims(&d, &ws, &[], &["hook", "stop"]),
        "without a Grok variable the hook is unnamed (the owner's own, as far as av can tell)"
    );
    assert!(
        !stop_claims(
            &d,
            &ws,
            &[("AV_AGENT", "claude"), ("GROK_HOOK_EVENT", "Stop")],
            &["hook", "stop"]
        ),
        "AV_AGENT wins over GROK_HOOK_EVENT"
    );
    assert!(
        stop_claims(
            &d,
            &ws,
            &[("GROK_SESSION_ID", "sess")],
            &["--agent", "grok", "hook", "stop"]
        ),
        "--agent wins, and grok is not the task owner"
    );
    assert!(
        !stop_claims(
            &d,
            &ws,
            &[("GROK_HOOK_EVENT", "Stop"), ("GROK_SESSION_ID", "sess")],
            &["--agent", "claude", "hook", "stop"]
        ),
        "--agent claude is the owner even when Grok's variables are set"
    );
}

#[test]
fn hook_stop_offers_grok_the_image_work_and_not_the_claude_default() {
    let (d, ws, scratch) = started();
    ok(av(
        &d,
        scratch.path(),
        &["agent", "set", "grok", "--best", "image", "--cost", "2"],
    ));
    ok(av(
        &d,
        scratch.path(),
        &[
            "post",
            "--repo",
            "demo",
            "--title",
            "Draw the logo",
            "--goal",
            "A flat logo",
            "--needs",
            "image",
            "--reserve",
            "0",
        ],
    ));
    let reason = |env: &[(&str, &str)], args: &[&str], session: &str| {
        let o = av_stdin_env(&d.url, &ws, env, args, &stop_input(&ws, session, false));
        assert!(o.status.success(), "{}", err(&o));
        out(&o)
    };
    let grok_wait = [("AV_HOOK_WAIT_SECS", "1"), ("GROK_HOOK_EVENT", "Stop")];
    let via_event = reason(&grok_wait, &["hook", "stop"], "logo-event");
    assert!(
        via_event.contains("Draw the logo") && via_event.contains("call start_task with no title"),
        "GROK_HOOK_EVENT: {via_event}"
    );
    let via_session = reason(
        &[("AV_HOOK_WAIT_SECS", "1"), ("GROK_SESSION_ID", "grok-sess")],
        &["hook", "stop"],
        "logo-session",
    );
    assert!(
        via_session.contains("Draw the logo"),
        "GROK_SESSION_ID: {via_session}"
    );
    let backlog = ok(av(&d, &ws, &["backlog"]));
    assert!(backlog.contains("Draw the logo"), "{backlog}");
    assert!(!backlog.contains("taken"), "{backlog}");

    let plain = reason(
        &[("AV_HOOK_WAIT_SECS", "1")],
        &["hook", "stop"],
        "logo-default",
    );
    assert!(
        !plain.contains("Draw the logo"),
        "the Claude default has no image profile: {plain}"
    );
    let overridden = reason(
        &[
            ("AV_HOOK_WAIT_SECS", "1"),
            ("AV_AGENT", "agent"),
            ("GROK_HOOK_EVENT", "Stop"),
        ],
        &["hook", "stop"],
        "logo-av-agent",
    );
    assert!(
        !overridden.contains("Draw the logo"),
        "AV_AGENT wins over the Grok environment: {overridden}"
    );
    let flagged = reason(
        &[("AV_HOOK_WAIT_SECS", "1")],
        &["--agent", "grok", "hook", "stop"],
        "logo-flag",
    );
    assert!(
        flagged.contains("Draw the logo"),
        "--agent grok with no Grok variables: {flagged}"
    );
}

fn av_env(url: &str, cwd: &Path, env: &[(&str, &str)], args: &[&str]) -> Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_av"));
    cmd.current_dir(cwd)
        .env("AV_URL", url)
        .env_remove("AV_TASK")
        .env_remove("AV_SESSION")
        .env_remove("AV_AGENT")
        .env_remove("AV_TOKEN")
        .env_remove("AV_DATABASE_URL")
        .env_remove("AV_STORE_ROOT")
        .env_remove("AV_WS_ROOT")
        .env_remove("GROK_HOOK_EVENT")
        .env_remove("GROK_SESSION_ID");
    for (k, v) in env {
        cmd.env(k, v);
    }
    cmd.args(args).output().unwrap()
}

fn write_script(path: &Path, text: &str) {
    std::fs::write(path, text).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755)).unwrap();
    }
}

#[test]
fn init_adds_the_repo_and_wires_claude_code() {
    let d = start_daemon();
    let home = TempDir::new().unwrap();
    let bin = TempDir::new().unwrap();
    let log = home.path().join("claude.log");
    write_script(
        &bin.path().join("claude"),
        &format!(
            "#!/bin/sh\necho \"$@\" >> {log}\ncase \"$2\" in\n  get) [ -f {added} ] ;;\n  add) touch {added} ;;\nesac\n",
            log = log.display(),
            added = home.path().join("added").display()
        ),
    );
    let path = format!(
        "{}:{}",
        bin.path().display(),
        std::env::var("PATH").unwrap_or_default()
    );
    let env = [
        ("HOME", home.path().to_str().unwrap()),
        ("PATH", path.as_str()),
    ];
    let first = ok(av_env(&d.url, &d.src, &env, &["init", "--name", "demo"]));
    assert!(first.contains("repo demo added from"), "{first}");
    assert!(first.contains("MCP server `av`: added"), "{first}");
    assert!(first.contains("av task start --repo demo"), "{first}");
    let calls = std::fs::read_to_string(&log).unwrap();
    assert!(
        calls.contains(&format!(
            "mcp add --transport http --scope user av {}/mcp",
            d.url
        )),
        "{calls}"
    );
    let settings: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(home.path().join(".claude/settings.json")).unwrap(),
    )
    .unwrap();
    for event in ["SessionStart", "PreToolUse", "PostToolUse", "Stop"] {
        assert_eq!(
            settings["hooks"][event].as_array().unwrap().len(),
            1,
            "{event}"
        );
    }

    assert!(
        !calls.contains("X-Av-Agent"),
        "no --agent, no name: {calls}"
    );

    let again = ok(av_env(&d.url, &d.src, &env, &["init", "--name", "demo"]));
    assert!(again.contains("repo demo already added"), "{again}");
    assert!(again.contains("already configured"), "{again}");

    let home2 = TempDir::new().unwrap();
    let log2 = home2.path().join("claude.log");
    write_script(
        &bin.path().join("claude"),
        &format!(
            "#!/bin/sh\necho \"$@\" >> {log}\ncase \"$2\" in\n  get) [ -f {added} ] ;;\n  add) touch {added} ;;\nesac\n",
            log = log2.display(),
            added = home2.path().join("added").display()
        ),
    );
    let env2 = [
        ("HOME", home2.path().to_str().unwrap()),
        ("PATH", path.as_str()),
    ];
    let named = ok(av_env(
        &d.url,
        &d.src,
        &env2,
        &["--agent", "claude", "init", "--name", "demo"],
    ));
    assert!(named.contains("MCP server `av`: added"), "{named}");
    let calls = std::fs::read_to_string(&log2).unwrap();
    assert!(
        calls.contains(&format!(
            "mcp add --transport http --scope user av {}/mcp --header X-Av-Agent: claude",
            d.url
        )),
        "{calls}"
    );
    let rerun = ok(av_env(
        &d.url,
        &d.src,
        &env2,
        &["--agent", "claude", "init", "--name", "demo"],
    ));
    assert!(
        rerun.contains("claude mcp remove --scope user av"),
        "{rerun}"
    );
    let settings: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(home.path().join(".claude/settings.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(settings["hooks"]["Stop"].as_array().unwrap().len(), 1);
    ok(av(
        &d,
        d.src.as_path(),
        &[
            "task", "start", "--repo", "demo", "--title", "T", "--goal", "G",
        ],
    ));
}

#[test]
fn init_wires_codex_grok_and_gemini_through_mcp() {
    let d = start_daemon();
    let home = TempDir::new().unwrap();
    let env = [("HOME", home.path().to_str().unwrap()), ("CODEX_HOME", "")];
    let out = ok(av_env(
        &d.url,
        &d.src,
        &env,
        &[
            "init", "--name", "demo", "--for", "codex", "--for", "grok", "--for", "gemini",
            "--for", "cursor",
        ],
    ));
    for name in ["Codex", "Grok Build", "Gemini CLI", "Cursor Agent"] {
        assert!(out.contains(&format!("{name} MCP server `av`")), "{out}");
    }
    let mcp = format!("{}/mcp", d.url);
    let codex = std::fs::read_to_string(home.path().join(".codex/config.toml")).unwrap();
    assert!(
        codex.contains(&format!("[mcp_servers.av]\nurl = \"{mcp}\"")),
        "{codex}"
    );
    let grok = std::fs::read_to_string(home.path().join(".grok/config.toml")).unwrap();
    assert!(grok.contains(&format!("url = \"{mcp}\"")), "{grok}");
    let gemini: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(home.path().join(".gemini/settings.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(gemini["mcpServers"]["av"]["httpUrl"], mcp.as_str());
    let cursor: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(home.path().join(".cursor/mcp.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(cursor["mcpServers"]["av"]["url"], mcp.as_str());
    assert_eq!(
        cursor["mcpServers"]["av"]["headers"]["X-Av-Agent"],
        "cursor"
    );
    assert!(!home.path().join(".claude").exists());
}

#[test]
fn init_installs_grok_hooks_unless_claude_settings_already_load_them() {
    let d = start_daemon();
    let home = TempDir::new().unwrap();
    let bin = TempDir::new().unwrap();
    let log = home.path().join("claude.log");
    write_script(
        &bin.path().join("claude"),
        &format!(
            "#!/bin/sh\necho \"$@\" >> {log}\ncase \"$2\" in\n  get) [ -f {added} ] ;;\n  add) touch {added} ;;\nesac\n",
            log = log.display(),
            added = home.path().join("added").display()
        ),
    );
    let path = format!(
        "{}:{}",
        bin.path().display(),
        std::env::var("PATH").unwrap_or_default()
    );
    let env = [
        ("HOME", home.path().to_str().unwrap()),
        ("PATH", path.as_str()),
        ("AV_HOOK_WAIT_SECS", "12"),
    ];
    let out = ok(av_env(
        &d.url,
        &d.src,
        &env,
        &["init", "--name", "demo", "--for", "grok", "--no-claude"],
    ));
    assert!(out.contains("Grok Build hooks:"), "{out}");
    let hooks_path = home.path().join(".grok/hooks/av.json");
    let read = |path: &Path| -> serde_json::Value {
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
    };
    let v = read(&hooks_path);
    assert!(
        v["hooks"]["SessionStart"][0]["hooks"][0]["command"]
            .as_str()
            .unwrap()
            .contains("av hook session-start"),
        "{v}"
    );
    let stop = &v["hooks"]["Stop"][0]["hooks"][0];
    assert!(
        stop["command"].as_str().unwrap().contains("av hook stop"),
        "{stop}"
    );
    assert_eq!(stop["timeout"], 72, "wait 12 plus the 60s margin");
    assert!(v["hooks"].get("PreToolUse").is_none(), "{v}");
    assert!(v["hooks"].get("PostToolUse").is_none(), "{v}");
    ok(av_env(
        &d.url,
        &d.src,
        &env,
        &["init", "--name", "demo", "--for", "grok", "--no-claude"],
    ));
    let again = read(&hooks_path);
    assert_eq!(again["hooks"]["Stop"].as_array().unwrap().len(), 1);

    // Claude settings already load the hook: do not write a second copy Grok would also run.
    let home2 = TempDir::new().unwrap();
    let settings = home2.path().join(".claude/settings.json");
    std::fs::create_dir_all(settings.parent().unwrap()).unwrap();
    std::fs::write(
        &settings,
        r#"{"hooks":{"Stop":[{"hooks":[{"type":"command","command":"av hook stop"}]}]}}"#,
    )
    .unwrap();
    let env2 = [
        ("HOME", home2.path().to_str().unwrap()),
        ("AV_HOOK_WAIT_SECS", "12"),
    ];
    let skipped = ok(av_env(
        &d.url,
        &d.src,
        &env2,
        &["init", "--name", "demo", "--for", "grok", "--no-claude"],
    ));
    assert!(skipped.contains("already reachable"), "{skipped}");
    assert!(!home2.path().join(".grok/hooks/av.json").exists());
    assert!(
        std::fs::read_to_string(home2.path().join(".grok/config.toml"))
            .unwrap()
            .contains("mcp_servers")
    );

    let claude = ok(av_env(&d.url, &d.src, &env, &["init", "--name", "demo"]));
    assert!(claude.contains("Claude Code hooks:"), "{claude}");
    assert!(!hooks_path.exists(), "both files would fire the Stop hook");
    let installed = read(&home.path().join(".claude/settings.json"));
    assert!(
        installed["hooks"]["Stop"][0]["hooks"][0]["command"]
            .as_str()
            .unwrap()
            .contains("av hook stop")
    );
}

#[test]
fn init_mcp_json_needs_a_path_and_path_needs_mcp_json() {
    let home = TempDir::new().unwrap();
    let env = [("HOME", home.path().to_str().unwrap())];
    let missing = av_env(
        "http://127.0.0.1:9",
        home.path(),
        &env,
        &["init", "--for", "mcp-json", "--no-claude"],
    );
    assert!(!missing.status.success());
    assert!(
        err(&missing).contains("--for mcp-json needs --path FILE"),
        "{}",
        err(&missing)
    );
    let extra = av_env(
        "http://127.0.0.1:9",
        home.path(),
        &env,
        &[
            "init",
            "--for",
            "grok",
            "--no-claude",
            "--path",
            "client.json",
        ],
    );
    assert!(!extra.status.success());
    assert!(
        err(&extra).contains("--path is only for --for mcp-json"),
        "{}",
        err(&extra)
    );
}

#[test]
fn init_merges_mcp_json_at_the_given_path() {
    let d = start_daemon();
    let home = TempDir::new().unwrap();
    let file = home.path().join("client.json");
    std::fs::write(
        &file,
        r#"{"theme":"dark","mcpServers":{"other":{"command":"x"}}}"#,
    )
    .unwrap();
    let env = [
        ("HOME", home.path().to_str().unwrap()),
        ("AV_TOKEN", "sekrit"),
    ];
    let out = ok(av_env(
        &d.url,
        &d.src,
        &env,
        &[
            "--agent",
            "muse",
            "init",
            "--name",
            "demo",
            "--for",
            "mcp-json",
            "--path",
            file.to_str().unwrap(),
        ],
    ));
    assert!(out.contains("mcp-json MCP server `av`"), "{out}");
    assert!(out.contains("reads the token from AV_TOKEN"), "{out}");
    let v: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&file).unwrap()).unwrap();
    assert_eq!(v["theme"], "dark");
    assert_eq!(v["mcpServers"]["other"]["command"], "x");
    assert_eq!(
        v["mcpServers"]["av"],
        serde_json::json!({
            "type": "http",
            "url": format!("{}/mcp", d.url),
            "headers": {
                "X-Av-Agent": "muse",
                "Authorization": "Bearer ${AV_TOKEN}",
            },
        })
    );
    assert!(home.path().join("client.json.bak").exists());
    assert!(!home.path().join(".claude").exists());
    assert!(!home.path().join(".grok").exists());
}

#[test]
fn init_outside_a_git_repository_says_so() {
    let d = start_daemon();
    let home = TempDir::new().unwrap();
    let o = av_env(
        &d.url,
        home.path(),
        &[
            ("HOME", home.path().to_str().unwrap()),
            (
                "GIT_CEILING_DIRECTORIES",
                home.path().parent().unwrap().to_str().unwrap(),
            ),
        ],
        &["init", "--no-claude"],
    );
    assert!(!o.status.success());
    assert!(err(&o).contains("inside a git repository"), "{}", err(&o));
}

/// Run in git repo A, `--source` imports B: the path the daemon reads, as with a daemon in Docker.
#[test]
fn init_imports_from_a_daemon_side_source() {
    let d = start_daemon();
    let home = TempDir::new().unwrap();
    let a = TempDir::new().unwrap();
    let git = Command::new("git")
        .args(["init", "-q"])
        .current_dir(a.path())
        .status()
        .unwrap();
    assert!(git.success());
    let env = [("HOME", home.path().to_str().unwrap())];
    let src = d.src.to_str().unwrap();
    let added = ok(av_env(
        &d.url,
        a.path(),
        &env,
        &["init", "--no-claude", "--name", "demo", "--source", src],
    ));
    assert!(
        added.contains(&format!("repo demo added from {src}")),
        "{added}"
    );
    let shown = ok(av(
        &d,
        a.path(),
        &["show", "app/greeter.py", "--repo", "demo"],
    ));
    assert!(shown.contains("def hello"), "B's file: {shown}");
}

fn free_port() -> u16 {
    std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

const FAKE_DAEMON: &str = r#"#!/usr/bin/env python3
import http.server, os, signal, sys
host, port = os.environ["AV_LISTEN"].rsplit(":", 1)
with open(os.environ["FAKE_ENV_OUT"], "w") as f:
    for k in ("AV_HOME", "AV_STORE_ROOT", "AV_WS_ROOT", "AV_LISTEN"):
        f.write(k + "=" + os.environ.get(k, "") + "\n")
class H(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.end_headers()
        self.wfile.write(b"ok")
    def log_message(self, *a):
        pass
srv = http.server.HTTPServer((host, int(port)), H)
signal.signal(signal.SIGTERM, lambda *a: sys.exit(0))
print("fake daemon ready", flush=True)
srv.serve_forever()
"#;

#[test]
fn up_starts_a_background_daemon_and_down_stops_it() {
    if Command::new("python3").arg("--version").output().is_err() {
        println!("skip: python3 not installed");
        return;
    }
    let home = TempDir::new().unwrap();
    let av_home = home.path().join("av-home");
    let daemon = home.path().join("fake-daemon");
    write_script(&daemon, FAKE_DAEMON);
    let env_out = home.path().join("env.txt");
    let url = format!("http://127.0.0.1:{}", free_port());
    let env = [
        ("HOME", home.path().to_str().unwrap()),
        ("AV_HOME", av_home.to_str().unwrap()),
        ("AV_DAEMON_BIN", daemon.to_str().unwrap()),
        ("FAKE_ENV_OUT", env_out.to_str().unwrap()),
    ];
    let up = ok(av_env(&url, home.path(), &env, &["up"]));
    assert!(up.contains(&format!("daemon running at {url}")), "{up}");
    assert!(av_home.join("daemon.pid").exists());
    let got = std::fs::read_to_string(&env_out).unwrap();
    let port = url.rsplit(':').next().unwrap();
    assert!(
        got.contains(&format!("AV_LISTEN=127.0.0.1:{port}")),
        "{got}"
    );
    assert!(
        got.contains(&format!(
            "AV_STORE_ROOT={}",
            av_home.join("store").display()
        )),
        "{got}"
    );
    assert!(
        got.contains(&format!("AV_WS_ROOT={}", home.path().join("ws").display())),
        "{got}"
    );
    let log = std::fs::read_to_string(av_home.join("daemon.log")).unwrap();
    assert!(log.contains("fake daemon ready"), "{log}");

    let again = ok(av_env(&url, home.path(), &env, &["up"]));
    assert!(again.contains("already running"), "{again}");

    let down = ok(av_env(&url, home.path(), &env, &["down"]));
    assert!(down.contains("daemon stopped"), "{down}");
    assert!(!av_home.join("daemon.pid").exists());
    let gone = av_env(&url, home.path(), &env, &["repo", "add", "x", "/nowhere"]);
    assert!(err(&gone).contains("av up"), "{}", err(&gone));
    let none = ok(av_env(&url, home.path(), &env, &["down"]));
    assert!(none.contains("no daemon running"), "{none}");
}

#[test]
fn up_reports_the_log_when_the_daemon_dies_at_start() {
    let home = TempDir::new().unwrap();
    let daemon = home.path().join("broken-daemon");
    write_script(
        &daemon,
        "#!/bin/sh\necho 'AV_DATABASE_URL: boom' >&2\nexit 3\n",
    );
    let url = format!("http://127.0.0.1:{}", free_port());
    let av_home = home.path().join("av-home");
    let o = av_env(
        &url,
        home.path(),
        &[
            ("HOME", home.path().to_str().unwrap()),
            ("AV_HOME", av_home.to_str().unwrap()),
            ("AV_DAEMON_BIN", daemon.to_str().unwrap()),
        ],
        &["up"],
    );
    assert!(!o.status.success());
    assert!(
        err(&o).contains("exited") && err(&o).contains("boom"),
        "{}",
        err(&o)
    );
    assert!(!av_home.join("daemon.pid").exists());
}

#[test]
fn up_refuses_a_remote_daemon_url() {
    let home = TempDir::new().unwrap();
    let o = av_env(
        "http://av.invalid:7420",
        home.path(),
        &[("HOME", home.path().to_str().unwrap())],
        &["up"],
    );
    assert!(!o.status.success());
    assert!(err(&o).contains("this machine"), "{}", err(&o));
}

fn mcp_endpoint(agent: &str, home: &Path, token: &str) -> (String, Vec<(String, String)>) {
    let expand = |v: &str| {
        v.replace("${AV_TOKEN}", token)
            .replace("${env:AV_TOKEN}", token)
    };
    match agent {
        "codex" | "grok" => {
            let dir = if agent == "codex" { ".codex" } else { ".grok" };
            let text = std::fs::read_to_string(home.join(dir).join("config.toml")).unwrap();
            let doc: toml_edit::DocumentMut = text.parse().unwrap();
            let av = &doc["mcp_servers"]["av"];
            let url = av["url"].as_str().unwrap().to_owned();
            let mut headers = vec![];
            if let Some(var) = av.get("bearer_token_env_var").and_then(|v| v.as_str()) {
                assert_eq!(var, "AV_TOKEN");
                headers.push(("Authorization".into(), format!("Bearer {token}")));
            }
            // Codex 0.166.0 reads static headers from `http_headers`; Grok from `headers`
            let key = if agent == "codex" {
                "http_headers"
            } else {
                "headers"
            };
            if let Some(h) = av.get(key).and_then(|h| h.as_inline_table()) {
                for (k, v) in h.iter() {
                    headers.push((k.to_owned(), expand(v.as_str().unwrap())));
                }
            }
            (url, headers)
        }
        "gemini" | "cursor" | "antigravity" => {
            let file = match agent {
                "gemini" => ".gemini/settings.json",
                "cursor" => ".cursor/mcp.json",
                _ => ".gemini/config/mcp_config.json",
            };
            let v: serde_json::Value =
                serde_json::from_str(&std::fs::read_to_string(home.join(file)).unwrap()).unwrap();
            let av = &v["mcpServers"]["av"];
            let headers = av["headers"]
                .as_object()
                .map(|h| {
                    h.iter()
                        .map(|(k, v)| {
                            let v = v.as_str().unwrap();
                            // Antigravity's docs give no expansion in this file
                            let v = if agent == "antigravity" {
                                v.to_owned()
                            } else {
                                expand(v)
                            };
                            (k.clone(), v)
                        })
                        .collect()
                })
                .unwrap_or_default();
            let url = if agent == "gemini" {
                &av["httpUrl"]
            } else {
                &av["url"]
            };
            (url.as_str().unwrap().to_owned(), headers)
        }
        // OpenCode 2.x reads `mcp.servers`; the token is the literal, so nothing is expanded
        "opencode" => {
            let file = home.join(".config/opencode/opencode.json");
            let v: serde_json::Value =
                serde_json::from_str(&std::fs::read_to_string(file).unwrap()).unwrap();
            let av = &v["mcp"]["servers"]["av"];
            assert_eq!(av["type"], "remote");
            let headers = av["headers"]
                .as_object()
                .map(|h| {
                    h.iter()
                        .map(|(k, v)| (k.clone(), v.as_str().unwrap().to_owned()))
                        .collect()
                })
                .unwrap_or_default();
            (av["url"].as_str().unwrap().to_owned(), headers)
        }
        other => panic!("no reader for {other}"),
    }
}

struct McpClient {
    url: String,
    headers: Vec<(String, String)>,
    session: Option<String>,
}

impl McpClient {
    fn send(&mut self, msg: serde_json::Value) -> serde_json::Value {
        let mut req = ureq::post(&self.url)
            .set("accept", "application/json, text/event-stream")
            .set("content-type", "application/json");
        for (k, v) in &self.headers {
            req = req.set(k, v);
        }
        if let Some(s) = &self.session {
            req = req.set("mcp-session-id", s);
        }
        let resp = req.send_string(&msg.to_string()).unwrap();
        if let Some(s) = resp.header("mcp-session-id") {
            self.session = Some(s.to_owned());
        }
        let text = resp.into_string().unwrap();
        serde_json::from_str(&text).unwrap_or(serde_json::Value::Null)
    }

    fn tool(&mut self, name: &str, args: serde_json::Value) -> String {
        let r = self.send(
            serde_json::json!({"jsonrpc": "2.0", "id": 2, "method": "tools/call",
            "params": {"name": name, "arguments": args}}),
        );
        assert_eq!(r["result"]["isError"], false, "{name}: {r}");
        r["result"]["content"][0]["text"]
            .as_str()
            .unwrap_or_default()
            .to_owned()
    }
}

#[test]
fn agents_wired_by_init_can_start_record_checkpoint_and_submit() {
    let d = start_daemon_with(Some("sekrit"));
    let home = TempDir::new().unwrap();
    let env = [
        ("HOME", home.path().to_str().unwrap()),
        ("CODEX_HOME", ""),
        ("XDG_CONFIG_HOME", ""),
        ("AV_TOKEN", "sekrit"),
    ];
    let out = ok(av_env(
        &d.url,
        &d.src,
        &env,
        &[
            "init",
            "--name",
            "demo",
            "--for",
            "codex",
            "--for",
            "grok",
            "--for",
            "gemini",
            "--for",
            "cursor",
            "--for",
            "antigravity",
            "--for",
            "opencode",
        ],
    ));
    assert!(!out.contains("sekrit"), "{out}");
    for agent in [
        "codex",
        "grok",
        "gemini",
        "cursor",
        "antigravity",
        "opencode",
    ] {
        let (url, headers) = mcp_endpoint(agent, home.path(), "sekrit");
        assert_eq!(url, format!("{}/mcp", d.url), "{agent}");
        // without the header the daemon refuses: the config, not luck, carries the token
        let denied = ureq::post(&url)
            .set("content-type", "application/json")
            .send_string("{}");
        assert!(
            matches!(denied, Err(ureq::Error::Status(401, _))),
            "{agent}"
        );
        let mut c = McpClient {
            url,
            headers,
            session: None,
        };
        // the name each CLI gives itself, which is not the agent's name: the config's
        // `X-Av-Agent` header must win (Codex's checked against Codex CLI 0.166.0 on 2026-11-04,
        // Grok's against Grok Build 1.0.70; Gemini's, Cursor's, Antigravity's and OpenCode's are
        // stand-ins)
        let client = match agent {
            "codex" => "codex-mcp-client",
            "grok" => "grok-shell-av",
            _ => "some-mcp-client",
        };
        let init = c.send(
            serde_json::json!({"jsonrpc": "2.0", "id": 1, "method": "initialize",
            "params": {"protocolVersion": "2025-06-18", "capabilities": {},
                       "clientInfo": {"name": client, "version": "1"}}}),
        );
        assert!(
            init["result"]["capabilities"]["tools"].is_object(),
            "{agent}: {init}"
        );
        c.send(serde_json::json!({"jsonrpc": "2.0", "method": "notifications/initialized"}));
        let started = c.tool(
            "start_task",
            serde_json::json!({"repo": "demo", "title": format!("{agent} task"),
                               "goal": "Make hello louder"}),
        );
        let ws = started
            .lines()
            .find_map(|l| l.strip_prefix("workspace: "))
            .unwrap_or_else(|| panic!("{agent}: no workspace line in {started}"));
        let status = c.tool("status", serde_json::json!({}));
        assert!(
            status.contains(&format!("\"owner_agent\": \"{agent}\"")),
            "{agent}: {status}"
        );
        std::fs::write(
            Path::new(ws.trim()).join("app/greeter.py"),
            format!("# {agent}\nclass Greeter:\n    def hello(self):\n        return 'HELLO'\n"),
        )
        .unwrap();
        c.tool(
            "record",
            serde_json::json!({"kind": "note", "summary": format!("{agent} was here"),
                               "anchors": [{"path": "app/greeter.py"}]}),
        );
        c.tool("checkpoint", serde_json::json!({}));
        let submitted = c.tool("submit", serde_json::json!({}));
        assert!(submitted.contains("in_review"), "{agent}: {submitted}");
    }
}

/// Antigravity ships its MCP config as a 0-byte file; the `X-Av-Agent` header in it names the
/// session over its `clientInfo.name` (a stand-in here: Antigravity's own was not seen).
#[test]
fn antigravity_is_wired_into_its_empty_config_and_named_by_its_header() {
    let d = start_daemon();
    let home = TempDir::new().unwrap();
    let config = home.path().join(".gemini/config/mcp_config.json");
    std::fs::create_dir_all(config.parent().unwrap()).unwrap();
    std::fs::write(&config, "").unwrap();
    let env = [("HOME", home.path().to_str().unwrap())];
    let out = ok(av_env(
        &d.url,
        &d.src,
        &env,
        &["init", "--name", "demo", "--for", "antigravity"],
    ));
    assert!(out.contains("Antigravity MCP server `av`"), "{out}");
    let (url, headers) = mcp_endpoint("antigravity", home.path(), "");
    assert_eq!(url, format!("{}/mcp", d.url));
    assert_eq!(
        headers,
        vec![("X-Av-Agent".to_owned(), "antigravity".to_owned())]
    );
    let mut c = McpClient {
        url,
        headers,
        session: None,
    };
    c.send(
        serde_json::json!({"jsonrpc": "2.0", "id": 1, "method": "initialize",
        "params": {"protocolVersion": "2025-06-18", "capabilities": {},
                   "clientInfo": {"name": "some-mcp-client", "version": "1"}}}),
    );
    c.send(serde_json::json!({"jsonrpc": "2.0", "method": "notifications/initialized"}));
    c.tool(
        "start_task",
        serde_json::json!({"repo": "demo", "title": "agy task", "goal": "Make hello louder"}),
    );
    let status = c.tool("status", serde_json::json!({}));
    assert!(
        status.contains("\"owner_agent\": \"antigravity\""),
        "{status}"
    );

    let env = [
        ("HOME", home.path().to_str().unwrap()),
        ("AV_TOKEN", "sekrit"),
    ];
    let out = ok(av_env(
        &d.url,
        &d.src,
        &env,
        &["init", "--name", "demo", "--for", "antigravity"],
    ));
    assert!(
        out.contains("(the token is written into the file: run `av init` again when it changes)"),
        "{out}"
    );
    assert!(!out.contains("sekrit"), "{out}");
}

/// OpenCode 2.3.2 names itself `opencode` in `clientInfo` (seen 2026-11-22); a stand-in name here
/// shows the `X-Av-Agent` header carries it either way.
#[test]
fn opencode_is_wired_through_its_global_config_and_named_by_its_header() {
    let d = start_daemon();
    let home = TempDir::new().unwrap();
    let env = [
        ("HOME", home.path().to_str().unwrap()),
        ("XDG_CONFIG_HOME", ""),
    ];
    let out = ok(av_env(
        &d.url,
        &d.src,
        &env,
        &["init", "--name", "demo", "--for", "opencode"],
    ));
    assert!(out.contains("OpenCode MCP server `av`"), "{out}");
    assert!(!out.contains("also reads"), "{out}");
    let plugins = home.path().join(".config/opencode/plugins");
    assert!(
        out.contains(&format!(
            "OpenCode plugin: {}",
            plugins.join("av.js").display()
        )),
        "{out}"
    );
    let plugin = std::fs::read_to_string(plugins.join("av.js")).unwrap();
    let bin = serde_json::to_string(env!("CARGO_BIN_EXE_av")).unwrap();
    assert_eq!(
        plugin.lines().nth(1),
        Some(format!("const AV_BIN = {bin};").as_str())
    );
    let (url, headers) = mcp_endpoint("opencode", home.path(), "");
    assert_eq!(url, format!("{}/mcp", d.url));
    assert_eq!(
        headers,
        vec![("X-Av-Agent".to_owned(), "opencode".to_owned())]
    );
    let mut c = McpClient {
        url,
        headers,
        session: None,
    };
    c.send(
        serde_json::json!({"jsonrpc": "2.0", "id": 1, "method": "initialize",
        "params": {"protocolVersion": "2025-11-25", "capabilities": {},
                   "clientInfo": {"name": "some-mcp-client", "version": "1"}}}),
    );
    c.send(serde_json::json!({"jsonrpc": "2.0", "method": "notifications/initialized"}));
    c.tool(
        "start_task",
        serde_json::json!({"repo": "demo", "title": "opencode task", "goal": "Make hello louder"}),
    );
    let status = c.tool("status", serde_json::json!({}));
    assert!(status.contains("\"owner_agent\": \"opencode\""), "{status}");

    std::fs::write(plugins.join("mine.js"), "export default {}\n").unwrap();
    let env = [
        ("HOME", home.path().to_str().unwrap()),
        ("XDG_CONFIG_HOME", ""),
        ("AV_TOKEN", "sekrit"),
    ];
    let out = ok(av_env(
        &d.url,
        &d.src,
        &env,
        &["init", "--name", "demo", "--for", "opencode"],
    ));
    assert!(
        out.contains("(the token is written into the file: run `av init` again when it changes)"),
        "{out}"
    );
    assert!(!out.contains("sekrit"), "{out}");
    let (_, headers) = mcp_endpoint("opencode", home.path(), "");
    assert!(
        headers.contains(&("Authorization".to_owned(), "Bearer sekrit".to_owned())),
        "{headers:?}"
    );
    // OpenCode loads every plugin in the folder: av's is replaced, nothing is added beside it
    let mut names: Vec<String> = std::fs::read_dir(&plugins)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    assert_eq!(names, ["av.js", "mine.js"]);
    assert!(
        !std::fs::read_to_string(plugins.join("av.js"))
            .unwrap()
            .contains("sekrit")
    );

    let xdg = TempDir::new().unwrap();
    let elsewhere = TempDir::new().unwrap();
    let env = [
        ("HOME", elsewhere.path().to_str().unwrap()),
        ("XDG_CONFIG_HOME", xdg.path().to_str().unwrap()),
    ];
    ok(av_env(
        &d.url,
        &d.src,
        &env,
        &["init", "--name", "demo", "--for", "opencode"],
    ));
    assert!(xdg.path().join("opencode/opencode.json").exists());
    assert!(!elsewhere.path().join(".config").exists());

    // OpenCode loads opencode.jsonc after opencode.json, so an `av` entry there wins
    let jsonc = xdg.path().join("opencode/opencode.jsonc");
    std::fs::write(&jsonc, "{}\n").unwrap();
    let out = ok(av_env(
        &d.url,
        &d.src,
        &env,
        &["init", "--name", "demo", "--for", "opencode"],
    ));
    assert!(
        out.contains(&format!(
            "OpenCode also reads {}: an mcp.servers.av entry there overrides this one",
            jsonc.display()
        )),
        "{out}"
    );

    // a file av cannot edit gets the entry by hand, and the hooks still come from the plugin
    let commented = TempDir::new().unwrap();
    std::fs::create_dir_all(commented.path().join("opencode")).unwrap();
    let config = commented.path().join("opencode/opencode.json");
    std::fs::write(&config, "// mine\n{}\n").unwrap();
    let env = [
        ("HOME", elsewhere.path().to_str().unwrap()),
        ("XDG_CONFIG_HOME", commented.path().to_str().unwrap()),
    ];
    let o = av_env(
        &d.url,
        &d.src,
        &env,
        &["init", "--name", "demo", "--for", "opencode"],
    );
    assert!(!o.status.success());
    assert!(err(&o).contains("comments"), "{}", err(&o));
    let plugin = commented.path().join("opencode/plugins/av.js");
    let printed = String::from_utf8_lossy(&o.stdout);
    assert!(
        printed.contains(&format!("OpenCode plugin: {}", plugin.display())),
        "{printed}"
    );
    assert!(plugin.exists());
    assert_eq!(std::fs::read_to_string(&config).unwrap(), "// mine\n{}\n");
}

/// `node` from `PATH`, else `None` after a `SKIPPED` line: the OpenCode plugin's tests need it.
fn node_or_skip(test: &str) -> Option<PathBuf> {
    let node = std::env::var_os("PATH")
        .map(|p| std::env::split_paths(&p).collect::<Vec<_>>())
        .unwrap_or_default()
        .into_iter()
        .map(|d| d.join("node"))
        .find(|p| p.is_file());
    if node.is_none() {
        use std::io::Write;
        let _ = writeln!(
            std::io::stderr(),
            "SKIPPED {test}: node is not on PATH, so the OpenCode plugin was NOT tested"
        );
    }
    node
}

/// OpenCode's config dir as `av init --for opencode` leaves it: `opencode.json` naming the daemon
/// at `url` (no file for `None`) and `plugins/av.js` running `av_bin`.
fn opencode_config(url: Option<&str>, av_bin: &Path) -> TempDir {
    let dir = TempDir::new().unwrap();
    if let Some(url) = url {
        let config = serde_json::json!({"mcp": {"servers": {"av": {
            "type": "remote", "url": format!("{url}/mcp"), "headers": {"X-Av-Agent": "opencode"},
        }}}});
        std::fs::write(dir.path().join("opencode.json"), config.to_string()).unwrap();
    }
    let plugin = std::fs::read_to_string(repo_root().join("integrations/opencode/av.js")).unwrap();
    let line = "const AV_BIN = \"av\";";
    assert_eq!(plugin.lines().nth(1), Some(line));
    let bin = serde_json::to_string(av_bin.to_str().unwrap()).unwrap();
    std::fs::create_dir_all(dir.path().join("plugins")).unwrap();
    std::fs::write(
        dir.path().join("plugins/av.js"),
        plugin.replacen(line, &format!("const AV_BIN = {bin};"), 1),
    )
    .unwrap();
    dir
}

/// Plays `script` (see `tests/opencode/harness.mjs`) to the plugin in `config` under `node`, with
/// the environment of a server that has no av variables (`env` on top), from a directory that is
/// no workspace. Every hook must return, whatever happened; the result is what the plugin did.
fn play(
    node: &Path,
    config: &Path,
    env: &[(&str, &str)],
    script: serde_json::Value,
) -> serde_json::Value {
    let cwd = TempDir::new().unwrap();
    let file = cwd.path().join("script.json");
    std::fs::write(&file, script.to_string()).unwrap();
    let mut cmd = Command::new(node);
    cmd.current_dir(cwd.path())
        .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/opencode/harness.mjs"))
        .arg(config.join("plugins/av.js"))
        .arg(&file)
        .env("AV_FOOTPRINT", "off");
    for var in [
        "AV_URL",
        "AV_TOKEN",
        "AV_AGENT",
        "AV_TASK",
        "AV_SESSION",
        "AV_HOOK_WAIT_SECS",
        "AV_HOOK_MAX_CONTINUES",
        "AV_OPENCODE_HOOK_TIMEOUT_MS",
        "AV_FOOTPRINT_ROOTS",
        "AV_WS_ROOT",
        "CLAUDE_ENV_FILE",
        "GROK_HOOK_EVENT",
        "GROK_SESSION_ID",
    ] {
        cmd.env_remove(var);
    }
    for (k, v) in env {
        cmd.env(k, v);
    }
    let o = cmd.output().unwrap();
    assert!(o.status.success(), "{}{}", out(&o), err(&o));
    let v: serde_json::Value = serde_json::from_str(&out(&o))
        .unwrap_or_else(|e| panic!("{e}: {}\nstderr: {}", out(&o), err(&o)));
    for step in v["steps"].as_array().unwrap() {
        assert_eq!(step["returned"], true, "{step}\nstderr: {}", err(&o));
    }
    v
}

fn oc_prompt(session: &str) -> serde_json::Value {
    serde_json::json!({"hook": "prompt", "event": {
        "sessionID": session, "messageID": "msg", "prompt": {"text": "go"}, "delivery": "steer",
    }})
}

fn oc_compaction(session: &str) -> serde_json::Value {
    serde_json::json!({"hook": "compaction", "event": {"sessionID": session}})
}

fn oc_idle(session: &str, background: serde_json::Value) -> serde_json::Value {
    serde_json::json!({"hook": "experimental.idle", "event": {
        "sessionID": session, "background": background,
    }})
}

/// What the tool read or wrote, as the result's content OpenCode hands `execute.after`.
const OC_CONTENT: &str = "tool output";

fn oc_tool(session: &str, tool: &str, input: serde_json::Value) -> serde_json::Value {
    serde_json::json!({"hook": "tool.execute.after", "event": {
        "tool": tool, "sessionID": session, "agent": "build", "messageID": "msg", "id": "call",
        "input": input, "status": "completed",
        "result": {"output": OC_CONTENT, "content": [{"type": "text", "text": OC_CONTENT}]},
    }})
}

/// The text parts the plugin added to step `i`'s result.
fn oc_added(v: &serde_json::Value, i: usize) -> Vec<String> {
    let content = v["steps"][i]["content"].as_array().unwrap();
    assert_eq!(content[0]["text"], OC_CONTENT, "{v}");
    content[1..]
        .iter()
        .map(|p| p["text"].as_str().unwrap().to_owned())
        .collect()
}

/// A stand-in `av` that appends `$AV_AGENT`, the hook's name and its stdin to `av.log` in `dir`
/// (one line per call), then runs the real one on that stdin.
fn logging_av(dir: &Path) -> (PathBuf, PathBuf) {
    let log = dir.join("av.log");
    let bin = dir.join("av-logged");
    write_script(
        &bin,
        &format!(
            "#!/bin/sh\nin=$(/bin/cat)\nprintf '%s %s %s\\n' \"$AV_AGENT\" \"$2\" \"$in\" >> '{}'\n\
             printf '%s' \"$in\" | '{}' \"$@\"\n",
            log.display(),
            env!("CARGO_BIN_EXE_av")
        ),
    );
    (bin, log)
}

/// The stdin of each logged call of `hook`, with the `AV_AGENT` it ran under.
fn logged(log: &Path, hook: &str) -> Vec<(String, serde_json::Value)> {
    std::fs::read_to_string(log)
        .unwrap_or_default()
        .lines()
        .filter_map(|l| {
            let (agent, rest) = l.split_once(' ')?;
            let (name, input) = rest.split_once(' ')?;
            (name == hook).then(|| (agent.to_owned(), serde_json::from_str(input).unwrap()))
        })
        .collect()
}

#[cfg(unix)]
#[test]
fn opencode_plugin_puts_the_pack_once_and_again_after_compaction() {
    let Some(node) = node_or_skip("opencode_plugin_puts_the_pack_once_and_again_after_compaction")
    else {
        return;
    };
    let (d, ws, _scratch) = started();
    let config = opencode_config(Some(&d.url), Path::new(env!("CARGO_BIN_EXE_av")));
    let v = play(
        &node,
        config.path(),
        &[],
        serde_json::json!({
            "sessions": {"s1": {"directory": ws}},
            "steps": [oc_prompt("s1"), oc_prompt("s1"), oc_compaction("s1"), oc_prompt("s1")],
        }),
    );
    assert_eq!(
        v["hooks"],
        serde_json::json!([
            "compaction",
            "experimental.idle",
            "prompt",
            "tool.execute.after"
        ])
    );
    let puts = v["puts"].as_array().unwrap();
    assert_eq!(puts.len(), 2, "{v}");
    for put in puts {
        assert_eq!(
            (&put["sessionID"], &put["key"]),
            (&"s1".into(), &"av-pack".into())
        );
        let pack = put["value"].as_str().unwrap();
        assert!(pack.contains("Improve greeting"), "{pack}");
        assert!(!pack.contains("sent earlier"), "sent in full: {pack}");
    }
}

#[cfg(unix)]
#[test]
fn opencode_plugin_does_not_put_a_pack_that_is_already_there() {
    let Some(node) = node_or_skip("opencode_plugin_does_not_put_a_pack_that_is_already_there")
    else {
        return;
    };
    let (d, ws, scratch) = started();
    let (bin, log) = logging_av(scratch.path());
    let config = opencode_config(Some(&d.url), &bin);
    let v = play(
        &node,
        config.path(),
        &[],
        serde_json::json!({
            "sessions": {"s1": {"directory": ws}},
            "entries": {"s1": [{"key": "av-pack", "value": "the pack from before"}]},
            "steps": [oc_prompt("s1"), oc_prompt("s1"), oc_compaction("s1"), oc_prompt("s1")],
        }),
    );
    let starts = logged(&log, "session-start");
    assert_eq!(starts.len(), 1, "{starts:?}");
    assert_eq!(
        starts[0].1,
        serde_json::json!({"session_id": "s1", "source": "compact", "cwd": ws})
    );
    let puts = v["puts"].as_array().unwrap();
    assert_eq!(puts.len(), 1, "{v}");
    assert!(
        puts[0]["value"]
            .as_str()
            .unwrap()
            .contains("Improve greeting")
    );
}

#[cfg(unix)]
#[test]
fn opencode_plugin_reads_an_entry_added_by_hand() {
    let Some(node) = node_or_skip("opencode_plugin_reads_an_entry_added_by_hand") else {
        return;
    };
    let (d, ws, _scratch) = started();
    let entry = |url: &str| {
        format!(
            "{{\"type\": \"remote\", \"url\": \"{url}/mcp\", // the daemon\n \
             \"headers\": {{\"X-Av-Agent\": \"opencode\",}},}}"
        )
    };
    let commented = format!(
        "// added by hand\n{{\"mcp\": {{\"servers\": {{\n  /* av */ \"av\": {},\n}}}}}}\n",
        entry(&d.url)
    );
    let wrong = "http://127.0.0.1:9";
    let server = |url: &str| format!("{{\"mcp\": {{\"servers\": {{\"av\": {}}}}}}}", entry(url));
    let cases = [
        (
            "a commented opencode.json",
            Some(commented.clone()),
            None,
            "",
        ),
        ("opencode.jsonc alone", None, Some(commented.clone()), ""),
        (
            "opencode.jsonc after opencode.json",
            Some(server(wrong)),
            Some(commented),
            "",
        ),
        // OpenCode's to expand: the server's own AV_URL stands in
        ("an {env:} url", Some(server("{env:AV_URL}")), None, &*d.url),
    ];
    for (case, json, jsonc, server_url) in cases {
        let config = opencode_config(None, Path::new(env!("CARGO_BIN_EXE_av")));
        for (name, text) in [("opencode.json", json), ("opencode.jsonc", jsonc)] {
            if let Some(text) = text {
                std::fs::write(config.path().join(name), text).unwrap();
            }
        }
        let env: &[(&str, &str)] = if server_url.is_empty() {
            &[]
        } else {
            &[("AV_URL", server_url)]
        };
        let v = play(
            &node,
            config.path(),
            env,
            serde_json::json!({
                "sessions": {"s1": {"directory": ws}},
                "steps": [oc_prompt("s1")],
            }),
        );
        let puts = v["puts"].as_array().unwrap();
        assert_eq!(puts.len(), 1, "{case}: {v}");
        assert!(
            puts[0]["value"]
                .as_str()
                .unwrap()
                .contains("Improve greeting"),
            "{case}: {v}"
        );
    }
}

/// Another task's record on `app/greeter.py`, for `pre-read` to hand on.
fn record_in_another_task(d: &Daemon, scratch: &Path) {
    ok(av(
        d,
        scratch,
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
    ok(av(
        d,
        &d.ws.join("demo/earlier-try"),
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
}

#[cfg(unix)]
#[test]
fn opencode_plugin_adds_other_tasks_records_to_a_read() {
    let Some(node) = node_or_skip("opencode_plugin_adds_other_tasks_records_to_a_read") else {
        return;
    };
    let (d, ws, scratch) = started();
    record_in_another_task(&d, scratch.path());
    let config = opencode_config(Some(&d.url), Path::new(env!("CARGO_BIN_EXE_av")));
    // as models write it: relative to the session's directory, not the server's
    let read = oc_tool("s1", "read", serde_json::json!({"path": "app/greeter.py"}));
    let v = play(
        &node,
        config.path(),
        &[],
        serde_json::json!({"sessions": {"s1": {"directory": ws}}, "steps": [read, read]}),
    );
    let added = oc_added(&v, 0);
    assert_eq!(added.len(), 1, "{v}");
    assert!(added[0].contains("greetings went stale"), "{added:?}");
    assert_eq!(oc_added(&v, 1), Vec::<String>::new(), "each record once");
}

#[cfg(unix)]
#[test]
fn opencode_plugin_checkpoints_after_an_edit() {
    let Some(node) = node_or_skip("opencode_plugin_checkpoints_after_an_edit") else {
        return;
    };
    let (d, ws, _scratch) = started();
    let config = opencode_config(Some(&d.url), Path::new(env!("CARGO_BIN_EXE_av")));
    let run = |step: serde_json::Value| {
        play(
            &node,
            config.path(),
            &[],
            serde_json::json!({"sessions": {"s1": {"directory": ws}}, "steps": [step]}),
        )
    };
    std::fs::write(ws.join("app/greeter.py"), "class Greeter:\n    pass\n").unwrap();
    run(oc_tool(
        "s1",
        "read",
        serde_json::json!({"path": "app/greeter.py"}),
    ));
    let status = ok(av(&d, &ws, &["status"]));
    assert!(!status.contains("app/greeter.py"), "a read: {status}");
    run(oc_tool(
        "s1",
        "edit",
        serde_json::json!({"path": "app/greeter.py", "oldString": "x", "newString": "y"}),
    ));
    let status = ok(av(&d, &ws, &["status"]));
    assert!(status.contains("changed (1): app/greeter.py"), "{status}");
    // past post-edit's 2 s debounce, as the next edit of a session would be
    set_last_checkpoint_ms(&ws, now_ms() - 2_100);
    std::fs::write(ws.join("README.md"), "# changed\n").unwrap();
    run(oc_tool(
        "s1",
        "write",
        serde_json::json!({"path": ws.join("README.md"), "content": "# changed\n"}),
    ));
    let status = ok(av(&d, &ws, &["status"]));
    assert!(status.contains("changed (2):"), "{status}");
    // `patch` names its files in the result (seen with OpenCode 2.3.2), not in the input
    set_last_checkpoint_ms(&ws, now_ms() - 2_100);
    std::fs::write(ws.join("app/extra.py"), "X = 1\n").unwrap();
    let mut patch = oc_tool(
        "s1",
        "patch",
        serde_json::json!({"patchText": "*** Begin Patch"}),
    );
    patch["event"]["result"]["metadata"] =
        serde_json::json!({"files": [{"file": "app/extra.py", "status": "added"}]});
    run(patch);
    let status = ok(av(&d, &ws, &["status"]));
    assert!(status.contains("changed (3):"), "{status}");
}

/// A server with no `AV_AGENT` runs unnamed hooks, which act as the task's owner whoever started
/// it, as Claude Code's do; the entry's `X-Av-Agent` names the MCP session only.
#[cfg(unix)]
#[test]
fn opencode_plugin_acts_as_the_tasks_owner() {
    let Some(node) = node_or_skip("opencode_plugin_acts_as_the_tasks_owner") else {
        return;
    };
    let (d, ws, _scratch) = started_as("alice");
    let config = opencode_config(Some(&d.url), Path::new(env!("CARGO_BIN_EXE_av")));
    let code = "def louder(name):\n    return name.upper()\n";
    std::fs::write(ws.join("app/extra.py"), code).unwrap();
    let v = play(
        &node,
        config.path(),
        &[],
        serde_json::json!({
            "sessions": {"s1": {"directory": ws}},
            "steps": [
                oc_tool("s1", "write", serde_json::json!({"path": "app/extra.py", "content": code})),
                oc_idle("s1", serde_json::json!([])),
            ],
        }),
    );
    let status = ok(av(&d, &ws, &["status"]));
    assert!(status.contains("changed (1): app/extra.py"), "{status}");
    let added = oc_added(&v, 0);
    assert!(!added.iter().any(|t| t.contains("now owns")), "{added:?}");
    let sent = v["synthetic"].as_array().unwrap();
    assert_eq!(sent.len(), 1, "{v}");
    let text = sent[0]["text"].as_str().unwrap();
    assert!(text.contains("no record explains"), "{text}");
    assert!(text.contains("louder"), "{text}");
}

#[cfg(unix)]
#[test]
fn opencode_plugin_asks_once_for_records_then_offers_queued_work() {
    let Some(node) = node_or_skip("opencode_plugin_asks_once_for_records_then_offers_queued_work")
    else {
        return;
    };
    let (d, ws, scratch) = started();
    std::fs::write(
        ws.join("app/greeter.py"),
        "class Greeter:\n    def hello(self, name):\n        return name.upper()\n",
    )
    .unwrap();
    post_item(&d, scratch.path(), "Fix the README typos");
    let (bin, log) = logging_av(scratch.path());
    let config = opencode_config(Some(&d.url), &bin);
    let shell = serde_json::json!([{"kind": "shell", "sessionID": "s1", "shellID": "sh", "command": "sleep 9"}]);
    // no AV_HOOK_WAIT_SECS on the server: the plugin's own default applies
    let v = play(
        &node,
        config.path(),
        &[],
        serde_json::json!({
            "sessions": {"s1": {"directory": ws}},
            "steps": [
                oc_idle("s1", serde_json::json!([])),
                oc_idle("s1", serde_json::json!([])),
                oc_idle("s1", shell),
            ],
        }),
    );
    let sent: Vec<&str> = v["synthetic"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| {
            assert_eq!(s["sessionID"], "s1");
            s["text"].as_str().unwrap()
        })
        .collect();
    assert_eq!(sent.len(), 2, "{v}");
    assert!(sent[0].contains("no record explains"), "{}", sent[0]);
    assert!(sent[0].contains("Greeter.hello"), "{}", sent[0]);
    assert!(sent[1].contains("Fix the README typos"), "{}", sent[1]);
    assert!(
        sent[1].ends_with("call start_task with no title"),
        "{}",
        sent[1]
    );
    let stops = logged(&log, "stop");
    assert_eq!(
        stops.len(),
        2,
        "no stop while the session has background work"
    );
    for ((agent, input), active) in stops.iter().zip([false, true]) {
        assert_eq!(agent, "", "unnamed, so the hook acts as the task's owner");
        assert_eq!(
            *input,
            serde_json::json!({"session_id": "s1", "cwd": ws, "stop_hook_active": active})
        );
    }
}

#[cfg(unix)]
#[test]
fn opencode_plugin_is_silent_outside_a_task_and_does_not_wait_for_the_pack() {
    let Some(node) =
        node_or_skip("opencode_plugin_is_silent_outside_a_task_and_does_not_wait_for_the_pack")
    else {
        return;
    };
    let (d, _ws, scratch) = started();
    post_item(&d, scratch.path(), "Fix the README typos");
    let outside = TempDir::new().unwrap();
    std::fs::write(outside.path().join("x.py"), "X = 1\n").unwrap();
    let (bin, _log) = logging_av(scratch.path());
    let config = opencode_config(Some(&d.url), &bin);
    let v = play(
        &node,
        config.path(),
        &[],
        serde_json::json!({
            "sessions": {"s1": {"directory": outside.path()}},
            "steps": [
                oc_prompt("s1"),
                oc_tool("s1", "read", serde_json::json!({"path": "x.py"})),
                oc_tool("s1", "edit", serde_json::json!({"path": "x.py"})),
                oc_idle("s1", serde_json::json!([])),
            ],
        }),
    );
    assert_eq!(
        (&v["puts"], &v["synthetic"]),
        (&serde_json::json!([]), &serde_json::json!([]))
    );
    assert_eq!(oc_added(&v, 1), Vec::<String>::new());
    assert_eq!(oc_added(&v, 2), Vec::<String>::new());

    // an hour-long `session-start`: the prompt goes on, and the hook still started it
    let pid = scratch.path().join("pid");
    let slow = scratch.path().join("slow-av");
    write_script(
        &slow,
        &format!(
            "#!/bin/sh\necho $$ > '{}'\nexec /bin/sleep 3600\n",
            pid.display()
        ),
    );
    let config = opencode_config(Some(&d.url), &slow);
    let v = play(
        &node,
        config.path(),
        &[],
        serde_json::json!({
            "sessions": {"s1": {"directory": outside.path()}},
            "steps": [oc_prompt("s1"), {"untilFile": pid}],
        }),
    );
    let pid = std::fs::read_to_string(&pid).unwrap();
    let _ = Command::new("kill").arg(pid.trim()).status();
    assert_eq!(v["puts"], serde_json::json!([]));
}

#[cfg(unix)]
#[test]
fn opencode_plugin_survives_a_broken_av() {
    let Some(node) = node_or_skip("opencode_plugin_survives_a_broken_av") else {
        return;
    };
    let (d, ws, scratch) = started();
    record_in_another_task(&d, scratch.path());
    std::fs::write(ws.join("app/greeter.py"), "class Greeter:\n    pass\n").unwrap();
    post_item(&d, scratch.path(), "Fix the README typos");
    let steps = serde_json::json!([
        oc_prompt("s1"),
        oc_tool("s1", "read", serde_json::json!({"path": "app/greeter.py"})),
        oc_tool("s1", "edit", serde_json::json!({"path": "app/greeter.py"})),
        oc_idle("s1", serde_json::json!([])),
    ]);
    let script = serde_json::json!({"sessions": {"s1": {"directory": ws}}, "steps": steps});
    let untouched = |v: &serde_json::Value, what: &str| {
        assert_eq!(v["puts"], serde_json::json!([]), "{what}: {v}");
        assert_eq!(v["synthetic"], serde_json::json!([]), "{what}: {v}");
        assert_eq!(oc_added(v, 1), Vec::<String>::new(), "{what}");
        assert_eq!(oc_added(v, 2), Vec::<String>::new(), "{what}");
    };
    // what each hook would act on, but from a broken av
    let answers = "case \"$2\" in session-start) echo PACK ;; \
                   stop) echo '{\"decision\": \"block\", \"reason\": \"go on\"}' ;; \
                   *) echo '{\"hookSpecificOutput\": {\"additionalContext\": \"more\"}}' ;; esac";
    let scripts = [
        (
            "garbage",
            "case \"$2\" in session-start) ;; *) echo '{\"decision\": ' ;; esac".to_owned(),
        ),
        ("exit 1", format!("{answers}\nexit 1")),
        ("past the limit", "exec /bin/sleep 3600".to_owned()),
    ];
    for (what, body) in scripts {
        let bin = scratch.path().join("broken-av");
        write_script(&bin, &format!("#!/bin/sh\n{body}\n"));
        let config = opencode_config(Some(&d.url), &bin);
        let env = [
            ("AV_OPENCODE_HOOK_TIMEOUT_MS", "300"),
            ("AV_HOOK_WAIT_SECS", "0"),
        ];
        untouched(&play(&node, config.path(), &env, script.clone()), what);
    }

    let empty = TempDir::new().unwrap();
    let config = opencode_config(Some(&d.url), &empty.path().join("av"));
    let path = [("PATH", empty.path().to_str().unwrap())];
    untouched(&play(&node, config.path(), &path, script.clone()), "no av");

    let config = opencode_config(
        Some("http://127.0.0.1:1"),
        Path::new(env!("CARGO_BIN_EXE_av")),
    );
    untouched(
        &play(&node, config.path(), &[], script.clone()),
        "dead daemon",
    );

    // OpenCode's own calls failing: the first nudge, then the queued work, reach `synthetic`
    let (bin, log) = logging_av(scratch.path());
    for fails in [
        serde_json::json!(["session.get"]),
        serde_json::json!(["entry.list", "synthetic"]),
        serde_json::json!(["entry.put", "synthetic"]),
    ] {
        let config = opencode_config(Some(&d.url), &bin);
        let mut script = script.clone();
        script["throw"] = fails.clone();
        let v = play(&node, config.path(), &[], script);
        assert_eq!(v["puts"], serde_json::json!([]), "{fails}: {v}");
        assert_eq!(v["synthetic"], serde_json::json!([]), "{fails}: {v}");
    }

    // without an `av` entry the children run with the server's AV_URL, and unnamed
    std::fs::remove_file(&log).unwrap();
    let config = opencode_config(None, &bin);
    let v = play(&node, config.path(), &[("AV_URL", &d.url)], script.clone());
    assert_eq!(v["puts"].as_array().unwrap().len(), 1, "{v}");
    let starts = logged(&log, "session-start");
    assert_eq!(starts.len(), 1, "{starts:?}");
    assert_eq!(starts[0].0, "");

    // a name comes from the server's environment (`av watch` sets one), not the entry's header
    std::fs::remove_file(&log).unwrap();
    let config = opencode_config(Some(&d.url), &bin);
    let v = play(
        &node,
        config.path(),
        &[("AV_AGENT", "watcher")],
        script.clone(),
    );
    assert_eq!(v["puts"].as_array().unwrap().len(), 1, "{v}");
    let starts = logged(&log, "session-start");
    assert_eq!(starts.len(), 1, "{starts:?}");
    assert_eq!(starts[0].0, "watcher");
}

#[cfg(unix)]
#[test]
fn opencode_plugin_leaves_child_sessions_their_tool_hooks_only() {
    let Some(node) = node_or_skip("opencode_plugin_leaves_child_sessions_their_tool_hooks_only")
    else {
        return;
    };
    let (d, ws, scratch) = started();
    record_in_another_task(&d, scratch.path());
    std::fs::write(ws.join("app/greeter.py"), "class Greeter:\n    pass\n").unwrap();
    post_item(&d, scratch.path(), "Fix the README typos");
    let config = opencode_config(Some(&d.url), Path::new(env!("CARGO_BIN_EXE_av")));
    let v = play(
        &node,
        config.path(),
        &[],
        serde_json::json!({
            "sessions": {"child": {"directory": ws, "parentID": "s1"}},
            "steps": [
                oc_prompt("child"),
                oc_tool("child", "read", serde_json::json!({"path": "app/greeter.py"})),
                oc_tool("child", "edit", serde_json::json!({"path": "app/greeter.py"})),
                oc_idle("child", serde_json::json!([])),
            ],
        }),
    );
    assert_eq!(
        (&v["puts"], &v["synthetic"]),
        (&serde_json::json!([]), &serde_json::json!([]))
    );
    let added = oc_added(&v, 1);
    assert!(
        added.len() == 1 && added[0].contains("greetings went stale"),
        "{added:?}"
    );
    let status = ok(av(&d, &ws, &["status"]));
    assert!(status.contains("changed (1): app/greeter.py"), "{status}");
}

#[cfg(unix)]
#[test]
fn opencode_plugin_sends_nothing_after_an_interrupt() {
    let Some(node) = node_or_skip("opencode_plugin_sends_nothing_after_an_interrupt") else {
        return;
    };
    let (d, ws, scratch) = started();
    let pid = scratch.path().join("pid");
    let bin = scratch.path().join("av-pid");
    write_script(
        &bin,
        &format!(
            "#!/bin/sh\necho $$ > '{}'\nexec '{}' \"$@\"\n",
            pid.display(),
            env!("CARGO_BIN_EXE_av")
        ),
    );
    let config = opencode_config(Some(&d.url), &bin);
    // with nothing queued, `av hook stop` waits an hour for work: the hook returns at all only
    // because the interrupt ended it
    let interrupted = serde_json::json!({
        "type": "session.execution.interrupted", "data": {"sessionID": "s1", "reason": "user"},
    });
    let mut idle = oc_idle("s1", serde_json::json!([]));
    idle["wait"] = false.into();
    let v = play(
        &node,
        config.path(),
        &[("AV_HOOK_WAIT_SECS", "3600")],
        serde_json::json!({
            "sessions": {"s1": {"directory": ws}},
            "steps": [idle, {"untilFile": pid}, {"emit": interrupted}],
        }),
    );
    assert_eq!(v["synthetic"], serde_json::json!([]), "{v}");
    let pid = std::fs::read_to_string(&pid).unwrap();
    let start = std::time::Instant::now();
    while alive(pid.trim()) {
        assert!(start.elapsed() < HANG, "av hook stop {pid} still runs");
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}

fn daemon_with_repo() -> (Daemon, TempDir) {
    let d = start_daemon();
    let scratch = TempDir::new().unwrap();
    ok(av(
        &d,
        scratch.path(),
        &["repo", "add", "demo", d.src.to_str().unwrap()],
    ));
    (d, scratch)
}

#[test]
fn agent_set_and_list_show_the_profiles_cheapest_first() {
    let (d, scratch) = daemon_with_repo();
    ok(av(
        &d,
        scratch.path(),
        &[
            "agent",
            "set",
            "claude",
            "--best",
            "code,tests",
            "--can",
            "docs",
            "--cost",
            "4",
        ],
    ));
    ok(av(
        &d,
        scratch.path(),
        &[
            "agent",
            "set",
            "muse",
            "--can",
            "code,docs",
            "--cost",
            "1",
            "--alias",
            "muse-code",
            "--alias",
            "muse-cli",
        ],
    ));
    let list = ok(av(&d, scratch.path(), &["agent", "list"]));
    let lines: Vec<&str> = list.lines().collect();
    assert!(lines[0].starts_with("muse"), "{list}");
    assert!(lines[0].contains("cost 1"), "{list}");
    assert!(lines[0].contains("can code,docs"), "{list}");
    assert!(lines[0].contains("aliases muse-code,muse-cli"), "{list}");
    assert!(lines[1].starts_with("claude"), "{list}");
    assert!(lines[1].contains("best code,tests"), "{list}");
}

#[test]
fn agent_rm_removes_a_profile_and_force_clears_one_that_owns_work() {
    let (d, scratch) = daemon_with_repo();
    ok(av(
        &d,
        scratch.path(),
        &["agent", "set", "muse", "--can", "docs", "--cost", "1"],
    ));
    ok(av(
        &d,
        scratch.path(),
        &["agent", "set", "agent", "--can", "code", "--cost", "3"],
    ));
    ok(av(
        &d,
        scratch.path(),
        &[
            "task",
            "start",
            "--repo",
            "demo",
            "--title",
            "Still open",
            "--goal",
            "g",
        ],
    ));
    let blocked = av(&d, scratch.path(), &["agent", "rm", "agent"]);
    assert!(!blocked.status.success(), "{}", out(&blocked));
    let msg = err(&blocked);
    assert!(msg.contains("HTTP 409"), "{msg}");
    assert!(msg.contains("force=true"), "{msg}");
    let forced = ok(av(&d, scratch.path(), &["agent", "rm", "agent", "--force"]));
    assert!(forced.contains("removed agent"), "{forced}");
    let rm = ok(av(&d, scratch.path(), &["agent", "rm", "muse"]));
    assert!(rm.contains("removed muse"), "{rm}");
    let missing = av(&d, scratch.path(), &["agent", "rm", "muse"]);
    assert!(!missing.status.success());
    let msg = err(&missing);
    assert!(msg.contains("not found: agent muse"), "{msg}");
    assert!(msg.contains("HTTP 404"), "{msg}");
    let list = ok(av(&d, scratch.path(), &["agent", "list"]));
    assert!(list.contains("no profiles"), "{list}");
}

#[test]
fn post_after_shows_waiting_on_in_the_backlog() {
    let (d, scratch) = daemon_with_repo();
    let post = |title: &str, extra: &[&str]| {
        let mut args = vec![
            "post", "--repo", "demo", "--title", title, "--goal", "A goal",
        ];
        args.extend_from_slice(extra);
        av(&d, scratch.path(), &args)
    };
    ok(post("Stop hook", &[]));
    let posted = ok(post("Keep agents busy", &["--after", "stop-hook"]));
    assert!(posted.contains("waiting on Stop hook"), "{posted}");
    let list = ok(av(&d, scratch.path(), &["backlog"]));
    let line = list
        .lines()
        .find(|l| l.contains("Keep agents busy"))
        .unwrap();
    assert!(line.contains("waiting on Stop hook"), "{list}");
    let line = list
        .lines()
        .find(|l| l.contains("Stop hook  ") || l.ends_with("Stop hook"))
        .unwrap();
    assert!(!line.contains("waiting on"), "{list}");
    let refused = post("Third", &["--after", "no-such-item"]);
    assert!(!refused.status.success(), "{}", out(&refused));
}

#[test]
fn post_backlog_and_drop() {
    let (d, scratch) = daemon_with_repo();
    let posted = ok(av(
        &d,
        scratch.path(),
        &[
            "post",
            "--repo",
            "demo",
            "--title",
            "draw the logo",
            "--goal",
            "A logo",
            "--needs",
            "image",
            "--priority",
            "2",
        ],
    ));
    let id = posted
        .split_whitespace()
        .find(|w| w.len() == 36 && w.matches('-').count() == 4)
        .unwrap_or_else(|| panic!("no id in {posted}"))
        .to_owned();
    let list = ok(av(&d, scratch.path(), &["backlog"]));
    assert!(list.contains("draw the logo"), "{list}");
    assert!(list.contains("queued"), "{list}");
    assert!(list.contains("needs image"), "{list}");

    ok(av(&d, scratch.path(), &["backlog", "drop", &id]));
    assert!(
        !ok(av(&d, scratch.path(), &["backlog", "--state", "queued"])).contains("draw the logo")
    );
    assert!(
        ok(av(&d, scratch.path(), &["backlog", "--state", "dropped"])).contains("draw the logo")
    );
}

#[test]
fn an_item_whose_task_was_abandoned_is_shown_and_requeued() {
    let (d, scratch) = daemon_with_repo();
    ok(av(
        &d,
        scratch.path(),
        &["agent", "set", "muse", "--can", "code", "--cost", "1"],
    ));
    let posted = ok(av(
        &d,
        scratch.path(),
        &[
            "post",
            "--repo",
            "demo",
            "--title",
            "Improve greeting",
            "--goal",
            "Make hello louder",
            "--reserve",
            "0",
        ],
    ));
    let id = posted
        .split_whitespace()
        .find(|w| w.len() == 36 && w.matches('-').count() == 4)
        .unwrap_or_else(|| panic!("no id in {posted}"))
        .to_owned();
    ok(av_as(&d, scratch.path(), "muse", &["next"]));
    let ws = d.ws.join("demo/improve-greeting");
    assert!(!ok(av(&d, scratch.path(), &["backlog"])).contains("task abandoned"));
    ok(av_as(&d, &ws, "muse", &["abandon"]));
    let list = ok(av(&d, scratch.path(), &["backlog"]));
    assert!(list.contains("taken (task abandoned)"), "{list}");

    let requeued = ok(av(&d, scratch.path(), &["backlog", "requeue", &id]));
    assert!(requeued.contains("Improve greeting"), "{requeued}");
    let queued = ok(av(&d, scratch.path(), &["backlog", "--state", "queued"]));
    assert!(queued.contains("Improve greeting"), "{queued}");
    let again = av(&d, scratch.path(), &["backlog", "requeue", &id]);
    assert!(!again.status.success(), "{}", out(&again));
}

#[test]
fn next_starts_the_task_writes_the_task_file_and_prints_the_pack() {
    let (d, scratch) = daemon_with_repo();
    ok(av(
        &d,
        scratch.path(),
        &["agent", "set", "muse", "--can", "code", "--cost", "1"],
    ));
    ok(av(
        &d,
        scratch.path(),
        &[
            "post",
            "--repo",
            "demo",
            "--title",
            "Improve greeting",
            "--goal",
            "Make hello louder",
            "--reserve",
            "0",
        ],
    ));
    let got = ok(av_as(&d, scratch.path(), "muse", &["next"]));
    let ws = d.ws.join("demo/improve-greeting");
    assert!(
        got.starts_with(&format!("workspace: {}", ws.display())),
        "{got}"
    );
    let file: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(ws.join(".av-task")).unwrap()).unwrap();
    assert_eq!(file["agent"], "muse", "{file}");
    assert_eq!(file["generation"], 1, "{file}");
    let status = ok(av_as(&d, &ws, "muse", &["status"]));
    assert!(status.contains("owner muse"), "{status}");
}

#[test]
fn a_team_limit_is_set_listed_and_reported_by_next() {
    let (d, scratch) = daemon_with_repo();
    let line = ok(av(
        &d,
        scratch.path(),
        &[
            "agent",
            "set",
            "claude",
            "--best",
            "code",
            "--max-open",
            "1",
        ],
    ));
    assert!(line.contains("max_open 1"), "{line}");
    assert!(ok(av(&d, scratch.path(), &["agent", "list"])).contains("max_open 1"));
    for title in ["First", "Second"] {
        ok(av(
            &d,
            scratch.path(),
            &[
                "post",
                "--repo",
                "demo",
                "--title",
                title,
                "--goal",
                "Make hello louder",
                "--needs",
                "code",
                "--reserve",
                "0",
            ],
        ));
    }
    ok(av_as(&d, scratch.path(), "claude", &["next"]));
    let full = ok(av_as(&d, scratch.path(), "claude", &["next"]));
    assert!(full.contains("You hold 1 open task, your limit"), "{full}");
}

#[test]
fn next_with_nothing_for_the_agent_says_so_and_exits_zero() {
    let (d, scratch) = daemon_with_repo();
    ok(av(
        &d,
        scratch.path(),
        &["agent", "set", "muse", "--can", "code", "--cost", "1"],
    ));
    ok(av(
        &d,
        scratch.path(),
        &[
            "post",
            "--repo",
            "demo",
            "--title",
            "t",
            "--goal",
            "g",
            "--needs",
            "image",
            "--reserve",
            "0",
        ],
    ));
    let got = ok(av_as(&d, scratch.path(), "muse", &["next"]));
    assert!(got.contains("No task is waiting for you"), "{got}");
    assert!(got.contains("1 queued"), "{got}");
}

#[test]
fn a_handoff_with_needs_is_taken_by_next_as_the_agent_that_fits() {
    let (d, ws, scratch) = started();
    ok(av(
        &d,
        scratch.path(),
        &[
            "agent",
            "set",
            "grok",
            "--best",
            "image",
            "--cost",
            "2",
            "--alias",
            "grok-shell-av",
        ],
    ));
    std::fs::write(
        ws.join("app/greeter.py"),
        "class Greeter:\n    def hello(self, name):\n        return name + \"!\"\n",
    )
    .unwrap();
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
            "draw the logo",
            "--needs",
            "image",
        ],
    ));
    assert!(handed.contains("handed_off"), "{handed}");

    let task = task_id_of(&ws);
    let skipped = ok(av_as(
        &d,
        scratch.path(),
        "grok",
        &["next", "--skip", &task],
    ));
    assert!(skipped.contains("No task is waiting for you"), "{skipped}");
    let got = ok(av_as(&d, scratch.path(), "grok-shell-av", &["next"]));
    assert!(got.contains("generation: 2"), "{got}");
    assert!(got.contains("Remaining: draw the logo"), "{got}");
    let status = ok(av_as(&d, &ws, "grok", &["status"]));
    assert!(status.contains("owner grok · generation 2"), "{status}");
}

#[test]
fn repo_list_names_the_imported_repos() {
    let d = start_daemon();
    let scratch = TempDir::new().unwrap();
    assert!(
        out(&av(&d, scratch.path(), &["repo", "list"])).contains("no repos"),
        "an empty daemon says so"
    );
    ok(av(
        &d,
        scratch.path(),
        &["repo", "add", "demo", d.src.to_str().unwrap()],
    ));
    let listed = ok(av(&d, scratch.path(), &["repo", "list"]));
    assert!(listed.starts_with("demo "), "{listed}");
}

#[test]
fn next_offers_another_agents_task_for_review_and_writes_no_task_file() {
    let (d, scratch) = daemon_with_repo();
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
    assert!(started.contains("improve-greeting"), "{started}");
    std::fs::write(
        ws.join("app/greeter.py"),
        "class Greeter:\n    def hello(self, name):\n        return name + \"!\"\n",
    )
    .unwrap();
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
    ok(av(&d, &ws, &["submit"]));
    let other = TempDir::new().unwrap();
    let got = ok(av_as(&d, other.path(), "grok", &["next", "--wait", "1"]));
    assert!(got.starts_with("review: task "), "{got}");
    assert!(
        got.contains("(demo/improve-greeting), owner: agent"),
        "{got}"
    );
    assert!(got.contains("Review it: review_task"), "{got}");
    assert!(!other.path().join(".av-task").exists());
}

fn image_item_queued() -> (Daemon, TempDir) {
    let (d, scratch) = daemon_with_repo();
    for (name, best) in [("claude", "code"), ("grok", "image")] {
        ok(av(
            &d,
            scratch.path(),
            &["agent", "set", name, "--best", best, "--cost", "2"],
        ));
    }
    ok(av(
        &d,
        scratch.path(),
        &[
            "post",
            "--repo",
            "demo",
            "--title",
            "Draw the logo",
            "--goal",
            "A flat logo",
            "--needs",
            "image",
            "--reserve",
            "0",
        ],
    ));
    (d, scratch)
}

#[test]
fn watch_runs_the_command_only_when_work_fits() {
    let (d, _scratch) = image_item_queued();
    let dir = TempDir::new().unwrap();
    let got = ok(av_as(
        &d,
        dir.path(),
        "claude",
        &["watch", "--once", "--", "sh", "-c", "touch ran"],
    ));
    assert!(
        !dir.path().join("ran").exists(),
        "nothing fits claude: {got}"
    );
    let got = ok(av_as(
        &d,
        dir.path(),
        "grok",
        &["watch", "--once", "--", "sh", "-c", "touch ran"],
    ));
    assert!(dir.path().join("ran").exists(), "{got}");
    assert!(got.contains("watch: backlog demo/Draw the logo"), "{got}");
    let queued = ok(av(&d, dir.path(), &["backlog", "--state", "queued"]));
    assert!(queued.contains("Draw the logo"), "{queued}");
}

#[test]
fn watch_passes_the_agent_name_and_daemon_url() {
    let (d, _scratch) = image_item_queued();
    let dir = TempDir::new().unwrap();
    ok(av_as(
        &d,
        dir.path(),
        "grok",
        &[
            "watch",
            "--once",
            "--",
            "sh",
            "-c",
            "echo $AV_AGENT $AV_URL > who",
        ],
    ));
    let who = std::fs::read_to_string(dir.path().join("who")).unwrap();
    assert_eq!(who.trim(), format!("grok {}", d.url));
}

/// A headless agent's Stop hook would wait up to `AV_HOOK_WAIT_SECS` holding its slot; `av watch` is the waiting loop, so it turns that off for its command unless its environment sets it.
#[test]
fn watch_turns_the_stop_hook_wait_off_for_its_command() {
    let (d, _scratch) = image_item_queued();
    let dir = TempDir::new().unwrap();
    let home = TempDir::new().unwrap();
    let seen = |set: Option<&str>| {
        let mut c = watch_in(
            &d,
            dir.path(),
            home.path(),
            &[
                "--once",
                "--",
                "sh",
                "-c",
                "echo \"${AV_HOOK_WAIT_SECS-unset}\" > wait",
            ],
        );
        match set {
            Some(v) => c.env("AV_HOOK_WAIT_SECS", v),
            None => c.env_remove("AV_HOOK_WAIT_SECS"),
        };
        ok(c.output().unwrap());
        std::fs::read_to_string(dir.path().join("wait"))
            .unwrap()
            .trim()
            .to_string()
    };
    assert_eq!(
        seen(None),
        "0",
        "the hook's wait is off for a watched command"
    );
    assert_eq!(
        seen(Some("30")),
        "30",
        "a wait set for av watch is passed on"
    );
}

#[test]
fn watch_stops_after_max_runs_and_backs_off_on_failure() {
    let (d, _scratch) = image_item_queued();
    let dir = TempDir::new().unwrap();
    let started = std::time::Instant::now();
    let o = av_as(
        &d,
        dir.path(),
        "grok",
        &["watch", "--every", "1", "--max-runs", "2", "--", "false"],
    );
    assert!(o.status.success(), "max-runs is a normal end: {}", err(&o));
    assert_eq!(err(&o).matches("failed").count(), 2, "{}", err(&o));
    // after the first failure it waited twice the interval
    assert!(started.elapsed() >= std::time::Duration::from_secs(2));
}

/// `av watch` as `agent` with its own `$AV_HOME`, so the host's agent slots are the test's.
fn watch_in(d: &Daemon, cwd: &Path, home: &Path, args: &[&str]) -> Command {
    let mut c = Command::new(env!("CARGO_BIN_EXE_av"));
    c.current_dir(cwd)
        .env("AV_URL", &d.url)
        .env("AV_HOME", home)
        .env_remove("AV_MAX_AGENTS")
        .env("AV_AGENT", "grok")
        .arg("watch")
        .args(args);
    c
}

#[test]
fn watch_starts_no_more_agents_than_the_host_has_slots() {
    let (d, _scratch) = image_item_queued();
    let dir = TempDir::new().unwrap();
    let home = TempDir::new().unwrap();
    let mut first = watch_in(
        &d,
        dir.path(),
        home.path(),
        &[
            "--once",
            "--max-agents",
            "1",
            "--",
            "sh",
            "-c",
            "touch first; sleep 4",
        ],
    )
    .stdout(std::process::Stdio::piped())
    .stderr(std::process::Stdio::piped())
    .spawn()
    .unwrap();
    let t = std::time::Instant::now();
    while !dir.path().join("first").exists() {
        assert!(t.elapsed() < HANG, "hung: first never ran");
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    let o = watch_in(
        &d,
        dir.path(),
        home.path(),
        &[
            "--once",
            "--max-agents",
            "1",
            "--",
            "sh",
            "-c",
            "touch second",
        ],
    )
    .output()
    .unwrap();
    assert!(o.status.success(), "{}", err(&o));
    assert!(!dir.path().join("second").exists(), "{}", out(&o));
    assert!(
        out(&o).contains("all 1 agent slots on this host are in use"),
        "{}",
        out(&o)
    );
    let o = watch_in(
        &d,
        dir.path(),
        home.path(),
        &["--once", "--", "sh", "-c", "touch second"],
    )
    .env("AV_MAX_AGENTS", "1")
    .output()
    .unwrap();
    assert!(!dir.path().join("second").exists(), "{}", out(&o));
    assert!(first.wait().unwrap().success());
    ok(watch_in(
        &d,
        dir.path(),
        home.path(),
        &[
            "--once",
            "--max-agents",
            "1",
            "--",
            "sh",
            "-c",
            "touch second",
        ],
    )
    .output()
    .unwrap());
    assert!(dir.path().join("second").exists());
}

#[test]
fn an_interrupted_watch_ends_its_command_too() {
    use std::io::BufRead;
    use std::os::unix::process::ExitStatusExt;
    let (d, _scratch) = image_item_queued();
    let cases = [
        (true, "INT"),
        (true, "TERM"),
        (false, "INT"),
        (false, "TERM"),
        (false, "HUP"),
        (false, "QUIT"),
    ];
    for (once, signal) in cases {
        let dir = TempDir::new().unwrap();
        let home = TempDir::new().unwrap();
        // the command and a child of its own, both waiting on a pipe nothing writes to
        let script = "mkfifo hold; cat hold & echo \"agent-started $$ $!\"; wait";
        let mut args = vec![];
        if once {
            args.push("--once");
        }
        args.extend(["--", "sh", "-c", script]);
        let mut child = watch_in(&d, dir.path(), home.path(), &args)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn()
            .unwrap();
        let mut lines = std::io::BufReader::new(child.stdout.take().unwrap()).lines();
        let pids: Vec<String> = loop {
            let line = lines
                .next()
                .expect("av ended before its command started")
                .unwrap();
            if let Some(rest) = line.strip_prefix("agent-started ") {
                break rest.split_whitespace().map(str::to_owned).collect();
            }
        };
        assert_eq!(pids.len(), 2, "{pids:?}");
        let sent = Command::new("kill")
            .args([&format!("-{signal}"), &child.id().to_string()])
            .status()
            .unwrap();
        assert!(sent.success());
        let status = child.wait().unwrap();
        let deadline = std::time::Instant::now() + HANG;
        let left = loop {
            let left: Vec<&String> = pids.iter().filter(|p| alive(p)).collect();
            if left.is_empty() || std::time::Instant::now() > deadline {
                break left;
            }
            std::thread::sleep(std::time::Duration::from_millis(20));
        };
        for p in &left {
            let _ = Command::new("kill").args(["-KILL", p]).status();
        }
        assert!(
            left.is_empty(),
            "once={once} {signal}: the command outlived av: {left:?}"
        );
        let number = match signal {
            "HUP" => 1,
            "INT" => 2,
            "QUIT" => 3,
            _ => 15,
        };
        assert_eq!(
            status.signal(),
            Some(number),
            "once={once} {signal}: {status}"
        );
    }
}

#[test]
fn a_waiting_watch_starts_its_agent_when_a_slot_frees() {
    let (d, _scratch) = image_item_queued();
    let dir = TempDir::new().unwrap();
    let home = TempDir::new().unwrap();
    let mut first = watch_in(
        &d,
        dir.path(),
        home.path(),
        &[
            "--once",
            "--max-agents",
            "1",
            "--",
            "sh",
            "-c",
            "touch first; sleep 3; touch first-done",
        ],
    )
    .spawn()
    .unwrap();
    let t = std::time::Instant::now();
    while !dir.path().join("first").exists() {
        assert!(t.elapsed() < HANG, "hung: first never ran");
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    let o = watch_in(
        &d,
        dir.path(),
        home.path(),
        &[
            "--every",
            "1",
            "--max-runs",
            "1",
            "--max-agents",
            "1",
            "--",
            "sh",
            "-c",
            "test -f first-done && touch second",
        ],
    )
    .output()
    .unwrap();
    assert!(o.status.success(), "{}", err(&o));
    assert!(dir.path().join("second").exists(), "{}", out(&o));
    assert!(first.wait().unwrap().success());
}

fn hold_build_slot(home: &Path) -> std::fs::File {
    std::fs::create_dir_all(home.join("builds")).unwrap();
    let f = std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(home.join("builds/slot-0.lock"))
        .unwrap();
    f.lock().unwrap();
    f
}

#[test]
fn validate_waits_for_a_free_build_slot_on_the_host() {
    let (d, ws, _scratch) = started();
    let home = TempDir::new().unwrap();
    for (flag, var) in [(Some("--max-builds"), None), (None, Some("1"))] {
        let held = hold_build_slot(home.path());
        let _ = std::fs::remove_file(ws.join("built"));
        let mut c = Command::new(env!("CARGO_BIN_EXE_av"));
        c.current_dir(&ws)
            .env("AV_URL", &d.url)
            .env("AV_HOME", home.path())
            .env_remove("AV_TASK")
            .env_remove("AV_SESSION")
            .env_remove("AV_GENERATION")
            .env_remove("AV_AGENT")
            .env_remove("AV_TOKEN")
            .env_remove("AV_MAX_BUILDS")
            .arg("validate");
        if let Some(flag) = flag {
            c.args([flag, "1"]);
        }
        if let Some(n) = var {
            c.env("AV_MAX_BUILDS", n);
        }
        let mut child = c
            .args(["--", "sh", "-c", "touch built"])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1500));
        assert!(
            !ws.join("built").exists(),
            "the only build slot is held: the command must wait"
        );
        assert!(
            child.try_wait().unwrap().is_none(),
            "validate gave up instead of waiting"
        );
        drop(held);
        let o = child.wait_with_output().unwrap();
        assert!(o.status.success(), "{}", err(&o));
        assert!(ws.join("built").exists(), "{}", out(&o));
        assert!(
            err(&o).contains("all 1 build slots on this host are in use; waiting"),
            "{}",
            err(&o)
        );
    }
}

#[test]
fn targets_are_added_listed_closed_and_counted_in_the_backlog() {
    let (d, scratch) = daemon_with_repo();
    let dir = scratch.path();
    let added = ok(av(
        &d,
        dir,
        &[
            "target",
            "add",
            "--repo",
            "demo",
            "--title",
            "Ship v1",
            "--due",
            "2026-12-01",
            "--detail",
            "the first release",
        ],
    ));
    assert!(added.contains("Ship v1"), "{added}");
    assert!(added.contains("ship-v1"), "{added}");

    ok(av(
        &d,
        dir,
        &[
            "post",
            "--repo",
            "demo",
            "--title",
            "Louder hello",
            "--goal",
            "g",
            "--target",
            "ship-v1",
        ],
    ));
    let posted = ok(av(
        &d,
        dir,
        &["post", "--repo", "demo", "--title", "Second", "--goal", "g"],
    ));
    let id = posted
        .split_whitespace()
        .find(|w| w.len() == 36 && w.matches('-').count() == 4)
        .unwrap()
        .to_owned();
    let linked = ok(av(&d, dir, &["backlog", "target", &id, "ship-v1"]));
    assert!(linked.contains("Ship v1"), "{linked}");

    let list = ok(av(&d, dir, &["backlog"]));
    assert_eq!(
        list.lines().filter(|l| l.contains("for Ship v1")).count(),
        2,
        "{list}"
    );

    let targets = ok(av(&d, dir, &["target", "list", "--repo", "demo"]));
    assert!(targets.contains("Ship v1"), "{targets}");
    assert!(targets.contains("0/2 merged"), "{targets}");
    assert!(targets.contains("due 2026-12-01"), "{targets}");

    ok(av(&d, dir, &["backlog", "target", &id]));
    let targets = ok(av(&d, dir, &["target", "list", "--repo", "demo"]));
    assert!(targets.contains("0/1 merged"), "{targets}");
    ok(av(
        &d,
        dir,
        &["target", "done", "ship-v1", "--repo", "demo"],
    ));
    let active = ok(av(&d, dir, &["target", "list", "--repo", "demo"]));
    assert!(!active.contains("Ship v1"), "{active}");
    let all = ok(av(&d, dir, &["target", "list", "--repo", "demo", "--all"]));
    assert!(all.contains("done"), "{all}");

    let refused = av(
        &d,
        dir,
        &[
            "post", "--repo", "demo", "--title", "Third", "--goal", "g", "--target", "ship-v1",
        ],
    );
    assert!(!refused.status.success(), "{}", out(&refused));
    ok(av(
        &d,
        dir,
        &["target", "reopen", "ship-v1", "--repo", "demo"],
    ));
    ok(av(
        &d,
        dir,
        &[
            "post", "--repo", "demo", "--title", "Third", "--goal", "g", "--target", "ship-v1",
        ],
    ));
}

struct Canned {
    url: String,
    seen: std::sync::Arc<std::sync::Mutex<Vec<String>>>,
}

fn canned(body: &'static str) -> Canned {
    use std::io::{BufRead, BufReader, Read, Write};
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let seen = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let log = seen.clone();
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let Ok(mut stream) = stream else { continue };
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut first = String::new();
            reader.read_line(&mut first).unwrap();
            let mut len = 0;
            loop {
                let mut line = String::new();
                reader.read_line(&mut line).unwrap();
                if line.trim().is_empty() {
                    break;
                }
                if let Some((k, v)) = line.split_once(':')
                    && k.eq_ignore_ascii_case("content-length")
                {
                    len = v.trim().parse().unwrap();
                }
            }
            let mut sent = vec![0; len];
            reader.read_exact(&mut sent).unwrap();
            let parts: Vec<&str> = first.split_whitespace().take(2).collect();
            log.lock().unwrap().push(format!(
                "{} {}",
                parts.join(" "),
                String::from_utf8_lossy(&sent)
            ));
            let reply = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            let _ = stream.write_all(reply.as_bytes());
        }
    });
    Canned { url, seen }
}

const DISK: &str = r#"{"ws_root": "/home/u/ws", "store_root": "/home/u/.margincode/store",
 "pg_dir": "/home/u/.margincode/pg", "retain_secs": 3600,
 "repos": [{"name": "demo", "store_bytes": 31457280,
   "build_cache": {"enabled": true, "config": "/home/u/ws/demo/.cargo/config.toml"},
   "build_cache_bytes": 1288490188, "scratch_bytes": 40960,
   "workspaces": [
     {"slug": "improve-greeting", "status": "merged", "bytes": 12582912, "build_bytes": 0, "age_secs": 7200, "reclaimable": true},
     {"slug": "fix-thing", "status": "open", "bytes": 7516192768, "build_bytes": 7193231360, "age_secs": 300, "reclaimable": false}]}],
 "outside": [{"session": "s1", "agent": "claude", "task": "0b6a", "slug": "fix-thing",
   "added": [{"path": "/home/u/.cache/huge", "bytes": 943718400},
             {"path": "/tmp/x/target", "bytes": 209715200}],
   "partial": false, "total_bytes": 1153433600, "at": "2026-11-02T10:15:00Z"}],
 "totals": {"workspaces_bytes": 7528775680, "build_bytes": 7193231360, "store_bytes": 31457280,
   "build_cache_bytes": 1288490188, "scratch_bytes": 40960}}"#;

#[test]
fn disk_shows_workspaces_caches_and_what_sessions_left_outside() {
    let c = canned(DISK);
    let dir = TempDir::new().unwrap();
    let o = ok(av_with(&c.url, dir.path(), &["disk"]));
    for want in [
        "demo",
        "improve-greeting",
        "merged",
        "12 MB",
        "reclaimable",
        "fix-thing",
        "BUILD",
        "6.7 GB",
        "build output 6.7 GB",
        "1.2 GB",
        "/home/u/ws/demo/.cargo/config.toml",
        "claude",
        "/home/u/.cache/huge",
        "900 MB",
        "/home/u/.margincode/pg",
    ] {
        assert!(o.contains(want), "missing {want:?} in\n{o}");
    }
    assert!(
        !o.lines()
            .any(|l| l.contains("fix-thing") && l.contains("reclaimable")),
        "{o}"
    );
    assert_eq!(c.seen.lock().unwrap()[0].trim(), "GET /v1/disk");

    let o = ok(av_with(&c.url, dir.path(), &["disk", "--json"]));
    let v: serde_json::Value = serde_json::from_str(&o).unwrap();
    assert_eq!(v, serde_json::from_str::<serde_json::Value>(DISK).unwrap());
}

const DISK_FS: &str = r#"{"fs_free_bytes": 53687091200, "fs_total_bytes": 214748364800,
 "min_free_bytes": 21474836480, "below_floor": false, "build_budget_bytes": 0}"#;

#[test]
fn disk_fs_prints_only_the_filesystem_line() {
    let c = canned(DISK_FS);
    let dir = TempDir::new().unwrap();
    let o = ok(av_with(&c.url, dir.path(), &["disk", "--fs"]));
    assert_eq!(
        o,
        "disk: 50.0 GB free of 200.0 GB, floor 20.0 GB (AV_DISK_MIN_FREE_GB)\n"
    );
    assert_eq!(c.seen.lock().unwrap()[0].trim(), "GET /v1/disk?fs=1");
    let o = ok(av_with(&c.url, dir.path(), &["disk", "--fs", "--json"]));
    let v: serde_json::Value = serde_json::from_str(&o).unwrap();
    assert_eq!(
        v,
        serde_json::from_str::<serde_json::Value>(DISK_FS).unwrap()
    );
    assert_eq!(c.seen.lock().unwrap()[1].trim(), "GET /v1/disk?fs=1");
}

const GC: &str = r#"{"reclaimed": [{"repo": "demo", "slug": "improve-greeting", "status": "merged",
   "path": "/home/u/ws/demo/improve-greeting", "bytes": 12582912, "build_bytes": 10485760}],
 "freed_bytes": 1301073100,
 "kept": [{"repo": "demo", "slug": "just-merged", "status": "merged",
   "path": "/home/u/ws/demo/just-merged", "bytes": 1024, "age_secs": 60}],
 "build_cache_bytes": 1288490188, "dry_run": true}"#;

#[test]
fn gc_dry_run_says_what_it_would_remove() {
    let c = canned(GC);
    let dir = TempDir::new().unwrap();
    let o = ok(av_with(
        &c.url,
        dir.path(),
        &["gc", "--dry-run", "--retain", "0", "--build-cache"],
    ));
    for want in [
        "would remove",
        "demo/improve-greeting",
        "12 MB (10 MB of it build output)",
        "build cache",
        "1.2 GB",
        "kept 1 (younger than retention)",
    ] {
        assert!(o.contains(want), "missing {want:?} in\n{o}");
    }
    let sent = c.seen.lock().unwrap()[0].clone();
    assert!(sent.starts_with("POST /v1/gc "), "{sent}");
    let body: serde_json::Value =
        serde_json::from_str(sent.split_once(' ').unwrap().1.split_once(' ').unwrap().1).unwrap();
    assert_eq!(body["dry_run"], true, "{body}");
    assert_eq!(body["retain_secs"], 0, "{body}");
    assert_eq!(body["build_cache"], true, "{body}");

    let o = ok(av_with(&c.url, dir.path(), &["gc", "--json"]));
    let v: serde_json::Value = serde_json::from_str(&o).unwrap();
    assert_eq!(v["freed_bytes"], 1301073100u64);
    let sent = c.seen.lock().unwrap()[1].clone();
    assert!(sent.contains("\"dry_run\":false"), "{sent}");
    assert!(!sent.contains("retain_secs"), "{sent}");
}

fn canned_daemon(status_line: &'static str, body: &'static str, version: &'static str) -> String {
    use std::io::{Read, Write};
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let Ok(mut s) = stream else { break };
            let mut buf = [0u8; 4096];
            let _ = s.read(&mut buf);
            let _ = write!(
                s,
                "HTTP/1.1 {status_line}\r\nx-av-version: {version}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
                body.len()
            );
        }
    });
    url
}

#[test]
fn a_daemon_of_another_version_is_warned_about_once() {
    let url = canned_daemon("200 OK", "[]", "0.0.1");
    let dir = TempDir::new().unwrap();
    let o = av_with(&url, dir.path(), &["repo", "list"]);
    assert!(o.status.success(), "{}", err(&o));
    let e = err(&o);
    assert_eq!(e.matches("warning: the daemon").count(), 1, "{e}");
    assert!(
        e.contains("0.0.1") && e.contains(env!("CARGO_PKG_VERSION")),
        "{e}"
    );
}

#[test]
fn a_daemon_of_the_same_version_is_not_warned_about() {
    let url = canned_daemon("200 OK", "[]", env!("CARGO_PKG_VERSION"));
    let dir = TempDir::new().unwrap();
    let o = av_with(&url, dir.path(), &["repo", "list"]);
    assert!(o.status.success(), "{}", err(&o));
    assert!(!err(&o).contains("warning"), "{}", err(&o));
}

#[test]
fn a_bare_404_says_the_daemon_is_probably_older() {
    let url = canned_daemon("404 Not Found", "", "0.0.1");
    let dir = TempDir::new().unwrap();
    let o = av_with(&url, dir.path(), &["repo", "list"]);
    assert!(!o.status.success());
    let e = err(&o);
    assert!(e.contains("probably older than this av"), "{e}");
    assert!(
        e.contains("0.0.1") && e.contains(env!("CARGO_PKG_VERSION")),
        "{e}"
    );
}

#[test]
fn a_plain_text_rejection_is_printed_not_request_failed() {
    // axum answers a path it cannot parse in plain text, not the daemon's JSON `error`
    let d = start_daemon();
    let scratch = TempDir::new().unwrap();
    let o = av(&d, scratch.path(), &["backlog", "accept", "not-a-uuid"]);
    assert!(!o.status.success());
    let e = err(&o);
    assert!(
        e.contains("not-a-uuid") && e.contains("HTTP 400") && !e.contains("request failed"),
        "{e}"
    );
}

#[test]
fn a_404_the_daemon_explains_is_left_alone() {
    let url = canned_daemon(
        "404 Not Found",
        r#"{"error":"no such task"}"#,
        env!("CARGO_PKG_VERSION"),
    );
    let dir = TempDir::new().unwrap();
    let o = av_with(&url, dir.path(), &["repo", "list"]);
    assert!(!o.status.success());
    let e = err(&o);
    assert!(e.contains("no such task") && !e.contains("older"), "{e}");
}

#[test]
fn repo_add_mines_history_and_why_names_the_mined_commit() {
    let d = start_daemon();
    let scratch = TempDir::new().unwrap();
    std::fs::write(
        d.src.join("app/greeter.py"),
        "class Greeter:\n    def hello(self, name):\n        return f'hello {name}'\n",
    )
    .unwrap();
    git(
        &d.src,
        &[
            "commit",
            "-q",
            "-am",
            "Greet by name so the log reads as a sentence",
        ],
    );
    let o = ok(av(
        &d,
        scratch.path(),
        &[
            "repo",
            "add",
            "demo",
            d.src.to_str().unwrap(),
            "--mine-history",
            "10",
        ],
    ));
    assert!(
        o.contains("mined 1 records from git history (demo/git-history)"),
        "the root commit is not mined: {o}"
    );
    let o = ok(av(&d, scratch.path(), &["repo", "mine", "demo"]));
    assert!(
        o.contains("mined 0 records") && o.contains("1 mined before"),
        "{o}"
    );
    let o = ok(av(
        &d,
        scratch.path(),
        &["why", "app/greeter.py:3", "--repo", "demo"],
    ));
    assert!(
        o.contains("imported history")
            && o.contains("note from git history: Greet by name so the log reads as a sentence"),
        "{o}"
    );
}

fn notes_fixture() -> (Daemon, PathBuf, TempDir, String) {
    let (d, ws, scratch) = started();
    let other = ok(av_as(
        &d,
        scratch.path(),
        "codex",
        &[
            "task",
            "start",
            "--repo",
            "demo",
            "--title",
            "Earlier",
            "--goal",
            "earlier work",
        ],
    ));
    let other_ws = PathBuf::from(
        other
            .lines()
            .next()
            .unwrap()
            .trim_start_matches("workspace: "),
    );
    ok(av_as(
        &d,
        &other_ws,
        "codex",
        &[
            "record",
            "--kind",
            "note",
            "--summary",
            "hello must stay pure: the CLI caches it",
            "--anchor",
            "app/greeter.py",
        ],
    ));
    std::fs::write(
        ws.join("app/greeter.py"),
        "class Greeter:\n    def hello(self, name):\n        return name.upper()\n",
    )
    .unwrap();
    ok(av(&d, &ws, &["checkpoint"]));
    ok(av(
        &d,
        &ws,
        &[
            "record",
            "--kind",
            "decision",
            "--summary",
            "upper() in hello, no flag",
            "--anchor",
            "app/greeter.py:Greeter.hello",
        ],
    ));
    let file: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(ws.join(".av-task")).unwrap()).unwrap();
    let id = file["task"].as_str().unwrap().to_owned();
    (d, ws, scratch, id)
}

#[test]
fn notes_renders_the_task_records_and_what_others_recorded_about_its_files() {
    let (d, ws, scratch, id) = notes_fixture();
    let md = ok(av(&d, &ws, &["notes"]));
    assert!(
        md.starts_with(&format!("<!-- margincode:notes v1 task={id} -->")),
        "{md}"
    );
    assert!(
        md.trim_end().ends_with("<!-- /margincode:notes -->"),
        "{md}"
    );
    assert!(
        md.contains("**Decisions**") && md.contains("upper() in hello, no flag"),
        "{md}"
    );
    assert!(
        md.contains("`app/greeter.py`") && md.contains("hello must stay pure"),
        "the other task's note on the changed file: {md}"
    );
    let by_slug = ok(av(&d, scratch.path(), &["notes", "improve-greeting"]));
    assert_eq!(by_slug, md);
    let by_repo_slug = ok(av(&d, scratch.path(), &["notes", "demo/improve-greeting"]));
    assert_eq!(by_repo_slug, md);
    let j: serde_json::Value = serde_json::from_str(&ok(av(
        &d,
        scratch.path(),
        &["notes", &id, "--format", "json"],
    )))
    .unwrap();
    assert_eq!(j["task"], id.as_str());
    assert_eq!(j["markdown"], md.as_str());
    assert_eq!(j["files"], serde_json::json!(["app/greeter.py"]));
    // reading notes sends nothing to any session: the pre-read hook still has the note to tell
    let input = serde_json::json!({"tool_input": {"file_path": ws.join("app/greeter.py")}});
    let o = av_stdin(&d, &ws, &["hook", "pre-read"], &input.to_string());
    assert!(out(&o).contains("hello must stay pure"), "{}", out(&o));
}

#[test]
fn notes_exit_codes() {
    let (d, ws, scratch, _) = notes_fixture();
    let o = av(
        &d,
        scratch.path(),
        &["notes", "00000000-0000-4000-8000-000000000000"],
    );
    assert_eq!(o.status.code(), Some(1), "{}", err(&o));
    let o = av(&d, scratch.path(), &["notes", "no-such-task"]);
    assert_eq!(o.status.code(), Some(1));
    assert!(err(&o).contains("no task \"no-such-task\""), "{}", err(&o));
    let o = av(&d, scratch.path(), &["notes"]);
    assert_eq!(o.status.code(), Some(1));
    assert!(err(&o).contains("no task"), "{}", err(&o));
    let o = av_with("http://127.0.0.1:1", &ws, &["notes"]);
    assert_eq!(o.status.code(), Some(1));
    assert!(err(&o).contains("cannot reach"), "{}", err(&o));
    let o = av(&d, &ws, &["notes", "--format", "yaml"]);
    assert_eq!(o.status.code(), Some(2));
    let o = av(&d, &ws, &["notes", "--gh-repo", "o/r"]);
    assert_eq!(o.status.code(), Some(2), "--gh-repo needs --pr");
}

fn fake_gh(dir: &Path) -> PathBuf {
    let script = dir.join("gh");
    std::fs::write(
        &script,
        concat!(
            "#!/bin/sh\n",
            "d=\"$(dirname \"$0\")\"\n",
            "n=$(cat \"$d/calls\" 2>/dev/null | wc -l)\n",
            "echo \"$*\" >> \"$d/calls\"\n",
            "if [ -e \"$d/fail\" ]; then echo 'HTTP 401: Bad credentials' >&2; exit 1; fi\n",
            "case \"$*\" in\n",
            "  'pr view'*) echo 42 ;;\n",
            "  *--paginate*) if [ -e \"$d/comments\" ]; then cat \"$d/comments\"; else echo '[]'; fi ;;\n",
            "  *) cat > \"$d/stdin-$n\"; echo '{}' ;;\n",
            "esac\n",
        ),
    )
    .unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).unwrap();
    }
    script
}

fn av_gh(d: &Daemon, cwd: &Path, gh: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_av"))
        .current_dir(cwd)
        .env("AV_URL", &d.url)
        .env_remove("AV_TASK")
        .env_remove("AV_SESSION")
        .env_remove("AV_AGENT")
        .env_remove("AV_TOKEN")
        .env("AV_GH", gh)
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn notes_pr_posts_once_then_updates_the_same_comment_in_place() {
    let (d, ws, _scratch, id) = notes_fixture();
    let ghd = TempDir::new().unwrap();
    let gh = fake_gh(ghd.path());
    let calls = || std::fs::read_to_string(ghd.path().join("calls")).unwrap_or_default();
    let stdin_of = |n: usize| -> String {
        let sent: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(ghd.path().join(format!("stdin-{n}"))).unwrap(),
        )
        .unwrap();
        sent["body"].as_str().unwrap().to_owned()
    };

    let o = ok(av_gh(
        &d,
        &ws,
        &gh,
        &["notes", "--pr", "7", "--gh-repo", "o/r"],
    ));
    assert!(o.contains("notes posted on pull request #7"), "{o}");
    let log = calls();
    assert!(
        log.contains("api --paginate repos/{owner}/{repo}/issues/7/comments")
            && log.contains("api -X POST repos/{owner}/{repo}/issues/7/comments --input -"),
        "{log}"
    );
    let body = stdin_of(1);
    assert!(body.contains(&format!("task={id}")), "{body}");

    // GitHub now has that comment, after a human's, with CRLF line ends
    let comments = serde_json::json!([
        {"id": 11, "body": "LGTM"},
        {"id": 55, "body": body.replace('\n', "\r\n")},
    ]);
    std::fs::write(ghd.path().join("comments"), comments.to_string()).unwrap();
    let o = ok(av_gh(&d, &ws, &gh, &["notes", "--pr", "7"]));
    assert!(o.contains("up to date (comment 55)"), "{o}");
    assert_eq!(calls().lines().count(), 3, "only the listing: {}", calls());

    ok(av(
        &d,
        &ws,
        &[
            "record",
            "--kind",
            "attempt",
            "--summary",
            "tried a --loud flag",
        ],
    ));
    let o = ok(av_gh(&d, &ws, &gh, &["notes", "--pr"]));
    assert!(
        o.contains("notes updated on pull request #42 (comment 55)"),
        "{o}"
    );
    let log = calls();
    assert!(
        log.contains("pr view --json number --jq .number")
            && log.contains("api -X PATCH repos/{owner}/{repo}/issues/comments/55 --input -"),
        "{log}"
    );
    let updated = stdin_of(log.lines().count() - 1);
    assert!(updated.contains("tried a --loud flag"), "{updated}");
    assert_eq!(updated.matches(&format!("task={id}")).count(), 1);

    std::fs::write(ghd.path().join("fail"), "").unwrap();
    let o = av_gh(&d, &ws, &gh, &["notes", "--pr", "7"]);
    assert_eq!(o.status.code(), Some(1));
    assert!(err(&o).contains("Bad credentials"), "{}", err(&o));
}

// A second directory stands in for the other host: it shares only the HTTP API with the daemon's workspace.

fn task_of(ws: &Path) -> String {
    let file = std::fs::read_to_string(ws.join(".av-task")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&file).unwrap();
    json["task"].as_str().unwrap().to_owned()
}

fn cloned() -> (Daemon, PathBuf, TempDir, PathBuf) {
    let (d, ws, _scratch) = started();
    let task = task_of(&ws);
    let host = TempDir::new().unwrap();
    let dir = host.path().join("co");
    let o = ok(av(
        &d,
        host.path(),
        &["remote", "clone", &task, dir.to_str().unwrap()],
    ));
    assert!(o.contains("checkout:") && o.contains(&task), "{o}");
    (d, ws, host, dir)
}

#[test]
fn a_checkout_on_another_host_checkpoints_through_a_patch() {
    let (d, ws, _host, dir) = cloned();
    assert_eq!(
        std::fs::read_to_string(dir.join("app/greeter.py")).unwrap(),
        std::fs::read_to_string(ws.join("app/greeter.py")).unwrap()
    );
    assert!(dir.join("README.md").exists());
    let tf = std::fs::read_to_string(dir.join(".av-task")).unwrap();
    assert!(tf.contains("\"remote\""), "{tf}");

    let louder = "class Greeter:\n    def hello(self, name):\n        return name.upper()\n";
    std::fs::write(dir.join("app/greeter.py"), louder).unwrap();
    std::fs::write(dir.join("app/wave.py"), "def wave():\n    return 1\n").unwrap();
    std::fs::remove_file(dir.join("README.md")).unwrap();
    let img: Vec<u8> = (0..2000u32)
        .map(|i| (i * 7 % 256) as u8)
        .chain([0])
        .collect();
    std::fs::write(dir.join("logo.bin"), &img).unwrap();
    std::fs::create_dir_all(dir.join("target/debug")).unwrap();
    std::fs::write(dir.join("target/debug/big"), [0u8; 64]).unwrap();

    let cp = ok(av(&d, &dir, &["checkpoint"]));
    assert!(
        cp.contains("changed (4): README.md, app/greeter.py, app/wave.py, logo.bin"),
        "{cp}"
    );
    assert_eq!(
        std::fs::read_to_string(ws.join("app/greeter.py")).unwrap(),
        louder
    );
    assert!(!ws.join("README.md").exists());
    assert_eq!(std::fs::read(ws.join("logo.bin")).unwrap(), img);
    assert!(!ws.join("target").exists());
    let shown = ok(av(
        &d,
        &dir,
        &["show", "app/wave.py", "--at", "task:improve-greeting"],
    ));
    assert!(shown.contains("def wave"), "{shown}");
    let st = ok(av(&d, &dir, &["status"]));
    assert!(st.contains("app/wave.py"), "{st}");

    std::fs::write(dir.join("app/wave.py"), "def wave():\n    return 2\n").unwrap();
    let cp = ok(av(&d, &dir, &["checkpoint"]));
    assert!(cp.contains("app/wave.py"), "{cp}");
    assert_eq!(
        std::fs::read_to_string(ws.join("app/wave.py")).unwrap(),
        "def wave():\n    return 2\n"
    );

    std::fs::write(ws.join("app/other.py"), "X = 1\n").unwrap();
    ok(av(&d, &ws, &["checkpoint"]));
    std::fs::write(dir.join("app/wave.py"), "def wave():\n    return 3\n").unwrap();
    let cp = ok(av(&d, &dir, &["checkpoint"]));
    assert!(cp.contains("app/wave.py"), "{cp}");
    assert_eq!(
        std::fs::read_to_string(dir.join("app/other.py")).unwrap(),
        "X = 1\n",
        "pulled"
    );
    assert_eq!(
        std::fs::read_to_string(ws.join("app/wave.py")).unwrap(),
        "def wave():\n    return 3\n"
    );
    let p = ok(av(&d, &dir, &["remote", "pull"]));
    assert!(p.contains("up to date"), "{p}");
}

#[test]
fn a_pull_that_meets_a_local_edit_of_the_same_lines_leaves_markers() {
    let (d, ws, _host, dir) = cloned();
    std::fs::write(
        ws.join("app/greeter.py"),
        "class Greeter:\n    def hello(self, name):\n        return name.upper()\n",
    )
    .unwrap();
    ok(av(&d, &ws, &["checkpoint"]));
    std::fs::write(
        dir.join("app/greeter.py"),
        "class Greeter:\n    def hello(self, name):\n        return name + '!'\n",
    )
    .unwrap();
    let o = av(&d, &dir, &["checkpoint"]);
    assert!(!o.status.success(), "{}", out(&o));
    assert!(err(&o).contains("app/greeter.py"), "{}", err(&o));
    let text = std::fs::read_to_string(dir.join("app/greeter.py")).unwrap();
    assert!(
        text.contains("<<<<<<<") && text.contains("name.upper()") && text.contains("'!'"),
        "{text}"
    );
    let both = "class Greeter:\n    def hello(self, name):\n        return name.upper() + '!'\n";
    std::fs::write(dir.join("app/greeter.py"), both).unwrap();
    ok(av(&d, &dir, &["checkpoint"]));
    assert_eq!(
        std::fs::read_to_string(ws.join("app/greeter.py")).unwrap(),
        both
    );
}

#[test]
fn submit_from_a_checkout_sends_its_edits_first() {
    let (d, ws, _host, dir) = cloned();
    std::fs::write(dir.join("app/wave.py"), "def wave():\n    return 1\n").unwrap();
    ok(av(
        &d,
        &dir,
        &["submit", "--record", "decision: add wave @ app/wave.py"],
    ));
    assert!(ws.join("app/wave.py").exists());
}

#[test]
fn task_start_with_checkout_makes_the_checkout_instead_of_using_the_workspace() {
    let d = start_daemon();
    let host = TempDir::new().unwrap();
    ok(av(
        &d,
        host.path(),
        &["repo", "add", "demo", d.src.to_str().unwrap()],
    ));
    let dir = host.path().join("mine");
    let o = ok(av(
        &d,
        host.path(),
        &[
            "task",
            "start",
            "--repo",
            "demo",
            "--title",
            "Far away",
            "--goal",
            "g",
            "--checkout",
            "mine",
        ],
    ));
    assert!(o.contains("## Task"), "{o}");
    assert!(dir.join("app/greeter.py").exists());
    std::fs::write(dir.join("app/far.py"), "F = 1\n").unwrap();
    ok(av(&d, &dir, &["checkpoint"]));
    assert!(d.ws.join("demo/far-away/app/far.py").exists());
}

fn posted_for_muse() -> (Daemon, TempDir) {
    let (d, scratch) = daemon_with_repo();
    ok(av(
        &d,
        scratch.path(),
        &["agent", "set", "muse", "--can", "code", "--cost", "1"],
    ));
    ok(av(
        &d,
        scratch.path(),
        &[
            "post",
            "--repo",
            "demo",
            "--title",
            "Improve greeting",
            "--goal",
            "Make hello louder",
            "--reserve",
            "0",
        ],
    ));
    (d, scratch)
}

#[test]
fn next_with_checkout_makes_the_checkout_instead_of_using_the_workspace() {
    let (d, scratch) = posted_for_muse();
    let got = ok(av_as(
        &d,
        scratch.path(),
        "muse",
        &["next", "--checkout", "mine"],
    ));
    let dir = scratch.path().join("mine");
    assert!(got.starts_with("checkout: mine"), "{got}");
    assert!(got.contains("## Task"), "{got}");
    assert!(dir.join("app/greeter.py").exists());
    let file: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(dir.join(".av-task")).unwrap()).unwrap();
    assert_eq!(file["agent"], "muse", "{file}");
    assert_eq!(file["generation"], 1, "{file}");
    assert!(file["remote"].is_object(), "{file}");
    std::fs::write(dir.join("app/far.py"), "F = 1\n").unwrap();
    ok(av_as(&d, &dir, "muse", &["checkpoint"]));
    assert!(d.ws.join("demo/improve-greeting/app/far.py").exists());
}

#[test]
fn a_claim_with_checkout_works_under_the_claims_generation() {
    let (d, ws, scratch) = started();
    ok(av(
        &d,
        scratch.path(),
        &["agent", "set", "grok", "--best", "image", "--cost", "2"],
    ));
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
            "app/greeter.py",
        ],
    ));
    ok(av(
        &d,
        &ws,
        &[
            "handoff",
            "--done",
            "nothing yet",
            "--remaining",
            "draw the logo",
            "--needs",
            "image",
        ],
    ));
    let got = ok(av_as(
        &d,
        scratch.path(),
        "grok",
        &["next", "--checkout", "logo"],
    ));
    assert!(got.contains("generation: 2"), "{got}");
    assert!(got.contains("Remaining: draw the logo"), "{got}");
    let dir = scratch.path().join("logo");
    let file: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(dir.join(".av-task")).unwrap()).unwrap();
    assert_eq!(file["generation"], 2, "{file}");
    std::fs::write(dir.join("logo.txt"), "o\n").unwrap();
    ok(av_as(&d, &dir, "grok", &["checkpoint"]));
    assert!(ws.join("logo.txt").exists());
}

#[test]
fn next_with_a_checkout_dir_that_is_not_empty_takes_nothing() {
    let (d, scratch) = posted_for_muse();
    std::fs::create_dir(scratch.path().join("mine")).unwrap();
    std::fs::write(scratch.path().join("mine/keep.txt"), "mine\n").unwrap();
    let o = av_as(&d, scratch.path(), "muse", &["next", "--checkout", "mine"]);
    assert!(!o.status.success(), "{}", out(&o));
    assert!(err(&o).contains("not empty"), "{}", err(&o));
    let queued = ok(av(&d, scratch.path(), &["backlog", "--state", "queued"]));
    assert!(queued.contains("Improve greeting"), "{queued}");
}

/// `av` as `agent` with its own `$AV_HOME`, so a run's directory is the test's.
fn av_home(d: &Daemon, home: &Path, agent: &str, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_av"))
        .current_dir(home)
        .env("AV_URL", &d.url)
        .env("AV_HOME", home)
        .env_remove("AV_TASK")
        .env_remove("AV_SESSION")
        .env_remove("AV_GENERATION")
        .env_remove("AV_MAX_AGENTS")
        .env("AV_AGENT", agent)
        .args(args)
        .output()
        .unwrap()
}

fn decision_recorded() -> (Daemon, TempDir) {
    let (d, ws, _scratch) = started();
    ok(av(
        &d,
        &ws,
        &[
            "record",
            "--kind",
            "decision",
            "--summary",
            "hello never shouts",
            "--body",
            "Decided with the team.",
            "--anchor",
            "app/greeter.py",
        ],
    ));
    (d, TempDir::new().unwrap())
}

const FAKE_DREAMER: &str = r#"
test -f DREAM.md || exit 9
echo "$AV_DREAM_MODE $AV_REPO $AV_DREAM_ROUND $AV_AGENT" >> rounds
rec=$(grep -o 'rec:[0-9a-f]*' bundle.json | head -1)
if [ -f refused.json ]; then quote='def hello(self, name):'; else quote='this line is nowhere in the file'; fi
printf '{"verdicts": [{"record": "%s", "verdict": "disputed", "reason": "hello returns the name as given", "evidence": [{"code": "app/greeter.py", "quote": "%s"}]}]}' "$rec" "$quote" > verdicts.json
"#;

const FAKE_VERIFIER: &str = r#"
vd=$(grep -o 'vd:[0-9a-f]*' bundle.json | head -1)
printf '{"reviews": [{"verdict": "%s", "agree": true, "evidence": [{"code": "app/greeter.py", "quote": "class Greeter:"}]}]}' "$vd" > reviews.json
"#;

fn run_dir(home: &Path, keep: impl Fn(&Path) -> bool) -> (String, PathBuf) {
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(home.join("dreams"))
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| keep(p))
        .collect();
    assert_eq!(dirs.len(), 1, "{dirs:?}");
    let dir = dirs.pop().unwrap();
    (dir.file_name().unwrap().to_str().unwrap().to_owned(), dir)
}

fn word_after<'a>(text: &'a str, start: &str) -> &'a str {
    let at = text
        .find(start)
        .unwrap_or_else(|| panic!("no {start:?} in {text}"));
    text[at..].split_whitespace().next().unwrap()
}

#[test]
fn dream_with_nothing_to_look_at_starts_no_agent() {
    let (d, _ws, _scratch) = started();
    let home = TempDir::new().unwrap();
    let got = ok(av_home(
        &d,
        home.path(),
        "dreamer",
        &["dream", "--repo", "demo", "--", "sh", "-c", "touch ran"],
    ));
    assert!(got.contains("nothing to review in demo"), "{got}");
    assert!(!home.path().join("dreams").exists());
    let o = av_home(&d, home.path(), "dreamer", &["dream", "--repo", "demo"]);
    assert!(
        !o.status.success() && err(&o).contains("after --"),
        "{}",
        err(&o)
    );
}

#[test]
fn dream_posts_what_its_agent_wrote_and_a_verify_run_makes_it_effective() {
    let (d, home) = decision_recorded();
    let home = home.path();
    let got = ok(av_home(
        &d,
        home,
        "dreamer",
        &["dream", "--repo", "demo", "--", "sh", "-c", FAKE_DREAMER],
    ));
    assert!(got.contains("refused: evidence[0]"), "{got}");
    assert!(got.contains("disputed -> vd:"), "{got}");
    assert!(got.contains("done: 1 proposed, 0 refused"), "{got}");
    let (id, dir) = run_dir(home, |_| true);
    assert_eq!(
        std::fs::read_to_string(dir.join("rounds")).unwrap(),
        "review demo 1 dreamer\nreview demo 2 dreamer\n"
    );
    let bundle = std::fs::read_to_string(dir.join("bundle.json")).unwrap();
    assert!(bundle.contains("hello never shouts"), "{bundle}");
    let shown = ok(av_home(&d, home, "dreamer", &["dream", "show", &id]));
    assert!(shown.contains("review demo by dreamer · done"), "{shown}");
    assert!(shown.contains("instructions "), "{shown}");

    let vd = word_after(&got, "vd:");
    let listed = ok(av_home(
        &d,
        home,
        "agent",
        &["verdict", "list", "--repo", "demo", "--state", "proposed"],
    ));
    assert!(
        listed.contains(&format!("{vd}  proposed  disputed  rec:")),
        "{listed}"
    );
    let one = ok(av_home(&d, home, "agent", &["verdict", "show", vd]));
    assert!(
        one.contains("decision by agent: hello never shouts"),
        "{one}"
    );
    assert!(
        one.contains("evidence: app/greeter.py: def hello(self, name):"),
        "{one}"
    );

    let got = ok(av_home(
        &d,
        home,
        "checker",
        &[
            "dream",
            "--verify",
            "--repo",
            "demo",
            "--",
            "sh",
            "-c",
            FAKE_VERIFIER,
        ],
    ));
    assert!(got.contains(&format!("{vd} agreed -> effective")), "{got}");
    let listed = ok(av_home(
        &d,
        home,
        "agent",
        &["verdict", "list", "--repo", "demo"],
    ));
    assert!(listed.contains(&format!("{vd}  effective")), "{listed}");
    let got = ok(av_home(
        &d,
        home,
        "checker",
        &["dream", "--verify", "--repo", "demo", "--", "false"],
    ));
    assert!(got.contains("nothing to verify in demo"), "{got}");
}

#[test]
fn a_dream_whose_agent_fails_posts_nothing_and_frees_its_area() {
    let (d, home) = decision_recorded();
    let home = home.path();
    let script = format!("{FAKE_DREAMER}\nexit 3");
    let o = av_home(
        &d,
        home,
        "dreamer",
        &["dream", "--repo", "demo", "--", "sh", "-c", &script],
    );
    assert!(!o.status.success(), "{}", out(&o));
    assert!(out(&o).contains("failed; nothing"), "{}", out(&o));
    let (id, _) = run_dir(home, |_| true);
    let shown = ok(av_home(&d, home, "dreamer", &["dream", "show", &id]));
    assert!(shown.contains("· failed"), "{shown}");
    let listed = ok(av_home(
        &d,
        home,
        "agent",
        &["verdict", "list", "--repo", "demo"],
    ));
    assert!(listed.contains("no verdicts in demo"), "{listed}");

    let o = av_home(
        &d,
        home,
        "dreamer",
        &["dream", "--repo", "demo", "--", "true"],
    );
    assert!(
        !o.status.success() && err(&o).contains("wrote no"),
        "{}",
        err(&o)
    );
    let got = ok(av_home(
        &d,
        home,
        "dreamer",
        &["dream", "--repo", "demo", "--", "sh", "-c", FAKE_DREAMER],
    ));
    assert!(got.contains("done: 1 proposed"), "{got}");
}

#[test]
fn a_person_accepts_rejects_and_retracts_verdicts_and_undoes_a_run() {
    let (d, home) = decision_recorded();
    let home = home.path();
    let dream = |home: &Path| {
        ok(av_home(
            &d,
            home,
            "dreamer",
            &[
                "dream",
                "--repo",
                "demo",
                "--fix-rounds",
                "1",
                "--",
                "sh",
                "-c",
                FAKE_DREAMER,
            ],
        ))
    };
    let got = dream(home);
    let vd = word_after(&got, "vd:").to_owned();
    let line = ok(av(&d, home, &["verdict", "reject", &vd]));
    assert!(line.contains(&format!("{vd}  rejected")), "{line}");
    let again = av(&d, home, &["verdict", "accept", &vd]);
    assert!(!again.status.success(), "a verdict is reviewed once");

    let got = dream(home);
    let vd = word_after(&got, "vd:").to_owned();
    let line = ok(av(&d, home, &["verdict", "accept", &vd]));
    assert!(line.contains(&format!("{vd}  effective")), "{line}");
    let line = ok(av(&d, home, &["verdict", "retract", &vd]));
    assert!(line.contains(&format!("{vd}  retracted")), "{line}");

    let got = dream(home);
    let vd = word_after(&got, "vd:").to_owned();
    let id = got
        .split("`av dream undo ")
        .nth(1)
        .and_then(|rest| rest.split('`').next())
        .unwrap_or_else(|| panic!("{got}"))
        .to_owned();
    let undone = ok(av(&d, home, &["dream", "undo", &id]));
    assert!(undone.contains("1 verdicts retracted"), "{undone}");
    let listed = ok(av(
        &d,
        home,
        &["verdict", "list", "--repo", "demo", "--state", "retracted"],
    ));
    assert!(listed.contains(&vd), "{listed}");
}

#[test]
fn a_review_that_names_no_verdict_is_refused_and_touches_nothing_else() {
    let (d, home) = decision_recorded();
    let home = home.path();
    let got = ok(av_home(
        &d,
        home,
        "dreamer",
        &["dream", "--repo", "demo", "--", "sh", "-c", FAKE_DREAMER],
    ));
    let vd = word_after(&got, "vd:").to_owned();
    let script = r#"printf '{"reviews": [{"verdict": "../../repos/demo/dreams", "agree": true, "evidence": [{"code": "app/greeter.py", "quote": "class Greeter:"}]}, {"agree": true}]}' > reviews.json"#;
    let got = ok(av_home(
        &d,
        home,
        "checker",
        &[
            "dream",
            "--verify",
            "--repo",
            "demo",
            "--fix-rounds",
            "0",
            "--",
            "sh",
            "-c",
            script,
        ],
    ));
    assert!(got.contains("is not a verdict id"), "{got}");
    assert!(got.contains("a review needs `verdict`"), "{got}");
    assert!(got.contains("done: 2 refused"), "{got}");
    let listed = ok(av_home(
        &d,
        home,
        "agent",
        &["verdict", "list", "--repo", "demo", "--state", "proposed"],
    ));
    assert!(listed.contains(&vd), "{listed}");
}

#[test]
fn dream_budget_is_appended_to_the_agent_command_as_max_budget_usd() {
    let (d, home) = decision_recorded();
    let home = home.path();
    for bad in ["0", "-1", "nan", "inf", "0.001", "two"] {
        let o = av_home(
            &d,
            home,
            "dreamer",
            &[
                "dream",
                "--repo",
                "demo",
                &format!("--budget-usd={bad}"),
                "--",
                "true",
            ],
        );
        assert!(!o.status.success(), "--budget-usd {bad} was taken");
        assert!(err(&o).contains("--budget-usd"), "{}", err(&o));
    }
    assert!(
        !home.join("dreams").exists(),
        "a refused budget started a run"
    );
    // `sh -c SCRIPT A B` gives the script $0=A, $1=B: what `av dream` appended after the command
    let script = format!("echo \"$0 $*\" >> argv\n{FAKE_DREAMER}");
    let got = ok(av_home(
        &d,
        home,
        "dreamer",
        &[
            "dream",
            "--repo",
            "demo",
            "--budget-usd",
            "2.5",
            "--",
            "sh",
            "-c",
            &script,
        ],
    ));
    assert!(got.contains("done: 1 proposed"), "{got}");
    let (_, dir) = run_dir(home, |_| true);
    assert_eq!(
        std::fs::read_to_string(dir.join("argv")).unwrap(),
        "--max-budget-usd 2.50\n--max-budget-usd 2.50\n",
        "every round gets the cap"
    );
}

#[test]
fn dream_also_read_names_directories_to_the_agent_and_sends_the_daemon_none() {
    use sha2::{Digest, Sha256};
    let (d, home) = decision_recorded();
    let home = home.path();
    let transcripts = TempDir::new().unwrap();
    let marked = transcripts.path().join("transcripts-a1b2c3");
    std::fs::create_dir(&marked).unwrap();
    std::fs::write(marked.join("session.jsonl"), "secret-transcript-line\n").unwrap();
    let other = TempDir::new().unwrap();
    let hex = |b: &[u8]| -> String {
        Sha256::digest(b)
            .iter()
            .map(|x| format!("{x:02x}"))
            .collect()
    };
    let shipped = std::fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../integrations/dream/DREAM.md"),
    )
    .unwrap();

    // without --also-read the agent gets what it always got, and the run records its hash
    let script = format!("env | grep '^AV_DREAM_ALSO_READ=' > env.txt\n{FAKE_DREAMER}");
    let got = ok(av_home(
        &d,
        home,
        "dreamer",
        &["dream", "--repo", "demo", "--", "sh", "-c", &script],
    ));
    assert!(got.contains("done: 1 proposed"), "{got}");
    let (plain_id, plain) = run_dir(home, |_| true);
    assert_eq!(std::fs::read(plain.join("DREAM.md")).unwrap(), shipped);
    assert!(!plain.join("also-read.txt").exists());
    assert_eq!(std::fs::read_to_string(plain.join("env.txt")).unwrap(), "");
    let run: serde_json::Value = ureq::get(&format!("{}/v1/dreams/{plain_id}", d.url))
        .call()
        .unwrap()
        .into_json()
        .unwrap();
    assert_eq!(run["dream"]["instructions_sha"], hex(&shipped));

    // a relative path is made absolute; the agent learns of both directories
    let rel = marked.strip_prefix(transcripts.path()).unwrap();
    let script = format!(
        "cat also-read.txt > seen.txt\necho \"$AV_DREAM_ALSO_READ\" > env.txt\n\
         cat \"$(head -1 also-read.txt)/session.jsonl\" > read.txt\n{FAKE_VERIFIER}"
    );
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_av"));
    cmd.current_dir(transcripts.path())
        .env("AV_URL", &d.url)
        .env("AV_HOME", home)
        .env_remove("AV_TASK")
        .env_remove("AV_SESSION")
        .env_remove("AV_GENERATION")
        .env_remove("AV_MAX_AGENTS")
        .env("AV_AGENT", "checker")
        .args(["dream", "--verify", "--repo", "demo", "--also-read"])
        .arg(rel)
        .arg("--also-read")
        .arg(other.path())
        .args(["--", "sh", "-c", &script]);
    let got = ok(cmd.output().unwrap());
    assert!(got.contains("agreed -> effective"), "{got}");
    let (id, dir) = run_dir(home, |p| p != plain);
    let marked = std::fs::canonicalize(&marked).unwrap();
    let other = std::fs::canonicalize(other.path()).unwrap();
    assert_eq!(
        std::fs::read_to_string(dir.join("seen.txt")).unwrap(),
        format!("{}\n{}\n", marked.display(), other.display())
    );
    assert_eq!(
        std::fs::read_to_string(dir.join("env.txt"))
            .unwrap()
            .trim_end(),
        std::env::join_paths([&marked, &other])
            .unwrap()
            .to_str()
            .unwrap()
    );
    assert_eq!(
        std::fs::read_to_string(dir.join("read.txt")).unwrap(),
        "secret-transcript-line\n"
    );
    let told = std::fs::read(dir.join("DREAM.md")).unwrap();
    assert!(told.starts_with(&shipped), "DREAM.md's own text is kept");
    let told = String::from_utf8(told).unwrap();
    assert!(
        told.contains("also-read.txt") && told.contains("not evidence"),
        "{told}"
    );
    let run: serde_json::Value = ureq::get(&format!("{}/v1/dreams/{id}", d.url))
        .call()
        .unwrap()
        .into_json()
        .unwrap();
    assert_eq!(run["dream"]["instructions_sha"], hex(told.as_bytes()));
    // what the daemon holds of the run names neither directory nor anything in them
    let verdicts: serde_json::Value = ureq::get(&format!("{}/v1/repos/demo/verdicts", d.url))
        .call()
        .unwrap()
        .into_json()
        .unwrap();
    for held in [run.to_string(), verdicts.to_string()] {
        for word in [
            "transcripts-a1b2c3",
            "secret-transcript-line",
            other.to_str().unwrap(),
        ] {
            assert!(!held.contains(word), "the daemon holds {word}: {held}");
        }
    }
}

#[test]
fn dream_also_read_refuses_what_is_not_a_directory_before_a_run_starts() {
    let (d, home) = decision_recorded();
    let home = home.path();
    let file = home.join("notes.txt");
    std::fs::write(&file, "x").unwrap();
    for bad in [home.join("no-such-dir"), file] {
        let o = av_home(
            &d,
            home,
            "dreamer",
            &[
                "dream",
                "--repo",
                "demo",
                "--also-read",
                bad.to_str().unwrap(),
                "--",
                "sh",
                "-c",
                "touch ran",
            ],
        );
        assert!(!o.status.success(), "{}", out(&o));
        assert!(err(&o).contains("--also-read"), "{}", err(&o));
        assert!(err(&o).contains(bad.to_str().unwrap()), "{}", err(&o));
    }
    assert!(!home.join("dreams").exists(), "a run was started");
}

#[test]
fn dream_every_goes_on_after_a_failed_run() {
    use std::io::BufRead;
    let (d, home) = decision_recorded();
    let home = home.path();
    for bad in ["0", "0s", "0h", "soon", "5w", "-1h"] {
        let o = av_home(
            &d,
            home,
            "dreamer",
            &[
                "dream",
                "--repo",
                "demo",
                &format!("--every={bad}"),
                "--",
                "true",
            ],
        );
        assert!(!o.status.success(), "--every {bad} was taken");
        assert!(err(&o).contains("--every"), "{}", err(&o));
    }
    let script = format!("{FAKE_DREAMER}\nexit 3");
    let mut child = Command::new(env!("CARGO_BIN_EXE_av"))
        .current_dir(home)
        .env("AV_URL", &d.url)
        .env("AV_HOME", home)
        .env_remove("AV_TASK")
        .env_remove("AV_SESSION")
        .env_remove("AV_GENERATION")
        .env_remove("AV_MAX_AGENTS")
        .env("AV_AGENT", "dreamer")
        .args([
            "dream", "--repo", "demo", "--every", "1d", "--", "sh", "-c", &script,
        ])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();
    let mut seen = vec![];
    for line in std::io::BufReader::new(child.stdout.take().unwrap()).lines() {
        let line = line.unwrap();
        let next = line.contains("next run in ");
        seen.push(line);
        if next {
            break;
        }
    }
    // the loop outlived its failed run: it is waiting for the next one, not gone
    assert!(child.try_wait().unwrap().is_none(), "{seen:?}");
    child.kill().unwrap();
    child.wait().unwrap();
    assert!(
        seen.iter().any(|l| l.contains("failed; nothing")),
        "{seen:?}"
    );
}

#[test]
fn a_promoted_decision_is_listed_as_waiting_for_a_review_and_becomes_a_convention() {
    let (d, home) = decision_recorded();
    let home = home.path();
    let promoter = r#"
rec=$(grep -o 'rec:[0-9a-f]*' bundle.json | head -1)
printf '{"verdicts": [{"record": "%s", "verdict": "promote", "text": "greetings never shout", "scope_glob": "app/**", "evidence": [{"record": "%s", "quote": "hello never shouts"}]}]}' "$rec" "$rec" > verdicts.json
"#;
    let o = av_home(
        &d,
        home,
        "checker",
        &[
            "dream",
            "--verify",
            "--promote",
            "--repo",
            "demo",
            "--",
            "true",
        ],
    );
    assert!(
        !o.status.success() && err(&o).contains("--promote"),
        "{}",
        err(&o)
    );
    let got = ok(av_home(
        &d,
        home,
        "dreamer",
        &[
            "dream",
            "--promote",
            "--repo",
            "demo",
            "--",
            "sh",
            "-c",
            promoter,
        ],
    ));
    assert!(got.contains("promote -> vd:"), "{got}");
    let (_, dir) = run_dir(home, |_| true);
    let told = std::fs::read_to_string(dir.join("DREAM.md")).unwrap();
    let shipped = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../integrations/dream/DREAM.md"),
    )
    .unwrap();
    assert!(
        told.starts_with(&shipped) && told.len() > shipped.len() && told.contains("scope_glob"),
        "a --promote run is told about promote: {told}"
    );
    assert!(!shipped.contains("promote"), "a plain run is not");
    let vd = word_after(&got, "vd:").to_owned();
    let listed = ok(av_home(
        &d,
        home,
        "agent",
        &["verdict", "list", "--repo", "demo", "--state", "proposed"],
    ));
    let line = listed
        .lines()
        .find(|l| l.contains(&vd))
        .unwrap_or_else(|| panic!("{listed}"));
    assert!(
        line.contains("convention for app/**: greetings never shout")
            && line.contains("(waits for a review)"),
        "{line}"
    );
    let (id, _) = run_dir(home, |_| true);
    let shown = ok(av_home(&d, home, "agent", &["dream", "show", &id]));
    assert!(
        shown.contains("convention for app/**: greetings never shout"),
        "{shown}"
    );
    // a verify run is handed nothing: a promote is reviewed by name
    let got = ok(av_home(
        &d,
        home,
        "checker",
        &["dream", "--verify", "--repo", "demo", "--", "false"],
    ));
    assert!(got.contains("nothing to verify in demo"), "{got}");

    let line = ok(av_home(&d, home, "vinh", &["verdict", "accept", &vd]));
    assert!(
        line.contains(&format!("{vd}  effective  promote")) && line.contains("(accepted)"),
        "{line}"
    );
    let conv = ok(av_home(
        &d,
        home,
        "agent",
        &["convention", "list", "--repo", "demo"],
    ));
    assert!(
        conv.contains("(app/**) greetings never shout") && conv.contains("from vd:"),
        "{conv}"
    );
    ok(av_home(&d, home, "vinh", &["verdict", "retract", &vd]));
    let conv = ok(av_home(
        &d,
        home,
        "agent",
        &["convention", "list", "--repo", "demo"],
    ));
    assert!(!conv.contains("greetings never shout"), "{conv}");
}

/// `av` with no agent name anywhere: no `--agent`, no `AV_AGENT`.
fn av_unnamed(d: &Daemon, home: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_av"))
        .current_dir(home)
        .env("AV_URL", &d.url)
        .env("AV_HOME", home)
        .env_remove("AV_TASK")
        .env_remove("AV_SESSION")
        .env_remove("AV_GENERATION")
        .env_remove("AV_MAX_AGENTS")
        .env_remove("AV_AGENT")
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn a_promote_from_a_default_dream_is_accepted_by_a_default_av_verdict_accept() {
    let (d, home) = decision_recorded();
    let home = home.path();
    let promoter = r#"
rec=$(grep -o 'rec:[0-9a-f]*' bundle.json | head -1)
printf '{"verdicts": [{"record": "%s", "verdict": "promote", "text": "greetings never shout", "scope_glob": "app/**", "evidence": [{"record": "%s", "quote": "hello never shouts"}]}]}' "$rec" "$rec" > verdicts.json
"#;
    let got = ok(av_unnamed(
        &d,
        home,
        &[
            "dream",
            "--promote",
            "--repo",
            "demo",
            "--",
            "sh",
            "-c",
            promoter,
        ],
    ));
    let vd = word_after(&got, "vd:").to_owned();
    let line = ok(av_unnamed(&d, home, &["verdict", "accept", &vd]));
    assert!(line.contains("(accepted)"), "{line}");
}

/// Whether `pid` is a live process (a zombie, killed and waiting for its parent, is not).
fn alive(pid: &str) -> bool {
    std::fs::read_to_string(format!("/proc/{pid}/stat")).is_ok_and(|s| {
        !s.rsplit(')')
            .next()
            .unwrap_or("")
            .trim_start()
            .starts_with('Z')
    })
}

#[test]
fn an_interrupted_dream_ends_its_agent_too() {
    use std::io::BufRead;
    let (d, home) = decision_recorded();
    let home = home.path();
    let cases = [
        (None, "INT"),
        (None, "TERM"),
        (None, "HUP"),
        (None, "QUIT"),
        (Some("1d"), "INT"),
        (Some("1d"), "HUP"),
    ];
    for (every, signal) in cases {
        let mut args = vec!["dream", "--repo", "demo"];
        args.extend(every.map(|e| ["--every", e]).into_iter().flatten());
        // the agent and a child of its own, both waiting on a pipe nothing writes to
        let script = "mkfifo hold; cat hold & echo \"agent-started $$ $!\"; wait";
        args.extend(["--", "sh", "-c", script]);
        let mut child = Command::new(env!("CARGO_BIN_EXE_av"))
            .current_dir(home)
            .env("AV_URL", &d.url)
            .env("AV_HOME", home)
            .env_remove("AV_TASK")
            .env_remove("AV_SESSION")
            .env_remove("AV_GENERATION")
            .env_remove("AV_MAX_AGENTS")
            .env("AV_AGENT", "dreamer")
            .args(&args)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn()
            .unwrap();
        let mut lines = std::io::BufReader::new(child.stdout.take().unwrap()).lines();
        let pids: Vec<String> = loop {
            let line = lines
                .next()
                .expect("av ended before its agent started")
                .unwrap();
            if let Some(rest) = line.strip_prefix("agent-started ") {
                break rest.split_whitespace().map(str::to_owned).collect();
            }
        };
        assert_eq!(pids.len(), 2, "{pids:?}");
        let sent = Command::new("kill")
            .args([&format!("-{signal}"), &child.id().to_string()])
            .status()
            .unwrap();
        assert!(sent.success());
        let status = child.wait().unwrap();
        // a SIGKILL takes effect a moment after it is sent: wait for it, up to `HANG`
        let deadline = std::time::Instant::now() + HANG;
        let left = loop {
            let left: Vec<&String> = pids.iter().filter(|p| alive(p)).collect();
            if left.is_empty() || std::time::Instant::now() > deadline {
                break left;
            }
            std::thread::sleep(std::time::Duration::from_millis(20));
        };
        for p in &left {
            let _ = Command::new("kill").args(["-KILL", p]).status();
        }
        assert!(
            left.is_empty(),
            "{every:?} {signal}: the agent outlived av: {left:?}"
        );
        // av ended the agent, then died of the signal it got, so its parent sees that signal
        use std::os::unix::process::ExitStatusExt;
        let number = match signal {
            "HUP" => 1,
            "INT" => 2,
            "QUIT" => 3,
            _ => 15,
        };
        assert_eq!(
            status.signal(),
            Some(number),
            "{every:?} {signal}: {status}"
        );
        // the run was ended as failed, so the area is free for the next one
        for dir in std::fs::read_dir(home.join("dreams")).unwrap() {
            let id = dir.unwrap().file_name().into_string().unwrap();
            let shown = ok(av_home(&d, home, "dreamer", &["dream", "show", &id]));
            assert!(shown.contains("· failed"), "{shown}");
        }
    }
}

#[test]
fn a_dream_started_with_sigint_ignored_keeps_it_ignored() {
    use std::io::BufRead;
    use std::os::unix::process::ExitStatusExt;
    let (d, home) = decision_recorded();
    let home = home.path();
    // as a script's background job is started: SIGINT ignored, which exec keeps
    let mut child = Command::new("sh")
        .args(["-c", "trap '' INT; exec \"$0\" \"$@\""])
        .arg(env!("CARGO_BIN_EXE_av"))
        .args([
            "dream",
            "--repo",
            "demo",
            "--every",
            "1d",
            "--",
            "sh",
            "-c",
            FAKE_DREAMER,
        ])
        .current_dir(home)
        .env("AV_URL", &d.url)
        .env("AV_HOME", home)
        .env_remove("AV_TASK")
        .env_remove("AV_SESSION")
        .env_remove("AV_GENERATION")
        .env_remove("AV_MAX_AGENTS")
        .env("AV_AGENT", "dreamer")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();
    let mut seen = vec![];
    for line in std::io::BufReader::new(child.stdout.take().unwrap()).lines() {
        let line = line.unwrap();
        let next = line.contains("next run in ");
        seen.push(line);
        if next {
            break;
        }
    }
    assert!(
        seen.iter().any(|l| l.contains("done: 1 proposed")),
        "{seen:?}"
    );
    // SIGINT first (still ignored after the run), then SIGTERM, which ends it
    for signal in ["-INT", "-TERM"] {
        Command::new("kill")
            .args([signal, &child.id().to_string()])
            .status()
            .unwrap();
    }
    let status = child.wait().unwrap();
    assert_eq!(status.signal(), Some(15), "{status}");
}

#[test]
fn dream_budget_is_refused_when_the_command_carries_its_own_cap() {
    let (d, home) = decision_recorded();
    let home = home.path();
    for own in [
        ["--max-budget-usd", "3"].as_slice(),
        &["--max-budget-usd=3"],
    ] {
        let mut args = vec![
            "dream",
            "--repo",
            "demo",
            "--budget-usd",
            "2",
            "--",
            "claude",
            "-p",
            "x",
        ];
        args.extend(own);
        let o = av_home(&d, home, "dreamer", &args);
        assert!(!o.status.success(), "{}", out(&o));
        assert!(
            err(&o).contains("--budget-usd") && err(&o).contains("--max-budget-usd"),
            "{}",
            err(&o)
        );
    }
    assert!(!home.join("dreams").exists(), "a run was started");
}

/// Adds the repo `demo` and starts the task "Over MCP" in it as Claude Code does after
/// `av init --for claude`: no `X-Av-Agent`, so the MCP session is named after the client,
/// `claude-code`. Returns the task's workspace.
fn start_over_mcp_as_claude_code(d: &Daemon) -> PathBuf {
    let scratch = TempDir::new().unwrap();
    ok(av(
        d,
        scratch.path(),
        &["repo", "add", "demo", d.src.to_str().unwrap()],
    ));
    let mcp = format!("{}/mcp", d.url);
    let init = ureq::post(&mcp)
        .set("accept", "application/json, text/event-stream")
        .send_json(
            serde_json::json!({"jsonrpc": "2.0", "id": 0, "method": "initialize",
            "params": {"protocolVersion": "2025-06-18", "capabilities": {},
                       "clientInfo": {"name": "claude-code", "version": "1"}}}),
        )
        .unwrap();
    let session = init.header("mcp-session-id").unwrap().to_owned();
    ureq::post(&mcp)
        .set("mcp-session-id", &session)
        .set("accept", "application/json, text/event-stream")
        .send_json(serde_json::json!({"jsonrpc": "2.0", "method": "notifications/initialized"}))
        .unwrap();
    let r: serde_json::Value = ureq::post(&mcp)
        .set("mcp-session-id", &session)
        .set("accept", "application/json, text/event-stream")
        .send_json(
            serde_json::json!({"jsonrpc": "2.0", "id": 1, "method": "tools/call",
            "params": {"name": "start_task", "arguments":
                {"repo": "demo", "title": "Over MCP", "goal": "Make hello louder"}}}),
        )
        .unwrap()
        .into_json()
        .unwrap();
    assert_eq!(r["result"]["isError"], false, "{r}");
    d.ws.join("demo/over-mcp")
}

/// F14: a task started over MCP has a `.av-task` the daemon wrote, so `av` and the hooks in its
/// workspace find the task with no `--task`. As `av init --for claude` sets it up: no
/// `X-Av-Agent` (the MCP session is named after the client, `claude-code`) and no `AV_AGENT`
/// (the CLI and hooks act as `agent`); the owner's writes must not be refused as "claim lost".
#[test]
fn validate_finds_a_task_started_over_mcp() {
    let d = start_daemon();
    let ws = start_over_mcp_as_claude_code(&d);
    std::fs::write(
        ws.join("app/greeter.py"),
        "class Greeter:\n    def hello(self, name):\n        return name.upper()\n",
    )
    .unwrap();
    let edited = av(&d, &ws, &["hook", "post-edit"]);
    assert!(edited.status.success(), "{}", err(&edited));
    assert!(!err(&edited).contains("claim lost"), "{}", err(&edited));
    let status = ok(av(&d, &ws, &["status"]));
    assert!(
        status.contains("changed (1): app/greeter.py"),
        "the hook's checkpoint landed: {status}"
    );
    let passed = ok(av(&d, &ws, &["validate", "--", "true"]));
    assert!(passed.contains("validation passed"), "{passed}");
}

/// F41: in that same default install the Stop hook acts as `agent` while the session's
/// `start_task` acts as `claude-code`. Once the session submits, its task is in review, and the
/// hook must not offer the session its own task as a review waiting: `start_task` would find
/// nothing there and the hook would block again, turn after paid turn.
#[test]
fn the_stop_hook_does_not_offer_a_session_its_own_task_for_review() {
    let d = start_daemon();
    let ws = start_over_mcp_as_claude_code(&d);
    std::fs::write(
        ws.join("app/greeter.py"),
        "class Greeter:\n    def hello(self, name):\n        return name.upper()\n",
    )
    .unwrap();
    ok(av_env(
        &d.url,
        &ws,
        &[("AV_AGENT", "claude-code")],
        &[
            "submit",
            "--record",
            "decision: shout the name @ app/greeter.py",
        ],
    ));
    let stopped = ok(av_stdin_env(
        &d.url,
        &ws,
        &[("AV_HOOK_WAIT_SECS", "1")],
        &["hook", "stop"],
        &stop_input(&ws, "cc-1", false),
    ));
    assert_eq!(
        stopped, "",
        "the session was offered its own task: {stopped}"
    );
    let other = ok(av_stdin_env(
        &d.url,
        &ws,
        &[("AV_HOOK_WAIT_SECS", "1"), ("AV_AGENT", "reviewer")],
        &["hook", "stop"],
        &stop_input(&ws, "cc-2", false),
    ));
    assert!(other.contains("a review is waiting"), "{other}");
}

fn mcp_named(d: &Daemon, name: &str) -> (String, String) {
    let mcp = format!("{}/mcp", d.url);
    let init = ureq::post(&mcp)
        .set("accept", "application/json, text/event-stream")
        .set("x-av-agent", name)
        .send_json(
            serde_json::json!({"jsonrpc": "2.0", "id": 0, "method": "initialize",
            "params": {"protocolVersion": "2025-06-18", "capabilities": {},
                       "clientInfo": {"name": "some-client", "version": "1"}}}),
        )
        .unwrap();
    let session = init.header("mcp-session-id").unwrap().to_owned();
    ureq::post(&mcp)
        .set("mcp-session-id", &session)
        .set("accept", "application/json, text/event-stream")
        .send_json(serde_json::json!({"jsonrpc": "2.0", "method": "notifications/initialized"}))
        .unwrap();
    (mcp, session)
}

fn mcp_tool(mcp: &(String, String), name: &str, args: serde_json::Value) -> serde_json::Value {
    let r: serde_json::Value = ureq::post(&mcp.0)
        .set("mcp-session-id", &mcp.1)
        .set("accept", "application/json, text/event-stream")
        .send_json(
            serde_json::json!({"jsonrpc": "2.0", "id": 1, "method": "tools/call",
            "params": {"name": name, "arguments": args}}),
        )
        .unwrap()
        .into_json()
        .unwrap();
    assert_eq!(r["result"]["isError"], false, "{r}");
    r
}

fn started_as(owner: &str) -> (Daemon, PathBuf, TempDir) {
    let d = start_daemon();
    let scratch = TempDir::new().unwrap();
    ok(av(
        &d,
        scratch.path(),
        &["repo", "add", "demo", d.src.to_str().unwrap()],
    ));
    ok(av_as(
        &d,
        scratch.path(),
        owner,
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
    ok(av_as(
        &d,
        &ws,
        owner,
        &[
            "record",
            "--kind",
            "decision",
            "--summary",
            "shout in hello",
            "--anchor",
            "app/greeter.py",
        ],
    ));
    (d, ws, scratch)
}

fn task_now(d: &Daemon, task: &str) -> serde_json::Value {
    ureq::get(&format!("{}/v1/tasks/{task}", d.url))
        .call()
        .unwrap()
        .into_json::<serde_json::Value>()
        .unwrap()["task"]
        .clone()
}

#[test]
fn a_named_previous_owner_is_fenced_after_a_claim_over_mcp() {
    let (d, ws, _scratch) = started_as("claude");
    ok(av_as(
        &d,
        &ws,
        "claude",
        &["handoff", "--done", "half", "--remaining", "rest"],
    ));
    let task = task_id_of(&ws);
    let grok = mcp_named(&d, "grok");
    let claimed = mcp_tool(&grok, "claim_task", serde_json::json!({"task_id": task}));
    assert!(claimed.to_string().contains("generation: 2"), "{claimed}");
    let before = task_now(&d, &task);

    std::fs::write(ws.join("app/greeter.py"), "def hello():\n    return 'HI'\n").unwrap();
    let o = av_stdin_env(
        &d.url,
        &ws,
        &[("AV_AGENT", "claude")],
        &["hook", "post-edit"],
        &hook_input(&ws, "PostToolUse", "cc-1"),
    );
    assert!(o.status.success(), "{}", err(&o));
    let now = task_now(&d, &task);
    assert_eq!(
        now["head_change"], before["head_change"],
        "a fenced edit landed"
    );
    assert_eq!(
        now["lease_expires"], before["lease_expires"],
        "it renewed grok's lease"
    );
    let told = out(&o);
    assert!(told.contains("additionalContext"), "{told}");
    assert!(told.contains("`grok` now owns this task"), "{told}");
    let input = hook_input(&ws, "PostToolUse", "cc-1");
    let told = out(&post_edit_script(&d, &ws, "claude", &input));
    assert!(told.contains("`grok` now owns this task"), "{told}");

    let o = av_as(&d, &ws, "claude", &["checkpoint"]);
    assert!(!o.status.success(), "{}", out(&o));
    let said = err(&o);
    assert!(said.contains("grok") && said.contains("claude"), "{said}");
    assert_eq!(task_now(&d, &task)["head_change"], before["head_change"]);

    post_item(&d, &ws, "Fix the README typos");
    let stop = |session: &str| {
        let o = stop_with(
            &d,
            &ws,
            &[("AV_AGENT", "claude"), ("AV_HOOK_WAIT_SECS", "1")],
            session,
            true,
        );
        assert!(o.status.success(), "{}", err(&o));
        out(&o)
    };
    assert!(stop("cc-1").contains("`grok` now owns this task"));
    assert!(stop("cc-1").is_empty(), "told twice");
    let file: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(ws.join(".av-task")).unwrap()).unwrap();
    assert!(file.get("nudged_session").is_none(), "{file}");
    assert!(file.get("continue_session").is_none(), "{file}");

    // a validation checkpoints nothing and is recorded at the head as it is, so over the edits the fence kept out it is refused
    let o = av_as(&d, &ws, "claude", &["validate", "--", "true"]);
    assert!(err(&o).contains("without a checkpoint"), "{}", err(&o));
    assert!(!o.status.success(), "{}", out(&o));
    assert!(
        err(&o).contains("holds edits its head does not"),
        "{}",
        err(&o)
    );
    assert_eq!(task_now(&d, &task)["head_change"], before["head_change"]);

    ok(av_as(&d, &ws, "grok", &["checkpoint"]));
}

#[test]
fn an_owner_under_an_alias_is_not_fenced() {
    let d = start_daemon();
    let scratch = TempDir::new().unwrap();
    ok(av(
        &d,
        scratch.path(),
        &["repo", "add", "demo", d.src.to_str().unwrap()],
    ));
    ureq::put(&format!("{}/v1/agents/claude-opus", d.url))
        .send_json(serde_json::json!({"best": ["code"], "aliases": ["opus"]}))
        .unwrap();
    ok(av(
        &d,
        scratch.path(),
        &[
            "post",
            "--repo",
            "demo",
            "--title",
            "Louder",
            "--goal",
            "Make hello louder",
        ],
    ));
    let taken = ok(av_as(&d, scratch.path(), "opus", &["next"]));
    let ws = PathBuf::from(
        taken
            .lines()
            .find_map(|l| l.strip_prefix("workspace: "))
            .unwrap_or_else(|| panic!("{taken}")),
    );
    let file: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(ws.join(".av-task")).unwrap()).unwrap();
    assert_eq!(file["agent"], "claude-opus", "{file}");
    assert_eq!(file["aliases"], serde_json::json!(["opus"]), "{file}");
    std::fs::write(ws.join("app/greeter.py"), "def hello():\n    return 'HI'\n").unwrap();
    ok(av_as(&d, &ws, "opus", &["checkpoint"]));
    ok(av_as(&d, &ws, "claude-opus", &["checkpoint"]));
}

#[test]
fn an_unnamed_command_is_not_refused_in_a_named_owners_workspace() {
    let (d, ws, _scratch) = started_as("claude");
    std::fs::write(ws.join("app/greeter.py"), "def hello():\n    return 'HI'\n").unwrap();
    let o = av(&d, &ws, &["checkpoint"]);
    assert!(o.status.success(), "{}", err(&o));
    assert!(
        err(&o).contains("acting on `claude`'s task with no agent name"),
        "{}",
        err(&o)
    );
}

#[test]
fn a_reviewer_can_validate_in_the_owners_workspace() {
    let (d, ws, _scratch) = started_as("claude");
    std::fs::write(ws.join("app/greeter.py"), "def hello():\n    return 'HI'\n").unwrap();
    ok(av_as(&d, &ws, "claude", &["submit"]));
    let task = task_id_of(&ws);
    let head = task_now(&d, &task)["head_change"].clone();
    let passed = ok(av_as(&d, &ws, "reviewer", &["validate", "--", "true"]));
    assert!(passed.contains("validation passed"), "{passed}");
    assert_eq!(task_now(&d, &task)["head_change"], head);
    assert!(passed.contains(&head.as_str().unwrap()[..12]), "{passed}");
}

#[test]
fn a_remote_clone_names_the_owner() {
    let (d, ws, _scratch) = started_as("claude");
    let task = task_of(&ws);
    let host = TempDir::new().unwrap();
    let dir = host.path().join("co");
    ok(av_as(
        &d,
        host.path(),
        "grok",
        &["remote", "clone", &task, dir.to_str().unwrap()],
    ));
    let file: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(dir.join(".av-task")).unwrap()).unwrap();
    assert_eq!(file["agent"], "claude", "{file}");
    std::fs::write(
        dir.join("app/greeter.py"),
        "def hello():\n    return 'HI'\n",
    )
    .unwrap();
    let o = av_as(&d, &dir, "grok", &["checkpoint"]);
    assert!(
        !o.status.success() && err(&o).contains("`claude` now owns this task"),
        "{}",
        err(&o)
    );
    let o = av_as(&d, &dir, "grok", &["validate", "--", "true"]);
    assert!(
        !o.status.success() && err(&o).contains("remote checkout"),
        "{}",
        err(&o)
    );
    ok(av_as(&d, &dir, "claude", &["checkpoint"]));
}

/// A task started under an alias stores the alias as its owner (only `next` resolves one): the
/// file names the profile's other names with it, so the owner under its profile's name (the name
/// Grok's hooks act as) holds the claim.
#[test]
fn an_owner_that_started_under_an_alias_goes_by_its_profiles_names() {
    let d = start_daemon();
    let scratch = TempDir::new().unwrap();
    ok(av(
        &d,
        scratch.path(),
        &["repo", "add", "demo", d.src.to_str().unwrap()],
    ));
    ureq::put(&format!("{}/v1/agents/grok", d.url))
        .send_json(serde_json::json!({"best": ["code"], "aliases": ["opus"]}))
        .unwrap();
    ok(av_as(
        &d,
        scratch.path(),
        "opus",
        &[
            "task",
            "start",
            "--repo",
            "demo",
            "--title",
            "Louder",
            "--goal",
            "Make hello louder",
        ],
    ));
    let ws = d.ws.join("demo/louder");
    let file: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(ws.join(".av-task")).unwrap()).unwrap();
    assert_eq!(file["agent"], "opus", "{file}");
    assert_eq!(file["aliases"], serde_json::json!(["grok"]), "{file}");
    std::fs::write(
        ws.join("app/greeter.py"),
        "def hello():
    return 'HI'
",
    )
    .unwrap();
    ok(av_as(&d, &ws, "grok", &["checkpoint"]));
    let o = av_as(&d, &ws, "codex", &["checkpoint"]);
    assert!(
        !o.status.success() && err(&o).contains("`opus` now owns this task"),
        "{}",
        err(&o)
    );
}

fn post_edit_script(d: &Daemon, ws: &Path, agent: &str, stdin: &str) -> Output {
    use std::io::Write as _;
    let bin = Path::new(env!("CARGO_BIN_EXE_av"))
        .parent()
        .unwrap()
        .to_owned();
    let path = format!("{}:{}", bin.display(), std::env::var("PATH").unwrap());
    let mut child = Command::new("bash")
        .arg(repo_root().join("integrations/claude-code/hooks/post-edit.sh"))
        .current_dir(ws)
        .env("PATH", path)
        .env("AV_URL", &d.url)
        .env("AV_AGENT", agent)
        .env_remove("AV_TASK")
        .env_remove("AV_SESSION")
        .env_remove("AV_GENERATION")
        .env_remove("GROK_HOOK_EVENT")
        .env("AV_FOOTPRINT", "off")
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

/// The daemon names the claimant in the file it hands out, so the new generation is never on disk unnamed for a named previous owner to write under.
#[test]
fn av_claims_new_generation_is_never_on_disk_unnamed() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    let (d, ws, _scratch) = started_as("claude");
    ok(av_as(
        &d,
        &ws,
        "claude",
        &["handoff", "--done", "half", "--remaining", "rest"],
    ));
    let task = task_id_of(&ws);
    let path = ws.join(".av-task");
    let done = Arc::new(AtomicBool::new(false));
    let watcher = {
        let done = done.clone();
        std::thread::spawn(move || {
            let (mut seen, mut unnamed) = (0u32, Vec::new());
            while !done.load(Ordering::SeqCst) {
                let Ok(text) = std::fs::read_to_string(&path) else {
                    continue;
                };
                let Ok(tf) = serde_json::from_str::<serde_json::Value>(&text) else {
                    continue;
                };
                if tf["generation"] == 2 {
                    seen += 1;
                    if tf["agent"] != "codex" {
                        unnamed.push(text);
                    }
                }
            }
            (seen, unnamed)
        })
    };
    ok(av_as(&d, &ws, "codex", &["claim", &task]));
    std::thread::sleep(std::time::Duration::from_millis(50));
    done.store(true, Ordering::SeqCst);
    let (seen, unnamed) = watcher.join().unwrap();
    assert!(seen > 0, "the watcher never read generation 2");
    assert!(unnamed.is_empty(), "generation 2 named nobody: {unnamed:?}");
}

#[test]
fn the_default_daemon_address_is_the_ipv4_loopback_the_daemon_listens_on() {
    let o = Command::new(env!("CARGO_BIN_EXE_av"))
        .env_remove("AV_URL")
        .arg("--help")
        .output()
        .unwrap();
    assert!(
        out(&o).contains("[default: http://127.0.0.1:7420]"),
        "{}",
        out(&o)
    );
}

#[test]
fn widen_in_a_full_workspace_writes_nothing_and_needs_a_path() {
    let (d, ws, _scratch) = started();
    assert_eq!(ok(av(&d, &ws, &["widen", "app"])), "written (0): \n");
    let o = av(&d, &ws, &["widen"]);
    assert!(!o.status.success());
}

/// The image builds from `deploy/Dockerfile`, which copies `crates/` and little else, under a
/// `.dockerignore` that drops `docs/` and every `*.md`: a file embedded from outside `crates/`
/// must be copied and not ignored, or the image breaks with nothing else failing (F1).
#[test]
fn the_docker_build_context_has_every_file_the_crates_embed() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let read = |p: &str| std::fs::read_to_string(root.join(p)).unwrap();
    let copied: Vec<String> = read("deploy/Dockerfile")
        .lines()
        .filter_map(|l| l.strip_prefix("COPY "))
        .filter(|l| !l.starts_with("--"))
        .filter_map(|l| l.split_whitespace().next())
        .map(|src| src.trim_end_matches('/').to_owned())
        .collect();
    let ignore = read(".dockerignore");
    let rules: Vec<(bool, regex::Regex)> = ignore
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|l| {
            let (keep, glob) = l.strip_prefix('!').map_or((false, l), |g| (true, g));
            let glob = glob.trim_matches('/');
            let re = regex::escape(glob)
                .replace(r"\*\*/", "(.*/)?")
                .replace(r"\*\*", ".*")
                .replace(r"\*", "[^/]*")
                .replace(r"\?", "[^/]");
            (keep, regex::Regex::new(&format!("^{re}(/.*)?$")).unwrap())
        })
        .collect();
    let ignored = |path: &str| {
        rules
            .iter()
            .rfind(|(_, re)| re.is_match(path))
            .is_some_and(|(keep, _)| !keep)
    };
    let embed = regex::Regex::new(r#"include_(?:str|bytes)!\(\s*"([^"]+)"\s*,?\s*\)"#).unwrap();
    let mut sources = vec![];
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(root.join("crates"))
        .unwrap()
        .map(|e| e.unwrap().path().join("src"))
        .collect();
    while let Some(dir) = dirs.pop() {
        for e in std::fs::read_dir(&dir).into_iter().flatten() {
            let p = e.unwrap().path();
            if p.is_dir() {
                dirs.push(p);
            } else if p.extension().is_some_and(|x| x == "rs") {
                sources.push(p);
            }
        }
    }
    let mut outside = vec![];
    for src in sources {
        for c in embed.captures_iter(&std::fs::read_to_string(&src).unwrap()) {
            let file = src.parent().unwrap().join(&c[1]);
            let file = file
                .canonicalize()
                .unwrap_or_else(|e| panic!("{} (from {}): {e}", file.display(), src.display()));
            let rel = file.strip_prefix(root.canonicalize().unwrap()).unwrap();
            let rel = rel.to_string_lossy().replace('\\', "/");
            if rel.starts_with("crates/") {
                continue;
            }
            assert!(
                copied
                    .iter()
                    .any(|c| rel == *c || rel.starts_with(&format!("{c}/"))),
                "{rel} (embedded by {}) is not copied by deploy/Dockerfile: {copied:?}",
                src.display()
            );
            assert!(!ignored(&rel), "{rel} is excluded by .dockerignore");
            outside.push(rel);
        }
    }
    assert!(
        outside.contains(&"integrations/dream/DREAM.md".to_owned()),
        "the scan finds the one embedded file known to sit outside crates/: {outside:?}"
    );
}

#[test]
fn context_stats_and_backfill_read_the_tasks_repo() {
    let (d, ws, _scratch) = started();
    let stats = ok(av(&d, &ws, &["context", "stats"]));
    assert_eq!(stats, "demo: 0 merged tasks measured\n");
    let json = ok(av(
        &d,
        &ws,
        &["context", "stats", "--repo", "demo", "--tasks", "--json"],
    ));
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["measured"], 0, "{v}");
    assert_eq!(v["rows"], serde_json::json!([]), "{v}");
    let filled = ok(av(&d, &ws, &["context", "backfill", "--limit", "5"]));
    assert_eq!(filled, "measured 0 merged tasks\n");
    let o = av(&d, &ws, &["context", "stats", "--repo", "nope"]);
    assert!(!o.status.success(), "{}", out(&o));
    assert!(
        !av(&d, &ws, &["context", "--budget", "100", "stats"])
            .status
            .success()
    );
}

fn ask_args<'a>(question: &'a str, undo: &'a str) -> Vec<&'a str> {
    vec![
        "ask",
        question,
        "--option",
        "Postgres",
        "--option",
        "SQLite",
        "--recommended",
        "Postgres",
        "--why",
        "the daemon already runs one",
        "--depends",
        "the cache's schema",
        "--undo",
        undo,
        "--anchor",
        "app/greeter.py:Greeter.hello",
    ]
}

#[test]
fn ask_prints_the_instruction_and_a_person_answers_by_number() {
    let (d, ws, scratch) = started();
    let asked = ok(av(
        &d,
        &ws,
        &ask_args("May the cache outlive a restart?", "outside"),
    ));
    assert!(
        asked.starts_with("Finish what does not depend on this, checkpoint, then stop this task."),
        "{asked}"
    );
    assert!(
        asked.contains("; 1 open on this task, 3 allowed)"),
        "{asked}"
    );
    let open = ok(av(&d, scratch.path(), &["answer"]));
    assert!(open.contains("May the cache outlive a restart?"), "{open}");
    assert!(open.contains("demo/improve-greeting (open)"), "{open}");
    assert!(
        open.contains("1 Postgres (recommended: the daemon already runs one)"),
        "{open}"
    );
    assert!(open.contains("2 SQLite"), "{open}");
    let id = open.split_whitespace().next().unwrap().to_owned();
    assert!(id.starts_with("rec:"), "{open}");
    let answered = ok(av(
        &d,
        scratch.path(),
        &["answer", &id, "2", "--comment", "keep it on one disk"],
    ));
    assert!(answered.contains("SQLite"), "{answered}");
    assert!(answered.contains("not the recommended"), "{answered}");
    let again = av(&d, scratch.path(), &["answer", &id, "1"]);
    assert!(!again.status.success());
    assert!(err(&again).contains("already answered"), "{}", err(&again));
    assert!(ok(av(&d, scratch.path(), &["answer"])).contains("no open questions"));
    let md = ok(av(&d, &ws, &["context", "--budget", "3000"]));
    assert!(
        md.contains("answered by user: May the cache outlive a restart? → SQLite"),
        "{md}"
    );
}

#[test]
fn an_unnamed_ask_takes_the_owners_level_and_an_unnamed_note_withdraws_it() {
    let d = start_daemon();
    let scratch = TempDir::new().unwrap();
    ok(av(
        &d,
        scratch.path(),
        &["repo", "add", "demo", d.src.to_str().unwrap()],
    ));
    ok(av_as(
        &d,
        scratch.path(),
        "claude-code",
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
    ok(av(
        &d,
        scratch.path(),
        &["agent", "set", "claude-code", "--ask-level", "ask_first"],
    ));
    ok(av(
        &d,
        scratch.path(),
        &["agent", "set", "grok", "--ask-level", "carry_on"],
    ));
    const PARKS: &str = "Finish what does not depend on this, checkpoint, then stop this task.";
    let asked = ok(av(
        &d,
        &ws,
        &ask_args("May the cache outlive a restart?", "workspace"),
    ));
    assert!(asked.starts_with(PARKS), "{asked}");
    let handle = asked
        .split("(question ")
        .nth(1)
        .and_then(|rest| rest.split(';').next())
        .unwrap()
        .to_owned();
    ok(av(
        &d,
        &ws,
        &[
            "record",
            "--kind",
            "note",
            "--summary",
            "found it in the docs",
            "--supersedes",
            &handle,
        ],
    ));

    let tf: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(ws.join(".av-task")).unwrap()).unwrap();
    let task = tf["task"].as_str().unwrap();
    let mut other = vec!["--task", task];
    other.extend(ask_args("May the cache outlive a restart?", "workspace"));
    let asked = ok(av_as(&d, scratch.path(), "grok", &other));
    assert!(asked.starts_with(PARKS), "{asked}");
}

#[test]
fn an_agents_ask_level_is_set_on_its_profile_and_decides_the_answer() {
    let (d, ws, scratch) = started();
    let bad = av(
        &d,
        scratch.path(),
        &["agent", "set", "agent", "--ask-level", "sometimes"],
    );
    assert!(!bad.status.success());
    assert!(err(&bad).contains("carry_on"), "{}", err(&bad));
    let set = ok(av(
        &d,
        scratch.path(),
        &["agent", "set", "agent", "--ask-level", "carry_on"],
    ));
    assert!(set.contains("ask_level carry_on"), "{set}");
    assert!(ok(av(&d, scratch.path(), &["agent", "list"])).contains("ask_level carry_on"));
    let asked = ok(av(
        &d,
        &ws,
        &ask_args("May the cache outlive a restart?", "outside"),
    ));
    assert!(
        asked.starts_with("Carry on with Postgres. Record what you build on it."),
        "{asked}"
    );
}

/// What a stand-in daemon does with each connection, for the hooks' failure cases.
#[derive(Clone)]
enum Fake {
    /// Accepts and never answers.
    Hang,
    /// Answers every request with this status and an `error`.
    Status(u16),
    /// Answers `body` with 200 after `delay`.
    Slow(std::time::Duration, &'static str),
    /// Answers the first `n` requests with `body`, then hangs.
    AnswersThenHangs(usize, &'static str),
}

fn answer(conn: &mut std::net::TcpStream, status: u16, body: &str) {
    use std::io::{Read as _, Write as _};
    let mut buf = [0u8; 65536];
    let _ = conn.read(&mut buf);
    let _ = write!(
        conn,
        "HTTP/1.1 {status} X\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
        body.len()
    );
}

/// A listener on a free port acting as `fake`; it lives as long as the test process.
fn fake_daemon(fake: Fake) -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    std::thread::spawn(move || {
        let mut held = Vec::new();
        for (i, conn) in listener.incoming().enumerate() {
            let Ok(mut conn) = conn else { continue };
            match fake.clone() {
                Fake::Hang => held.push(conn),
                Fake::AnswersThenHangs(n, _) if i >= n => held.push(conn),
                Fake::AnswersThenHangs(_, body) => answer(&mut conn, 200, body),
                Fake::Status(status) => answer(&mut conn, status, r#"{"error":"stand-in"}"#),
                Fake::Slow(delay, body) => {
                    std::thread::spawn(move || {
                        std::thread::sleep(delay);
                        answer(&mut conn, 200, body);
                    });
                }
            }
        }
    });
    url
}

/// An address where nothing listens.
fn refused_url() -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    format!("http://{}", listener.local_addr().unwrap())
}

const HOOKS: [&str; 4] = ["session-start", "pre-read", "post-edit", "stop"];

/// A workspace with a `.av-task` for a task no daemon has, beside its own av home.
fn hook_fixture() -> TempDir {
    let dir = TempDir::new().unwrap();
    std::fs::create_dir_all(dir.path().join("ws")).unwrap();
    std::fs::write(
        dir.path().join("ws/.av-task"),
        r#"{"task":"t-1","session":"s-1"}"#,
    )
    .unwrap();
    std::fs::write(dir.path().join("ws/app.py"), "x = 1\n").unwrap();
    dir
}

fn hook_payload(hook: &str, ws: &Path) -> String {
    let file = ws.join("app.py");
    match hook {
        "session-start" => {
            serde_json::json!({"session_id": "cc-1", "source": "startup", "cwd": ws})
        }
        "pre-read" => serde_json::json!({
            "hook_event_name": "PreToolUse", "tool_input": {"file_path": file}, "cwd": ws,
        }),
        "post-edit" => serde_json::json!({
            "hook_event_name": "PostToolUse", "tool_input": {"file_path": file}, "cwd": ws,
        }),
        _ => serde_json::json!({
            "hook_event_name": "Stop", "session_id": "cc-1", "stop_hook_active": false, "cwd": ws,
        }),
    }
    .to_string()
}

/// `av hook HOOK` in `fixture`'s workspace against `url`; fails the test when it still runs after
/// `HANG` (and stops it). The Stop hook's wait for work is on (600 s) unless `env` says otherwise.
fn run_hook(url: &str, fixture: &Path, hook: &str, env: &[(&str, &str)]) -> Output {
    use std::io::Write as _;
    let ws = fixture.join("ws");
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_av"));
    cmd.current_dir(&ws)
        .env("AV_URL", url)
        .env("AV_HOME", fixture.join("av-home"))
        .env("AV_HOOK_WAIT_SECS", "600")
        .env("AV_FOOTPRINT", "off")
        .env_remove("AV_TASK")
        .env_remove("AV_SESSION")
        .env_remove("AV_AGENT")
        .env_remove("AV_TOKEN")
        .env_remove("GROK_HOOK_EVENT")
        .env_remove("GROK_SESSION_ID")
        .env_remove("CLAUDE_ENV_FILE")
        .env_remove("AV_HOOK_MAX_CONTINUES");
    for var in [
        "AV_HOOK_SESSION_START_SECS",
        "AV_HOOK_PRE_READ_SECS",
        "AV_HOOK_POST_EDIT_SECS",
        "AV_HOOK_STOP_SECS",
    ] {
        cmd.env_remove(var);
    }
    for (k, v) in env {
        cmd.env(k, v);
    }
    let mut child = cmd
        .args(["hook", hook])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    let _ = child
        .stdin
        .take()
        .unwrap()
        .write_all(hook_payload(hook, &ws).as_bytes());
    let started = std::time::Instant::now();
    while child.try_wait().unwrap().is_none() {
        if started.elapsed() > HANG {
            let _ = child.kill();
            let o = child.wait_with_output().unwrap();
            panic!(
                "av hook {hook} against {url} still ran after {HANG:?}; stderr: {}",
                err(&o)
            );
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    child.wait_with_output().unwrap()
}

/// Every hook kind at once, each in its own fixture, so a test waits for the longest one only.
fn every_hook(url: &str, env: &[(&str, &str)]) -> Vec<(&'static str, Output)> {
    std::thread::scope(|s| {
        let runs: Vec<_> = HOOKS
            .iter()
            .map(|hook| {
                s.spawn(move || {
                    let fixture = hook_fixture();
                    (*hook, run_hook(url, fixture.path(), hook, env))
                })
            })
            .collect();
        runs.into_iter().map(|r| r.join().unwrap()).collect()
    })
}

/// Exit 0 and nothing on stdout, which a host would read as a block or as malformed JSON.
fn assert_let_through(hook: &str, o: &Output) {
    assert!(
        o.status.success(),
        "{hook}: exit {:?}: {}",
        o.status.code(),
        err(o)
    );
    assert_eq!(out(o), "", "{hook}: stdout; stderr: {}", err(o));
}

#[test]
fn hooks_give_up_on_a_daemon_that_accepts_and_never_answers() {
    let url = fake_daemon(Fake::Hang);
    for (hook, o) in every_hook(&url, &[]) {
        assert_let_through(hook, &o);
    }
}

#[test]
fn hooks_let_the_agent_on_when_nothing_listens_or_the_daemon_fails() {
    for url in [
        refused_url(),
        fake_daemon(Fake::Status(500)),
        fake_daemon(Fake::Status(401)),
    ] {
        for (hook, o) in every_hook(&url, &[]) {
            assert_let_through(hook, &o);
        }
    }
}

/// A slow daemon's answer to every request: a pack, a record, a notice and an unrecorded
/// change, so each hook prints something once it has the answer.
const SLOW_ANSWER: &str = r#"{"markdown":"slow answer","notices":["slow notice"],"status":"open","unrecorded_symbols":["f"]}"#;

#[test]
fn hooks_give_up_on_a_slow_daemon_at_their_budget_and_wait_for_it_within_it() {
    let slow = fake_daemon(Fake::Slow(std::time::Duration::from_secs(3), SLOW_ANSWER));
    let budget = |secs| {
        let mut env = every_budget(secs).to_vec();
        env.push(("AV_HOOK_WAIT_SECS", "0"));
        env
    };
    for (hook, o) in every_hook(&slow, &budget("1")) {
        assert_let_through(hook, &o);
    }
    for (hook, o) in every_hook(&slow, &budget("60")) {
        assert!(o.status.success(), "{hook}: {}", err(&o));
        let want = match hook {
            "post-edit" => "slow notice",
            "stop" => "\"decision\":\"block\"",
            _ => "slow answer",
        };
        assert!(out(&o).contains(want), "{hook}: {}", out(&o));
    }
}

#[test]
fn the_stop_hook_gives_up_when_the_daemon_hangs_during_the_wait_for_work() {
    // the checkpoint, `unrecorded` and the first look at the backlog are answered (nothing
    // waits); the next look, 15 s later, hangs
    let url = fake_daemon(Fake::AnswersThenHangs(3, r#"{"would":null}"#));
    let fixture = hook_fixture();
    let o = run_hook(
        &url,
        fixture.path(),
        "stop",
        &[("AV_AGENT", "claude-code"), ("AV_HOOK_STOP_SECS", "2")],
    );
    assert_let_through("stop", &o);
}

#[test]
fn the_stop_hooks_wait_for_work_outlasts_its_budget() {
    let (d, ws, scratch) = started();
    let o = std::thread::scope(|s| {
        let poster = s.spawn(|| {
            // past the 1 s budget of the first look, before the next look 15 s on
            std::thread::sleep(std::time::Duration::from_secs(3));
            post_item(&d, scratch.path(), "Arrives during the wait");
        });
        let o = stop_with(
            &d,
            &ws,
            &[("AV_HOOK_WAIT_SECS", "60"), ("AV_HOOK_STOP_SECS", "1")],
            "cc-wait",
            false,
        );
        poster.join().unwrap();
        o
    });
    assert!(o.status.success(), "{}", err(&o));
    let v: serde_json::Value =
        serde_json::from_str(&out(&o)).unwrap_or_else(|e| panic!("{e}: {} / {}", out(&o), err(&o)));
    assert_eq!(v["decision"], "block", "{v}");
    assert!(
        v["reason"]
            .as_str()
            .unwrap()
            .contains("Arrives during the wait"),
        "{v}"
    );
}

fn every_budget(secs: &'static str) -> [(&'static str, &'static str); 4] {
    [
        ("AV_HOOK_SESSION_START_SECS", secs),
        ("AV_HOOK_PRE_READ_SECS", secs),
        ("AV_HOOK_POST_EDIT_SECS", secs),
        ("AV_HOOK_STOP_SECS", secs),
    ]
}

#[test]
fn after_a_hook_gave_up_the_next_hooks_skip_the_daemon_for_a_while() {
    let url = fake_daemon(Fake::Hang);
    let fixture = hook_fixture();
    let first = run_hook(
        &url,
        fixture.path(),
        "pre-read",
        &[("AV_HOOK_PRE_READ_SECS", "1")],
    );
    assert_let_through("pre-read", &first);
    // given an hour each, these would still wait at HANG unless they skip the daemon
    for hook in HOOKS {
        let o = run_hook(&url, fixture.path(), hook, &every_budget("3600"));
        assert_let_through(hook, &o);
    }
}

#[test]
fn the_skip_names_one_daemon_and_ends_by_itself() {
    let url = fake_daemon(Fake::Slow(std::time::Duration::ZERO, SLOW_ANSWER));
    let fixture = hook_fixture();
    let mark = |marked: &str, age_secs: u64| {
        let home = fixture.path().join("av-home");
        std::fs::create_dir_all(&home).unwrap();
        let at = now_ms() / 1000 - age_secs;
        std::fs::write(
            home.join("hooks-unreachable.json"),
            serde_json::json!({"url": marked, "at": at}).to_string(),
        )
        .unwrap();
    };
    let pre_read = || run_hook(&url, fixture.path(), "pre-read", &[]);
    mark(&url, 0);
    assert_let_through("pre-read", &pre_read());
    mark(&url, 31);
    assert!(out(&pre_read()).contains("slow answer"));
    mark("http://127.0.0.1:9", 0);
    assert!(out(&pre_read()).contains("slow answer"));
}
