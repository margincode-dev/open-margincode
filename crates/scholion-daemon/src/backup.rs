//! Git backup push (Plan 3 Task 6). A merge never waits on or fails because of the backup: it
//! starts a push in the background, and failures are stored and retried with backoff.

use std::time::Duration;

use axum::Json;
use axum::extract::{Path, State};
use scholion_core::{AvError, BackupState, Result};
use serde::Deserialize;

use crate::AppState;
use crate::error::ApiResult;

/// The longest wait between two attempts.
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

/// Pushes `repo`'s trunk to its remote now, if one is set, and stores the outcome.
pub(crate) async fn run_backup(st: &AppState, repo: &str) -> Result<BackupState> {
    let _guard = PUSH_LOCK.lock().await;
    let state = st.pg.backup_state(repo).await?;
    let Some(remote) = state.remote.clone() else {
        return Ok(state);
    };
    match st.vcs.push_backup(repo, &remote).await {
        Ok(pushed) => st.pg.backup_succeeded(repo, &pushed).await,
        Err(e) => {
            tracing::warn!(%repo, error = %e, "backup push failed");
            st.pg
                .backup_failed(repo, &e.to_string(), backoff(state.failures + 1))
                .await
        }
    }
}

/// `run_backup` in the background (after a merge or undo): the caller does not wait.
pub(crate) fn spawn_backup(st: &AppState, repo: &str) {
    let (st, repo) = (st.clone(), repo.to_owned());
    tokio::spawn(async move {
        if let Err(e) = run_backup(&st, &repo).await {
            tracing::warn!(%repo, error = %e, "backup state could not be stored");
        }
    });
}

/// One pass of the retry loop: pushes every repo whose next attempt is due. Returns how many.
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
            st.pg.set_backup_remote(repo, Some(url)).await?;
        }
    }
    st.pg.mark_backups_due().await
}

pub(crate) async fn get_backup(
    State(st): State<AppState>,
    Path(repo): Path<String>,
) -> ApiResult<BackupState> {
    Ok(Json(st.pg.backup_state(&repo).await?))
}

#[derive(Deserialize)]
pub(crate) struct BackupBody {
    /// `null` removes the backup.
    remote: Option<String>,
}

/// Sets or clears the remote; a new one is pushed right away.
pub(crate) async fn set_backup(
    State(st): State<AppState>,
    Path(repo): Path<String>,
    Json(b): Json<BackupBody>,
) -> ApiResult<BackupState> {
    let remote = b.remote.as_deref().map(str::trim).filter(|r| !r.is_empty());
    if let Some(r) = remote
        && (r.starts_with('-') || r.starts_with("ext::"))
    {
        return Err(AvError::Invalid(format!("{r:?} is not a remote git may push to")).into());
    }
    let state = st.pg.set_backup_remote(&repo, remote).await?;
    if state.remote.is_some() {
        spawn_backup(&st, &repo);
    }
    Ok(Json(state))
}

/// Pushes now and returns the outcome (also what the retry loop would do once due).
pub(crate) async fn push_backup(
    State(st): State<AppState>,
    Path(repo): Path<String>,
) -> ApiResult<BackupState> {
    Ok(Json(run_backup(&st, &repo).await?))
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
