//! `av hook …` (Plan 3 Task 9): commands for agent hooks (Claude Code `SessionStart`,
//! `PreToolUse`, `PostToolUse` and `Stop`). A hook must never break the agent, so every failure is one line on stderr and
//! exit code 0, and a directory without a task is silent.

use std::io::{IsTerminal, Read, Write as _};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde_json::{Value, json};

use crate::client::Client;
use crate::footprint;
use crate::taskfile::{self, TaskFile};

/// `post-edit` checkpoints at most this often.
const DEBOUNCE_MS: u64 = 2_000;

/// The hook JSON Claude Code writes to stdin, if any.
fn hook_input() -> Value {
    let stdin = std::io::stdin();
    let mut text = String::new();
    if stdin.is_terminal() || stdin.lock().read_to_string(&mut text).is_err() {
        return Value::Null;
    }
    serde_json::from_str(&text).unwrap_or(Value::Null)
}

/// Where to look for `.av-task`: the edited file's directory, else the session's `cwd` from the
/// hook input, else the process's working directory.
fn start_dir(input: &Value) -> Option<PathBuf> {
    if let Some(dir) = input["tool_input"]["file_path"]
        .as_str()
        .and_then(|f| PathBuf::from(f).parent().map(PathBuf::from))
    {
        return Some(dir);
    }
    match input["cwd"].as_str() {
        Some(cwd) => Some(PathBuf::from(cwd)),
        None => std::env::current_dir().ok(),
    }
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or_default()
}

/// Prints a small context pack for the task of this directory. After a compaction or `/clear`
/// the agent has lost what it was sent, so the session starts over and everything is sent again.
/// Then gives the task a scratch directory (`$AV_SCRATCH`, removed with the task) and, task or
/// not, takes the footprint snapshot the Stop hook compares against (Plan 11).
pub fn session_start(client: &Client, budget: u32, code: &str, map: bool, max_code: Option<usize>) {
    let input = hook_input();
    let found = start_dir(&input).and_then(|d| taskfile::find(&d));
    let session = input["session_id"]
        .as_str()
        .map(str::to_owned)
        .or_else(|| found.as_ref().map(|(_, tf)| tf.session.clone()));
    let task_ws_root = found
        .as_ref()
        .and_then(|(dir, _)| dir.parent()?.parent().map(Path::to_path_buf));
    if let Some((dir, tf)) = found {
        print_pack(client, &input, &dir, tf, budget, code, map, max_code);
        if let Some(scratch) = scratch_dir(client, &dir) {
            announce_scratch(&scratch);
        }
    }
    if let Some(session) = session {
        snapshot_footprint(client, &session, task_ws_root);
    }
}

#[allow(clippy::too_many_arguments)]
fn print_pack(
    client: &Client,
    input: &Value,
    dir: &Path,
    mut tf: TaskFile,
    budget: u32,
    code: &str,
    map: bool,
    max_code: Option<usize>,
) {
    if matches!(input["source"].as_str(), Some("compact" | "clear")) {
        tf.session = uuid::Uuid::new_v4().to_string();
        if let Err(e) = taskfile::write(dir, &tf) {
            eprintln!(
                "av: hook session-start: cannot update {}: {e}",
                taskfile::NAME
            );
        }
    }
    let budget = budget.to_string();
    let map = map.to_string();
    let mut query = vec![
        ("budget", budget.as_str()),
        ("session", tf.session.as_str()),
        ("code", code),
        ("map", map.as_str()),
    ];
    let max_code = max_code.map(|n| n.to_string());
    if let Some(n) = &max_code {
        query.push(("max_code", n.as_str()));
    }
    match client.get(&format!("/v1/tasks/{}/context", tf.task), &query) {
        Ok(pack) => println!("{}", pack["markdown"].as_str().unwrap_or_default()),
        Err(e) => eprintln!("av: hook session-start: {e}"),
    }
}

