//! Rows of the live measure of packs (`margincode_core::outcome`): one outcome per merged task,
//! and the files each task's agent opened through the daemon (`task_reads`).

use chrono::{DateTime, Utc};
use margincode_core::outcome::{Measures, SentPack};
use margincode_core::*;
use serde::Serialize;
use sqlx::Row;
use uuid::Uuid;

use crate::pg::{PgStore, serr};

/// A task's packs, oldest first, as `margincode_core::outcome::measure` takes them, with the
/// packer version and budget of the first.
pub struct TaskPacks {
    pub packs: Vec<SentPack>,
    pub first: Option<(i32, i32)>,
}

pub struct NewOutcome<'a> {
    pub task: &'a Task,
    pub merge_commit: Option<&'a str>,
    /// `vcs` or `touched`.
    pub changed_source: &'a str,
    pub packs: &'a TaskPacks,
    pub measures: &'a Measures,
    pub files_opened: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct OutcomeRow {
    pub task_id: Uuid,
    pub slug: String,
    pub title: String,
    pub merge_commit: Option<String>,
    pub changed_source: String,
    pub packer_version: Option<i32>,
    pub first_budget: Option<i32>,
    pub packs: i32,
    pub code_tokens: i64,
    pub code_tokens_changed: i64,
    pub code_tokens_used: i64,
    pub files_changed: i32,
    pub files_missed: i32,
    pub missed_paths: Vec<String>,
    pub files_named_only: i32,
    pub files_opened: i32,
    pub opened_not_sent: i32,
    pub opened_not_sent_paths: Vec<String>,
    pub computed_at: DateTime<Utc>,
    /// Its task is no longer merged with this merge commit (reverted or undone since).
    pub reverted: bool,
}

/// The fields of a receipt item the measure reads; an item without an id or tier is skipped, so
/// one odd item does not lose the task's outcome.
fn sent_item(v: &serde_json::Value) -> Option<ReceiptItem> {
    Some(ReceiptItem {
        id: v["id"].as_str()?.to_owned(),
        tier: v["tier"].as_str()?.parse().ok()?,
        score: 0.0,
        reason: v["reason"].as_str().unwrap_or_default().to_owned(),
        source: serde_json::from_value(v["source"].clone()).ok().flatten(),
        tokens: v["tokens"].as_u64().unwrap_or(0) as u32,
    })
}

impl PgStore {
    pub async fn task_packs(&self, task: TaskId) -> Result<TaskPacks> {
        let rows = sqlx::query(
            "SELECT session, packer_version, budget, items FROM context_receipts
             WHERE task_id = $1 ORDER BY created_at, id",
        )
        .bind(task.0)
        .fetch_all(self.pool())
        .await
        .map_err(serr)?;
        let mut out = TaskPacks {
            packs: Vec::with_capacity(rows.len()),
            first: None,
        };
        for r in rows {
            if out.first.is_none() {
                out.first = Some((
                    r.try_get("packer_version").map_err(serr)?,
                    r.try_get("budget").map_err(serr)?,
                ));
            }
            let items: Vec<serde_json::Value> = r
                .try_get::<sqlx::types::Json<Vec<serde_json::Value>>, _>("items")
                .map_err(serr)?
                .0;
            out.packs.push(SentPack {
                session: r.try_get("session").map_err(serr)?,
                items: items.iter().filter_map(sent_item).collect(),
            });
        }
        Ok(out)
    }

