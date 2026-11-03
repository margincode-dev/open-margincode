//! Plan 11: what agents leave on disk. The workspace of a merged or abandoned task is removed
//! once it has been in that status for `AV_WS_RETAIN_SECS` (by the sweep, `POST /v1/gc`, or with
//! the merge or abandon itself when that is 0), and comes back when an undo or revert returns the
//! task to work. `GET /v1/disk` shows what is where, a workspace's own build output (`target/`,
//! Plan 12) counted apart. The daemon deletes only under `ws_root`.

use std::collections::{BTreeMap, BTreeSet, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::Utc;
use scholion_core::{AvError, Result, Task, TaskId, TaskStatus};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::AppState;
use crate::error::ApiResult;

/// How often the daemon looks for workspaces to reclaim.
pub const RECLAIM_TICK: Duration = Duration::from_secs(60);

/// `<ws_root>/<repo>/.scratch/<slug>`: a task's temporary files, removed with its workspace.
const SCRATCH: &str = ".scratch";
/// `<ws_root>/<repo>/.cache`: a cache every workspace of a repo shares (none is written today;
/// Plan 11's shared cargo target lived here and `av gc --build-cache` still removes one).
const BUILD_CACHE: &str = ".cache";
/// `<ws_root>/<repo>/<slug>/target`: a task's own build output (cargo's default target dir;
/// Plan 12), counted apart so storage per task is a number of its own.
const BUILD_DIR: &str = "target";

pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .route("/v1/disk", get(disk))
        .route("/v1/gc", post(gc))
        .route("/v1/footprint", post(post_footprint))
}

/// Most footprint reports kept.
const FOOTPRINTS: usize = 200;

pub(crate) type Footprints = Arc<Mutex<VecDeque<Footprint>>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Added {
    path: String,
    bytes: u64,
}

/// What one session added outside any task workspace (the Stop hook's diff against its start).
/// The daemon only shows it: the paths are the agent's or the human's to remove.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Footprint {
    session: String,
    #[serde(default)]
    agent: Option<String>,
    #[serde(default)]
    task: Option<String>,
    /// The task's slug, looked up by the daemon when `task` names one it knows.
    #[serde(default)]
    slug: Option<String>,
    #[serde(default)]
    added: Vec<Added>,
    #[serde(default)]
    partial: bool,
    #[serde(default)]
    total_bytes: u64,
    #[serde(default = "Utc::now")]
    at: chrono::DateTime<Utc>,
}

/// Keeps a session's report (replacing its earlier one), newest first, at most `FOOTPRINTS`.
async fn post_footprint(
    State(st): State<AppState>,
    Json(mut f): Json<Footprint>,
) -> ApiResult<Value> {
    if f.session.trim().is_empty() {
        return Err(AvError::Invalid("session must not be blank".into()).into());
    }
    f.total_bytes = f.added.iter().map(|a| a.bytes).sum();
    f.at = Utc::now();
    f.slug = None;
    if let Some(id) = f
        .task
        .as_deref()
        .and_then(|t| uuid::Uuid::parse_str(t).ok())
        && let Ok(task) = st.store.get_task(TaskId(id)).await
    {
        f.slug = Some(task.slug);
    }
    let mut kept = st.footprints.lock().unwrap_or_else(|e| e.into_inner());
    kept.retain(|old| old.session != f.session);
    kept.push_front(f);
    kept.truncate(FOOTPRINTS);
    Ok(Json(json!({"kept": kept.len()})))
}

/// What a reclaim removed (or, in a dry run, would remove) and what it kept.
#[derive(Debug, Default, Serialize)]
pub(crate) struct Reclaimed {
    reclaimed: Vec<Gone>,
    freed_bytes: u64,
    /// Merged and abandoned tasks whose workspace is younger than the retention.
    kept: Vec<Kept>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_cache_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    errors: Vec<Failed>,
    dry_run: bool,
}

