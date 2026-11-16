//! `.av-task`: written into a task's workspace so later commands (from any subdirectory) know the
//! task. The daemon writes it when it hands a workspace out (a start, a claim, a resume), and the
//! CLI when it starts or takes a task and in a remote checkout; both use this one definition.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub const NAME: &str = ".av-task";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskFile {
    pub task: String,
    pub session: String,
    /// The claim generation this workspace's owner writes under, and that owner. A later claim
    /// rewrites both. The CLI writes its own name, which fences off a previous owner's writes
    /// from this directory; the daemon writes no name (F14), and then a previous owner's hooks
    /// there write under the new generation.
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
    /// The agent session whose Stop hooks `av hook stop` has continued because av had work for
    /// it, and how many times (`AV_HOOK_MAX_CONTINUES` caps it). A new session starts at zero.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continue_session: Option<String>,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub continues: u32,
    /// Plan 14: set in a checkout on another host than the daemon's (`av remote clone`), whose
    /// checkpoints go to the daemon as patches.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote: Option<Remote>,
}

impl TaskFile {
    /// The file of a workspace just handed out under `generation`, naming no agent and with
    /// nothing else set (the daemon's, F14).
    pub fn new(task: String, session: String, generation: i64) -> Self {
        Self {
            task,
            session,
            generation: Some(generation),
            agent: None,
            last_checkpoint_ms: None,
            nudged_session: None,
            continue_session: None,
            continues: 0,
            remote: None,
        }
    }
}

/// Where a remote checkout last agreed with the daemon.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Remote {
    /// The daemon's head (a change id) the checkout was last synced to.
    pub base: String,
    /// The checkout's own git commit with exactly that head's files.
    pub commit: String,
}

fn is_zero(n: &u32) -> bool {
    *n == 0
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
