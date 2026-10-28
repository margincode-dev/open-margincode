//! `av hook …` (Plan 3 Task 9): commands for agent hooks (Claude Code `SessionStart` and
//! `PostToolUse`). A hook must never break the agent, so every failure is one line on stderr and
//! exit code 0, and a directory without a task is silent.

use std::io::{IsTerminal, Read};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

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

/// As `generation_for` in main.rs, for the hook's own task file.
fn generation(tf: &TaskFile, agent: &str) -> Option<i64> {
    match &tf.agent {
        Some(owner) if owner != agent => Some(0),
        _ => tf.generation,
    }
}