/// `<ws_root>/<repo>/.scratch/<slug>` for the task workspace `dir`, created; the daemon removes
/// it with the workspace. `None` when `dir` is not the task's workspace as the daemon knows it
/// (a `.av-task` copied elsewhere), so nothing is created beside a directory av does not own.
fn scratch_dir(client: &Client, dir: &Path) -> Option<PathBuf> {
    let tf = taskfile::find(dir)?.1;
    let task = client.get(&format!("/v1/tasks/{}", tf.task), &[]).ok()?;
    let ws = PathBuf::from(task["task"]["workspace_path"].as_str()?);
    let same = |a: &Path, b: &Path| {
        a == b || matches!((a.canonicalize(), b.canonicalize()), (Ok(x), Ok(y)) if x == y)
    };
    if !same(&ws, dir) {
        return None;
    }
    let scratch = dir.parent()?.join(".scratch").join(dir.file_name()?);
    if let Err(e) = std::fs::create_dir_all(&scratch) {
        eprintln!(
            "av: hook session-start: cannot create {}: {e}",
            scratch.display()
        );
        return None;
    }
    Some(scratch)
}

/// `'…'` for sh.
fn sh_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

/// Exports `AV_SCRATCH` to the session's later commands (`$CLAUDE_ENV_FILE`) and says so after
/// the pack (in the hook, so the packs themselves do not change).
fn announce_scratch(scratch: &Path) {
    let exported = std::env::var_os("CLAUDE_ENV_FILE")
        .filter(|f| !f.is_empty())
        .is_some_and(|file| {
            let line = format!(
                "export AV_SCRATCH={}\n",
                sh_quote(&scratch.to_string_lossy())
            );
            let written = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&file)
                .and_then(|mut f| f.write_all(line.as_bytes()));
            if let Err(e) = &written {
                eprintln!("av: hook session-start: cannot write CLAUDE_ENV_FILE: {e}");
            }
            written.is_ok()
        });
    if exported {
        println!("Temporary files: $AV_SCRATCH (removed with the task).");
    } else {
        println!(
            "Temporary files: {} (removed with the task).",
            scratch.display()
        );
    }
}

/// `AV_FOOTPRINT=off` (or `0`, `false`, `no`) turns the footprint off.
fn footprint_on() -> bool {
    !matches!(
        std::env::var("AV_FOOTPRINT")
            .unwrap_or_default()
            .trim()
            .to_ascii_lowercase()
            .as_str(),
        "off" | "0" | "false" | "no"
    )
}

fn env_dir(key: &str) -> Option<PathBuf> {
    std::env::var_os(key)
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
}

/// The home dir, the temp dir and `AV_FOOTPRINT_ROOTS` (the platform's path-list separator).
fn footprint_roots() -> Vec<PathBuf> {
    let mut roots: Vec<PathBuf> = crate::local::home_dir().into_iter().collect();
    roots.push(std::env::temp_dir());
    if let Some(more) = std::env::var_os("AV_FOOTPRINT_ROOTS") {
        roots.extend(std::env::split_paths(&more).filter(|p| !p.as_os_str().is_empty()));
    }
    roots.dedup();
    roots
}

/// What the footprint never counts: the workspace root (`AV_WS_ROOT`, else `~/ws`; reclaimed
/// with the tasks), the task's own workspace root, and av's home (its Postgres and these files).
fn footprint_exclude(task_ws_root: Option<PathBuf>) -> Vec<PathBuf> {
    let mut ex: Vec<PathBuf> = env_dir("AV_WS_ROOT")
        .or_else(|| crate::local::home_dir().map(|h| h.join("ws")))
        .into_iter()
        .collect();
    ex.extend(task_ws_root);
    ex.push(crate::local::av_home());
    ex
}

fn footprint_deadline() -> Instant {
    Instant::now() + Duration::from_secs(env_number("AV_FOOTPRINT_SECS", 20))
}

/// `$AV_HOME/footprint/<session>.json`, the session id reduced to a safe file name.
fn footprint_file(session: &str) -> PathBuf {
    let safe: String = session
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    crate::local::av_home()
        .join("footprint")
        .join(format!("{safe}.json"))
}

fn dirs_json(dirs: &[(PathBuf, u64)]) -> Value {
    dirs.iter()
        .map(|(p, b)| json!({"path": p, "bytes": b}))
        .collect()
}