    /// Replaces the task's outcome: a task merged again after a revert is measured again.
    pub async fn put_outcome(&self, o: NewOutcome<'_>) -> Result<()> {
        let m = o.measures;
        sqlx::query(
            "INSERT INTO pack_outcomes (task_id, repo, merge_commit, changed_source, packer_version,
               first_budget, packs, code_tokens, code_tokens_changed, code_tokens_used,
               files_changed, files_missed, missed_paths, files_named_only, files_opened,
               opened_not_sent, opened_not_sent_paths)
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17)
             ON CONFLICT (task_id) DO UPDATE SET repo = EXCLUDED.repo,
               merge_commit = EXCLUDED.merge_commit, changed_source = EXCLUDED.changed_source,
               packer_version = EXCLUDED.packer_version, first_budget = EXCLUDED.first_budget,
               packs = EXCLUDED.packs, code_tokens = EXCLUDED.code_tokens,
               code_tokens_changed = EXCLUDED.code_tokens_changed,
               code_tokens_used = EXCLUDED.code_tokens_used, files_changed = EXCLUDED.files_changed,
               files_missed = EXCLUDED.files_missed, missed_paths = EXCLUDED.missed_paths,
               files_named_only = EXCLUDED.files_named_only, files_opened = EXCLUDED.files_opened,
               opened_not_sent = EXCLUDED.opened_not_sent,
               opened_not_sent_paths = EXCLUDED.opened_not_sent_paths, computed_at = now()",
        )
        .bind(o.task.id.0)
        .bind(&o.task.repo)
        .bind(o.merge_commit)
        .bind(o.changed_source)
        .bind(o.packs.first.map(|f| f.0))
        .bind(o.packs.first.map(|f| f.1))
        .bind(o.packs.packs.len() as i32)
        .bind(m.code_tokens as i64)
        .bind(m.code_tokens_changed as i64)
        .bind(m.code_tokens_used as i64)
        .bind(m.changed.len() as i32)
        .bind(m.missed.len() as i32)
        .bind(&m.missed)
        .bind(m.named_only.len() as i32)
        .bind(o.files_opened)
        .bind(m.opened_not_sent.len() as i32)
        .bind(&m.opened_not_sent)
        .execute(self.pool())
        .await
        .map_err(serr)?;
        Ok(())
    }

    /// One more read of `path` by the task's agent; `expanded` when it came through `expand`.
    pub async fn note_read(&self, task: TaskId, path: &str, expanded: bool) -> Result<()> {
        sqlx::query(
            "INSERT INTO task_reads (task_id, path, expanded) VALUES ($1, $2, $3)
             ON CONFLICT (task_id, path) DO UPDATE
               SET reads = task_reads.reads + 1, expanded = task_reads.expanded OR EXCLUDED.expanded",
        )
        .bind(task.0)
        .bind(path)
        .bind(expanded)
        .execute(self.pool())
        .await
        .map_err(serr)?;
        Ok(())
    }

    pub async fn read_paths(&self, task: TaskId) -> Result<Vec<String>> {
        let rows: Vec<(String,)> = sqlx::query_as(
            "SELECT path FROM task_reads WHERE task_id = $1 ORDER BY path COLLATE \"C\"",
        )
        .bind(task.0)
        .fetch_all(self.pool())
        .await
        .map_err(serr)?;
        Ok(rows.into_iter().map(|(p,)| p).collect())
    }

    /// Every outcome of the repo's tasks, oldest first.
    pub async fn outcomes(&self, repo: &str) -> Result<Vec<OutcomeRow>> {
        let rows = sqlx::query(
            "SELECT o.*, t.slug, t.title,
               NOT (t.status = 'merged' AND t.merge_commit IS NOT DISTINCT FROM o.merge_commit)
                 AS reverted
             FROM pack_outcomes o JOIN tasks t ON t.id = o.task_id
             WHERE o.repo = $1 ORDER BY o.computed_at, o.task_id",
        )
        .bind(repo)
        .fetch_all(self.pool())
        .await
        .map_err(serr)?;
        let mut out = Vec::with_capacity(rows.len());
        for r in rows {
            out.push(OutcomeRow {
                task_id: r.try_get("task_id").map_err(serr)?,
                slug: r.try_get("slug").map_err(serr)?,
                title: r.try_get("title").map_err(serr)?,
                merge_commit: r.try_get("merge_commit").map_err(serr)?,
                changed_source: r.try_get("changed_source").map_err(serr)?,
                packer_version: r.try_get("packer_version").map_err(serr)?,
                first_budget: r.try_get("first_budget").map_err(serr)?,
                packs: r.try_get("packs").map_err(serr)?,
                code_tokens: r.try_get("code_tokens").map_err(serr)?,
                code_tokens_changed: r.try_get("code_tokens_changed").map_err(serr)?,
                code_tokens_used: r.try_get("code_tokens_used").map_err(serr)?,
                files_changed: r.try_get("files_changed").map_err(serr)?,
                files_missed: r.try_get("files_missed").map_err(serr)?,
                missed_paths: r.try_get("missed_paths").map_err(serr)?,
                files_named_only: r.try_get("files_named_only").map_err(serr)?,
                files_opened: r.try_get("files_opened").map_err(serr)?,
                opened_not_sent: r.try_get("opened_not_sent").map_err(serr)?,
                opened_not_sent_paths: r.try_get("opened_not_sent_paths").map_err(serr)?,
                computed_at: r.try_get("computed_at").map_err(serr)?,
                reverted: r.try_get("reverted").map_err(serr)?,
            });
        }
        Ok(out)
    }

    /// Merged tasks of the repo with no outcome yet, oldest first, at most `limit`.
    pub async fn merged_without_outcome(&self, repo: &str, limit: i64) -> Result<Vec<TaskId>> {
        let rows: Vec<(Uuid,)> = sqlx::query_as(
            "SELECT t.id FROM tasks t
             WHERE t.repo = $1 AND t.status = 'merged'
               AND NOT EXISTS (SELECT 1 FROM pack_outcomes o WHERE o.task_id = t.id)
             ORDER BY t.created_at, t.id LIMIT $2",
        )
        .bind(repo)
        .bind(limit)
        .fetch_all(self.pool())
        .await
        .map_err(serr)?;
        Ok(rows.into_iter().map(|(id,)| TaskId(id)).collect())
    }
}
