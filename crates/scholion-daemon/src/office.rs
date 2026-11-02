//! Plan 10 A3: the office and each agent's timeline, for the dashboard (`/ui`). Read-only and
//! built from what the store already keeps (records, feed events, tasks, the reviews `next` has
//! reserved); nothing new is recorded, and no cost or usage is tracked (that is the commercial
//! edition's).

use std::collections::BTreeSet;
use std::time::Instant;

use axum::extract::{Path, Query, State};
use axum::routing::get;
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use scholion_core::*;
use scholion_store::TaskSummary;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::AppState;
use crate::error::ApiResult;

/// Entries a timeline returns unless `limit` says otherwise, and the most it returns.
const TIMELINE_DEFAULT: i64 = 100;
const TIMELINE_MAX: i64 = 500;

/// The most open or in-review tasks the office looks at.
const OFFICE_TASKS: usize = 1000;

pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .route("/v1/office", get(office))
        .route("/v1/agents/{name}/timeline", get(timeline))
}

#[derive(Deserialize)]
struct TimelineQuery {
    repo: Option<String>,
    limit: Option<i64>,
}

/// What an agent did, newest first (`PgStore::agent_timeline`).
async fn timeline(
    State(st): State<AppState>,
    Path(name): Path<String>,
    Query(q): Query<TimelineQuery>,
) -> ApiResult<Value> {
    let name = name.trim();
    if name.is_empty() {
        return Err(AvError::Invalid("agent name must not be blank".into()).into());
    }
    let repo = q.repo.as_deref().filter(|r| !r.is_empty());
    let limit = q.limit.unwrap_or(TIMELINE_DEFAULT).clamp(1, TIMELINE_MAX);
    let items = st.pg.agent_timeline(name, repo, limit).await?;
    Ok(Json(json!({ "agent": name, "items": items })))
}

#[derive(Deserialize)]
struct OfficeQuery {
    repo: Option<String>,
}

/// One task as the office shows it.
fn desk_task(t: &TaskSummary) -> Value {
    json!({
        "id": t.task.id, "repo": t.task.repo, "slug": t.task.slug, "title": t.task.title,
        "status": t.task.status.as_str(),
    })
}

/// When an open task's owner last wrote to it: its lease runs `lease_secs` from the last fenced
/// write, and from the start before the first one (as `abandoned_tasks` reckons it).
fn last_write(t: &TaskSummary, lease_secs: u64) -> DateTime<Utc> {
    let lease = chrono::Duration::seconds(i64::try_from(lease_secs).unwrap_or(i64::MAX / 2));
    match t.task.lease_expires {
        Some(until) => until - lease,
        None => t.created_at,
    }
}

/// Every agent with a profile (cheapest first), then everyone else who holds an open or
/// in-review task or a reserved review (by name). For each: its open tasks, its tasks waiting
/// for review, the reviews `next` reserved for it, what it did last, and a state:
/// `working` (an open task written to within a lease), `quiet` (open tasks, none written to
/// within a lease: `next` offers them to others), `reviewing` (only reserved reviews) or `idle`.
async fn office(State(st): State<AppState>, Query(q): Query<OfficeQuery>) -> ApiResult<Value> {
    let repo = q.repo.as_deref().filter(|r| !r.is_empty());
    let profiles = st.store.list_profiles().await?;
    let open = st
        .pg
        .list_tasks(repo, Some(TaskStatus::Open), OFFICE_TASKS)
        .await?;
    let in_review = st
        .pg
        .list_tasks(repo, Some(TaskStatus::InReview), OFFICE_TASKS)
        .await?;
    let reserved: Vec<(TaskId, String, u64)> = {
        let now = Instant::now();
        let map = st.reviews.lock().unwrap_or_else(|p| p.into_inner());
        map.iter()
            .filter(|(_, (_, until))| *until > now)
            .map(|(task, (who, until))| (*task, who.clone(), (*until - now).as_secs()))
            .collect()
    };

    let mut names: Vec<String> = profiles.iter().map(|p| p.name.clone()).collect();
    let others: BTreeSet<String> = open
        .iter()
        .chain(&in_review)
        .map(|t| t.task.owner_agent.clone())
        .chain(reserved.iter().map(|(_, who, _)| who.clone()))
        .filter(|n| !names.contains(n))
        .collect();
    names.extend(others);

    let now = Utc::now();
    let mut agents = Vec::with_capacity(names.len());
    for name in &names {
        let mine: Vec<&TaskSummary> = open
            .iter()
            .filter(|t| &t.task.owner_agent == name)
            .collect();
        let waiting: Vec<Value> = in_review
            .iter()
            .filter(|t| &t.task.owner_agent == name)
            .map(desk_task)
            .collect();
        let reviewing: Vec<Value> = reserved
            .iter()
            .filter(|(_, who, _)| who == name)
            .filter_map(|(task, _, left)| {
                let t = in_review.iter().find(|t| t.task.id == *task)?;
                let mut v = desk_task(t);
                v["reserved_secs"] = json!(left);
                Some(v)
            })
            .collect();
        let last = st
            .pg
            .agent_timeline(name, repo, 1)
            .await?
            .into_iter()
            .next();
        let wrote = mine.iter().map(|t| last_write(t, st.lease_secs)).max();
        let last_at = [last.as_ref().map(|l| l.at), wrote]
            .into_iter()
            .flatten()
            .max();
        let lease = chrono::Duration::seconds(i64::try_from(st.lease_secs).unwrap_or(i64::MAX / 2));
        let live = mine
            .iter()
            .any(|t| last_write(t, st.lease_secs) + lease > now);
        let state = if !mine.is_empty() {
            if live { "working" } else { "quiet" }
        } else if !reviewing.is_empty() {
            "reviewing"
        } else {
            "idle"
        };
        let open_tasks: Vec<Value> = mine
            .iter()
            .map(|t| {
                let mut v = desk_task(t);
                v["last_write"] = json!(last_write(t, st.lease_secs));
                v
            })
            .collect();
        agents.push(json!({
            "name": name,
            "profile": profiles.iter().find(|p| &p.name == name),
            "state": state,
            "open": open_tasks,
            "in_review": waiting,
            "reviewing": reviewing,
            "last": last,
            "last_at": last_at,
        }));
    }
    Ok(Json(json!({
        "repo": repo, "lease_secs": st.lease_secs, "now": now, "agents": agents,
    })))
}
