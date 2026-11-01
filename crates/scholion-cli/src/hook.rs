//! `av hook …` (Plan 3 Task 9): commands for agent hooks (Claude Code `SessionStart`,
//! `PreToolUse`, `PostToolUse` and `Stop`). A hook must never break the agent, so every failure is one line on stderr and
//! exit code 0, and a directory without a task is silent.

use std::io::{IsTerminal, Read};
use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde_json::{Value, json};

use crate::client::Client;
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
pub fn session_start(client: &Client, budget: u32, code: &str, map: bool, max_code: Option<usize>) {
    let input = hook_input();
    let Some((dir, mut tf)) = start_dir(&input).and_then(|d| taskfile::find(&d)) else {
        return;
    };
    if matches!(input["source"].as_str(), Some("compact" | "clear")) {
        tf.session = uuid::Uuid::new_v4().to_string();
        if let Err(e) = taskfile::write(&dir, &tf) {
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
/// Any failure allows the stop, and a directory without a task is silent.
pub fn stop(client: &Client, agent: &str) {
    let input = hook_input();
    let Some((dir, mut tf)) = start_dir(&input).and_then(|d| taskfile::find(&d)) else {
        return;
    };
    let session = input["session_id"]
        .as_str()
        .unwrap_or(&tf.session)
        .to_owned();
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
