//! Plan 11: what agents leave on disk. The workspace of a merged or abandoned task is removed
//! once it has been in that status for `AV_WS_RETAIN_SECS` (by the sweep, `POST /v1/gc`, or with
//! the merge or abandon itself when that is 0), and comes back when an undo or revert returns the
//! task to work. `GET /v1/disk` shows what is where, a workspace's own build output (`target/`,
//! Plan 12) counted apart. The daemon deletes only under `ws_root`.
//!
//! The free-space floor (`AV_DISK_MIN_FREE_GB`, `Floor`): when the filesystem holding `ws_root`
//! has less free than that, the reclaim tick and every task start first reclaim as `POST /v1/gc`
//! does (finished tasks' workspaces oldest first, whatever the retention, then each repo's build
//! cache); while it is still below, no new task workspace is made (HTTP 507) and
//! `GET /v1/attention` says `disk_low`. Tasks that already have a workspace keep working.
//!
//! The build budget (`AV_BUILD_CACHE_MAX_GB`, `Budget`): each repo's build output (every
//! workspace's own `target/` plus `<repo>/.cache/`) is held under it by the reclaim tick and
//! `POST /v1/gc` (`enforce_budget`): finished tasks' `target/` first, then idle ones, then the deps
//! cache. A `target/` touched within `AV_BUILD_IDLE_SECS` is never removed.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::extract::{Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use margincode_core::{AvError, Result, Task, TaskId, TaskStatus};
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

/// The free-space floor: its size in bytes (0 = off), since when the disk has been below it, and
/// one lock, so a burst of task starts on a full disk runs one reclaim rather than one each.
#[derive(Clone, Default)]
pub(crate) struct Floor {
    min_free: Arc<AtomicU64>,
    low_since: Arc<Mutex<Option<DateTime<Utc>>>>,
    reclaiming: Arc<tokio::sync::Mutex<()>>,
}

impl Floor {
    pub(crate) fn set(&self, bytes: u64) {
        self.min_free.store(bytes, Ordering::Relaxed);
    }

    pub(crate) fn bytes(&self) -> u64 {
        self.min_free.load(Ordering::Relaxed)
    }

    /// Records whether the disk is below the floor now; when it is, says since when.
    fn mark(&self, low: bool) -> Option<DateTime<Utc>> {
        let mut since = self.low_since.lock().unwrap_or_else(|e| e.into_inner());
        if low {
            Some(*since.get_or_insert_with(Utc::now))
        } else {
            *since = None;
            None
        }
    }
}

/// The build budget (`AV_BUILD_CACHE_MAX_GB`): the most build output one repo keeps, in bytes
/// (0 = off), how long an unfinished task's `target/` must be untouched before it may go, what
/// the last enforcement left over budget, and one lock, so the tick and `POST /v1/gc` never
/// enforce at once.
#[derive(Clone)]
pub(crate) struct Budget {
    max: Arc<AtomicU64>,
    idle_secs: Arc<AtomicU64>,
    over: Arc<Mutex<BTreeMap<String, Over>>>,
    enforcing: Arc<tokio::sync::Mutex<()>>,
}

impl Default for Budget {
    fn default() -> Self {
        Self {
            max: Arc::default(),
            idle_secs: Arc::new(AtomicU64::new(crate::wiring::DEFAULT_BUILD_IDLE_SECS)),
            over: Arc::default(),
            enforcing: Arc::default(),
        }
    }
}

impl Budget {
    pub(crate) fn set(&self, bytes: u64) {
        self.max.store(bytes, Ordering::Relaxed);
    }

    pub(crate) fn bytes(&self) -> u64 {
        self.max.load(Ordering::Relaxed)
    }

    pub(crate) fn set_idle(&self, secs: u64) {
        self.idle_secs.store(secs, Ordering::Relaxed);
    }

