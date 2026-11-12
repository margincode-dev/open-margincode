//! Dreams and verdicts (Plan 16 Phase 2): the rows of an offline look at records. The rules for
//! what a verdict must show are the daemon's (`margincode-daemon`'s `dream.rs`); here are the rows
//! and the one definition of a verdict's state.

use chrono::{DateTime, Utc};
use margincode_core::*;
use serde::Serialize;
use serde_json::Value;
use sqlx::Row;
use sqlx::postgres::PgRow;
use uuid::Uuid;

use crate::pg::{PgStore, RECORD_COLS, serr};

/// A verdict's state, by its newest review and whether a `retracted` row names it:
/// `proposed` (no review yet, no effect), `effective` (the newest review agrees), `rejected`
/// (it disagrees), `retracted`. A `retracted` row itself is a `retraction`.
pub(crate) const VERDICT_STATE: &str = "CASE
    WHEN v.verdict = 'retracted' THEN 'retraction'
    WHEN EXISTS (SELECT 1 FROM record_verdicts x WHERE x.retracts = v.id) THEN 'retracted'
    ELSE COALESCE((SELECT CASE WHEN w.agree THEN 'effective' ELSE 'rejected' END
                   FROM verdict_reviews w WHERE w.verdict_id = v.id
                   ORDER BY w.id DESC LIMIT 1), 'proposed')
  END";

const VERDICT_COLS: &str = "v.id, v.record_id, v.verdict, v.reason, v.canonical, v.retracts,
    v.evidence, v.anchor_hashes, v.author_agent, v.dream_id, v.created_at";

const DREAM_COLS: &str = "id, repo, mode, area, agent, instructions_sha, status, worklist, counts,
    started_at, finished_at, lease_expires";

#[derive(Debug, Clone, Serialize)]
pub struct Dream {
    pub id: Uuid,
    pub repo: String,
    /// `review` (proposes verdicts on records) or `verify` (reviews proposed verdicts).
    pub mode: String,
    /// A path prefix; empty is the whole repository.
    pub area: String,
    pub agent: String,
    pub instructions_sha: Option<String>,
    /// `running`, `done`, `failed` or `undone`.
    pub status: String,
    pub worklist: Value,
    pub counts: Value,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub lease_expires: DateTime<Utc>,
}

pub struct NewDream<'a> {
    pub repo: &'a str,
    pub mode: &'a str,
    pub area: &'a str,
    pub agent: &'a str,
    pub instructions_sha: Option<&'a str>,
    pub worklist: Value,
    pub lease_secs: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct VerdictRow {
    pub id: Uuid,
    pub record_id: RecordId,
    /// `confirmed`, `disputed`, `duplicate`, `outdated` or `retracted`.
    pub verdict: String,
    pub reason: String,
    pub canonical: Option<RecordId>,
    pub retracts: Option<Uuid>,
    pub evidence: Value,
    #[serde(skip)]
    pub anchor_hashes: Value,
    pub author_agent: String,
    pub dream_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    /// `proposed`, `effective`, `rejected`, `retracted` or `retraction` (`VERDICT_STATE`).
    pub state: String,
}

pub struct NewVerdict<'a> {
    pub record: RecordId,
    pub verdict: &'a str,
    pub reason: &'a str,
    pub canonical: Option<RecordId>,
    pub retracts: Option<Uuid>,
    pub evidence: Value,
    pub anchor_hashes: Value,
    pub author_agent: &'a str,
    pub dream: Option<Uuid>,
}

/// A record a dream may look at, with what the worklist's ranking needs.
pub struct DreamCandidate {
    pub record: Record,
    /// Whether any verdict was ever written on it.
    pub had_verdict: bool,
}

