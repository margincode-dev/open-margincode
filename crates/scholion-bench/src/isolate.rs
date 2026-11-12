//! `run --agent-user NAME`: the agent CLI runs as a user that can read only its own working
//! copy, so a session cannot read the grading tests, the bench tree or an earlier session.
//!
//! The harness runs as root and stays root: it prepares each session, hands the working copy to
//! the agent user ([`AgentUser::give`]), starts the agent CLI as that user, then kills whatever
//! that user still runs and takes the working copy back ([`take_back`]) before the hidden tests
//! are copied in. One user serves every session, so what a session leaves is root's alone before
//! the next one starts. Before the first session [`preflight`] tries, as that user, the paths a
//! session must not see and refuses to run if one opens.
//!
//! File permissions only. Still open, and named in `bench/README.md`: the daemon answers any
//! session about any repository, the network, directories every user may write (`/tmp`), and
//! `verify_cmd`, which runs what the agent wrote as the harness's user.

use std::io::Write as _;
use std::os::unix::fs::{MetadataExt, PermissionsExt, lchown};
use std::os::unix::process::CommandExt;
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};

use crate::Result;
use crate::runner::{Arm, RunConfig, Vendor};
use crate::spec::TaskSpec;

/// The user the agent CLI runs as.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentUser {
    pub name: String,
    pub uid: u32,
    pub gid: u32,
    /// The home the password database names for the user (what `getpwuid` answers, whatever
    /// `HOME` says); `None` when the entry names none.
    pub home: Option<PathBuf>,
}

/// The home of the user `name` (or of the entry with that `uid`) in password-database text:
/// `getent passwd` output or `/etc/passwd`. `None`: no such entry. `Some(None)`: an entry whose
/// home field is empty.
fn passwd_home(text: &str, name: &str, uid: u32) -> Option<Option<PathBuf>> {
    let uid = uid.to_string();
    text.lines().find_map(|line| {
        let f: Vec<&str> = line.split(':').collect();
        if f.len() < 6 || f[0].starts_with('#') || (f[0] != name && f[2] != uid) {
            return None;
        }
        Some((!f[5].is_empty()).then(|| PathBuf::from(f[5])))
    })
}

/// The user's home as the password database has it: `getent passwd NAME` (which asks every
/// source the host is set up with), else `/etc/passwd`.
fn home_of(name: &str, uid: u32) -> Result<Option<PathBuf>> {
    let asked = Command::new("getent").args(["passwd", name]).output();
    let text = match asked {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).into_owned(),
        _ => std::fs::read_to_string("/etc/passwd").unwrap_or_default(),
    };
    passwd_home(&text, name, uid).ok_or_else(|| {
        format!(
            "--agent-user {name}: no entry for that user in the password database (`getent \
             passwd {name}`, /etc/passwd), so its home cannot be checked"
        )
    })
}

/// `id FLAG [NAME]` as a number.
fn id(flag: &str, name: Option<&str>) -> Result<u32> {
    let out = Command::new("id")
        .arg(flag)
        .args(name)
        .output()
        .map_err(|e| format!("id: {e}"))?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_owned());
    }
    String::from_utf8_lossy(&out.stdout)
        .trim()
        .parse()
        .map_err(|e| format!("id {flag}: {e}"))
}

/// The user this process runs as.
pub fn harness_uid() -> Result<u32> {
    id("-u", None)
}

fn io_err(path: &Path, e: std::io::Error) -> String {
    format!("{}: {e}", path.display())
}

/// Sets a path's permission bits.
pub fn set_mode(path: &Path, mode: u32) -> Result<()> {
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode))
        .map_err(|e| io_err(path, e))
}

/// Creates `path` (and what is missing above it) if it is not there, closed to everyone but the
/// harness. A directory that exists is left as it is: the preflight says when it is open.
pub fn private_dir(path: &Path) -> Result<()> {
    if path.exists() {
        return Ok(());
    }
    std::fs::create_dir_all(path).map_err(|e| io_err(path, e))?;
    set_mode(path, 0o700)
}

/// An empty file only the harness can read, for output that is then written through
/// `File::create` (which keeps an existing file's mode).
pub fn private_file(path: &Path) -> Result<()> {
    std::fs::write(path, b"").map_err(|e| io_err(path, e))?;
    set_mode(path, 0o600)
}

