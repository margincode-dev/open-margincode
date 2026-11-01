//! jj-lib backed `Vcs`. Pinned to `jj-lib =0.46.0` (docs: https://docs.rs/jj-lib/0.46.0).
//! This is the only module allowed to import `jj_lib`.
//!
//! Layout under `store_root/<repo>/`:
//!   git/    bare clone of the imported git repo (backend of the jj repo)
//!   main/   the jj "default" workspace that owns `.jj/repo`
//!   trunk   name of the trunk bookmark (e.g. `main`)
//! Task workspaces live at `ws_root/<repo>/<slug>`.
//!
//! Concurrency: every repo has one `RepoLoader` (so one `Store`) shared by all reads and by every
//! workspace this process opens; writes therefore keep that store's cached commit metadata
//! current. Mutations (import, new workspace, snapshot) hold a per-repo mutex. Reads address
//! immutable commits by id and take no lock, so packs and indexing never wait behind a snapshot.
//!
//! `ChangeId` carries the jj *commit id* hex in v1 (immutable reads); stable
//! change-id semantics are deferred to Plan 3.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use futures::{AsyncReadExt as _, StreamExt as _};
use jj_lib::backend::TreeValue;
use jj_lib::commit::Commit;
use jj_lib::config::{ConfigLayer, ConfigSource, StackedConfig};
use jj_lib::default_backend_factories::{default_backend_factories, default_working_copy_factory};
use jj_lib::git::{self, GitImportOptions};
use jj_lib::gitignore::GitIgnoreFile;
use jj_lib::matchers::{EverythingMatcher, NothingMatcher};
use jj_lib::merge::Merge;
use jj_lib::merged_tree::MergedTree;
use jj_lib::object_id::ObjectId as _;
use jj_lib::op_store::RefTarget;
use jj_lib::ref_name::{RefName, WorkspaceNameBuf};
use jj_lib::repo::{ReadonlyRepo, Repo as _, RepoLoader};
use jj_lib::repo_path::RepoPath;
use jj_lib::rewrite::rebase_commit;
use jj_lib::settings::UserSettings;
use jj_lib::store::Store;
use jj_lib::working_copy::SnapshotOptions;
use jj_lib::workspace::Workspace;
use pollster::block_on;
use scholion_core::{
    AvError, ChangeId, LogEntry, MergeOutcome, RebaseOutcome, Result, Vcs, WorkspaceInfo,
};

/// Scholion's own per-workspace metadata (written by the `av` CLI); never part of a task's changes.
const AGENT_IGNORES: &[u8] = b"/.av-task\n";

/// Largest new file the working copy will snapshot (jj's own default is 1 MiB; agents write bigger).
const MAX_NEW_FILE_SIZE: u64 = 256 * 1024 * 1024;

fn verr<E: std::fmt::Display>(e: E) -> AvError {
    AvError::Vcs(e.to_string())
}

struct Inner {
    store_root: PathBuf,
    ws_root: PathBuf,
    locks: Mutex<HashMap<String, Arc<Mutex<()>>>>,
    loaders: Mutex<HashMap<String, RepoLoader>>,
}

#[derive(Clone)]
pub struct JjVcs {
    inner: Arc<Inner>,
}

impl JjVcs {
    pub fn new(store_root: PathBuf, ws_root: PathBuf) -> Self {
        Self {
            inner: Arc::new(Inner {
                store_root,
                ws_root,
                locks: Mutex::new(HashMap::new()),
                loaders: Mutex::new(HashMap::new()),
            }),
        }
    }