#[derive(Debug, Serialize)]
struct Gone {
    repo: String,
    slug: String,
    status: String,
    path: PathBuf,
    /// Workspace, its build output and its scratch dir together.
    bytes: u64,
    /// The part of `bytes` that was the workspace's `target/`.
    build_bytes: u64,
}

#[derive(Debug, Serialize)]
struct Kept {
    repo: String,
    slug: String,
    status: String,
    path: PathBuf,
    bytes: u64,
    build_bytes: u64,
    age_secs: u64,
}

#[derive(Debug, Serialize)]
struct Failed {
    repo: String,
    slug: String,
    error: String,
}

fn ended(status: TaskStatus) -> bool {
    matches!(status, TaskStatus::Merged | TaskStatus::Abandoned)
}

/// `ws_root` joined with `parts`, each a plain name (no separators, never `.` or `..`): nothing
/// the daemon deletes is outside `ws_root`.
pub(crate) fn under_ws_root(st: &AppState, parts: &[&str]) -> Result<PathBuf> {
    let mut path = st.ws_root.clone();
    for part in parts {
        let plain =
            !part.is_empty() && *part != "." && *part != ".." && !part.contains(['/', '\\', '\0']);
        if !plain {
            return Err(AvError::Invalid(format!(
                "{part:?} is not a plain name under the workspace root"
            )));
        }
        path.push(part);
    }
    if !path.starts_with(&st.ws_root) {
        return Err(AvError::Invalid(format!(
            "{} is outside the workspace root",
            path.display()
        )));
    }
    Ok(path)
}

fn exists(p: &Path) -> bool {
    std::fs::symlink_metadata(p).is_ok()
}

/// Bytes of the files under `p` (symlinks count as themselves, never followed).
fn bytes_of(p: &Path) -> u64 {
    let Ok(meta) = std::fs::symlink_metadata(p) else {
        return 0;
    };
    if !meta.is_dir() {
        return meta.len();
    }
    std::fs::read_dir(p)
        .map(|entries| entries.flatten().map(|e| bytes_of(&e.path())).sum())
        .unwrap_or(0)
}

async fn measure(p: PathBuf) -> u64 {
    tokio::task::spawn_blocking(move || bytes_of(&p))
        .await
        .unwrap_or(0)
}

/// Files with more than one link already counted in one `GET /v1/disk` (device, inode).
type Seen = Arc<Mutex<HashSet<(u64, u64)>>>;

/// `(device, inode)` of a file other paths link to too; `None` for a file with one link.
#[cfg(unix)]
fn shared_inode(meta: &std::fs::Metadata) -> Option<(u64, u64)> {
    use std::os::unix::fs::MetadataExt;
    (meta.nlink() > 1).then(|| (meta.dev(), meta.ino()))
}

#[cfg(not(unix))]
fn shared_inode(_: &std::fs::Metadata) -> Option<(u64, u64)> {
    None
}

/// `bytes_of`, but a file hard-linked elsewhere counts only the first time `seen` meets it (the
/// deps cache links one copy into every task's `target/`), and `skip` is left out.
fn bytes_once(p: &Path, skip: Option<&Path>, seen: &Mutex<HashSet<(u64, u64)>>) -> u64 {
    if skip == Some(p) {
        return 0;
    }
    let Ok(meta) = std::fs::symlink_metadata(p) else {
        return 0;
    };
    if !meta.is_dir() {
        return match shared_inode(&meta) {
            Some(id) if !seen.lock().unwrap_or_else(|e| e.into_inner()).insert(id) => 0,
            _ => meta.len(),
        };
    }
    std::fs::read_dir(p)
        .map(|entries| {
            entries
                .flatten()
                .map(|e| bytes_once(&e.path(), skip, seen))
                .sum()
        })
        .unwrap_or(0)
}

async fn measure_once(p: PathBuf, skip: Option<PathBuf>, seen: &Seen) -> u64 {
    let seen = seen.clone();
    tokio::task::spawn_blocking(move || bytes_once(&p, skip.as_deref(), &seen))
        .await
        .unwrap_or(0)
}