    /// Keeps the repos an enforcement left over budget, each with the time it was first found
    /// over (kept from the pass before when it was over then too); returns them.
    fn record(&self, over: Vec<(String, u64)>) -> Vec<Over> {
        let budget = self.bytes();
        let now = Utc::now();
        let mut kept = self.over.lock().unwrap_or_else(|e| e.into_inner());
        let next: BTreeMap<String, Over> = over
            .into_iter()
            .map(|(repo, bytes)| {
                let since = kept.get(&repo).map_or(now, |o| o.since);
                let o = Over {
                    repo: repo.clone(),
                    bytes,
                    budget_bytes: budget,
                    since,
                };
                (repo, o)
            })
            .collect();
        *kept = next;
        kept.values().cloned().collect()
    }

    /// The repos the last enforcement left over budget; none while the budget is off.
    pub(crate) fn over(&self) -> Vec<Over> {
        if self.bytes() == 0 {
            return Vec::new();
        }
        let kept = self.over.lock().unwrap_or_else(|e| e.into_inner());
        kept.values().cloned().collect()
    }
}

/// A repo the last enforcement left over its build budget.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct Over {
    pub(crate) repo: String,
    /// Its build output then (every `target/` plus `.cache/`), as the enforcement reckoned it.
    pub(crate) bytes: u64,
    pub(crate) budget_bytes: u64,
    /// Since when enforcement has found it over (since the daemon started).
    pub(crate) since: DateTime<Utc>,
}

/// The disk under `ws_root` is below the floor.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Low {
    pub(crate) free: u64,
    pub(crate) floor: u64,
    /// When it was first measured below (since the daemon started).
    pub(crate) since: DateTime<Utc>,
}

impl Low {
    /// Why a new task is refused: the floor and the free bytes, and what still works.
    fn reason(&self) -> String {
        format!(
            "{} ({} bytes) free under the workspace root is below the floor AV_DISK_MIN_FREE_GB, \
             {} ({} bytes), even after reclaiming finished tasks' workspaces and the build \
             caches; no new task workspace is made until there is room. Tasks already started \
             keep working: submit, merge or abandon them, free space, or lower the floor",
            crate::attention::bytes_text(self.free),
            self.free,
            crate::attention::bytes_text(self.floor),
            self.floor
        )
    }
}

/// Free and total bytes of the filesystem holding `path` (or its nearest existing ancestor):
/// `statvfs`, free being what an unprivileged process may still write. `None` when unknown.
#[cfg(unix)]
// the field types differ by platform (u32 block counts on macOS)
#[allow(clippy::useless_conversion)]
pub(crate) fn fs_space(path: &Path) -> Option<(u64, u64)> {
    use std::os::unix::ffi::OsStrExt;
    path.ancestors().find_map(|dir| {
        let c = std::ffi::CString::new(dir.as_os_str().as_bytes()).ok()?;
        let mut vfs = std::mem::MaybeUninit::<libc::statvfs>::uninit();
        // SAFETY: `c` is a NUL-terminated path and `vfs` is valid for writes; statvfs fills it
        // in whole when it returns 0, and it is read only then.
        let vfs = unsafe {
            if libc::statvfs(c.as_ptr(), vfs.as_mut_ptr()) != 0 {
                return None;
            }
            vfs.assume_init()
        };
        let block = u64::from(vfs.f_frsize);
        Some((
            u64::from(vfs.f_bavail).saturating_mul(block),
            u64::from(vfs.f_blocks).saturating_mul(block),
        ))
    })
}

/// Not measured off Unix (the daemon ships for Linux and macOS): unknown, never below the floor.
#[cfg(not(unix))]
pub(crate) fn fs_space(_: &Path) -> Option<(u64, u64)> {
    None
}

/// Measures the free space under `ws_root` against the floor: `Some` when below it. `None` when
/// at or above it, when the floor is off, and when free space is unknown.
pub(crate) fn below_floor(st: &AppState) -> Option<Low> {
    let floor = st.floor.bytes();
    let free = (floor > 0)
        .then(|| fs_space(&st.ws_root))
        .flatten()
        .map(|(free, _)| free);
    match free {
        Some(free) if free < floor => Some(Low {
            free,
            floor,
            since: st.floor.mark(true)?,
        }),
        _ => {
            st.floor.mark(false);
            None
        }
    }
}

