//! Plan 6 E2: merge correctness, fuzzed. Random tasks edit random trees concurrently and merge in
//! random orders, with reverts of earlier merges mixed in; after every merge and revert the result is checked against a line model and against
//! `git merge-tree`. Invariants:
//! - no committed line is lost (trunk equals the model, file for file);
//! - main advances only through merge and revert, one first-parent commit each;
//! - a conflict is reported iff two edits overlap (same line, delete against edit, add/add);
//! - on clean merges the tree equals the one `git merge-tree` writes, and git conflicts iff jj does.
//!
//! CI runs a fixed seed and few cases. A long run: `PROPTEST_CASES=2000 PROPTEST_RNG_SEED=<n>
//! cargo test -p scholion-vcs --test merge_fuzz --release`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

use proptest::prelude::*;
use proptest::test_runner::{Config, RngSeed, TestCaseError, TestRunner};
use scholion_core::{ChangeId, MergeOutcome, Vcs};
use scholion_vcs::JjVcs;
use tempfile::TempDir;

const SHARED: &str = "shared.txt";

/// Clean results, conflicts and reverts seen, so the run can show it exercised each.
static MERGED: AtomicUsize = AtomicUsize::new(0);
static CONFLICTED: AtomicUsize = AtomicUsize::new(0);
static REVERTS: AtomicUsize = AtomicUsize::new(0);

/// Files as lines; absent files are not in the map.
type Tree = BTreeMap<String, Vec<String>>;

#[derive(Debug, Clone)]
enum Op {
    /// Replaces line `2 * slot` (even lines only, so edits by two tasks never touch adjacent
    /// lines, where git and jj may legitimately disagree about a conflict).
    Edit {
        file: usize,
        slot: usize,
    },
    Delete {
        file: usize,
    },
    /// Adds `new_<task>.txt`.
    AddOwn,
    /// Adds `shared.txt` with this task's content.
    AddShared,
}

#[derive(Debug, Clone)]
struct Case {
    /// Lines per base file (`f0.txt`, `f1.txt`, …).
    files: Vec<usize>,
    /// Per task, in merge order: how many earlier merges land before its workspace starts
    /// (clamped to its index), its operations, and which earlier clean merge (modulo their
    /// number) to revert right after its own merge, if any.
    tasks: Vec<(usize, Vec<Op>, Option<usize>)>,
}

fn op() -> impl Strategy<Value = Op> {
    prop_oneof![
        6 => (0..3usize, 0..4usize).prop_map(|(file, slot)| Op::Edit { file, slot }),
        1 => (0..3usize).prop_map(|file| Op::Delete { file }),
        1 => Just(Op::AddOwn),
        1 => Just(Op::AddShared),
    ]
}

/// Mostly 0 (every task works from the same base, so they overlap), sometimes later.
fn start() -> impl Strategy<Value = usize> {
    prop_oneof![3 => Just(0usize), 1 => 1..5usize]
}

fn case() -> impl Strategy<Value = Case> {
    (
        prop::collection::vec(4..12usize, 1..=3),
        prop::collection::vec(
            (
                start(),
                prop::collection::vec(op(), 0..5),
                prop::option::weighted(0.2, 0..8usize),
            ),
            2..=5,
        ),
    )
        .prop_map(|(files, tasks)| Case { files, tasks })
}

fn git(dir: &Path, args: &[&str]) -> (bool, String) {
    let out = Command::new("git")
        .current_dir(dir)
        .args([
            "-c",
            "user.name=t",
            "-c",
            "user.email=t@t",
            "-c",
            "commit.gpgsign=false",
        ])
        .args(args)
        .output()
        .unwrap();
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).trim().to_owned(),
    )
}

fn git_ok(dir: &Path, args: &[&str]) -> String {
    let (ok, out) = git(dir, args);
    assert!(ok, "git {args:?} failed");
    out
}

fn text(lines: &[String]) -> String {
    lines.iter().map(|l| format!("{l}\n")).collect()
}

