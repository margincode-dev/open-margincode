//! Host-wide slots: at most N of something at once on this machine, across every process that
//! shares `$AV_HOME`. A slot is a file `<dir>/slot-<i>.lock` held with an OS file lock. The OS
//! drops a lock when its process dies, so a killed process never leaks one.
//!
//! Two kinds: running agents (`av watch --max-agents`, `AV_MAX_AGENTS`, `$AV_HOME/agents`)
//! and running builds (`av validate --max-builds`, `AV_MAX_BUILDS`, `$AV_HOME/builds`). Anything
//! else that starts agents or runs checks on the host takes the same slots and so shares one
//! budget with them; a process in a sandbox should be counted by whoever started it, outside.
//!
//! A freed slot can stay held for a moment: a child the same process forks just then shares the
//! slot file's lock until it execs (the file is close-on-exec, so never longer). `wait_for` polls,
//! so it only waits that moment more; a single `take` may see the slot as busy.

use std::ffi::OsString;
use std::fs::{File, OpenOptions, TryLockError};
use std::path::{Path, PathBuf};
use std::time::Duration;

/// What a slot counts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Agent CLIs running.
    Agents,
    /// Builds (validations: the command and the required checks) running.
    Builds,
}

impl Kind {
    /// The env var that sets how many: `AV_MAX_AGENTS` or `AV_MAX_BUILDS`.
    pub fn var(self) -> &'static str {
        match self {
            Kind::Agents => "AV_MAX_AGENTS",
            Kind::Builds => "AV_MAX_BUILDS",
        }
    }

    /// The slot dir under an av home: `<av_home>/agents` or `<av_home>/builds`.
    pub fn dir_in(self, av_home: &Path) -> PathBuf {
        av_home.join(match self {
            Kind::Agents => "agents",
            Kind::Builds => "builds",
        })
    }

    /// This host's slot dir: `dir_in(av_home())`.
    pub fn host_dir(self) -> PathBuf {
        self.dir_in(&av_home())
    }

    /// The limit its env var sets; `None` when unset or empty (no limit).
    pub fn limit_from_env(self) -> Result<Option<u32>, String> {
        let v = std::env::var(self.var()).ok();
        limit(v.as_deref()).map_err(|e| format!("{}: {e}", self.var()))
    }
}

/// A limit as written in an env var: `None` when unset or empty, else a count of at least 1.
pub fn limit(value: Option<&str>) -> Result<Option<u32>, String> {
    match value.map(str::trim).filter(|v| !v.is_empty()) {
        None => Ok(None),
        Some(v) => match v.parse::<u32>() {
            Ok(n) if n >= 1 => Ok(Some(n)),
            _ => Err(format!("{v:?} is not a count of 1 or more")),
        },
    }
}

/// av's own directory: `$AV_HOME`, else `.scholion` in the user's home (the daemon and the CLI
/// use the same default).
pub fn av_home() -> PathBuf {
    av_home_from(|k| std::env::var_os(k))
}

/// `av_home` with the environment given, for tests.
pub fn av_home_from(var: impl Fn(&str) -> Option<OsString>) -> PathBuf {
    match var("AV_HOME").filter(|v| !v.is_empty()) {
        Some(v) => PathBuf::from(v),
        None => home_from(var)
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".scholion"),
    }
}

/// The user's home: `$HOME`, else `%USERPROFILE%` (Windows sets no `HOME` outside Git Bash).
pub fn home_dir() -> Option<PathBuf> {
    home_from(|k| std::env::var_os(k))
}

/// `home_dir` with the environment given, for tests.
pub fn home_from(var: impl Fn(&str) -> Option<OsString>) -> Option<PathBuf> {
    ["HOME", "USERPROFILE"]
        .into_iter()
        .find_map(|k| var(k).filter(|v| !v.is_empty()))
        .map(PathBuf::from)
}

/// Takes a free one of `n` slots in `dir`: the open, locked slot file, held until dropped.
/// `None` when all `n` are held.
pub fn take(dir: &Path, n: u32) -> std::io::Result<Option<File>> {
    std::fs::create_dir_all(dir)?;
    for i in 0..n {
        let f = OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(dir.join(format!("slot-{i}.lock")))?;
        match f.try_lock() {
            Ok(()) => return Ok(Some(f)),
            Err(TryLockError::WouldBlock) => {}
            Err(TryLockError::Error(e)) => return Err(e),
        }
    }
    Ok(None)
}

/// Waits until one of `n` slots in `dir` is free and takes it, looking every `poll`.
/// `on_full` runs once, the first time all are held (to say that it waits).
pub fn wait_for(
    dir: &Path,
    n: u32,
    poll: Duration,
    on_full: impl FnOnce(),
) -> std::io::Result<File> {
    let mut on_full = Some(on_full);
    loop {
        if let Some(slot) = take(dir, n)? {
            return Ok(slot);
        }
        if let Some(say) = on_full.take() {
            say();
        }
        std::thread::sleep(poll);
    }
}
