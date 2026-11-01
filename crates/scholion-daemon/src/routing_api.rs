//! Plan 8: agent profiles, the backlog and `next` (HTTP). The rules that pick an item for an
//! agent are `scholion_core::routing`; this module only moves data and starts or claims tasks.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use axum::extract::{Path, Query, State};
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use chrono::Utc;
use scholion_core::routing::{self, BacklogItem, BacklogState, NewBacklogItem, Profile};
use scholion_core::*;
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use crate::AppState;
use crate::api::{create_started, first_pack_request, pack_for_task};
use crate::error::ApiResult;
use crate::flow::{claim_as, resume_as};

pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .route("/v1/agents", get(list_agents))
        .route("/v1/agents/{name}", put(put_agent))
        .route("/v1/backlog", post(post_backlog).get(list_backlog))
        .route("/v1/backlog/next", post(next))
        .route("/v1/backlog/{id}", delete(drop_backlog))
}

async fn list_agents(State(st): State<AppState>) -> ApiResult<Value> {
    Ok(Json(json!({ "agents": st.store.list_profiles().await? })))
}

#[derive(Deserialize)]
struct AgentBody {
    #[serde(default)]
    best: Vec<String>,
    #[serde(default)]
    can: Vec<String>,
    #[serde(default = "default_cost_rank")]
    cost_rank: i32,
    #[serde(default)]
    aliases: Vec<String>,
    #[serde(default)]
    max_open: Option<i32>,
}

fn default_cost_rank() -> i32 {
    5
}

async fn put_agent(
    State(st): State<AppState>,
    Path(name): Path<String>,
    Json(b): Json<AgentBody>,
) -> ApiResult<Value> {
    let name = name.trim().to_owned();
    st.store
        .put_profile(&Profile {
            name: name.clone(),
            best: b.best,
            can: b.can,
            cost_rank: b.cost_rank,
            aliases: b.aliases,
            max_open: b.max_open,
        })
        .await?;
    let stored = st
        .store
        .list_profiles()
        .await?
        .into_iter()
        .find(|p| p.name == name);
    Ok(Json(json!({ "agent": stored })))
}

#[derive(Deserialize)]
struct PostBacklog {
    repo: String,
    title: String,
    goal: String,
    #[serde(default)]
    needs: Vec<String>,
    prefer: Option<String>,
    #[serde(default)]
    priority: i32,
    reserve_secs: Option<i64>,
    posted_by: Option<String>,
}

async fn post_backlog(
    State(st): State<AppState>,
    Json(b): Json<PostBacklog>,
) -> ApiResult<routing::BacklogEntry> {
    let entry = st
        .store
        .post_backlog(NewBacklogItem {
            repo: b.repo,
            title: b.title,
            goal: b.goal,
            needs: b.needs,
            prefer: b.prefer,
            priority: b.priority,
            reserve_secs: b.reserve_secs.unwrap_or(routing::DEFAULT_RESERVE_SECS),
            posted_by: b.posted_by.unwrap_or_else(|| "user".into()),
        })
        .await?;
    Ok(Json(entry))
}

#[derive(Deserialize)]
struct ListBacklog {
    repo: Option<String>,
    state: Option<String>,
}

async fn list_backlog(
    State(st): State<AppState>,
    Query(q): Query<ListBacklog>,
) -> ApiResult<Value> {
    let state = q
        .state
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(str::parse::<BacklogState>)
        .transpose()?;
    let items = st.store.list_backlog(q.repo.as_deref(), state).await?;
    // the agent each item is reserved for first, so a client need not know the ranking rules
    let profiles = st.store.list_profiles().await?;
    let items: Vec<Value> = items
        .iter()
        .map(|e| {
            let mut v = json!(e);
            v["best_agent"] = json!(routing::rank(&e.item, &profiles).first());
            v
        })
        .collect();
    Ok(Json(json!({ "items": items })))
}

