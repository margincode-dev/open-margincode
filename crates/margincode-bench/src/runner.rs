//! One benchmark session in one arm: prepare the working directory, run the agent CLI, verify.
//! Claude Code is the default. Grok is `grok -p … --output-format json --always-approve
//! --max-turns N --cwd`, with no `--max-budget-usd` (the turn cap stands in for it).
//! Nothing here touches the source repo except `git clone` from it and `git show`.

use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use globset::Glob;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use margincode_agentrun::AgentCommand;
use margincode_agentrun::process::{run_with_timeout, shell};

use crate::Result;
use crate::events::{ParsedSession, ResultEvent, Usage};
use crate::isolate::{self, AgentUser};
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

/// Which agent CLI a session runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Vendor {
    #[default]
    Claude,
    Grok,
}

impl Vendor {
    pub fn as_str(self) -> &'static str {
        match self {
            Vendor::Claude => "claude",
            Vendor::Grok => "grok",
        }
    }
}

#[derive(Debug, Clone)]
pub struct RunConfig {
    /// Throwaway copies and per-session files go under here (`~/bench-work`).
    pub work_root: PathBuf,
    pub claude_bin: String,
    /// The `grok` binary when [`Vendor`] is `Grok`.
    pub grok_bin: String,
    /// Claude Code, or Grok. Grok is never passed `--max-budget-usd`.
    pub vendor: Vendor,
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
    /// Keep each session's clone after verify. Off: a valid session's clone is removed (the
    /// git arms' `ws`, the av arms' import copy) and an av task is abandoned, so the bench
    /// daemon reclaims its workspace; logs, events, `pack.md` and `outcome.json` stay. An
    /// invalid session keeps everything, for debugging.
    pub keep_work: bool,
    /// Start the agent CLI as this user, which can read only its own working copy
    /// (`run --agent-user`, see `isolate`). `None`: as this process's own user, as before.
    pub agent_user: Option<AgentUser>,
    /// The bench daemon's workspace root as this process sees it (`AV_WS_ROOT`). Only read with
    /// `agent_user` and an av arm: the preflight looks at it before the first session.
    pub av_ws_root: Option<PathBuf>,
    /// Plan 16 Phase 1 (M1): seed the pool's `no_commit` notes (the wrong-lesson pool's wrong
    /// notes) as the records of an earlier task whose merge was then reverted, so their
    /// standing is `reverted`; the other notes stay the records of an open earlier task.
    /// Off: every note is the open task's, as before.
    pub wrong_reverted: bool,
    /// Plan 16 M2 stage B: a file `margincode-bench dream-freeze` wrote. Each av session's store
    /// gets that file's verdicts and reviews for its task once it is seeded, before the agent's
    /// task starts ([`crate::dreamed::replay`]). `None`: the raw store, as before.
    pub dreamed: Option<PathBuf>,
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
    /// cl100k tokens of `NOTES.md` (`notes`) or of the pack the av task starts with (`pack.md`,
    /// av arms), measured before the session; `None` in the other arms.
    #[serde(default)]
    pub memory_tokens: Option<u64>,
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
            "user.name=margincode-bench",
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

/// E4: each non-noise relay note becomes an empty commit by the earlier agent on top of the
/// base, its summary the subject and its body the message, so `git log` carries what that
/// agent learned. Noise notes (E4-at-scale filler) and notes marked `no_commit` skip this: they
/// still land in `NOTES.md` and as av records.
pub fn relay_commits(dest: &Path, notes: &[RelayNote]) -> Result<()> {
    for n in notes.iter().filter(|n| !n.noise && !n.no_commit) {
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
    std::fs::write(dest.join("NOTES.md"), notes_md(notes)).map_err(|e| e.to_string())?;
    commit_notes_md(dest)
}

/// The `NOTES.md` of the `notes` arm: every relay note, in pool order.
pub fn notes_md(notes: &[RelayNote]) -> String {
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
    md
}

/// The `CLAUDE.md` that points at `NOTES.md`, and one commit of both by the earlier agent.
fn commit_notes_md(dest: &Path) -> Result<()> {
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
/// Seeds the relay notes as an earlier task's records and returns each one's `(pair, record id)`,
/// in pool order. With `wrong_reverted`, the `no_commit` notes are a second earlier task's,
/// whose merge is reverted before the agent's task starts ([`reverted_task`]).
fn seed_relay_records(
    spec: &TaskSpec,
    daemon: &Daemon,
    repo: &str,
    wrong_reverted: bool,
) -> Result<Seeded> {
    let (task, generation) = earlier_task(daemon, repo, "agent-a", "earlier work")?;
    let reverted = if wrong_reverted && spec.relay_notes.iter().any(|n| n.no_commit) {
        Some(earlier_task(
            daemon,
            repo,
            "agent-r",
            "earlier work, reverted",
        )?)
    } else {
        None
    };
    let mut seeded = Vec::with_capacity(spec.relay_notes.len());
    for n in &spec.relay_notes {
        let (t, g, agent) = match &reverted {
            Some((rt, rg)) if n.no_commit => (rt.as_str(), *rg, "agent-r"),
            _ => (task.as_str(), generation, "agent-a"),
        };
        let mut anchor = json!({"path": n.path});
        if let Some(sym) = &n.symbol {
            anchor["symbol"] = json!(sym);
        }
        let rec = daemon.post(
            &format!("/v1/tasks/{t}/records"),
            json!({"kind": n.kind, "summary": n.summary, "body": n.body,
                   "anchors": [anchor], "agent": agent}),
            Some(g),
        )?;
        seeded.push((
            n.pair.clone(),
            rec["id"].as_str().unwrap_or_default().to_owned(),
        ));
    }
    if let Some((t, g)) = &reverted {
        reverted_task(daemon, t, *g)?;
    }
    let mut tasks = vec![(OPEN_TASK.to_owned(), task)];
    if let Some((t, _)) = reverted {
        tasks.push((REVERTED_TASK.to_owned(), t));
    }
    Ok(Seeded {
        notes: seeded,
        tasks,
    })
}

/// What [`seed_relay_records`] made: the notes' records and the earlier tasks that hold them.
#[derive(Debug, Clone, Default)]
pub struct Seeded {
    /// `(pair, record id)` of every relay note, in pool order.
    pub notes: Vec<(String, String)>,
    /// `(role, task id)` of each earlier task: [`OPEN_TASK`], and [`REVERTED_TASK`] with
    /// `wrong_reverted` when the pool has a `no_commit` note.
    pub tasks: Vec<(String, String)>,
}

/// The role of the earlier task whose records are the pool's notes.
pub const OPEN_TASK: &str = "open";
/// The role of the earlier task whose merge was reverted (`wrong_reverted`).
pub const REVERTED_TASK: &str = "reverted";

/// A task's store as every av session (and `dream-freeze`) starts it: the scrubbed copy of the
/// base revision with the relay commits, imported as `name`, and the relay notes seeded as
/// earlier tasks' records. Returns the copy's directory and what was seeded.
pub fn seed_store(
    spec: &TaskSpec,
    cfg: &RunConfig,
    daemon: &Daemon,
    name: &str,
) -> Result<(PathBuf, Seeded)> {
    let src = cfg.import_dir.join(name);
    scrubbed_copy(&spec.repo_path, &spec.base_rev, &src)?;
    relay_commits(&src, &spec.relay_notes)?;
    daemon.add_repo(
        name,
        &format!("{}/{name}", cfg.import_mount.trim_end_matches('/')),
    )?;
    let seeded = if spec.relay_notes.is_empty() {
        Seeded::default()
    } else {
        seed_relay_records(spec, daemon, name, cfg.wrong_reverted)?
    };
    Ok((src, seeded))
}

/// Starts an earlier task as `agent`: (id, generation).
fn earlier_task(daemon: &Daemon, repo: &str, agent: &str, title: &str) -> Result<(String, i64)> {
    let started = daemon.post(
        "/v1/tasks",
        json!({"repo": repo, "title": title, "goal": "Earlier work on this code",
               "agent": agent, "session": format!("relay-{}", uuid::Uuid::new_v4())}),
        None,
    )?;
    let task = started["task"]["id"]
        .as_str()
        .ok_or("no task id")?
        .to_owned();
    Ok((task, started["task"]["generation"].as_i64().unwrap_or(1)))
}

/// The file the reverted task adds and its revert removes again, so trunk's tree is the base's.
pub const REVERTED_FILE: &str = "AV_BENCH_REVERTED.md";

/// Takes the task through submit, a passing validation, approval and merge, then reverts the
/// merge: its records' standing becomes `reverted`. Its change is one new file
/// ([`REVERTED_FILE`]), so after the revert trunk has the base's tree again.
fn reverted_task(daemon: &Daemon, task: &str, generation: i64) -> Result<()> {
    let t = daemon.get(&format!("/v1/tasks/{task}"))?;
    let ws = PathBuf::from(
        t["task"]["workspace_path"]
            .as_str()
            .ok_or("the reverted task has no workspace path")?,
    );
    std::fs::write(
        ws.join(REVERTED_FILE),
        "Work that was merged, then reverted.\n",
    )
    .map_err(|e| format!("{}: {e}", ws.display()))?;
    let g = Some(generation);
    daemon.post(
        &format!("/v1/tasks/{task}/submit"),
        json!({"agent": "agent-r"}),
        g,
    )?;
    daemon.post(
        &format!("/v1/tasks/{task}/validate"),
        json!({"agent": "agent-r", "command": "bench", "exit_code": 0}),
        g,
    )?;
    daemon.post(
        &format!("/v1/tasks/{task}/review"),
        json!({"reviewer": "bench-reviewer", "verdict": "approve"}),
        None,
    )?;
    let merged = daemon.post(
        &format!("/v1/tasks/{task}/merge"),
        json!({"agent": "bench-merger"}),
        None,
    )?;
    if merged["status"] != "merged" {
        return Err(format!("the reverted task did not merge: {merged}"));
    }
    daemon.post(
        &format!("/v1/tasks/{task}/revert"),
        json!({"agent": "bench-merger", "reason": "the bench seeds this work as reverted"}),
        None,
    )?;
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

    /// Imports a repo and waits for it: the import runs as a job on the daemon, so no request
    /// timeout limits how large the repo may be.
    pub fn add_repo(&self, name: &str, git_source: &str) -> Result<Value> {
        self.post(
            "/v1/repos",
            json!({"name": name, "git_source": git_source, "wait": false}),
            None,
        )?;
        loop {
            let job = self.get(&format!("/v1/repos/{name}/import"))?;
            match job["state"].as_str() {
                Some("running") => std::thread::sleep(Duration::from_secs(1)),
                Some("done") => return Ok(job),
                _ => {
                    return Err(format!(
                        "import of {name} failed: {}",
                        job["error"].as_str().unwrap_or("no reason given")
                    ));
                }
            }
        }
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
    /// The scrubbed copy the bench daemon imported the repo from.
    source: PathBuf,
    /// `(pair, record id)` of every relay note seeded as a record, in pool order.
    seeded: Vec<(String, String)>,
    /// The pack the daemon answered the task start with (what `pack.md` holds).
    start_pack: Value,
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
    let (src, seeded) = seed_store(spec, cfg, daemon, &name)?;
    if let Some(file) = &cfg.dreamed {
        let frozen = crate::dreamed::Frozen::load(file)?;
        crate::dreamed::replay(
            frozen.task(spec, cfg.wrong_reverted)?,
            spec,
            daemon,
            &name,
            &seeded,
        )?;
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
        source: src,
        seeded: seeded.notes,
        start_pack: started["pack"].clone(),
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

/// What the hooks themselves write (`hook-count`, `pre-read.out`), by name.
const HOOK_FILES: [&str; 2] = ["hook-count", "pre-read.out"];

/// The session's settings file. The hook commands write `HOOK_FILES` into `hook_dir`, which
/// must be a directory the user the agent CLI runs as can write to.
fn settings(arm: Arm, cfg: &RunConfig, hook_dir: &Path) -> Value {
    if !arm.uses_av() {
        return json!({});
    }
    let count = sh_quote(&hook_dir.join("hook-count").to_string_lossy());
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
        // Claude Code's transcript keeps a SessionStart hook's output but not a PreToolUse
        // hook's, so what the agent was told about a file is kept here (`pre-read.out`).
        let said = sh_quote(&hook_dir.join("pre-read.out").to_string_lossy());
        let read = format!("echo pre-read >> {count}; {av} hook pre-read | tee -a {said}");
        // The prefix is on the hook command: a bench backlog is empty, and the default
        // wait is 600s. A Stop hook that does not inherit the session environment would
        // otherwise stall the run.
        let stop = format!("echo stop >> {count}; AV_HOOK_WAIT_SECS=0 {av} hook stop");
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
    let runs = cfg.work_root.join("runs");
    let run_dir = runs.join(run_id);
    let dir = run_dir.join(format!("{}-{}-r{rep}", spec.name, arm.as_str()));
    if dir.exists() {
        std::fs::remove_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    }
    std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let user = cfg.agent_user.as_ref();
    let mut lent = user.map(Lent::new);
    if let Some(lent) = &mut lent {
        // the agent passes through these to its working copy and may list none of them; the
        // session's own directory is closed again as soon as the agent is done
        lent.close.push(dir.clone());
        for d in [&runs, &run_dir, &dir] {
            isolate::set_mode(d, 0o711)?;
        }
        if arm.uses_av() {
            isolate::private_dir(&cfg.import_dir)?;
        }
    }
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
        memory_tokens: None,
    };

    let (workspace, av_task, daemon) = if arm.uses_av() {
        let url = cfg.av_url.as_deref().ok_or("the av arms need --av-url")?;
        let daemon = Daemon::new(url, cfg.av_token.as_deref());
        daemon.preflight()?;
        let t = start_av_task(spec, arm, rep, run_id, cfg, &daemon, &dir)?;
        if let Some(lent) = &mut lent {
            // the daemon's directory of this repo holds the workspace and nothing a later
            // session may see: closed with the session
            if let Some(repo_dir) = t.workspace.parent() {
                lent.close.push(repo_dir.to_path_buf());
            }
            lent.trees.push(t.workspace.clone());
            daemon_workspace_can_be_lent(&t.workspace, cfg)?;
            let _ = isolate::set_mode(&dir.join("pack.md"), 0o600);
        }
        wait_for_index(&daemon, &t.repo, cfg)?;
        (t.workspace.clone(), Some(t), Some(daemon))
    } else {
        let ws = dir.join("ws");
        if let Some(lent) = &mut lent {
            lent.trees.push(ws.clone());
        }
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

    let memory = match arm {
        Arm::Notes => Some(workspace.join("NOTES.md")),
        a if a.uses_av() => Some(dir.join("pack.md")),
        _ => None,
    };
    outcome.memory_tokens = memory
        .and_then(|p| std::fs::read_to_string(p).ok())
        .map(|t| crate::calibrate::cl100k_tokens(&t));

    // The hooks are run by the agent CLI, so with an agent user they write their files
    // (`HOOK_FILES`) in a directory of its own (its home), and those are copied to the session
    // directory once the session is over.
    let home = dir.join("home");
    let hook_dir = match user {
        Some(_) => &home,
        None => &dir,
    };
    let settings_path = dir.join("settings.json");
    let mcp_path = dir.join("mcp.json");
    write_json(&settings_path, &settings(arm, cfg, hook_dir))?;
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
        if user.is_some() {
            let _ = isolate::set_mode(&dir.join("setup.log"), 0o600);
        }
        if w.exit != Some(0) {
            return Ok(invalid(
                outcome,
                format!("setup_cmd failed (exit {:?})", w.exit),
            ));
        }
    }

    let stderr_path = dir.join(match cfg.vendor {
        Vendor::Claude => "claude.stderr",
        Vendor::Grok => "grok.stderr",
    });
    if let (Some(user), Some(lent)) = (user, &mut lent) {
        // The harness is done preparing: from here to the take-back it runs nothing in the
        // working copy (git would refuse a repository that is another user's).
        // The transcript and stderr are written through this process's handles, so the agent
        // needs no access to the files, its own included.
        isolate::private_file(&outcome.events_path)?;
        isolate::private_file(&stderr_path)?;
        let tmp = home.join("tmp");
        std::fs::create_dir_all(&tmp).map_err(|e| format!("{}: {e}", tmp.display()))?;
        lent.trees.extend([config_dir.clone(), home.clone()]);
        if av_task.is_some()
            && let (Some(repo_dir), Some(slug)) = (workspace.parent(), workspace.file_name())
        {
            // `av hook session-start` makes the task's scratch directory beside the workspace,
            // in a directory of the daemon's where the agent user cannot: made here for it
            let scratch = repo_dir.join(".scratch").join(slug);
            std::fs::create_dir_all(&scratch).map_err(|e| format!("{}: {e}", scratch.display()))?;
            lent.trees.push(scratch);
            isolate::set_mode(repo_dir, 0o711)?;
        }
        for tree in &lent.trees {
            user.give(tree)?;
        }
    }
    let ran = {
        let paths = SessionPaths {
            workspace: &workspace,
            events: &outcome.events_path,
            stderr: &stderr_path,
        };
        let mut cmd = match cfg.vendor {
            Vendor::Claude => claude_command(
                spec,
                arm,
                cfg,
                &paths,
                &settings_path,
                &mcp_path,
                &config_dir,
            ),
            Vendor::Grok => grok_command(spec, cfg, &paths),
        };
        if let Some(user) = user {
            // a home of its own: the harness's is root's, and closed to it
            cmd = cmd
                .run_as(user.uid, user.gid)
                .env("HOME", &home)
                .env("TMPDIR", home.join("tmp"))
                .env("USER", &user.name)
                .env("LOGNAME", &user.name);
            for var in HOME_ENV {
                cmd = cmd.env_remove(var);
            }
        }
        cmd.run()
    };
    if let Some(lent) = &mut lent {
        // before the checkpoint, the hidden tests and verify: nothing of the agent's runs any
        // more, and the working copy is root's alone
        lent.take_back()?;
        for name in HOOK_FILES {
            keep_hook_file(&home.join(name), &dir.join(name))?;
        }
    }
    let ran = ran?;
    outcome.wall_secs = ran.wall.as_secs_f64();
    outcome.claude_killed = ran.killed;
    outcome.claude_exit = ran.exit;

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
            let dest = test_dest(&workspace, f, user)?;
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            std::fs::write(&dest, bytes).map_err(|e| format!("{}: {e}", dest.display()))?;
        }
    }
    for t in &spec.local_tests {
        let src = spec.spec_dir.join(&t.source);
        let dest = test_dest(&workspace, &t.path, user)?;
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
    if !cfg.keep_work && outcome.valid {
        remove_work(&workspace, av_task.as_ref(), daemon.as_ref());
    }
    Ok(outcome)
}

/// After the take-back: what the hooks wrote in the agent user's home is kept in the session
/// directory, the harness's alone. The agent owned that home, so only a regular file is copied:
/// a link it left under the name is not followed, nor a device or pipe read.
fn keep_hook_file(from: &Path, to: &Path) -> Result<()> {
    match std::fs::symlink_metadata(from) {
        Ok(m) if m.is_file() => {}
        _ => return Ok(()),
    }
    std::fs::copy(from, to).map_err(|e| format!("{}: {e}", from.display()))?;
    isolate::set_mode(to, 0o600)
}

/// Variables that point a program at the harness user's home, which the agent user cannot
/// enter; without them a program falls back to `HOME`, which is the session's own.
const HOME_ENV: [&str; 5] = [
    "XDG_CONFIG_HOME",
    "XDG_CACHE_HOME",
    "XDG_DATA_HOME",
    "XDG_STATE_HOME",
    "XDG_RUNTIME_DIR",
];

/// What a session lends the agent user (`--agent-user`), taken back when the agent is done and,
/// through `Drop`, however the session ends: a session that stops half way must not leave the
/// next one a directory to read.
struct Lent<'a> {
    user: &'a AgentUser,
    /// Handed to the user for the session: the working copy, its config dir, its home.
    trees: Vec<PathBuf>,
    /// Directories the user passes through during the session, closed to it afterwards.
    close: Vec<PathBuf>,
    taken_back: bool,
}

impl<'a> Lent<'a> {
    fn new(user: &'a AgentUser) -> Self {
        Lent {
            user,
            trees: vec![],
            close: vec![],
            taken_back: false,
        }
    }

    /// Kills what the user still runs, then makes every tree root's alone and closes the
    /// directories.
    fn take_back(&mut self) -> Result<()> {
        self.user.kill_processes()?;
        for tree in &self.trees {
            isolate::take_back(tree)?;
        }
        for dir in &self.close {
            if dir.is_dir() {
                isolate::set_mode(dir, 0o700)?;
            }
        }
        self.taken_back = true;
        Ok(())
    }
}

impl Drop for Lent<'_> {
    fn drop(&mut self) {
        if !self.taken_back
            && let Err(e) = self.take_back()
        {
            eprintln!("warning: {e}");
        }
    }
}

/// With an agent user, the daemon's workspace is lent to it for the session. That needs the
/// workspace where the preflight looked (under `av_ws_root`) and a daemon that runs as root, so
/// that it still snapshots a workspace whose files are another user's.
fn daemon_workspace_can_be_lent(workspace: &Path, cfg: &RunConfig) -> Result<()> {
    let real = |p: &Path| p.canonicalize().unwrap_or_else(|_| p.to_path_buf());
    if let Some(root) = &cfg.av_ws_root
        && !real(workspace).starts_with(real(root))
    {
        return Err(format!(
            "--agent-user: the daemon's workspace {} is not under --av-ws-root {}, which is \
             what was checked before the first session",
            workspace.display(),
            root.display()
        ));
    }
    let owner = std::fs::metadata(workspace)
        .map_err(|e| format!("{}: {e}", workspace.display()))?
        .uid();
    if owner != 0 {
        return Err(format!(
            "--agent-user needs a bench daemon that runs as root: its workspace {} belongs to \
             uid {owner}, and that daemon could not write to a workspace lent to the agent user",
            workspace.display()
        ));
    }
    Ok(())
}

/// Where verify's hidden test `rel` goes in the working copy. After an agent user wrote there,
/// never through a symlink it left, and never into a file it made (`isolate::fresh_dest`).
fn test_dest(workspace: &Path, rel: &str, user: Option<&AgentUser>) -> Result<PathBuf> {
    match user {
        Some(_) => isolate::fresh_dest(workspace, rel),
        None => Ok(workspace.join(rel)),
    }
}

/// After verify: the git arms' clone goes; an av task's import copy goes and the task is
/// abandoned, so the bench daemon's retention reclaims its workspace. Failures are warnings:
/// the session's result already stands.
fn remove_work(workspace: &Path, av_task: Option<&AvTask>, daemon: Option<&Daemon>) {
    let remove = |p: &Path| {
        if p.exists()
            && let Err(e) = std::fs::remove_dir_all(p)
        {
            eprintln!("warning: could not remove {}: {e}", p.display());
        }
    };
    match (av_task, daemon) {
        (Some(t), Some(daemon)) => {
            remove(&t.source);
            if let Err(e) = daemon.post(
                &format!("/v1/tasks/{}/abandon", t.id),
                json!({"agent": AGENT}),
                Some(t.generation),
            ) {
                eprintln!("warning: could not abandon bench task {}: {e}", t.id);
            }
        }
        _ => remove(workspace),
    }
}

fn bench_env(mut cmd: AgentCommand, cfg: &RunConfig, agent: &str) -> AgentCommand {
    cmd = cmd.env("AV_AGENT", agent).env("AV_HOOK_WAIT_SECS", "0");
    for v in NESTED_ENV {
        cmd = cmd.env_remove(v);
    }
    if let Some(url) = &cfg.av_url {
        cmd = cmd.env("AV_URL", url);
    }
    match &cfg.av_token {
        Some(t) => cmd.env("AV_TOKEN", t),
        None => cmd.env_remove("AV_TOKEN"),
    }
}

struct SessionPaths<'a> {
    workspace: &'a Path,
    events: &'a Path,
    stderr: &'a Path,
}

fn claude_command(
    spec: &TaskSpec,
    arm: Arm,
    cfg: &RunConfig,
    paths: &SessionPaths<'_>,
    settings_path: &Path,
    mcp_path: &Path,
    config_dir: &Path,
) -> AgentCommand {
    let mut tools = cfg.allowed_tools.clone();
    if arm.uses_av() && cfg.av_mcp {
        tools.push("mcp__av".into());
    }
    let mut cmd = AgentCommand::new(&cfg.claude_bin)
        .arg("-p")
        .arg(&spec.goal)
        .args(["--output-format", "stream-json", "--verbose"])
        .args(["--max-turns", &spec.max_turns.to_string()])
        .args(["--model", &spec.model])
        .args(["--setting-sources", "project"])
        .arg("--settings")
        .arg(settings_path.as_os_str())
        .arg("--strict-mcp-config")
        .arg("--mcp-config")
        .arg(mcp_path.as_os_str())
        .args(["--permission-mode", &cfg.permission_mode])
        .arg("--no-session-persistence")
        .arg("--allowedTools")
        .args(&tools)
        .cwd(paths.workspace)
        .env("CLAUDE_CONFIG_DIR", config_dir)
        // The Stop hook (`--jit-hooks`) would wait up to 600 s for backlog work; a bench task's
        // backlog is empty, so that wait would only eat the session timeout. The hook command
        // also prefixes AV_HOOK_WAIT_SECS=0, in case the CLI does not pass this environment in.
        .path_prepend(spec.path_prepend.clone())
        .timeout(Duration::from_secs(spec.session_timeout_secs))
        .events_path(paths.events)
        .stderr_path(paths.stderr);
    if cfg.prices.is_none() {
        cmd = cmd.max_budget_usd(Some(cfg.session_cap_usd));
    }
    bench_env(cmd, cfg, AGENT)
}

/// `grok -p <prompt> --output-format json --always-approve --max-turns N --cwd <dir>`.
/// No `--max-budget-usd` and no `--confirm`: vendors without a budget flag are capped by turns.
fn grok_command(spec: &TaskSpec, cfg: &RunConfig, paths: &SessionPaths<'_>) -> AgentCommand {
    let cmd = AgentCommand::new(&cfg.grok_bin)
        .arg("-p")
        .arg(&spec.goal)
        .args(["--output-format", "json"])
        .arg("--always-approve")
        .args(["--max-turns", &spec.max_turns.to_string()])
        .arg("--cwd")
        .arg(paths.workspace.as_os_str())
        .cwd(paths.workspace)
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .path_prepend(spec.path_prepend.clone())
        .timeout(Duration::from_secs(spec.session_timeout_secs))
        .events_path(paths.events)
        .stderr_path(paths.stderr);
    bench_env(cmd, cfg, "grok")
}

fn json_u64(v: &Value) -> u64 {
    v.as_u64()
        .or_else(|| {
            v.as_f64()
                .filter(|n| n.is_finite() && *n >= 0.0)
                .map(|n| n as u64)
        })
        .unwrap_or(0)
}

/// Grok `--output-format json` is one JSON object (`usage.input_tokens`,
/// `usage.cache_read_input_tokens`, `usage.output_tokens`, `total_cost_usd`, `num_turns`).
/// `usage.reasoning_tokens` is ignored: [`Usage`] has no field for it.
pub fn parse_grok(text: &str) -> ParsedSession {
    let mut s = ParsedSession::default();
    let Ok(v) = serde_json::from_str::<Value>(text.trim()) else {
        s.bad_lines = 1;
        return s;
    };
    if !v.is_object() {
        s.bad_lines = 1;
        return s;
    }
    let u = &v["usage"];
    s.result = Some(ResultEvent {
        cost_usd: v["total_cost_usd"].as_f64().unwrap_or(0.0),
        num_turns: v["num_turns"].as_u64().map(|n| n as u32),
        is_error: v["is_error"].as_bool().unwrap_or(false),
        usage: Usage {
            input: json_u64(&u["input_tokens"]),
            cache_creation: 0,
            cache_read: json_u64(&u["cache_read_input_tokens"]),
            output: json_u64(&u["output_tokens"]),
        },
        duration_ms: v["duration_ms"].as_u64(),
    });
    s
}

/// Claude's stream-json, or Grok's one JSON object, as a [`ParsedSession`].
pub fn parse_session(vendor: Vendor, text: &str) -> ParsedSession {
    match vendor {
        Vendor::Claude => crate::events::parse_stream(text),
        Vendor::Grok => parse_grok(text),
    }
}

/// Where one record stands in one pack.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Standing {
    /// `handle`, `summary` or `full` when the pack shows the record.
    pub tier: Option<String>,
    /// Its 1-based place in the pack's list of omitted items, when it is only listed there.
    pub omitted_at: Option<usize>,
    /// Its score, when the pack shows it.
    pub score: Option<f64>,
    /// Records the pack shows, at any tier.
    pub records_shown: usize,
    /// Items the pack left out, listed or not.
    pub omitted_total: usize,
    pub tokens: u64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub degraded: Vec<String>,
}