/// `<ws_root>/<repo>/.cache/deps-target` (Plan 12 Task 5, `deps_cache.rs`).
fn deps_cache_dir(st: &AppState, repo: &str) -> Result<PathBuf> {
    under_ws_root(st, &[repo, BUILD_CACHE, crate::deps_cache::DIR])
}

/// Under `<repo>/.cache/`: build output on its way out of a workspace, and a seed being built.
const RECLAIM: &str = "reclaim";
const SEED: &str = "seed";

/// A fresh staging dir `<repo>/.cache/<kind>/<slug>-<ms>-<n>`: on the workspaces' filesystem, so
/// a `target/` moves there or back with one rename (instant on any mount, where copying the
/// thousands of files a Rust build holds takes a minute on Docker Desktop's).
fn staging(st: &AppState, repo: &str, kind: &str, slug: &str) -> Result<PathBuf> {
    static N: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let n = N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let name = format!("{slug}-{}-{n}", Utc::now().timestamp_millis());
    under_ws_root(st, &[repo, BUILD_CACHE, kind, &name])
}

/// Removes the staging dirs a daemon that stopped mid-job left (start-up: no job runs then).
pub(crate) async fn clear_staging(st: &AppState, repo: &str) {
    for kind in [RECLAIM, SEED] {
        let Ok(dir) = under_ws_root(st, &[repo, BUILD_CACHE, kind]) else {
            continue;
        };
        if exists(&dir)
            && let Err(e) = remove_tree(dir).await
        {
            tracing::warn!(%repo, error = %e, "removing leftover build output failed");
        }
    }
}

/// Takes `task`'s `target/` out of its workspace; says whether it had one. With the build cache
/// on, the dir is moved aside (one rename) and a background job keeps its dependencies in the
/// repo's deps cache, removes it and adds `build:reclaimed`; with it off, or if the move fails,
/// it is removed here and the event added at once.
async fn retire_build(st: &AppState, task: &Task) -> Result<bool> {
    let target = under_ws_root(st, &[&task.repo, &task.slug, BUILD_DIR])?;
    if !exists(&target) {
        return Ok(false);
    }
    if st.build_cache {
        let aside = staging(st, &task.repo, RECLAIM, &task.slug)?;
        let moved = aside
            .parent()
            .map_or(Ok(()), std::fs::create_dir_all)
            .and_then(|()| std::fs::rename(&target, &aside));
        match moved {
            Ok(()) => {
                let (st, task) = (st.clone(), task.clone());
                tokio::spawn(async move { finish_retire(&st, &task, aside).await });
                return Ok(true);
            }
            Err(e) => {
                tracing::warn!(task = %task.slug, error = %e, "moving the build output aside failed")
            }
        }
    }
    let bytes = measure(target.clone()).await;
    remove_tree(target).await?;
    st.store
        .add_event(
            &task.repo,
            "build:reclaimed",
            Some(task.id),
            json!({"slug": task.slug, "bytes": bytes, "cached_bytes": 0}),
        )
        .await?;
    Ok(true)
}

/// The background half of `retire_build`.
async fn finish_retire(st: &AppState, task: &Task, aside: PathBuf) {
    let bytes = measure(aside.clone()).await;
    let cached_bytes = match deps_cache_dir(st, &task.repo) {
        Ok(cache) => {
            let from = aside.clone();
            match tokio::task::spawn_blocking(move || crate::deps_cache::harvest(&from, &cache))
                .await
            {
                Ok(Ok(placed)) => placed.bytes,
                Ok(Err(e)) => {
                    tracing::warn!(task = %task.slug, error = %e, "keeping the dependencies failed");
                    0
                }
                Err(e) => {
                    tracing::warn!(task = %task.slug, error = %e, "keeping the dependencies failed");
                    0
                }
            }
        }
        Err(_) => 0,
    };
    if let Err(e) = remove_tree(aside).await {
        tracing::warn!(task = %task.slug, error = %e, "removing the build output failed");
    }
    let payload = json!({"slug": task.slug, "bytes": bytes, "cached_bytes": cached_bytes});
    if let Err(e) = st
        .store
        .add_event(&task.repo, "build:reclaimed", Some(task.id), payload)
        .await
    {
        tracing::warn!(task = %task.slug, error = %e, "recording build:reclaimed failed");
    }
}