    /// Runs `f` on a blocking thread holding the per-repo lock (jj-lib futures are driven with `pollster`).
    async fn run<T: Send + 'static>(
        &self,
        repo: &str,
        f: impl FnOnce(&Inner) -> Result<T> + Send + 'static,
    ) -> Result<T> {
        valid_component("repo", repo)?;
        let inner = self.inner.clone();
        let lock = {
            let mut map = inner.locks.lock().unwrap_or_else(|e| e.into_inner());
            map.entry(repo.to_owned()).or_default().clone()
        };
        tokio::task::spawn_blocking(move || {
            let _guard = lock.lock().unwrap_or_else(|e| e.into_inner());
            f(&inner)
        })
        .await
        .map_err(verr)?
    }

    /// Runs a read of immutable data (addressed by commit id) on a blocking thread, without the
    /// per-repo lock.
    async fn read<T: Send + 'static>(
        &self,
        repo: &str,
        f: impl FnOnce(&Inner) -> Result<T> + Send + 'static,
    ) -> Result<T> {
        valid_component("repo", repo)?;
        let inner = self.inner.clone();
        tokio::task::spawn_blocking(move || f(&inner))
            .await
            .map_err(verr)?
    }
}

fn settings() -> Result<UserSettings> {
    let mut config = StackedConfig::with_defaults();
    let layer = ConfigLayer::parse(
        ConfigSource::User,
        "user.name = \"Scholion\"\nuser.email = \"scholion@localhost\"\n",
    )
    .map_err(verr)?;
    config.add_layer(layer);
    UserSettings::from_config_and_home_dir(config, None).map_err(verr)
}