/// Every entry under `path` becomes `uid:gid`'s. Symlinks are changed themselves, never
/// followed. With `close`, group and other lose every permission (and set-id bits go).
fn chown_tree(path: &Path, uid: u32, gid: u32, close: bool) -> std::io::Result<()> {
    let meta = std::fs::symlink_metadata(path)?;
    lchown(path, Some(uid), Some(gid))?;
    if meta.file_type().is_symlink() {
        return Ok(());
    }
    if close {
        let mode = meta.permissions().mode() & 0o700;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode))?;
    }
    if meta.is_dir() {
        for entry in std::fs::read_dir(path)? {
            chown_tree(&entry?.path(), uid, gid, close)?;
        }
    }
    Ok(())
}

/// After a session: the tree is root's again and nobody else can enter or read any of it, so
/// neither a later session nor a file the agent linked elsewhere reaches what verify adds.
/// Nothing if `path` is not there.
pub fn take_back(path: &Path) -> Result<()> {
    if std::fs::symlink_metadata(path).is_err() {
        return Ok(());
    }
    chown_tree(path, 0, 0, true).map_err(|e| format!("taking back {}: {e}", path.display()))?;
    match std::fs::symlink_metadata(path) {
        Ok(m) if m.is_dir() => set_mode(path, 0o700),
        _ => Ok(()),
    }
}

/// Where a hidden test goes in a working copy the agent has written to: no symlink the agent
/// left on the way is followed (it is removed), and a file already there is removed first, so
/// the test is a new file and not one the agent linked or still holds open.
pub fn fresh_dest(workspace: &Path, rel: &str) -> Result<PathBuf> {
    let mut dest = workspace.to_path_buf();
    for part in Path::new(rel).components() {
        dest.push(part);
        if !matches!(part, Component::Normal(_)) {
            continue;
        }
        if let Ok(m) = std::fs::symlink_metadata(&dest)
            && m.file_type().is_symlink()
        {
            std::fs::remove_file(&dest).map_err(|e| io_err(&dest, e))?;
        }
    }
    match std::fs::symlink_metadata(&dest) {
        Ok(m) if m.is_dir() => Err(format!(
            "{}: a directory is where the hidden test goes",
            dest.display()
        )),
        Ok(_) => std::fs::remove_file(&dest)
            .map(|()| dest.clone())
            .map_err(|e| io_err(&dest, e)),
        Err(_) => Ok(dest),
    }
}

/// What a process of the agent user is asked to do with a path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Access {
    /// Read a file's first byte.
    Read,
    /// List a directory.
    List,
    /// Change into a directory.
    Enter,
    /// Find a program to execute, by path or on `PATH`.
    Execute,
    /// Write to a path: for a directory, make or remove entries in it.
    Write,
}

impl Access {
    fn code(self) -> char {
        match self {
            Access::Read => 'r',
            Access::List => 'l',
            Access::Enter => 'e',
            Access::Execute => 'x',
            Access::Write => 'w',
        }
    }
}

/// Run by `sh` as the agent user: one `<code> <path>` per line on stdin, the numbers of the
/// lines that worked on stdout.
const PROBE: &str = r#"n=0
while IFS= read -r line; do
  k=${line%% *}; p=${line#* }
  case "$k" in
    r) head -c 1 -- "$p" >/dev/null 2>&1 && echo "$n";;
    l) ls -- "$p" >/dev/null 2>&1 && echo "$n";;
    e) (cd -- "$p") >/dev/null 2>&1 && echo "$n";;
    x) command -v -- "$p" >/dev/null 2>&1 && echo "$n";;
    w) [ -w "$p" ] && echo "$n";;
  esac
  n=$((n+1))
done
"#;

impl AgentUser {
    /// Looks the user up by name (`id -u`, `id -g`, and its home in the password database).
    /// Root is refused: it reads everything.
    pub fn resolve(name: &str) -> Result<AgentUser> {
        let unknown = |e: String| format!("--agent-user {name}: no such user ({e})");
        let uid = id("-u", Some(name)).map_err(unknown)?;
        let gid = id("-g", Some(name)).map_err(unknown)?;
        if uid == 0 {
            return Err(format!(
                "--agent-user {name}: that user is root, which can read everything"
            ));
        }
        Ok(AgentUser {
            name: name.to_owned(),
            uid,
            gid,
            home: home_of(name, uid)?,
        })
    }

