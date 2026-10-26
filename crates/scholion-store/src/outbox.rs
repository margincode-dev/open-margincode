use scholion_core::*;
use sqlx::Row;
use sqlx::postgres::PgRow;
use uuid::Uuid;

use crate::pg::{PgStore, RECORD_COLS, serr};

impl PgStore {
    pub async fn enqueue<'a, E>(executor: E, row: &NewOutbox) -> Result<()>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        sqlx::query(
            "INSERT INTO outbox (
                kind, op, entity_id, repo, task_id, record_id, symbol_id, convention_id,
                path, change_id, superseded, text
             ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)",
        )
        .bind(row.kind.as_str())
        .bind(row.op.as_str())
        .bind(&row.entity_id)
        .bind(&row.repo)
        .bind(row.task_id.map(|id| id.0))
        .bind(row.record_id.map(|id| id.0))
        .bind(row.symbol_id.map(|id| id.0))
        .bind(row.convention_id.map(|id| id.0))
        .bind(&row.path)
        .bind(row.change_id.as_ref().map(|id| id.0.clone()))
        .bind(row.superseded)
        .bind(&row.text)
        .execute(executor)
        .await
        .map_err(serr)?;
        Ok(())
    }

    /// `enqueue` for many rows in one statement; ids ascend in slice order.
    pub async fn enqueue_many<'a, E>(executor: E, rows: &[NewOutbox]) -> Result<()>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        if rows.is_empty() {
            return Ok(());
        }
        let col = |f: fn(&NewOutbox) -> Option<String>| rows.iter().map(f).collect::<Vec<_>>();
        sqlx::query(
            "INSERT INTO outbox (
                kind, op, entity_id, repo, task_id, record_id, symbol_id, convention_id,
                path, change_id, superseded, text
             )
             SELECT kind, op, entity_id, repo, task_id, record_id, symbol_id, convention_id,
                    path, change_id, superseded, text
             FROM unnest($1::text[], $2::text[], $3::text[], $4::text[], $5::uuid[], $6::uuid[],
                         $7::bigint[], $8::bigint[], $9::text[], $10::text[], $11::bool[],
                         $12::text[])
                  WITH ORDINALITY AS u(kind, op, entity_id, repo, task_id, record_id, symbol_id,
                                       convention_id, path, change_id, superseded, text, ord)
             ORDER BY ord",
        )
        .bind(col(|r| Some(r.kind.as_str().to_owned())))
        .bind(col(|r| Some(r.op.as_str().to_owned())))
        .bind(col(|r| Some(r.entity_id.clone())))
        .bind(col(|r| Some(r.repo.clone())))
        .bind(
            rows.iter()
                .map(|r| r.task_id.map(|id| id.0))
                .collect::<Vec<_>>(),
        )
        .bind(
            rows.iter()
                .map(|r| r.record_id.map(|id| id.0))
                .collect::<Vec<_>>(),
        )
        .bind(
            rows.iter()
                .map(|r| r.symbol_id.map(|id| id.0))
                .collect::<Vec<_>>(),
        )
        .bind(
            rows.iter()
                .map(|r| r.convention_id.map(|id| id.0))
                .collect::<Vec<_>>(),
        )
        .bind(col(|r| r.path.clone()))
        .bind(col(|r| r.change_id.as_ref().map(|c| c.0.clone())))
        .bind(rows.iter().map(|r| r.superseded).collect::<Vec<_>>())
        .bind(col(|r| r.text.clone()))
        .execute(executor)
        .await
        .map_err(serr)?;
        Ok(())
    }

    /// Claims up to `limit` rows in id order. A claimed row stays locked for 60 seconds.
    pub async fn claim_outbox(&self, limit: i64) -> Result<Vec<OutboxRow>> {
        let rows = sqlx::query(
            "WITH picked AS (
               SELECT id FROM outbox
               WHERE locked_until IS NULL OR locked_until < now()
               ORDER BY id
               LIMIT $1
               FOR UPDATE SKIP LOCKED
             )
             UPDATE outbox o
             SET attempts = o.attempts + 1,
                 locked_until = now() + interval '60 seconds'
             FROM picked
             WHERE o.id = picked.id
             RETURNING o.id, o.kind, o.op, o.entity_id, o.repo, o.task_id, o.record_id,
                       o.symbol_id, o.convention_id, o.path, o.change_id, o.superseded,
                       o.text, o.attempts, o.last_error",
        )
        .bind(limit)
        .fetch_all(self.pool())
        .await
        .map_err(serr)?;
        let mut out = Vec::with_capacity(rows.len());
        for row in &rows {
            out.push(outbox_from_row(row)?);
        }
        out.sort_by_key(|row| row.id);
        Ok(out)
    }

    pub async fn complete_outbox(&self, id: i64) -> Result<()> {
        sqlx::query("DELETE FROM outbox WHERE id = $1")
            .bind(id)
            .execute(self.pool())
            .await
            .map_err(serr)?;
        Ok(())
    }

    pub async fn fail_outbox(&self, id: i64, error: &str) -> Result<()> {
        sqlx::query(
            "UPDATE outbox
             SET locked_until = now() + LEAST(attempts, 60) * interval '5 seconds',
                 last_error = $2
             WHERE id = $1",
        )
        .bind(id)
        .bind(error)
        .execute(self.pool())
        .await
        .map_err(serr)?;
        Ok(())
    }

    pub async fn clear_outbox(&self, repo: Option<&str>) -> Result<u64> {
        let result = sqlx::query("DELETE FROM outbox WHERE $1::text IS NULL OR repo = $1")
            .bind(repo)
            .execute(self.pool())
            .await
            .map_err(serr)?;
        Ok(result.rows_affected())
    }

    pub async fn outbox_count(&self, repo: Option<&str>) -> Result<i64> {
        sqlx::query_scalar("SELECT count(*) FROM outbox WHERE $1::text IS NULL OR repo = $1")
            .bind(repo)
            .fetch_one(self.pool())
            .await
            .map_err(serr)
    }

    /// Includes superseded records so a rebuild can mark them.
    pub async fn list_records_for_embed(&self, repo: Option<&str>) -> Result<Vec<Record>> {
        let rows = sqlx::query(&format!(
            "SELECT {RECORD_COLS} FROM records r
             JOIN tasks t ON t.id = r.task_id
             WHERE $1::text IS NULL OR t.repo = $1
             ORDER BY r.created_at, r.id"
        ))
        .bind(repo)
        .fetch_all(self.pool())
        .await
        .map_err(serr)?;
        self.hydrate_records(rows).await
    }

    pub async fn list_all_conventions(&self, repo: Option<&str>) -> Result<Vec<Convention>> {
        let rows = sqlx::query(
            "SELECT id, repo, scope_glob, text, required FROM conventions
             WHERE ($1::text IS NULL OR repo = $1) AND removed_at IS NULL
             ORDER BY id",
        )
        .bind(repo)
        .fetch_all(self.pool())
        .await
        .map_err(serr)?;
        let mut out = Vec::with_capacity(rows.len());
        for row in &rows {
            out.push(Convention {
                id: ConventionId(row.try_get("id").map_err(serr)?),
                repo: row.try_get("repo").map_err(serr)?,
                scope_glob: row.try_get("scope_glob").map_err(serr)?,
                text: row.try_get("text").map_err(serr)?,
                required: row.try_get("required").map_err(serr)?,
                check_command: None,
            });
        }
        Ok(out)
    }
}

