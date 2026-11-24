//! Learn from use: at each merge, how good the task's packs were (`margincode_core::outcome`,
//! stored in `pack_outcomes`), the files its agent opened through the daemon (`task_reads`), and
//! the routes that read the measures back. All of it is best effort: a failure is logged and
//! the merge, read or expand it rides on answers as if nothing was asked.

use axum::extract::{Path, Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use margincode_core::outcome::{self, PathChange};
use margincode_core::*;
use margincode_store::{NewOutcome, OutcomeRow};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::AppState;
use crate::error::ApiResult;

pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .route("/v1/repos/{name}/context/outcomes", get(outcomes))
        .route("/v1/repos/{name}/context/backfill", post(backfill))
}

/// Measures a task that just merged. Called once the merge is recorded, outside its locks.
pub(crate) async fn after_merge(st: &AppState, task: TaskId) {
    if let Err(e) = measure(st, task).await {
        tracing::warn!(task = %task, error = %e, "pack outcome not stored");
    }
}

async fn measure(st: &AppState, id: TaskId) -> Result<()> {
    let task = st.store.get_task(id).await?;
    let merge = st.store.merge_state(id).await?;
    let (changed, source) = changed_paths(st, &task, &merge).await?;
    let packs = st.pg.task_packs(id).await?;
    let opened = st.pg.read_paths(id).await?;
    let measures = outcome::measure(&packs.packs, &changed, &opened);
    st.pg
        .put_outcome(NewOutcome {
            task: &task,
            merge_commit: merge.merge_commit.as_ref().map(|c| c.0.as_str()),
            changed_source: source,
            packs: &packs,
            measures: &measures,
            files_opened: opened.len() as i32,
        })
        .await
}

/// What the merge changed, from the VCS diff of `pre_merge` to the merge commit; where the VCS can
/// no longer give it, the task's last touched paths.
async fn changed_paths(
    st: &AppState,
    task: &Task,
    merge: &MergeState,
) -> Result<(Vec<(String, PathChange)>, &'static str)> {
    if let (Some(pre), Some(main)) = (&merge.pre_merge, &merge.merge_commit)
        && let Ok(paths) = st.vcs.changed_paths(&task.repo, pre, main).await
    {
        return Ok((classify(st, &task.repo, pre, main, paths).await, "vcs"));
    }
    let touched = st.store.touched_paths(task.id).await?;
    let changed = classify(
        st,
        &task.repo,
        &task.base_change,
        &task.head_change,
        touched,
    )
    .await;
    Ok((changed, "touched"))
}

/// Added or deleted by whether the path exists at each end; a path either end cannot be read at
/// counts as modified.
async fn classify(
    st: &AppState,
    repo: &str,
    from: &ChangeId,
    to: &ChangeId,
    paths: Vec<String>,
) -> Vec<(String, PathChange)> {
    let gone = |r: Option<&Result<Vec<u8>>>| matches!(r, Some(Err(AvError::NotFound(_))));
    let (old, new) = futures::join!(
        st.vcs.read_files(repo, from, &paths),
        st.vcs.read_files(repo, to, &paths)
    );
    let (old, new) = (old.unwrap_or_default(), new.unwrap_or_default());
    paths
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let change = if gone(old.get(i).map(|(_, r)| r)) {
                PathChange::Added
            } else if gone(new.get(i).map(|(_, r)| r)) {
                PathChange::Deleted
            } else {
                PathChange::Modified
            };
            (p.clone(), change)
        })
        .collect()
}

/// Notes that the task's agent opened `path`, on a task of its own so the call it rides on
/// neither waits for it nor fails with it.
pub(crate) fn note_read(st: &AppState, task: TaskId, path: &str, expanded: bool) {
    let path = path.trim().replace('\\', "/");
    let path = path.trim_start_matches("./").to_owned();
    if path.is_empty() {
        return;
    }
    let pg = st.pg.clone();
    tokio::spawn(async move {
        if let Err(e) = pg.note_read(task, &path, expanded).await {
            tracing::warn!(task = %task, %path, error = %e, "task read not stored");
        }
    });
}

/// An `expand` of `sym:N` opened that symbol's file. A record has no path, so nothing is noted.
pub(crate) fn note_expand(st: &AppState, task: TaskId, item_id: &str) {
    let Some(id) = item_id
        .strip_prefix("sym:")
        .and_then(|n| n.parse::<i64>().ok())
    else {
        return;
    };
    let st = st.clone();
    tokio::spawn(async move {
        match st.index.get_symbols(&[SymbolId(id)]).await {
            Ok(found) => {
                if let Some(s) = found.into_iter().next() {
                    note_read(&st, task, &s.symbol.path, true);
                }
            }
            Err(e) => tracing::warn!(task = %task, error = %e, "task read not stored"),
        }
    });
}