/// Gives `task`'s new or restored workspace the repo's built dependencies, in the background:
/// the seed is built in a staging dir and moved in whole as the workspace's `target/`, unless by
/// then the workspace is gone or has a `target/` of its own (the agent started building), so
/// cargo never sees half a seed. The feed gets `build:seeded`.
pub(crate) fn seed_dependencies(st: &AppState, task: &Task) {
    if !st.build_cache {
        return;
    }
    let Ok(cache) = deps_cache_dir(st, &task.repo) else {
        return;
    };
    if !exists(&cache) {
        return;
    }
    let (st, task) = (st.clone(), task.clone());
    tokio::spawn(async move {
        if let Err(e) = seed_job(&st, &task, cache).await {
            tracing::warn!(task = %task.slug, error = %e, "seeding the dependencies failed");
        }
    });
}

async fn seed_job(st: &AppState, task: &Task, cache: PathBuf) -> Result<()> {
    let ws = under_ws_root(st, &[&task.repo, &task.slug])?;
    let staged = staging(st, &task.repo, SEED, &task.slug)?;
    let into = staged.clone();
    let built = tokio::task::spawn_blocking(move || crate::deps_cache::seed(&cache, &into))
        .await
        .map_err(|e| AvError::Vcs(e.to_string()))?;
    let (placed, skipped) = match built {
        Ok(placed) => {
            let _held = st.lock_task(task.id).await;
            let target = ws.join(BUILD_DIR);
            if !exists(&ws) {
                (placed, Some("the workspace is gone"))
            } else if exists(&target) {
                (placed, Some("the workspace has a target/ already"))
            } else {
                match std::fs::rename(&staged, &target) {
                    Ok(()) => (placed, None),
                    Err(e) => {
                        tracing::warn!(task = %task.slug, error = %e, "moving the seed in failed");
                        (placed, Some("moving it in failed"))
                    }
                }
            }
        }
        Err(e) => {
            tracing::warn!(task = %task.slug, error = %e, "building the seed stopped");
            (Default::default(), Some("building it failed"))
        }
    };
    if exists(&staged) {
        remove_tree(staged).await?;
    }
    let payload = match skipped {
        None => json!({"slug": task.slug, "linked": placed.linked, "copied": placed.copied,
                       "bytes": placed.bytes}),
        Some(why) => json!({"slug": task.slug, "skipped": why}),
    };
    st.store
        .add_event(&task.repo, "build:seeded", Some(task.id), payload)
        .await?;
    Ok(())
}

async fn remove_tree(p: PathBuf) -> Result<()> {
    tokio::task::spawn_blocking(move || match std::fs::remove_dir_all(&p) {
        Err(e) if e.kind() != std::io::ErrorKind::NotFound => {
            Err(AvError::Vcs(format!("removing {}: {e}", p.display())))
        }
        _ => Ok(()),
    })
    .await
    .map_err(|e| AvError::Vcs(e.to_string()))?
}

