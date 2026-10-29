//! Task specs (`bench/tasks/*.toml`) and `scholion-bench check`.

use std::path::{Path, PathBuf};
use std::process::Command;

use globset::Glob;
use regex::Regex;
use serde::Deserialize;

use crate::Result;

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskSpec {
    /// Used in directory and repo names: lowercase letters, digits and `-`.
    pub name: String,
    /// `~/` is expanded; `load_spec_with_root` swaps the parent directory.
    pub repo_path: PathBuf,
    /// The commit both arms start from (the real fix's parent).
    pub base_rev: String,
    /// The real fix; `hidden_tests` are taken from it.
    #[serde(default)]
    pub fix_rev: Option<String>,
    /// Written from the commit message, without naming files. Identical in every arm.
    pub goal: String,
    /// Files the real fix changed. An edit to one of these, or to `created_files`, is a relevant
    /// edit.
    pub relevant_files: Vec<String>,
    /// Files the real fix added; they must not exist at `base_rev`.
    #[serde(default)]
    pub created_files: Vec<String>,
    /// Copied from `fix_rev` into the working directory after the session, before `verify_cmd`,
    /// overwriting what the agent left there (no arm sees them during the session).
    #[serde(default)]
    pub hidden_tests: Vec<String>,
    /// Tests written for the benchmark (the real fix had none), copied in like `hidden_tests`.
    #[serde(default)]
    pub local_tests: Vec<LocalTest>,
    /// Run with `sh -c` in the working directory; exit 0 = accepted.
    pub verify_cmd: String,
    #[serde(default = "default_verify_timeout")]
    pub verify_timeout_secs: u64,
    pub max_turns: u32,
    pub model: String,
    /// Run with `sh -c` in the working directory before the session (both arms).
    #[serde(default)]
    pub setup_cmd: String,
    /// Directories put first on the agent's `PATH` (so it finds the interpreter that has the
    /// test runner). `~/` is expanded.
    #[serde(default)]
    pub path_prepend: Vec<PathBuf>,
    /// The `claude` process is killed after this long.
    #[serde(default = "default_session_timeout")]
    pub session_timeout_secs: u64,
    #[serde(default, rename = "dead_end")]
    pub dead_ends: Vec<DeadEnd>,
    /// E4 knowledge relay: a `notes.toml` (relative to the spec file) of what an earlier agent
    /// learned. Every arm gets the notes as commits on top of the base; `notes` also as
    /// `NOTES.md`, the av arms as anchored records of an earlier task (instead of `dead_end`s).
    #[serde(default)]
    pub relay_pool: Option<PathBuf>,
    /// The pool note this task needs (the rest are noise for it).
    #[serde(default)]
    pub relay_pair: Option<String>,
    /// The notes of `relay_pool`, read when the spec loads.
    #[serde(skip)]
    pub relay_notes: Vec<RelayNote>,
    /// The directory of the spec file, which `LocalTest::source` is relative to.
    #[serde(skip)]
    pub spec_dir: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalTest {
    /// Where it goes in the working directory.
    pub path: String,
    /// The file, relative to the spec file.
    pub source: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeadEnd {
    pub description: String,
    pub file_glob: String,
    /// A regex over the text an edit writes.
    pub pattern: String,
}

/// One thing an earlier agent learned (`[[note]]` in a relay pool).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RelayNote {
    /// The task that needs it.
    pub pair: String,
    /// A record kind: `attempt` (a dead end) or `decision`.
    pub kind: String,
    /// The file it is about, and optionally the symbol.
    pub path: String,
    #[serde(default)]
    pub symbol: Option<String>,
    /// At most 200 characters (the record limit).
    pub summary: String,
    #[serde(default)]
    pub body: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RelayPool {
    note: Vec<RelayNote>,
}

fn load_relay(spec: &mut TaskSpec) -> Result<()> {
    let Some(pool) = &spec.relay_pool else {
        return Ok(());
    };
    let path = spec.spec_dir.join(pool);
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let pool: RelayPool = toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    for n in &pool.note {
        if n.summary.chars().count() > 200 {
            return Err(format!(
                "{}: note for {} has a summary over 200 characters",
                path.display(),
                n.pair
            ));
        }
        if !matches!(n.kind.as_str(), "attempt" | "decision" | "note") {
            return Err(format!(
                "{}: note for {}: kind must be attempt, decision or note",
                path.display(),
                n.pair
            ));
        }
    }
    if let Some(pair) = &spec.relay_pair
        && !pool.note.iter().any(|n| &n.pair == pair)
    {
        return Err(format!(
            "{}: no note for relay_pair {pair:?}",
            path.display()
        ));
    }
    spec.relay_notes = pool.note;
    Ok(())
}

fn default_verify_timeout() -> u64 {
    300
}

fn default_session_timeout() -> u64 {
    1800
}

impl TaskSpec {
    /// Relevant-edit targets: `relevant_files` and `created_files`.
    pub fn is_relevant(&self, path: &str) -> bool {
        self.relevant_files.iter().any(|f| f == path)
            || self.created_files.iter().any(|f| f == path)
    }
}

pub fn load_spec(path: &Path) -> Result<TaskSpec> {
    load_spec_with_root(path, None)
}

/// With `repos_root`, `repo_path` becomes `repos_root/<its last component>`, so the same task
/// files work on any machine that has the repos side by side.
pub fn load_spec_with_root(path: &Path, repos_root: Option<&Path>) -> Result<TaskSpec> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut spec: TaskSpec =
        toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    spec.spec_dir = path.parent().map(Path::to_path_buf).unwrap_or_default();
    load_relay(&mut spec)?;
    spec.path_prepend = spec.path_prepend.iter().map(|p| expand_home(p)).collect();
    spec.repo_path = match repos_root {
        Some(root) => root.join(spec.repo_path.file_name().unwrap_or_default()),
        None => expand_home(&spec.repo_path),
    };
    Ok(spec)
}

fn expand_home(p: &Path) -> PathBuf {
    match (p.strip_prefix("~"), std::env::var_os("HOME")) {
        (Ok(rest), Some(home)) => PathBuf::from(home).join(rest),
        _ => p.to_path_buf(),
    }
}

/// Every spec file in `dir` (`*.toml`), sorted by file name.
pub fn spec_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(dir)
        .map_err(|e| format!("{}: {e}", dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "toml"))
        .collect();
    files.sort();
    Ok(files)
}