fn base_tree(case: &Case) -> Tree {
    case.files
        .iter()
        .enumerate()
        .map(|(f, n)| {
            let lines = (0..*n).map(|i| format!("f{f} line {i}")).collect();
            (format!("f{f}.txt"), lines)
        })
        .collect()
}

/// The task's tree: its operations applied to `base`. Operations that do not apply there (a
/// line past the end, a file already gone or already present) are skipped.
fn apply(base: &Tree, task: usize, ops: &[Op]) -> Tree {
    let mut tree = base.clone();
    for op in ops {
        match op {
            Op::Edit { file, slot } => {
                if let Some(lines) = tree.get_mut(&format!("f{file}.txt"))
                    && let Some(line) = lines.get_mut(2 * slot)
                {
                    *line = format!("task {task} edits f{file} line {}", 2 * slot);
                }
            }
            Op::Delete { file } => {
                tree.remove(&format!("f{file}.txt"));
            }
            Op::AddOwn => {
                tree.entry(format!("new_{task}.txt"))
                    .or_insert_with(|| vec![format!("added by task {task}")]);
            }
            Op::AddShared => {
                tree.entry(SHARED.into())
                    .or_insert_with(|| vec![format!("shared, from task {task}")]);
            }
        }
    }
    tree
}

/// Three-way merge of whole trees by the rules the generator allows: per line for files both
/// sides keep (lines are never inserted or removed), per file otherwise. `Err` lists the
/// conflicted paths.
fn model_merge(base: &Tree, ours: &Tree, theirs: &Tree) -> Result<Tree, BTreeSet<String>> {
    let paths: BTreeSet<&String> = base
        .keys()
        .chain(ours.keys())
        .chain(theirs.keys())
        .collect();
    let (mut out, mut conflicts) = (Tree::new(), BTreeSet::new());
    for path in paths {
        let (b, o, t) = (base.get(path), ours.get(path), theirs.get(path));
        let merged = if o == t {
            o.cloned()
        } else if o == b {
            t.cloned()
        } else if t == b {
            o.cloned()
        } else if let (Some(b), Some(o), Some(t)) = (b, o, t) {
            let mut lines = Vec::with_capacity(b.len());
            let mut clash = false;
            for i in 0..b.len() {
                lines.push(if o[i] == t[i] || t[i] == b[i] {
                    o[i].clone()
                } else if o[i] == b[i] {
                    t[i].clone()
                } else {
                    clash = true;
                    b[i].clone()
                });
            }
            if clash {
                conflicts.insert(path.clone());
                continue;
            }
            Some(lines)
        } else {
            // delete against edit, or add/add with different content
            conflicts.insert(path.clone());
            continue;
        };
        if let Some(lines) = merged {
            out.insert(path.clone(), lines);
        }
    }
    if conflicts.is_empty() {
        Ok(out)
    } else {
        Err(conflicts)
    }
}

struct Repo {
    _src: TempDir,
    store: TempDir,
    ws: TempDir,
    vcs: JjVcs,
}

impl Repo {
    fn git_dir(&self) -> std::path::PathBuf {
        self.store.path().join("demo/git")
    }
    fn workspace(&self, slug: &str) -> std::path::PathBuf {
        self.ws.path().join("demo").join(slug)
    }
    async fn tree_at(&self, change: &ChangeId) -> Tree {
        let mut tree = Tree::new();
        for path in self.vcs.list_files("demo", change).await.unwrap() {
            let bytes = self.vcs.read_file("demo", change, &path).await.unwrap();
            let text = String::from_utf8(bytes).unwrap();
            tree.insert(path, text.lines().map(str::to_owned).collect());
        }
        tree
    }
}

async fn repo(base: &Tree) -> Repo {
    let src = TempDir::new().unwrap();
    git_ok(src.path(), &["init", "-q", "-b", "main"]);
    for (path, lines) in base {
        std::fs::write(src.path().join(path), text(lines)).unwrap();
    }
    git_ok(src.path(), &["add", "-A"]);
    git_ok(src.path(), &["commit", "-q", "-m", "init"]);
    let store = TempDir::new().unwrap();
    let ws = TempDir::new().unwrap();
    let vcs = JjVcs::new(store.path().to_path_buf(), ws.path().to_path_buf());
    vcs.init_repo("demo", src.path()).await.unwrap();
    Repo {
        _src: src,
        store,
        ws,
        vcs,
    }
}

