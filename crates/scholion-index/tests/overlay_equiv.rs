//! Plan 7's invariant: a task's view (main seen through the task's overlay) reads exactly what a
//! full index of the task's tree reads. Random main trees in Python, Rust and TypeScript, with
//! function names drawn from a small pool so references fan out, then random steps: the task
//! edits, adds, deletes or restores a file, or main merges a change. After every step each read
//! method's results through the overlay view are compared with a fresh full index of the task's
//! tree, symbol ids replaced by `path:qname`.
//!
//! The overlay is written by hand here (task-scope rows for the paths whose content differs from
//! main, tombstones for paths the task deleted, a `scope_overlays` row): the overlay writes are
//! Plan 7 Task 4. Overlay reads are Task 3; until then `overlay_reads_match_a_full_index` fails
//! and is ignored. Run it with `cargo test -p scholion-index --test overlay_equiv -- --ignored`.
//!
//! CI runs a fixed seed. A long run: `PROPTEST_CASES=500 PROPTEST_RNG_SEED=<n> cargo test ...`.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::atomic::{AtomicUsize, Ordering};

use proptest::prelude::*;
use proptest::test_runner::{Config, RngSeed, TestCaseError, TestRunner};
use scholion_core::{Anchor, SymbolId};
use scholion_index::{AnchorResolution, CodeIndex, PgIndex, ScopeView};
use scholion_store::PgStore;
use scholion_testutil::TestDb;
use sqlx::PgPool;

/// Function names. Few, so most are defined in several files and every call fans out.
const NAMES: &[&str] = &["load", "save", "parse", "render", "get", "fetch"];
const LANGS: &[&str] = &["py", "rs", "ts"];

/// A file: its language and its functions, each a name and the names it calls.
#[derive(Debug, Clone)]
struct FileSpec {
    lang: usize,
    funcs: Vec<(usize, Vec<usize>)>,
}

#[derive(Debug, Clone)]
enum Step {
    /// The task rewrites an existing file (index into the sorted paths of its tree).
    TaskEdit(usize, FileSpec),
    TaskAdd(FileSpec),
    TaskDelete(usize),
    /// The task puts a file back to main's content (or deletes one main does not have).
    TaskRestore(usize),
    /// Main merges a rewrite of one of its files.
    MainEdit(usize, FileSpec),
    MainAdd(FileSpec),
    MainDelete(usize),
}

#[derive(Debug, Clone)]
struct Case {
    main: Vec<FileSpec>,
    steps: Vec<Step>,
}

fn file_spec() -> impl Strategy<Value = FileSpec> {
    (
        0..LANGS.len(),
        prop::collection::vec(
            (0..NAMES.len(), prop::collection::vec(0..NAMES.len(), 0..3)),
            1..4,
        ),
    )
        .prop_map(|(lang, funcs)| FileSpec { lang, funcs })
}

fn step() -> impl Strategy<Value = Step> {
    prop_oneof![
        4 => (any::<usize>(), file_spec()).prop_map(|(i, f)| Step::TaskEdit(i, f)),
        2 => file_spec().prop_map(Step::TaskAdd),
        3 => any::<usize>().prop_map(Step::TaskDelete),
        1 => any::<usize>().prop_map(Step::TaskRestore),
        2 => (any::<usize>(), file_spec()).prop_map(|(i, f)| Step::MainEdit(i, f)),
        1 => file_spec().prop_map(Step::MainAdd),
        1 => any::<usize>().prop_map(Step::MainDelete),
    ]
}

fn case() -> impl Strategy<Value = Case> {
    (
        prop::collection::vec(file_spec(), 2..6),
        prop::collection::vec(step(), 1..6),
    )
        .prop_map(|(main, steps)| Case { main, steps })
}

/// Source text. Function names may repeat within a file; the index keeps what the parser sees.
fn render(f: &FileSpec) -> String {
    let mut out = String::new();
    for (k, (name, calls)) in f.funcs.iter().enumerate() {
        // the first function takes a pool name, so names repeat across files; later ones are
        // suffixed, so a file never defines one qname twice
        let name = if k == 0 {
            NAMES[*name].to_owned()
        } else {
            format!("{}_{k}", NAMES[*name])
        };
        let calls: Vec<String> = calls.iter().map(|c| format!("{}(x)", NAMES[*c])).collect();
        let body = if calls.is_empty() {
            "x".to_owned()
        } else {
            calls.join(" + ")
        };
        out += &match LANGS[f.lang] {
            "py" => format!("def {name}(x):\n    return {body}\n\n"),
            "rs" => format!("fn {name}(x: i32) -> i32 {{\n    {body}\n}}\n\n"),
            _ => {
                format!("export function {name}(x: number): number {{\n    return {body};\n}}\n\n")
            }
        };
    }
    out
}