    /// `program` as this user (no supplementary groups: std drops them when root sets a uid),
    /// in `/`, with no stdin.
    fn command(&self, program: &str) -> Command {
        let mut cmd = Command::new(program);
        cmd.uid(self.uid)
            .gid(self.gid)
            .current_dir("/")
            .stdin(Stdio::null());
        cmd
    }

    /// Hands a tree to the agent for its session: everything in it is the user's, and only the
    /// user can enter it.
    pub fn give(&self, path: &Path) -> Result<()> {
        chown_tree(path, self.uid, self.gid, false)
            .map_err(|e| format!("handing {} to user {}: {e}", path.display(), self.name))?;
        set_mode(path, 0o700)
    }

    /// The live processes of this user, from `/proc` (zombies are not counted: a container
    /// without an init never reaps them). Empty where there is no `/proc`.
    pub fn processes(&self) -> Vec<u32> {
        let Ok(entries) = std::fs::read_dir("/proc") else {
            return vec![];
        };
        entries
            .flatten()
            .filter_map(|e| {
                let pid: u32 = e.file_name().to_str()?.parse().ok()?;
                if e.metadata().ok()?.uid() != self.uid {
                    return None;
                }
                let stat = std::fs::read_to_string(e.path().join("stat")).ok()?;
                let state = stat.rsplit(')').next()?.trim_start().chars().next()?;
                (state != 'Z').then_some(pid)
            })
            .collect()
    }