impl Standing {
    /// `full`, `summary`, `handle`, `omitted #k` or `absent`.
    pub fn place(&self) -> String {
        match (&self.tier, self.omitted_at) {
            (Some(t), _) => t.clone(),
            (None, Some(k)) => format!("omitted #{k}"),
            (None, None) => "absent".into(),
        }
    }

    /// Whether the pack shows the record (a handle is its clipped summary).
    pub fn shown(&self) -> bool {
        self.tier.is_some()
    }
}

/// What the pre-read hook answers for one file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreRead {
    pub path: String,
    /// Records in the answer.
    pub records: usize,
    /// Whether the lesson of the task is one of them.
    pub lesson: bool,
}

/// What a relay task's av session is handed about its own lesson (`first-packs`): no agent runs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FirstPack {
    pub task: String,
    /// `rec:<8 hex>` of the task's own lesson (the note of its `relay_pair`).
    pub lesson: String,
    /// Records seeded on the earlier task: lessons and noise.
    pub seeded: usize,
    /// The pack the task start answered with, before the records were embedded. This is the
    /// `pack.md` a `run` keeps; no agent reads it.
    pub at_start: Standing,
    /// The pack the session-start hook prints, once the repo's outbox has drained.
    pub at_hook: Standing,
    /// The pre-read hook's answer for each of the task's `relevant_files`, in that order, under
    /// the hook pack's session (so a record the pack showed is not repeated).
    pub pre_read: Vec<PreRead>,
    /// How many of the task's `relevant_files` the hook pack names: records must not be bought
    /// with the code the agent needs.
    #[serde(default)]
    pub files_named: usize,
    #[serde(default)]
    pub files: usize,
}

