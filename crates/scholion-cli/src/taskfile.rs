//! `.av-task`: written into a task's workspace so later commands (from any subdirectory) know the task.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub const NAME: &str = ".av-task";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskFile {
    pub task: String,
    pub session: String,
    /// The claim generation this workspace's owner writes under, and that owner. A later claim
    /// rewrites both, which fences off the previous owner's writes from this directory.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    /// When `av hook post-edit` last checkpointed (Unix ms), for its debounce.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_checkpoint_ms: Option<u64>,
    /// The agent session `av hook stop` last asked to record its changes; it asks once per session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nudged_session: Option<String>,
}

/// Nearest `.av-task` in `start` or any ancestor, with the directory that holds it.
pub fn find(start: &Path) -> Option<(PathBuf, TaskFile)> {
    start.ancestors().find_map(|dir| {
        let text = std::fs::read_to_string(dir.join(NAME)).ok()?;
        Some((dir.to_path_buf(), serde_json::from_str(&text).ok()?))
    })
}

pub fn write(dir: &Path, tf: &TaskFile) -> std::io::Result<()> {
    std::fs::write(
        dir.join(NAME),
        serde_json::to_string(tf).expect("serializable"),
    )
}