fn git_cmd(dir: &Path, args: &[&str]) -> Result<String> {
    let out = Command::new("git")
        .current_dir(dir)
        .args(args)
        .output()
        .map_err(verr)?;
    if !out.status.success() {
        return Err(AvError::Vcs(format!(
            "git {args:?}: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        )));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_owned())
}

/// Longest a backup push may take before its git process is killed.
const PUSH_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(60);

/// `git_cmd` for network operations: no credential prompts, killed after `timeout`.
fn git_cmd_timeout(dir: &Path, args: &[&str], timeout: std::time::Duration) -> Result<String> {
    let mut child = Command::new("git")
        .current_dir(dir)
        .env("GIT_TERMINAL_PROMPT", "0")
        .args(args)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(verr)?;
    let deadline = std::time::Instant::now() + timeout;
    while child.try_wait().map_err(verr)?.is_none() {
        if std::time::Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            return Err(AvError::Vcs(format!(
                "git {args:?} timed out after {}s",
                timeout.as_secs()
            )));
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    let out = child.wait_with_output().map_err(verr)?;
    if !out.status.success() {
        return Err(AvError::Vcs(format!(
            "git {args:?}: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        )));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_owned())
}

/// Repo names and workspace slugs become directory names under the store / workspace roots, so they
/// must be a single plain path component. Unicode is fine (slugs derive from task titles).
fn valid_component(kind: &str, s: &str) -> Result<()> {
    let bad = s.is_empty()
        || s.len() > 100
        || s.starts_with('.')
        || s.chars().any(|c| c == '/' || c == '\\' || c.is_control());
    if bad {
        return Err(AvError::Invalid(format!("invalid {kind} name {s:?}")));
    }
    Ok(())
}

impl Inner {
    fn repo_dir(&self, repo: &str) -> PathBuf {
        self.store_root.join(repo)
    }

    /// `.jj/repo` of the repo's default workspace; every task workspace points at it.
    fn jj_repo_path(&self, repo: &str) -> PathBuf {
        self.repo_dir(repo).join("main").join(".jj").join("repo")
    }

    /// The repo's shared loader, opened on first use.
    fn loader(&self, repo: &str) -> Result<RepoLoader> {
        let mut map = self.loaders.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(l) = map.get(repo) {
            return Ok(l.clone());
        }
        let path = self.jj_repo_path(repo);
        if !path.is_dir() {
            return Err(AvError::NotFound(format!("repo {repo}")));
        }
        let loader =
            RepoLoader::init_from_file_system(&settings()?, &path, &default_backend_factories())
                .map_err(verr)?;
        map.insert(repo.to_owned(), loader.clone());
        Ok(loader)
    }

    fn store(&self, repo: &str) -> Result<Arc<Store>> {
        Ok(self.loader(repo)?.store().clone())
    }

    /// The repo at its latest operation (needed for the view: bookmarks, working-copy commits).
    fn head(&self, repo: &str) -> Result<Arc<ReadonlyRepo>> {
        block_on(self.loader(repo)?.load_at_head()).map_err(verr)
    }

    fn load_main(&self, repo: &str) -> Result<(Workspace, Arc<ReadonlyRepo>)> {
        self.load_workspace(repo, &self.repo_dir(repo).join("main"))
    }

    /// Opens a task workspace on the shared loader (as `Workspace::load` would, minus a new store).
    fn load_workspace(&self, repo: &str, root: &Path) -> Result<(Workspace, Arc<ReadonlyRepo>)> {
        let loader = self.loader(repo)?;
        let working_copy = default_working_copy_factory()
            .load_working_copy(
                loader.store().clone(),
                root.to_path_buf(),
                root.join(".jj").join("working_copy"),
                &settings()?,
            )
            .map_err(verr)?;
        let ws = Workspace::new(root, self.jj_repo_path(repo), working_copy, loader.clone())
            .map_err(verr)?;
        let ro = block_on(loader.load_at_head()).map_err(verr)?;
        Ok((ws, ro))
    }

    fn commit(&self, store: &Arc<Store>, change: &ChangeId) -> Result<Commit> {
        let id = jj_lib::backend::CommitId::try_from_hex(&change.0)
            .ok_or_else(|| AvError::Invalid(format!("bad change id {}", change.0)))?;
        store.get_commit(&id).map_err(verr)
    }

    fn commit_in_repo(&self, ro: &Arc<ReadonlyRepo>, change: &ChangeId) -> Result<Commit> {
        self.commit(ro.store(), change)
    }

    fn init_repo(&self, repo: &str, git_source: &Path) -> Result<ChangeId> {
        // Only an existing local directory is accepted: no URLs, no `ext::` transports, and once
        // canonicalised the path is absolute, so git can never mistake it for an option.
        let source = git_source
            .canonicalize()
            .ok()
            .filter(|p| p.is_dir())
            .ok_or_else(|| {
                AvError::Invalid(format!(
                    "git_source must be an existing local directory: {git_source:?}"
                ))
            })?;
        let dir = self.repo_dir(repo);
        if dir.exists() {
            return Err(AvError::Invalid(format!("repo {repo} already exists")));
        }
        let result = self.init_repo_in(&dir, repo, &source);
        // the import wrote through its own store; open the shared loader fresh afterwards
        self.loaders
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .remove(repo);
        if result.is_err() {
            // leave nothing behind, so a corrected retry is not blocked by "already exists"
            let _ = std::fs::remove_dir_all(&dir);
        }
        result
    }

    fn init_repo_in(&self, dir: &Path, repo: &str, source: &Path) -> Result<ChangeId> {
        std::fs::create_dir_all(dir.join("main")).map_err(verr)?;
        let git_dir = dir.join("git");
        git_cmd(
            dir,
            &[
                "clone",
                "--bare",
                "--quiet",
                "--",
                &source.to_string_lossy(),
                &git_dir.to_string_lossy(),
            ],
        )?;
        let trunk = git_cmd(&git_dir, &["symbolic-ref", "--short", "HEAD"])?;
        std::fs::write(dir.join("trunk"), &trunk).map_err(verr)?;

        let (_ws, repo_ro) = block_on(Workspace::init_external_git(
            &settings()?,
            &dir.join("main"),
            &git_dir,
        ))
        .map_err(verr)?;
        let mut tx = repo_ro.start_transaction();
        let opts = GitImportOptions {
            abandon_unreachable_commits: false,
            record_synthetic_predecessors: false,
            remote_auto_track_bookmarks: HashMap::new(),
        };
        block_on(git::import_refs(tx.repo_mut(), &opts)).map_err(verr)?;
        block_on(tx.repo_mut().rebase_descendants()).map_err(verr)?;
        let repo_ro = block_on(tx.commit("import git refs")).map_err(verr)?;
        Ok(ChangeId(self.trunk_commit(repo, &repo_ro)?.id().hex()))
    }

    fn trunk_name(&self, repo: &str) -> Result<String> {
        let name = std::fs::read_to_string(self.repo_dir(repo).join("trunk")).map_err(verr)?;
        Ok(name.trim().to_owned())
    }

    fn trunk_commit(&self, repo: &str, ro: &Arc<ReadonlyRepo>) -> Result<Commit> {
        let name = self.trunk_name(repo)?;
        let target = ro.view().get_local_bookmark(RefName::new(&name));
        let id = target.as_normal().ok_or_else(|| {
            AvError::Vcs(format!("trunk bookmark {name} is missing or conflicted"))
        })?;
        ro.store().get_commit(id).map_err(verr)
    }

    fn ws_dir(&self, repo: &str, slug: &str) -> PathBuf {
        self.ws_root.join(repo).join(slug)
    }

    fn create_workspace(&self, repo: &str, slug: &str) -> Result<WorkspaceInfo> {
        valid_component("workspace", slug)?;
        let ro = self.head(repo)?;
        let trunk = self.trunk_commit(repo, &ro)?;
        let dir = self.ws_dir(repo, slug);
        if dir.exists() {
            return Err(AvError::Invalid(format!("workspace {slug} already exists")));
        }
        std::fs::create_dir_all(&dir).map_err(verr)?;
        let name = WorkspaceNameBuf::from(slug);
        let (mut ws, ro) = block_on(Workspace::init_workspace_with_existing_repo(
            &dir,
            &self.jj_repo_path(repo),
            &ro,
            &*default_working_copy_factory(),
            name.clone(),
        ))
        .map_err(verr)?;
        let mut tx = ro.start_transaction();
        let wc = block_on(tx.repo_mut().check_out(name, &trunk)).map_err(verr)?;
        block_on(tx.repo_mut().rebase_descendants()).map_err(verr)?;
        let ro = block_on(tx.commit(format!("create workspace {slug}"))).map_err(verr)?;
        block_on(ws.check_out(ro.op_id().clone(), None, &wc)).map_err(verr)?;
        Ok(WorkspaceInfo {
            path: dir,
            base_change: ChangeId(trunk.id().hex()),
        })
    }

    fn snapshot(&self, repo: &str, slug: &str) -> Result<ChangeId> {
        valid_component("workspace", slug)?;
        let dir = self.ws_dir(repo, slug);
        if !dir.exists() {
            return Err(AvError::NotFound(format!("workspace {slug}")));
        }
        let (mut ws, ro) = self.load_workspace(repo, &dir)?;
        let name = ws.workspace_name().to_owned();
        let wc_id = ro
            .view()
            .get_wc_commit_id(&name)
            .ok_or_else(|| AvError::Vcs(format!("workspace {slug} has no working-copy commit")))?
            .clone();
        let wc_commit = ro.store().get_commit(&wc_id).map_err(verr)?;
        let mut locked = block_on(ws.start_working_copy_mutation()).map_err(verr)?;
        let options = SnapshotOptions {
            base_ignores: GitIgnoreFile::empty()
                .chain(RepoPath::root(), Path::new(""), AGENT_IGNORES)
                .map_err(verr)?,
            progress: None,
            start_tracking_matcher: &EverythingMatcher,
            force_tracking_matcher: &NothingMatcher,
            max_new_file_size: MAX_NEW_FILE_SIZE,
        };
        let (new_tree, _stats) = block_on(locked.locked_wc().snapshot(&options)).map_err(verr)?;
        let ro = if new_tree.tree_ids() != wc_commit.tree().tree_ids() {
            let mut tx = ro.start_transaction();
            let new_commit = block_on(
                tx.repo_mut()
                    .rewrite_commit(&wc_commit)
                    .set_tree(new_tree)
                    .write(),
            )
            .map_err(verr)?;
            tx.repo_mut()
                .set_wc_commit(name.clone(), new_commit.id().clone())
                .map_err(verr)?;
            block_on(tx.repo_mut().rebase_descendants()).map_err(verr)?;
            block_on(tx.commit("snapshot working copy")).map_err(verr)?
        } else {
            ro
        };
        block_on(locked.finish(ro.op_id().clone())).map_err(verr)?;
        let head = ro
            .view()
            .get_wc_commit_id(&name)
            .ok_or_else(|| AvError::Vcs("wc commit vanished".into()))?;
        Ok(ChangeId(head.hex()))
    }

    fn changed_paths(&self, repo: &str, base: &ChangeId, head: &ChangeId) -> Result<Vec<String>> {
        let store = self.store(repo)?;
        let (b, h) = (self.commit(&store, base)?, self.commit(&store, head)?);
        block_on(async {
            let mut stream = b.tree().diff_stream(&h.tree(), &EverythingMatcher);
            let mut paths = vec![];
            while let Some(entry) = stream.next().await {
                entry.values.map_err(verr)?;
                paths.push(entry.path.as_internal_file_string().to_owned());
            }
            Ok(paths)
        })
    }

    fn list_files(&self, repo: &str, change: &ChangeId) -> Result<Vec<String>> {
        let store = self.store(repo)?;
        let tree = self.commit(&store, change)?.tree();
        let mut files = vec![];
        for (path, value) in tree.entries() {
            if value.map_err(verr)?.is_present() {
                files.push(path.as_internal_file_string().to_owned());
            }
        }
        Ok(files)
    }

    fn read_files(
        &self,
        repo: &str,
        change: &ChangeId,
        paths: &[String],
    ) -> Result<Vec<(String, Result<Vec<u8>>)>> {
        let store = self.store(repo)?;
        let tree = self.commit(&store, change)?.tree();
        let read_one = |path: &str| -> Result<Vec<u8>> {
            let repo_path = RepoPath::from_internal_string(path).map_err(verr)?;
            block_on(async {
                let value = tree.path_value(repo_path).await.map_err(verr)?;
                match value.as_resolved() {
                    Some(Some(TreeValue::File { id, .. })) => {
                        let mut reader = store.read_file(repo_path, id).await.map_err(verr)?;
                        let mut buf = Vec::new();
                        reader.read_to_end(&mut buf).await.map_err(verr)?;
                        Ok(buf)
                    }
                    _ => Err(AvError::NotFound(format!("file {path} at {}", change.0))),
                }
            })
        };
        Ok(paths.iter().map(|p| (p.clone(), read_one(p))).collect())
    }
}

/// Paths of the conflicted files in `tree`.
fn conflict_paths(tree: &MergedTree) -> Vec<String> {
    let mut paths: Vec<String> = tree
        .conflicts()
        .map(|(path, _)| path.as_internal_file_string().to_owned())
        .collect();
    paths.sort();
    paths
}

impl Inner {
    fn trunk(&self, repo: &str) -> Result<ChangeId> {
        let (_ws, ro) = self.load_main(repo)?;
        Ok(ChangeId(self.trunk_commit(repo, &ro)?.id().hex()))
    }

    fn push_backup(&self, repo: &str, remote_url: &str, branch: Option<&str>) -> Result<ChangeId> {
        let url = remote_url.trim();
        if url.is_empty() || url.starts_with('-') || url.starts_with("ext::") {
            return Err(AvError::Invalid(format!(
                "backup remote {remote_url:?} is not a URL or path git may push to"
            )));
        }
        let name = self.trunk_name(repo)?;
        let git_dir = self.repo_dir(repo).join("git");
        let refname = format!("refs/heads/{name}");
        let target = match branch {
            Some(b) => {
                scholion_core::check_backup_branch(b)?;
                // git has the last word on what a branch name is
                git_cmd(&git_dir, &["check-ref-format", &format!("refs/heads/{b}")]).map_err(
                    |_| AvError::Invalid(format!("backup branch {b:?} is not a git branch name")),
                )?;
                format!("refs/heads/{b}")
            }
            None => refname.clone(),
        };
        let pushed = git_cmd(&git_dir, &["rev-parse", "--verify", &refname])?;
        // no `+` and no --force: a remote that moved elsewhere rejects the push
        git_cmd_timeout(
            &git_dir,
            &[
                "push",
                "--porcelain",
                "--",
                url,
                &format!("{refname}:{target}"),
            ],
            PUSH_TIMEOUT,
        )?;
        Ok(ChangeId(pushed))
    }

    fn trunk_log(&self, repo: &str, limit: usize) -> Result<Vec<LogEntry>> {
        let (_ws, ro) = self.load_main(repo)?;
        let mut commit = self.trunk_commit(repo, &ro)?;
        let root = ro.store().root_commit_id().clone();
        let mut out = vec![];
        while out.len() < limit && commit.id() != &root {
            out.push(LogEntry {
                change: ChangeId(commit.id().hex()),
                parents: commit
                    .parent_ids()
                    .iter()
                    .map(|p| ChangeId(p.hex()))
                    .collect(),
                description: commit.description().to_owned(),
            });
            let Some(first) = commit.parent_ids().first().cloned() else {
                break;
            };
            commit = ro.store().get_commit(&first).map_err(verr)?;
        }
        Ok(out)
    }

    /// Points trunk at `target`, exports it to the bare git dir and commits the operation.
    fn move_trunk(
        &self,
        repo: &str,
        ro: &Arc<ReadonlyRepo>,
        target: &Commit,
        op: &str,
    ) -> Result<()> {
        let name = self.trunk_name(repo)?;
        let mut tx = ro.start_transaction();
        tx.repo_mut()
            .set_local_bookmark_target(RefName::new(&name), RefTarget::normal(target.id().clone()));
        let stats = git::export_refs(tx.repo_mut()).map_err(verr)?;
        if !stats.failed_bookmarks.is_empty() {
            return Err(AvError::Vcs(format!(
                "could not export {name} to git: {:?}",
                stats.failed_bookmarks
            )));
        }
        block_on(tx.repo_mut().rebase_descendants()).map_err(verr)?;
        block_on(tx.commit(op)).map_err(verr)?;
        Ok(())
    }

    fn merge_task(
        &self,
        repo: &str,
        slug: &str,
        expected_head: &ChangeId,
        message: &str,
    ) -> Result<MergeOutcome> {
        let head = self.snapshot(repo, slug)?;
        if &head != expected_head {
            return Err(AvError::IllegalState(format!(
                "workspace {slug} changed since it was approved (head {} is not {}); submit again",
                short(&head.0),
                short(&expected_head.0)
            )));
        }
        let (_ws, ro) = self.load_main(repo)?;
        let task = self.commit_in_repo(&ro, &head)?;
        let trunk = self.trunk_commit(repo, &ro)?;
        let base_tree = block_on(task.parent_tree(ro.as_ref())).map_err(verr)?;
        let merged = block_on(MergedTree::merge(Merge::from_vec(vec![
            (trunk.tree(), "main".to_owned()),
            (base_tree, "task base".to_owned()),
            (task.tree(), format!("task {slug}")),
        ])))
        .map_err(verr)?;
        if merged.has_conflict() {
            return Ok(MergeOutcome::Conflicted {
                paths: conflict_paths(&merged),
            });
        }
        let mut tx = ro.start_transaction();
        let commit = block_on(
            tx.repo_mut()
                .new_commit(vec![trunk.id().clone()], merged)
                .set_description(message)
                .write(),
        )
        .map_err(verr)?;
        let ro = block_on(tx.commit(format!("av merge {slug}: write commit"))).map_err(verr)?;
        self.move_trunk(repo, &ro, &commit, &format!("av merge {slug}"))?;
        Ok(MergeOutcome::Merged {
            pre_merge: ChangeId(trunk.id().hex()),
            main: ChangeId(commit.id().hex()),
        })
    }

    fn rebase_workspace(&self, repo: &str, slug: &str) -> Result<RebaseOutcome> {
        let head = self.snapshot(repo, slug)?;
        let (mut ws, ro) = self.load_workspace(repo, &self.ws_dir(repo, slug))?;
        let name = ws.workspace_name().to_owned();
        let wc = self.commit_in_repo(&ro, &head)?;
        let trunk = self.trunk_commit(repo, &ro)?;
        let base = ChangeId(trunk.id().hex());
        if wc.parent_ids() == [trunk.id().clone()] {
            return Ok(RebaseOutcome {
                conflicts: conflict_paths(&wc.tree()),
                head,
                base,
            });
        }
        let mut tx = ro.start_transaction();
        let rebased = block_on(rebase_commit(
            tx.repo_mut(),
            wc.clone(),
            vec![trunk.id().clone()],
        ))
        .map_err(verr)?;
        tx.repo_mut()
            .set_wc_commit(name, rebased.id().clone())
            .map_err(verr)?;
        block_on(tx.repo_mut().rebase_descendants()).map_err(verr)?;
        let ro = block_on(tx.commit(format!("av sync {slug}"))).map_err(verr)?;
        block_on(ws.check_out(ro.op_id().clone(), Some(&wc.tree()), &rebased)).map_err(verr)?;
        Ok(RebaseOutcome {
            head: ChangeId(rebased.id().hex()),
            base,
            conflicts: conflict_paths(&rebased.tree()),
        })
    }

    fn revert_merge(
        &self,
        repo: &str,
        merge_commit: &ChangeId,
        message: &str,
    ) -> Result<MergeOutcome> {
        let (_ws, ro) = self.load_main(repo)?;
        let merge = self.commit_in_repo(&ro, merge_commit)?;
        if merge.parent_ids().len() != 1 {
            return Err(AvError::IllegalState(format!(
                "{} has {} parents; only a single-parent merge commit can be reverted",
                short(&merge_commit.0),
                merge.parent_ids().len()
            )));
        }
        let trunk = self.trunk_commit(repo, &ro)?;
        let before = block_on(merge.parent_tree(ro.as_ref())).map_err(verr)?;
        // trunk + (before - merge): the merge's own change, inverted, on top of later work
        let reverted = block_on(MergedTree::merge(Merge::from_vec(vec![
            (trunk.tree(), "main".to_owned()),
            (merge.tree(), format!("merge {}", short(&merge_commit.0))),
            (before, "before the merge".to_owned()),
        ])))
        .map_err(verr)?;
        if reverted.has_conflict() {
            return Ok(MergeOutcome::Conflicted {
                paths: conflict_paths(&reverted),
            });
        }
        let mut tx = ro.start_transaction();
        let commit = block_on(
            tx.repo_mut()
                .new_commit(vec![trunk.id().clone()], reverted)
                .set_description(message)
                .write(),
        )
        .map_err(verr)?;
        let op = format!("av revert {}", short(&merge_commit.0));
        let ro = block_on(tx.commit(format!("{op}: write commit"))).map_err(verr)?;
        self.move_trunk(repo, &ro, &commit, &op)?;
        Ok(MergeOutcome::Merged {
            pre_merge: ChangeId(trunk.id().hex()),
            main: ChangeId(commit.id().hex()),
        })
    }

    fn undo_merge(&self, repo: &str, pre_merge: &ChangeId, merge_commit: &ChangeId) -> Result<()> {
        let (_ws, ro) = self.load_main(repo)?;
        let trunk = self.trunk_commit(repo, &ro)?;
        if trunk.id().hex() != merge_commit.0 {
            return Err(AvError::IllegalState(format!(
                "main moved since the merge (now {}); undo the later merges first",
                short(&trunk.id().hex())
            )));
        }
        let pre = self.commit_in_repo(&ro, pre_merge)?;
        self.move_trunk(
            repo,
            &ro,
            &pre,
            &format!("av undo merge {}", short(&merge_commit.0)),
        )
    }
}

fn short(hex: &str) -> &str {
    &hex[..hex.len().min(12)]
}

#[async_trait]
impl Vcs for JjVcs {
    async fn init_repo(&self, repo: &str, git_source: &Path) -> Result<ChangeId> {
        let (name, src) = (repo.to_owned(), git_source.to_owned());
        self.run(repo, move |i| i.init_repo(&name, &src)).await
    }

    async fn create_workspace(&self, repo: &str, slug: &str) -> Result<WorkspaceInfo> {
        let (name, slug) = (repo.to_owned(), slug.to_owned());
        self.run(repo, move |i| i.create_workspace(&name, &slug))
            .await
    }

    async fn snapshot(&self, repo: &str, slug: &str) -> Result<ChangeId> {
        let (name, slug) = (repo.to_owned(), slug.to_owned());
        self.run(repo, move |i| i.snapshot(&name, &slug)).await
    }

    async fn changed_paths(
        &self,
        repo: &str,
        base: &ChangeId,
        head: &ChangeId,
    ) -> Result<Vec<String>> {
        let (name, b, h) = (repo.to_owned(), base.clone(), head.clone());
        self.read(repo, move |i| i.changed_paths(&name, &b, &h))
            .await
    }

    async fn list_files(&self, repo: &str, change: &ChangeId) -> Result<Vec<String>> {
        let (name, c) = (repo.to_owned(), change.clone());
        self.read(repo, move |i| i.list_files(&name, &c)).await
    }

    async fn read_file(&self, repo: &str, change: &ChangeId, path: &str) -> Result<Vec<u8>> {
        let (name, c, p) = (repo.to_owned(), change.clone(), vec![path.to_owned()]);
        let mut out = self
            .read(repo, move |i| i.read_files(&name, &c, &p))
            .await?;
        out.pop().map_or_else(
            || Err(AvError::Vcs("read_files returned nothing".into())),
            |(_, r)| r,
        )
    }

    async fn read_files(
        &self,
        repo: &str,
        change: &ChangeId,
        paths: &[String],
    ) -> Result<Vec<(String, Result<Vec<u8>>)>> {
        let (name, c, p) = (repo.to_owned(), change.clone(), paths.to_vec());
        self.read(repo, move |i| i.read_files(&name, &c, &p)).await
    }

    async fn list_repos(&self) -> Result<Vec<String>> {
        let root = self.inner.store_root.clone();
        tokio::task::spawn_blocking(move || {
            let mut names = Vec::new();
            for entry in std::fs::read_dir(&root).map_err(verr)? {
                let entry = entry.map_err(verr)?;
                // a repo is a directory with a jj repo in its default workspace
                if entry.path().join("main/.jj/repo").is_dir()
                    && let Some(n) = entry.file_name().to_str()
                {
                    names.push(n.to_owned());
                }
            }
            names.sort();
            Ok(names)
        })
        .await
        .map_err(verr)?
    }

    async fn trunk(&self, repo: &str) -> Result<ChangeId> {
        let name = repo.to_owned();
        self.run(repo, move |i| i.trunk(&name)).await
    }

    async fn merge_task(
        &self,
        repo: &str,
        slug: &str,
        expected_head: &ChangeId,
        message: &str,
    ) -> Result<MergeOutcome> {
        let (name, slug, head, msg) = (
            repo.to_owned(),
            slug.to_owned(),
            expected_head.clone(),
            message.to_owned(),
        );
        self.run(repo, move |i| i.merge_task(&name, &slug, &head, &msg))
            .await
    }

    async fn push_backup(
        &self,
        repo: &str,
        remote_url: &str,
        branch: Option<&str>,
    ) -> Result<ChangeId> {
        let (name, url, branch) = (
            repo.to_owned(),
            remote_url.to_owned(),
            branch.map(str::to_owned),
        );
        // reads only the exported git ref, which merges and undos update atomically
        self.read(repo, move |i| i.push_backup(&name, &url, branch.as_deref()))
            .await
    }

    async fn trunk_log(&self, repo: &str, limit: usize) -> Result<Vec<LogEntry>> {
        let name = repo.to_owned();
        self.run(repo, move |i| i.trunk_log(&name, limit)).await
    }

    async fn rebase_workspace(&self, repo: &str, slug: &str) -> Result<RebaseOutcome> {
        let (name, slug) = (repo.to_owned(), slug.to_owned());
        self.run(repo, move |i| i.rebase_workspace(&name, &slug))
            .await
    }

    async fn revert_merge(
        &self,
        repo: &str,
        merge_commit: &ChangeId,
        message: &str,
    ) -> Result<MergeOutcome> {
        let (name, commit, msg) = (repo.to_owned(), merge_commit.clone(), message.to_owned());
        self.run(repo, move |i| i.revert_merge(&name, &commit, &msg))
            .await
    }

    async fn undo_merge(
        &self,
        repo: &str,
        pre_merge: &ChangeId,
        merge_commit: &ChangeId,
    ) -> Result<()> {
        let (name, pre, merged) = (repo.to_owned(), pre_merge.clone(), merge_commit.clone());
        self.run(repo, move |i| i.undo_merge(&name, &pre, &merged))
            .await
    }
}
