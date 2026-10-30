//! Feeds file contents from the VCS into the code index.

use std::collections::HashSet;

use scholion_core::{AvError, ChangeId, Result, Task};
use scholion_index::{IndexStats, MAX_FILE_BYTES};

use crate::AppState;

const INDEXABLE_EXTS: [&str; 4] = ["py", "ts", "tsx", "rs"];
/// Files read (in one VCS call) and indexed (in one index transaction) at a time.
const BATCH_FILES: usize = 200;

pub(crate) fn is_indexable(path: &str) -> bool {
    path.rsplit_once('.')
        .is_some_and(|(_, ext)| INDEXABLE_EXTS.contains(&ext))
}

/// Indexes every indexable file of `change` into `(repo, scope)`, then records the scope as
/// reflecting `change`. Used for `main` (and tests); task scopes use overlays instead.
pub(crate) async fn index_change(
    state: &AppState,
    repo: &str,
    scope: &str,
    change: &ChangeId,
) -> Result<IndexStats> {
    let files: Vec<String> = state
        .vcs
        .list_files(repo, change)
        .await?
        .into_iter()
        .filter(|p| is_indexable(p))
        .collect();
    let mut total = IndexStats::default();
    for batch in files.chunks(BATCH_FILES) {
        let read: Vec<(String, Vec<u8>)> = state
            .vcs
            .read_files(repo, change, batch)
            .await?
            .into_iter()
            .filter_map(|(path, r)| r.ok().map(|bytes| (path, bytes)))
            .collect();
        let stats = state
            .index
            .index_paths_at(repo, scope, Some(change), scope == "main", read)
            .await?;
        total.parsed += stats.parsed;
        total.cached += stats.cached;
        total.unchanged += stats.unchanged;
        total.skipped += stats.skipped;
        total.failed += stats.failed;
    }
    state.index.set_scope_change(repo, scope, change).await?;
    Ok(total)
}

/// Re-indexes `changed` paths at `head` into `main` (or a full scope). Returns warnings for files
/// that could not be indexed; those never fail the call.
pub(crate) async fn reindex_changed(
    state: &AppState,
    repo: &str,
    scope: &str,
    head: &ChangeId,
    changed: &[String],
) -> Result<Vec<String>> {
    let mut warnings = vec![];
    let mut to_index = vec![];
    let mut to_remove = vec![];
    let wanted: Vec<String> = changed
        .iter()
        .filter(|p| is_indexable(p))
        .cloned()
        .collect();
    for (path, read) in state.vcs.read_files(repo, head, &wanted).await? {
        match read {
            Ok(bytes) => {
                if bytes.len() > MAX_FILE_BYTES {
                    warnings.push(format!(
                        "{path}: not indexed (larger than {MAX_FILE_BYTES} bytes)"
                    ));
                } else if bytes.contains(&0) || std::str::from_utf8(&bytes).is_err() {
                    warnings.push(format!("{path}: not indexed (binary or non-UTF8)"));
                }
                to_index.push((path, bytes));
            }
            Err(AvError::NotFound(_)) => to_remove.push(path),
            Err(e) => warnings.push(format!("{path}: unreadable ({e})")),
        }
    }
    if !to_remove.is_empty() {
        state.index.remove_paths(repo, scope, &to_remove).await?;
    }
    if !to_index.is_empty() {
        state
            .index
            .index_paths_at(repo, scope, Some(head), true, to_index)
            .await?;
    }
    // `index_paths_at` clears the recorded change; after an incremental update at `head` the
    // scope (main after merge/revert/undo) again fully reflects that change.
    state.index.set_scope_change(repo, scope, head).await?;
    Ok(warnings)
}

/// Opens a task overlay at `base` (usually B), then reconciles to `main`'s indexed tip when needed.
pub(crate) async fn init_task_overlay(
    state: &AppState,
    repo: &str,
    scope: &str,
    base: &ChangeId,
) -> Result<()> {
    state.index.ensure_overlay(repo, scope, base).await?;
    reconcile_overlay(state, repo, scope, base).await
}

/// Task head used for overlay writes: H after the first checkpoint, else B.
pub(crate) fn overlay_head(task: &Task) -> &ChangeId {
    if task.head_change.0.is_empty() {
        &task.base_change
    } else {
        &task.head_change
    }
}

/// True when the overlay's `main_at` lags `main`'s indexed tip (or either is missing).
pub(crate) async fn overlay_needs_reconcile(
    state: &AppState,
    repo: &str,
    scope: &str,
) -> Result<bool> {
    let Some(main_at) = state.index.overlay_main_at(repo, scope).await? else {
        return Ok(false);
    };
    let Some(t) = state.index.scope_change(repo, "main").await? else {
        return Ok(false);
    };
    Ok(main_at != t)
}

/// Reconcile this task's overlay to `main` before any index read. Caller holds `lock_task`.
pub(crate) async fn reconcile_task_overlay(state: &AppState, task: &Task) -> Result<()> {
    reconcile_overlay(state, &task.repo, &task.slug, overlay_head(task)).await
}