type Tree = BTreeMap<String, String>;

fn nth(tree: &Tree, i: usize) -> Option<String> {
    (!tree.is_empty()).then(|| tree.keys().nth(i % tree.len()).unwrap().clone())
}

/// Applies a step to (main, task). Files are added under fresh names (`n<counter>.<ext>`).
fn apply(step: &Step, main: &mut Tree, task: &mut Tree, counter: &mut usize) {
    let fresh = |f: &FileSpec, counter: &mut usize| {
        *counter += 1;
        format!("n{counter}.{}", LANGS[f.lang])
    };
    match step {
        Step::TaskEdit(i, f) => {
            if let Some(p) = nth(task, *i) {
                // keep the extension so the path stays one language
                let ext = p.rsplit_once('.').unwrap().1;
                let lang = LANGS.iter().position(|l| *l == ext).unwrap();
                task.insert(p, render(&FileSpec { lang, ..f.clone() }));
            }
        }
        Step::TaskAdd(f) => {
            task.insert(fresh(f, counter), render(f));
        }
        Step::TaskDelete(i) => {
            if let Some(p) = nth(task, *i) {
                task.remove(&p);
            }
        }
        Step::TaskRestore(i) => {
            let paths: BTreeSet<&String> = task.keys().chain(main.keys()).collect();
            if let Some(p) = paths
                .iter()
                .nth(*i % paths.len().max(1))
                .map(|p| (*p).clone())
            {
                match main.get(&p) {
                    Some(text) => task.insert(p, text.clone()),
                    None => task.remove(&p),
                };
            }
        }
        // Main's merge reaches the task's tree too unless the task changed that file: the task
        // is rebased on the new main, which is what reconciling an overlay stands for.
        Step::MainEdit(i, f) => {
            if let Some(p) = nth(main, *i) {
                let ext = p.rsplit_once('.').unwrap().1;
                let lang = LANGS.iter().position(|l| *l == ext).unwrap();
                let text = render(&FileSpec { lang, ..f.clone() });
                if task.get(&p) == main.get(&p) {
                    task.insert(p.clone(), text.clone());
                }
                main.insert(p, text);
            }
        }
        Step::MainAdd(f) => {
            let p = fresh(f, counter);
            task.insert(p.clone(), render(f));
            main.insert(p, render(f));
        }
        Step::MainDelete(i) => {
            if let Some(p) = nth(main, *i) {
                if task.get(&p) == main.get(&p) {
                    task.remove(&p);
                }
                main.remove(&p);
            }
        }
    }
}

fn files(tree: &Tree, paths: impl IntoIterator<Item = String>) -> Vec<(String, Vec<u8>)> {
    paths
        .into_iter()
        .filter_map(|p| tree.get(&p).map(|t| (p, t.as_bytes().to_vec())))
        .collect()
}

/// Brings `scope` to `tree` incrementally, as the daemon does after a merge.
async fn sync_scope(idx: &PgIndex, repo: &str, scope: &str, from: &Tree, to: &Tree) {
    let changed: Vec<String> = to
        .iter()
        .filter(|(p, t)| from.get(*p) != Some(t))
        .map(|(p, _)| p.clone())
        .collect();
    let gone: Vec<String> = from
        .keys()
        .filter(|p| !to.contains_key(*p))
        .cloned()
        .collect();
    if !gone.is_empty() {
        idx.remove_paths(repo, scope, &gone).await.unwrap();
    }
    if !changed.is_empty() {
        idx.index_paths(repo, scope, files(to, changed))
            .await
            .unwrap();
    }
}

/// The overlay as Task 4 will keep it: rows for every path whose content in the task's tree
/// differs from main's, a tombstone for each path the task lacks.
async fn write_overlay(
    idx: &PgIndex,
    pool: &PgPool,
    repo: &str,
    scope: &str,
    main: &Tree,
    task: &Tree,
) {
    idx.drop_scope(repo, scope).await.unwrap();
    sqlx::query("DELETE FROM scope_overlays WHERE repo = $1 AND scope = $2")
        .bind(repo)
        .bind(scope)
        .execute(pool)
        .await
        .unwrap();
    let differ: Vec<String> = task
        .iter()
        .filter(|(p, t)| main.get(*p) != Some(t))
        .map(|(p, _)| p.clone())
        .collect();
    if !differ.is_empty() {
        idx.index_paths(repo, scope, files(task, differ))
            .await
            .unwrap();
    }
    for p in main.keys().filter(|p| !task.contains_key(*p)) {
        sqlx::query(
            "INSERT INTO index_files (repo, scope, path, content_hash, deleted) VALUES ($1, $2, $3, '', true)",
        )
        .bind(repo)
        .bind(scope)
        .bind(p)
        .execute(pool)
        .await
        .unwrap();
    }
    sqlx::query("INSERT INTO scope_overlays (repo, scope, main_at) VALUES ($1, $2, 'test')")
        .bind(repo)
        .bind(scope)
        .execute(pool)
        .await
        .unwrap();
}