/// Below the floor, reclaims as `POST /v1/gc` would: the workspaces of merged and abandoned tasks,
/// oldest first, whatever `AV_WS_RETAIN_SECS` says, then, when still below, each repo's build
/// cache. Says whether the disk is still below afterwards. A reclaim that fails is logged; the
/// answer is what is measured then.
pub(crate) async fn relieve(st: &AppState) -> Option<Low> {
    below_floor(st)?;
    let _one = st.floor.reclaiming.lock().await;
    // another call may have reclaimed while this one waited
    let low = below_floor(st)?;
    tracing::warn!(
        free_bytes = low.free,
        floor_bytes = low.floor,
        "free disk space is below AV_DISK_MIN_FREE_GB; reclaiming finished tasks' workspaces"
    );
    for build_cache in [false, true] {
        let retain = (!build_cache).then_some(0);
        match reclaim_with(st, retain, false, build_cache, false).await {
            Ok(r) => tracing::info!(
                workspaces = r.reclaimed.len(),
                freed_bytes = r.freed_bytes,
                build_cache,
                "reclaimed for the free-space floor"
            ),
            Err(e) => {
                tracing::warn!(error = %e, build_cache, "reclaiming for the free-space floor failed")
            }
        }
        below_floor(st)?;
    }
    let low = below_floor(st)?;
    tracing::warn!(
        free_bytes = low.free,
        floor_bytes = low.floor,
        "free disk space is still below AV_DISK_MIN_FREE_GB; no new task workspace until there is room"
    );
    Some(low)
}

/// Before a new task workspace is made: room for it, or `InsufficientStorage` (HTTP 507) naming
/// the floor and the free bytes.
pub(crate) async fn ensure_room(st: &AppState) -> Result<()> {
    match relieve(st).await {
        None => Ok(()),
        Some(low) => Err(AvError::InsufficientStorage(low.reason())),
    }
}

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
    pub(crate) path: String,
    pub(crate) bytes: u64,
}

/// What one session added outside any task workspace (the Stop hook's diff against its start).
/// The daemon only shows it: the paths are the agent's or the human's to remove.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Footprint {
    pub(crate) session: String,
    #[serde(default)]
    pub(crate) agent: Option<String>,
    #[serde(default)]
    pub(crate) task: Option<String>,
    /// The task's slug, looked up by the daemon when `task` names one it knows.
    #[serde(default)]
    pub(crate) slug: Option<String>,
    #[serde(default)]
    pub(crate) added: Vec<Added>,
    #[serde(default)]
    pub(crate) partial: bool,
    #[serde(default)]
    pub(crate) total_bytes: u64,
    #[serde(default = "Utc::now")]
    pub(crate) at: chrono::DateTime<Utc>,
    /// The containers, images and volumes carrying the task's `av.task` label, as the hook's
    /// `docker` listed them (`{containers, images, volumes}`); kept as sent, never acted on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) containers: Option<Value>,
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
    /// What the build budget removed (or would), when it is on.
    #[serde(skip_serializing_if = "Option::is_none")]
    build_budget: Option<Enforced>,
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
/// it is removed here and the event added at once. `reason` (the build budget's `budget`) goes
/// into the event.
async fn retire_build(st: &AppState, task: &Task, reason: Option<&'static str>) -> Result<bool> {
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
                tokio::spawn(async move { finish_retire(&st, &task, aside, reason).await });
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
            reclaimed_payload(task, bytes, 0, reason),
        )
        .await?;
    Ok(true)
}

fn reclaimed_payload(task: &Task, bytes: u64, cached: u64, reason: Option<&str>) -> Value {
    let mut payload = json!({"slug": task.slug, "bytes": bytes, "cached_bytes": cached});
    if let Some(reason) = reason {
        payload["reason"] = json!(reason);
    }
    payload
}

