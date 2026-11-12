//! Plan 14: a task's checkout on another host than the daemon's. The checkout is a plain git
//! repository the CLI makes (`av remote clone`); its commit `Remote::commit` holds exactly the
//! daemon's head `Remote::base`. A checkpoint sends `git diff` from that commit to the files as
//! they are now (`POST /v1/tasks/{id}/checkpoint?base=`); the daemon applies it to the workspace it
//! owns and snapshots it. When the daemon's head has moved (409), `pull` fetches the change since
//! the base (`GET /v1/tasks/{id}/patch?from=`), moves the checkout onto it and puts the local edits
//! back with a three-way apply; the checkpoint is then sent again.
//!
//! The CLI runs git here, on the agent's host; the daemon runs none of it.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

use serde_json::{Value, json};

use crate::client::{CliError, Client, Result};
use crate::taskfile::{self, Remote, TaskFile};

fn local(e: impl std::fmt::Display) -> CliError {
    CliError::Local(e.to_string())
}

/// Settings every git command here runs with: exact bytes (no line-ending or filter conversion),
/// git's own path quoting, an identity for the CLI's commits.
const GIT_CONFIG: &[&str] = &[
    "-c",
    "core.autocrlf=false",
    "-c",
    "core.safecrlf=false",
    "-c",
    "core.quotepath=on",
    "-c",
    "user.name=av",
    "-c",
    "user.email=av@localhost",
    "-c",
    "commit.gpgsign=false",
];

/// `git diff` as the daemon reads it (`margincode_vcs::patch`).
const DIFF: &[&str] = &[
    "diff",
    "--binary",
    "--full-index",
    "--no-renames",
    "--no-color",
    "--no-ext-diff",
    "--no-textconv",
    "--no-relative",
    "--src-prefix=a/",
    "--dst-prefix=b/",
];

struct Git<'a> {
    dir: &'a Path,
    /// `GIT_INDEX_FILE`, for building a commit without touching the checkout's own index.
    index: Option<PathBuf>,
}

impl<'a> Git<'a> {
    fn new(dir: &'a Path) -> Self {
        Git { dir, index: None }
    }

    fn command(&self, args: &[&str]) -> Command {
        let mut c = Command::new("git");
        c.current_dir(self.dir)
            .args(GIT_CONFIG)
            .args(args)
            .stdin(Stdio::null());
        if let Some(i) = &self.index {
            c.env("GIT_INDEX_FILE", i);
        }
        c
    }

    /// Runs git; its stdout, or an error with its stderr.
    fn run(&self, args: &[&str]) -> Result<Vec<u8>> {
        let out = self
            .command(args)
            .output()
            .map_err(|e| local(format!("cannot run git: {e}")))?;
        if !out.status.success() {
            return Err(local(format!(
                "git {}: {}",
                args.join(" "),
                String::from_utf8_lossy(&out.stderr).trim()
            )));
        }
        Ok(out.stdout)
    }

    fn line(&self, args: &[&str]) -> Result<String> {
        Ok(String::from_utf8_lossy(&self.run(args)?).trim().to_owned())
    }

    /// `git apply ARGS` of `patch` (written to a file in `.git`); whether it applied, and stderr.
    fn apply(&self, args: &[&str], patch: &[u8]) -> Result<(bool, String)> {
        let file = self.dir.join(".git/av-apply.diff");
        std::fs::write(&file, patch).map_err(local)?;
        let path = file.to_string_lossy().into_owned();
        let mut all = vec!["apply"];
        all.extend_from_slice(args);
        all.push(&path);
        let out = self
            .command(&all)
            .output()
            .map_err(|e| local(format!("cannot run git: {e}")))?;
        let _ = std::fs::remove_file(&file);
        Ok((
            out.status.success(),
            String::from_utf8_lossy(&out.stderr).trim().to_owned(),
        ))
    }
}

/// Holds the checkout's lock: hooks may checkpoint while a command does.
fn lock(dir: &Path) -> Result<std::fs::File> {
    let at = dir.join(".git/av-lock");
    margincode_agentrun::slots::wait_for(&at, 1, Duration::from_millis(100), || {})
        .map_err(|e| local(format!("locking {}: {e}", at.display())))
}

/// The task's head as a patch from `from` (the empty tree when `None`): (patch, head, tree).
fn fetch(
    client: &Client,
    task: &str,
    from: Option<&str>,
) -> Result<(Vec<u8>, String, Option<String>)> {
    let query: Vec<(&str, &str)> = from.map(|f| vec![("from", f)]).unwrap_or_default();
    let (bytes, headers) = client.get_bytes_with(
        &format!("/v1/tasks/{task}/patch"),
        &query,
        &["x-av-head", "x-av-tree"],
    )?;
    let head = headers
        .first()
        .cloned()
        .flatten()
        .ok_or_else(|| local("the daemon sent no X-Av-Head; it is older than this av"))?;
    Ok((bytes, head, headers.get(1).cloned().flatten()))
}

