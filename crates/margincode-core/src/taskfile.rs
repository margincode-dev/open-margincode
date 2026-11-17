//! `.av-task`: written into a task's workspace so later commands (from any subdirectory) know the
//! task. The daemon and the CLI share this one definition.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub const NAME: &str = ".av-task";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskFile {
    pub task: String,
    pub session: String,
    /// The claim generation this workspace's owner writes under, and that owner. A later claim
    /// rewrites both. The CLI writes its own name; the daemon writes one only for an agent that
    /// named itself (`X-Av-Agent`), else none. `claim_of` says what a reader may send.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    /// Other names of `agent` (its profile's aliases, written by the daemon): a reader under one
    /// of them is the owner too.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_checkpoint_ms: Option<u64>,
    /// The agent session `av hook stop` last asked to record its changes; it asks once per session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nudged_session: Option<String>,
    /// The agent session `av hook stop` last told that its claim was taken over; it tells
    /// once per session. Not `nudged_session`: a previous owner's mark must not stop the new
    /// owner's record nudge.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub told_session: Option<String>,
    /// The agent session whose Stop hooks `av hook stop` has continued because av had work for
    /// it, and how many times (`AV_HOOK_MAX_CONTINUES` caps it). A new session starts at zero.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continue_session: Option<String>,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub continues: u32,
    /// Set in a checkout on another host than the daemon's (`av remote clone`), whose
    /// checkpoints go to the daemon as patches.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote: Option<Remote>,
}

impl TaskFile {
    /// The file of a workspace just handed out under `generation`, naming no agent.
    pub fn new(task: String, session: String, generation: i64) -> Self {
        Self {
            task,
            session,
            generation: Some(generation),
            agent: None,
            aliases: Vec::new(),
            last_checkpoint_ms: None,
            nudged_session: None,
            told_session: None,
            continue_session: None,
            continues: 0,
            remote: None,
        }
    }

    /// The owner the file names: none when it names nobody or the default `agent`, the name
    /// every unnamed CLI and hook acts as (it tells nobody apart).
    pub fn owner(&self) -> Option<&str> {
        self.agent
            .as_deref()
            .filter(|a| !a.trim().is_empty() && *a != DEFAULT_AGENT)
    }
}

pub const DEFAULT_AGENT: &str = "agent";

#[derive(Debug, PartialEq, Eq)]
pub enum Claim<'a> {
    /// The file's generation: the reader is the owner, or one of the two has no name.
    Holds(Option<i64>),
    /// The reader has a name, and the file names another owner: the reader's claim was taken
    /// over (or it never had one). Its writes from this directory are refused.
    TakenBy(&'a str),
}

/// The one rule for what a reader sends, given its explicit name (`None`: no `--agent`, no
/// `AV_AGENT`, not a Grok hook). Names fence only when both sides are known: an unnamed reader
/// cannot be told from the owner's own hooks, and a file that names nobody cannot tell who
/// owns it. A reader named as the owner or one of its aliases holds the claim.
pub fn claim_of<'a>(tf: &'a TaskFile, reader: Option<&str>) -> Claim<'a> {
    match (reader, tf.owner()) {
        (Some(r), Some(owner)) if r != owner && !tf.aliases.iter().any(|a| a == r) => {
            Claim::TakenBy(owner)
        }
        _ => Claim::Holds(tf.generation),
    }
}

/// What a fenced reader is told, after `av: ` (the CLI's refusal, the hooks' notice).
pub fn taken_over(owner: &str, reader: &str) -> String {
    format!("`{owner}` now owns this task; you are `{reader}`. Stop editing this workspace.")
}

/// The other names `owner` goes by, for a file's `aliases`: the profile it names or else is an
/// alias of (given as `(name, aliases)`; a profile's own name wins, as in `routing::resolve`),
/// that profile's name and aliases, `owner` left out. None when no profile knows it. A task stores the name it was started or claimed under, which may be an
/// alias (only `next` resolves one), so an owner under any of the profile's names holds the claim.
pub fn other_names<'a>(
    owner: &str,
    profiles: impl IntoIterator<Item = (&'a str, &'a [String])>,
) -> Vec<String> {
    let profiles: Vec<(&str, &[String])> = profiles.into_iter().collect();
    let Some((name, aliases)) = profiles
        .iter()
        .find(|(name, _)| *name == owner)
        .or_else(|| {
            profiles
                .iter()
                .find(|(_, aliases)| aliases.iter().any(|a| a == owner))
        })
        .copied()
    else {
        return Vec::new();
    };
    let mut names: Vec<String> = Vec::new();
    for n in std::iter::once(name).chain(aliases.iter().map(String::as_str)) {
        if n != owner && !names.iter().any(|m| m == n) {
            names.push(n.to_owned());
        }
    }
    names
}

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
/// debounce stamp and counts): writing back its copy would undo a claim made since, and the new
/// owner would be refused as "claim lost". A file that changed hands, or is gone or unreadable,
/// is left alone. A claim that lands between this read and the write is still lost; that window
/// is one small file's read and write.
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
    fn names_fence_only_when_both_sides_are_known() {
        let mut tf = TaskFile::new("t".into(), "s".into(), 3);
        assert_eq!(claim_of(&tf, Some("grok")), Claim::Holds(Some(3)));
        tf.agent = Some(DEFAULT_AGENT.into());
        assert_eq!(claim_of(&tf, Some("grok")), Claim::Holds(Some(3)));
        tf.agent = Some("claude-opus".into());
        tf.aliases = vec!["opus".into()];
        // an unnamed reader may be the owner's own hook
        assert_eq!(claim_of(&tf, None), Claim::Holds(Some(3)));
        assert_eq!(claim_of(&tf, Some("claude-opus")), Claim::Holds(Some(3)));
        assert_eq!(claim_of(&tf, Some("opus")), Claim::Holds(Some(3)));
        assert_eq!(claim_of(&tf, Some("grok")), Claim::TakenBy("claude-opus"));
    }

    #[test]
    fn an_owner_stored_as_an_alias_goes_by_its_profiles_names() {
        let opus = vec!["opus".to_owned(), "claude".to_owned()];
        let grok: Vec<String> = Vec::new();
        let profiles = || [("claude-opus", opus.as_slice()), ("grok", grok.as_slice())];
        assert_eq!(other_names("claude-opus", profiles()), ["opus", "claude"]);
        assert_eq!(other_names("opus", profiles()), ["claude-opus", "claude"]);
        assert!(other_names("grok", profiles()).is_empty());
        assert!(other_names("codex", profiles()).is_empty());
        // a profile's own name wins over another profile, listed first, that has it as an alias
        let aaa = vec!["claude".to_owned()];
        let listed = [("aaa", aaa.as_slice()), ("claude", &opus[..1])];
        assert_eq!(other_names("claude", listed), ["opus"]);
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
