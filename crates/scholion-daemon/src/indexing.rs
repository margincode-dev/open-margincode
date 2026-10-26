//! Feeds file contents from the VCS into the code index.

use scholion_core::{AvError, ChangeId, Result};
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
/// reflecting `change` (task start copies `main` only when it is recorded at the task's base).
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

/// Re-indexes `changed` paths at `head`. Returns human-readable warnings for files that could not be
/// indexed; those never fail the call.
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
    Ok(warnings)
}
