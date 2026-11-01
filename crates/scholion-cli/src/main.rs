mod agents;
mod client;
mod hook;
mod local;
mod runner;
mod taskfile;
mod watch;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use client::{CliError, Client, Result};
use scholion_core::{MAX_SUMMARY_CHARS, RecordKind};
use serde_json::{Value, json};
use taskfile::TaskFile;

#[derive(Parser)]
#[command(
    name = "scholion",
    version,
    about = "Scholion: context-first version control for coding agents"
)]
struct Cli {
    /// Daemon address.
    #[arg(
        long,
        env = "AV_URL",
        default_value = "http://localhost:7420",
        global = true
    )]
    url: String,
    /// Task id (default: nearest .av-task file).
    #[arg(long, env = "AV_TASK", global = true)]
    task: Option<String>,
    /// Session id used to avoid re-sending context (default: from .av-task).
    #[arg(long, env = "AV_SESSION", global = true)]
    session: Option<String>,
    /// Agent name recorded on tasks and records.
    #[arg(long, env = "AV_AGENT", default_value = "agent", global = true)]
    agent: String,
    /// Bearer token for a daemon started with AV_TOKEN.
    #[arg(long, env = "AV_TOKEN", global = true, hide_env_values = true)]
    token: Option<String>,
    /// Claim generation writes are made under (default: from .av-task when it names this agent).
    #[arg(long, env = "AV_GENERATION", global = true)]
    generation: Option<i64>,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Start a daemon on this machine in the background (no Docker: it runs its own Postgres
    /// unless AV_DATABASE_URL is set). Data in $AV_HOME (default ~/.scholion), workspaces in ~/ws.
    Up,
    /// Stop the daemon `av up` started.
    Down,
    /// Set up the git repository of this directory: start the daemon if needed, add the repo,
    /// and install the Claude Code hooks and MCP server.
    Init {
        /// Repo name (default: the repository's directory name).
        #[arg(long)]
        name: Option<String>,
        /// Agent CLI to wire to the daemon (repeatable): claude (hooks and MCP), codex, grok,
        /// gemini (MCP). Default: claude.
        #[arg(long = "for", value_enum)]
        agents: Vec<agents::Agent>,
        /// Leave Claude Code's settings alone (same as naming only other agents).
        #[arg(long)]
        no_claude: bool,
    },
    /// Register a git repository (path as seen by the daemon).
    Repo {
        #[command(subcommand)]
        cmd: RepoCmd,
    },
    /// Task lifecycle.
    Task {
        #[command(subcommand)]
        cmd: TaskCmd,
    },
    /// Snapshot the workspace and refresh the index.
    Checkpoint,
    /// Record an intent, decision, attempt (dead end) or note.
    Record {
        #[arg(long, required_unless_present = "json", requires = "summary")]
        kind: Option<String>,
        #[arg(long, required_unless_present = "json", requires = "kind")]
        summary: Option<String>,
        /// Record many at once: a JSON list of {kind, summary, body?, anchors?: ["path[:Symbol]"]}
        /// from FILE, or from stdin with `-`. Replaces --kind and --summary.
        #[arg(long, value_name = "FILE|-", conflicts_with_all = ["kind", "summary", "body", "anchors"])]
        json: Option<String>,
        #[arg(long)]
        body: Option<String>,
        /// path or path:symbol (repeatable)
        #[arg(long = "anchor")]
        anchors: Vec<String>,
        /// The `rcp:…` id of the pack you worked from (printed by `av context`).
        #[arg(long)]
        receipt: Option<String>,
    },
    /// Print a token-budgeted context pack for the task.
    Context {
        #[arg(long, default_value_t = 800)]
        budget: u32,
        /// `signatures`: path, lines and signature of each code item; `full`: with its source.
        #[arg(long, default_value = "signatures", value_parser = ["full", "signatures"])]
        code: String,
        #[arg(long)]
        focus: Option<String>,
        /// Forget what this session was already sent (e.g. after the agent's context was compacted)
        /// and start a new session; everything is sent in full again.
        #[arg(long)]
        fresh: bool,
    },
    /// Show task status.
    Status,
    /// Print the full text behind an id shown in a pack (rec:… or sym:…).
    Expand { id: String },
    /// Conventions: rules served in packs when their path glob matches the task's files.
    Convention {
        #[command(subcommand)]
        cmd: ConventionCmd,
    },
    /// Print the manifest of a context pack: what was sent, at which tier, and why.
    Receipt { id: String },
    /// The task lifecycle event feed (started, submit, approve, merged, conflicted, ...).
    Events {
        #[arg(long)]
        repo: Option<String>,
        /// Only events after this id (default: from now on).
        #[arg(long)]
        since: Option<i64>,
        /// Keep waiting for new events until interrupted.
        #[arg(long)]
        follow: bool,
    },
    /// Rebuild the semantic index from Postgres (all repos, or one with --repo).
    Reindex {
        #[arg(long)]
        repo: Option<String>,
    },
    /// Hand the task to review (a conflicted task is rebased onto main first).
    Submit {
        /// Record and submit in one call: "KIND: SUMMARY @ path[:Symbol]" (repeatable).
        #[arg(long = "record", value_name = "KIND: SUMMARY @ path[:Symbol]")]
        records: Vec<String>,
    },
    /// Run a command in the workspace and record its result at the current head (needed to approve).
    Validate {
        /// The command and its arguments, after `--`.
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, required = true)]
        command: Vec<String>,
    },
    /// Approve or request changes on a task in review (as --agent, who must not own it).
    Review {
        /// Task id to review (default: the current task).
        task_id: Option<String>,
        #[arg(long, value_parser = ["approve", "request-changes"])]
        verdict: String,
        /// `summary|path:symbol|body` (anchor and body optional; repeatable).
        #[arg(long = "finding")]
        findings: Vec<String>,
    },
    /// Merge an approved task into main.
    Merge {
        /// Task id to merge (default: the current task).
        task_id: Option<String>,
    },
    /// Rebase the task onto the current main; conflicts are left as markers in the workspace.
    Sync,
    /// Undo this task's merge (only while it is still the newest change on main).
    Undo {
        /// Task id (default: the current task).
        task_id: Option<String>,
    },
    /// Revert a merged task on main with a new commit that inverts its merge, even after later
    /// merges; the task goes back to open for its owner.
    Revert {
        /// Task id (default: the current task).
        task_id: Option<String>,
        #[arg(long, default_value = "")]
        reason: String,
    },
    /// Find the merge that broke main: a binary search over main's history with the repo's
    /// check (recorded results first, the runner for the rest). Needs --trust, like `av runner`.
    Bisect {
        #[arg(long)]
        repo: Option<String>,
        #[arg(long)]
        trust: bool,
        /// How many commits of main to search.
        #[arg(long, default_value_t = 100)]
        limit: usize,
        #[arg(long)]
        work_dir: Option<PathBuf>,
    },
    /// Hand the task to another agent: records the state and prints the handoff bundle.
    Handoff {
        #[arg(long, default_value = "")]
        done: String,
        #[arg(long, default_value = "")]
        remaining: String,
        /// What is currently broken (failing tests, half-done edits).
        #[arg(long, default_value = "")]
        broken: String,
        #[arg(long, default_value = "")]
        questions: String,
        #[arg(long, default_value_t = 4000)]
        budget: u32,
        /// Tags the next agent must have, comma separated (e.g. image); `av next` offers the task
        /// only to agents that have them.
        #[arg(long, value_delimiter = ',')]
        needs: Vec<String>,
        /// Pin the task to this agent for its reserve window.
        #[arg(long)]
        prefer: Option<String>,
    },
    /// Commands for agent hooks; they never fail (exit 0) and are silent outside a task.
    Hook {
        #[command(subcommand)]
        cmd: HookCmd,
    },
    /// Full-text search over the repo's records (default repo: the current task's).
    Search {
        query: String,
        #[arg(long)]
        repo: Option<String>,
        #[arg(long, default_value_t = 10)]
        limit: usize,
    },
    /// Main's history, newest first (default repo: the current task's); --path: only the
    /// commits that changed that path.
    Log {
        #[arg(long)]
        repo: Option<String>,
        #[arg(long, default_value_t = 20)]
        limit: usize,
        #[arg(long)]
        path: Option<String>,
    },
    /// Run the repo's post-merge check on main, then on every merge as it lands (follows the
    /// event feed). The command comes from the daemon (`av repo settings --check`), so it only
    /// runs with --trust.
    Runner {
        #[arg(long)]
        repo: Option<String>,
        /// Run the command from the daemon's settings on this machine.
        #[arg(long)]
        trust: bool,
        /// Check main if it has no result yet, then exit.
        #[arg(long)]
        once: bool,
        /// Where trees are materialized (default: a temp directory).
        #[arg(long)]
        work_dir: Option<PathBuf>,
    },
    /// Main's health from the post-merge checks: green, red since which merge (and its task), or
    /// unknown.
    Health {
        #[arg(long)]
        repo: Option<String>,
    },
    /// Why a line of main is the way it is: the merge that last changed it, its task and goal,
    /// and the task's decisions, dead ends and notes. `path:line`.
    Why {
        target: String,
        #[arg(long)]
        repo: Option<String>,
    },
    /// Print a file at a commit, read from the daemon (no workspace needed).
    Show {
        path: String,
        /// main (default), task:<slug> (its last checkpoint) or a commit id.
        #[arg(long, default_value = "main")]
        at: String,
        #[arg(long)]
        repo: Option<String>,
    },
    /// Unified diff between two commits (each main, task:<slug> or a commit id).
    Diff {
        #[arg(long)]
        from: String,
        #[arg(long, default_value = "main")]
        to: String,
        #[arg(long)]
        repo: Option<String>,
    },
    /// End the current task for good (only its owner can); its records stay searchable.
    Abandon,
    /// Print the task's handoff bundle, or with --pr the review bundle (tests, risks, checklist).
    Bundle {
        #[arg(long)]
        pr: bool,
        #[arg(long, default_value_t = 4000)]
        budget: u32,
    },
    /// Take over a handed-off task (or an open one whose lease expired) as --agent.
    Claim {
        task_id: String,
        #[arg(long, default_value_t = 4000)]
        budget: u32,
    },
    /// Agent profiles: what each agent is good at and what it costs (used by `next`).
    Agent {
        #[command(subcommand)]
        cmd: AgentCmd,
    },
    /// Post a task to the backlog. An agent that fits it takes it with `av next`.
    Post {
        #[arg(long)]
        repo: String,
        #[arg(long)]
        title: String,
        #[arg(long)]
        goal: String,
        /// Tags the agent must have, comma separated (e.g. image,ui).
        #[arg(long, value_delimiter = ',')]
        needs: Vec<String>,
        /// Pin the item to this agent for its reserve window.
        #[arg(long)]
        prefer: Option<String>,
        /// Higher is taken first.
        #[arg(long, default_value_t = 0)]
        priority: i32,
        /// Seconds only the best-fitting agent may take it (default 600); then any agent that can.
        #[arg(long)]
        reserve: Option<i64>,
    },
    /// List the backlog, or drop an item.
    #[command(args_conflicts_with_subcommands = true)]
    Backlog {
        #[command(subcommand)]
        cmd: Option<BacklogCmd>,
        #[arg(long)]
        repo: Option<String>,
        /// queued, taken or dropped.
        #[arg(long)]
        state: Option<String>,
    },
    /// Take the next task that fits --agent: a backlog item (a new workspace and a pack) or a
    /// handed-off task that names tags the agent has (the handoff bundle).
    Next {
        #[arg(long)]
        repo: Option<String>,
        #[arg(long, default_value_t = 800)]
        budget: u32,
        /// With nothing to take, wait up to this many seconds for something (the daemon caps it).
        #[arg(long)]
        wait: Option<u64>,
    },
    /// Run an agent command whenever work fits --agent: looks every --every seconds without taking
    /// anything, then runs the command (with AV_AGENT and AV_URL set) and waits for it. The command
    /// takes the work itself, e.g. `av watch --agent grok -- grok -p "take your next av task"`.
    Watch {
        #[arg(long)]
        repo: Option<String>,
        /// Seconds between looks (and after a run).
        #[arg(long, default_value_t = 15)]
        every: u64,
        /// Stop after this many runs.
        #[arg(long)]
        max_runs: Option<u32>,
        /// Look once, run at most once, then exit.
        #[arg(long)]
        once: bool,
        /// The agent command, after `--`.
        #[arg(last = true, required = true)]
        cmd: Vec<String>,
    },
}

