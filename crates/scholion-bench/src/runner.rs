//! One benchmark session in one arm: prepare the working directory, run headless Claude Code,
//! verify. Nothing here touches the source repo except `git clone` from it and `git show`.

use std::fs::File;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use globset::Glob;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::Result;
use crate::spec::{RelayNote, TaskSpec, git_in, resolve_rev};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Arm {
    /// E4 control: the base tree without anything the earlier agent learned (no relay commits,
    /// no notes, no records).
    Cold,
    /// Plain git checkout, no av.
    Baseline,
    /// As `Baseline`, plus a `NOTES.md` of what earlier agents learned (E4; `CLAUDE.md` points
    /// at it).
    Notes,
    /// av workspaces, records, hooks and MCP, but packs reduced to the Task section.
    AvNoPacks,
    /// Full av.
    Av,
}

impl Arm {
    /// The E3 arms.
    pub const ALL: [Arm; 3] = [Arm::Baseline, Arm::AvNoPacks, Arm::Av];
    /// Every arm `--arms` accepts (E4 adds `cold` and `notes`).
    pub const KNOWN: [Arm; 5] = [
        Arm::Cold,
        Arm::Baseline,
        Arm::Notes,
        Arm::AvNoPacks,
        Arm::Av,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Arm::Cold => "cold",
            Arm::Baseline => "baseline",
            Arm::Notes => "notes",
            Arm::AvNoPacks => "av_no_packs",
            Arm::Av => "av",
        }
    }

    pub fn parse(s: &str) -> Result<Arm> {
        Arm::KNOWN
            .into_iter()
            .find(|a| a.as_str() == s)
            .ok_or_else(|| {
                format!("unknown arm {s:?}; expected cold, baseline, notes, av_no_packs or av")
            })
    }

    pub fn uses_av(self) -> bool {
        matches!(self, Arm::AvNoPacks | Arm::Av)
    }

    fn short(self) -> &'static str {
        match self {
            Arm::Cold => "cold",
            Arm::Baseline => "base",
            Arm::Notes => "notes",
            Arm::AvNoPacks => "nopacks",
            Arm::Av => "av",
        }
    }
}

#[derive(Debug, Clone)]
pub struct RunConfig {
    /// Throwaway copies and per-session files go under here (`~/bench-work`).
    pub work_root: PathBuf,
    pub claude_bin: String,
    /// The `av` CLI the hooks run.
    pub av_bin: String,
    /// The bench daemon (av arms only).
    pub av_url: Option<String>,
    pub av_token: Option<String>,
    /// Host directory the bench daemon imports from, and the same directory as the daemon sees
    /// it (`/import` in the bench compose stack; the host path itself for a host daemon).
    pub import_dir: PathBuf,
    pub import_mount: String,
    pub allowed_tools: Vec<String>,
    pub permission_mode: String,
    /// Passed to `claude --max-budget-usd`; also what a session without a `result` event costs.
    pub session_cap_usd: f64,
    /// A Claude config dir whose `.credentials.json` is copied into each session's empty one.
    pub claude_home: Option<PathBuf>,
    /// Token budget of the session-start pack.
    pub hook_budget: u32,
    /// Run the av arms even when the bench daemon has no semantic layer (packs then have no
    /// semantic code search, which undersells av).
    pub allow_no_semantic: bool,
    /// How long to wait for a new repo's embeddings to drain before its session starts.
    pub index_wait_secs: u64,
    /// Give the av arms the av MCP server as well as the hooks. Off measures the shipped
    /// Claude Code integration (`integrations/claude-code`), which is hooks only.
    pub av_mcp: bool,
    /// `--code` for the session-start hook (`full` or `signatures`); `None` = the hook's default.
    pub hook_code: Option<String>,
    /// More arguments for the session-start hook, e.g. `--no-map`, `--max-code=3`.
    pub hook_args: Vec<String>,
    /// Also install the just-in-time hooks in the av arms: `PreToolUse` on reads and edits
    /// (`av hook pre-read`: other tasks' records about the file) and `Stop` (`av hook stop`).
    pub jit_hooks: bool,
    /// Cost sessions from their token usage at these prices instead of Claude Code's
    /// `total_cost_usd`, and do not pass `--max-budget-usd` (it is in Claude prices).
    pub prices: Option<crate::metrics::Prices>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunOutcome {
    pub task: String,
    pub arm: Arm,
    pub rep: u32,
    pub run_dir: PathBuf,
    pub events_path: PathBuf,
    pub verify_exit: Option<i32>,
    pub verify_timed_out: bool,
    pub valid: bool,
    pub invalid_reason: Option<String>,
    pub wall_secs: f64,
    pub claude_killed: bool,
    pub claude_exit: Option<i32>,
}

impl RunOutcome {
    pub fn success(&self) -> bool {
        self.valid && self.verify_exit == Some(0)
    }
}

const AGENT: &str = "claude-code";
const SETUP_TIMEOUT: Duration = Duration::from_secs(1800);

/// Variables a parent Claude Code session sets that must not leak into the benchmarked one.
const NESTED_ENV: [&str; 4] = [
    "CLAUDECODE",
    "CLAUDE_CODE_ENTRYPOINT",
    "CLAUDE_CODE_SESSION_ID",
    "CLAUDE_CODE_SSE_PORT",
];

fn sh_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', r"'\''"))
}