fn dirs_of(v: &Value) -> Vec<(PathBuf, u64)> {
    v.as_array()
        .map(|a| {
            a.iter()
                .filter_map(|d| Some((PathBuf::from(d["path"].as_str()?), d["bytes"].as_u64()?)))
                .collect()
        })
        .unwrap_or_default()
}

fn paths_of(v: &Value) -> Vec<PathBuf> {
    v.as_array()
        .map(|a| {
            a.iter()
                .filter_map(Value::as_str)
                .map(PathBuf::from)
                .collect()
        })
        .unwrap_or_default()
}

/// Session start: the heavy directories as they are now, saved for the Stop hook. Only when the
/// daemon answers (av is in use) and the footprint is on.
fn snapshot_footprint(client: &Client, session: &str, task_ws_root: Option<PathBuf>) {
    if !footprint_on() || client.get_text("/healthz").is_err() {
        return;
    }
    let roots = footprint_roots();
    let exclude = footprint_exclude(task_ws_root);
    let scan = footprint::scan(&roots, &exclude, footprint_deadline());
    let file = footprint_file(session);
    let snap = json!({
        "at": now_ms() / 1000,
        "roots": roots,
        "exclude": exclude,
        "dirs": dirs_json(&scan.dirs),
        "partial": scan.partial,
        "warned": false,
    });
    let saved = file
        .parent()
        .map_or(Ok(()), std::fs::create_dir_all)
        .and_then(|()| std::fs::write(&file, snap.to_string()));
    if let Err(e) = saved {
        eprintln!(
            "av: hook session-start: cannot save {}: {e}",
            file.display()
        );
    }
}

/// Stop: what the session added since its snapshot. Blocks once per session (the snapshot's
/// `warned`) when that is at least `AV_FOOTPRINT_WARN_MB`, and reports any addition to the
/// daemon (`GET /v1/disk` shows it under `outside`). Returns whether it blocked.
fn footprint_stop(
    client: &Client,
    agent: &str,
    session: &str,
    task: Option<&(PathBuf, TaskFile)>,
) -> bool {
    if !footprint_on() {
        return false;
    }
    let file = footprint_file(session);
    let Some(mut snap) = std::fs::read_to_string(&file)
        .ok()
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
    else {
        return false;
    };
    let after = footprint::scan(
        &paths_of(&snap["roots"]),
        &paths_of(&snap["exclude"]),
        footprint_deadline(),
    );
    let added = footprint::diff(&dirs_of(&snap["dirs"]), &after.dirs);
    if added.is_empty() {
        return false;
    }
    let partial = after.partial || snap["partial"] == true;
    let total: u64 = added.iter().map(|(_, b)| b).sum();
    let warn_at = env_number("AV_FOOTPRINT_WARN_MB", 512u64).saturating_mul(1024 * 1024);
    let mut blocked = false;
    if total >= warn_at && snap["warned"] != true {
        snap["warned"] = json!(true);
        // without the mark this would block every stop, so no mark, no block
        match std::fs::write(&file, snap.to_string()) {
            Ok(()) => {
                let reason = footprint_reason(&added, total, task.map(|(dir, _)| dir.as_path()));
                println!("{}", json!({"decision": "block", "reason": reason}));
                blocked = true;
            }
            Err(e) => eprintln!("av: hook stop: cannot update {}: {e}", file.display()),
        }
    }
    let body = json!({
        "session": session,
        "agent": agent,
        "task": task.map(|(_, tf)| tf.task.clone()),
        "added": dirs_json(&added),
        "partial": partial,
    });
    if let Err(e) = client.post("/v1/footprint", &body) {
        eprintln!("av: hook stop: footprint not reported: {e}");
    }
    blocked
}

/// Most paths the footprint block names.
const FOOTPRINT_PATHS: usize = 5;