/// Writes `tree` into the workspace over `base` (deleting what `tree` drops).
fn write_workspace(dir: &Path, base: &Tree, tree: &Tree) {
    for path in base.keys() {
        if !tree.contains_key(path) {
            std::fs::remove_file(dir.join(path)).unwrap();
        }
    }
    for (path, lines) in tree {
        if base.get(path) != Some(lines) {
            std::fs::write(dir.join(path), text(lines)).unwrap();
        }
    }
}

struct Started {
    slug: String,
    base_change: ChangeId,
    base: Tree,
    head: ChangeId,
    tree: Tree,
}

/// Trunk as the run knows it: the commit, the model tree, and the clean merges so far (each
/// commit with its tree and its parent's tree), which reverts pick from.
struct Trunk {
    change: ChangeId,
    tree: Tree,
    merges: Vec<(ChangeId, Tree, Tree)>,
}

impl Trunk {
    /// Checks one merge (or revert) against the model and `git merge-tree` of the same three
    /// commits, then advances on a clean result.
    async fn check(
        &mut self,
        r: &Repo,
        label: &str,
        outcome: MergeOutcome,
        (base_commit, base): (&ChangeId, &Tree),
        (theirs_commit, theirs): (&ChangeId, &Tree),
    ) -> Result<(), TestCaseError> {
        let expected = model_merge(base, &self.tree, theirs);
        let git_dir = r.git_dir();
        let (git_clean, git_out) = git(
            &git_dir,
            &[
                "merge-tree",
                "--write-tree",
                "--name-only",
                &format!("--merge-base={}", base_commit.0),
                &self.change.0,
                &theirs_commit.0,
            ],
        );
        match (outcome, expected) {
            (MergeOutcome::Merged { pre_merge, main }, Ok(merged)) => {
                prop_assert_eq!(&pre_merge, &self.change, "{} built on a stale trunk", label);
                prop_assert_eq!(&r.vcs.trunk("demo").await.unwrap(), &main);
                let log = r.vcs.trunk_log("demo", 1).await.unwrap();
                prop_assert_eq!(&log[0].change, &main);
                prop_assert_eq!(
                    &log[0].parents,
                    &vec![self.change.clone()],
                    "{} is not one first-parent commit",
                    label
                );
                prop_assert_eq!(
                    &r.tree_at(&main).await,
                    &merged,
                    "{}: trunk lost or invented content",
                    label
                );
                prop_assert!(git_clean, "{}: jj is clean, git conflicts", label);
                let jj_tree = git_ok(&git_dir, &["rev-parse", &format!("{}^{{tree}}", main.0)]);
                let git_tree = git_out.lines().next().unwrap_or_default();
                prop_assert_eq!(
                    jj_tree.as_str(),
                    git_tree,
                    "{}: tree differs from git merge-tree",
                    label
                );
                MERGED.fetch_add(1, Ordering::Relaxed);
                self.merges
                    .push((main.clone(), merged.clone(), self.tree.clone()));
                self.change = main;
                self.tree = merged;
            }
            (MergeOutcome::Conflicted { paths }, Err(conflicts)) => {
                let paths: BTreeSet<String> = paths.into_iter().collect();
                prop_assert_eq!(&paths, &conflicts, "{}", label);
                prop_assert_eq!(
                    &r.vcs.trunk("demo").await.unwrap(),
                    &self.change,
                    "{}: a conflict moved trunk",
                    label
                );
                prop_assert!(
                    !git_clean,
                    "{}: jj conflicts on {:?}, git is clean",
                    label,
                    paths
                );
                let git_paths: BTreeSet<String> = git_out
                    .lines()
                    .skip(1)
                    .take_while(|l| !l.is_empty())
                    .map(str::to_owned)
                    .collect();
                prop_assert_eq!(
                    &git_paths,
                    &paths,
                    "{}: git conflicts on other paths",
                    label
                );
                CONFLICTED.fetch_add(1, Ordering::Relaxed);
            }
            (outcome, expected) => {
                prop_assert!(false, "{}: jj {:?}, model {:?}", label, outcome, expected);
            }
        }
        Ok(())
    }
}

