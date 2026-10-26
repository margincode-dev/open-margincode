use std::time::Duration;

use scholion_core::*;
use sqlx::Row;

use crate::pg::{PgStore, serr};

const BACKUP_COLS: &str = "remote, last_pushed_change, failures, last_error, next_retry";

fn backup_from_row(row: &sqlx::postgres::PgRow) -> Result<BackupState> {
    Ok(BackupState {
        remote: row.try_get("remote").map_err(serr)?,
        last_pushed: row
            .try_get::<Option<String>, _>("last_pushed_change")
            .map_err(serr)?
            .map(ChangeId),
        failures: row.try_get::<i32, _>("failures").map_err(serr)? as u32,
        last_error: row.try_get("last_error").map_err(serr)?,
        next_retry: row.try_get("next_retry").map_err(serr)?,
    })
}

impl PgStore {
    /// The repo's backup state; the default (no remote) when none was ever configured.
    pub async fn backup_state(&self, repo: &str) -> Result<BackupState> {
        let row = sqlx::query(&format!(
            "SELECT {BACKUP_COLS} FROM backup_state WHERE repo = $1"
        ))
        .bind(repo)
        .fetch_optional(self.pool())
        .await
        .map_err(serr)?;
        row.as_ref()
            .map(backup_from_row)
            .transpose()
            .map(Option::unwrap_or_default)
    }

    /// Sets (or with `None` clears) the remote. A new remote starts clean and is due at once.
    pub async fn set_backup_remote(&self, repo: &str, remote: Option<&str>) -> Result<BackupState> {
        let row = sqlx::query(&format!(
            "INSERT INTO backup_state (repo, remote, next_retry) VALUES ($1, $2, now())
             ON CONFLICT (repo) DO UPDATE
             SET remote = EXCLUDED.remote, failures = 0, last_error = NULL,
                 next_retry = CASE WHEN EXCLUDED.remote IS NULL THEN NULL ELSE now() END,
                 updated_at = now()
             RETURNING {BACKUP_COLS}"
        ))
        .bind(repo)
        .bind(remote)
        .fetch_one(self.pool())
        .await
        .map_err(serr)?;
        backup_from_row(&row)
    }

    pub async fn backup_succeeded(&self, repo: &str, pushed: &ChangeId) -> Result<BackupState> {
        let row = sqlx::query(&format!(
            "UPDATE backup_state
             SET last_pushed_change = $2, failures = 0, last_error = NULL, next_retry = NULL,
                 updated_at = now()
             WHERE repo = $1 RETURNING {BACKUP_COLS}"
        ))
        .bind(repo)
        .bind(&pushed.0)
        .fetch_one(self.pool())
        .await
        .map_err(serr)?;
        backup_from_row(&row)
    }

    /// Counts a failed push and schedules the next attempt `retry_in` from now.
    pub async fn backup_failed(
        &self,
        repo: &str,
        error: &str,
        retry_in: Duration,
    ) -> Result<BackupState> {
        let row = sqlx::query(&format!(
            "UPDATE backup_state
             SET failures = failures + 1, last_error = $2,
                 next_retry = now() + make_interval(secs => $3), updated_at = now()
             WHERE repo = $1 RETURNING {BACKUP_COLS}"
        ))
        .bind(repo)
        .bind(error)
        .bind(retry_in.as_secs_f64())
        .fetch_one(self.pool())
        .await
        .map_err(serr)?;
        backup_from_row(&row)
    }

    /// Repos with a remote whose next attempt is due, by name.
    pub async fn due_backups(&self) -> Result<Vec<String>> {
        sqlx::query_scalar(
            "SELECT repo FROM backup_state
             WHERE remote IS NOT NULL AND next_retry <= now()
             ORDER BY repo COLLATE \"C\"",
        )
        .fetch_all(self.pool())
        .await
        .map_err(serr)
    }

    /// Makes every configured backup due, e.g. at startup, so a push lost to a restart is retried.
    pub async fn mark_backups_due(&self) -> Result<()> {
        sqlx::query(
            "UPDATE backup_state SET next_retry = now()
             WHERE remote IS NOT NULL AND next_retry IS NULL",
        )
        .execute(self.pool())
        .await
        .map_err(serr)?;
        Ok(())
    }
}