#[derive(Subcommand)]
enum AgentCmd {
    /// Set (replace) the profile of an agent.
    Set {
        name: String,
        /// Tags it is best at, comma separated.
        #[arg(long, value_delimiter = ',')]
        best: Vec<String>,
        /// Tags it can do, comma separated.
        #[arg(long, value_delimiter = ',')]
        can: Vec<String>,
        /// 1 is the cheapest.
        #[arg(long, default_value_t = 5)]
        cost: i32,
        /// Another name the agent introduces itself with (repeatable).
        #[arg(long = "alias")]
        aliases: Vec<String>,
        /// The most open tasks it may hold at once (an agent CLI with subagents works in
        /// parallel). Default: no limit.
        #[arg(long)]
        max_open: Option<i32>,
    },
    /// List the profiles, cheapest first.
    List,
}

#[derive(Subcommand)]
enum BacklogCmd {
    /// Withdraw a queued item.
    Drop { id: String },
}

#[derive(Subcommand)]
enum HookCmd {
    /// Print a small context pack for the task of this directory (Claude Code `SessionStart`).
    SessionStart {
        #[arg(long, default_value_t = 800)]
        budget: u32,
        /// `signatures`: path, lines and signature of each code item; `full`: with its source.
        /// Claude Code must read a file before editing it, so a body in the pack is sent twice.
        #[arg(long, default_value = "signatures", value_parser = ["full", "signatures"])]
        code: String,
        /// Leave out the Map section (repo layout, file skeletons).
        #[arg(long)]
        no_map: bool,
        /// At most this many code items, the best scored: each one named is a file the agent
        /// may open.
        #[arg(long)]
        max_code: Option<usize>,
    },
    /// Checkpoint the task, at most every 2 s (Claude Code `PostToolUse` on edits).
    PostEdit,
    /// Print what other tasks recorded about the file being opened (Claude Code `PreToolUse` on
    /// reads and edits), each record once per session.
    PreRead,
    /// Ask once per agent session to record the changes no record explains (Claude Code `Stop`).
    Stop,
}

#[derive(Subcommand)]
enum ConventionCmd {
    Add {
        #[arg(long)]
        repo: String,
        /// Path glob the rule applies to, e.g. `src/**/*.rs`.
        #[arg(long)]
        scope: String,
        /// Never dropped from a pack by ranking (hard constraint).
        #[arg(long)]
        required: bool,
        /// With --required: a command `av validate` runs in the workspace; approval needs it to
        /// exit 0 at the head.
        #[arg(long, requires = "required")]
        check: Option<String>,
        text: String,
    },
    /// Remove a convention (it stays in history, hidden from packs and checks).
    Rm {
        id: i64,
        #[arg(long)]
        repo: Option<String>,
    },
    /// Every convention of the repo (default repo: the current task's).
    List {
        #[arg(long)]
        repo: Option<String>,
    },
}

#[derive(Subcommand)]
enum RepoCmd {
    /// List the imported repos and the commit each one's main points at.
    List,
    Add {
        name: String,
        git_source: String,
    },
    /// Show the git backup of main; --set a remote (URL or path), --clear it, or --push now.
    Backup {
        name: String,
        #[arg(long, conflicts_with = "clear")]
        set: Option<String>,
        #[arg(long)]
        clear: bool,
        #[arg(long)]
        push: bool,
    },
    /// Show the repo's policy, or change it: which files may differ after a conflict
    /// resolution without a new review (--carry-glob, repeatable), and the post-merge check.
    Settings {
        name: String,
        #[arg(long = "carry-glob")]
        carry_globs: Vec<String>,
        /// Command `av runner` runs on every new main ("" clears it).
        #[arg(long)]
        check: Option<String>,
    },
}

