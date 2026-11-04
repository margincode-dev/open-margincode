//! Cold start from git history: an imported repo's commits become records, so day one of a repo
//! is not empty (Plan 13 Phase 2). Opt-in: `av repo add --mine-history N`, `av init
//! --mine-history N`, or later `av repo mine NAME` (`POST /v1/repos/{name}/history`).
//!
//! Each commit worth mining (`scholion_core::history::worth_mining`: one parent, not housekeeping,
//! a message that says something, at most `MAX_FILES` files) becomes one `note` written by
//! `MINED_AUTHOR` (`git`), with the commit as its `change_id`, the subject as its summary, the
//! rest of the message as its body and the files the commit touched as its anchors. They belong
//! to one task per repo, `git-history` (owned by `git`, status merged, no workspace), so no table
//! changes. A commit already mined is skipped, so mining again adds only what is new.

use std::collections::HashSet;

use axum::Json;
use axum::extract::{Path, State};
use scholion_core::history::{self, MINED_AUTHOR};
use scholion_core::{
    Anchor, AvError, MAX_SUMMARY_CHARS, NewRecord, NewTask, RecordKind, Result, Task, TaskStatus,
};
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::error::ApiResult;

/// The title (and so the slug, `git-history`) of the task that holds a repo's mined records.
pub const HISTORY_TITLE: &str = "Git history";
pub const HISTORY_SLUG: &str = "git-history";
/// Most records one mining run writes (and the default of `av repo mine`).
pub const MAX_MINE: usize = 5000;
pub const DEFAULT_MINE: usize = 500;
/// A commit touching more files than this is a sweep (a rename, a reformat), not one decision.
pub const MAX_FILES: usize = 25;
/// Commits looked at per record wanted: merges and housekeeping are skipped, not counted.
const SCAN_FACTOR: usize = 4;
/// Message text kept per record (the subject is the summary, the rest the body).
const MAX_MESSAGE: usize = 2000;

/// One mining run at a time in this daemon: two would both see a commit as new.
static MINING: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Mined {
    /// Records written by this run.
    pub records: usize,
    /// Commits looked at.
    pub scanned: usize,
    /// Commits skipped because an earlier run mined them.
    pub already: usize,
    /// The task holding the repo's mined records (`repo/git-history`).
    pub task: Option<String>,
}

/// The repo's history task, if mining ever ran.
pub(crate) async fn history_task(st: &AppState, repo: &str) -> Result<Option<Task>> {
    match st.store.task_by_slug(repo, HISTORY_SLUG).await {
        Ok(t) if t.owner_agent == MINED_AUTHOR => return Ok(Some(t)),
        Ok(_) | Err(AvError::NotFound(_)) => {}
        Err(e) => return Err(e),
    }
    // the slug was taken by a task of an agent's: ours got a suffix
    let rows = st
        .pg
        .list_tasks(Some(repo), Some(TaskStatus::Merged), 500)
        .await?;
    Ok(rows
        .into_iter()
        .map(|r| r.task)
        .find(|t| t.owner_agent == MINED_AUTHOR && t.title == HISTORY_TITLE))
}

async fn ensure_history_task(st: &AppState, repo: &str) -> Result<Task> {
    if let Some(t) = history_task(st, repo).await? {
        return Ok(t);
    }
    let t = st
        .store
        .create_task(NewTask {
            repo: repo.to_owned(),
            title: HISTORY_TITLE.to_owned(),
            goal: "Records mined from the repository's git history: one note per commit worth \
                   keeping, anchored to the files it touched."
                .to_owned(),
            owner_agent: MINED_AUTHOR.to_owned(),
        })
        .await?;
    // never open: nothing may claim, route or resume it
    st.store.set_status(t.id, TaskStatus::Merged).await?;
    st.store.get_task(t.id).await
}

/// Mines at most `limit` new records from the history of `repo`'s trunk, newest commits first.
pub(crate) async fn mine(st: &AppState, repo: &str, limit: usize) -> Result<Mined> {
    let limit = limit.min(MAX_MINE);
    if limit == 0 {
        return Ok(Mined::default());
    }
    let _one = MINING.lock().await;
    let task = ensure_history_task(st, repo).await?;
    let done: HashSet<String> = st
        .store
        .task_records(task.id)
        .await?
        .into_iter()
        .filter_map(|r| r.change_id.map(|c| c.0))
        .collect();
    let commits = st
        .vcs
        .history(repo, limit.saturating_mul(SCAN_FACTOR))
        .await?;
    let mut out = Mined {
        task: Some(format!("{repo}/{}", task.slug)),
        ..Mined::default()
    };
    for c in commits {
        if out.records >= limit {
            break;
        }
        out.scanned += 1;
        if done.contains(&c.change.0) {
            out.already += 1;
            continue;
        }
        let (subject, body) = history::split_message(&c.description);
        let Some(text) = history::worth_mining(c.parents.len(), subject, body, MAX_MESSAGE) else {
            continue;
        };
        let mut paths = st.vcs.changed_paths(repo, &c.parents[0], &c.change).await?;
        if paths.is_empty() || paths.len() > MAX_FILES {
            continue;
        }
        paths.sort();
        let short: String = c.change.0.chars().take(12).collect();
        let rest = text
            .split_once('\n')
            .map(|(_, r)| r.trim())
            .unwrap_or_default();
        let body = if rest.is_empty() {
            format!("Mined from git commit {short}.")
        } else {
            format!("{rest}\n\nMined from git commit {short}.")
        };
        st.store
            .add_record(NewRecord {
                task_id: task.id,
                kind: RecordKind::Note,
                summary: history::clip_chars(subject.trim(), MAX_SUMMARY_CHARS),
                body,
                change_id: Some(c.change.clone()),
                supersedes: None,
                author_agent: MINED_AUTHOR.to_owned(),
                anchors: paths
                    .into_iter()
                    .map(|path| Anchor { path, symbol: None })
                    .collect(),
            })
            .await?;
        out.records += 1;
    }
    Ok(out)
}

#[derive(Deserialize)]
pub(crate) struct MineBody {
    limit: Option<usize>,
}

/// `POST /v1/repos/{name}/history`: mine (more of) an imported repo's history.
pub(crate) async fn post_history(
    State(st): State<AppState>,
    Path(repo): Path<String>,
    Json(b): Json<MineBody>,
) -> ApiResult<Mined> {
    st.vcs.trunk(&repo).await?; // an unknown repo is 404 before any task is made
    Ok(Json(
        mine(&st, &repo, b.limit.unwrap_or(DEFAULT_MINE)).await?,
    ))
}