/// The names of the directories in `dir` that do not start with `.`, sorted.
fn plain_dirs(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .map(|entries| {
            entries
                .flatten()
                .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
                .filter_map(|e| e.file_name().to_str().map(str::to_owned))
                .filter(|n| !n.starts_with('.'))
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    names
}

/// Removes the workspace and scratch dir of `task` (the caller holds its `lock_task`); `None` when
/// neither is on disk. A dry run only measures.
async fn reclaim_locked(st: &AppState, task: &Task, dry_run: bool) -> Result<Option<Gone>> {
    let dir = under_ws_root(st, &[&task.repo, &task.slug])?;
    let scratch = under_ws_root(st, &[&task.repo, SCRATCH, &task.slug])?;
    let (has_dir, has_scratch) = (exists(&dir), exists(&scratch));
    if !has_dir && !has_scratch {
        return Ok(None);
    }
    let build_bytes = measure(dir.join(BUILD_DIR)).await;
    let bytes = measure(dir.clone()).await + measure(scratch.clone()).await;
    if !dry_run {
        if has_dir {
            // its target/ first, so the dependencies are kept and the slow part runs apart
            if let Err(e) = retire_build(st, task).await {
                tracing::warn!(task = %task.slug, error = %e, "taking the build output out failed");
            }
            st.vcs.remove_workspace(&task.repo, &task.slug).await?;
        }
        if has_scratch {
            remove_tree(scratch).await?;
        }
        st.store
            .add_event(
                &task.repo,
                "workspace:reclaimed",
                Some(task.id),
                json!({"slug": task.slug, "bytes": bytes, "build_bytes": build_bytes}),
            )
            .await?;
    }
    Ok(Some(Gone {
        repo: task.repo.clone(),
        slug: task.slug.clone(),
        status: task.status.as_str().to_owned(),
        path: st.to_host_path(&dir),
        bytes,
        build_bytes,
    }))
}

/// Removes the workspaces of merged and abandoned tasks that have been so for at least `retain`
/// seconds (`None`: none), and with `build_cache` every repo's build cache. `kept` lists the
/// younger ones. A task that failed is logged and listed under `errors`; the others go on.
pub(crate) async fn reclaim(
    st: &AppState,
    retain: Option<u64>,
    dry_run: bool,
    build_cache: bool,
) -> Result<Reclaimed> {
    reclaim_with(st, retain, dry_run, build_cache, true).await
}

async fn reclaim_with(
    st: &AppState,
    retain: Option<u64>,
    dry_run: bool,
    build_cache: bool,
    with_kept: bool,
) -> Result<Reclaimed> {
    let mut out = Reclaimed {
        dry_run,
        ..Reclaimed::default()
    };
    let mut due = HashSet::new();
    if let Some(retain) = retain {
        for task in st.store.reclaimable_tasks(retain).await? {
            due.insert(task.id);
            let on_disk = [
                under_ws_root(st, &[&task.repo, &task.slug]),
                under_ws_root(st, &[&task.repo, SCRATCH, &task.slug]),
            ]
            .iter()
            .any(|p| p.as_ref().is_ok_and(|p| exists(p)));
            if !on_disk {
                continue;
            }
            let _guard = st.lock_task(task.id).await;
            // an undo or revert may have brought it back while this waited for the lock
            let task = st.store.get_task(task.id).await?;
            if !ended(task.status) {
                continue;
            }
            match reclaim_locked(st, &task, dry_run).await {
                Ok(Some(gone)) => {
                    out.freed_bytes += gone.bytes;
                    out.reclaimed.push(gone);
                }
                Ok(None) => {}
                Err(e) => {
                    tracing::warn!(task = %task.slug, error = %e, "reclaiming a workspace failed");
                    out.errors.push(Failed {
                        repo: task.repo.clone(),
                        slug: task.slug.clone(),
                        error: e.to_string(),
                    });
                }
            }
        }
    }
    if with_kept {
        let now = Utc::now();
        for (task, since) in st.store.tasks_with_status_since(None).await? {
            if !ended(task.status) || due.contains(&task.id) {
                continue;
            }
            let Ok(dir) = under_ws_root(st, &[&task.repo, &task.slug]) else {
                continue;
            };
            if !exists(&dir) {
                continue;
            }
            out.kept.push(Kept {
                repo: task.repo.clone(),
                slug: task.slug.clone(),
                status: task.status.as_str().to_owned(),
                path: st.to_host_path(&dir),
                build_bytes: measure(dir.join(BUILD_DIR)).await,
                bytes: measure(dir).await,
                age_secs: (now - since).num_seconds().max(0) as u64,
            });
        }
    }
    if build_cache {
        let mut total = 0;
        for repo in plain_dirs(&st.ws_root) {
            let cache = under_ws_root(st, &[&repo, BUILD_CACHE])?;
            if !exists(&cache) {
                continue;
            }
            total += measure(cache.clone()).await;
            if !dry_run {
                remove_tree(cache).await?;
            }
        }
        out.freed_bytes += total;
        out.build_cache_bytes = Some(total);
    }
    Ok(out)
}

/// What `task`, just merged or abandoned, gives back at once; the caller holds its `lock_task`.
/// With `AV_WS_RETAIN_SECS=0` the whole workspace goes. Otherwise its build output (`target/`,
/// the bulk of a Rust task's disk, rebuilt on demand) goes now and the workspace itself waits
/// for the retention (an undo, a reviewer's look); `never` keeps both. Says what the client is
/// told under `workspace` and `build`: `removed`, `kept` (or the removal failed: logged, and the
/// sweep retries the workspace), and for the build `none` when there was none.
pub(crate) async fn reclaim_task(st: &AppState, task: &Task) -> (&'static str, &'static str) {
    match st.ws_retain {
        Some(0) => match reclaim_locked(st, task, false).await {
            Ok(_) => ("removed", "removed"),
            Err(e) => {
                tracing::warn!(task = %task.slug, error = %e, "removing the workspace failed");
                ("kept", "kept")
            }
        },
        None => ("kept", "kept"),
        Some(_) => match retire_build(st, task).await {
            Ok(true) => ("kept", "removed"),
            Ok(false) => ("kept", "none"),
            Err(e) => {
                tracing::warn!(task = %task.slug, error = %e, "removing the build output failed");
                ("kept", "kept")
            }
        },
    }
}

/// Re-creates the workspace of `task` at its head when it was reclaimed (an undo or revert
/// returns a merged task to work). Says what happened: `kept` (it was there) or `restored`.
///
/// The store is left as it is: the workspace comes back at the same path with the head as its
/// working-copy commit, so the stored path, base and head all still hold (`set_workspace` would
/// reset the head to the base and lose the approval made at it).
pub(crate) async fn ensure_workspace(st: &AppState, task: &Task) -> Result<&'static str> {
    let dir = under_ws_root(st, &[&task.repo, &task.slug])?;
    if exists(&dir) {
        return Ok("kept");
    }
    st.vcs
        .restore_workspace(&task.repo, &task.slug, &task.head_change)
        .await?;
    seed_dependencies(st, task);
    Ok("restored")
}

/// The daemon's reclaim loop: every `reclaim_tick`, removes the workspaces past `ws_retain`.
/// `None` (nothing started) when the retention is `never`.
pub(crate) fn spawn_sweep(state: &AppState) -> Option<tokio::task::JoinHandle<()>> {
    let retain = state.ws_retain?;
    let state = state.clone();
    Some(tokio::spawn(async move {
        let mut every = tokio::time::interval(state.reclaim_tick);
        every.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        loop {
            every.tick().await;
            if let Err(e) = sweep_once(&state, retain).await {
                tracing::warn!(error = %e, "reclaiming workspaces failed");
            }
        }
    }))
}

/// One pass of the sweep.
pub(crate) async fn sweep_once(st: &AppState, retain: u64) -> Result<()> {
    let r = reclaim_with(st, Some(retain), false, false, false).await?;
    if !r.reclaimed.is_empty() {
        tracing::info!(
            workspaces = r.reclaimed.len(),
            freed_bytes = r.freed_bytes,
            "reclaimed the workspaces of merged and abandoned tasks"
        );
    }
    Ok(())
}

#[derive(Deserialize)]
struct GcBody {
    #[serde(default)]
    dry_run: bool,
    /// Overrides `AV_WS_RETAIN_SECS` for this call (also when that is `never`).
    retain_secs: Option<u64>,
    #[serde(default)]
    build_cache: bool,
}

async fn gc(State(st): State<AppState>, Json(b): Json<GcBody>) -> ApiResult<Reclaimed> {
    let retain = b.retain_secs.or(st.ws_retain);
    Ok(Json(reclaim(&st, retain, b.dry_run, b.build_cache).await?))
}

/// Every repo's workspaces (with their task's status, how long it has had it, and whether a
/// reclaim would remove it), scratch, build cache and store, with totals.
async fn disk(State(st): State<AppState>) -> ApiResult<Value> {
    let now = Utc::now();
    let mut tasks: BTreeMap<(String, String), (Task, chrono::DateTime<Utc>)> = BTreeMap::new();
    for (task, since) in st.store.tasks_with_status_since(None).await? {
        tasks.insert((task.repo.clone(), task.slug.clone()), (task, since));
    }
    let mut names: BTreeSet<String> = st
        .store
        .list_repos()
        .await?
        .into_iter()
        .map(|r| r.name)
        .collect();
    names.extend(plain_dirs(&st.ws_root));
    let (mut ws_total, mut store_total, mut cache_total, mut scratch_total) = (0, 0, 0, 0);
    let mut build_total = 0;
    // each repo's build cache is measured before its workspaces, so a dependency the deps cache
    // links into every task's target/ counts once, under the cache
    let seen: Seen = Arc::default();
    let mut repos = Vec::new();
    for name in names {
        let Ok(repo_dir) = under_ws_root(&st, &[&name]) else {
            continue;
        };
        let store_bytes = match &st.store_root {
            Some(root) => measure(root.join(&name)).await,
            None => 0,
        };
        let build_cache_bytes = measure_once(repo_dir.join(BUILD_CACHE), None, &seen).await;
        let scratch_bytes = measure(repo_dir.join(SCRATCH)).await;
        let mut workspaces = Vec::new();
        for slug in plain_dirs(&repo_dir) {
            let (ws, target) = (repo_dir.join(&slug), repo_dir.join(&slug).join(BUILD_DIR));
            let build_bytes = measure_once(target.clone(), None, &seen).await;
            let bytes = build_bytes + measure_once(ws, Some(target), &seen).await;
            ws_total += bytes;
            build_total += build_bytes;
            let entry = tasks.get(&(name.clone(), slug.clone()));
            let age = entry.map(|(_, since)| (now - *since).num_seconds().max(0) as u64);
            let reclaimable = entry.is_some_and(|(t, _)| ended(t.status))
                && matches!((st.ws_retain, age), (Some(r), Some(a)) if a >= r);
            workspaces.push(json!({
                "slug": slug,
                "status": entry.map(|(t, _)| t.status.as_str()),
                "bytes": bytes,
                "build_bytes": build_bytes,
                "age_secs": age,
                "reclaimable": reclaimable,
            }));
        }
        store_total += store_bytes;
        cache_total += build_cache_bytes;
        scratch_total += scratch_bytes;
        let config = crate::build_cache::is_ours(&st, &name)
            .then(|| crate::build_cache::config_path(&st, &name).ok())
            .flatten()
            .map(|p| st.to_host_path(&p));
        repos.push(json!({
            "name": name,
            "store_bytes": store_bytes,
            "build_cache": {"enabled": st.build_cache, "config": config},
            "build_cache_bytes": build_cache_bytes,
            "scratch_bytes": scratch_bytes,
            "workspaces": workspaces,
        }));
    }
    let outside: Vec<Footprint> = st
        .footprints
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .iter()
        .cloned()
        .collect();
    Ok(Json(json!({
        "ws_root": st.ws_host_root,
        "store_root": st.store_root,
        "pg_dir": st.pg_dir,
        "retain_secs": st.ws_retain,
        "repos": repos,
        "outside": outside,
        "totals": {
            "workspaces_bytes": ws_total,
            "build_bytes": build_total,
            "store_bytes": store_total,
            "build_cache_bytes": cache_total,
            "scratch_bytes": scratch_total,
        },
    })))
}