fn outbox_from_row(row: &PgRow) -> Result<OutboxRow> {
    let kind: String = row.try_get("kind").map_err(serr)?;
    let op: String = row.try_get("op").map_err(serr)?;
    Ok(OutboxRow {
        id: row.try_get("id").map_err(serr)?,
        attempts: row.try_get("attempts").map_err(serr)?,
        last_error: row.try_get("last_error").map_err(serr)?,
        row: NewOutbox {
            kind: kind.parse()?,
            op: op.parse()?,
            entity_id: row.try_get("entity_id").map_err(serr)?,
            repo: row.try_get("repo").map_err(serr)?,
            task_id: row
                .try_get::<Option<Uuid>, _>("task_id")
                .map_err(serr)?
                .map(TaskId),
            record_id: row
                .try_get::<Option<Uuid>, _>("record_id")
                .map_err(serr)?
                .map(RecordId),
            symbol_id: row
                .try_get::<Option<i64>, _>("symbol_id")
                .map_err(serr)?
                .map(SymbolId),
            convention_id: row
                .try_get::<Option<i64>, _>("convention_id")
                .map_err(serr)?
                .map(ConventionId),
            path: row.try_get("path").map_err(serr)?,
            change_id: row
                .try_get::<Option<String>, _>("change_id")
                .map_err(serr)?
                .map(ChangeId),
            superseded: row.try_get("superseded").map_err(serr)?,
            text: row.try_get("text").map_err(serr)?,
        },
    })
}