fn dream_from(r: &PgRow) -> Result<Dream> {
    Ok(Dream {
        id: r.try_get("id").map_err(serr)?,
        repo: r.try_get("repo").map_err(serr)?,
        mode: r.try_get("mode").map_err(serr)?,
        area: r.try_get("area").map_err(serr)?,
        agent: r.try_get("agent").map_err(serr)?,
        instructions_sha: r.try_get("instructions_sha").map_err(serr)?,
        status: r.try_get("status").map_err(serr)?,
        worklist: r.try_get("worklist").map_err(serr)?,
        counts: r.try_get("counts").map_err(serr)?,
        started_at: r.try_get("started_at").map_err(serr)?,
        finished_at: r.try_get("finished_at").map_err(serr)?,
        lease_expires: r.try_get("lease_expires").map_err(serr)?,
    })
}

fn verdict_from(r: &PgRow) -> Result<VerdictRow> {
    Ok(VerdictRow {
        id: r.try_get("id").map_err(serr)?,
        record_id: RecordId(r.try_get("record_id").map_err(serr)?),
        verdict: r.try_get("verdict").map_err(serr)?,
        reason: r.try_get("reason").map_err(serr)?,
        canonical: r
            .try_get::<Option<Uuid>, _>("canonical")
            .map_err(serr)?
            .map(RecordId),
        retracts: r.try_get("retracts").map_err(serr)?,
        evidence: r.try_get("evidence").map_err(serr)?,
        anchor_hashes: r.try_get("anchor_hashes").map_err(serr)?,
        author_agent: r.try_get("author_agent").map_err(serr)?,
        dream_id: r.try_get("dream_id").map_err(serr)?,
        created_at: r.try_get("created_at").map_err(serr)?,
        state: r.try_get("state").map_err(serr)?,
    })
}

/// Whether one area contains the other (a path prefix by whole segments; '' contains all).
pub fn areas_overlap(a: &str, b: &str) -> bool {
    let within = |inner: &str, outer: &str| {
        outer.is_empty()
            || inner == outer
            || inner
                .strip_prefix(outer)
                .is_some_and(|rest| rest.starts_with('/'))
    };
    within(a, b) || within(b, a)
}