/// `git -C repo <args>`, stdout on success.
pub(crate) fn git_in(repo: &Path, args: &[&str]) -> Result<Vec<u8>> {
    let out = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .output()
        .map_err(|e| format!("git: {e}"))?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_owned());
    }
    Ok(out.stdout)
}

/// The full commit id of `rev`, refusing option-like revs.
pub(crate) fn resolve_rev(repo: &Path, rev: &str) -> Result<String> {
    if rev.starts_with('-') || rev.is_empty() {
        return Err(format!("revision {rev:?} is not a commit name"));
    }
    let out = git_in(
        repo,
        &[
            "rev-parse",
            "--verify",
            "--quiet",
            "--end-of-options",
            &format!("{rev}^{{commit}}"),
        ],
    )
    .map_err(|_| format!("revision {rev:?} does not resolve to a commit"))?;
    Ok(String::from_utf8_lossy(&out).trim().to_owned())
}

pub(crate) fn exists_at(repo: &Path, rev: &str, path: &str) -> bool {
    git_in(
        repo,
        &[
            "cat-file",
            "-e",
            "--end-of-options",
            &format!("{rev}:{path}"),
        ],
    )
    .is_ok()
}

fn safe_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 40
        && name
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        && !name.starts_with('-')
}

pub fn check_spec(spec: &TaskSpec) -> Result<()> {
    let fail = |msg: String| Err(format!("task {}: {msg}", spec.name));
    if !safe_name(&spec.name) {
        return fail("name must be lowercase letters, digits and '-', at most 40".into());
    }
    if !spec.repo_path.join(".git").exists() {
        return fail(format!(
            "repo path {} is not a git checkout",
            spec.repo_path.display()
        ));
    }
    let repo = &spec.repo_path;
    let base = match resolve_rev(repo, &spec.base_rev) {
        Ok(b) => b,
        Err(e) => return fail(format!("base_rev: {e}")),
    };
    if spec.relevant_files.is_empty() && spec.created_files.is_empty() {
        return fail("relevant_files is empty".into());
    }
    for f in &spec.relevant_files {
        if !exists_at(repo, &base, f) {
            return fail(format!("relevant file {f} does not exist at base_rev"));
        }
    }
    for f in &spec.created_files {
        if exists_at(repo, &base, f) {
            return fail(format!("created file {f} already exists at base_rev"));
        }
    }
    if !spec.hidden_tests.is_empty() {
        let Some(fix) = &spec.fix_rev else {
            return fail("hidden_tests needs fix_rev".into());
        };
        let fix = match resolve_rev(repo, fix) {
            Ok(f) => f,
            Err(e) => return fail(format!("fix_rev: {e}")),
        };
        for f in &spec.hidden_tests {
            if !exists_at(repo, &fix, f) {
                return fail(format!("hidden test {f} does not exist at fix_rev"));
            }
        }
    } else if let Some(fix) = &spec.fix_rev
        && let Err(e) = resolve_rev(repo, fix)
    {
        return fail(format!("fix_rev: {e}"));
    }
    for t in &spec.local_tests {
        if !spec.spec_dir.join(&t.source).is_file() {
            return fail(format!(
                "local test source {} not found",
                t.source.display()
            ));
        }
        if t.path.starts_with('/') || t.path.split('/').any(|c| c == "..") {
            return fail(format!(
                "local test path {} must stay inside the repo",
                t.path
            ));
        }
    }
    for d in &spec.dead_ends {
        if let Err(e) = Regex::new(&d.pattern) {
            return fail(format!("dead_end pattern {:?}: {e}", d.pattern));
        }
        if let Err(e) = Glob::new(&d.file_glob) {
            return fail(format!("dead_end file_glob {:?}: {e}", d.file_glob));
        }
    }
    if spec.verify_cmd.trim().is_empty() {
        return fail("verify_cmd is empty".into());
    }
    if spec.max_turns == 0 {
        return fail("max_turns is 0".into());
    }
    Ok(())
}

/// Problems that do not stop a run: a `path_prepend` directory that is not there.
pub fn check_warnings(spec: &TaskSpec) -> Vec<String> {
    spec.path_prepend
        .iter()
        .filter(|d| !d.is_dir())
        .map(|d| {
            format!(
                "task {}: warning: path_prepend {} is not a directory",
                spec.name,
                d.display()
            )
        })
        .collect()
}
