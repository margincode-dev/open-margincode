//! Derived data (Plan 5, after Scion): values defined from a commit and its parent's value,
//! cached by the immutable commit id. The first kind is `blame_v1`: for one path, which commit of
//! main last changed each line. Through the merge marker that commit names a task, and the task
//! carries its intent, decisions and dead ends: `why` a line is the way it is.

use axum::Json;
use axum::extract::{Path, Query, State};
use scholion_core::{AvError, ChangeId, LogEntry, RecordKind, Result};
use serde::Deserialize;
use serde_json::{Value, json};
use similar::{DiffOp, TextDiff};

use crate::AppState;
use crate::error::ApiResult;

const BLAME: &str = "blame_v1";
/// Commits of main a blame walks back through, and commits that touched the path it keeps.
const BLAME_SCAN: usize = 2000;
const BLAME_TOUCHING: usize = 500;

/// Whether a commit is a task merge (or revert) written by av; anything else is imported
/// history, where a blame stops and owns every remaining line.
fn is_av_commit(c: &LogEntry) -> bool {
    c.description
        .lines()
        .any(|l| l.starts_with("av task ") || l.starts_with("av revert "))
}

/// Run-length attribution: `[[change, count], ...]` covering the file's lines in order.
type Runs = Vec<(String, usize)>;

fn expand(runs: &Runs) -> Vec<String> {
    runs.iter()
        .flat_map(|(c, n)| std::iter::repeat_n(c.clone(), *n))
        .collect()
}

fn compress(lines: &[String]) -> Runs {
    let mut out: Runs = vec![];
    for c in lines {
        match out.last_mut() {
            Some((last, n)) if last == c => *n += 1,
            _ => out.push((c.clone(), 1)),
        }
    }
    out
}

async fn text_at(
    st: &AppState,
    repo: &str,
    change: Option<&ChangeId>,
    path: &str,
) -> Result<Option<String>> {
    let Some(change) = change else {
        return Ok(None);
    };
    match st.vcs.read_file(repo, change, path).await {
        Ok(bytes) => Ok(Some(String::from_utf8_lossy(&bytes).into_owned())),
        Err(AvError::NotFound(_)) => Ok(None),
        Err(e) => Err(e),
    }
}

/// The newest commit of main that touched `path`, and per line of the file there the commit that
/// last changed it. Walks back to the newest cached blame (or to imported history), then forward,
/// caching each step.
pub(crate) async fn blame(
    st: &AppState,
    repo: &str,
    path: &str,
) -> Result<(LogEntry, Vec<String>)> {
    let touching = st
        .vcs
        .path_log(repo, path, BLAME_TOUCHING, BLAME_SCAN)
        .await?;
    let Some(newest) = touching.first().cloned() else {
        return Err(AvError::NotFound(format!("{path} has no history on main")));
    };
    // back: find where to start computing
    let mut start = touching.len() - 1;
    let mut runs: Option<Runs> = None;
    for (i, c) in touching.iter().enumerate() {
        if let Some(v) = st.store.derived_get(repo, BLAME, &c.change, path).await? {
            runs = serde_json::from_value(v).ok();
            if runs.is_some() {
                start = i;
                break;
            }
        }
        if !is_av_commit(c) {
            start = i;
            break;
        }
    }
    // forward: each commit's blame from its parent's
    let mut lines: Vec<String> = runs.as_ref().map(expand).unwrap_or_default();
    let have_start = runs.is_some();
    for (i, c) in touching[..=start].iter().enumerate().rev() {
        if i == start && have_start {
            continue;
        }
        let now = text_at(st, repo, Some(&c.change), path)
            .await?
            .unwrap_or_default();
        lines = if i == start {
            // the oldest commit considered owns all its lines
            vec![c.change.0.clone(); now.lines().count()]
        } else {
            let before = text_at(st, repo, c.parents.first(), path)
                .await?
                .unwrap_or_default();
            let diff = TextDiff::from_lines(&before, &now);
            let mut next = Vec::with_capacity(now.lines().count());
            for op in diff.ops() {
                match *op {
                    DiffOp::Equal { old_index, len, .. } => {
                        next.extend(lines.iter().skip(old_index).take(len).cloned());
                    }
                    DiffOp::Insert { new_len, .. } | DiffOp::Replace { new_len, .. } => {
                        next.extend(std::iter::repeat_n(c.change.0.clone(), new_len));
                    }
                    DiffOp::Delete { .. } => {}
                }
            }
            next
        };
        let value =
            serde_json::to_value(compress(&lines)).map_err(|e| AvError::Store(e.to_string()))?;
        st.store
            .derived_put(repo, BLAME, &c.change, path, &value)
            .await?;
    }
    Ok((newest, lines))
}

#[derive(Deserialize)]
pub(crate) struct WhyQuery {
    path: String,
    /// 1-based line number.
    line: usize,
}

/// Why a line of main is the way it is: the commit that last changed it, that commit's task with
/// its goal, and the task's decisions, dead ends, notes and review findings.
pub(crate) async fn why(
    State(st): State<AppState>,
    Path(repo): Path<String>,
    Query(q): Query<WhyQuery>,
) -> ApiResult<Value> {
    let (newest, lines) = blame(&st, &repo, &q.path).await?;
    let Some(change) = q.line.checked_sub(1).and_then(|i| lines.get(i)).cloned() else {
        return Err(AvError::NotFound(format!(
            "{} has {} lines on main; line {} does not exist",
            q.path,
            lines.len(),
            q.line
        ))
        .into());
    };
    let text = text_at(&st, &repo, Some(&newest.change), &q.path)
        .await?
        .unwrap_or_default();
    let line_text = text.lines().nth(q.line - 1).unwrap_or_default().to_owned();
    let commit = st
        .vcs
        .trunk_log(&repo, BLAME_SCAN)
        .await?
        .into_iter()
        .find(|c| c.change.0 == change);
    let description = commit
        .as_ref()
        .map(|c| c.description.clone())
        .unwrap_or_default();
    let task = crate::checks::task_of(&st, &description).await;
    let records: Vec<Value> = match &task {
        Some(t) => st
            .store
            .task_records(t.id)
            .await?
            .into_iter()
            .filter(|r| {
                matches!(
                    r.kind,
                    RecordKind::Decision
                        | RecordKind::Attempt
                        | RecordKind::Note
                        | RecordKind::Review
                )
            })
            .map(|r| {
                json!({"id": format!("rec:{}", &r.id.0.simple().to_string()[..8]),
                            "kind": r.kind.as_str(), "summary": r.summary, "by": r.author_agent})
            })
            .collect(),
        None => vec![],
    };
    Ok(Json(json!({
        "path": q.path,
        "line": q.line,
        "line_text": line_text,
        "commit": change,
        "commit_description": description.lines().next().unwrap_or_default(),
        "task": task.as_ref().map(|t| json!({"id": t.id, "slug": t.slug, "title": t.title, "goal": t.goal})),
        "records": records,
    })))
}