async fn run_case(case: Case) -> Result<(), TestCaseError> {
    let initial = base_tree(&case);
    let r = repo(&initial).await;
    let change = r.vcs.trunk("demo").await.unwrap();
    prop_assert_eq!(&r.tree_at(&change).await, &initial);
    let mut trunk = Trunk {
        change,
        tree: initial,
        merges: Vec::new(),
    };

    // Tasks start in the order of their start points; merges run in task order.
    let n = case.tasks.len();
    let start_at: Vec<usize> = (0..n).map(|i| case.tasks[i].0.min(i)).collect();
    let mut started: Vec<Option<Started>> = (0..n).map(|_| None).collect();

    for step in 0..n {
        for (i, (_, ops, _)) in case.tasks.iter().enumerate() {
            if start_at[i] != step {
                continue;
            }
            let slug = format!("t{i}");
            let info = r.vcs.create_workspace("demo", &slug).await.unwrap();
            prop_assert_eq!(&info.base_change, &trunk.change);
            let tree = apply(&trunk.tree, i, ops);
            write_workspace(&r.workspace(&slug), &trunk.tree, &tree);
            let head = r.vcs.snapshot("demo", &slug).await.unwrap();
            started[i] = Some(Started {
                slug,
                base_change: info.base_change,
                base: trunk.tree.clone(),
                head,
                tree,
            });
        }

        let t = started[step]
            .as_ref()
            .expect("a task starts before its merge");
        let label = format!("merge {}", t.slug);
        let outcome = r
            .vcs
            .merge_task("demo", &t.slug, &t.head, &label)
            .await
            .unwrap();
        trunk
            .check(
                &r,
                &label,
                outcome,
                (&t.base_change, &t.base),
                (&t.head, &t.tree),
            )
            .await?;

        // A revert inverts an earlier merge: trunk + (its parent - it).
        if let Some(k) = case.tasks[step].2
            && !trunk.merges.is_empty()
        {
            let (commit, after, before) = trunk.merges[k % trunk.merges.len()].clone();
            let parent = r.vcs.trunk_log("demo", usize::MAX).await.unwrap();
            let parent = parent
                .iter()
                .find(|e| e.change == commit)
                .and_then(|e| e.parents.first().cloned())
                .expect("a merge commit on trunk has a parent");
            let label = format!("revert {}", commit.0);
            let outcome = r.vcs.revert_merge("demo", &commit, &label).await.unwrap();
            REVERTS.fetch_add(1, Ordering::Relaxed);
            trunk
                .check(&r, &label, outcome, (&commit, &after), (&parent, &before))
                .await?;
        }
    }
    Ok(())
}

fn config() -> Config {
    let mut c = Config::default();
    if std::env::var_os("PROPTEST_CASES").is_none() {
        c.cases = 24;
    }
    if std::env::var_os("PROPTEST_RNG_SEED").is_none() {
        c.rng_seed = RngSeed::Fixed(0x5c401107);
    }
    c.failure_persistence = None;
    c
}

#[test]
fn concurrent_merges_match_the_model_and_git() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let mut runner = TestRunner::new(config());
    runner.run(&case(), |c| rt.block_on(run_case(c))).unwrap();
    let [merged, conflicted, reverts] =
        [&MERGED, &CONFLICTED, &REVERTS].map(|c| c.load(Ordering::Relaxed));
    eprintln!("{merged} clean, {conflicted} conflicts, {reverts} of them reverts");
    assert!(
        merged > 0 && conflicted > 0 && reverts > 0,
        "the cases never exercised one of the outcomes"
    );
}
