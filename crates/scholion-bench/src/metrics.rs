//! Per-session metrics (Plan 4 "Metrics").

use globset::Glob;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::events::{ParsedSession, ToolUse};
use crate::spec::TaskSpec;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionMetrics {
    /// `input + cache_creation + output` over the messages up to and including the one that
    /// makes the first relevant edit; `None` without one.
    pub tokens_to_first_edit: Option<u64>,
    /// The `result` event's session usage (same fields) when present, else the sum over messages.
    pub tokens_total: u64,
    /// `total_cost_usd`, or the per-session cap when the stream has no `result` event.
    pub cost_usd: f64,
    pub turns: u32,
    pub tool_calls: u32,
    pub repeated_dead_ends: u32,
    pub truncated: bool,
}

/// Tools that write files, and the input key that names the file.
fn edit_target(t: &ToolUse) -> Option<&str> {
    let key = match t.name.as_str() {
        "Edit" | "Write" | "MultiEdit" => "file_path",
        "NotebookEdit" => "notebook_path",
        _ => return None,
    };
    t.input[key].as_str()
}

/// The text an edit writes (never `old_string`: removing dead-end code is not repeating it).
fn written_texts(t: &ToolUse) -> Vec<&str> {
    match t.name.as_str() {
        "Edit" => t.input["new_string"].as_str().into_iter().collect(),
        "Write" => t.input["content"].as_str().into_iter().collect(),
        "MultiEdit" => t.input["edits"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|e| e["new_string"].as_str())
            .collect(),
        "NotebookEdit" => t.input["new_source"].as_str().into_iter().collect(),
        _ => vec![],
    }
}

/// `path` relative to the session's working directory.
pub fn relative<'a>(path: &'a str, cwd: Option<&str>) -> &'a str {
    if let Some(cwd) = cwd
        && let Some(rest) = path.strip_prefix(cwd.trim_end_matches('/'))
        && let Some(rest) = rest.strip_prefix('/')
    {
        return rest;
    }
    path.strip_prefix("./").unwrap_or(path)
}

pub fn compute(spec: &TaskSpec, s: &ParsedSession, session_cap_usd: f64) -> SessionMetrics {
    let cwd = s.cwd.as_deref();
    let dead_ends: Vec<(globset::GlobMatcher, Regex)> = spec
        .dead_ends
        .iter()
        .filter_map(|d| {
            Some((
                Glob::new(&d.file_glob).ok()?.compile_matcher(),
                Regex::new(&d.pattern).ok()?,
            ))
        })
        .collect();

    let mut running = 0u64;
    let mut first_edit = None;
    let mut tool_calls = 0u32;
    let mut repeated = 0u32;
    for m in &s.messages {
        running += m.usage.counted();
        for t in &m.tool_uses {
            tool_calls += 1;
            let Some(target) = edit_target(t) else {
                continue;
            };
            let rel = relative(target, cwd);
            if first_edit.is_none() && spec.is_relevant(rel) {
                first_edit = Some(running);
            }
            let texts = written_texts(t);
            if dead_ends
                .iter()
                .any(|(glob, re)| glob.is_match(rel) && texts.iter().any(|x| re.is_match(x)))
            {
                repeated += 1;
            }
        }
    }
    let total = match &s.result {
        Some(r) if r.usage.counted() > 0 => r.usage.counted(),
        _ => running,
    };
    SessionMetrics {
        // per-message output counts can lag the session's; never report more than the total
        tokens_to_first_edit: first_edit.map(|t| t.min(total)),
        tokens_total: total,
        cost_usd: s.result.as_ref().map_or(session_cap_usd, |r| r.cost_usd),
        turns: s
            .result
            .as_ref()
            .and_then(|r| r.num_turns)
            .unwrap_or(s.messages.len() as u32),
        tool_calls,
        repeated_dead_ends: repeated,
        truncated: s.truncated(),
    }
}