impl PgStore {
    /// Starts a dream. One run of a mode per area at a time: `Conflict` when a running one's
    /// area contains this one's or is contained in it. A running dream whose lease ran out is
    /// first marked `failed`, so a run that died blocks nothing.
    pub async fn create_dream(&self, d: NewDream<'_>) -> Result<Dream> {
        let mut tx = self.pool().begin().await.map_err(serr)?;
        // one writer of this decision per repo
        sqlx::query("SELECT pg_advisory_xact_lock(hashtext('dreams:' || $1))")
            .bind(d.repo)
            .execute(&mut *tx)
            .await
            .map_err(serr)?;
        sqlx::query(
            "UPDATE dreams SET status = 'failed', finished_at = now()
             WHERE repo = $1 AND status = 'running' AND lease_expires < now()",
        )
        .bind(d.repo)
        .execute(&mut *tx)
        .await
        .map_err(serr)?;
        let running = sqlx::query(
            "SELECT id, area FROM dreams WHERE repo = $1 AND mode = $2 AND status = 'running'",
        )
        .bind(d.repo)
        .bind(d.mode)
        .fetch_all(&mut *tx)
        .await
        .map_err(serr)?;
        for r in &running {
            let area: String = r.try_get("area").map_err(serr)?;
            if areas_overlap(&area, d.area) {
                let id: Uuid = r.try_get("id").map_err(serr)?;
                let shown = if area.is_empty() {
                    "the whole repository"
                } else {
                    &area
                };
                return Err(AvError::Conflict(format!(
                    "dream {id} is running on {shown}; one {} run per area at a time",
                    d.mode
                )));
            }
        }
        let row = sqlx::query(&format!(
            "INSERT INTO dreams (id, repo, mode, area, agent, instructions_sha, worklist,
                                 lease_expires)
             VALUES ($1, $2, $3, $4, $5, $6, $7, now() + make_interval(secs => $8))
             RETURNING {DREAM_COLS}"
        ))
        .bind(Uuid::new_v4())
        .bind(d.repo)
        .bind(d.mode)
        .bind(d.area)
        .bind(d.agent)
        .bind(d.instructions_sha)
        .bind(&d.worklist)
        .bind(d.lease_secs as f64)
        .fetch_one(&mut *tx)
        .await
        .map_err(serr)?;
        tx.commit().await.map_err(serr)?;
        dream_from(&row)
    }

    pub async fn get_dream(&self, id: Uuid) -> Result<Dream> {
        let row = sqlx::query(&format!("SELECT {DREAM_COLS} FROM dreams WHERE id = $1"))
            .bind(id)
            .fetch_optional(self.pool())
            .await
            .map_err(serr)?
            .ok_or_else(|| AvError::NotFound(format!("dream {id}")))?;
        dream_from(&row)
    }

    /// Ends a dream with `status` and `counts`; only a running one (else `IllegalState`), or a
    /// `done` one being `undone`.
    pub async fn end_dream(&self, id: Uuid, status: &str, counts: Value) -> Result<Dream> {
        let from: &[&str] = if status == "undone" {
            &["running", "done", "failed"]
        } else {
            &["running"]
        };
        let from: Vec<String> = from.iter().map(|s| s.to_string()).collect();
        let row = sqlx::query(&format!(
            "UPDATE dreams SET status = $2, counts = $3, finished_at = now()
             WHERE id = $1 AND status = ANY($4) RETURNING {DREAM_COLS}"
        ))
        .bind(id)
        .bind(status)
        .bind(&counts)
        .bind(&from)
        .fetch_optional(self.pool())
        .await
        .map_err(serr)?;
        match row {
            Some(r) => dream_from(&r),
            None => {
                let d = self.get_dream(id).await?;
                Err(AvError::IllegalState(format!(
                    "dream {id} is {}, not running",
                    d.status
                )))
            }
        }
    }

    /// Whether the dream is running and inside its lease.
    pub fn dream_live(d: &Dream) -> bool {
        d.status == "running" && d.lease_expires > Utc::now()
    }

    pub async fn insert_verdict(&self, v: NewVerdict<'_>) -> Result<VerdictRow> {
        let id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO record_verdicts (id, record_id, verdict, reason, canonical, retracts,
                                          evidence, anchor_hashes, author_agent, dream_id)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
        )
        .bind(id)
        .bind(v.record.0)
        .bind(v.verdict)
        .bind(v.reason)
        .bind(v.canonical.map(|c| c.0))
        .bind(v.retracts)
        .bind(&v.evidence)
        .bind(&v.anchor_hashes)
        .bind(v.author_agent)
        .bind(v.dream)
        .execute(self.pool())
        .await
        .map_err(
            |e| match e.as_database_error().and_then(|d| d.constraint()) {
                Some("record_verdicts_retracts") => {
                    AvError::Conflict("that verdict is already retracted".into())
                }
                _ => serr(e),
            },
        )?;
        self.get_verdict(id).await
    }

    pub async fn get_verdict(&self, id: Uuid) -> Result<VerdictRow> {
        let row = sqlx::query(&format!(
            "SELECT {VERDICT_COLS}, {VERDICT_STATE} AS state FROM record_verdicts v
             WHERE v.id = $1"
        ))
        .bind(id)
        .fetch_optional(self.pool())
        .await
        .map_err(serr)?
        .ok_or_else(|| AvError::NotFound(format!("verdict {id}")))?;
        verdict_from(&row)
    }

    /// A verdict by its full id, `vd:<hex>` or a hex prefix of 6 or more characters.
    pub async fn find_verdict(&self, id_or_prefix: &str) -> Result<VerdictRow> {
        let typed = id_or_prefix.trim().to_lowercase();
        let typed = typed.strip_prefix("vd:").unwrap_or(&typed).replace('-', "");
        if typed.len() < 6 || typed.len() > 32 || !typed.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(AvError::Invalid(format!(
                "{id_or_prefix:?} is not a verdict id (vd:… or 6 or more hex characters)"
            )));
        }
        let rows = sqlx::query(
            "SELECT id FROM record_verdicts
             WHERE replace(id::text, '-', '') LIKE $1 || '%' LIMIT 2",
        )
        .bind(&typed)
        .fetch_all(self.pool())
        .await
        .map_err(serr)?;
        match rows.as_slice() {
            [] => Err(AvError::NotFound(format!("verdict {id_or_prefix}"))),
            [one] => self.get_verdict(one.try_get("id").map_err(serr)?).await,
            _ => Err(AvError::Invalid(format!(
                "{id_or_prefix:?} names more than one verdict; give more of the id"
            ))),
        }
    }

    /// The verdicts on records of `repo`, newest first; `state` keeps one state
    /// (`VERDICT_STATE`), `dream` one run's, `record` one record's.
    pub async fn list_verdicts(
        &self,
        repo: &str,
        state: Option<&str>,
        dream: Option<Uuid>,
        record: Option<RecordId>,
        limit: i64,
    ) -> Result<Vec<VerdictRow>> {
        let rows = sqlx::query(&format!(
            "SELECT * FROM (
               SELECT {VERDICT_COLS}, {VERDICT_STATE} AS state
               FROM record_verdicts v
               JOIN records r ON r.id = v.record_id JOIN tasks t ON t.id = r.task_id
               WHERE t.repo = $1 AND ($3::uuid IS NULL OR v.dream_id = $3)
                 AND ($4::uuid IS NULL OR v.record_id = $4)
             ) q WHERE ($2::text IS NULL OR q.state = $2)
             ORDER BY q.created_at DESC, q.id LIMIT $5"
        ))
        .bind(repo)
        .bind(state)
        .bind(dream)
        .bind(record.map(|r| r.0))
        .bind(limit)
        .fetch_all(self.pool())
        .await
        .map_err(serr)?;
        rows.iter().map(verdict_from).collect()
    }

    /// Writes a review of a proposed verdict. `Conflict` unless the verdict is still proposed:
    /// checked and written under a lock on the verdict's row, so two reviews at once leave one.
    pub async fn add_verdict_review(
        &self,
        verdict: Uuid,
        reviewer: &str,
        dream: Option<Uuid>,
        agree: bool,
        evidence: &Value,
    ) -> Result<VerdictRow> {
        let mut tx = self.pool().begin().await.map_err(serr)?;
        let state: Option<String> = sqlx::query_scalar(&format!(
            "SELECT {VERDICT_STATE} FROM record_verdicts v WHERE v.id = $1 FOR UPDATE"
        ))
        .bind(verdict)
        .fetch_optional(&mut *tx)
        .await
        .map_err(serr)?;
        match state.as_deref() {
            None => return Err(AvError::NotFound(format!("verdict {verdict}"))),
            Some("proposed") => {}
            Some(other) => {
                return Err(AvError::Conflict(format!(
                    "verdict {verdict} is {other}, not proposed"
                )));
            }
        }
        sqlx::query(
            "INSERT INTO verdict_reviews (verdict_id, reviewer, dream_id, agree, evidence)
             VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(verdict)
        .bind(reviewer)
        .bind(dream)
        .bind(agree)
        .bind(evidence)
        .execute(&mut *tx)
        .await
        .map_err(serr)?;
        tx.commit().await.map_err(serr)?;
        self.get_verdict(verdict).await
    }

    /// The live records of `repo` a dream may judge, newest first, at most `scan`: decisions,
    /// attempts and notes written by agents (not mined from git), not superseded, not hidden
    /// by a verdict in effect, with no verdict waiting for its review, and, with an `area`,
    /// anchored under it.
    pub async fn dream_candidates(
        &self,
        repo: &str,
        area: &str,
        scan: i64,
    ) -> Result<Vec<DreamCandidate>> {
        let rows = sqlx::query(&format!(
            "SELECT {RECORD_COLS},
                    EXISTS (SELECT 1 FROM record_verdicts v WHERE v.record_id = r.id)
                      AS had_verdict
             FROM records r JOIN tasks t ON t.id = r.task_id
             WHERE t.repo = $1
               AND r.kind IN ('decision', 'attempt', 'note')
               AND r.author_agent <> $2
               AND NOT EXISTS (SELECT 1 FROM records n WHERE n.supersedes = r.id)
               AND NOT EXISTS (
                 SELECT 1 FROM record_verdicts v WHERE v.record_id = r.id
                   AND (({VERDICT_STATE}) = 'proposed'
                        OR (({VERDICT_STATE}) = 'effective'
                            AND v.verdict IN ('duplicate', 'outdated'))))
               AND ($3 = '' OR EXISTS (
                 SELECT 1 FROM anchors a WHERE a.record_id = r.id
                   AND (a.path = $3 OR a.path LIKE replace(replace(replace($3,
                        '\\', '\\\\'), '%', '\\%'), '_', '\\_') || '/%')))
             ORDER BY r.created_at DESC, r.id LIMIT $4"
        ))
        .bind(repo)
        .bind(margincode_core::history::MINED_AUTHOR)
        .bind(area)
        .bind(scan)
        .fetch_all(self.pool())
        .await
        .map_err(serr)?;
        let had: Vec<bool> = rows
            .iter()
            .map(|r| r.try_get("had_verdict").map_err(serr))
            .collect::<Result<_>>()?;
        let records = self.hydrate_records(rows).await?;
        Ok(records
            .into_iter()
            .zip(had)
            .map(|(record, had_verdict)| DreamCandidate {
                record,
                had_verdict,
            })
            .collect())
    }

    /// How many packs of the last `days` days showed each record of `repo`, by the `rec:` id a
    /// receipt lists it under.
    pub async fn records_shown(
        &self,
        repo: &str,
        days: i32,
    ) -> Result<std::collections::HashMap<String, i64>> {
        let rows = sqlx::query(
            "SELECT i->>'id' AS id, count(*) AS n
             FROM context_receipts c JOIN tasks t ON t.id = c.task_id,
                  jsonb_array_elements(c.items) i
             WHERE t.repo = $1 AND c.created_at > now() - make_interval(days => $2)
               AND i->>'id' LIKE 'rec:%'
             GROUP BY 1",
        )
        .bind(repo)
        .bind(days)
        .fetch_all(self.pool())
        .await
        .map_err(serr)?;
        rows.iter()
            .map(|r| {
                Ok((
                    r.try_get("id").map_err(serr)?,
                    r.try_get("n").map_err(serr)?,
                ))
            })
            .collect()
    }

    /// An event of `repo` by its id, if there is one.
    pub async fn event_in_repo(&self, repo: &str, id: i64) -> Result<bool> {
        sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM events WHERE id = $1 AND repo = $2)")
            .bind(id)
            .bind(repo)
            .fetch_one(self.pool())
            .await
            .map_err(serr)
    }

    /// The lifecycle events of a task, oldest first, without its checkpoints.
    pub async fn task_events(&self, task: TaskId, limit: i64) -> Result<Vec<Value>> {
        let rows = sqlx::query(
            "SELECT id, kind, at FROM events
             WHERE task_id = $1 AND kind <> 'checkpoint' ORDER BY id LIMIT $2",
        )
        .bind(task.0)
        .bind(limit)
        .fetch_all(self.pool())
        .await
        .map_err(serr)?;
        rows.iter()
            .map(|r| {
                let id: i64 = r.try_get("id").map_err(serr)?;
                let kind: String = r.try_get("kind").map_err(serr)?;
                let at: DateTime<Utc> = r.try_get("at").map_err(serr)?;
                Ok(serde_json::json!({"id": id, "kind": kind, "at": at}))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::areas_overlap;

    #[test]
    fn areas_overlap_by_whole_path_segments() {
        assert!(areas_overlap("", "src/billing"));
        assert!(areas_overlap("src", "src/billing"));
        assert!(areas_overlap("src/billing", "src"));
        assert!(areas_overlap("src/billing", "src/billing"));
        assert!(!areas_overlap("src/bill", "src/billing"));
        assert!(!areas_overlap("src/billing", "src/api"));
    }
}