fn footprint_reason(added: &[(PathBuf, u64)], total: u64, task_dir: Option<&Path>) -> String {
    let mut list: Vec<String> = added
        .iter()
        .take(FOOTPRINT_PATHS)
        .map(|(p, b)| format!("{} ({})", p.display(), footprint::human(*b)))
        .collect();
    if added.len() > FOOTPRINT_PATHS {
        list.push(format!("and {} more", added.len() - FOOTPRINT_PATHS));
    }
    let what = format!(
        "av: this session added {} outside any task workspace: {}.",
        footprint::human(total),
        list.join(", ")
    );
    match task_dir.and_then(Path::parent) {
        Some(repo_dir) => format!(
            "{what} Build inside the workspace (removed when the task ends) or the repo's cache \
             {}, move it to $AV_SCRATCH, or remove it; then record where anything kept lives.",
            repo_dir.join(".cache").display()
        ),
        None => format!(
            "{what} Remove what is no longer needed, or say where anything kept lives and why."
        ),
    }
}

/// Before the agent reads or edits a file: what other tasks recorded about that file (decisions,
/// dead ends), as Claude Code `additionalContext`, each record once per session. Prints nothing
/// when there is nothing new or the file is outside the task's workspace.
pub fn pre_read(client: &Client) {
    let input = hook_input();
    let Some((dir, tf)) = start_dir(&input).and_then(|d| taskfile::find(&d)) else {
        return;
    };
    let Some(file) = input["tool_input"]["file_path"].as_str().map(PathBuf::from) else {
        return;
    };
    let Some(path) = file
        .strip_prefix(&dir)
        .ok()
        .and_then(|p| p.to_str())
        .map(|p| p.replace('\\', "/"))
    else {
        return;
    };
    let query = [("path", path.as_str()), ("session", tf.session.as_str())];
    match client.get(&format!("/v1/tasks/{}/records/at", tf.task), &query) {
        Ok(r) => {
            let md = r["markdown"].as_str().unwrap_or_default();
            if !md.is_empty() {
                let event = input["hook_event_name"].as_str().unwrap_or("PreToolUse");
                println!(
                    "{}",
                    json!({"hookSpecificOutput": {"hookEventName": event, "additionalContext": md}})
                );
            }
        }
        Err(e) => eprintln!("av: hook pre-read: {e}"),
    }
}

/// Checkpoints the task of this directory, unless the last hook checkpoint was under 2 s ago.
pub fn post_edit(client: &Client, agent: &str) {
    let input = hook_input();
    let Some((dir, mut tf)) = start_dir(&input).and_then(|d| taskfile::find(&d)) else {
        return;
    };
    let now = now_ms();
    if tf
        .last_checkpoint_ms
        .is_some_and(|t| now.saturating_sub(t) < DEBOUNCE_MS)
    {
        return;
    }
    // stamp first, so a burst of edits right behind this one is debounced too
    tf.last_checkpoint_ms = Some(now);
    if let Err(e) = taskfile::write(&dir, &tf) {
        eprintln!("av: hook post-edit: cannot update {}: {e}", taskfile::NAME);
    }
    let generation = generation(&tf, agent);
    if let Err(e) = client.post_as(
        &format!("/v1/tasks/{}/checkpoint", tf.task),
        &json!({}),
        generation,
    ) {
        eprintln!("av: hook post-edit: checkpoint failed: {e}");
    }
}

/// Most symbol names the stop request lists.
const NUDGE_SYMBOLS: usize = 8;

/// How long `stop` waits for work, in seconds (`AV_HOOK_WAIT_SECS`); 0 turns the wait off.
const DEFAULT_WAIT_SECS: u64 = 600;
/// Most times `stop` continues one agent session (`AV_HOOK_MAX_CONTINUES`).
const DEFAULT_MAX_CONTINUES: u32 = 20;
/// Seconds between two looks at the backlog while waiting, as in `av watch`'s default.
const POLL: Duration = Duration::from_secs(15);

/// A number from the environment; the default when it is unset or not a number.
fn env_number<T: std::str::FromStr>(name: &str, default: T) -> T {
    std::env::var(name)
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .unwrap_or(default)
}

/// Seconds past the wait that the Stop hook may run before Claude Code cancels it (a canceled hook's
/// output is discarded, so the timeout must exceed the wait).
const STOP_TIMEOUT_MARGIN: u64 = 60;

/// The Stop hook's command timeout for a wait of `wait` seconds.
pub fn stop_timeout_for(wait: u64) -> u64 {
    wait.saturating_add(STOP_TIMEOUT_MARGIN)
}