async fn drop_backlog(State(st): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Value> {
    st.store.drop_backlog(id).await?;
    Ok(Json(json!({ "id": id, "state": "dropped" })))
}

#[derive(Deserialize)]
struct NextBody {
    repo: Option<String>,
    agent: String,
    session: String,
    budget: Option<u32>,
    family: Option<AgentFamily>,
    /// Only say what `next` would give (Plan 9): nothing is taken, claimed or reserved.
    #[serde(default)]
    dry_run: bool,
    /// With nothing to give, wait up to this many seconds for something (at most `max_wait`).
    wait_secs: Option<u64>,
}

/// The longest `next` holds a call open waiting for work. Under the 60 s that many HTTP clients
/// allow a call; MCP clients' own limits were not measured (Plan 9 Task 7 Step 1).
pub const MAX_WAIT_SECS: u64 = 50;

/// How often a waiting `next` looks again.
const WAIT_STEP: Duration = Duration::from_secs(2);

/// How long a review offered by `next` stays with the agent it was offered to.
pub const REVIEW_RESERVE_SECS: u64 = 600;

/// Reviews offered by `next`: the agent each was offered to, and until when. In memory only: a
/// restart forgets them, which at worst offers a review twice.
pub(crate) type Reviews = Arc<Mutex<HashMap<TaskId, (String, Instant)>>>;

const REVIEW_TODO: &str = "Review it: review_task, run its tests in its workspace and validate, \
                           then post_review. If you approve, merge it.";
const MERGE_TODO: &str = "It is approved: merge it.";

/// What `next` may give an agent, best first.
#[derive(Debug)]
pub(crate) enum Choice {
    /// Another agent's task in review; `approved` at its current head (then it only needs a merge).
    Review { task: Task, approved: bool },
    /// A handed-off task to claim; the item stands for it (`id` is the task id).
    Handoff(BacklogItem),
    /// An open task whose owner went quiet (its lease ran out): claimed, or resumed by its owner.
    Abandoned(BacklogItem),
    /// A queued backlog item to start.
    Backlog(BacklogItem),
}

/// Everything `agent` may be given now, best first (reviews and merges, then handoffs and backlog
/// items in the order of `routing::order_for`), and how many items are queued.
pub(crate) async fn choose(
    st: &AppState,
    agent: &str,
    repo: Option<&str>,
    profiles: &[Profile],
) -> Result<(Vec<Choice>, usize)> {
    let mut choices = review_choices(st, agent, repo, profiles).await?;

    let queued = st
        .store
        .list_backlog(repo, Some(BacklogState::Queued))
        .await?;
    // Tasks waiting for a new agent are candidates under their own ids: every handoff (its
    // window starts at the handoff; the agent that handed it off neither takes nor holds it),
    // and every open task whose owner went quiet (no window: it already waited a whole lease, so
    // whoever fits and asks first gets it, its restarted owner included).
    let mut items: Vec<BacklogItem> = queued.iter().map(|e| e.item.clone()).collect();
    let mut kind: HashMap<Uuid, bool> = HashMap::new(); // task id -> abandoned?
    let mut excluded: HashMap<Uuid, String> = HashMap::new();
    for h in st.store.handed_off_tasks(repo).await? {
        // Tags given at the handoff hold. Without them the task keeps its item's tags, unless only
        // the agent that handed it off has those: then any other agent may take it.
        let needs = if h.needs.is_empty() {
            let inherited = task_needs(st, &h.task).await?;
            if others_can(&h.task.owner_agent, &inherited, profiles) {
                inherited
            } else {
                Vec::new()
            }
        } else {
            h.needs.clone()
        };
        kind.insert(h.task.id.0, false);
        excluded.insert(h.task.id.0, h.task.owner_agent.clone());
        items.push(task_item(
            &h.task,
            needs,
            h.prefer.clone(),
            h.since,
            routing::DEFAULT_RESERVE_SECS,
        ));
    }
    for a in st.store.abandoned_tasks(repo, st.lease_secs).await? {
        let needs = task_needs(st, &a.task).await?;
        kind.insert(a.task.id.0, true);
        items.push(task_item(&a.task, needs, None, a.since, 0));
    }
    for item in routing::order_for_excluding(agent, &items, profiles, Utc::now(), &excluded) {
        choices.push(match kind.get(&item.id) {
            Some(false) => Choice::Handoff(item.clone()),
            Some(true) => Choice::Abandoned(item.clone()),
            None => Choice::Backlog(item.clone()),
        });
    }
    Ok((choices, queued.len()))
}

/// What a dry run says about a choice: its kind, the task or backlog item, its repo and title.
fn would(c: &Choice) -> Value {
    let (from, item) = match c {
        Choice::Review { task, .. } => {
            return json!({ "from": "review", "task_id": task.id, "repo": task.repo,
                           "title": task.title });
        }
        Choice::Handoff(i) => ("handoff", i),
        Choice::Abandoned(i) => ("abandoned", i),
        Choice::Backlog(i) => {
            return json!({ "from": "backlog", "backlog_id": i.id, "repo": i.repo,
                           "title": i.title });
        }
    };
    json!({ "from": from, "task_id": item.id, "repo": item.repo, "title": item.title })
}

/// An existing task as a candidate for `routing::order_for` (its `id` is the task id).
fn task_item(
    task: &Task,
    needs: Vec<String>,
    prefer: Option<String>,
    since: chrono::DateTime<Utc>,
    reserve_secs: i64,
) -> BacklogItem {
    BacklogItem {
        id: task.id.0,
        repo: task.repo.clone(),
        title: task.title.clone(),
        goal: task.goal.clone(),
        needs,
        prefer,
        priority: 0,
        reserve_secs,
        created_at: since,
    }
}

/// The tags a task's agent must have: those of the backlog item it was made from, if any.
async fn task_needs(st: &AppState, task: &Task) -> Result<Vec<String>> {
    Ok(st
        .store
        .list_backlog(Some(&task.repo), Some(BacklogState::Taken))
        .await?
        .into_iter()
        .find(|e| e.task_id == Some(task.id))
        .map(|e| e.item.needs)
        .unwrap_or_default())
}

/// Whether some agent other than `owner` can do work that needs `needs`.
fn others_can(owner: &str, needs: &[String], profiles: &[Profile]) -> bool {
    profiles.iter().any(|p| p.name != owner && p.can_do(needs))
}

/// Whether `agent` (its profile, if any) can do work that needs `needs`. With no profile, only
/// untagged work.
fn fits(agent: &str, needs: &[String], profiles: &[Profile]) -> bool {
    match profiles.iter().find(|p| p.name == agent) {
        Some(p) => p.can_do(needs),
        None => needs.is_empty(),
    }
}

/// Whether a live offer of `task` names another agent than `agent`.
fn reserved_for_other(st: &AppState, task: TaskId, agent: &str) -> bool {
    let reviews = st.reviews.lock().unwrap_or_else(|e| e.into_inner());
    reviews
        .get(&task)
        .is_some_and(|(who, until)| who != agent && *until > Instant::now())
}

/// Tasks in review that `agent` may review or merge, oldest first: not its own, tags it can do
/// (or tags no other agent can do), not offered to another agent right now.
async fn review_choices(
    st: &AppState,
    agent: &str,
    repo: Option<&str>,
    profiles: &[Profile],
) -> Result<Vec<Choice>> {
    let mut rows = st
        .pg
        .list_tasks(repo, Some(TaskStatus::InReview), 500)
        .await?;
    rows.sort_by(|a, b| {
        a.created_at
            .cmp(&b.created_at)
            .then(a.task.id.0.cmp(&b.task.id.0))
    });
    let mut out = Vec::new();
    for row in rows {
        let task = row.task;
        if task.owner_agent == agent || reserved_for_other(st, task.id, agent) {
            continue;
        }
        // An agent reviews work it could have done; but when no agent other than the owner could,
        // any other agent reviews it rather than nobody.
        let needs = task_needs(st, &task).await?;
        if others_can(&task.owner_agent, &needs, profiles) && !fits(agent, &needs, profiles) {
            continue;
        }
        let approved = matches!(
            st.store.latest_verdict(task.id, &task.head_change).await?,
            Some((_, Verdict::Approve))
        );
        out.push(Choice::Review { task, approved });
    }
    Ok(out)
}

/// Offers `task` to `agent` for review: reserves it, unless another agent got it meanwhile.
fn offer_review(st: &AppState, task: Task, approved: bool, agent: &str) -> Result<Value> {
    {
        let mut reviews = st.reviews.lock().unwrap_or_else(|e| e.into_inner());
        let now = Instant::now();
        reviews.retain(|_, (_, until)| *until > now);
        if reviews.get(&task.id).is_some_and(|(who, _)| who != agent) {
            return Err(AvError::IllegalState("offered to another agent".into()));
        }
        reviews.insert(
            task.id,
            (
                agent.to_owned(),
                now + Duration::from_secs(REVIEW_RESERVE_SECS),
            ),
        );
    }
    let todo = if approved { MERGE_TODO } else { REVIEW_TODO };
    Ok(json!({ "from": "review", "task": task, "approved": approved, "next": todo }))
}

/// The one item that fits the caller, made into its task: a queued backlog item is started (a
/// workspace and a first pack, like `start_task`), a handed-off task that names tags is claimed
/// (the handoff bundle, like `claim_task`). With nothing to give it returns `{"task": null,
/// "queued": N}`: N queued items, none of which the caller may take now.
async fn next(State(st): State<AppState>, Json(b): Json<NextBody>) -> ApiResult<Value> {
    let agent = b.agent.trim().to_owned();
    if agent.is_empty() {
        return Err(AvError::Invalid("agent must not be empty".into()).into());
    }
    let profiles = st.store.list_profiles().await?;
    let agent = routing::resolve(&profiles, &agent)
        .map(|p| p.name.clone())
        .unwrap_or(agent);

    let deadline = Instant::now() + Duration::from_secs(b.wait_secs.unwrap_or(0)).min(st.max_wait);
    loop {
        // profiles are read again on each look: one may change while an agent waits
        let profiles = st.store.list_profiles().await?;
        let (choices, queued) = choose(&st, &agent, b.repo.as_deref(), &profiles).await?;
        let (choices, full) = within_limit(&st, &agent, &profiles, choices).await?;
        let nothing = |key: &str| {
            let mut v = json!({ key: null, "queued": queued });
            if let Some((open, max)) = full {
                v["at_capacity"] = json!({ "open": open, "max": max });
            }
            v
        };
        if b.dry_run {
            return Ok(Json(match choices.first() {
                Some(c) => json!({ "would": would(c) }),
                None => nothing("would"),
            }));
        }
        if let Some(v) = act(&st, &agent, &b, choices).await? {
            return Ok(Json(v));
        }
        // nothing to give: wait for something (the call is dropped if the client goes away)
        let now = Instant::now();
        if now >= deadline {
            return Ok(Json(nothing("task")));
        }
        tokio::time::sleep(WAIT_STEP.min(deadline - now)).await;
    }
}

/// An agent CLI that runs subagents is a team and may hold several open tasks, up to its
/// profile's `max_open`. At the limit it gets only what finishes work: reviews and merges of
/// others' tasks, and its own abandoned tasks back. Returns the choices left and, at the limit,
/// `(open, max)`.
async fn within_limit(
    st: &AppState,
    agent: &str,
    profiles: &[Profile],
    choices: Vec<Choice>,
) -> Result<(Vec<Choice>, Option<(i64, i32)>)> {
    let Some(max) = profiles
        .iter()
        .find(|p| p.name == agent)
        .and_then(|p| p.max_open)
    else {
        return Ok((choices, None));
    };
    let open = st.store.count_open_tasks(agent).await?;
    if open < i64::from(max) {
        return Ok((choices, None));
    }
    let mut kept = Vec::new();
    for c in choices {
        let keep = match &c {
            Choice::Review { .. } => true,
            Choice::Abandoned(item) => {
                st.store.get_task(TaskId(item.id)).await?.owner_agent == agent
            }
            Choice::Handoff(_) | Choice::Backlog(_) => false,
        };
        if keep {
            kept.push(c);
        }
    }
    Ok((kept, Some((open, max))))
}

/// Acts on the first choice that works: offers a review, claims or resumes a task, or starts a
/// backlog item. A choice that cannot be acted on is skipped and the next one tried, so one bad
/// item (a repo that is gone) cannot block the rest; the first error is returned only when
/// nothing else worked. `None`: there was nothing to act on.
async fn act(
    st: &AppState,
    agent: &str,
    b: &NextBody,
    choices: Vec<Choice>,
) -> Result<Option<Value>> {
    let mut first_err: Option<AvError> = None;
    for choice in choices {
        let made = match choice {
            Choice::Review { task, approved } => offer_review(st, task, approved, agent),
            Choice::Handoff(item) => claim_as(st, TaskId(item.id), agent, b.budget, b.family)
                .await
                .map(|mut v| {
                    v["from"] = json!("handoff");
                    v
                }),
            Choice::Abandoned(item) => {
                let id = TaskId(item.id);
                let own = st.store.get_task(id).await?.owner_agent == agent;
                let taken = if own {
                    resume_as(st, id, agent, b.budget, b.family).await
                } else {
                    claim_as(st, id, agent, b.budget, b.family).await
                };
                taken.map(|mut v| {
                    v["from"] = json!("abandoned");
                    v
                })
            }
            Choice::Backlog(item) => start_from_backlog(st, &item, agent, b).await,
        };
        match made {
            Ok(v) => return Ok(Some(v)),
            // someone else got there first
            Err(AvError::IllegalState(_)) => {}
            Err(e) => {
                first_err.get_or_insert(e);
            }
        }
    }
    match first_err {
        Some(e) => Err(e),
        None => Ok(None),
    }
}

/// Takes `item` for `agent` and starts its task. Waits for a start turn first, like `start_task`,
/// and makes the task on a task of its own, so a client that leaves cannot stop it half done.
async fn start_from_backlog(
    st: &AppState,
    item: &BacklogItem,
    agent: &str,
    b: &NextBody,
) -> Result<Value> {
    let turn = st
        .starts
        .clone()
        .acquire_owned()
        .await
        .map_err(|e| AvError::Store(format!("start queue closed: {e}")))?;
    if !st.store.take_backlog(item.id, agent).await? {
        return Err(AvError::IllegalState("the item was taken".into()));
    }
    let task = {
        let st = st.clone();
        let (item, agent) = (item.clone(), agent.to_owned());
        tokio::spawn(async move {
            let _turn = turn;
            match create_started(&st, item.repo, item.title, item.goal, agent, PackMode::Full).await
            {
                Ok(task) => {
                    st.store.link_backlog_task(item.id, task.id).await?;
                    Ok(task)
                }
                Err(e) => {
                    // no task exists, so the item can be offered again
                    let _ = st.store.untake_backlog(item.id).await;
                    Err(e)
                }
            }
        })
        .await
        .map_err(|e| AvError::Store(format!("task start stopped: {e}")))??
    };
    let req = first_pack_request(task.id, b.budget, b.session.clone());
    let (task, pack) = pack_for_task(st, task.id, req).await?;
    Ok(json!({ "task": task, "pack": pack, "from": "backlog", "backlog_id": item.id }))
}
