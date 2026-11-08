//! Plan 15 Phase 2: `GET /v1/attention`, what needs a person or an agent now, worst first. The
//! dashboard's "Needs attention" strip used to compute this in the page; it reads this route
//! now, and so can an alerting extension, a digest script or a cron job. Read-only, built from
//! what the store already keeps (tasks and their feed events, the backlog and profiles, targets,
//! post-merge checks) and the footprints sessions reported; no cost or usage is tracked.
//!
//! Kinds (stable strings) and severities (1 low .. 3 high):
//! - 3 `disk_low`: free space under the workspace root is below `AV_DISK_MIN_FREE_GB` even after
//!   the daemon reclaimed what it could; no new task starts until there is room.
//! - 3 `main_red`: the post-merge check of main's newest checked commit failed.
//! - 3 `conflicted`: a task's merge conflicted; its owner must sync, resolve and resubmit.
//! - 2 `stale_review`: in review, no approval at its head, for over `review_secs`.
//! - 2 `approved_not_merged`: approved at its head and not merged for over `approved_secs`.
//! - 2 `unclaimed_handoff`: handed off and not claimed for over `handoff_secs`.
//! - 2 `quiet_lease`: open, and its owner has not written to it for a whole lease.
//! - 1 `unroutable_backlog`: a queued item no profile fits.
//! - 1 `proposed_backlog`: a proposed item waiting for a human to accept or drop it.
//! - 1 `overdue_target`: an active target past its due day (UTC).
//! - 1 `build_over_budget`: a repo's build output is still over `AV_BUILD_CACHE_MAX_GB` after the
//!   daemon removed what it may (only `target/`s touched within `AV_BUILD_IDLE_SECS` remain).
//! - 1 `footprint_outside`: a session left at least `footprint_bytes` outside any workspace,
//!   reported within `footprint_secs`.

use std::collections::HashMap;

use axum::extract::{Query, State};
use axum::routing::get;
use axum::{Json, Router};
use chrono::{DateTime, Duration, NaiveTime, Utc};
use scholion_core::routing::{BacklogState, TargetStatus};
use scholion_core::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use uuid::Uuid;

use crate::AppState;
use crate::error::ApiResult;

/// The thresholds of `GET /v1/attention`: how long a task may wait in a status before it needs
/// attention, and what footprint is worth a row. The defaults are the dashboard's former
/// constants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttentionLimits {
    /// `AV_ATTN_REVIEW_SECS` (default 7200): a review nobody gave.
    pub review_secs: u64,
    /// `AV_ATTN_APPROVED_SECS` (default 3600): an approval nobody merged.
    pub approved_secs: u64,
    /// `AV_ATTN_HANDOFF_SECS` (default 3600): a handoff nobody claimed.
    pub handoff_secs: u64,
    /// `AV_ATTN_FOOTPRINT_BYTES` (default 512 MiB, the hooks' warning size): what a session may
    /// leave outside any workspace before it is worth a row.
    pub footprint_bytes: u64,
    /// `AV_ATTN_FOOTPRINT_SECS` (default 86400): footprints older than this are left out, since
    /// the files may be gone.
    pub footprint_secs: u64,
}

impl Default for AttentionLimits {
    fn default() -> Self {
        Self {
            review_secs: 2 * 3600,
            approved_secs: 3600,
            handoff_secs: 3600,
            footprint_bytes: 512 * 1024 * 1024,
            footprint_secs: 24 * 3600,
        }
    }
}

pub(crate) fn routes() -> Router<AppState> {
    Router::new().route("/v1/attention", get(attention))
}

/// Each kind's severity and its place among kinds of that severity (the strip's order). A
/// stale review and an approval nobody merged share a place, so they interleave by `since`.
fn rank(kind: &str) -> (u8, u8) {
    match kind {
        "disk_low" => (3, 0),
        "main_red" => (3, 1),
        "conflicted" => (3, 2),
        "stale_review" | "approved_not_merged" => (2, 0),
        "unclaimed_handoff" => (2, 1),
        "quiet_lease" => (2, 2),
        "unroutable_backlog" => (1, 0),
        "proposed_backlog" => (1, 1),
        "overdue_target" => (1, 2),
        "build_over_budget" => (1, 3),
        _ => (1, 4), // footprint_outside
    }
}