/// The Stop hook timeout `av init` writes: `AV_HOOK_WAIT_SECS` (default 600) plus a margin, as
/// `integrations/claude-code/install.sh` sets it.
pub fn stop_timeout_secs() -> u64 {
    stop_timeout_for(env_number("AV_HOOK_WAIT_SECS", DEFAULT_WAIT_SECS))
}

/// Before the agent ends its turn (Claude Code `Stop`), two checks, at most one block per call:
/// 1. if the task has changes that no record explains, block once per agent session and ask for
///    the decisions and dead ends behind them (records that are never written are the likelier
///    way knowledge fails to reach the next agent);
/// 2. else, if av has work that fits this agent (a dry run of `next`, like `av watch`), block
///    with a reason that tells it to call `start_task`, waiting up to `AV_HOOK_WAIT_SECS` for
///    work to appear. The wait costs no model tokens. `AV_HOOK_MAX_CONTINUES` caps how often one
///    session is continued, so the loop always ends.
///
/// Before both, task or not: when the session added more than `AV_FOOTPRINT_WARN_MB` outside any
/// task workspace since it started, block once and say where (`footprint_stop`).
///
/// Any failure allows the stop, and a directory without a task is silent.
pub fn stop(client: &Client, agent: &str) {
    let input = hook_input();
    let found = start_dir(&input).and_then(|d| taskfile::find(&d));
    let session = input["session_id"]
        .as_str()
        .map(str::to_owned)
        .or_else(|| found.as_ref().map(|(_, tf)| tf.session.clone()));
    if let Some(session) = &session
        && footprint_stop(client, agent, session, found.as_ref())
    {
        return;
    }
    let Some((dir, mut tf)) = found else {
        return;
    };
    let session = session.unwrap_or_else(|| tf.session.clone());
    if input["stop_hook_active"] != true && nudge_to_record(client, agent, &dir, &mut tf, &session)
    {
        return;
    }
    keep_busy(client, agent, &dir, &mut tf, &session);
}

/// Check 1: prints the block and returns true when it asked the agent to record its changes.
fn nudge_to_record(
    client: &Client,
    agent: &str,
    dir: &std::path::Path,
    tf: &mut TaskFile,
    session: &str,
) -> bool {
    if tf.nudged_session.as_deref() == Some(session) {
        return false;
    }
    // the last edits may be inside post-edit's debounce
    if let Err(e) = client.post_as(
        &format!("/v1/tasks/{}/checkpoint", tf.task),
        &json!({}),
        generation(tf, agent),
    ) {
        eprintln!("av: hook stop: checkpoint failed: {e}");
        return false;
    }
    let r = match client.get(&format!("/v1/tasks/{}/unrecorded", tf.task), &[]) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("av: hook stop: {e}");
            return false;
        }
    };
    let syms: Vec<&str> = r["unrecorded_symbols"]
        .as_array()
        .map(|a| a.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();
    if r["status"] != "open" || syms.is_empty() {
        return false;
    }
    tf.nudged_session = Some(session.to_owned());
    if let Err(e) = taskfile::write(dir, tf) {
        eprintln!("av: hook stop: cannot update {}: {e}", taskfile::NAME);
    }
    let mut names = syms[..syms.len().min(NUDGE_SYMBOLS)].join(", ");
    if syms.len() > NUDGE_SYMBOLS {
        names.push_str(&format!(" and {} more", syms.len() - NUDGE_SYMBOLS));
    }
    let reason = format!(
        "av: no record explains your changes to {names}. Record why in one call so the next \
         agent does not redo or undo it: `av record --json -` with a list of {{kind, summary, \
         anchors}}, or `av submit --record \"decision: ... @ path:Symbol\"` if you are done. \
         Use kind attempt for anything you tried that failed. If nothing is worth recording, \
         say so and stop."
    );
    println!("{}", json!({"decision": "block", "reason": reason}));
    true
}

/// What a dry run of `next` says is waiting, as a phrase for the stop reason.
fn waiting(would: &Value) -> String {
    let what = match would["from"].as_str() {
        Some("review") => "a review is waiting",
        Some("handoff") => "a handed-off task is waiting",
        Some("abandoned") => "an abandoned task of yours is waiting",
        _ => "work is waiting",
    };
    let title = would["title"].as_str().unwrap_or("(untitled)");
    match would["repo"].as_str() {
        Some(repo) => format!("{what}: {title} ({repo})"),
        None => format!("{what}: {title}"),
    }
}