impl FirstPack {
    /// Whether either hook gives the agent the lesson's whole summary: the pack at the
    /// `summary` or `full` tier (a handle is cut to 70 characters), or a pre-read answer.
    pub fn in_full(&self) -> bool {
        matches!(self.at_hook.tier.as_deref(), Some("summary" | "full"))
            || self.pre_read.iter().any(|p| p.lesson)
    }
}

/// Where `lesson` stands in `pack`, read from the pack's receipt, which is kept as `keep`.
fn standing(daemon: &Daemon, pack: &Value, lesson: &str, keep: &Path) -> Result<Standing> {
    let md = pack["markdown"].as_str().unwrap_or_default();
    let listed: Vec<&str> = pack["omitted"]
        .as_array()
        .map(|a| a.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();
    let omitted_total = md
        .lines()
        .find_map(|l| l.strip_prefix("omitted ")?.split(' ').next()?.parse().ok())
        .unwrap_or(listed.len());
    let mut s = Standing {
        omitted_at: listed
            .iter()
            .position(|l| l.contains(lesson))
            .map(|i| i + 1),
        omitted_total,
        tokens: pack["tokens"].as_u64().unwrap_or(0),
        degraded: pack["degraded"]
            .as_array()
            .map(|a| {
                a.iter()
                    .filter_map(Value::as_str)
                    .map(str::to_owned)
                    .collect()
            })
            .unwrap_or_default(),
        ..Standing::default()
    };
    let Some(receipt) = pack["receipt"].as_str() else {
        return Err("the pack has no receipt, so its items cannot be read".into());
    };
    let receipt = daemon.get(&format!("/v1/receipts/{receipt}"))?;
    write_json(keep, &receipt)?;
    for item in receipt["items"].as_array().into_iter().flatten() {
        let id = item["id"].as_str().unwrap_or_default();
        if !id.starts_with("rec:") {
            continue;
        }
        s.records_shown += 1;
        if id == lesson {
            s.tier = Some(match &item["tier"] {
                Value::String(t) => t.to_lowercase(),
                other => other.to_string(),
            });
            s.score = item["score"].as_f64();
        }
    }
    Ok(s)
}

/// The session-start hook's query, from the same settings `run` gives the hook.
fn hook_query(cfg: &RunConfig, session: &str) -> Result<String> {
    let mut q = format!(
        "budget={}&session={session}&code={}",
        cfg.hook_budget,
        cfg.hook_code.as_deref().unwrap_or("signatures")
    );
    let mut map = true;
    for a in &cfg.hook_args {
        if a == "--no-map" {
            map = false;
        } else if let Some(n) = a.strip_prefix("--max-code=") {
            q.push_str(&format!("&max_code={n}"));
        } else {
            return Err(format!("first-packs does not know the hook argument {a}"));
        }
    }
    q.push_str(&format!("&map={map}"));
    Ok(q)
}

/// Sets a relay task up as `run` does for the `av` arm, then asks the daemon for what the hooks
/// would print instead of starting an agent. Writes `pack.md` (at task start), `hook-pack.md`
/// and `first-pack.json` under the run directory. Costs nothing.
pub fn first_pack(spec: &TaskSpec, run_id: &str, cfg: &RunConfig) -> Result<FirstPack> {
    let pair = spec
        .relay_pair
        .as_deref()
        .ok_or_else(|| format!("task {} has no relay_pair", spec.name))?;
    let dir = first_pack_dir(cfg, run_id, &spec.name);
    if dir.exists() {
        std::fs::remove_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    }
    std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let url = cfg.av_url.as_deref().ok_or("first-packs needs --av-url")?;
    let daemon = Daemon::new(url, cfg.av_token.as_deref());
    daemon.preflight()?;
    let t = start_av_task(spec, Arm::Av, 1, run_id, cfg, &daemon, &dir)?;
    let id = t
        .seeded
        .iter()
        .find(|(p, _)| p == pair)
        .map(|(_, id)| id.replace('-', ""))
        .filter(|id| id.len() >= 8)
        .ok_or_else(|| format!("task {}: no record was seeded for {pair}", spec.name))?;
    let lesson = format!("rec:{}", &id[..8]);
    let at_start = standing(&daemon, &t.start_pack, &lesson, &dir.join("receipt.json"))?;
    wait_for_index(&daemon, &t.repo, cfg)?;

    let session = uuid::Uuid::new_v4().to_string();
    let pack = daemon.get(&format!(
        "/v1/tasks/{}/context?{}",
        t.id,
        hook_query(cfg, &session)?
    ))?;
    std::fs::write(
        dir.join("hook-pack.md"),
        pack["markdown"].as_str().unwrap_or_default(),
    )
    .map_err(|e| e.to_string())?;
    let at_hook = standing(&daemon, &pack, &lesson, &dir.join("hook-receipt.json"))?;
    let hook_md = pack["markdown"].as_str().unwrap_or_default();
    let files_named = spec
        .relevant_files
        .iter()
        .filter(|f| hook_md.contains(f.as_str()))
        .count();
    let mut pre_read = vec![];
    for path in &spec.relevant_files {
        let r = daemon.get(&format!(
            "/v1/tasks/{}/records/at?path={path}&session={session}",
            t.id
        ))?;
        let ids: Vec<&str> = r["records"]
            .as_array()
            .map(|a| a.iter().filter_map(Value::as_str).collect())
            .unwrap_or_default();
        pre_read.push(PreRead {
            path: path.clone(),
            records: ids.len(),
            lesson: ids.contains(&lesson.as_str()),
        });
    }
    let out = FirstPack {
        task: spec.name.clone(),
        lesson,
        seeded: t.seeded.len(),
        at_start,
        at_hook,
        pre_read,
        files_named,
        files: spec.relevant_files.len(),
    };
    write_json(
        &dir.join("first-pack.json"),
        &serde_json::to_value(&out).expect("json"),
    )?;
    if !cfg.keep_work {
        remove_work(&t.workspace, Some(&t), Some(&daemon));
    }
    Ok(out)
}

/// Where `first_pack` writes for one task.
pub fn first_pack_dir(cfg: &RunConfig, run_id: &str, task: &str) -> PathBuf {
    cfg.work_root
        .join("runs")
        .join(run_id)
        .join(format!("{task}-first-pack"))
}

/// `first-packs.md`: one line per task, and how many tasks each channel gave their lesson.
pub fn first_packs_report(rows: &[FirstPack], noise: usize) -> String {
    let mut out = format!(
        "# First packs: is the task's own lesson in them?\n\nNoise notes: {noise}. Tasks: {}. \
         No agent ran.\n\n| task | records seeded | pack at task start | hook pack | records in \
         hook pack | omitted | relevant files named | pre-read (file: lesson, records) |\n\
         |---|---|---|---|---|---|---|---|\n",
        rows.len()
    );
    for r in rows {
        let pre: Vec<String> = r
            .pre_read
            .iter()
            .map(|p| {
                format!(
                    "{}: {}, {}",
                    p.path,
                    if p.lesson { "yes" } else { "no" },
                    p.records
                )
            })
            .collect();
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {}/{} | {} |\n",
            r.task,
            r.seeded,
            r.at_start.place(),
            r.at_hook.place(),
            r.at_hook.records_shown,
            r.at_hook.omitted_total,
            r.files_named,
            r.files,
            pre.join("; ")
        ));
    }
    let n = |f: &dyn Fn(&FirstPack) -> bool| rows.iter().filter(|r| f(r)).count();
    let all = rows.len();
    out.push_str(&format!(
        "\nLesson shown in the pack at task start: {} of {all}. In the hook pack: {} of {all}. \
         In the hook pack or a pre-read answer: {} of {all}.\n\nLesson shown in full (the hook \
         pack at the summary or full tier, or a pre-read answer): {} of {all}. Relevant files \
         named in the hook pack: {} of {}. Packs marked degraded: {}.\n",
        n(&|r| r.at_start.shown()),
        n(&|r| r.at_hook.shown()),
        n(&|r| r.at_hook.shown() || r.pre_read.iter().any(|p| p.lesson)),
        n(&|r| r.in_full()),
        rows.iter().map(|r| r.files_named).sum::<usize>(),
        rows.iter().map(|r| r.files).sum::<usize>(),
        n(&|r| !r.at_hook.degraded.is_empty()),
    ));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grok_json_maps_usage_and_ignores_reasoning_tokens() {
        let text = r#"{
            "sessionId": "01a1926b",
            "num_turns": 3,
            "usage": {
                "input_tokens": 12855,
                "cache_read_input_tokens": 8832,
                "output_tokens": 1975,
                "reasoning_tokens": 1115,
                "total_tokens": 24777
            },
            "total_cost_usd": 0.031
        }"#;
        let s = parse_grok(text);
        assert_eq!(
            s.total_usage(),
            Usage {
                input: 12855,
                cache_creation: 0,
                cache_read: 8832,
                output: 1975,
            }
        );
        let r = s.result.as_ref().unwrap();
        assert_eq!(r.num_turns, Some(3));
        assert!((r.cost_usd - 0.031).abs() < 1e-12);
        // 1115 is not added into any Usage field.
        let u = s.total_usage();
        assert_eq!(
            u.input + u.cache_creation + u.cache_read + u.output,
            12855 + 8832 + 1975
        );
    }

    #[test]
    fn grok_output_that_is_not_json_is_truncated() {
        let s = parse_grok("not json");
        assert!(s.truncated());
        assert_eq!(s.bad_lines, 1);
        assert_eq!(parse_session(Vendor::Grok, "[]").bad_lines, 1);
    }
}