#[derive(Deserialize)]
struct OutcomesQuery {
    /// `tasks=1`: every measured task's row too.
    tasks: Option<u8>,
}

/// The measures of a repo's merged tasks, medians per packer version. A task whose merge was
/// reverted or undone keeps its row (`reverted: true`) and is left out of the totals.
async fn outcomes(
    State(st): State<AppState>,
    Path(repo): Path<String>,
    Query(q): Query<OutcomesQuery>,
) -> ApiResult<Value> {
    st.vcs.trunk(&repo).await?;
    let rows = st.pg.outcomes(&repo).await?;
    let mut answer = summary(&repo, &rows);
    if q.tasks == Some(1) {
        answer["rows"] = json!(rows);
    }
    Ok(Json(answer))
}

fn summary(repo: &str, rows: &[OutcomeRow]) -> Value {
    let live: Vec<&OutcomeRow> = rows.iter().filter(|r| !r.reverted).collect();
    let mut versions: Vec<Option<i32>> = live.iter().map(|r| r.packer_version).collect();
    versions.sort();
    versions.dedup();
    let by_packer: Vec<Value> = versions
        .iter()
        .map(|v| {
            let of: Vec<&OutcomeRow> = live
                .iter()
                .copied()
                .filter(|r| r.packer_version == *v)
                .collect();
            let with_code: Vec<&OutcomeRow> =
                of.iter().copied().filter(|r| r.code_tokens > 0).collect();
            let share = |part: fn(&OutcomeRow) -> i64| {
                median(
                    with_code
                        .iter()
                        .map(|r| part(r) as f64 / r.code_tokens as f64)
                        .collect(),
                )
            };
            let count =
                |n: fn(&OutcomeRow) -> i64| median(of.iter().map(|r| n(r) as f64).collect());
            json!({
                "packer_version": v,
                "tasks": of.len(),
                "no_code": of.len() - with_code.len(),
                "no_reads": of.iter().filter(|r| r.files_opened == 0).count(),
                "changed_share": share(|r| r.code_tokens_changed),
                "used_share": share(|r| r.code_tokens_used),
                "code_tokens": count(|r| r.code_tokens),
                "files_changed": count(|r| r.files_changed.into()),
                "files_missed": count(|r| r.files_missed.into()),
                "files_named_only": count(|r| r.files_named_only.into()),
                "opened_not_sent": count(|r| r.opened_not_sent.into()),
            })
        })
        .collect();
    json!({
        "repo": repo,
        "measured": live.len(),
        "reverted": rows.len() - live.len(),
        "no_reads": live.iter().filter(|r| r.files_opened == 0).count(),
        "by_packer": by_packer,
    })
}

fn median(mut xs: Vec<f64>) -> Option<f64> {
    if xs.is_empty() {
        return None;
    }
    xs.sort_by(f64::total_cmp);
    let mid = xs.len() / 2;
    Some(if xs.len() % 2 == 1 {
        xs[mid]
    } else {
        (xs[mid - 1] + xs[mid]) / 2.0
    })
}

#[derive(Deserialize)]
struct BackfillQuery {
    limit: Option<i64>,
}

const BACKFILL_LIMIT: i64 = 200;
const MAX_BACKFILL: i64 = 1000;

/// Measures merged tasks of the repo that have no outcome yet (merged before outcomes were
/// kept), oldest first, at most `limit` (default 200) per call.
async fn backfill(
    State(st): State<AppState>,
    Path(repo): Path<String>,
    Query(q): Query<BackfillQuery>,
) -> ApiResult<Value> {
    st.vcs.trunk(&repo).await?;
    let limit = q.limit.unwrap_or(BACKFILL_LIMIT).clamp(1, MAX_BACKFILL);
    let todo = st.pg.merged_without_outcome(&repo, limit).await?;
    let (mut computed, mut failed) = (0, vec![]);
    for id in &todo {
        match measure(&st, *id).await {
            Ok(()) => computed += 1,
            Err(e) => {
                tracing::warn!(task = %id, error = %e, "pack outcome not stored");
                failed.push(json!({"task": id, "error": e.to_string()}));
            }
        }
    }
    Ok(Json(json!({
        "computed": computed,
        "failed": failed,
        "more": todo.len() as i64 == limit,
    })))
}

#[cfg(test)]
mod tests {
    use super::median;

    #[test]
    fn median_of_an_even_count_is_the_mean_of_the_middle_two() {
        assert_eq!(median(vec![]), None);
        assert_eq!(median(vec![3.0, 1.0, 2.0]), Some(2.0));
        assert_eq!(median(vec![4.0, 1.0, 3.0, 2.0]), Some(2.5));
    }
}
