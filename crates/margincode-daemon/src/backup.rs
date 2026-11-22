//! Git backup push. A merge never waits on or fails because of the backup: it
//! starts a push in the background, and failures are stored and retried with backoff.
//!
//! Trunk can also advance from the remote. `POST /v1/repos/{name}/trunk/advance`
//! fetches the remote's `main` (or a named branch) and fast-forwards trunk to it when it descends
//! from trunk, so pull requests merged on the remote come back; a backup with `advance` set does
//! that before each push.

use std::time::Duration;

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use margincode_core::{Advance, AvError, BackupState, Result, without_credentials};
use serde::Deserialize;
use serde_json::json;

use crate::AppState;
use crate::error::{ApiError, ApiResult};

const MAX_BACKOFF: Duration = Duration::from_secs(300);
/// How often the retry loop looks for due repos.
pub const BACKUP_TICK: Duration = Duration::from_secs(5);

/// Wait before the next attempt after `failures` consecutive failures: 1 s, 2 s, 4 s … 300 s.
pub fn backoff(failures: u32) -> Duration {
    if failures == 0 {
        return Duration::ZERO;
    }
    let secs = 1u64.checked_shl(failures - 1).unwrap_or(u64::MAX);
    Duration::from_secs(secs).min(MAX_BACKOFF)
}

/// One push at a time across the daemon: pushes are rare, and two at once for one repo would
/// race on the stored state.
static PUSH_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

pub(crate) async fn run_backup(st: &AppState, repo: &str) -> Result<BackupState> {
    let _guard = PUSH_LOCK.lock().await;
    let state = st.pg.backup_state(repo).await?;
    let Some(remote) = state.remote.clone() else {
        return Ok(state);
    };
    if state.advance {
        // a failed or refused advance is logged; the push goes on as it would without it
        match advance_trunk(st, repo, &remote, ADVANCE_BRANCH).await {
            Ok(Advance::Diverged {
                trunk,
                remote: theirs,
            }) => tracing::warn!(
                %repo, trunk = %trunk.0, remote = %theirs.0,
                "not advancing trunk: it and the remote's {ADVANCE_BRANCH} have diverged"
            ),
            Ok(_) => {}
            Err(e) => tracing::warn!(
                %repo, error = %without_credentials(&e.to_string()),
                "advancing trunk before the backup failed"
            ),
        }
    }
    match st
        .vcs
        .push_backup(repo, &remote, state.branch.as_deref())
        .await
    {
        Ok(pushed) => st.pg.backup_succeeded(repo, &pushed).await,
        Err(e) => {
            let error = without_credentials(&e.to_string());
            tracing::warn!(%repo, %error, "backup push failed");
            st.pg
                .backup_failed(repo, &error, backoff(state.failures + 1))
                .await
        }
    }
}

pub(crate) fn spawn_backup(st: &AppState, repo: &str) {
    let (st, repo) = (st.clone(), repo.to_owned());
    tokio::spawn(async move {
        if let Err(e) = run_backup(&st, &repo).await {
            tracing::warn!(%repo, error = %e, "backup state could not be stored");
        }
    });
}

pub async fn backup_tick(st: &AppState) -> Result<usize> {
    let due = st.pg.due_backups().await?;
    for repo in &due {
        run_backup(st, repo).await?;
    }
    Ok(due.len())
}

/// At startup: applies `AV_BACKUP_REMOTES` (a changed remote starts clean) and makes every
/// configured backup due, so a push lost to a restart happens now.
pub async fn start_backups(st: &AppState, remotes: &[(String, String)]) -> Result<()> {
    for (repo, url) in remotes {
        if st.pg.backup_state(repo).await?.remote.as_deref() != Some(url.as_str()) {
            st.pg.set_backup_remote(repo, Some(url), None).await?;
        }
    }
    st.pg.mark_backups_due().await
}

/// `state` as clients see it: no credentials in the remote or the last error, also when an older
/// daemon stored them.
pub(crate) fn shown(mut state: BackupState) -> BackupState {
    state.remote = state.remote.as_deref().map(without_credentials);
    state.last_error = state.last_error.as_deref().map(without_credentials);
    state
}

pub(crate) async fn get_backup(
    State(st): State<AppState>,
    Path(repo): Path<String>,
) -> ApiResult<BackupState> {
    Ok(Json(shown(st.pg.backup_state(&repo).await?)))
}

#[derive(Deserialize)]
pub(crate) struct BackupBody {
    /// `null` removes the backup.
    remote: Option<String>,
    /// The remote branch main is pushed to; leave it out for the trunk's own name.
    #[serde(default)]
    branch: Option<String>,
    /// Advance trunk from the remote's `main` before each push; left out = off.
    #[serde(default)]
    advance: bool,
}