#[derive(Debug, Serialize)]
struct Item {
    kind: &'static str,
    severity: u8,
    repo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task_id: Option<TaskId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    agent: Option<String>,
    /// When the situation began (see `docs/reference.md`).
    since: DateTime<Utc>,
    /// What it is about: a task's, backlog item's or target's title, or a one-line summary.
    title: String,
    detail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    backlog_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bytes: Option<u64>,
}

impl Item {
    fn new(kind: &'static str, repo: Option<&str>, since: DateTime<Utc>) -> Self {
        Self {
            kind,
            severity: rank(kind).0,
            repo: repo.map(str::to_owned),
            task_id: None,
            slug: None,
            agent: None,
            since,
            title: String::new(),
            detail: String::new(),
            backlog_id: None,
            target_id: None,
            bytes: None,
        }
    }

    fn task(kind: &'static str, t: &Task, since: DateTime<Utc>, detail: String) -> Self {
        Self {
            task_id: Some(t.id),
            slug: Some(t.slug.clone()),
            agent: Some(t.owner_agent.clone()),
            title: t.title.clone(),
            detail,
            ..Self::new(kind, Some(&t.repo), since)
        }
    }
}

#[derive(Deserialize)]
struct AttentionQuery {
    repo: Option<String>,
}

fn secs(n: u64) -> Duration {
    Duration::seconds(i64::try_from(n).unwrap_or(i64::MAX / 2))
}

/// `512 MiB`, `4.0 KiB`: as the dashboard writes sizes.
pub(crate) fn bytes_text(n: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
    let (mut x, mut i) = (n as f64, 0);
    while x >= 1024.0 && i < UNITS.len() - 1 {
        x /= 1024.0;
        i += 1;
    }
    if i > 0 && x < 10.0 {
        format!("{x:.1} {}", UNITS[i])
    } else {
        format!("{} {}", x.round(), UNITS[i])
    }
}