/// When `main_at` ≠ T (`main`'s `scope_change`), indexes `changed(main_at, T)` into the overlay
/// from `head` (unless already in O) and sets `main_at = T`. Repeats if main advanced mid-apply.
/// Caller holds `lock_task`.
pub(crate) async fn reconcile_overlay(
    state: &AppState,
    repo: &str,
    scope: &str,
    head: &ChangeId,
) -> Result<()> {
    // Main can advance (another task's merge) between reading T and writing main_at. Loop until
    // stable; a full merge↔reconcile advisory lock is out of scope for this change.
    for _ in 0..16 {
        let Some(main_at) = state.index.overlay_main_at(repo, scope).await? else {
            return Ok(());
        };
        let Some(t) = state.index.scope_change(repo, "main").await? else {
            return Ok(());
        };
        if main_at == t {
            return Ok(());
        }
        let changed = state.vcs.changed_paths(repo, &main_at, &t).await?;
        let existing: HashSet<String> = state
            .index
            .overlay_paths(repo, scope)
            .await?
            .into_iter()
            .collect();
        let needed: Vec<String> = changed
            .into_iter()
            .filter(|p| is_indexable(p) && !existing.contains(p))
            .collect();
        apply_overlay_from_head(state, repo, scope, head, &t, &needed, false).await?;
        state.index.set_overlay_main_at(repo, scope, &t).await?;
    }
    Ok(())
}

/// For paths that changed between `old_head` and `new_head`, update the overlay from H vs T:
/// leave O when H equals T, tombstone when deleted in H, else index from H.
pub(crate) async fn update_overlay_changed(
    state: &AppState,
    repo: &str,
    scope: &str,
    old_head: &ChangeId,
    new_head: &ChangeId,
) -> Result<Vec<String>> {
    reconcile_overlay(state, repo, scope, new_head).await?;
    let Some(t) = state.index.scope_change(repo, "main").await? else {
        return Ok(vec![]);
    };
    let delta = state.vcs.changed_paths(repo, old_head, new_head).await?;
    let wanted: Vec<String> = delta.into_iter().filter(|p| is_indexable(p)).collect();
    apply_overlay_from_head(state, repo, scope, new_head, &t, &wanted, true).await
}

/// After a sync rebase onto trunk: O becomes exactly `changed(T, head)`.
pub(crate) async fn replace_task_overlay(
    state: &AppState,
    repo: &str,
    scope: &str,
    head: &ChangeId,
) -> Result<Vec<String>> {
    let Some(t) = state.index.scope_change(repo, "main").await? else {
        return Err(AvError::Index(format!(
            "main of {repo} has no recorded change"
        )));
    };
    state.index.clear_overlay(repo, scope).await?;
    // ensure the overlay row exists (merge undo recreates after drop_scope)
    state.index.ensure_overlay(repo, scope, &t).await?;
    let changed = state.vcs.changed_paths(repo, &t, head).await?;
    let wanted: Vec<String> = changed.into_iter().filter(|p| is_indexable(p)).collect();
    // every path in changed(T, H) differs; no leave-O check
    let warnings = apply_overlay_from_head(state, repo, scope, head, &t, &wanted, false).await?;
    state.index.set_overlay_main_at(repo, scope, &t).await?;
    Ok(warnings)
}

/// Rebuilds a task's overlay after undo/revert restored its scope.
pub(crate) async fn rebuild_task_overlay(state: &AppState, task: &Task) -> Result<()> {
    replace_task_overlay(state, &task.repo, &task.slug, overlay_head(task)).await?;
    Ok(())
}

/// Indexes / tombstones / clears `paths` in the overlay by comparing `head` to `main_at` (`t`).
/// When `leave_when_equal`, a path whose H content equals T leaves O.
async fn apply_overlay_from_head(
    state: &AppState,
    repo: &str,
    scope: &str,
    head: &ChangeId,
    t: &ChangeId,
    paths: &[String],
    leave_when_equal: bool,
) -> Result<Vec<String>> {
    if paths.is_empty() {
        return Ok(vec![]);
    }
    let mut warnings = vec![];
    let mut to_upsert = vec![];
    let mut to_tomb = vec![];
    let mut to_clear = vec![];
    let h_reads = state.vcs.read_files(repo, head, paths).await?;
    let t_reads = state.vcs.read_files(repo, t, paths).await?;
    let t_by_path: std::collections::HashMap<_, _> = t_reads.into_iter().collect();
    for (path, h) in h_reads {
        let t_bytes = match t_by_path.get(&path) {
            Some(Ok(b)) => Some(b.as_slice()),
            Some(Err(AvError::NotFound(_))) | None => None,
            Some(Err(e)) => {
                warnings.push(format!("{path}: main unreadable ({e})"));
                continue;
            }
        };
        match h {
            Ok(bytes) => {
                if leave_when_equal && t_bytes == Some(bytes.as_slice()) {
                    to_clear.push(path);
                    continue;
                }
                if bytes.len() > MAX_FILE_BYTES {
                    warnings.push(format!(
                        "{path}: not indexed (larger than {MAX_FILE_BYTES} bytes)"
                    ));
                } else if bytes.contains(&0) || std::str::from_utf8(&bytes).is_err() {
                    warnings.push(format!("{path}: not indexed (binary or non-UTF8)"));
                }
                to_upsert.push((path, bytes));
            }
            Err(AvError::NotFound(_)) => {
                if leave_when_equal && t_bytes.is_none() {
                    // neither tree has it — nothing to mask
                    to_clear.push(path);
                } else {
                    to_tomb.push(path);
                }
            }
            Err(e) => warnings.push(format!("{path}: unreadable ({e})")),
        }
    }
    if !to_clear.is_empty() {
        state
            .index
            .clear_overlay_paths(repo, scope, &to_clear)
            .await?;
    }
    if !to_tomb.is_empty() {
        state
            .index
            .tombstone_overlay_paths(repo, scope, &to_tomb)
            .await?;
    }
    if !to_upsert.is_empty() {
        state
            .index
            .upsert_overlay_paths(repo, scope, to_upsert)
            .await?;
    }
    Ok(warnings)
}
