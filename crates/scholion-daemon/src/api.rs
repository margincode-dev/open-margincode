use std::path::PathBuf;

use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::routing::{get, post};
use axum::{Json, Router};
use scholion_core::*;
use scholion_pack::{CodeDetail, Pack, PackRequest};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use uuid::Uuid;

use crate::AppState;
use crate::error::ApiResult;
use crate::flow::fence;
use crate::indexing::{index_change, reindex_changed};

pub(crate) const DEFAULT_BUDGET: u32 = 4000;

pub fn router(state: AppState) -> Router {
    let open = Router::new().route("/healthz", get(|| async { "ok" }));
    let api = routes()
        .route(
            "/mcp",
            post(crate::mcp::post)
                .get(crate::mcp::get)
                .delete(crate::mcp::delete)
                .layer(axum::extract::DefaultBodyLimit::max(
                    crate::mcp::MAX_REQUEST_BYTES,
                )),
        )
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            crate::auth::require_token,
        ));
    open.merge(api).with_state(state)
}

/// Every `/v1` route, without the token check: `router` adds it, and the MCP tools call these
/// in-process after `/mcp` itself passed it.
pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .route("/v1/repos", post(add_repo))
        .route("/v1/reindex", post(crate::reindex::reindex))
        .route("/v1/tasks", post(start_task))
        .route("/v1/tasks/{id}", get(get_task))
        .route("/v1/tasks/{id}/checkpoint", post(checkpoint))
        .route("/v1/tasks/{id}/submit", post(crate::flow::submit))
        .route("/v1/tasks/{id}/validate", post(crate::flow::validate))
        .route("/v1/tasks/{id}/review", post(crate::flow::review))
        .route("/v1/tasks/{id}/merge", post(crate::flow::merge))
        .route("/v1/tasks/{id}/sync", post(crate::flow::sync))
        .route("/v1/tasks/{id}/undo", post(crate::flow::undo))
        .route("/v1/tasks/{id}/revert", post(crate::flow::revert))
        .route("/v1/tasks/{id}/handoff", post(crate::flow::handoff))
        .route("/v1/tasks/{id}/claim", post(crate::flow::claim))
        .route("/v1/tasks/{id}/bundle", get(bundle))
        .route("/v1/tasks/{id}/abandon", post(crate::flow::abandon))
        .route("/v1/repos/{name}/search", get(search))
        .route("/v1/repos/{name}/log", get(log))
        .route("/v1/repos/{name}/files/{*path}", get(read_file_at))
        .route("/v1/repos/{name}/tree", get(tree))
        .route("/v1/repos/{name}/diff", get(diff))
        .route("/v1/tasks/{id}/records", post(add_record))
        .route("/v1/tasks/{id}/context", get(context))
        .route("/v1/tasks/{id}/items/{item_id}", get(expand))
        .route("/v1/receipts/{id}", get(get_receipt))
        .route(
            "/v1/repos/{name}/conventions",
            post(add_convention).get(list_conventions),
        )
        .route("/v1/events", get(events))
        .route(
            "/v1/repos/{name}/conventions/{id}",
            axum::routing::delete(remove_convention),
        )
        .route("/v1/tasks/{id}/checks", get(task_checks))
        .route("/v1/repos/{name}/checks", post(crate::checks::post_check))
        .route("/v1/repos/{name}/health", get(crate::checks::health))
        .route("/v1/repos/{name}/why", get(crate::derived::why))
        .route(
            "/v1/repos/{name}/settings",
            get(get_settings).put(put_settings),
        )
        .route(
            "/v1/repos/{name}/backup",
            get(crate::backup::get_backup).post(crate::backup::set_backup),
        )
        .route(
            "/v1/repos/{name}/backup/push",
            post(crate::backup::push_backup),
        )
        .route("/v1/maintenance/prune", post(prune))
        .route("/v1/repos/{name}/outbox", get(outbox))
}

/// Sessions idle this long lose their ledger in `prune`.
const PRUNE_IDLE: std::time::Duration = std::time::Duration::from_secs(30 * 86_400);

