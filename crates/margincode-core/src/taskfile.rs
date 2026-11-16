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

/// Applies `change` to the file in `dir` as it is now, and writes it, only while it still holds
/// `held`'s task and generation. Returns whether it wrote.
///
/// For a process that read the file earlier and keeps its own fields there (the hooks' session,
/// debounce stamp and counts): writing back the copy it read would undo a claim made since,
/// putting the previous owner's generation back, and the new owner would be refused as "claim
/// lost" (F39). A file that changed hands, or is gone or unreadable, is left alone. A claim that
/// lands between this read and the write is still lost; that window is one small file's read
/// and write.
pub fn update(
    dir: &Path,
    held: &TaskFile,
    change: impl FnOnce(&mut TaskFile),
) -> std::io::Result<bool> {
    let Ok(text) = std::fs::read_to_string(dir.join(NAME)) else {
        return Ok(false);
    };
    let Ok(mut now) = serde_json::from_str::<TaskFile>(&text) else {
        return Ok(false);
    };
    if now.task != held.task || now.generation != held.generation {
        return Ok(false);
    }
    change(&mut now);
    write(dir, &now)?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dir(name: &str) -> PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let d =
            std::env::temp_dir().join(format!("av-taskfile-{name}-{}-{nanos}", std::process::id()));
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn update_changes_the_file_as_it_is_now_while_it_holds_the_same_claim() {
        let d = dir("same");
        let held = TaskFile::new("t".into(), "s1".into(), 1);
        let mut resumed = held.clone();
        resumed.session = "s2".into();
        write(&d, &resumed).unwrap();
        assert!(update(&d, &held, |tf| tf.continues = 3).unwrap());
        let now = find(&d).unwrap().1;
        assert_eq!((now.session.as_str(), now.continues), ("s2", 3));
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn update_leaves_a_file_that_changed_hands_or_is_gone() {
        let d = dir("claimed");
        let held = TaskFile::new("t".into(), "s1".into(), 1);
        let mut claimed = TaskFile::new("t".into(), "s9".into(), 2);
        claimed.agent = Some("codex".into());
        write(&d, &claimed).unwrap();
        assert!(!update(&d, &held, |tf| tf.continues = 3).unwrap());
        let now = find(&d).unwrap().1;
        assert_eq!((now.generation, now.continues), (Some(2), 0));
        let other = TaskFile::new("other".into(), "s1".into(), 2);
        assert!(!update(&d, &other, |tf| tf.continues = 3).unwrap());
        std::fs::remove_file(d.join(NAME)).unwrap();
        assert!(!update(&d, &held, |tf| tf.continues = 3).unwrap());
        assert!(!d.join(NAME).exists());
        std::fs::remove_dir_all(&d).unwrap();
    }
}