/// Everything a view answers, symbol ids written as `path:qname`.
#[derive(Debug, PartialEq)]
struct Reads {
    symbols: BTreeMap<String, Vec<String>>,
    resolve: BTreeMap<String, String>,
    search: BTreeMap<String, Vec<String>>,
    lexical: BTreeMap<String, Vec<(String, String)>>,
    neighbors: BTreeMap<String, Vec<(String, u8)>>,
    in_degree: BTreeMap<String, u32>,
}

async fn names(idx: &PgIndex, ids: &[SymbolId]) -> HashMap<SymbolId, String> {
    idx.get_symbols(ids)
        .await
        .unwrap()
        .into_iter()
        .map(|s| (s.id, format!("{}:{}", s.symbol.path, s.symbol.qname)))
        .collect()
}

/// Reads `view` for the paths of `tree` (and `extra`, paths it lacks).
async fn reads(idx: &PgIndex, view: &ScopeView, tree: &Tree, extra: &BTreeSet<String>) -> Reads {
    let paths: BTreeSet<&String> = tree.keys().chain(extra).collect();
    let mut r = Reads {
        symbols: BTreeMap::new(),
        resolve: BTreeMap::new(),
        search: BTreeMap::new(),
        lexical: BTreeMap::new(),
        neighbors: BTreeMap::new(),
        in_degree: BTreeMap::new(),
    };
    let mut all_ids = vec![];
    for p in &paths {
        let syms = idx.symbols_in(view, p).await.unwrap();
        all_ids.extend(syms.iter().map(|s| s.id));
        r.symbols.insert(
            (*p).clone(),
            syms.iter()
                .map(|s| {
                    let y = &s.symbol;
                    format!(
                        "{} {:?} {} {}-{} {:?}",
                        y.qname, y.kind, y.signature, y.start_line, y.end_line, y.refs
                    )
                })
                .collect(),
        );
        for anchor_sym in NAMES.iter().map(|n| Some(n.to_string())).chain([None]) {
            let a = Anchor {
                path: (*p).clone(),
                symbol: anchor_sym.clone(),
            };
            let got = match idx.resolve(view, &a).await.unwrap() {
                AnchorResolution::Symbol(id) => names(idx, &[id]).await[&id].clone(),
                AnchorResolution::PathOnly(p) => format!("path-only {p}"),
                AnchorResolution::Stale => "stale".into(),
            };
            r.resolve
                .insert(format!("{p}#{}", anchor_sym.unwrap_or_default()), got);
        }
    }
    for n in NAMES {
        let hits = idx.search_symbols(view, n, 50).await.unwrap();
        r.search.insert(
            n.to_string(),
            hits.iter()
                .map(|s| format!("{}:{}", s.symbol.path, s.symbol.qname))
                .collect(),
        );
        let terms = vec![n.to_string(), "x".to_string()];
        let lex = idx.search_code_lexical(view, &terms, 50).await.unwrap();
        let ids: Vec<SymbolId> = lex.iter().map(|(id, _)| *id).collect();
        let by = names(idx, &ids).await;
        r.lexical.insert(
            n.to_string(),
            lex.iter()
                .map(|(id, s)| (by[id].clone(), format!("{s:.4}")))
                .collect(),
        );
    }
    let by = names(idx, &all_ids).await;
    for hops in [1u8, 2] {
        for seed in &all_ids {
            let near = idx.neighbors(view, &[*seed], hops).await.unwrap();
            let ids: Vec<SymbolId> = near.iter().map(|(id, _)| *id).collect();
            let nb = names(idx, &ids).await;
            let mut v: Vec<(String, u8)> =
                near.iter().map(|(id, h)| (nb[id].clone(), *h)).collect();
            v.sort();
            r.neighbors.insert(format!("{}@{hops}", by[seed]), v);
        }
    }
    for (id, n) in idx.in_degree(view, &all_ids).await.unwrap() {
        r.in_degree.insert(by[&id].clone(), n);
    }
    r
}