    /// Kills every process of this user (`kill -KILL -1` run as the user), so nothing a session
    /// started outlives it with a file or a directory open. The user must be the agent's alone.
    pub fn kill_processes(&self) -> Result<()> {
        for _ in 0..20 {
            let _ = self
                .command("kill")
                .args(["-KILL", "--", "-1"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
            if self.processes().is_empty() {
                return Ok(());
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        Err(format!(
            "processes of user {} survived the end of the session: {:?}",
            self.name,
            self.processes()
        ))
    }

    /// Tries each probe as this user, in one process, and says which worked. `path` puts
    /// directories first on that process's `PATH` (for [`Access::Execute`]).
    pub fn allowed(&self, probes: &[(Access, PathBuf)], path: &[PathBuf]) -> Result<Vec<bool>> {
        let mut input = String::new();
        for (access, p) in probes {
            // a program named without a directory is looked up on PATH, as the agent would
            let bare = *access == Access::Execute && p.components().count() == 1;
            let text = if bare {
                p.to_string_lossy().into_owned()
            } else {
                absolute(p)?.to_string_lossy().into_owned()
            };
            if text.contains('\n') {
                return Err(format!("{text:?}: a path with a newline"));
            }
            input.push_str(&format!("{} {text}\n", access.code()));
        }
        let mut cmd = self.command("sh");
        cmd.args(["-c", PROBE])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());
        if !path.is_empty() {
            let inherited = std::env::var_os("PATH").unwrap_or_default();
            let dirs = path
                .iter()
                .cloned()
                .chain(std::env::split_paths(&inherited));
            cmd.env(
                "PATH",
                std::env::join_paths(dirs).map_err(|e| e.to_string())?,
            );
        }
        let mut child = cmd
            .spawn()
            .map_err(|e| format!("cannot start a process as user {}: {e}", self.name))?;
        let mut stdin = child.stdin.take().expect("piped");
        let writer = std::thread::spawn(move || {
            let _ = stdin.write_all(input.as_bytes());
        });
        let out = child.wait_with_output().map_err(|e| e.to_string())?;
        let _ = writer.join();
        if !out.status.success() {
            return Err(format!(
                "the probe as user {} failed ({})",
                self.name, out.status
            ));
        }
        let mut worked = vec![false; probes.len()];
        for n in String::from_utf8_lossy(&out.stdout).lines() {
            if let Some(slot) = n
                .trim()
                .parse::<usize>()
                .ok()
                .and_then(|i| worked.get_mut(i))
            {
                *slot = true;
            }
        }
        Ok(worked)
    }
}

fn absolute(p: &Path) -> Result<PathBuf> {
    let p = if p.as_os_str().is_empty() {
        Path::new(".")
    } else {
        p
    };
    std::path::absolute(p).map_err(|e| io_err(p, e))
}

/// One thing the preflight tries as the agent user.
struct Check {
    access: Access,
    path: PathBuf,
    what: String,
}

impl Check {
    fn line(&self) -> String {
        let verb = match self.access {
            Access::Read => "read",
            Access::List => "list",
            Access::Enter => "enter",
            Access::Execute => "execute",
            Access::Write => "write to",
        };
        format!("  {verb} {} ({})", self.path.display(), self.what)
    }
}

fn add(checks: &mut Vec<Check>, access: Access, path: &Path, what: impl Into<String>) {
    if !checks.iter().any(|c| c.access == access && c.path == path) {
        checks.push(Check {
            access,
            path: path.to_path_buf(),
            what: what.into(),
        });
    }
}

/// The entries of a directory, sorted; none if it cannot be read.
fn entries(dir: &Path) -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = std::fs::read_dir(dir)
        .map(|d| d.flatten().map(|e| e.path()).collect())
        .unwrap_or_default();
    v.sort();
    v
}

/// How far down a shared directory the preflight looks for copies of grading files, and how
/// many entries it looks at in all.
const SCAN_DEPTH: usize = 6;
const SCAN_ENTRIES: usize = 200_000;

/// Directories every user can write and read (the temp dirs), where an earlier session, or a
/// person looking into one, may have left a copy of a grading file.
fn shared_dirs() -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = vec![];
    for d in [
        std::env::temp_dir(),
        "/tmp".into(),
        "/var/tmp".into(),
        "/dev/shm".into(),
    ] {
        if let Ok(d) = d.canonicalize()
            && d.is_dir()
            && !dirs.contains(&d)
        {
            dirs.push(d);
        }
    }
    dirs
}

/// The files a session must never see, with their bytes: each task's hidden tests (the ones
/// taken from `fix_rev` and the ones written for the benchmark).
fn grading_files(specs: &[TaskSpec]) -> Result<Vec<(String, Vec<u8>)>> {
    let mut files = vec![];
    for s in specs {
        let what = format!("a copy of a hidden test of {}", s.name);
        if let Some(fix) = s.fix_rev.as_deref() {
            for f in &s.hidden_tests {
                let rev = format!("{fix}:{f}");
                let bytes = crate::spec::git_in(&s.repo_path, &["show", "--end-of-options", &rev])?;
                files.push((what.clone(), bytes));
            }
        }
        for t in &s.local_tests {
            let src = s.spec_dir.join(&t.source);
            let bytes = std::fs::read(&src).map_err(|e| io_err(&src, e))?;
            files.push((what.clone(), bytes));
        }
    }
    files.retain(|(_, bytes)| !bytes.is_empty());
    Ok(files)
}

/// Adds to `found` every regular file under `dir` whose bytes are exactly those of a grading
/// file, whatever its name. Symlinks are not followed.
fn find_copies(
    dir: &Path,
    grading: &[(String, Vec<u8>)],
    depth: usize,
    budget: &mut usize,
    found: &mut Vec<(PathBuf, String)>,
) {
    for path in entries(dir) {
        if *budget == 0 {
            return;
        }
        *budget -= 1;
        let Ok(meta) = std::fs::symlink_metadata(&path) else {
            continue;
        };
        if meta.is_dir() {
            if depth > 1 {
                find_copies(&path, grading, depth - 1, budget, found);
            }
        } else if meta.is_file()
            && grading.iter().any(|(_, b)| b.len() as u64 == meta.len())
            && let Ok(bytes) = std::fs::read(&path)
            && let Some((what, _)) = grading.iter().find(|(_, b)| *b == bytes)
        {
            found.push((path, what.clone()));
        }
    }
}

/// What the preflight says of the agent user's home.
const HOME_WHAT: &str = "that user's home in the password database: what a session leaves there, \
                         a later one finds";

/// The files that make a directory a results directory: `run` writes the first, `first-packs`
/// the second (beside `packs/`, which holds the tasks' lessons).
const RESULT_FILES: [&str; 2] = ["rows.jsonl", "first-packs.jsonl"];

/// How far below the parent of `--out` the preflight looks for other results directories, and
/// how many entries it looks at in all.
const RESULTS_DEPTH: usize = 3;
const RESULTS_ENTRIES: usize = 20_000;

/// Adds to `found` the directories under `dir` that hold a results file (`RESULT_FILES`), down
/// to `depth` levels. `skip` (this run's own `--out`) is left out, a directory that is found is
/// not looked into further (closing it closes what is in it), and symlinks are not followed.
fn results_dirs(
    dir: &Path,
    skip: &Path,
    depth: usize,
    budget: &mut usize,
    found: &mut Vec<PathBuf>,
) {
    for path in entries(dir) {
        if *budget == 0 {
            return;
        }
        *budget -= 1;
        if path == skip || !std::fs::symlink_metadata(&path).is_ok_and(|m| m.is_dir()) {
            continue;
        }
        if RESULT_FILES.iter().any(|f| path.join(f).is_file()) {
            found.push(path);
        } else if depth > 1 {
            results_dirs(&path, skip, depth - 1, budget, found);
        }
    }
}

/// The checks the agent user did not get the `want`ed answer on.
fn failing<'a>(
    user: &AgentUser,
    checks: &'a [Check],
    path: &[PathBuf],
    want: bool,
) -> Result<Vec<&'a Check>> {
    let probes: Vec<(Access, PathBuf)> =
        checks.iter().map(|c| (c.access, c.path.clone())).collect();
    let got = user.allowed(&probes, path)?;
    Ok(checks
        .iter()
        .zip(got)
        .filter(|(_, ok)| *ok != want)
        .map(|(c, _)| c)
        .collect())
}