/// Deletes the ledgers of sessions idle for 30 days and parse-cache entries no scope uses.
/// Embedding rows still queued for a repo, and whether a semantic layer drains them: the benchmark
/// waits for zero before a session, so packs can use semantic search from the start.
async fn outbox(State(st): State<AppState>, Path(name): Path<String>) -> ApiResult<Value> {
    let pending = st.pg.outbox_count(Some(&name)).await?;
    Ok(Json(
        json!({ "pending": pending, "semantic": st.semantic.is_some() }),
    ))
}

async fn prune(State(st): State<AppState>) -> ApiResult<Value> {
    let sessions = st.pg.prune_sessions(PRUNE_IDLE).await?;
    let parse_cache = st.index.prune_parse_cache().await?;
    Ok(Json(
        json!({ "sessions": sessions, "parse_cache": parse_cache }),
    ))
}

fn clip_chars(s: &str, max: usize) -> String {
    s.chars().take(max).collect()
}

#[derive(Deserialize)]
struct AddRepo {
    name: String,
    git_source: PathBuf,
}

async fn add_repo(State(st): State<AppState>, Json(req): Json<AddRepo>) -> ApiResult<Value> {
    let main = st.vcs.init_repo(&req.name, &req.git_source).await?;
    let stats = index_change(&st, &req.name, "main", &main).await?;
    Ok(Json(
        json!({ "name": req.name, "main_change": main, "indexed_files": stats.parsed + stats.cached }),
    ))
}

#[derive(Deserialize)]
struct StartTask {
    repo: String,
    title: String,
    goal: String,
    agent: String,
    session: String,
    budget: Option<u32>,
    #[serde(default)]
    pack_mode: PackMode,
}

#[derive(Serialize)]
struct Started {
    task: Task,
    pack: Pack,
}

async fn start_task(State(st): State<AppState>, Json(req): Json<StartTask>) -> ApiResult<Started> {
    let task = st
        .store
        .create_task(NewTask {
            repo: req.repo.clone(),
            title: req.title,
            goal: req.goal.clone(),
            owner_agent: req.agent.clone(),
        })
        .await?;
    if req.pack_mode != PackMode::Full {
        st.store.set_pack_mode(task.id, req.pack_mode).await?;
    }
    match prepare_workspace(&st, &task).await {
        Ok(()) => {}
        Err(e) => {
            // do not leave an open task without a workspace behind
            let _ = st.store.set_status(task.id, TaskStatus::Abandoned).await;
            return Err(e.into());
        }
    }
    let summary = clip_chars(req.goal.lines().next().unwrap_or("").trim(), 200);
    // the body only carries what the summary does not
    let body = if req.goal.trim() == summary {
        String::new()
    } else {
        req.goal
    };
    st.store
        .add_record(NewRecord {
            task_id: task.id,
            kind: RecordKind::Intent,
            summary,
            body,
            change_id: None,
            supersedes: None,
            author_agent: req.agent,
            anchors: vec![],
        })
        .await?;
    st.store
        .renew_lease(task.id, task.generation, st.lease_secs)
        .await?;
    let task = st.store.get_task(task.id).await?;
    let pack = st
        .packer
        .build(PackRequest {
            task_id: task.id,
            budget_tokens: req.budget.unwrap_or(DEFAULT_BUDGET),
            focus: None,
            session: Some(req.session),
            family: AgentFamily::default(),
            code: CodeDetail::Full,
            map: true,
            max_code: None,
        })
        .await?;
    Ok(Json(Started { task, pack }))
}

async fn prepare_workspace(st: &AppState, task: &Task) -> Result<()> {
    let info = st.vcs.create_workspace(&task.repo, &task.slug).await?;
    st.store
        .set_workspace(task.id, &st.to_host_path(&info.path), &info.base_change)
        .await?;
    // A task starts at trunk, which `main` is normally indexed at: copy those rows in Postgres
    // instead of reading and indexing every file again. Otherwise index from the VCS.
    let copied = st
        .index
        .clone_scope(&task.repo, "main", &task.slug, &info.base_change)
        .await?;
    if !copied {
        index_change(st, &task.repo, &task.slug, &info.base_change).await?;
    }
    Ok(())
}