/// `{items: [{kind, severity, repo, task_id?, slug?, agent?, since, title, detail, ...}],
/// generated_at}`, worst first: severity, then the kind's place, then the oldest `since`.
async fn attention(
    State(st): State<AppState>,
    Query(q): Query<AttentionQuery>,
) -> ApiResult<Value> {
    let repo = q.repo.as_deref().filter(|r| !r.is_empty());
    let known: Vec<String> = st
        .store
        .list_repos()
        .await?
        .into_iter()
        .map(|r| r.name)
        .collect();
    let repos: Vec<String> = match repo {
        Some(r) if !known.iter().any(|k| k == r) => {
            return Err(AvError::NotFound(format!("repo {r}")).into());
        }
        Some(r) => vec![r.to_owned()],
        None => known,
    };
    let lim = st.attention;
    let now = Utc::now();
    let mut items = Vec::new();

    // the disk is daemon-wide, so it is listed whatever the repo; measured, not reclaimed, here
    if let Some(low) = crate::disk::below_floor(&st) {
        let mut it = Item::new("disk_low", None, low.since);
        it.bytes = Some(low.free);
        it.title = format!(
            "{} free on the workspace disk, below the {} floor",
            bytes_text(low.free),
            bytes_text(low.floor)
        );
        it.detail = "no new task starts until there is room (AV_DISK_MIN_FREE_GB); tasks already \
                     started keep working"
            .into();
        items.push(it);
    }

    // what the last enforcement of the build budget left over it; read, not measured, here
    for o in st.budget.over() {
        if !repos.contains(&o.repo) {
            continue;
        }
        let mut it = Item::new("build_over_budget", Some(&o.repo), o.since);
        it.bytes = Some(o.bytes);
        it.title = format!(
            "{} of build output in {}, over its {} budget",
            bytes_text(o.bytes),
            o.repo,
            bytes_text(o.budget_bytes)
        );
        it.detail =
            "only targets built within AV_BUILD_IDLE_SECS are left (AV_BUILD_CACHE_MAX_GB); \
                     they go once idle, or merge or abandon their tasks"
                .into();
        items.push(it);
    }

    // main red: the oldest commit of the red run, with the task that landed it
    for r in &repos {
        let Ok(h) = crate::checks::main_health(&st, r).await else {
            continue; // a repo still importing has no main to check
        };
        if h["state"] != "red" {
            continue;
        }
        let since = h["since_at"]
            .as_str()
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
            .unwrap_or(now);
        let short: String = h["since"].as_str().unwrap_or("").chars().take(8).collect();
        let mut it = Item::new("main_red", Some(r), since);
        it.task_id = serde_json::from_value(h["task_id"].clone()).ok();
        it.slug = h["task"].as_str().map(str::to_owned);
        it.title = format!(
            "{} exited {} on main",
            h["command"].as_str().unwrap_or("the check"),
            h["exit_code"]
        );
        it.detail = [
            Some(format!("since @{short}")),
            it.slug.as_ref().map(|s| format!("task {s}")),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(" · ");
        items.push(it);
    }

    // tasks: conflicted, waiting for a review or a merge, handed off, quiet
    let quiet: HashMap<TaskId, DateTime<Utc>> = st
        .store
        .abandoned_tasks(repo, st.lease_secs)
        .await?
        .into_iter()
        .map(|s| (s.task.id, s.since - secs(st.lease_secs)))
        .collect();
    for f in st.pg.tasks_in_flight(repo).await? {
        let t = &f.task;
        let owner = &t.owner_agent;
        let older = |since: DateTime<Utc>, limit: u64| now - since > secs(limit);
        let it = match t.status {
            TaskStatus::Conflicted => Some(Item::task(
                "conflicted",
                t,
                f.since,
                format!("{owner} must sync, resolve and resubmit"),
            )),
            TaskStatus::InReview => match f.approved_at {
                Some(at) => older(at, lim.approved_secs).then(|| {
                    Item::task(
                        "approved_not_merged",
                        t,
                        at,
                        format!("approved; {owner} can merge it"),
                    )
                }),
                None => older(f.since, lim.review_secs).then(|| {
                    Item::task(
                        "stale_review",
                        t,
                        f.since,
                        format!("submitted by {owner}, no verdict yet"),
                    )
                }),
            },
            TaskStatus::HandedOff => older(f.since, lim.handoff_secs)
                .then(|| Item::task("unclaimed_handoff", t, f.since, format!("from {owner}"))),
            TaskStatus::Open => quiet.get(&t.id).map(|wrote| {
                Item::task(
                    "quiet_lease",
                    t,
                    *wrote,
                    format!("{owner} has not written to it for a whole lease"),
                )
            }),
            _ => None,
        };
        items.extend(it);
    }

    // backlog items no agent can do: without profiles nothing is ranked, yet any agent takes
    // untagged work, so only what needs a skill, or what no profiled agent fits
    let profiles = st.store.list_profiles().await?;
    for e in st
        .store
        .list_backlog(repo, Some(BacklogState::Queued))
        .await?
    {
        let i = &e.item;
        let fits = !routing::rank(i, &profiles).is_empty();
        if fits || (i.needs.is_empty() && profiles.is_empty()) {
            continue;
        }
        let mut it = Item::new("unroutable_backlog", Some(&i.repo), i.created_at);
        it.backlog_id = Some(i.id);
        it.title = i.title.clone();
        it.detail = if i.needs.is_empty() {
            "no agent's profile fits it".into()
        } else {
            format!("needs {}", i.needs.join(", "))
        };
        items.push(it);
    }

    // proposals waiting for a human: low, but nobody else will move them
    for e in st
        .store
        .list_backlog(repo, Some(BacklogState::Proposed))
        .await?
    {
        let i = &e.item;
        let mut it = Item::new("proposed_backlog", Some(&i.repo), i.created_at);
        it.backlog_id = Some(i.id);
        it.agent = Some(e.posted_by.clone());
        it.title = i.title.clone();
        it.detail = match e.why.as_deref() {
            Some(why) => format!("proposed by {}: {why}", e.posted_by),
            None => format!("proposed by {}", e.posted_by),
        };
        items.push(it);
    }

    // targets past their due day: overdue from the end of it (UTC)
    let today = now.date_naive();
    for t in st
        .store
        .list_targets(repo, Some(TargetStatus::Active))
        .await?
    {
        let Some(due) = t.due.filter(|d| *d < today) else {
            continue;
        };
        let since = (due + Duration::days(1)).and_time(NaiveTime::MIN).and_utc();
        let mut it = Item::new("overdue_target", Some(&t.repo), since);
        it.target_id = Some(t.id);
        it.title = t.title.clone();
        it.detail = format!("due {due} · {}/{} merged", t.merged, t.items);
        items.push(it);
    }

    // footprints are daemon-wide and kept until it restarts: only recent big ones, and with a
    // repo named only its tasks' (or a session's with no task)
    let footprints: Vec<crate::disk::Footprint> = st
        .footprints
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .iter()
        .filter(|f| f.total_bytes >= lim.footprint_bytes && now - f.at < secs(lim.footprint_secs))
        .cloned()
        .collect();
    for f in footprints {
        let task = match f.task.as_deref().map(Uuid::parse_str) {
            Some(Ok(id)) => st.store.get_task(TaskId(id)).await.ok(),
            _ => None,
        };
        let task_repo = task.as_ref().map(|t| t.repo.clone());
        if let Some(r) = repo
            && f.task.is_some()
            && task_repo.as_deref() != Some(r)
        {
            continue;
        }
        let mut it = Item::new("footprint_outside", task_repo.as_deref(), f.at);
        it.task_id = task.as_ref().map(|t| t.id);
        it.slug = f.slug.clone();
        it.agent = f.agent.clone();
        it.bytes = Some(f.total_bytes);
        it.title = format!(
            "{} left {} outside any workspace",
            f.agent.as_deref().unwrap_or("a session"),
            bytes_text(f.total_bytes)
        );
        it.detail = [
            f.slug.as_ref().map(|s| format!("task {s}")),
            f.added.first().map(|a| a.path.clone()),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(" · ");
        items.push(it);
    }

    items.sort_by(|a, b| {
        let (sa, pa) = rank(a.kind);
        let (sb, pb) = rank(b.kind);
        sb.cmp(&sa).then(pa.cmp(&pb)).then(a.since.cmp(&b.since))
    });
    Ok(Json(json!({ "items": items, "generated_at": now })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizes_read_as_the_dashboard_writes_them() {
        assert_eq!(bytes_text(512), "512 B");
        assert_eq!(bytes_text(4096), "4.0 KiB");
        assert_eq!(bytes_text(512 * 1024 * 1024), "512 MiB");
        assert_eq!(bytes_text(3 * 1024 * 1024 * 1024 / 2), "1.5 GiB");
    }

    #[test]
    fn kinds_rank_worst_first() {
        let order = [
            "disk_low",
            "main_red",
            "conflicted",
            "stale_review",
            "unclaimed_handoff",
            "quiet_lease",
            "unroutable_backlog",
            "proposed_backlog",
            "overdue_target",
            "build_over_budget",
            "footprint_outside",
        ];
        let ranks: Vec<(u8, u8)> = order.iter().map(|k| rank(k)).collect();
        let mut sorted = ranks.clone();
        sorted.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
        assert_eq!(ranks, sorted);
        assert_eq!(rank("approved_not_merged"), rank("stale_review"));
    }
}