/// Check 2: prints the block and counts it when av has work for this agent, polling for up to
/// `AV_HOOK_WAIT_SECS`. Returns without printing when nothing fits, the cap is reached, or the
/// daemon cannot be asked.
fn keep_busy(
    client: &Client,
    agent: &str,
    dir: &std::path::Path,
    tf: &mut TaskFile,
    session: &str,
) {
    let wait = env_number("AV_HOOK_WAIT_SECS", DEFAULT_WAIT_SECS);
    if wait == 0 {
        return;
    }
    let max = env_number("AV_HOOK_MAX_CONTINUES", DEFAULT_MAX_CONTINUES);
    let used = if tf.continue_session.as_deref() == Some(session) {
        tf.continues
    } else {
        0
    };
    if used >= max {
        return;
    }
    let deadline = Instant::now() + Duration::from_secs(wait);
    let would = loop {
        let r = client.post(
            "/v1/backlog/next",
            &json!({"agent": agent, "session": "av-hook", "dry_run": true}),
        );
        match r {
            Ok(r) if !r["would"].is_null() => break r["would"].clone(),
            Ok(_) => {}
            Err(e) => {
                eprintln!("av: hook stop: {e}");
                return;
            }
        }
        let left = deadline.saturating_duration_since(Instant::now());
        if left.is_zero() {
            return;
        }
        std::thread::sleep(left.min(POLL));
    };
    tf.continue_session = Some(session.to_owned());
    tf.continues = used + 1;
    if let Err(e) = taskfile::write(dir, tf) {
        eprintln!("av: hook stop: cannot update {}: {e}", taskfile::NAME);
    }
    let reason = format!("av: {} - call start_task with no title", waiting(&would));
    println!("{}", json!({"decision": "block", "reason": reason}));
}

/// As `generation_for` in main.rs, for the hook's own task file.
fn generation(tf: &TaskFile, agent: &str) -> Option<i64> {
    match &tf.agent {
        Some(owner) if owner != agent => Some(0),
        _ => tf.generation,
    }
}

/// Which agent `av hook` acts as.
///
/// `named` is true when `--agent` or a non-empty `AV_AGENT` was set (`cli_agent` is that
/// value). Otherwise a Grok hook (`grok_hook`: `GROK_HOOK_EVENT` or `GROK_SESSION_ID`) acts as
/// `grok`, and anything else keeps `cli_agent` (the Claude default, `agent` when unset).
pub fn pick_hook_agent(cli_agent: &str, named: bool, grok_hook: bool) -> String {
    if named {
        cli_agent.to_owned()
    } else if grok_hook {
        "grok".to_owned()
    } else {
        cli_agent.to_owned()
    }
}

/// `pick_hook_agent` for this process: `--agent` / `AV_AGENT`, else `grok` under Grok's hook
/// environment, else `cli_agent`.
pub fn hook_agent(cli_agent: &str) -> String {
    let named =
        agent_flag_present() || std::env::var("AV_AGENT").is_ok_and(|v| !v.trim().is_empty());
    let grok_hook = std::env::var_os("GROK_HOOK_EVENT").is_some()
        || std::env::var_os("GROK_SESSION_ID").is_some();
    pick_hook_agent(cli_agent, named, grok_hook)
}

fn agent_flag_present() -> bool {
    for arg in std::env::args().skip(1) {
        if arg == "--" {
            break;
        }
        if arg == "--agent" || arg.starts_with("--agent=") {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hook_agent_is_grok_under_grok_and_the_claude_default_otherwise() {
        assert_eq!(pick_hook_agent("agent", false, true), "grok");
        assert_eq!(pick_hook_agent("agent", false, false), "agent");
        assert_eq!(stop_timeout_for(600), 660);
    }

    #[test]
    fn an_explicit_agent_wins_over_the_grok_hook_environment() {
        assert_eq!(pick_hook_agent("muse", true, true), "muse");
        assert_eq!(pick_hook_agent("cursor", true, false), "cursor");
    }
}