/// Masked paths, deletions and fanned-out names seen, so a run shows it exercised them.
static MASKED: AtomicUsize = AtomicUsize::new(0);
static TOMBSTONES: AtomicUsize = AtomicUsize::new(0);
static FANOUT: AtomicUsize = AtomicUsize::new(0);

struct Ctx {
    idx: PgIndex,
    pool: PgPool,
    n: AtomicUsize,
}

/// Runs `case`, comparing `actual(view of the task)` with a full index of the task's tree after
/// every step. `overlay`: the actual view is main through an overlay; otherwise a second full
/// index (the harness checking itself).
async fn run_case(ctx: &Ctx, case: Case, overlay: bool) -> Result<(), TestCaseError> {
    let k = ctx.n.fetch_add(1, Ordering::Relaxed);
    let repo = format!("r{k}");
    let (idx, pool) = (&ctx.idx, &ctx.pool);
    let mut main: Tree = case
        .main
        .iter()
        .enumerate()
        .map(|(i, f)| (format!("f{i}.{}", LANGS[f.lang]), render(f)))
        .collect();
    let mut task = main.clone();
    sync_scope(idx, &repo, "main", &Tree::new(), &main).await;
    let mut counter = 0;
    for (s, step) in case.steps.iter().enumerate() {
        let before = main.clone();
        apply(step, &mut main, &mut task, &mut counter);
        sync_scope(idx, &repo, "main", &before, &main).await;

        let full = format!("full{s}");
        sync_scope(idx, &repo, &full, &Tree::new(), &task).await;
        let actual = if overlay {
            write_overlay(idx, pool, &repo, "task", &main, &task).await;
            ScopeView::new(&repo, "task")
        } else {
            let other = format!("again{s}");
            sync_scope(idx, &repo, &other, &Tree::new(), &task).await;
            ScopeView::new(&repo, &other)
        };
        let extra: BTreeSet<String> = main
            .keys()
            .filter(|p| !task.contains_key(*p))
            .cloned()
            .collect();
        MASKED.fetch_add(
            main.iter().filter(|(p, t)| task.get(*p) != Some(t)).count(),
            Ordering::Relaxed,
        );
        TOMBSTONES.fetch_add(extra.len(), Ordering::Relaxed);
        let defined: Vec<String> = task
            .values()
            .flat_map(|t| {
                NAMES
                    .iter()
                    .filter(|n| t.contains(&format!(" {n}(")))
                    .map(|n| n.to_string())
                    .collect::<Vec<_>>()
            })
            .collect();
        if NAMES
            .iter()
            .any(|n| defined.iter().filter(|d| d == n).count() > 1)
        {
            FANOUT.fetch_add(1, Ordering::Relaxed);
        }

        let want = reads(idx, &ScopeView::new(&repo, &full), &task, &extra).await;
        let got = reads(idx, &actual, &task, &extra).await;
        prop_assert_eq!(got, want, "after step {} ({:?})", s, step);
        idx.drop_scope(&repo, &full).await.unwrap();
    }
    Ok(())
}

fn config(cases: u32) -> Config {
    let mut c = Config::default();
    if std::env::var_os("PROPTEST_CASES").is_none() {
        c.cases = cases;
    }
    if std::env::var_os("PROPTEST_RNG_SEED").is_none() {
        c.rng_seed = RngSeed::Fixed(0x0e71a7);
    }
    c.failure_persistence = None;
    c
}

fn run(overlay: bool, cases: u32) {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let db = rt.block_on(TestDb::new());
    let pool = rt
        .block_on(PgStore::connect(&db.url))
        .unwrap()
        .pool()
        .clone();
    let ctx = Ctx {
        idx: PgIndex::new(pool.clone()),
        pool,
        n: AtomicUsize::new(0),
    };
    TestRunner::new(config(cases))
        .run(&case(), |c| rt.block_on(run_case(&ctx, c, overlay)))
        .unwrap();
    let [masked, tombs, fan] = [&MASKED, &TOMBSTONES, &FANOUT].map(|c| c.load(Ordering::Relaxed));
    eprintln!("{masked} masked paths, {tombs} tombstones, {fan} steps with fanned-out names");
    assert!(
        masked > 0 && tombs > 0 && fan > 0,
        "the cases never exercised one of masked paths, deletions and shared names"
    );
}

/// The comparison itself: two full indexes of the same tree read the same. If this fails, the
/// harness (or the index's determinism), not the overlay, is wrong.
#[test]
fn two_full_indexes_of_a_tree_read_the_same() {
    run(false, 8);
}

#[test]
#[ignore = "overlay reads are Plan 7 Task 3"]
fn overlay_reads_match_a_full_index() {
    run(true, 16);
}