fn write_json(path: &Path, v: &Value) -> Result<()> {
    std::fs::write(path, serde_json::to_vec_pretty(v).expect("json"))
        .map_err(|e| format!("{}: {e}", path.display()))
}

fn git_ok(dir: &Path, args: &[&str]) -> Result<()> {
    let out = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args([
            "-c",
            "user.name=scholion-bench",
            "-c",
            "user.email=bench@localhost",
        ])
        .args(["-c", "commit.gpgsign=false"])
        .args(args)
        .output()
        .map_err(|e| format!("git: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "git {}: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(())
}

/// `dest` = the tree of `rev` as a single orphan commit `base` on branch `bench`, with no other
/// refs, no remote, no reflog and no unreachable objects: neither arm can read the real fix or
/// earlier approaches from history.
pub fn scrubbed_copy(repo: &Path, rev: &str, dest: &Path) -> Result<()> {
    let commit = resolve_rev(repo, rev)?;
    if dest.exists() {
        std::fs::remove_dir_all(dest).map_err(|e| format!("{}: {e}", dest.display()))?;
    }
    let out = Command::new("git")
        .args(["clone", "-q", "--no-hardlinks", "--no-tags", "--"])
        .arg(repo)
        .arg(dest)
        .output()
        .map_err(|e| format!("git clone: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "git clone: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    git_ok(dest, &["checkout", "-q", "--detach", &commit])?;
    git_ok(dest, &["checkout", "-q", "--orphan", "bench"])?;
    git_ok(dest, &["commit", "-q", "--allow-empty", "-m", "base"])?;
    let refs = git_in(dest, &["for-each-ref", "--format=%(refname)"])?;
    for r in String::from_utf8_lossy(&refs).lines() {
        if r != "refs/heads/bench" {
            git_ok(dest, &["update-ref", "-d", r])?;
        }
    }
    git_ok(dest, &["remote", "remove", "origin"])?;
    git_ok(dest, &["reflog", "expire", "--expire=now", "--all"])?;
    git_ok(dest, &["gc", "-q", "--prune=now"])?;
    Ok(())
}

/// E4: each relay note becomes an empty commit by the earlier agent on top of the base, its
/// summary the subject and its body the message, so `git log` carries what that agent learned.
pub fn relay_commits(dest: &Path, notes: &[RelayNote]) -> Result<()> {
    for n in notes {
        let mut args = vec![
            "-c",
            "user.name=agent-a",
            "-c",
            "user.email=agent-a@localhost",
            "-c",
            "commit.gpgsign=false",
            "commit",
            "-q",
            "--allow-empty",
            "-m",
            &n.summary,
        ];
        if !n.body.trim().is_empty() {
            args.extend(["-m", &n.body]);
        }
        git_ok(dest, &args)?;
    }
    Ok(())
}

/// E4, `notes` arm: every relay note in `NOTES.md`, and a `CLAUDE.md` that points at it,
/// committed by the earlier agent.
pub fn write_notes_md(dest: &Path, notes: &[RelayNote]) -> Result<()> {
    let mut md = String::from(
        "# Notes\n\nWhat earlier agents learned about this code: decisions to follow and \
         approaches that failed.\n",
    );
    for n in notes {
        let at = match &n.symbol {
            Some(sym) => format!("{}:{sym}", n.path),
            None => n.path.clone(),
        };
        md.push_str(&format!("\n## {} ({at})\n\n{}\n", n.kind, n.summary));
        if !n.body.trim().is_empty() {
            md.push_str(&format!("\n{}\n", n.body.trim()));
        }
    }
    std::fs::write(dest.join("NOTES.md"), md).map_err(|e| e.to_string())?;
    std::fs::write(
        dest.join("CLAUDE.md"),
        "Read NOTES.md before you change code: it records decisions to follow and approaches \
         that already failed.\n",
    )
    .map_err(|e| e.to_string())?;
    git_ok(dest, &["add", "NOTES.md", "CLAUDE.md"])?;
    git_ok(
        dest,
        &[
            "-c",
            "user.name=agent-a",
            "-c",
            "user.email=agent-a@localhost",
            "-c",
            "commit.gpgsign=false",
            "commit",
            "-q",
            "-m",
            "Keep notes for the next agent",
        ],
    )
}

/// E4, av arms: the relay notes as records of an earlier task, anchored to their code, so
/// packs and the pre-read hook can show them to the agent's task.
fn seed_relay_records(spec: &TaskSpec, daemon: &Daemon, repo: &str) -> Result<()> {
    let started = daemon.post(
        "/v1/tasks",
        json!({"repo": repo, "title": "earlier work", "goal": "Earlier work on this code",
               "agent": "agent-a", "session": format!("relay-{}", uuid::Uuid::new_v4())}),
        None,
    )?;
    let task = started["task"]["id"].as_str().ok_or("no task id")?;
    let generation = started["task"]["generation"].as_i64().unwrap_or(1);
    for n in &spec.relay_notes {
        let mut anchor = json!({"path": n.path});
        if let Some(sym) = &n.symbol {
            anchor["symbol"] = json!(sym);
        }
        daemon.post(
            &format!("/v1/tasks/{task}/records"),
            json!({"kind": n.kind, "summary": n.summary, "body": n.body,
                   "anchors": [anchor], "agent": "agent-a"}),
            Some(generation),
        )?;
    }
    Ok(())
}

/// Every file of `rev` exists in `dir` with the same bytes.
pub fn tree_matches(repo: &Path, rev: &str, dir: &Path) -> Result<()> {
    let names = git_in(
        repo,
        &[
            "ls-tree",
            "-r",
            "-z",
            "--name-only",
            "--end-of-options",
            rev,
        ],
    )?;
    for name in names.split(|b| *b == 0).filter(|n| !n.is_empty()) {
        let name = String::from_utf8_lossy(name);
        let want = git_in(
            repo,
            &["show", "--end-of-options", &format!("{rev}:{name}")],
        )?;
        let have = std::fs::read(dir.join(&*name))
            .map_err(|e| format!("workspace is missing {name}: {e}"))?;
        if want != have {
            return Err(format!("workspace file {name} differs from {rev}"));
        }
    }
    Ok(())
}

struct Waited {
    exit: Option<i32>,
    killed: bool,
}

/// Runs `cmd` in its own process group; after `timeout` the whole group is SIGKILLed.
fn run_with_timeout(mut cmd: Command, timeout: Duration) -> Result<Waited> {
    cmd.process_group(0);
    let mut child = cmd.spawn().map_err(|e| format!("spawn: {e}"))?;
    let start = Instant::now();
    loop {
        if let Some(status) = child.try_wait().map_err(|e| e.to_string())? {
            return Ok(Waited {
                exit: status.code(),
                killed: false,
            });
        }
        if start.elapsed() >= timeout {
            let _ = Command::new("kill")
                .args(["-KILL", "--", &format!("-{}", child.id())])
                .status();
            let _ = child.kill();
            let _ = child.wait();
            return Ok(Waited {
                exit: None,
                killed: true,
            });
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

fn shell(script: &str, cwd: &Path, log: &Path) -> Result<Command> {
    let out = File::create(log).map_err(|e| format!("{}: {e}", log.display()))?;
    let err = out.try_clone().map_err(|e| e.to_string())?;
    let mut cmd = Command::new("sh");
    cmd.arg("-c")
        .arg(script)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .stdout(out)
        .stderr(err);
    Ok(cmd)
}

/// The bench daemon's HTTP API.
pub struct Daemon {
    url: String,
    token: Option<String>,
}

impl Daemon {
    pub fn new(url: &str, token: Option<&str>) -> Daemon {
        Daemon {
            url: url.trim_end_matches('/').to_owned(),
            token: token.map(str::to_owned),
        }
    }

    fn req(&self, method: &str, path: &str) -> ureq::Request {
        let r =
            ureq::request(method, &format!("{}{path}", self.url)).timeout(Duration::from_secs(600));
        match &self.token {
            Some(t) => r.set("Authorization", &format!("Bearer {t}")),
            None => r,
        }
    }

    fn call(&self, r: ureq::Request, body: Option<Value>) -> Result<Value> {
        let resp = match body {
            Some(b) => r.send_json(b),
            None => r.call(),
        };
        match resp {
            Ok(resp) => resp.into_json().map_err(|e| e.to_string()),
            Err(ureq::Error::Status(code, resp)) => Err(format!(
                "daemon answered {code}: {}",
                resp.into_string().unwrap_or_default()
            )),
            Err(e) => Err(format!("daemon at {} unreachable: {e}", self.url)),
        }
    }

    pub fn get(&self, path: &str) -> Result<Value> {
        self.call(self.req("GET", path), None)
    }

    pub fn post(&self, path: &str, body: Value, generation: Option<i64>) -> Result<Value> {
        let mut r = self.req("POST", path);
        if let Some(g) = generation {
            r = r.set("X-Av-Generation", &g.to_string());
        }
        self.call(r, Some(body))
    }

    /// Up, and accepting our token: run this before anything is spent.
    pub fn preflight(&self) -> Result<()> {
        ureq::get(&format!("{}/healthz", self.url))
            .timeout(Duration::from_secs(5))
            .call()
            .map_err(|e| format!("av daemon at {} is not up: {e}", self.url))?;
        match self
            .req("GET", "/v1/tasks/00000000-0000-0000-0000-000000000000")
            .call()
        {
            Ok(_) | Err(ureq::Error::Status(404, _)) => Ok(()),
            Err(ureq::Error::Status(401, _)) => Err(format!(
                "av daemon at {} refused the token (401): set AV_TOKEN to the daemon's token",
                self.url
            )),
            Err(e) => Err(format!("av daemon at {}: {e}", self.url)),
        }
    }
}

struct AvTask {
    repo: String,
    id: String,
    generation: i64,
    workspace: PathBuf,
}

fn start_av_task(
    spec: &TaskSpec,
    arm: Arm,
    rep: u32,
    run_id: &str,
    cfg: &RunConfig,
    daemon: &Daemon,
    dir: &Path,
) -> Result<AvTask> {
    let name = format!("b{run_id}-{}-{}-{rep}", spec.name, arm.short());
    let src = cfg.import_dir.join(&name);
    scrubbed_copy(&spec.repo_path, &spec.base_rev, &src)?;
    relay_commits(&src, &spec.relay_notes)?;
    daemon.post(
        "/v1/repos",
        json!({"name": name, "git_source": format!("{}/{name}", cfg.import_mount.trim_end_matches('/'))}),
        None,
    )?;
    if !spec.relay_notes.is_empty() {
        seed_relay_records(spec, daemon, &name)?;
    }
    // The start pack is only kept for calibration (`pack.md`), so it goes to a session of its
    // own: under the agent's session the ledger would mark everything as sent, and the agent's
    // first pack would be nothing but back-references.
    let session = uuid::Uuid::new_v4().to_string();
    let pack_mode = if arm == Arm::AvNoPacks {
        "task_only"
    } else {
        "full"
    };
    let started = daemon.post(
        "/v1/tasks",
        json!({"repo": name, "title": spec.name, "goal": spec.goal, "agent": AGENT,
               "session": format!("bench-start-{session}"), "pack_mode": pack_mode}),
        None,
    )?;
    let task = &started["task"];
    let id = task["id"].as_str().ok_or("no task id")?.to_owned();
    let generation = task["generation"].as_i64().unwrap_or(1);
    let workspace = PathBuf::from(task["workspace_path"].as_str().ok_or("no workspace path")?);
    if let Some(md) = started["pack"]["markdown"].as_str() {
        let _ = std::fs::write(dir.join("pack.md"), md);
    }
    tree_matches(
        &spec.repo_path,
        &resolve_rev(&spec.repo_path, &spec.base_rev)?,
        &workspace,
    )
    .map_err(|e| format!("av workspace {}: {e}", workspace.display()))?;
    write_json(
        &workspace.join(".av-task"),
        &json!({"task": id, "session": session, "generation": generation, "agent": AGENT}),
    )?;
    if spec.relay_notes.is_empty() {
        seed_dead_ends(spec, daemon, &id, generation)?;
    }
    Ok(AvTask {
        repo: name,
        id,
        generation,
        workspace,
    })
}

/// With a semantic layer, waits until the repo's embeddings are in the vector index, so the
/// session's first pack can use semantic search; without one, refuses unless allowed.
fn wait_for_index(daemon: &Daemon, repo: &str, cfg: &RunConfig) -> Result<()> {
    let start = Instant::now();
    loop {
        let s = daemon.get(&format!("/v1/repos/{repo}/outbox"))?;
        if s["semantic"] != true {
            if cfg.allow_no_semantic {
                return Ok(());
            }
            return Err(
                "the bench daemon has no semantic layer (embedder + Qdrant), so av packs would \
                 have no semantic code search; fix the stack or pass --allow-no-semantic"
                    .into(),
            );
        }
        if s["pending"].as_i64() == Some(0) {
            return Ok(());
        }
        if start.elapsed() >= Duration::from_secs(cfg.index_wait_secs) {
            return Err(format!(
                "repo {repo} still has {} embeddings queued after {} s; is the embedder up?",
                s["pending"], cfg.index_wait_secs
            ));
        }
        std::thread::sleep(Duration::from_secs(2));
    }
}

/// Each `[[dead_end]]` becomes an `attempt` record anchored to the base files its glob matches.
fn seed_dead_ends(spec: &TaskSpec, daemon: &Daemon, task: &str, generation: i64) -> Result<()> {
    if spec.dead_ends.is_empty() {
        return Ok(());
    }
    let names = git_in(
        &spec.repo_path,
        &[
            "ls-tree",
            "-r",
            "--name-only",
            "--end-of-options",
            &spec.base_rev,
        ],
    )?;
    let names = String::from_utf8_lossy(&names);
    for d in &spec.dead_ends {
        let glob = Glob::new(&d.file_glob)
            .map_err(|e| e.to_string())?
            .compile_matcher();
        let anchors: Vec<Value> = names
            .lines()
            .filter(|n| glob.is_match(n))
            .take(20)
            .map(|n| json!({"path": n}))
            .collect();
        let summary: String = d.description.chars().take(200).collect();
        daemon.post(
            &format!("/v1/tasks/{task}/records"),
            json!({"kind": "attempt", "summary": summary, "body": d.description,
                   "anchors": anchors, "agent": AGENT}),
            Some(generation),
        )?;
    }
    Ok(())
}

fn settings(arm: Arm, cfg: &RunConfig, dir: &Path) -> Value {
    if !arm.uses_av() {
        return json!({});
    }
    let count = sh_quote(&dir.join("hook-count").to_string_lossy());
    let av = sh_quote(&cfg.av_bin);
    let code = cfg
        .hook_code
        .as_deref()
        .map(|c| format!(" --code {}", sh_quote(c)))
        .unwrap_or_default();
    let extra: String = cfg
        .hook_args
        .iter()
        .map(|a| format!(" {}", sh_quote(a)))
        .collect();
    let start = format!(
        "echo session-start >> {count}; {av} hook session-start --budget {}{code}{extra}",
        cfg.hook_budget
    );
    let edit = format!("echo post-edit >> {count}; {av} hook post-edit");
    let mut hooks = json!({
        "SessionStart": [{"hooks": [{"type": "command", "command": start}]}],
        "PostToolUse": [{"matcher": "Edit|Write|MultiEdit|NotebookEdit",
                         "hooks": [{"type": "command", "command": edit}]}]
    });
    if cfg.jit_hooks {
        let read = format!("echo pre-read >> {count}; {av} hook pre-read");
        let stop = format!("echo stop >> {count}; {av} hook stop");
        hooks["PreToolUse"] = json!([{"matcher": "Read|Edit|MultiEdit",
                                      "hooks": [{"type": "command", "command": read}]}]);
        hooks["Stop"] = json!([{"hooks": [{"type": "command", "command": stop}]}]);
    }
    json!({ "hooks": hooks })
}

fn mcp_config(arm: Arm, cfg: &RunConfig, task: Option<&AvTask>) -> Value {
    let (Some(url), Some(task), true) = (&cfg.av_url, task, cfg.av_mcp) else {
        return json!({"mcpServers": {}});
    };
    debug_assert!(arm.uses_av());
    let mut headers = json!({"X-Av-Task": task.id, "X-Av-Agent": AGENT});
    if let Some(t) = &cfg.av_token {
        headers["Authorization"] = json!(format!("Bearer {t}"));
    }
    json!({"mcpServers": {"av": {"type": "http",
        "url": format!("{}/mcp", url.trim_end_matches('/')), "headers": headers}}})
}

fn invalid(mut o: RunOutcome, reason: String) -> RunOutcome {
    o.valid = false;
    o.invalid_reason = Some(reason);
    o
}

/// Runs one session. `Err` means the run as a whole should stop (the daemon is down, the
/// workspace does not match the base tree): nothing was spent on this session. A session that
/// ran but must not count comes back `valid: false`.
pub fn run_session(
    spec: &TaskSpec,
    arm: Arm,
    rep: u32,
    run_id: &str,
    cfg: &RunConfig,
) -> Result<RunOutcome> {
    let dir = cfg.work_root.join("runs").join(run_id).join(format!(
        "{}-{}-r{rep}",
        spec.name,
        arm.as_str()
    ));
    if dir.exists() {
        std::fs::remove_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    }
    std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let mut outcome = RunOutcome {
        task: spec.name.clone(),
        arm,
        rep,
        run_dir: dir.clone(),
        events_path: dir.join("events.jsonl"),
        verify_exit: None,
        verify_timed_out: false,
        valid: true,
        invalid_reason: None,
        wall_secs: 0.0,
        claude_killed: false,
        claude_exit: None,
    };

    let (workspace, av_task, daemon) = if arm.uses_av() {
        let url = cfg.av_url.as_deref().ok_or("the av arms need --av-url")?;
        let daemon = Daemon::new(url, cfg.av_token.as_deref());
        daemon.preflight()?;
        let t = start_av_task(spec, arm, rep, run_id, cfg, &daemon, &dir)?;
        wait_for_index(&daemon, &t.repo, cfg)?;
        (t.workspace.clone(), Some(t), Some(daemon))
    } else {
        let ws = dir.join("ws");
        scrubbed_copy(&spec.repo_path, &spec.base_rev, &ws)?;
        if arm != Arm::Cold {
            relay_commits(&ws, &spec.relay_notes)?;
        }
        if arm == Arm::Notes {
            write_notes_md(&ws, &spec.relay_notes)?;
        }
        tree_matches(
            &spec.repo_path,
            &resolve_rev(&spec.repo_path, &spec.base_rev)?,
            &ws,
        )?;
        (ws, None, None)
    };

    let settings_path = dir.join("settings.json");
    let mcp_path = dir.join("mcp.json");
    write_json(&settings_path, &settings(arm, cfg, &dir))?;
    write_json(&mcp_path, &mcp_config(arm, cfg, av_task.as_ref()))?;
    let config_dir = dir.join("claude-config");
    std::fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    if let Some(home) = &cfg.claude_home {
        let creds = home.join(".credentials.json");
        if creds.exists() {
            std::fs::copy(&creds, config_dir.join(".credentials.json"))
                .map_err(|e| format!("{}: {e}", creds.display()))?;
        }
    }

    if !spec.setup_cmd.trim().is_empty() {
        let w = run_with_timeout(
            shell(&spec.setup_cmd, &workspace, &dir.join("setup.log"))?,
            SETUP_TIMEOUT,
        )?;
        if w.exit != Some(0) {
            return Ok(invalid(
                outcome,
                format!("setup_cmd failed (exit {:?})", w.exit),
            ));
        }
    }

    let mut tools = cfg.allowed_tools.clone();
    if arm.uses_av() && cfg.av_mcp {
        tools.push("mcp__av".into());
    }
    let mut cmd = Command::new(&cfg.claude_bin);
    cmd.arg("-p")
        .arg(&spec.goal)
        .args(["--output-format", "stream-json", "--verbose"])
        .args(["--max-turns", &spec.max_turns.to_string()])
        .args(["--model", &spec.model])
        .args(["--setting-sources", "project"])
        .arg("--settings")
        .arg(&settings_path)
        .arg("--strict-mcp-config")
        .arg("--mcp-config")
        .arg(&mcp_path)
        .args(["--permission-mode", &cfg.permission_mode])
        .arg("--no-session-persistence")
        .arg("--allowedTools")
        .args(&tools)
        .current_dir(&workspace)
        .stdin(Stdio::null())
        .stdout(File::create(&outcome.events_path).map_err(|e| e.to_string())?)
        .stderr(File::create(dir.join("claude.stderr")).map_err(|e| e.to_string())?)
        .env("CLAUDE_CONFIG_DIR", &config_dir)
        .env("AV_AGENT", AGENT);
    if !spec.path_prepend.is_empty() {
        let inherited = std::env::var_os("PATH").unwrap_or_default();
        let dirs = spec
            .path_prepend
            .iter()
            .cloned()
            .chain(std::env::split_paths(&inherited));
        cmd.env(
            "PATH",
            std::env::join_paths(dirs).map_err(|e| format!("path_prepend: {e}"))?,
        );
    }
    if cfg.prices.is_none() {
        cmd.args(["--max-budget-usd", &format!("{:.2}", cfg.session_cap_usd)]);
    }
    for v in NESTED_ENV {
        cmd.env_remove(v);
    }
    if let Some(url) = &cfg.av_url {
        cmd.env("AV_URL", url);
    }
    match &cfg.av_token {
        Some(t) => cmd.env("AV_TOKEN", t),
        None => cmd.env_remove("AV_TOKEN"),
    };
    let started = Instant::now();
    let w = run_with_timeout(cmd, Duration::from_secs(spec.session_timeout_secs))?;
    outcome.wall_secs = started.elapsed().as_secs_f64();
    outcome.claude_killed = w.killed;
    outcome.claude_exit = w.exit;

    if let (Some(t), Some(daemon)) = (&av_task, &daemon) {
        daemon.post(
            &format!("/v1/tasks/{}/checkpoint", t.id),
            json!({}),
            Some(t.generation),
        )?;
        let fired = std::fs::read_to_string(dir.join("hook-count"))
            .map(|s| s.lines().count())
            .unwrap_or(0);
        if fired == 0 {
            outcome = invalid(outcome, "the av hooks never fired".into());
        }
    }

    // after the last checkpoint, so the hidden tests never enter av's history
    if !spec.hidden_tests.is_empty() {
        let fix = spec
            .fix_rev
            .as_deref()
            .ok_or("hidden_tests needs fix_rev")?;
        for f in &spec.hidden_tests {
            let bytes = git_in(
                &spec.repo_path,
                &["show", "--end-of-options", &format!("{fix}:{f}")],
            )?;
            let dest = workspace.join(f);
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            std::fs::write(&dest, bytes).map_err(|e| format!("{}: {e}", dest.display()))?;
        }
    }
    for t in &spec.local_tests {
        let src = spec.spec_dir.join(&t.source);
        let dest = workspace.join(&t.path);
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        std::fs::copy(&src, &dest).map_err(|e| format!("{}: {e}", src.display()))?;
    }
    let v = run_with_timeout(
        shell(&spec.verify_cmd, &workspace, &dir.join("verify.log"))?,
        Duration::from_secs(spec.verify_timeout_secs),
    )?;
    outcome.verify_exit = v.exit;
    outcome.verify_timed_out = v.killed;
    write_json(
        &dir.join("outcome.json"),
        &serde_json::to_value(&outcome).expect("json"),
    )?;
    Ok(outcome)
}