/// `av bisect`: the first commit of main (oldest first) whose check fails, with its task.
fn bisect(
    client: &Client,
    cli: &Cli,
    repo: &str,
    limit: usize,
    work: Option<PathBuf>,
) -> Result<()> {
    let settings = client.get(&format!("/v1/repos/{repo}/settings"), &[])?;
    let command = settings["check_command"]
        .as_str()
        .filter(|c| !c.trim().is_empty())
        .ok_or_else(|| CliError::Local(format!("repo {repo} has no check command")))?
        .to_owned();
    let limit_s = limit.to_string();
    let log = client.get(
        &format!("/v1/repos/{repo}/log"),
        &[("limit", limit_s.as_str())],
    )?;
    let mut commits: Vec<Value> = log["commits"].as_array().cloned().unwrap_or_default();
    commits.reverse(); // oldest first
    if commits.is_empty() {
        return Err(CliError::Local("main has no history".into()));
    }
    let runner = runner::Runner {
        client,
        work_dir: work.unwrap_or_else(|| runner::default_work_dir(repo)),
        repo: repo.to_owned(),
        name: cli.agent.clone(),
    };
    let change = |i: usize| s(&commits[i]["change"]).to_owned();
    let last = commits.len() - 1;
    let main_bad = match commits[last]["check"]["exit_code"].as_i64() {
        Some(code) => code != 0,
        None => runner.check(&change(last), &command)? != 0,
    };
    if !main_bad {
        println!("main passes `{command}`: nothing to bisect");
        return Ok(());
    }
    let known = |i: usize| commits[i]["check"]["exit_code"].as_i64().map(|c| c != 0);
    let first = runner::first_bad(commits.len(), known, |i| {
        Ok(runner.check(&change(i), &command)? != 0)
    })?;
    let culprit = &commits[first];
    let description = s(&culprit["description"]);
    println!(
        "first bad: {} {}",
        change(first).chars().take(12).collect::<String>(),
        description.lines().next().unwrap_or("")
    );
    let task_id = description
        .lines()
        .rev()
        .find_map(|l| l.trim().strip_prefix("av task "))
        .and_then(|rest| rest.split_whitespace().next());
    match task_id {
        Some(id) => {
            let t = client.get(&format!("/v1/tasks/{id}"), &[])?;
            println!(
                "task {} ({}), merged by {}: {}",
                s(&t["task"]["slug"]),
                id,
                s(&t["merge"]["merged_by"]),
                s(&t["task"]["goal"])
            );
            println!("to take it out of main: av revert {id} --reason \"breaks `{command}`\"");
        }
        None if first == 0 => println!("the oldest commit searched already fails; widen --limit"),
        None => println!("that commit is not a task merge"),
    }
    Ok(())
}

/// Runs `cmd` in `dir`, prints its output, and returns its exit code and last 20 lines.
fn run_capturing(
    cmd: &mut std::process::Command,
    dir: &std::path::Path,
    name: &str,
) -> Result<(i32, String)> {
    let out = cmd
        .current_dir(dir)
        .output()
        .map_err(|e| CliError::Local(format!("cannot run {name}: {e}")))?;
    let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    print!("{text}");
    let lines: Vec<&str> = text.lines().collect();
    let tail = lines[lines.len().saturating_sub(20)..].join("\n");
    Ok((out.status.code().unwrap_or(-1), tail))
}

/// One line about a repo's backup state.
fn backup_line(b: &Value) -> String {
    let Some(remote) = b["remote"].as_str() else {
        return "backup: off".to_owned();
    };
    let mut line = format!("backup: {remote}");
    match b["last_pushed"].as_str() {
        Some(c) => line.push_str(&format!(
            " · pushed {}",
            c.chars().take(12).collect::<String>()
        )),
        None => line.push_str(" · never pushed"),
    }
    let failures = b["failures"].as_u64().unwrap_or(0);
    if failures > 0 {
        line.push_str(&format!(
            " · {failures} failed ({}) · next retry {}",
            s(&b["last_error"]),
            s(&b["next_retry"])
        ));
    }
    line
}

#[derive(Subcommand)]
enum TaskCmd {
    Start {
        #[arg(long)]
        repo: String,
        #[arg(long)]
        title: String,
        #[arg(long)]
        goal: String,
        #[arg(long, default_value_t = 800)]
        budget: u32,
    },
}

/// `path` or `path:symbol` -> anchor JSON.
fn parse_anchor(s: &str) -> Value {
    match s.split_once(':') {
        Some((path, symbol)) if !symbol.is_empty() => json!({"path": path, "symbol": symbol}),
        Some((path, _)) => json!({"path": path, "symbol": null}),
        None => json!({"path": s, "symbol": null}),
    }
}

/// `KIND: SUMMARY @ path[:Symbol]` -> record JSON (the anchor is optional).
fn parse_record_spec(s: &str) -> Result<Value> {
    let bad = || {
        CliError::Local(format!(
            "expected KIND: SUMMARY @ path[:Symbol], got {s:?} (the anchor after @ is optional)"
        ))
    };
    let (kind, rest) = s.split_once(':').ok_or_else(bad)?;
    let (summary, anchor) = match rest.rsplit_once(" @ ") {
        Some((summary, anchor)) => (summary, Some(anchor.trim())),
        None => (rest, None),
    };
    let (kind, summary) = (kind.trim(), summary.trim());
    if kind.is_empty() || summary.is_empty() {
        return Err(bad());
    }
    let anchors: Vec<Value> = anchor
        .filter(|a| !a.is_empty())
        .map(parse_anchor)
        .into_iter()
        .collect();
    Ok(json!({"kind": kind, "summary": summary, "anchors": anchors}))
}

/// The items of `av record --json`: a JSON list read from FILE or stdin (`-`); anchors are
/// `path[:Symbol]` strings.
fn read_record_items(src: &str) -> Result<Vec<Value>> {
    let text = if src == "-" {
        let mut t = String::new();
        std::io::Read::read_to_string(&mut std::io::stdin(), &mut t)
            .map_err(|e| CliError::Local(format!("cannot read stdin: {e}")))?;
        t
    } else {
        std::fs::read_to_string(src)
            .map_err(|e| CliError::Local(format!("cannot read {src}: {e}")))?
    };
    let items: Vec<Value> = serde_json::from_str(&text).map_err(|e| {
        CliError::Local(format!(
            "--json expects a list of {{kind, summary, body?, anchors?}}: {e}"
        ))
    })?;
    items
        .into_iter()
        .enumerate()
        .map(|(i, mut item)| {
            // everything the daemon would refuse on its own, checked before the first POST so a
            // bad item sends nothing
            for key in ["kind", "summary"] {
                if item[key].as_str().is_none_or(str::is_empty) {
                    return Err(CliError::Local(format!(
                        "records[{i}]: every record needs a {key}: {item}"
                    )));
                }
            }
            let kind = s(&item["kind"]);
            if kind.parse::<RecordKind>().is_err() {
                return Err(CliError::Local(format!(
                    "records[{i}]: unknown kind {kind:?} (use decision, attempt, note, handoff, \
                     intent, review or validation)"
                )));
            }
            let n = s(&item["summary"]).chars().count();
            if n > MAX_SUMMARY_CHARS {
                return Err(CliError::Local(format!(
                    "records[{i}]: summary is {n} chars, max {MAX_SUMMARY_CHARS}"
                )));
            }
            let anchors: Vec<Value> = item["anchors"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|a| match a.as_str() {
                    Some(text) => parse_anchor(text),
                    None => a.clone(),
                })
                .collect();
            item["anchors"] = Value::Array(anchors);
            Ok(item)
        })
        .collect()
}

/// `summary|path:symbol|body` -> finding JSON (anchor and body optional).
fn parse_finding(s: &str) -> Value {
    let mut parts = s.splitn(3, '|');
    let summary = parts.next().unwrap_or_default();
    let anchors: Vec<Value> = parts
        .next()
        .filter(|a| !a.is_empty())
        .map(parse_anchor)
        .into_iter()
        .collect();
    json!({"summary": summary, "anchors": anchors, "body": parts.next().unwrap_or_default()})
}