/// A commit of the checkout: `parent`'s files (none when `None`) with `patch` applied, built in a
/// separate index. Warns when its tree is not the daemon's `tree`.
fn commit_of(
    dir: &Path,
    parent: Option<&str>,
    patch: &[u8],
    head: &str,
    tree: Option<&str>,
) -> Result<String> {
    let index = dir.join(".git/av-index");
    let _ = std::fs::remove_file(&index);
    let git = Git {
        dir,
        index: Some(index.clone()),
    };
    match parent {
        Some(p) => git.run(&["read-tree", p])?,
        None => git.run(&["read-tree", "--empty"])?,
    };
    if !patch.is_empty() {
        let (ok, stderr) = git.apply(&["--cached", "--binary", "--whitespace=nowarn"], patch)?;
        if !ok {
            let _ = std::fs::remove_file(&index);
            return Err(local(format!(
                "the daemon's patch does not apply to this checkout ({stderr}); clone the task again"
            )));
        }
    }
    let mine = git.line(&["write-tree"])?;
    let _ = std::fs::remove_file(&index);
    if let Some(t) = tree
        && t != mine
    {
        eprintln!(
            "av: warning: this checkout's files at {} differ from the daemon's (tree {mine}, daemon {t})",
            short(head)
        );
    }
    let message = format!("av: {head}");
    let mut args = vec!["commit-tree", mine.as_str(), "-m", message.as_str()];
    if let Some(p) = parent {
        args.extend(["-p", p]);
    }
    git.line(&args)
}

fn short(id: &str) -> &str {
    &id[..id.len().min(12)]
}

/// `av remote clone`: makes `dir` a checkout of `task` at its head and writes its `.av-task`.
pub fn clone(client: &Client, task: &str, dir: &Path, agent: &str, session: &str) -> Result<Value> {
    let r = client.get(&format!("/v1/tasks/{task}"), &[])?;
    let t = &r["task"];
    let id = t["id"].as_str().unwrap_or(task).to_owned();
    if dir.exists() && std::fs::read_dir(dir).map_err(local)?.next().is_some() {
        return Err(local(format!("{} exists and is not empty", dir.display())));
    }
    std::fs::create_dir_all(dir).map_err(local)?;
    // git runs in `dir`, so every path handed to it (the separate index) must not be relative
    let dir = &std::path::absolute(dir).map_err(local)?;
    let git = Git::new(dir);
    git.run(&["init", "-q"])?;
    // as in a workspace: the task file and build output are never part of a change
    let info = dir.join(".git/info");
    std::fs::create_dir_all(&info).map_err(local)?;
    std::fs::write(info.join("exclude"), "/.av-task\n/target/\n").map_err(local)?;
    // the checkout's bytes are the daemon's bytes: no line-ending or filter conversion
    std::fs::write(info.join("attributes"), "* -text -filter -ident\n").map_err(local)?;
    let (patch, head, tree) = fetch(client, &id, None)?;
    let commit = commit_of(dir, None, &patch, &head, tree.as_deref())?;
    git.run(&["update-ref", "HEAD", &commit])?;
    git.run(&["reset", "-q", "--hard"])?;
    let tf = TaskFile {
        task: id.clone(),
        session: session.to_owned(),
        generation: t["generation"].as_i64(),
        agent: Some(agent.to_owned()),
        last_checkpoint_ms: None,
        nudged_session: None,
        continue_session: None,
        continues: 0,
        remote: Some(Remote {
            base: head.clone(),
            commit,
        }),
    };
    taskfile::write(dir, &tf).map_err(local)?;
    Ok(json!({"task": t, "head_change": head}))
}

/// What `pull` did.
pub struct Pulled {
    pub head: String,
    /// Whether the daemon's head had moved since the checkout's base.
    pub moved: bool,
    /// Files where the local edits and the daemon's changes met; they hold conflict markers.
    pub conflicts: Vec<String>,
}

/// `av remote pull`: moves the checkout onto the daemon's head, keeping its local edits.
pub fn pull(client: &Client, dir: &Path) -> Result<Pulled> {
    let _lock = lock(dir)?;
    pull_locked(client, dir)
}

fn read_tf(dir: &Path) -> Result<(TaskFile, Remote)> {
    let (_, tf) = taskfile::find(dir).ok_or(CliError::NoTask)?;
    let r = tf
        .remote
        .clone()
        .ok_or_else(|| local("not a remote checkout (make one with `av remote clone`)"))?;
    Ok((tf, r))
}

