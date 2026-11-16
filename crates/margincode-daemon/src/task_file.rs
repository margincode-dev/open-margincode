//! The `.av-task` the daemon writes into a workspace it hands out (F14): on a start, a start
//! from the backlog, a claim and a resume, so hooks and `av` commands in a workspace an agent got
//! over MCP or HTTP find their task. Snapshots and patches leave the file out (`AGENT_IGNORES`
//! in `margincode-vcs`). A failed write is logged and never fails the hand-out.
//!
//! The file names no agent. Its `agent` is the name `av` and the hooks in that directory act as,
//! which the daemon does not know: Claude Code's hooks act as `agent` (no `AV_AGENT`) while its
//! MCP session is named after the client (`claude-code`), and a file naming that owner would make
//! the CLI send generation 0, refused as "claim lost", for the owner's own writes. The cost: after
//! a claim, a previous owner's hooks still running in that directory write under the new
//! generation and are accepted (the MCP tools of its session are still fenced). The CLI writes
//! its own name when it starts or takes a task.

use std::io::{Read, Write};
use std::path::Path;

use margincode_core::taskfile::{NAME, TaskFile};

use crate::AppState;

/// More than any task file needs; a longer one is not read.
const MAX_READ: u64 = 64 * 1024;

/// Writes the file of `task` under `generation` into the workspace at `dir` (a path of this
/// process, not a host path), with `session`. A file already there for the same task and
/// generation (a resume) keeps everything else it holds: the name the CLI wrote, and the hooks'
/// counters, so the Stop hook's cap on continues still counts across a resume. A claim raises the
/// generation, so its file starts afresh.
pub(crate) fn write(st: &AppState, dir: &Path, task: &str, session: String, generation: i64) {
    if let Err(e) = try_write(st, dir, task, session, generation) {
        tracing::warn!(task, dir = %dir.display(), error = %e, "could not write {NAME}");
    }
}

fn try_write(
    st: &AppState,
    dir: &Path,
    task: &str,
    session: String,
    generation: i64,
) -> std::io::Result<()> {
    if !dir.starts_with(&st.ws_root) || !dir.is_dir() {
        return Err(std::io::Error::other(
            "not a workspace directory of this daemon",
        ));
    }
    let path = dir.join(NAME);
    let tf = match read_existing(&path) {
        Some(old) if old.task == task && old.generation == Some(generation) => TaskFile {
            session,
            remote: None,
            ..old
        },
        _ => TaskFile::new(task.to_owned(), session, generation),
    };
    let text = serde_json::to_string(&tf).map_err(std::io::Error::other)?;
    // Removed, then made anew: `create_new` follows no link planted at the name, and no
    // temporary file is left for a snapshot to take. On a start no client has the path yet; a
    // hook that reads it during a claim or a resume finds no task and does nothing. A hook that
    // writes it in between makes `create_new` fail once; the second try replaces its file.
    let mut tries = 0;
    loop {
        match std::fs::remove_file(&path) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
        {
            Ok(mut f) => return f.write_all(text.as_bytes()),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists && tries == 0 => tries += 1,
            Err(e) => return Err(e),
        }
    }
}

/// The task file at `path` when it is a regular file of at most `MAX_READ` bytes. A link, a FIFO
/// or a device planted at the name (by the agent, or tracked by the repository) is not followed
/// or read: it would block the claim, which holds the task's lock, or fill memory.
fn read_existing(path: &Path) -> Option<TaskFile> {
    let meta = std::fs::symlink_metadata(path).ok()?;
    if !meta.file_type().is_file() || meta.len() > MAX_READ {
        return None;
    }
    let mut open = std::fs::OpenOptions::new();
    open.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        // a link or a FIFO swapped in after the check: refused, and opening does not wait
        open.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
    }
    let f = open.open(path).ok()?;
    if !f.metadata().ok()?.is_file() {
        return None;
    }
    let mut text = String::new();
    f.take(MAX_READ).read_to_string(&mut text).ok()?;
    serde_json::from_str(&text).ok()
}
