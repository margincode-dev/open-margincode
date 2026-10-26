//! Post-merge checks (Plan 5, Scion's "no merge queue: test after merge"). `av runner` runs the
//! repo's check command on every new main and posts the result here; the daemon itself never runs
//! commands. Main's health is the newest result: green, or red since the oldest commit of the
//! current red run, whose task (from the merge marker) is the likely culprit.

use std::collections::HashMap;

use axum::Json;
use axum::extract::{Path, State};
use scholion_core::{AvError, ChangeId, CheckRow, LogEntry, Result, TaskId};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::AppState;
use crate::error::ApiResult;

/// How far back on main health looks for checked commits.
const HEALTH_DEPTH: usize = 200;

#[derive(Deserialize)]
pub(crate) struct CheckBody {
    change: String,
    command: String,
    exit_code: i32,
    #[serde(default)]
    output_tail: String,
    #[serde(default)]
    runner: String,
}

/// Stores one check result (a re-run of the same command on the same commit replaces it).
pub(crate) async fn post_check(
    State(st): State<AppState>,
    Path(repo): Path<String>,
    Json(b): Json<CheckBody>,
) -> ApiResult<Value> {
    if b.change.is_empty() || !b.change.bytes().all(|c| c.is_ascii_hexdigit()) {
        return Err(AvError::Invalid(format!("bad change id {:?}", b.change)).into());
    }
    let row = CheckRow {
        change: b.change.to_ascii_lowercase(),
        command: b.command,
        exit_code: b.exit_code,
        output_tail: b
            .output_tail
            .chars()
            .rev()
            .take(4000)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect(),
        runner: b.runner,
        at: chrono::Utc::now(),
    };
    st.store.add_check(&repo, &row).await?;
    Ok(Json(json!({"stored": true, "change": row.change})))
}

/// Per change: its newest result per command (`None` when unchecked).
pub(crate) async fn results_by_change(
    st: &AppState,
    repo: &str,
    commits: &[LogEntry],
) -> Result<HashMap<String, Vec<CheckRow>>> {
    let changes: Vec<String> = commits.iter().map(|c| c.change.0.clone()).collect();
    let mut by: HashMap<String, HashMap<String, CheckRow>> = HashMap::new();
    for row in st.store.checks_for(repo, &changes).await? {
        by.entry(row.change.clone())
            .or_default()
            .insert(row.command.clone(), row);
    }
    Ok(by
        .into_iter()
        .map(|(change, cmds)| (change, cmds.into_values().collect()))
        .collect())
}

/// The failing result among `rows`, else any result (for display).
pub(crate) fn summary(rows: &[CheckRow]) -> Option<&CheckRow> {
    rows.iter().find(|r| r.exit_code != 0).or(rows.first())
}

/// The task a merge commit's description names (`av task <uuid>` on its last lines).
pub(crate) async fn task_of(st: &AppState, description: &str) -> Option<scholion_core::Task> {
    let id = description
        .lines()
        .rev()
        .find_map(|l| l.trim().strip_prefix("av task "))?
        .split_whitespace()
        .next()?;
    let id = uuid::Uuid::parse_str(id).ok()?;
    st.store.get_task(TaskId(id)).await.ok()
}

/// `{main, state: green|red|unknown, checked, since, task, command, exit_code}`.
pub(crate) async fn main_health(st: &AppState, repo: &str) -> Result<Value> {
    let main: ChangeId = st.vcs.trunk(repo).await?;
    let commits = st.vcs.trunk_log(repo, HEALTH_DEPTH).await?;
    let results = results_by_change(st, repo, &commits).await?;
    let checked: Vec<(&LogEntry, &CheckRow)> = commits
        .iter()
        .filter_map(|c| Some((c, summary(results.get(&c.change.0)?)?)))
        .collect();
    let Some((newest, row)) = checked.first() else {
        return Ok(json!({"main": main, "state": "unknown"}));
    };
    if row.exit_code == 0 {
        return Ok(json!({"main": main, "state": "green", "checked": newest.change}));
    }
    // the oldest commit of the red run that ends at the newest checked commit
    let (since, failing) = checked
        .iter()
        .take_while(|(_, r)| r.exit_code != 0)
        .last()
        .copied()
        .unwrap_or((newest, row));
    let task = task_of(st, &since.description).await;
    Ok(json!({
        "main": main,
        "state": "red",
        "checked": newest.change,
        "since": since.change,
        "task": task.as_ref().map(|t| &t.slug),
        "task_id": task.as_ref().map(|t| t.id),
        "command": failing.command,
        "exit_code": failing.exit_code,
    }))
}

pub(crate) async fn health(
    State(st): State<AppState>,
    Path(repo): Path<String>,
) -> ApiResult<Value> {
    Ok(Json(main_health(&st, &repo).await?))
}