fn task_id(id: Uuid) -> TaskId {
    TaskId(id)
}

async fn changed_for(st: &AppState, task: &Task) -> Result<Vec<String>> {
    if task.base_change.0.is_empty() || task.head_change.0.is_empty() {
        return Ok(vec![]);
    }
    st.vcs
        .changed_paths(&task.repo, &task.base_change, &task.head_change)
        .await
}

async fn get_task(State(st): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Value> {
    let task = st.store.get_task(task_id(id)).await?;
    let record_count = st.store.task_records(task.id).await?.len();
    let changed = changed_for(&st, &task).await?;
    let merge = st.store.merge_state(task.id).await?;
    let verdict = st.store.latest_verdict(task.id, &task.head_change).await?;
    let backup = st.pg.backup_state(&task.repo).await?;
    Ok(Json(json!({
        "task": task, "record_count": record_count, "changed": changed, "merge": merge,
        "backup": backup,
        "verdict": verdict.map(|(reviewer, v)| json!({"reviewer": reviewer, "verdict": v.as_str()})),
    })))
}

async fn checkpoint(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
) -> ApiResult<Value> {
    let _guard = st.lock_task(task_id(id)).await;
    let task = st.store.get_task(task_id(id)).await?;
    fence(&st, &task, &headers).await?;
    let (head, changed, warnings) = snapshot_and_index(&st, &task).await?;
    Ok(Json(
        json!({ "head_change": head, "changed": changed, "warnings": warnings }),
    ))
}

/// Snapshot the workspace, re-index what changed, and record the new head.
/// Returns (head, changed paths, indexing warnings). The caller holds `AppState::lock_task`.
pub(crate) async fn snapshot_and_index(
    st: &AppState,
    task: &Task,
) -> Result<(ChangeId, Vec<String>, Vec<String>)> {
    let head = st.vcs.snapshot(&task.repo, &task.slug).await?;
    let changed = st
        .vcs
        .changed_paths(&task.repo, &task.base_change, &head)
        .await?;
    let warnings = reindex_changed(st, &task.repo, &task.slug, &head, &changed).await?;
    st.store.set_head(task.id, &head, &changed).await?;
    Ok((head, changed, warnings))
}

#[derive(Deserialize)]
struct RecordBody {
    kind: RecordKind,
    summary: String,
    body: Option<String>,
    anchors: Option<Vec<Anchor>>,
    agent: String,
    change_id: Option<ChangeId>,
    supersedes: Option<RecordId>,
    /// `rcp:…` of the pack the author worked from (Plan 2b).
    receipt: Option<String>,
}

async fn add_record(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Json(b): Json<RecordBody>,
) -> ApiResult<Value> {
    let _guard = st.lock_task(task_id(id)).await;
    let task = st.store.get_task(task_id(id)).await?;
    fence(&st, &task, &headers).await?;
    let anchors = b.anchors.unwrap_or_default();
    // Remember what the anchored code looked like, so later packs can flag the claim when it changes.
    let hashes = st.packer.anchor_hashes(task.id, &anchors).await?;
    let record = st
        .store
        .add_record_with(
            NewRecord {
                task_id: task.id,
                kind: b.kind,
                summary: b.summary,
                body: b.body.unwrap_or_default(),
                change_id: b.change_id,
                supersedes: b.supersedes,
                author_agent: b.agent,
                anchors,
            },
            &hashes,
            b.receipt.as_deref(),
        )
        .await?;
    let receipt = match &b.receipt {
        Some(r) => st.store.get_receipt(r).await?["id"].clone(),
        None => Value::Null,
    };
    let mut out = serde_json::to_value(&record).map_err(|e| AvError::Store(e.to_string()))?;
    out["receipt"] = receipt;
    Ok(Json(out))
}

#[derive(Deserialize)]
struct ConventionBody {
    scope_glob: String,
    text: String,
    #[serde(default)]
    required: bool,
    /// A check command (required conventions only): approval needs it to pass at the head.
    #[serde(default)]
    check: Option<String>,
}

async fn add_convention(
    State(st): State<AppState>,
    Path(repo): Path<String>,
    Json(b): Json<ConventionBody>,
) -> ApiResult<Value> {
    let id = st
        .store
        .add_convention(&repo, &b.scope_glob, &b.text)
        .await?;
    let check = b.check.filter(|c| !c.trim().is_empty());
    if check.is_some() && !b.required {
        return Err(AvError::Invalid(
            "a check command needs a required convention (--required)".into(),
        )
        .into());
    }
    if b.required {
        st.store.set_convention_required(id, true).await?;
    }
    if let Some(c) = &check {
        st.store.set_convention_check(id, Some(c)).await?;
    }
    Ok(Json(
        json!({ "id": id.0, "repo": repo, "required": b.required, "check": check }),
    ))
}

async fn remove_convention(
    State(st): State<AppState>,
    Path((repo, id)): Path<(String, i64)>,
) -> ApiResult<Value> {
    st.store.remove_convention(&repo, ConventionId(id)).await?;
    Ok(Json(json!({"removed": id})))
}

/// The checks `av validate` must run for this task: required conventions with a check command
/// whose glob matches a changed path.
async fn task_checks(State(st): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Value> {
    let task = st.store.get_task(task_id(id)).await?;
    let checks: Vec<Value> = crate::flow::applicable_checks(&st, &task)
        .await?
        .into_iter()
        .map(|c| {
            json!({"convention_id": c.id.0, "command": c.check_command,
                   "scope_glob": c.scope_glob, "text": c.text})
        })
        .collect();
    Ok(Json(json!({"checks": checks})))
}

async fn get_receipt(State(st): State<AppState>, Path(id): Path<String>) -> ApiResult<Value> {
    Ok(Json(st.store.get_receipt(&id).await?))
}

#[derive(Deserialize)]
struct ContextQuery {
    budget: Option<u32>,
    focus: Option<String>,
    session: Option<String>,
    family: Option<AgentFamily>,
    /// `full` (default) or `signatures`: code items without their bodies.
    code: Option<CodeDetail>,
    /// `false`: no Map section (file skeletons, repo layout).
    map: Option<bool>,
    /// At most this many code items, the best scored.
    max_code: Option<usize>,
}

async fn context(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
    Query(q): Query<ContextQuery>,
) -> ApiResult<Pack> {
    let pack = st
        .packer
        .build(PackRequest {
            task_id: task_id(id),
            budget_tokens: q.budget.unwrap_or(DEFAULT_BUDGET),
            focus: q.focus.filter(|f| !f.trim().is_empty()),
            session: q.session.filter(|s| !s.trim().is_empty()),
            family: q.family.unwrap_or_default(),
            code: q.code.unwrap_or_default(),
            map: q.map.unwrap_or(true),
            max_code: q.max_code,
        })
        .await?;
    Ok(Json(with_main_health(&st, task_id(id), pack).await))
}

/// Puts a warning at the top of a pack while main's post-merge check is red, so no agent builds
/// on a broken main without knowing (a few tokens over budget, only then).
async fn with_main_health(st: &AppState, task: TaskId, mut pack: Pack) -> Pack {
    let Ok(t) = st.store.get_task(task).await else {
        return pack;
    };
    let Ok(h) = crate::checks::main_health(st, &t.repo).await else {
        return pack;
    };
    if h["state"] == "red" {
        let since = h["since"].as_str().unwrap_or_default();
        pack.markdown = format!(
            "> **main is red** since {} (task {}): `{}` exit {}. Check `av health` and `av log` \
             before building on it.\n\n{}",
            &since[..since.len().min(12)],
            h["task"].as_str().unwrap_or("?"),
            h["command"].as_str().unwrap_or_default(),
            h["exit_code"],
            pack.markdown
        );
    }
    pack
}

/// Most records `search` returns.
const MAX_SEARCH: usize = 20;
/// Most trunk commits `log` returns.
const MAX_LOG: usize = 200;

#[derive(Deserialize)]
struct SearchQuery {
    #[serde(default)]
    q: String,
    limit: Option<usize>,
}

/// Full-text search over the repo's records (superseded ones hidden), at most 20.
async fn search(
    State(st): State<AppState>,
    Path(repo): Path<String>,
    Query(q): Query<SearchQuery>,
) -> ApiResult<Value> {
    let limit = q.limit.unwrap_or(10).clamp(1, MAX_SEARCH);
    let records = st.store.search_records_fts(&repo, &q.q, limit).await?;
    Ok(Json(json!({ "records": records })))
}

#[derive(Deserialize)]
struct LogQuery {
    limit: Option<usize>,
    /// Only the commits that changed this path.
    path: Option<String>,
}

/// Commits of main's history a path log looks through.
const MAX_PATH_LOG_SCAN: usize = 2000;

/// Main's history, newest first: what merged, and the merge commit an undo would restore from.
/// With `?path=`, only the commits that changed that path.
async fn log(
    State(st): State<AppState>,
    Path(repo): Path<String>,
    Query(q): Query<LogQuery>,
) -> ApiResult<Value> {
    let limit = q.limit.unwrap_or(20).clamp(1, MAX_LOG);
    let commits = match q.path.as_deref() {
        Some(path) => {
            st.vcs
                .path_log(&repo, path, limit, MAX_PATH_LOG_SCAN)
                .await?
        }
        None => st.vcs.trunk_log(&repo, limit).await?,
    };
    // each commit with its post-merge check (the failing one if any), or null
    let results = crate::checks::results_by_change(&st, &repo, &commits).await?;
    let commits: Vec<Value> = commits
        .into_iter()
        .map(|c| {
            let check = results
                .get(&c.change.0)
                .and_then(|rows| crate::checks::summary(rows))
                .map(
                    |r| json!({"command": r.command, "exit_code": r.exit_code, "runner": r.runner}),
                );
            let mut v = json!(c);
            v["check"] = check.unwrap_or(Value::Null);
            v
        })
        .collect();
    Ok(Json(json!({ "commits": commits })))
}

#[derive(Deserialize)]
struct EventsQuery {
    repo: Option<String>,
    /// Events after this id. Without it the feed starts at its current head (nothing old).
    since: Option<i64>,
    /// Seconds to wait for a first event when there is none yet (long-poll), at most 25.
    wait: Option<u64>,
    limit: Option<usize>,
}

const MAX_EVENTS: usize = 500;
const MAX_EVENT_WAIT: std::time::Duration = std::time::Duration::from_secs(25);
/// How often a long-poll looks for new events. Polling the indexed table keeps no wake-up state
/// that a commit could race past.
const EVENT_POLL: std::time::Duration = std::time::Duration::from_millis(200);

/// The lifecycle event feed, oldest first: `{events, next}`, where `next` is the `since` to pass
/// on the following call.
async fn events(State(st): State<AppState>, Query(q): Query<EventsQuery>) -> ApiResult<Value> {
    let repo = q.repo.as_deref();
    let since = match q.since {
        Some(s) => s,
        None => st.store.last_event_id(repo).await?,
    };
    let limit = q.limit.unwrap_or(100).clamp(1, MAX_EVENTS);
    let wait = std::time::Duration::from_secs(q.wait.unwrap_or(0)).min(MAX_EVENT_WAIT);
    let deadline = tokio::time::Instant::now() + wait;
    loop {
        let found = st.store.events_since(repo, since, limit).await?;
        if !found.is_empty() || tokio::time::Instant::now() >= deadline {
            let next = found.last().map_or(since, |e| e.id);
            return Ok(Json(json!({"events": found, "next": next})));
        }
        tokio::time::sleep(EVENT_POLL).await;
    }
}

async fn get_settings(
    State(st): State<AppState>,
    Path(repo): Path<String>,
) -> ApiResult<RepoSettings> {
    Ok(Json(st.store.repo_settings(&repo).await?))
}

#[derive(Deserialize)]
struct SettingsBody {
    carry_globs: Option<Vec<String>>,
    check_command: Option<String>,
}

/// Updates the given settings; fields left out keep their value. An empty `check_command`
/// clears it.
async fn put_settings(
    State(st): State<AppState>,
    Path(repo): Path<String>,
    Json(b): Json<SettingsBody>,
) -> ApiResult<RepoSettings> {
    let mut s = st.store.repo_settings(&repo).await?;
    if let Some(globs) = b.carry_globs {
        s.carry_globs = globs;
    }
    if let Some(cmd) = b.check_command {
        s.check_command = Some(cmd).filter(|c| !c.trim().is_empty());
    }
    st.store.set_repo_settings(&repo, &s).await?;
    Ok(Json(s))
}

/// Every convention of the repo, oldest first.
async fn list_conventions(
    State(st): State<AppState>,
    Path(repo): Path<String>,
) -> ApiResult<Value> {
    Ok(Json(
        json!({"conventions": st.store.conventions(&repo).await?}),
    ))
}

#[derive(Deserialize)]
struct AtQuery {
    at: Option<String>,
    /// `raw=1`: the file's bytes as the response body instead of JSON.
    #[serde(default)]
    raw: Option<u8>,
}

/// `at` as a commit: `main` (the default) is trunk, `task:<slug>` is that task's last checkpoint
/// (its base before the first one), and anything else is taken as a full commit id.
pub(crate) async fn resolve_at(st: &AppState, repo: &str, at: Option<&str>) -> Result<ChangeId> {
    match at.unwrap_or("main") {
        "main" => st.vcs.trunk(repo).await,
        at => match at.strip_prefix("task:") {
            Some(slug) => {
                let t = st.store.task_by_slug(repo, slug).await?;
                Ok(if t.head_change.0.is_empty() {
                    t.base_change
                } else {
                    t.head_change
                })
            }
            None if !at.is_empty() && at.bytes().all(|b| b.is_ascii_hexdigit()) => {
                Ok(ChangeId(at.to_ascii_lowercase()))
            }
            None => Err(AvError::Invalid(format!(
                "bad at={at:?}; expected main, task:<slug> or a commit id"
            ))),
        },
    }
}

/// One file at a commit, without a workspace: `content` when it is UTF-8 text, else `binary`;
/// with `raw=1` the bytes themselves (how `av runner` materializes a tree).
async fn read_file_at(
    State(st): State<AppState>,
    Path((repo, path)): Path<(String, String)>,
    Query(q): Query<AtQuery>,
) -> std::result::Result<axum::response::Response, crate::error::ApiError> {
    use axum::response::IntoResponse;
    let at = resolve_at(&st, &repo, q.at.as_deref()).await?;
    let bytes = st.vcs.read_file(&repo, &at, &path).await?;
    if q.raw == Some(1) {
        return Ok((
            [(axum::http::header::CONTENT_TYPE, "application/octet-stream")],
            bytes,
        )
            .into_response());
    }
    let size = bytes.len();
    Ok(Json(match String::from_utf8(bytes) {
        Ok(text) => json!({"path": path, "at": at, "bytes": size, "content": text}),
        Err(_) => json!({"path": path, "at": at, "bytes": size, "binary": true}),
    })
    .into_response())
}

/// Every file at a commit (`.gitignore`-filtered), without a workspace.
async fn tree(
    State(st): State<AppState>,
    Path(repo): Path<String>,
    Query(q): Query<AtQuery>,
) -> ApiResult<Value> {
    let at = resolve_at(&st, &repo, q.at.as_deref()).await?;
    let files = st.vcs.list_files(&repo, &at).await?;
    Ok(Json(json!({"at": at, "files": files})))
}

#[derive(Deserialize)]
struct DiffQuery {
    from: String,
    to: Option<String>,
}

/// Per changed path between two commits (each `main`, `task:<slug>` or a commit id; `to`
/// defaults to main): `added`, `deleted` or `modified`, with a unified diff for text files.
async fn diff(
    State(st): State<AppState>,
    Path(repo): Path<String>,
    Query(q): Query<DiffQuery>,
) -> ApiResult<Value> {
    let from = resolve_at(&st, &repo, Some(&q.from)).await?;
    let to = resolve_at(&st, &repo, q.to.as_deref()).await?;
    let paths = st.vcs.changed_paths(&repo, &from, &to).await?;
    let read = |change: ChangeId, paths: Vec<String>| {
        let st = st.clone();
        let repo = repo.clone();
        async move { st.vcs.read_files(&repo, &change, &paths).await }
    };
    let (old, new) =
        futures::try_join!(read(from.clone(), paths.clone()), read(to.clone(), paths))?;
    let mut files = Vec::with_capacity(old.len());
    for ((path, a), (_, b)) in old.into_iter().zip(new) {
        let present = |r: Result<Vec<u8>>| match r {
            Ok(bytes) => Ok(Some(bytes)),
            Err(AvError::NotFound(_)) => Ok(None),
            Err(e) => Err(e),
        };
        let (a, b) = (present(a)?, present(b)?);
        let status = match (&a, &b) {
            (None, _) => "added",
            (_, None) => "deleted",
            _ => "modified",
        };
        let text = |x: &Option<Vec<u8>>| match x {
            None => Some(String::new()),
            Some(bytes) => String::from_utf8(bytes.clone()).ok(),
        };
        let diff = match (text(&a), text(&b)) {
            (Some(a), Some(b)) => json!(scholion_pack::diff::unified_diff(&path, &a, &b)),
            _ => Value::Null,
        };
        files.push(json!({"path": path, "status": status, "diff": diff, "binary": diff.is_null()}));
    }
    Ok(Json(json!({"from": from, "to": to, "files": files})))
}

#[derive(Deserialize)]
struct BundleQuery {
    kind: Option<String>,
    budget: Option<u32>,
    family: Option<AgentFamily>,
}

/// `?kind=handoff` (default) or `?kind=pr`: the review bundle with tests, risks and checklist.
async fn bundle(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
    Query(q): Query<BundleQuery>,
) -> ApiResult<scholion_pack::Bundle> {
    let req = scholion_pack::BundleRequest {
        task_id: task_id(id),
        budget_tokens: q.budget.unwrap_or(DEFAULT_BUDGET),
        family: q.family.unwrap_or_default(),
    };
    let bundle = match q.kind.as_deref().unwrap_or("handoff") {
        "handoff" => st.packer.handoff_bundle(req).await?,
        "pr" => st.packer.pr_bundle(req).await?,
        other => {
            return Err(AvError::Invalid(format!(
                "unknown bundle kind {other:?}; expected handoff or pr"
            ))
            .into());
        }
    };
    Ok(Json(bundle))
}

#[derive(Deserialize)]
struct ExpandQuery {
    session: Option<String>,
}

/// With `?session=`, the expansion is charged to that session (and capped by `AV_EXPAND_CAP_TOKENS`).
async fn expand(
    State(st): State<AppState>,
    Path((id, item_id)): Path<(Uuid, String)>,
    Query(q): Query<ExpandQuery>,
) -> ApiResult<Value> {
    match q.session.filter(|s| !s.trim().is_empty()) {
        Some(session) => {
            let e = st
                .packer
                .expand_charged(task_id(id), &item_id, &session, st.expand_cap)
                .await?;
            Ok(Json(json!({
                "full": e.full,
                "tokens": e.tokens,
                "session_expand_tokens": e.session_expand_tokens,
            })))
        }
        None => {
            let full = st.packer.expand(task_id(id), &item_id).await?;
            Ok(Json(json!({ "full": full })))
        }
    }
}