fn pull_locked(client: &Client, dir: &Path) -> Result<Pulled> {
    let (mut tf, r) = read_tf(dir)?;
    let git = Git::new(dir);
    let (patch, head, tree) = fetch(client, &tf.task, Some(&r.base))?;
    if head == r.base && patch.is_empty() {
        return Ok(Pulled {
            head,
            moved: false,
            conflicts: vec![],
        });
    }
    // the local edits, kept as a commit (so nothing is lost whatever happens next) and a patch
    git.run(&["add", "-A"])?;
    let now = git.line(&["write-tree"])?;
    let base_tree = git.line(&["rev-parse", &format!("{}^{{tree}}", r.commit)])?;
    let edits = if now != base_tree {
        let mut args = DIFF.to_vec();
        args.extend([r.commit.as_str(), now.as_str()]);
        let saved = git.line(&[
            "commit-tree",
            &now,
            "-p",
            &r.commit,
            "-m",
            "av: local edits",
        ])?;
        Some((git.run(&args)?, saved))
    } else {
        None
    };
    let commit = commit_of(dir, Some(&r.commit), &patch, &head, tree.as_deref())?;
    git.run(&["update-ref", "HEAD", &commit])?;
    git.run(&["reset", "-q", "--hard", &commit])?;
    tf.remote = Some(Remote {
        base: head.clone(),
        commit,
    });
    taskfile::write(dir, &tf).map_err(local)?;
    let mut conflicts = vec![];
    if let Some((edits, saved)) = edits {
        let (ok, stderr) = git.apply(&["--3way", "--binary", "--whitespace=nowarn"], &edits)?;
        if !ok {
            let unmerged = git.line(&["diff", "--name-only", "--diff-filter=U"])?;
            conflicts = unmerged.lines().map(str::to_owned).collect();
            if conflicts.is_empty() {
                return Err(local(format!(
                    "pulled to {} but your edits did not go back on ({stderr}); they are in commit {saved} (`git checkout {saved} -- .`)",
                    short(&head)
                )));
            }
        }
    }
    Ok(Pulled {
        head,
        moved: true,
        conflicts,
    })
}

/// A checkpoint from a remote checkout: sends the patch, and on a stale base pulls and sends again
/// once. Returns the daemon's answer, as a local checkpoint does.
pub fn checkpoint(client: &Client, dir: &Path, generation: Option<i64>) -> Result<Value> {
    let _lock = lock(dir)?;
    let mut pulled = false;
    loop {
        let (mut tf, r) = read_tf(dir)?;
        let git = Git::new(dir);
        git.run(&["add", "-A"])?;
        let tree = git.line(&["write-tree"])?;
        let mut args = vec!["diff", "--cached"];
        args.extend_from_slice(&DIFF[1..]);
        args.push(&r.commit);
        let patch = git.run(&args)?;
        let sent = client.post_bytes(
            &format!("/v1/tasks/{}/checkpoint", tf.task),
            &[("base", &r.base)],
            &patch,
            generation,
        );
        let v = match sent {
            Ok(v) => v,
            Err(CliError::Api {
                status: 409, body, ..
            }) if !pulled && body["head_change"].is_string() => {
                let p = pull_locked(client, dir)?;
                if !p.conflicts.is_empty() {
                    return Err(local(format!(
                        "the daemon's head moved to {} and changed lines you also changed in: {}; \
                         resolve the conflict markers there, then `av checkpoint` again",
                        short(&p.head),
                        p.conflicts.join(", ")
                    )));
                }
                pulled = true;
                continue;
            }
            Err(e) => return Err(e),
        };
        let head = v["head_change"].as_str().unwrap_or_default().to_owned();
        let theirs = v["tree"].as_str();
        let commit = if theirs == Some(tree.as_str()) {
            let message = format!("av: {head}");
            let c = git.line(&["commit-tree", &tree, "-p", &r.commit, "-m", &message])?;
            git.run(&["update-ref", "HEAD", &c])?;
            c
        } else {
            // the daemon kept something else (a file its ignores drop, a conflict): take its
            // files as the base and leave the difference as a local edit
            let (p, h, t) = fetch(client, &tf.task, Some(&r.base))?;
            let c = commit_of(dir, Some(&r.commit), &p, &h, t.as_deref())?;
            git.run(&["update-ref", "HEAD", &c])?;
            git.run(&["reset", "-q", &c])?;
            c
        };
        tf.remote = Some(Remote { base: head, commit });
        taskfile::write(dir, &tf).map_err(local)?;
        return Ok(v);
    }
}

/// Checkpoints `task`: from a remote checkout (the nearest `.av-task` from `cwd` says so) through
/// a patch, else by asking the daemon to snapshot its workspace.
pub fn checkpoint_task(
    client: &Client,
    task: &str,
    generation: Option<i64>,
    cwd: &Path,
) -> Result<Value> {
    if let Some((dir, tf)) = taskfile::find(cwd)
        && tf.task == task
        && tf.remote.is_some()
    {
        return checkpoint(client, &dir, generation);
    }
    client.post_as(
        &format!("/v1/tasks/{task}/checkpoint"),
        &json!({}),
        generation,
    )
}

/// Sends a remote checkout's edits before a command the daemon answers from its workspace
/// (submit, record, handoff, sync). Does nothing elsewhere.
pub fn push_if_remote(
    client: &Client,
    task: &str,
    generation: Option<i64>,
    cwd: &Path,
) -> Result<bool> {
    match taskfile::find(cwd) {
        Some((dir, tf)) if tf.task == task && tf.remote.is_some() => {
            checkpoint(client, &dir, generation)?;
            Ok(true)
        }
        _ => Ok(false),
    }
}