/// Sets or clears the remote and branch (setting only a remote means the trunk's name again); a
/// new target is pushed right away.
pub(crate) async fn set_backup(
    State(st): State<AppState>,
    Path(repo): Path<String>,
    Json(b): Json<BackupBody>,
) -> ApiResult<BackupState> {
    let remote = b.remote.as_deref().map(str::trim).filter(|r| !r.is_empty());
    if let Some(r) = remote
        && (r.starts_with('-') || r.starts_with("ext::"))
    {
        return Err(AvError::Invalid(without_credentials(&format!(
            "{r:?} is not a remote git may push to"
        )))
        .into());
    }
    if let Some(b) = &b.branch {
        margincode_core::check_backup_branch(b)?;
    }
    let state = st
        .pg
        .set_backup_target(&repo, remote, b.branch.as_deref(), b.advance)
        .await?;
    if state.remote.is_some() {
        spawn_backup(&st, &repo);
    }
    Ok(Json(shown(state)))
}

pub(crate) async fn push_backup(
    State(st): State<AppState>,
    Path(repo): Path<String>,
) -> ApiResult<BackupState> {
    Ok(Json(shown(run_backup(&st, &repo).await?)))
}

/// The remote branch trunk advances from when none is named.
pub const ADVANCE_BRANCH: &str = "main";

/// Advances `repo`'s trunk from `branch` of `remote` (`Vcs::advance_trunk`) under the merge
/// lock. After a fast-forward it does for main what a merge does: a `trunk:advanced` event, and
/// main's index and the lean-build config brought to the new trunk.
pub(crate) async fn advance_trunk(
    st: &AppState,
    repo: &str,
    remote: &str,
    branch: &str,
) -> Result<Advance> {
    let _guard = crate::flow::MERGE_LOCK.lock().await;
    let outcome = st.vcs.advance_trunk(repo, remote, branch).await?;
    if let Advance::FastForwarded { from, to } = &outcome {
        let payload = json!({
            "from": from, "to": to, "remote": without_credentials(remote), "branch": branch,
        });
        st.store
            .add_event(repo, "trunk:advanced", None, payload)
            .await?;
        let changed = st.vcs.changed_paths(repo, from, to).await?;
        for w in crate::indexing::reindex_changed(st, repo, "main", to, &changed).await? {
            tracing::warn!(%repo, "advance: {w}");
        }
        crate::build_cache::ensure_logged(st, repo).await;
    }
    Ok(outcome)
}

#[derive(Deserialize, Default)]
pub(crate) struct AdvanceBody {
    /// Default: the backup remote.
    #[serde(default)]
    remote: Option<String>,
    /// Default: `main`.
    #[serde(default)]
    branch: Option<String>,
}

/// `POST /v1/repos/{name}/trunk/advance`: `200` with the `Advance` (fast-forwarded, up to date,
/// or ahead: the remote is behind trunk), `409` with both heads when they diverged, `400`
/// without a remote. A fast-forward is pushed to the backup as a merge is.
pub(crate) async fn post_advance(
    State(st): State<AppState>,
    Path(repo): Path<String>,
    body: Option<Json<AdvanceBody>>,
) -> std::result::Result<Response, ApiError> {
    let b = body.map(|Json(b)| b).unwrap_or_default();
    let remote = match b.remote.as_deref().map(str::trim).filter(|r| !r.is_empty()) {
        Some(r) => r.to_owned(),
        None => st.pg.backup_state(&repo).await?.remote.ok_or_else(|| {
            AvError::Invalid(format!(
                "repo {repo} has no backup remote: name one (`av repo advance {repo} --remote URL`)"
            ))
        })?,
    };
    let branch = b.branch.as_deref().unwrap_or(ADVANCE_BRANCH);
    let outcome = advance_trunk(&st, &repo, &remote, branch).await?;
    if let Advance::Diverged {
        trunk,
        remote: theirs,
    } = &outcome
    {
        let error = format!(
            "trunk ({}) and the remote's {branch} ({}) have diverged: neither contains the other; trunk is unchanged",
            short(&trunk.0),
            short(&theirs.0)
        );
        let body = json!({"outcome": "diverged", "trunk": trunk, "remote": theirs, "error": error});
        return Ok((StatusCode::CONFLICT, Json(body)).into_response());
    }
    if matches!(outcome, Advance::FastForwarded { .. }) {
        spawn_backup(&st, &repo);
    }
    Ok(Json(outcome).into_response())
}

fn short(hex: &str) -> &str {
    &hex[..hex.len().min(12)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backoff_doubles_and_caps_at_300s() {
        let secs: Vec<u64> = (0..12).map(|n| backoff(n).as_secs()).collect();
        assert_eq!(secs, [0, 1, 2, 4, 8, 16, 32, 64, 128, 256, 300, 300]);
        assert_eq!(backoff(200), MAX_BACKOFF, "no overflow");
    }
}
