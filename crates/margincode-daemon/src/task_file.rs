//! The `.av-task` file the daemon writes into every workspace it hands out (start, claim, resume, and one re-created
//! by an undo or revert), so hooks and `av` commands there find their task. Snapshots and patches leave it out
//! (`AGENT_IGNORES` in `margincode-vcs`); a failed write is logged and never fails the hand-out.
//!
//! The file names its owner only when the agent named itself (`NamedByAgent`), with its profile's other names, in the
//! same write that hands out the generation: a reader that saw the new generation has seen the name too, so no named
//! previous owner gets a write in under it. Otherwise it names none: the name `av` and the hooks act as is unknown to
//! the daemon (Claude Code's hooks act as `agent` while its MCP session is named `claude-code`), and a wrong name would
//! fence the owner. Names fence only when both sides are known (`margincode_core::taskfile::claim_of`), so after a
//! claim a previous owner with no name still writes under the new generation (its MCP tools are still fenced). The CLI
//! writes its own name when it starts or takes a task.

use std::io::{Read, Write};
use std::path::Path;

use margincode_core::Task;
use margincode_core::taskfile::{NAME, TaskFile};

use crate::AppState;

/// More than any task file needs; a longer one is not read.
const MAX_READ: u64 = 64 * 1024;

/// Writes the file of `task` under `generation` into the workspace at `dir` (a path of this
/// process, not a host path), with `session`. A file already there for the same task and
/// generation (a resume) keeps everything else it holds: the name the CLI wrote, and the hooks'
/// counters, so the Stop hook's cap on continues still counts across a resume. A claim raises the
/// generation, so its file starts afresh. `owner`, when the agent named itself, is written as the
/// file's owner.
pub(crate) fn write(
    st: &AppState,
    dir: &Path,
    task: &str,
    session: String,
    generation: i64,
    owner: Option<&Owner>,
) {
    if let Err(e) = try_write(st, dir, task, session, generation, owner) {
        tracing::warn!(task, dir = %dir.display(), error = %e, "could not write {NAME}");
    }
}

fn try_write(
    st: &AppState,
    dir: &Path,
    task: &str,
    session: String,
    generation: i64,
    owner: Option<&Owner>,
) -> std::io::Result<()> {
    if !dir.starts_with(&st.ws_root) || !dir.is_dir() {
        return Err(std::io::Error::other(
            "not a workspace directory of this daemon",
        ));
    }
    let path = dir.join(NAME);
    let mut tf = match read_existing(&path) {
        Some(old) if old.task == task && old.generation == Some(generation) => TaskFile {
            session,
            remote: None,
            ..old
        },
        _ => TaskFile::new(task.to_owned(), session, generation),
    };
    if let Some(owner) = owner {
        tf.agent = Some(owner.name.clone());
        tf.aliases = owner.aliases.clone();
    }
    put(&path, &tf)
}

/// A request's agent named itself (an MCP session named by `X-Av-Agent`), rather than being
/// named after its client program: the workspace the request is handed then names its owner
/// (`owner_of`). The MCP layer adds it to each in-process request of such a session.
#[derive(Clone, Copy, Debug)]
pub(crate) struct NamedByAgent;

/// The header a client sends on a claim or `next` to have the file it is handed name the owner,
/// as `NamedByAgent` does: `av claim` and `av next` send it, since they write that same name
/// into the file themselves once answered. Named in the daemon's own write, the new
/// generation is never on disk unnamed for a named previous owner to read.
pub(crate) const NAME_OWNER_HEADER: &str = "x-av-name-owner";

pub(crate) fn names_owner(ext: &axum::http::Extensions, headers: &axum::http::HeaderMap) -> bool {
    ext.get::<NamedByAgent>().is_some() || headers.contains_key(NAME_OWNER_HEADER)
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Owner {
    pub(crate) name: String,
    pub(crate) aliases: Vec<String>,
}

/// The owner of `task` as a file names it, for an agent that named itself: its hooks and `av` act
/// under that name too (`av watch` sets `AV_AGENT` to it, Grok's hooks act as `grok`), so the file
/// fences a named previous owner from the workspace and never the owner. The task's owner may be
/// an alias it was started or claimed under, so the aliases are the profile's other names
/// (`margincode_core::taskfile::other_names`). None, so the file names nobody, when the profiles
/// cannot be read: naming the owner without its other names could fence it.
pub(crate) async fn owner_of(st: &AppState, task: &Task) -> Option<Owner> {
    let profiles = match st.store.list_profiles().await {
        Ok(profiles) => profiles,
        Err(e) => {
            tracing::warn!(task = %task.slug, error = %e, "could not name the owner in {NAME}");
            return None;
        }
    };
    let aliases = margincode_core::taskfile::other_names(
        &task.owner_agent,
        profiles
            .iter()
            .map(|p| (p.name.as_str(), p.aliases.as_slice())),
    );
    Some(Owner {
        name: task.owner_agent.clone(),
        aliases,
    })
}

/// Names the owner (`owner_of`) in the file a start just wrote, for an agent that named itself.
/// A claim and a resume name it in their own write; a start can name it after, as no previous
/// owner is in a new workspace. Only into the file of the task's current generation. A failure is
/// logged; the file then names nobody.
pub(crate) async fn name_owner(st: &AppState, id: margincode_core::TaskId) {
    let _guard = st.lock_task(id).await;
    let task = match st.store.get_task(id).await {
        Ok(task) => task,
        Err(e) => {
            tracing::warn!(task = %id.0, error = %e, "could not name the owner in {NAME}");
            return;
        }
    };
    let Some(owner) = owner_of(st, &task).await else {
        return;
    };
    let dir = st.to_local_path(&task.workspace_path);
    if let Err(e) = try_name(st, &dir, &task, &owner) {
        tracing::warn!(task = %task.slug, error = %e, "could not name the owner in {NAME}");
    }
}

fn try_name(st: &AppState, dir: &Path, task: &Task, owner: &Owner) -> std::io::Result<()> {
    if !dir.starts_with(&st.ws_root) || !dir.is_dir() {
        return Err(std::io::Error::other(
            "not a workspace directory of this daemon",
        ));
    }
    let path = dir.join(NAME);
    let Some(mut tf) = read_existing(&path)
        .filter(|old| old.task == task.id.to_string() && old.generation == Some(task.generation))
    else {
        return Ok(());
    };
    tf.agent = Some(owner.name.clone());
    tf.aliases = owner.aliases.clone();
    put(&path, &tf)
}

fn put(path: &Path, tf: &TaskFile) -> std::io::Result<()> {
    let text = serde_json::to_string(tf).map_err(std::io::Error::other)?;
    // Removed, then made anew: `create_new` follows no link planted at the name, and no
    // temporary file is left for a snapshot to take. On a start no client has the path yet; a
    // hook that reads it during a claim or a resume finds no task and does nothing. A hook that
    // writes it in between makes `create_new` fail once; the second try replaces its file.
    let mut tries = 0;
    loop {
        match std::fs::remove_file(path) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
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