/// Before the first session, with no model: as the agent user, what a session needs must work
/// (its run directory, the agent CLI, `av`, the `path_prepend` directories) and what it must
/// not see must fail (the bench tree and its grading tests, the source repos, the import dir,
/// earlier runs and sessions, the workspaces the daemon kept, the results directory `out` and
/// the results directories beside it, its own home in the password database). Refuses to run
/// otherwise and names each path. Nothing when the run has no agent user.
///
/// It makes the harness's own directories as they must be (`runs/` and the run's directory:
/// entered, never listed; the import dir and `out`, if missing: root's alone) and changes
/// nothing else.
pub fn preflight(
    specs: &[TaskSpec],
    arms: &[Arm],
    cfg: &RunConfig,
    run_id: &str,
    out: &Path,
) -> Result<()> {
    let Some(user) = &cfg.agent_user else {
        return Ok(());
    };
    let flag = format!("--agent-user {}", user.name);
    let me = harness_uid()?;
    if me != 0 {
        return Err(format!(
            "{flag}: the harness must run as root (it runs as uid {me}): it starts the agent as \
             another user and takes each working copy back afterwards"
        ));
    }
    let av = arms.iter().any(|a| a.uses_av());
    let work_root = absolute(&cfg.work_root)?;
    let runs = work_root.join("runs");
    let run_dir = runs.join(run_id);
    // Can the user get there at all? Asked of the directories that exist, before anything is
    // made or changed under the work root.
    let mut way: Vec<Check> = vec![];
    let mut up: Vec<&Path> = run_dir.ancestors().filter(|d| d.is_dir()).collect();
    up.reverse();
    for dir in up {
        add(&mut way, Access::Enter, dir, "");
    }
    if let Some(at) = failing(user, &way, &[], true)?.first() {
        let mode = std::fs::metadata(&at.path).map_or(0, |m| m.mode() & 0o7777);
        return Err(format!(
            "{flag}: user {} cannot reach the work root {}: it may not enter {} (mode {mode:o}). \
             A work root under a directory closed to other users, such as /root, cannot be used \
             with --agent-user. Pass a --work-root (and --import-dir) that user can enter; the \
             existing one is not moved or changed.",
            user.name,
            work_root.display(),
            at.path.display()
        ));
    }
    std::fs::create_dir_all(&run_dir).map_err(|e| io_err(&run_dir, e))?;
    for d in [&runs, &run_dir] {
        set_mode(d, 0o711)?;
    }
    let ws_root = match (&cfg.av_ws_root, av) {
        (Some(root), true) => Some(absolute(root)?),
        (None, true) => {
            return Err(format!(
                "{flag}: the av arms need --av-ws-root (or AV_WS_ROOT), the bench daemon's \
                 workspace root as the harness sees it, so it can be checked before the first \
                 session"
            ));
        }
        (_, false) => None,
    };
    if av {
        private_dir(&cfg.import_dir)?;
    }
    // the results directory is the harness's alone when the harness makes it
    let out = absolute(out)?;
    private_dir(&out)?;
    let mut path: Vec<PathBuf> = vec![];
    for dir in specs.iter().flat_map(|s| &s.path_prepend) {
        if !path.contains(dir) {
            path.push(dir.clone());
        }
    }

    // what a session needs
    let mut need = vec![];
    add(&mut need, Access::Enter, &run_dir, "the run's directory");
    let cli = match cfg.vendor {
        Vendor::Claude => &cfg.claude_bin,
        Vendor::Grok => &cfg.grok_bin,
    };
    add(&mut need, Access::Execute, Path::new(cli), "the agent CLI");
    if av {
        let av_bin = Path::new(&cfg.av_bin);
        add(
            &mut need,
            Access::Execute,
            av_bin,
            "the av CLI the hooks run",
        );
    }
    if let Some(root) = &ws_root {
        add(
            &mut need,
            Access::Enter,
            root,
            "the daemon's workspace root",
        );
    }
    for dir in path.iter().filter(|d| d.is_dir()) {
        add(&mut need, Access::Enter, dir, "a path_prepend directory");
    }
    let missing = failing(user, &need, &path, true)?;

    // what a session must not reach
    let mut deny = vec![];
    for s in specs {
        let dir = absolute(&s.spec_dir)?;
        let what = "the task files, grading tests and reference fixes";
        add(&mut deny, Access::Enter, &dir, what);
        add(&mut deny, Access::List, &dir, what);
        for t in &s.local_tests {
            let src = absolute(&s.spec_dir.join(&t.source))?;
            let what = format!("a hidden test of {}", s.name);
            add(&mut deny, Access::Read, &src, what);
        }
        if let Some(pool) = &s.relay_pool {
            let pool = absolute(&s.spec_dir.join(pool))?;
            add(
                &mut deny,
                Access::Read,
                &pool,
                "the relay notes of every arm",
            );
        }
        let repo = absolute(&s.repo_path)?;
        let what = format!("the source repo of {}, which holds the fix", s.name);
        add(&mut deny, Access::Enter, &repo, what);
    }
    if av {
        let dir = absolute(&cfg.import_dir)?;
        let what = "the import dir: the av arms' copies, with the relay commits";
        add(&mut deny, Access::Enter, &dir, what);
    }
    add(&mut deny, Access::List, &runs, "the runs root");
    for other in entries(&runs).iter().filter(|p| **p != run_dir) {
        add(&mut deny, Access::Enter, other, "an earlier run's sessions");
    }
    add(&mut deny, Access::List, &run_dir, "the run's directory");
    for session in entries(&run_dir) {
        let what = "an earlier session of this run";
        add(&mut deny, Access::Enter, &session, what);
    }
    if let Some(root) = &ws_root {
        add(&mut deny, Access::List, root, "the daemon's workspace root");
        for repo in entries(root) {
            add(
                &mut deny,
                Access::Enter,
                &repo,
                "workspaces the daemon kept",
            );
        }
    }
    // Results: the rows and reports of earlier sessions, and where `first-packs` wrote, packs
    // with the tasks' lessons, which a `cold` or `baseline` session must not see. This run's
    // `--out`, and the results directories found beside it.
    let what = "the results directory (--out)";
    add(&mut deny, Access::Enter, &out, what);
    add(&mut deny, Access::List, &out, what);
    for entry in entries(&out) {
        let access = if entry.is_dir() {
            Access::Enter
        } else {
            Access::Read
        };
        add(&mut deny, access, &entry, "in the results directory");
    }
    if let Some(beside) = out.parent() {
        let mut found = vec![];
        let mut budget = RESULTS_ENTRIES;
        results_dirs(beside, &out, RESULTS_DEPTH, &mut budget, &mut found);
        if budget == 0 {
            eprintln!(
                "scholion-bench: {flag}: the search for other results directories under {} \
                 stopped after {RESULTS_ENTRIES} entries; the rest was not looked at",
                beside.display()
            );
        }
        for dir in &found {
            let what = "results beside --out: rows, or packs with the tasks' lessons";
            add(&mut deny, Access::Enter, dir, what);
        }
    }
    // The user's own home, as the password database has it: the harness sets `HOME` to the
    // session's directory, but a program can look the home up (`getpwuid`). One user serves
    // every session, so a home it can write to carries work from one session to the next, and
    // one it can list shows what was left there.
    let home = user.home.as_deref().filter(|h| h.is_dir());
    if let Some(home) = home {
        add(&mut deny, Access::Write, home, HOME_WHAT);
        add(&mut deny, Access::List, home, HOME_WHAT);
    }
    // A copy of a grading file where every user reads: a session of an earlier, open run (or
    // someone looking into it) may have left one in a temp dir, and no permission of the
    // harness's own directories covers that.
    let grading = grading_files(specs)?;
    let mut copies = vec![];
    let mut budget = SCAN_ENTRIES;
    for dir in shared_dirs() {
        find_copies(&dir, &grading, SCAN_DEPTH, &mut budget, &mut copies);
    }
    if budget == 0 {
        // a search that stopped early must not read as a search that found nothing
        eprintln!(
            "scholion-bench: {flag}: the search for copies of grading files in the temp \
             directories stopped after {SCAN_ENTRIES} entries; the rest was not looked at"
        );
    }
    for (path, what) in &copies {
        add(&mut deny, Access::Read, path, what.as_str());
    }
    let open = failing(user, &deny, &[], false)?;
    if open.is_empty() && missing.is_empty() {
        return Ok(());
    }
    // everything at once, so one look at the message is enough to fix the set-up
    let who = format!("user {} (uid {})", user.name, user.uid);
    let mut msg = format!("{flag}: refusing to run.");
    if !open.is_empty() {
        let shown: Vec<String> = open.iter().take(40).map(|c| c.line()).collect();
        let more = match open.len() - shown.len() {
            0 => String::new(),
            n => format!("\n  and {n} more"),
        };
        msg.push_str(&format!(
            "\nAs {who} a session could:\n{}{more}\nClose each of them to that user. For a \
             directory only the harness needs, `chmod 700` on it (or on the top of a mounted \
             bench tree) is enough: root still reads it. The daemon's workspace root needs \
             `chmod 711`.",
            shown.join("\n")
        ));
        if let Some(home) = home.filter(|_| open.iter().any(|c| c.what == HOME_WHAT)) {
            msg.push_str(&format!(
                " The user's own home is closed by giving the user one that does not exist \
                 (`usermod -d /nonexistent {}`) or by making it root's alone (`chown root:root \
                 {home} && chmod 700 {home}`).",
                user.name,
                home = home.display()
            ));
        }
    }
    if !missing.is_empty() {
        let lines: Vec<String> = missing.iter().map(|c| c.line()).collect();
        msg.push_str(&format!(
            "\nAs {who} a session could not, and must:\n{}\nPut what the sessions run (the \
             agent CLI, av, the tasks' path_prepend directories) where that user can reach it.",
            lines.join("\n")
        ));
    }
    Err(msg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_and_unknown_users_are_refused_as_the_agent_user() {
        let err = AgentUser::resolve("root").unwrap_err();
        assert!(err.contains("is root"), "{err}");
        let err = AgentUser::resolve("no-such-user-here").unwrap_err();
        assert!(err.contains("no such user"), "{err}");
    }

    #[test]
    fn the_home_is_read_from_the_password_database() {
        let text = "root:x:0:0:root:/root:/bin/bash\n\
                    # a comment:x:1:1::/commented:/bin/sh\n\
                    short:x:7\n\
                    nobody:x:65534:65534:nobody:/nonexistent:/usr/sbin/nologin\n\
                    agent:x:1001:1001:the bench agent,,,:/home/agent:/bin/sh\n\
                    homeless:x:1002:1002:::/bin/sh\n";
        let home = |name: &str, uid: u32| passwd_home(text, name, uid);
        assert_eq!(home("nobody", 65534), Some(Some("/nonexistent".into())));
        assert_eq!(home("agent", 1001), Some(Some("/home/agent".into())));
        // a user given by number is found by its uid
        assert_eq!(home("1001", 1001), Some(Some("/home/agent".into())));
        assert_eq!(home("homeless", 1002), Some(None), "an empty home field");
        assert_eq!(home("ghost", 4242), None, "no entry");
        assert_eq!(home("short", 7), None, "a line without a home field");
        // what `getent passwd NAME` prints: the one line
        let one = "agent:x:1001:1001::/srv/agent:/bin/sh\n";
        assert_eq!(
            passwd_home(one, "agent", 1001),
            Some(Some("/srv/agent".into()))
        );
    }

    #[test]
    fn results_directories_beside_out_are_found_by_the_files_they_hold() {
        let tmp = tempfile::tempdir().unwrap();
        let top = tmp.path();
        let out = top.join("tonight");
        for d in [
            "tonight",
            "pilot",
            "first-pack/before/n500/packs",
            "first-pack/a/b/too-deep",
            "venv/lib",
            "pilot/nested",
        ] {
            std::fs::create_dir_all(top.join(d)).unwrap();
        }
        for f in [
            "tonight/rows.jsonl",
            "pilot/rows.jsonl",
            "pilot/nested/rows.jsonl",
            "first-pack/before/n500/first-packs.jsonl",
            "first-pack/a/b/too-deep/rows.jsonl",
            "venv/lib/rows.py",
            "rows.jsonl",
        ] {
            std::fs::write(top.join(f), "{}\n").unwrap();
        }
        // a link to a results directory is not one: what it points at is found where it is
        std::os::unix::fs::symlink(top.join("pilot"), top.join("latest")).unwrap();
        let mut found = vec![];
        let mut budget = RESULTS_ENTRIES;
        results_dirs(top, &out, RESULTS_DEPTH, &mut budget, &mut found);
        assert_eq!(
            found,
            [top.join("first-pack/before/n500"), top.join("pilot")],
            "not --out itself, nothing inside one that is found, nothing below the depth"
        );
        // a search that ran out of entries stops
        let (mut found, mut budget) = (vec![], 1);
        results_dirs(top, &out, RESULTS_DEPTH, &mut budget, &mut found);
        assert_eq!((found, budget), (vec![], 0));
    }

    #[test]
    fn a_hidden_test_never_goes_through_a_link_or_into_a_file_the_agent_left() {
        let tmp = tempfile::tempdir().unwrap();
        let ws = tmp.path().join("ws");
        let elsewhere = tmp.path().join("elsewhere");
        std::fs::create_dir_all(ws.join("real")).unwrap();
        std::fs::create_dir_all(&elsewhere).unwrap();
        std::fs::write(elsewhere.join("kept"), "agent's").unwrap();
        // a directory on the way is a link out of the working copy
        std::os::unix::fs::symlink(&elsewhere, ws.join("tests")).unwrap();
        let dest = fresh_dest(&ws, "tests/test_a.py").unwrap();
        assert_eq!(dest, ws.join("tests/test_a.py"));
        assert!(std::fs::symlink_metadata(ws.join("tests")).is_err());
        // the file itself is a link, or a file the agent made
        std::os::unix::fs::symlink(elsewhere.join("kept"), ws.join("real/test_b.py")).unwrap();
        std::fs::write(ws.join("real/test_c.py"), "agent's").unwrap();
        for f in ["real/test_b.py", "real/test_c.py"] {
            let dest = fresh_dest(&ws, f).unwrap();
            assert_eq!(dest, ws.join(f));
            assert!(std::fs::symlink_metadata(&dest).is_err(), "{f} is gone");
        }
        assert_eq!(
            std::fs::read_to_string(elsewhere.join("kept")).unwrap(),
            "agent's",
            "what the links pointed at is untouched"
        );
        // a path that is free is just the path
        assert_eq!(
            fresh_dest(&ws, "new/dir/test_d.py").unwrap(),
            ws.join("new/dir/test_d.py")
        );
        assert!(fresh_dest(&ws, "real").is_err(), "a directory in the way");
    }
}
