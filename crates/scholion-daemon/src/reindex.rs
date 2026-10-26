//! `POST /v1/reindex` and the worker's tick: both serialize on one Postgres advisory lock.

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use axum::Json;
use axum::extract::State;
use scholion_core::{AvError, NewOutbox, OutboxOp, Result, SemanticKind, TaskId};
use scholion_store::PgStore;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::pool::PoolConnection;
use sqlx::postgres::Postgres;

use crate::AppState;
use crate::error::ApiResult;
use crate::worker::{DrainStats, OutboxWorker};

/// Advisory lock key shared by reindex and the worker tick.
pub const SEMANTIC_LOCK_KEY: i64 = 7_420_002;

fn serr(e: sqlx::Error) -> AvError {
    AvError::Store(e.to_string())
}

/// The lock is session-scoped: lock and unlock must run on the same pooled connection.
async fn unlock(conn: &mut PoolConnection<Postgres>) {
    let _ = sqlx::query("SELECT pg_advisory_unlock($1)")
        .bind(SEMANTIC_LOCK_KEY)
        .execute(&mut **conn)
        .await;
}

/// One worker tick: skipped (`None`) while a reindex holds the lock.
pub async fn worker_tick(pool: &PgPool, worker: &OutboxWorker) -> Result<Option<DrainStats>> {
    let mut conn = pool.acquire().await.map_err(serr)?;
    let got: bool = sqlx::query_scalar("SELECT pg_try_advisory_lock($1)")
        .bind(SEMANTIC_LOCK_KEY)
        .fetch_one(&mut *conn)
        .await
        .map_err(serr)?;
    if !got {
        return Ok(None);
    }
    let result = worker.drain_once().await;
    unlock(&mut conn).await;
    result.map(Some)
}

#[derive(Deserialize, Default)]
pub(crate) struct ReindexRequest {
    repo: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct ReindexReport {
    repo: Option<String>,
    enqueued: usize,
    upserted: usize,
    degraded: Vec<String>,
}

pub(crate) async fn reindex(
    State(st): State<AppState>,
    body: Option<Json<ReindexRequest>>,
) -> ApiResult<ReindexReport> {
    let repo = body
        .and_then(|Json(b)| b.repo)
        .filter(|r| !r.trim().is_empty());
    let mut conn = st.pg.pool().acquire().await.map_err(serr)?;
    sqlx::query("SELECT pg_advisory_lock($1)")
        .bind(SEMANTIC_LOCK_KEY)
        .execute(&mut *conn)
        .await
        .map_err(serr)?;
    let result = run(&st, repo.as_deref()).await;
    unlock(&mut conn).await;
    Ok(Json(result?))
}

async fn run(st: &AppState, repo: Option<&str>) -> Result<ReindexReport> {
    let rows = rebuild_rows(st, repo).await?;
    let enqueued = rows.len();
    let mut tx = st.pg.pool().begin().await.map_err(serr)?;
    sqlx::query("DELETE FROM outbox WHERE $1::text IS NULL OR repo = $1")
        .bind(repo)
        .execute(&mut *tx)
        .await
        .map_err(serr)?;
    for row in &rows {
        PgStore::enqueue(&mut *tx, row).await?;
    }
    tx.commit().await.map_err(serr)?;

    let mut report = ReindexReport {
        repo: repo.map(str::to_owned),
        enqueued,
        upserted: 0,
        degraded: vec![],
    };
    let (Some(embedder), Some(vectors), Some(worker)) =
        (st.embedder.clone(), st.vectors.clone(), st.worker())
    else {
        report.degraded.push("semantic".into());
        return Ok(report);
    };
    // One repo: drop only its points. All repos: recreate the collections at the embedder's
    // current dimension (the recovery path after a model change).
    let prepared = match repo {
        Some(r) => vectors.delete_repo(r).await,
        None => match embedder.dimension().await {
            Ok(dim) => vectors.recreate(dim).await,
            Err(e) => Err(e),
        },
    };
    if let Err(e) = prepared {
        tracing::warn!(error = %e, "reindex: vector index unavailable");
        report.degraded.push("semantic".into());
        return Ok(report);
    }
    loop {
        let stats = worker.drain_once().await?;
        report.upserted += stats.done;
        // Stop when nothing more can be claimed now (failed rows wait for their backoff).
        if stats.done == 0 || st.pg.outbox_count(repo).await? == 0 {
            break;
        }
    }
    if st.pg.outbox_count(repo).await? > 0 {
        report.degraded.push("semantic".into());
    }
    Ok(report)
}

/// Every embeddable entity of `repo` (or all repos), as upserts. Code chunks follow the
/// embedding rule: `main`, plus a task scope's chunks only for the paths that task changed.
async fn rebuild_rows(st: &AppState, repo: Option<&str>) -> Result<Vec<NewOutbox>> {
    let mut rows = vec![];
    let mut task_repo: HashMap<TaskId, String> = HashMap::new();
    for rec in st.pg.list_records_for_embed(repo).await? {
        if let Entry::Vacant(slot) = task_repo.entry(rec.task_id) {
            slot.insert(st.store.get_task(rec.task_id).await?.repo);
        }
        let text = if rec.body.trim().is_empty() {
            rec.summary.clone()
        } else {
            format!("{}\n{}", rec.summary, rec.body)
        };
        rows.push(NewOutbox {
            kind: SemanticKind::Record,
            op: OutboxOp::Upsert,
            entity_id: rec.id.0.to_string(),
            repo: task_repo[&rec.task_id].clone(),
            task_id: Some(rec.task_id),
            record_id: Some(rec.id),
            symbol_id: None,
            convention_id: None,
            path: None,
            change_id: rec.change_id.clone(),
            superseded: rec.superseded_by.is_some(),
            text: Some(text),
        });
    }
    for c in st.pg.list_all_conventions(repo).await? {
        rows.push(NewOutbox {
            kind: SemanticKind::Convention,
            op: OutboxOp::Upsert,
            entity_id: c.id.0.to_string(),
            repo: c.repo.clone(),
            task_id: None,
            record_id: None,
            symbol_id: None,
            convention_id: Some(c.id),
            path: None,
            change_id: None,
            superseded: false,
            text: Some(c.text.clone()),
        });
    }
    let mut touched: HashMap<TaskId, HashSet<String>> = HashMap::new();
    for chunk in st.index.list_code_chunks(repo).await? {
        if chunk.scope != "main" {
            let Some(tid) = chunk.task_id else { continue };
            if let Entry::Vacant(slot) = touched.entry(tid) {
                slot.insert(st.store.touched_paths(tid).await?.into_iter().collect());
            }
            if !touched[&tid].contains(&chunk.path) {
                continue;
            }
        }
        rows.push(NewOutbox {
            kind: SemanticKind::CodeChunk,
            op: OutboxOp::Upsert,
            entity_id: chunk.symbol_id.0.to_string(),
            repo: chunk.repo.clone(),
            task_id: chunk.task_id,
            record_id: None,
            symbol_id: Some(chunk.symbol_id),
            convention_id: None,
            path: Some(chunk.path.clone()),
            change_id: chunk.change_id.clone(),
            superseded: false,
            text: Some(chunk.text.clone()),
        });
    }
    Ok(rows)
}