fn strings(v: &Value) -> Vec<&str> {
    v.as_array()
        .map(|a| a.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default()
}

fn short(id: &str) -> String {
    id.replace('-', "").chars().take(8).collect()
}

fn resolve_task(cli: &Cli) -> Result<(String, String)> {
    let cwd = std::env::current_dir().map_err(|e| CliError::Local(e.to_string()))?;
    let file = taskfile::find(&cwd).map(|(_, f)| f);
    match (&cli.task, file) {
        (Some(task), file) => {
            let session = cli
                .session
                .clone()
                .or(file.map(|f| f.session))
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
            Ok((task.clone(), session))
        }
        (None, Some(f)) => Ok((f.task, cli.session.clone().unwrap_or(f.session))),
        (None, None) => Err(CliError::NoTask),
    }
}

/// The generation a write to `task` is made under: `--generation`, else the `.av-task` of this
/// workspace when it is about `task`. When that file names another agent, this agent's claim was
/// taken over, so the write goes out under generation 0 and the daemon refuses it.
fn generation_for(cli: &Cli, task: &str) -> Option<i64> {
    if cli.generation.is_some() {
        return cli.generation;
    }
    let cwd = std::env::current_dir().ok()?;
    let (_, tf) = taskfile::find(&cwd)?;
    if tf.task != task {
        return None;
    }
    match &tf.agent {
        Some(owner) if *owner != cli.agent => Some(0),
        _ => tf.generation,
    }
}

fn write_taskfile(workspace: &std::path::Path, tf: &TaskFile) {
    if let Err(e) = taskfile::write(workspace, tf) {
        eprintln!(
            "av: warning: could not write {} in {}: {e}",
            taskfile::NAME,
            workspace.display()
        );
    }
}

/// `--repo`, else the repo of the current task.
fn repo_for(cli: &Cli, client: &Client, repo: &Option<String>) -> Result<String> {
    if let Some(r) = repo {
        return Ok(r.clone());
    }
    let (task, _) = resolve_task(cli)?;
    let r = client.get(&format!("/v1/tasks/{task}"), &[])?;
    Ok(s(&r["task"]["repo"]).to_owned())
}

/// `muse  cost 1  best a,b  can c  aliases x,y` (empty parts left out).
fn agent_line(a: &Value) -> String {
    let list = |k: &str| {
        a[k].as_array()
            .map(|v| {
                v.iter()
                    .filter_map(Value::as_str)
                    .collect::<Vec<_>>()
                    .join(",")
            })
            .unwrap_or_default()
    };
    let mut line = format!("{}  cost {}", s(&a["name"]), a["cost_rank"]);
    for (label, key) in [("best", "best"), ("can", "can"), ("aliases", "aliases")] {
        let v = list(key);
        if !v.is_empty() {
            line.push_str(&format!("  {label} {v}"));
        }
    }
    if let Some(m) = a["max_open"].as_i64() {
        line.push_str(&format!("  max_open {m}"));
    }
    line
}

/// ` · needs image,ui`, or nothing for an untagged item.
fn tags_suffix(needs: &Value) -> String {
    let tags: Vec<&str> = needs
        .as_array()
        .map(|v| v.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();
    if tags.is_empty() {
        String::new()
    } else {
        format!(" · needs {}", tags.join(","))
    }
}

fn s(v: &Value) -> &str {
    v.as_str().unwrap_or_default()
}

/// The pack's receipt id on stderr, so agents can cite it (`av record --receipt`).
fn print_receipt(pack: &Value) {
    if let Some(id) = pack["receipt"].as_str() {
        eprintln!("av: receipt {id}");
    }
    if let Some(n) = pack["session_expand_tokens"].as_u64().filter(|n| *n > 0) {
        eprintln!("av: expanded this session: {n} tokens");
    }
}

/// Each `degraded` entry of a response, as `av: note: …` on stderr.
fn print_notes(r: &Value) {
    for d in r["degraded"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
    {
        eprintln!("av: note: {d}");
    }
}

/// Polls a repo import until it is done; a failed one is an error.
pub(crate) fn wait_import(client: &Client, name: &str) -> Result<Value> {
    loop {
        let job = client.get(&format!("/v1/repos/{name}/import"), &[])?;
        match job["state"].as_str() {
            Some("running") => std::thread::sleep(std::time::Duration::from_secs(1)),
            Some("done") => return Ok(job),
            _ => {
                return Err(CliError::Local(format!(
                    "import of {name} failed: {}",
                    job["error"].as_str().unwrap_or("no reason given")
                )));
            }
        }
    }
}

fn run(cli: Cli) -> Result<()> {
    let client = Client::new(&cli.url, cli.token.clone());
    match &cli.cmd {
        Cmd::Up => local::up(&client, &cli.url)?,
        Cmd::Down => local::down(&client, &cli.url)?,
        Cmd::Init {
            name,
            agents,
            no_claude,
        } => {
            let mut agents = if agents.is_empty() {
                vec![agents::Agent::Claude]
            } else {
                agents.clone()
            };
            agents.dedup();
            if *no_claude {
                agents.retain(|a| *a != agents::Agent::Claude);
            }
            local::init(
                &client,
                &cli.url,
                cli.token.as_deref(),
                name.as_deref(),
                &agents,
            )?
        }
        Cmd::Repo {
            cmd: RepoCmd::Add { name, git_source },
        } => {
            // a large repo takes longer than any request timeout: start the import, then poll it
            client.post(
                "/v1/repos",
                &json!({"name": name, "git_source": git_source, "wait": false}),
            )?;
            let r = wait_import(&client, name)?;
            println!(
                "repo {} added (main {}, {} files indexed)",
                s(&r["name"]),
                s(&r["main_change"]).chars().take(12).collect::<String>(),
                r["indexed_files"]
            );
        }
        Cmd::Repo {
            cmd:
                RepoCmd::Settings {
                    name,
                    carry_globs,
                    check,
                },
        } => {
            let path = format!("/v1/repos/{name}/settings");
            let r = if carry_globs.is_empty() && check.is_none() {
                client.get(&path, &[])?
            } else {
                let mut body = json!({});
                if !carry_globs.is_empty() {
                    body["carry_globs"] = json!(carry_globs);
                }
                if let Some(c) = check {
                    body["check_command"] = json!(c);
                }
                client.put(&path, &body)?
            };
            let globs: Vec<&str> = r["carry_globs"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .collect();
            println!("carry globs: {}", globs.join(" "));
            println!("check: {}", r["check_command"].as_str().unwrap_or("(none)"));
        }
        Cmd::Repo {
            cmd:
                RepoCmd::Backup {
                    name,
                    set,
                    clear,
                    push,
                },
        } => {
            let path = format!("/v1/repos/{name}/backup");
            let mut r = if let Some(url) = set {
                client.post(&path, &json!({"remote": url}))?
            } else if *clear {
                client.post(&path, &json!({"remote": null}))?
            } else {
                client.get(&path, &[])?
            };
            if *push {
                r = client.post(&format!("{path}/push"), &json!({}))?;
            }
            println!("{}", backup_line(&r));
            if *push && r["failures"].as_u64().unwrap_or(0) > 0 {
                return Err(CliError::Local("backup push failed".into()));
            }
        }
        Cmd::Task {
            cmd:
                TaskCmd::Start {
                    repo,
                    title,
                    goal,
                    budget,
                },
        } => {
            let session = cli
                .session
                .clone()
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
            let r = client.post(
                "/v1/tasks",
                &json!({"repo": repo, "title": title, "goal": goal, "agent": cli.agent, "session": session, "budget": budget}),
            )?;
            let task = &r["task"];
            let workspace = PathBuf::from(s(&task["workspace_path"]));
            let tf = TaskFile {
                task: s(&task["id"]).to_owned(),
                session: session.clone(),
                generation: task["generation"].as_i64(),
                agent: Some(cli.agent.clone()),
                last_checkpoint_ms: None,
                nudged_session: None,
            };
            write_taskfile(&workspace, &tf);
            println!("workspace: {}", workspace.display());
            println!("task: {} ({})", s(&task["id"]), s(&task["slug"]));
            println!("session: {session}\n");
            println!("{}", s(&r["pack"]["markdown"]));
            print_receipt(&r["pack"]);
        }
        Cmd::Checkpoint => {
            let (task, _) = resolve_task(&cli)?;
            let r = client.post_as(
                &format!("/v1/tasks/{task}/checkpoint"),
                &json!({}),
                generation_for(&cli, &task),
            )?;
            let changed: Vec<&str> = r["changed"]
                .as_array()
                .map(|a| a.iter().filter_map(Value::as_str).collect())
                .unwrap_or_default();
            println!(
                "head: {}",
                s(&r["head_change"]).chars().take(12).collect::<String>()
            );
            println!("changed ({}): {}", changed.len(), changed.join(", "));
            for w in r["warnings"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
            {
                eprintln!("av: warning: {w}");
            }
        }
        Cmd::Record {
            kind,
            summary,
            json: json_src,
            body,
            anchors,
            receipt,
        } => {
            let items = match (json_src, kind, summary) {
                (Some(src), _, _) => read_record_items(src)?,
                (None, Some(kind), Some(summary)) => {
                    let anchors: Vec<Value> = anchors.iter().map(|a| parse_anchor(a)).collect();
                    vec![json!({"kind": kind, "summary": summary, "body": body,
                                "anchors": anchors})]
                }
                _ => {
                    return Err(CliError::Local(
                        "give --json FILE|- or both --kind and --summary".into(),
                    ));
                }
            };
            let (task, _) = resolve_task(&cli)?;
            for mut item in items {
                item["agent"] = json!(cli.agent);
                if item["receipt"].is_null() {
                    item["receipt"] = json!(receipt);
                }
                let r = client.post_as(
                    &format!("/v1/tasks/{task}/records"),
                    &item,
                    generation_for(&cli, &task),
                )?;
                println!("recorded rec:{} ({})", short(s(&r["id"])), s(&r["kind"]));
            }
        }
        Cmd::Context {
            budget,
            code,
            focus,
            fresh,
        } => {
            let (task, mut session) = resolve_task(&cli)?;
            if *fresh {
                session = uuid::Uuid::new_v4().to_string();
                // later calls dedup against this fresh pack, so persist the new session
                let cwd = std::env::current_dir().map_err(|e| CliError::Local(e.to_string()))?;
                if let Some((dir, mut tf)) = taskfile::find(&cwd) {
                    tf.session = session.clone();
                    if let Err(e) = taskfile::write(&dir, &tf) {
                        eprintln!("av: warning: could not update {}: {e}", taskfile::NAME);
                    }
                }
            }
            let budget = budget.to_string();
            let mut query = vec![
                ("budget", budget.as_str()),
                ("code", code.as_str()),
                ("session", session.as_str()),
            ];
            if let Some(f) = focus {
                query.push(("focus", f.as_str()));
            }
            let r = client.get(&format!("/v1/tasks/{task}/context"), &query)?;
            print!("{}", s(&r["markdown"]));
            print_notes(&r);
            print_receipt(&r);
        }
        Cmd::Convention {
            cmd:
                ConventionCmd::Add {
                    repo,
                    scope,
                    required,
                    check,
                    text,
                },
        } => {
            let r = client.post(
                &format!("/v1/repos/{repo}/conventions"),
                &json!({"scope_glob": scope, "text": text, "required": required, "check": check}),
            )?;
            println!(
                "convention conv:{} added{}",
                r["id"],
                match (*required, check.is_some()) {
                    (true, true) => " (required, check)",
                    (true, false) => " (required)",
                    _ => "",
                }
            );
        }
        Cmd::Convention {
            cmd: ConventionCmd::Rm { id, repo },
        } => {
            let repo = repo_for(&cli, &client, repo)?;
            client.delete(&format!("/v1/repos/{repo}/conventions/{id}"))?;
            println!("convention conv:{id} removed");
        }
        Cmd::Convention {
            cmd: ConventionCmd::List { repo },
        } => {
            let repo = repo_for(&cli, &client, repo)?;
            let r = client.get(&format!("/v1/repos/{repo}/conventions"), &[])?;
            for c in r["conventions"].as_array().into_iter().flatten() {
                let required = if c["required"] == true {
                    " (required)"
                } else {
                    ""
                };
                let check = c["check_command"]
                    .as_str()
                    .map(|cmd| format!(" [check: {cmd}]"))
                    .unwrap_or_default();
                println!(
                    "conv:{} ({}){required}{check} {}",
                    c["id"],
                    s(&c["scope_glob"]),
                    s(&c["text"])
                );
            }
        }
        Cmd::Events {
            repo,
            since,
            follow,
        } => {
            let mut since = *since;
            loop {
                let mut query = vec![];
                let since_s = since.map(|n| n.to_string());
                if let Some(n) = since_s.as_deref() {
                    query.push(("since", n));
                }
                if let Some(r) = repo.as_deref() {
                    query.push(("repo", r));
                }
                if *follow {
                    query.push(("wait", "25"));
                }
                let r = client.get("/v1/events", &query)?;
                for e in r["events"].as_array().into_iter().flatten() {
                    let slug = e["payload"]["slug"]
                        .as_str()
                        .map(|s| format!(" {s}"))
                        .unwrap_or_default();
                    println!(
                        "{} {} {} task {}{slug} {}",
                        e["id"],
                        s(&e["at"]).chars().take(19).collect::<String>(),
                        s(&e["kind"]),
                        s(&e["task_id"]).chars().take(8).collect::<String>(),
                        s(&e["change"]).chars().take(12).collect::<String>()
                    );
                }
                since = r["next"].as_i64();
                if !*follow {
                    break;
                }
            }
        }
        Cmd::Receipt { id } => {
            let r = client.get(&format!("/v1/receipts/{id}"), &[])?;
            println!(
                "{} · task {} · head {} · packer v{} · {}/{} tokens",
                s(&r["id"]),
                s(&r["task_id"]),
                s(&r["head_change"]).chars().take(12).collect::<String>(),
                r["packer_version"],
                r["tokens"],
                r["budget"]
            );
            println!(
                "{:<22} {:<8} {:<10} {:>6}  source",
                "id", "tier", "reason", "tokens"
            );
            for it in r["items"].as_array().into_iter().flatten() {
                let src = match it["source"].as_array() {
                    Some(a) if a.len() == 3 => format!("{}:{}-{}", s(&a[0]), a[1], a[2]),
                    _ => String::new(),
                };
                println!(
                    "{:<22} {:<8} {:<10} {:>6}  {src}",
                    s(&it["id"]),
                    s(&it["tier"]),
                    s(&it["reason"]),
                    it["tokens"]
                );
            }
            let omitted = r["omitted"].as_array().map_or(0, Vec::len);
            if omitted > 0 {
                println!("omitted: {omitted}");
            }
        }
        Cmd::Reindex { repo } => {
            let r = client.post("/v1/reindex", &json!({ "repo": repo }))?;
            println!(
                "reindex: {} code files re-extracted, enqueued {}, upserted {}",
                r["code_files_reindexed"], r["enqueued"], r["upserted"]
            );
            print_notes(&r);
        }
        Cmd::Status => {
            let (task, _) = resolve_task(&cli)?;
            let r = client.get(&format!("/v1/tasks/{task}"), &[])?;
            let changed: Vec<&str> = r["changed"]
                .as_array()
                .map(|a| a.iter().filter_map(Value::as_str).collect())
                .unwrap_or_default();
            println!(
                "{} · {} · {} records",
                s(&r["task"]["slug"]),
                s(&r["task"]["status"]),
                r["record_count"]
            );
            println!(
                "owner {} · generation {}{}",
                s(&r["task"]["owner_agent"]),
                r["task"]["generation"],
                r["task"]["lease_expires"]
                    .as_str()
                    .map(|l| format!(" · lease until {l}"))
                    .unwrap_or_default()
            );
            println!("changed ({}): {}", changed.len(), changed.join(", "));
            if r["backup"]["remote"].is_string() {
                println!("{}", backup_line(&r["backup"]));
            }
            if let Some(v) = r["verdict"].as_object() {
                println!(
                    "verdict at head: {} by {}",
                    s(&v["verdict"]),
                    s(&v["reviewer"])
                );
            }
            if let Some(m) = r["merge"]["merge_commit"].as_str() {
                println!(
                    "merged as {} by {}",
                    m.chars().take(12).collect::<String>(),
                    s(&r["merge"]["merged_by"])
                );
            }
            let conflicts: Vec<&str> = r["merge"]["conflicts"]
                .as_array()
                .map(|a| a.iter().filter_map(Value::as_str).collect())
                .unwrap_or_default();
            if !conflicts.is_empty() {
                println!("conflicts: {}", conflicts.join(", "));
            }
        }
        Cmd::Submit { records } => {
            let records = records
                .iter()
                .map(|r| parse_record_spec(r))
                .collect::<Result<Vec<_>>>()?;
            let (task, _) = resolve_task(&cli)?;
            let mut body = json!({"agent": cli.agent});
            if !records.is_empty() {
                body["records"] = Value::Array(records);
            }
            let r = client.post_as(
                &format!("/v1/tasks/{task}/submit"),
                &body,
                generation_for(&cli, &task),
            )?;
            println!(
                "submitted ({}) · head {} · {} changed",
                s(&r["status"]),
                s(&r["head"]).chars().take(12).collect::<String>(),
                r["changed"].as_array().map_or(0, Vec::len)
            );
            let recorded = r["recorded"].as_array().map_or(0, Vec::len);
            if recorded > 0 {
                println!("recorded {recorded}");
            }
            if let Some(next) = r["next"].as_str().filter(|n| !n.is_empty()) {
                println!("{next}");
            }
            // Advisory, not an error: agents read stderr output as a failure and resubmit.
            let unrecorded = r["unrecorded_symbols"].as_array().map_or(0, Vec::len);
            if unrecorded > 0 {
                let plural = if unrecorded == 1 { "" } else { "s" };
                println!(
                    "advisory: {unrecorded} changed symbol{plural} with no record anchored \
                     (consider `av record --anchor path:Symbol`, or in one call \
                     `av submit --record \"decision: ... @ path:Symbol\"`)"
                );
            }
        }
        Cmd::Validate { command } => {
            let (task, _) = resolve_task(&cli)?;
            let cwd = std::env::current_dir().map_err(|e| CliError::Local(e.to_string()))?;
            let root = taskfile::find(&cwd).map(|(dir, _)| dir).unwrap_or(cwd);
            // Checkpoint first, so the checks that apply follow the files as they are now.
            client.post_as(
                &format!("/v1/tasks/{task}/checkpoint"),
                &json!({}),
                generation_for(&cli, &task),
            )?;
            let (code, tail) = run_capturing(
                std::process::Command::new(&command[0]).args(&command[1..]),
                &root,
                &command[0],
            )?;
            let wanted = client.get(&format!("/v1/tasks/{task}/checks"), &[])?;
            let mut checks = vec![];
            let mut checks_failed = false;
            for c in wanted["checks"].as_array().into_iter().flatten() {
                let cmd = s(&c["command"]);
                let (exit, out) = run_capturing(
                    std::process::Command::new("sh").args(["-c", cmd]),
                    &root,
                    cmd,
                )?;
                println!(
                    "check conv:{} {} (`{cmd}`, exit {exit})",
                    c["convention_id"],
                    if exit == 0 { "passed" } else { "FAILED" }
                );
                checks_failed |= exit != 0;
                checks.push(json!({"convention_id": c["convention_id"], "exit_code": exit, "output_tail": out}));
            }
            let r = client.post_as(
                &format!("/v1/tasks/{task}/validate"),
                &json!({"agent": cli.agent, "command": command.join(" "), "exit_code": code,
                        "output_tail": tail, "checks": checks}),
                generation_for(&cli, &task),
            )?;
            if checks_failed && code == 0 {
                return Err(CliError::Local(
                    "a required convention check failed; approval is refused until it passes"
                        .into(),
                ));
            }
            println!(
                "validation {} at head {} (exit {code})",
                if code == 0 { "passed" } else { "FAILED" },
                s(&r["head"]).chars().take(12).collect::<String>()
            );
            if code != 0 {
                return Err(CliError::Local(format!(
                    "`{}` exited {code}",
                    command.join(" ")
                )));
            }
        }
        Cmd::Review {
            task_id,
            verdict,
            findings,
        } => {
            let task = match task_id {
                Some(t) => t.clone(),
                None => resolve_task(&cli)?.0,
            };
            let findings: Vec<Value> = findings.iter().map(|f| parse_finding(f)).collect();
            let r = client.post(
                &format!("/v1/tasks/{task}/review"),
                &json!({"reviewer": cli.agent, "verdict": verdict.replace('-', "_"), "findings": findings}),
            )?;
            println!(
                "{} at head {} · task {}",
                s(&r["verdict"]),
                s(&r["head"]).chars().take(12).collect::<String>(),
                s(&r["status"])
            );
        }
        Cmd::Merge { task_id } => {
            let task = match task_id {
                Some(t) => t.clone(),
                None => resolve_task(&cli)?.0,
            };
            let r = client.post(
                &format!("/v1/tasks/{task}/merge"),
                &json!({"agent": cli.agent}),
            )?;
            let conflicts = strings(&r["conflicts"]);
            if conflicts.is_empty() {
                println!(
                    "merged · main {} · {} files",
                    s(&r["main"]).chars().take(12).collect::<String>(),
                    r["changed"].as_array().map_or(0, Vec::len)
                );
            } else {
                println!("conflicted · main unchanged · {}", conflicts.join(", "));
                return Err(CliError::Local(
                    "merge conflicts: the owner runs `av sync`, fixes the markers, then `av submit`"
                        .into(),
                ));
            }
        }
        Cmd::Sync => {
            let (task, _) = resolve_task(&cli)?;
            let r = client.post_as(
                &format!("/v1/tasks/{task}/sync"),
                &json!({"agent": cli.agent}),
                generation_for(&cli, &task),
            )?;
            println!(
                "on main {} · head {}",
                s(&r["base"]).chars().take(12).collect::<String>(),
                s(&r["head"]).chars().take(12).collect::<String>()
            );
            let conflicts = strings(&r["conflicts"]);
            if !conflicts.is_empty() {
                println!("conflicts ({}): {}", conflicts.len(), conflicts.join(", "));
            }
        }
        Cmd::Revert { task_id, reason } => {
            let task = match task_id {
                Some(t) => t.clone(),
                None => resolve_task(&cli)?.0,
            };
            let r = client.post(
                &format!("/v1/tasks/{task}/revert"),
                &json!({"agent": cli.agent, "reason": reason}),
            )?;
            println!(
                "reverted {} in {} · task {}",
                s(&r["reverted"]).chars().take(12).collect::<String>(),
                s(&r["main"]).chars().take(12).collect::<String>(),
                s(&r["status"])
            );
        }
        Cmd::Bisect {
            repo,
            trust,
            limit,
            work_dir,
        } => {
            let repo = repo_for(&cli, &client, repo)?;
            if !*trust {
                return Err(CliError::Local(format!(
                    "av bisect runs the check command stored in the daemon for repo {repo}; \
                     pass --trust to run it on this machine"
                )));
            }
            bisect(&client, &cli, &repo, *limit, work_dir.clone())?;
        }
        Cmd::Undo { task_id } => {
            let task = match task_id {
                Some(t) => t.clone(),
                None => resolve_task(&cli)?.0,
            };
            let r = client.post(
                &format!("/v1/tasks/{task}/undo"),
                &json!({"agent": cli.agent}),
            )?;
            println!(
                "{} · main back at {}",
                s(&r["status"]),
                s(&r["main"]).chars().take(12).collect::<String>()
            );
        }
        Cmd::Handoff {
            done,
            remaining,
            broken,
            questions,
            budget,
            needs,
            prefer,
        } => {
            let (task, _) = resolve_task(&cli)?;
            let r = client.post_as(
                &format!("/v1/tasks/{task}/handoff"),
                &json!({"agent": cli.agent, "done": done, "remaining": remaining,
                        "broken": broken, "questions": questions, "budget": budget,
                        "needs": needs, "prefer": prefer}),
                generation_for(&cli, &task),
            )?;
            println!(
                "{} · head {} · another agent continues with `av claim {task}`\n",
                s(&r["status"]),
                s(&r["head"]).chars().take(12).collect::<String>()
            );
            println!("{}", s(&r["bundle"]["markdown"]));
        }
        Cmd::Hook { cmd } => match cmd {
            HookCmd::SessionStart {
                budget,
                code,
                no_map,
                max_code,
            } => hook::session_start(&client, *budget, code, !*no_map, *max_code),
            HookCmd::PostEdit => hook::post_edit(&client, &cli.agent),
            HookCmd::PreRead => hook::pre_read(&client),
            HookCmd::Stop => hook::stop(&client, &cli.agent),
        },
        Cmd::Search { query, repo, limit } => {
            let repo = repo_for(&cli, &client, repo)?;
            let limit = limit.to_string();
            let r = client.get(
                &format!("/v1/repos/{repo}/search"),
                &[("q", query.as_str()), ("limit", limit.as_str())],
            )?;
            let hits = r["records"].as_array().cloned().unwrap_or_default();
            if hits.is_empty() {
                println!("no records match {query:?} in {repo}");
            }
            for h in hits {
                println!(
                    "rec:{} {}: {} (task {})",
                    short(s(&h["id"])),
                    s(&h["kind"]),
                    s(&h["summary"]),
                    short(s(&h["task_id"]))
                );
            }
        }
        Cmd::Log { repo, limit, path } => {
            let repo = repo_for(&cli, &client, repo)?;
            let limit = limit.to_string();
            let mut query = vec![("limit", limit.as_str())];
            if let Some(p) = path.as_deref() {
                query.push(("path", p));
            }
            let r = client.get(&format!("/v1/repos/{repo}/log"), &query)?;
            for c in r["commits"].as_array().into_iter().flatten() {
                let mark = match c["check"]["exit_code"].as_i64() {
                    Some(0) => "✓",
                    Some(_) => "✗",
                    None => "·",
                };
                println!(
                    "{mark} {} {}",
                    s(&c["change"]).chars().take(12).collect::<String>(),
                    s(&c["description"]).lines().next().unwrap_or("")
                );
            }
        }
        Cmd::Runner {
            repo,
            trust,
            once,
            work_dir,
        } => {
            let repo = repo_for(&cli, &client, repo)?;
            if !*trust {
                return Err(CliError::Local(format!(
                    "av runner runs the check command stored in the daemon for repo {repo}; \
                     pass --trust to run it on this machine"
                )));
            }
            let r = runner::Runner {
                client: &client,
                work_dir: work_dir
                    .clone()
                    .unwrap_or_else(|| runner::default_work_dir(&repo)),
                repo,
                name: cli.agent.clone(),
            };
            r.run(*once)?;
        }
        Cmd::Health { repo } => {
            let repo = repo_for(&cli, &client, repo)?;
            let h = client.get(&format!("/v1/repos/{repo}/health"), &[])?;
            match s(&h["state"]) {
                "red" => println!(
                    "main is red since {} (task {}): `{}` exit {}",
                    s(&h["since"]).chars().take(12).collect::<String>(),
                    h["task"].as_str().unwrap_or("?"),
                    s(&h["command"]),
                    h["exit_code"]
                ),
                "green" => println!(
                    "main is green (checked {})",
                    s(&h["checked"]).chars().take(12).collect::<String>()
                ),
                _ => println!("main is unchecked (run `av runner --trust`)"),
            }
        }
        Cmd::Why { target, repo } => {
            let repo = repo_for(&cli, &client, repo)?;
            let (path, line) = target
                .rsplit_once(':')
                .filter(|(_, l)| l.parse::<usize>().is_ok())
                .ok_or_else(|| CliError::Local(format!("expected path:line, got {target:?}")))?;
            let r = client.get(
                &format!("/v1/repos/{repo}/why"),
                &[("path", path), ("line", line)],
            )?;
            println!("{path}:{line}  {}", s(&r["line_text"]));
            println!(
                "last changed in {} {}",
                s(&r["commit"]).chars().take(12).collect::<String>(),
                s(&r["commit_description"])
            );
            if r["task"].is_object() {
                println!("task {}: {}", s(&r["task"]["slug"]), s(&r["task"]["goal"]));
                for rec in r["records"].as_array().into_iter().flatten() {
                    println!(
                        "- {} {}: {}",
                        s(&rec["id"]),
                        s(&rec["kind"]),
                        s(&rec["summary"])
                    );
                }
            } else {
                println!("(not from an av task: imported history)");
            }
        }
        Cmd::Show { path, at, repo } => {
            let repo = repo_for(&cli, &client, repo)?;
            let r = client.get(
                &format!("/v1/repos/{repo}/files/{path}"),
                &[("at", at.as_str())],
            )?;
            match r["content"].as_str() {
                Some(text) => print!("{text}"),
                None => println!("binary file, {} bytes", r["bytes"]),
            }
        }
        Cmd::Diff { from, to, repo } => {
            let repo = repo_for(&cli, &client, repo)?;
            let r = client.get(
                &format!("/v1/repos/{repo}/diff"),
                &[("from", from.as_str()), ("to", to.as_str())],
            )?;
            for f in r["files"].as_array().into_iter().flatten() {
                match f["diff"].as_str() {
                    Some(d) => print!("{d}"),
                    None => println!("{} {} (binary)", s(&f["status"]), s(&f["path"])),
                }
            }
        }
        Cmd::Abandon => {
            let (task, _) = resolve_task(&cli)?;
            let r = client.post_as(
                &format!("/v1/tasks/{task}/abandon"),
                &json!({"agent": cli.agent}),
                generation_for(&cli, &task),
            )?;
            println!("{}", s(&r["status"]));
        }
        Cmd::Bundle { pr, budget } => {
            let (task, _) = resolve_task(&cli)?;
            let budget = budget.to_string();
            let kind = if *pr { "pr" } else { "handoff" };
            let r = client.get(
                &format!("/v1/tasks/{task}/bundle"),
                &[("kind", kind), ("budget", budget.as_str())],
            )?;
            println!("{}", s(&r["markdown"]));
        }
        Cmd::Claim { task_id, budget } => {
            let r = client.post(
                &format!("/v1/tasks/{task_id}/claim"),
                &json!({"agent": cli.agent, "budget": budget}),
            )?;
            let task = &r["task"];
            let workspace = PathBuf::from(s(&task["workspace_path"]));
            let session = s(&r["session"]).to_owned();
            write_taskfile(
                &workspace,
                &TaskFile {
                    task: s(&task["id"]).to_owned(),
                    session: session.clone(),
                    generation: r["generation"].as_i64(),
                    agent: Some(cli.agent.clone()),
                    last_checkpoint_ms: None,
                    nudged_session: None,
                },
            );
            println!("workspace: {}", workspace.display());
            println!("task: {} ({})", s(&task["id"]), s(&task["slug"]));
            println!("session: {session}");
            println!("generation: {}\n", r["generation"]);
            println!("{}", s(&r["bundle"]["markdown"]));
        }
        Cmd::Agent {
            cmd:
                AgentCmd::Set {
                    name,
                    best,
                    can,
                    cost,
                    aliases,
                    max_open,
                },
        } => {
            let r = client.put(
                &format!("/v1/agents/{name}"),
                &json!({"best": best, "can": can, "cost_rank": cost, "aliases": aliases,
                        "max_open": max_open}),
            )?;
            println!("{}", agent_line(&r["agent"]));
        }
        Cmd::Repo { cmd: RepoCmd::List } => {
            let r = client.get("/v1/repos", &[])?;
            let repos = r.as_array().cloned().unwrap_or_default();
            if repos.is_empty() {
                println!("no repos (av repo add NAME PATH)");
            }
            for repo in &repos {
                let main: String = s(&repo["main_change"]).chars().take(12).collect();
                let main = if main.is_empty() { "?".into() } else { main };
                match repo["git_source"].as_str() {
                    Some(src) => println!("{}  main {main}  from {src}", s(&repo["name"])),
                    None => println!("{}  main {main}", s(&repo["name"])),
                }
            }
        }
        Cmd::Agent {
            cmd: AgentCmd::List,
        } => {
            let r = client.get("/v1/agents", &[])?;
            let agents = r["agents"].as_array().cloned().unwrap_or_default();
            if agents.is_empty() {
                println!("no profiles yet: av agent set NAME --best TAGS --can TAGS --cost N");
            }
            for a in &agents {
                println!("{}", agent_line(a));
            }
        }
        Cmd::Post {
            repo,
            title,
            goal,
            needs,
            prefer,
            priority,
            reserve,
        } => {
            let r = client.post(
                "/v1/backlog",
                &json!({"repo": repo, "title": title, "goal": goal, "needs": needs,
                        "prefer": prefer, "priority": priority, "reserve_secs": reserve,
                        "posted_by": cli.agent}),
            )?;
            println!(
                "posted {} · {}{}",
                s(&r["id"]),
                s(&r["title"]),
                tags_suffix(&r["needs"])
            );
        }
        Cmd::Backlog {
            cmd: Some(BacklogCmd::Drop { id }),
            ..
        } => {
            client.delete(&format!("/v1/backlog/{id}"))?;
            println!("dropped {id}");
        }
        Cmd::Backlog {
            cmd: None,
            repo,
            state,
        } => {
            let mut query: Vec<(&str, &str)> = vec![];
            if let Some(r) = &repo {
                query.push(("repo", r));
            }
            if let Some(st) = &state {
                query.push(("state", st));
            }
            let r = client.get("/v1/backlog", &query)?;
            let items = r["items"].as_array().cloned().unwrap_or_default();
            if items.is_empty() {
                println!("the backlog is empty");
            }
            for i in &items {
                let taken = i["taken_by"]
                    .as_str()
                    .map(|a| format!(" · taken by {a}"))
                    .unwrap_or_default();
                println!(
                    "{}  {}  {}  p{}  {}{}{}",
                    s(&i["id"]),
                    s(&i["state"]),
                    s(&i["repo"]),
                    i["priority"],
                    s(&i["title"]),
                    tags_suffix(&i["needs"]),
                    taken
                );
            }
        }
        Cmd::Watch {
            repo,
            every,
            max_runs,
            once,
            cmd,
        } => watch::run(
            &client,
            &watch::Watch {
                agent: &cli.agent,
                url: &cli.url,
                repo: repo.as_deref(),
                every: std::time::Duration::from_secs((*every).max(1)),
                max_runs: *max_runs,
                once: *once,
                cmd,
            },
        )?,
        Cmd::Next { repo, budget, wait } => {
            let session = cli
                .session
                .clone()
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
            let r = client.post(
                "/v1/backlog/next",
                &json!({"repo": repo, "agent": cli.agent, "session": session, "budget": budget,
                        "wait_secs": wait}),
            )?;
            if r["task"].is_null() && r["at_capacity"].is_object() {
                let open = &r["at_capacity"]["open"];
                println!(
                    "You hold {open} open task{}, your limit: submit or hand off one before \
                     taking new work. Reviews of others' work are still offered.",
                    if open == 1 { "" } else { "s" }
                );
                return Ok(());
            }
            if r["task"].is_null() {
                println!(
                    "No task is waiting for you ({} queued for other agents or still reserved).",
                    r["queued"]
                );
                return Ok(());
            }
            let task = &r["task"];
            if r["from"] == "review" {
                // someone else's task: nothing is written here; review it by id
                println!(
                    "review: task {} ({}/{}), owner: {}",
                    s(&task["id"]),
                    s(&task["repo"]),
                    s(&task["slug"]),
                    s(&task["owner_agent"])
                );
                println!("workspace: {}", s(&task["workspace_path"]));
                println!("{}", s(&r["next"]));
                return Ok(());
            }
            // a claimed or resumed task comes with its handoff bundle
            let handoff = r["from"] == "handoff" || r["from"] == "abandoned";
            let workspace = PathBuf::from(s(&task["workspace_path"]));
            // a claim starts a fresh session; the owner name is the profile's, not an alias
            let session = if handoff {
                s(&r["session"]).to_owned()
            } else {
                session
            };
            let generation = if handoff {
                r["generation"].as_i64()
            } else {
                task["generation"].as_i64()
            };
            write_taskfile(
                &workspace,
                &TaskFile {
                    task: s(&task["id"]).to_owned(),
                    session: session.clone(),
                    generation,
                    agent: Some(s(&task["owner_agent"]).to_owned()),
                    last_checkpoint_ms: None,
                    nudged_session: None,
                },
            );
            println!("workspace: {}", workspace.display());
            println!("task: {} ({})", s(&task["id"]), s(&task["slug"]));
            println!("session: {session}");
            if handoff {
                println!("generation: {}\n", r["generation"]);
                println!("{}", s(&r["bundle"]["markdown"]));
            } else {
                println!();
                println!("{}", s(&r["pack"]["markdown"]));
                print_receipt(&r["pack"]);
            }
        }
        Cmd::Expand { id } => {
            let (task, session) = resolve_task(&cli)?;
            let r = client.get(
                &format!("/v1/tasks/{task}/items/{id}"),
                &[("session", session.as_str())],
            )?;
            println!("{}", s(&r["full"]));
            if let Some(total) = r["session_expand_tokens"].as_u64() {
                eprintln!(
                    "av: expanded {} tokens (session total {total})",
                    r["tokens"]
                );
            }
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("av: {e}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_anchor_splits_path_and_symbol() {
        assert_eq!(
            parse_anchor("src/a.py:Greeter.hello"),
            json!({"path": "src/a.py", "symbol": "Greeter.hello"})
        );
        assert_eq!(
            parse_anchor("README.md"),
            json!({"path": "README.md", "symbol": null})
        );
        assert_eq!(
            parse_anchor("a.py:"),
            json!({"path": "a.py", "symbol": null})
        );
    }

    #[test]
    fn parse_record_spec_splits_kind_summary_and_anchor() {
        assert_eq!(
            parse_record_spec("decision: Use allocate @ tallyho/money.py:Money.allocate").unwrap(),
            json!({"kind": "decision", "summary": "Use allocate",
                   "anchors": [{"path": "tallyho/money.py", "symbol": "Money.allocate"}]})
        );
        assert_eq!(
            parse_record_spec("attempt: tried caching").unwrap(),
            json!({"kind": "attempt", "summary": "tried caching", "anchors": []})
        );
        let e = parse_record_spec("no colon here").unwrap_err().to_string();
        assert!(e.contains("KIND: SUMMARY @ path[:Symbol]"), "{e}");
    }

    #[test]
    fn context_budget_defaults_to_800() {
        let cli = Cli::try_parse_from(["av", "context"]).unwrap();
        match cli.cmd {
            Cmd::Context { budget, .. } => assert_eq!(budget, 800),
            _ => panic!("not a context command"),
        }
    }

    #[test]
    fn context_code_defaults_to_signatures() {
        let cli = Cli::try_parse_from(["av", "context"]).unwrap();
        match cli.cmd {
            Cmd::Context { code, .. } => assert_eq!(code, "signatures"),
            _ => panic!("not a context command"),
        }
        assert!(Cli::try_parse_from(["av", "context", "--code", "full"]).is_ok());
        assert!(Cli::try_parse_from(["av", "context", "--code", "bodies"]).is_err());
    }

    #[test]
    fn task_start_budget_defaults_to_800() {
        let cli = Cli::try_parse_from([
            "av", "task", "start", "--repo", "r", "--title", "t", "--goal", "g",
        ])
        .unwrap();
        match cli.cmd {
            Cmd::Task {
                cmd: TaskCmd::Start { budget, .. },
            } => assert_eq!(budget, 800),
            _ => panic!("not a task start command"),
        }
    }

    #[test]
    fn short_matches_the_server_record_id_format() {
        assert_eq!(short("1a2b3c4d-0000-4000-8000-000000000000"), "1a2b3c4d");
    }
}