/// The background half of `retire_build`.
async fn finish_retire(st: &AppState, task: &Task, aside: PathBuf, reason: Option<&str>) {
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
    let payload = reclaimed_payload(task, bytes, cached_bytes, reason);
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
            if let Err(e) = retire_build(st, task, None).await {
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
        Some(_) => match retire_build(st, task, None).await {
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

/// The daemon's reclaim loop: every `reclaim_tick`, removes the workspaces past `ws_retain`, then
/// reclaims more while the disk is below the free-space floor (`relieve`), then holds each repo
/// to its build budget (`enforce_budget`). `None` (nothing started) when the retention is
/// `never` and the floor and the budget are off.
pub(crate) fn spawn_sweep(state: &AppState) -> Option<tokio::task::JoinHandle<()>> {
    if state.ws_retain.is_none() && state.floor.bytes() == 0 && state.budget.bytes() == 0 {
        return None;
    }
    let state = state.clone();
    Some(tokio::spawn(async move {
        let mut every = tokio::time::interval(state.reclaim_tick);
        every.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        loop {
            every.tick().await;
            if let Some(retain) = state.ws_retain
                && let Err(e) = sweep_once(&state, retain).await
            {
                tracing::warn!(error = %e, "reclaiming workspaces failed");
            }
            relieve(&state).await;
            if let Err(e) = enforce_budget(&state, false).await {
                tracing::warn!(error = %e, "holding the repos to their build budget failed");
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

// ---- the build budget (`AV_BUILD_CACHE_MAX_GB`) ----

/// One task workspace's `target/`, as the build budget sees it.
struct Built {
    slug: String,
    bytes: u64,
    /// The newest modification time of `target/` and what is at depth 1 and 2 under it: a build
    /// under way writes `debug/.cargo-lock` and new files into `debug/deps/`, which changes it.
    touched: Option<std::time::SystemTime>,
}

/// The newest modification time of `p` and its entries down to depth 2.
fn newest_mtime(p: &Path) -> Option<std::time::SystemTime> {
    fn walk(p: &Path, depth: u8, newest: &mut Option<std::time::SystemTime>) {
        let Ok(meta) = std::fs::symlink_metadata(p) else {
            return;
        };
        if let Ok(m) = meta.modified() {
            *newest = (*newest).max(Some(m));
        }
        if depth < 2 && meta.is_dir() {
            for e in std::fs::read_dir(p).into_iter().flatten().flatten() {
                walk(&e.path(), depth + 1, newest);
            }
        }
    }
    let mut newest = None;
    walk(p, 0, &mut newest);
    newest
}

async fn touched(target: PathBuf) -> Option<std::time::SystemTime> {
    tokio::task::spawn_blocking(move || newest_mtime(&target))
        .await
        .unwrap_or(None)
}

/// A repo's build output as the budget counts it: its `.cache/` (but build output already on its
/// way out in `.cache/reclaim/`) and every workspace's `target/`. The cache is measured first, so
/// a file the deps cache links into a `target/` counts once, under the cache, as `GET /v1/disk`
/// counts it.
async fn build_usage(st: &AppState, repo: &str) -> Result<(u64, Vec<Built>)> {
    let seen: Seen = Arc::default();
    let repo_dir = under_ws_root(st, &[repo])?;
    let cache = under_ws_root(st, &[repo, BUILD_CACHE])?;
    let leaving = under_ws_root(st, &[repo, BUILD_CACHE, RECLAIM])?;
    let cache_bytes = measure_once(cache, Some(leaving), &seen).await;
    let mut built = Vec::new();
    for slug in plain_dirs(&repo_dir) {
        let target = under_ws_root(st, &[repo, &slug, BUILD_DIR])?;
        if !exists(&target) {
            continue;
        }
        let bytes = measure_once(target.clone(), None, &seen).await;
        built.push(Built {
            slug,
            bytes,
            touched: touched(target).await,
        });
    }
    Ok((cache_bytes, built))
}

/// A repo's build output in bytes as the budget counts it, when the budget is on.
pub(crate) async fn budget_usage(st: &AppState, repo: &str) -> Option<u64> {
    if st.budget.bytes() == 0 {
        return None;
    }
    let (cache, built) = build_usage(st, repo).await.ok()?;
    Some(cache + built.iter().map(|b| b.bytes).sum::<u64>())
}

/// What one enforcement of the build budget removed (or, in a dry run, would remove).
#[derive(Debug, Default, Serialize)]
pub(crate) struct Enforced {
    /// Per repo.
    budget_bytes: u64,
    removed: Vec<Evicted>,
    freed_bytes: u64,
    /// Repos still over afterwards: only `target/`s touched within `AV_BUILD_IDLE_SECS` remain.
    over: Vec<Over>,
}

#[derive(Debug, Serialize)]
struct Evicted {
    repo: String,
    /// `finished` (a merged or abandoned task's `target/`), `idle` (an unfinished task's,
    /// untouched for `AV_BUILD_IDLE_SECS`) or `deps_cache` (`<repo>/.cache/deps-target`).
    what: &'static str,
    slug: Option<String>,
    status: Option<String>,
    bytes: u64,
    /// For `idle`: how long nothing in it changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_secs: Option<u64>,
}

/// Seconds since `t`: 0 when it is in the future, and an unknown time counts as long ago.
fn secs_since(t: Option<std::time::SystemTime>) -> u64 {
    match t {
        Some(t) => std::time::SystemTime::now()
            .duration_since(t)
            .map_or(0, |d| d.as_secs()),
        None => u64::MAX,
    }
}

/// Holds every repo to the build budget: for each over it, removes build output until it is
/// under, in this order: (a) the `target/` of merged and abandoned tasks, the one longest in
/// that status first (through `retire_build`, so the deps cache keeps their dependencies);
/// (b) the `target/` of unfinished tasks that nothing touched for `AV_BUILD_IDLE_SECS`, least
/// recently touched first (one touched later may be building and is never removed); (c) the
/// repo's deps cache. Workspaces, scratch dirs and anything else stay. Each `target/` goes under
/// its task's `lock_task`, the status and the idle time read again there; the feed gets
/// `build:reclaimed` with `reason: budget`. The repos still over are kept for
/// `GET /v1/attention` (`build_over_budget`). `None` when the budget is off.
pub(crate) async fn enforce_budget(st: &AppState, dry_run: bool) -> Result<Option<Enforced>> {
    let budget = st.budget.bytes();
    if budget == 0 {
        st.budget.record(Vec::new());
        return Ok(None);
    }
    let _one = st.budget.enforcing.lock().await;
    let idle_for = st.budget.idle_secs.load(Ordering::Relaxed);
    let mut tasks: HashMap<(String, String), (Task, DateTime<Utc>)> = HashMap::new();
    for (task, since) in st.store.tasks_with_status_since(None).await? {
        tasks.insert((task.repo.clone(), task.slug.clone()), (task, since));
    }
    let mut out = Enforced {
        budget_bytes: budget,
        ..Enforced::default()
    };
    let mut over = Vec::new();
    for repo in plain_dirs(&st.ws_root) {
        let Ok((cache_bytes, built)) = build_usage(st, &repo).await else {
            continue;
        };
        // measured once: what a removal moves into .cache/reclaim/ is gone from here on, and
        // what its harvest adds to the deps cache is counted by the next pass
        let mut total = cache_bytes + built.iter().map(|b| b.bytes).sum::<u64>();
        if total <= budget {
            continue;
        }
        tracing::info!(
            %repo,
            build_bytes = total,
            budget_bytes = budget,
            "build output is over AV_BUILD_CACHE_MAX_GB; removing finished, then idle targets"
        );
        let task_of = |b: &Built| tasks.get(&(repo.clone(), b.slug.clone()));
        let mut finished: Vec<(&Built, &Task, DateTime<Utc>)> = built
            .iter()
            .filter_map(|b| task_of(b).map(|(t, since)| (b, t, *since)))
            .filter(|(_, t, _)| ended(t.status))
            .collect();
        finished.sort_by_key(|(_, _, since)| *since);
        let mut idle: Vec<(&Built, &Task)> = built
            .iter()
            .filter_map(|b| task_of(b).map(|(t, _)| (b, t)))
            .filter(|(b, t)| !ended(t.status) && secs_since(b.touched) >= idle_for)
            .collect();
        idle.sort_by_key(|(b, _)| b.touched);
        let steps = finished
            .into_iter()
            .map(|(b, t, _)| ("finished", b, t))
            .chain(idle.into_iter().map(|(b, t)| ("idle", b, t)));
        for (what, b, task) in steps {
            if total <= budget {
                break;
            }
            let mut status = task.status;
            if !dry_run {
                let _held = st.lock_task(task.id).await;
                let task = st.store.get_task(task.id).await?;
                status = task.status;
                // an undo puts a finished task back to work, and an idle one may have started
                // building while this waited for the lock
                let still = if what == "finished" {
                    ended(task.status)
                } else {
                    let target = under_ws_root(st, &[&task.repo, &task.slug, BUILD_DIR])?;
                    secs_since(touched(target).await) >= idle_for
                };
                if !still {
                    continue;
                }
                match retire_build(st, &task, Some("budget")).await {
                    Ok(true) => {}
                    Ok(false) => continue,
                    Err(e) => {
                        tracing::warn!(task = %task.slug, error = %e,
                            "removing a target/ over the build budget failed");
                        continue;
                    }
                }
                tracing::info!(
                    %repo, task = %task.slug, status = task.status.as_str(), bytes = b.bytes,
                    why = what, "removed a target/ over AV_BUILD_CACHE_MAX_GB"
                );
            }
            total = total.saturating_sub(b.bytes);
            out.freed_bytes += b.bytes;
            out.removed.push(Evicted {
                repo: repo.clone(),
                what,
                slug: Some(b.slug.clone()),
                status: Some(status.as_str().to_owned()),
                bytes: b.bytes,
                idle_secs: (what == "idle").then(|| secs_since(b.touched)),
            });
        }
        if total > budget
            && let Ok(deps) = deps_cache_dir(st, &repo)
            && exists(&deps)
        {
            let bytes = measure(deps.clone()).await;
            let removed = dry_run
                || match remove_tree(deps).await {
                    Ok(()) => {
                        tracing::info!(%repo, bytes,
                            "removed the deps cache over AV_BUILD_CACHE_MAX_GB");
                        true
                    }
                    Err(e) => {
                        tracing::warn!(%repo, error = %e,
                            "removing the deps cache over the build budget failed");
                        false
                    }
                };
            if removed {
                total = total.saturating_sub(bytes);
                out.freed_bytes += bytes;
                out.removed.push(Evicted {
                    repo: repo.clone(),
                    what: "deps_cache",
                    slug: None,
                    status: None,
                    bytes,
                    idle_secs: None,
                });
            }
        }
        if total > budget {
            tracing::warn!(
                %repo,
                build_bytes = total,
                budget_bytes = budget,
                "build output is still over AV_BUILD_CACHE_MAX_GB: only targets touched within \
                 AV_BUILD_IDLE_SECS remain"
            );
            over.push((repo.clone(), total));
        }
    }
    out.over = if dry_run {
        let now = Utc::now();
        over.into_iter()
            .map(|(repo, bytes)| Over {
                repo,
                bytes,
                budget_bytes: budget,
                since: now,
            })
            .collect()
    } else {
        st.budget.record(over)
    };
    Ok(Some(out))
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
    let mut out = reclaim(&st, retain, b.dry_run, b.build_cache).await?;
    // after the workspaces: what they took with them no longer counts
    out.build_budget = enforce_budget(&st, b.dry_run).await?;
    out.freed_bytes += out.build_budget.as_ref().map_or(0, |b| b.freed_bytes);
    Ok(Json(out))
}

#[derive(Deserialize)]
struct DiskQuery {
    /// `1` or `true`: only the filesystem numbers (`fs_only`).
    fs: Option<String>,
}

impl DiskQuery {
    fn fs_only(&self) -> Result<bool> {
        match self.fs.as_deref().map(str::to_ascii_lowercase).as_deref() {
            None | Some("0" | "false") => Ok(false),
            Some("1" | "true") => Ok(true),
            Some(other) => Err(AvError::Invalid(format!(
                "fs must be 1, true, 0 or false, not {other:?}"
            ))),
        }
    }
}

/// The filesystem holding `ws_root` (null where not measured), the floor (0 = off) and the
/// build budget (0 = off): measured by `fs_space` alone, so it costs no directory walk.
fn fs_fields(st: &AppState) -> serde_json::Map<String, Value> {
    let fs = fs_space(&st.ws_root);
    let below = below_floor(st).is_some();
    [
        ("fs_free_bytes", json!(fs.map(|(free, _)| free))),
        ("fs_total_bytes", json!(fs.map(|(_, total)| total))),
        ("min_free_bytes", json!(st.floor.bytes())),
        ("below_floor", json!(below)),
        // AV_BUILD_CACHE_MAX_GB, per repo (0 = off)
        ("build_budget_bytes", json!(st.budget.bytes())),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_owned(), v))
    .collect()
}

/// Every repo's workspaces (with their task's status, how long it has had it, and whether a
/// reclaim would remove it), scratch, build cache and store, with totals. With `?fs=1` only
/// `fs_fields`: no store query and no walk (Margincode Cloud's `disk_low` alert asks every 60 s).
async fn disk(State(st): State<AppState>, Query(q): Query<DiskQuery>) -> ApiResult<Value> {
    if q.fs_only()? {
        return Ok(Json(Value::Object(fs_fields(&st))));
    }
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
    let budget = st.budget.bytes();
    let mut any_over = false;
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
        let mut repo_build = 0;
        for slug in plain_dirs(&repo_dir) {
            let (ws, target) = (repo_dir.join(&slug), repo_dir.join(&slug).join(BUILD_DIR));
            let build_bytes = measure_once(target.clone(), None, &seen).await;
            let bytes = build_bytes + measure_once(ws, Some(target), &seen).await;
            ws_total += bytes;
            build_total += build_bytes;
            repo_build += build_bytes;
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
        // as the budget counts it (its targets and .cache/, but what is on its way out)
        let over_budget = budget_usage(&st, &name)
            .await
            .is_some_and(|used| used > budget);
        any_over |= over_budget;
        repos.push(json!({
            "name": name,
            "store_bytes": store_bytes,
            "build_cache": {"enabled": st.build_cache, "config": config},
            "build_cache_bytes": build_cache_bytes,
            // every workspace's target/; the budget counts these plus build_cache_bytes
            "build_bytes": repo_build,
            "build_budget_bytes": budget,
            "over_budget": over_budget,
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
    let mut out = json!({
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
            "over_budget": any_over,
        },
    });
    if let Value::Object(m) = &mut out {
        m.extend(fs_fields(&st));
    }
    Ok(Json(out))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fs_1_or_true_asks_for_the_filesystem_numbers_only() {
        let q = |fs: Option<&str>| DiskQuery {
            fs: fs.map(str::to_owned),
        };
        for on in ["1", "true", "TRUE"] {
            assert!(q(Some(on)).fs_only().unwrap(), "{on}");
        }
        for off in [None, Some("0"), Some("false")] {
            assert!(!q(off).fs_only().unwrap(), "{off:?}");
        }
        assert!(q(Some("yes")).fs_only().is_err());
    }

    #[test]
    fn free_space_of_a_temp_dir_is_measured_here() {
        let dir = tempfile::tempdir().unwrap();
        let space = fs_space(dir.path());
        if cfg!(unix) {
            let (free, total) = space.expect("statvfs answers on Unix");
            assert!(total > 0, "total {total}");
            assert!(free <= total, "free {free} of {total}");
        } else {
            assert_eq!(space, None);
        }
        // a path not made yet is measured on the filesystem it would be made on
        assert_eq!(
            fs_space(&dir.path().join("not/yet")).map(|(_, t)| t),
            space.map(|(_, t)| t)
        );
    }
}
