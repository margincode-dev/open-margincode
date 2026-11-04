//! E4 at scale: the deterministic irrelevant notes `run --noise N` adds to the relay arms.

use std::collections::BTreeSet;
use std::path::Path;

use crate::common::*;
use scholion_bench::noise::{Anchor, add_noise, anchors_at, generate, interleave};
use scholion_bench::spec::{RelayNote, TaskSpec, load_spec};
use tempfile::TempDir;

/// A small package: one module with a function, a class and a method, a file with no symbols,
/// a test file and a README.
fn package_repo(dir: &Path) -> String {
    git(dir, &["init", "-q", "-b", "main"]);
    std::fs::create_dir_all(dir.join("pkg")).unwrap();
    std::fs::create_dir_all(dir.join("tests")).unwrap();
    std::fs::write(
        dir.join("pkg/shop.py"),
        "def total(xs):\n    return sum(xs)\n\n\nclass Cart:\n    def add(self, x):\n        pass\n",
    )
    .unwrap();
    std::fs::write(dir.join("pkg/__init__.py"), "").unwrap();
    std::fs::write(
        dir.join("tests/test_shop.py"),
        "def test_total():\n    assert True\n",
    )
    .unwrap();
    std::fs::write(dir.join("README.md"), "# shop\n").unwrap();
    git(dir, &["add", "-A"]);
    git(dir, &["commit", "-q", "-m", "base"]);
    git(dir, &["rev-parse", "HEAD"])
}

fn anchors() -> Vec<Anchor> {
    let repo = TempDir::new().unwrap();
    let base = package_repo(repo.path());
    anchors_at(repo.path(), &base).unwrap()
}

#[test]
fn anchors_are_the_base_trees_source_files_and_symbols_but_not_its_tests() {
    let got: BTreeSet<(String, Option<String>)> =
        anchors().into_iter().map(|a| (a.path, a.symbol)).collect();
    let want: BTreeSet<(String, Option<String>)> = [
        ("pkg/__init__.py", None),
        ("pkg/shop.py", None),
        ("pkg/shop.py", Some("total")),
        ("pkg/shop.py", Some("Cart")),
        ("pkg/shop.py", Some("Cart.add")),
    ]
    .into_iter()
    .map(|(p, s)| (p.to_owned(), s.map(str::to_owned)))
    .collect();
    assert_eq!(got, want);
}

fn texts(v: &[RelayNote]) -> Vec<String> {
    v.iter()
        .map(|n| {
            format!(
                "{}|{}|{:?}|{}|{}",
                n.kind, n.path, n.symbol, n.summary, n.body
            )
        })
        .collect()
}

#[test]
fn noise_is_deterministic_and_each_level_extends_the_one_below() {
    let a = anchors();
    let fifty = generate(&a, 50, 7);
    let two_hundred = generate(&a, 200, 7);
    assert_eq!(fifty.len(), 50);
    assert_eq!(two_hundred.len(), 200);
    assert_eq!(texts(&fifty), texts(&generate(&a, 50, 7)), "same seed");
    assert_eq!(
        texts(&fifty),
        texts(&two_hundred[..50]),
        "N=50 is the first 50 of N=200"
    );
    assert_ne!(
        texts(&fifty),
        texts(&generate(&a, 50, 8)),
        "another seed, other notes"
    );
    assert!(generate(&a, 0, 7).is_empty());
    assert!(generate(&[], 10, 7).is_empty(), "no anchors, no notes");
}

#[test]
fn every_noise_note_is_a_valid_distinct_record_on_a_real_anchor() {
    let a = anchors();
    // more notes than anchors × templates: they must still be distinct
    let notes = generate(&a, 300, 1);
    let mut seen = BTreeSet::new();
    for n in &notes {
        assert!(n.noise, "{n:?}");
        assert!(n.pair.starts_with("noise-"), "{n:?}");
        assert!(
            matches!(n.kind.as_str(), "attempt" | "decision" | "note"),
            "{n:?}"
        );
        assert!(n.summary.chars().count() <= 200, "{n:?}");
        assert!(!n.body.trim().is_empty(), "{n:?}");
        assert!(
            a.iter().any(|x| x.path == n.path && x.symbol == n.symbol),
            "anchored where the repo has that file and symbol: {n:?}"
        );
        assert!(seen.insert(n.summary.clone()), "duplicate summary: {n:?}");
    }
    let kinds: BTreeSet<&str> = notes.iter().map(|n| n.kind.as_str()).collect();
    assert_eq!(kinds.len(), 3, "all three kinds: {kinds:?}");
    let files: BTreeSet<&str> = notes.iter().map(|n| n.path.as_str()).collect();
    assert_eq!(files.len(), 2, "every source file gets some: {files:?}");
}

fn note(pair: &str, noise: bool) -> RelayNote {
    RelayNote {
        pair: pair.into(),
        kind: "note".into(),
        path: "a.py".into(),
        symbol: None,
        summary: format!("about {pair}"),
        body: String::new(),
        noise,
    }
}

fn pairs(v: &[RelayNote]) -> Vec<String> {
    v.iter().map(|n| n.pair.clone()).collect()
}

#[test]
fn interleaving_keeps_both_orders_and_zero_noise_changes_nothing() {
    let base: Vec<RelayNote> = ["s1", "s2", "s3"].iter().map(|p| note(p, false)).collect();
    let noise: Vec<RelayNote> = (1..=20).map(|i| note(&format!("n{i}"), true)).collect();

    assert_eq!(pairs(&interleave(base.clone(), vec![], 1)), pairs(&base));

    let mixed = interleave(base.clone(), noise.clone(), 1);
    assert_eq!(mixed.len(), 23);
    let signal: Vec<RelayNote> = mixed.iter().filter(|n| !n.noise).cloned().collect();
    assert_eq!(pairs(&signal), pairs(&base), "the lessons keep their order");
    let filler: Vec<RelayNote> = mixed.iter().filter(|n| n.noise).cloned().collect();
    assert_eq!(pairs(&filler), pairs(&noise), "the noise keeps its order");
    assert_ne!(
        pairs(&mixed[..3]),
        pairs(&base),
        "the lessons are not simply left on top"
    );
    assert_eq!(
        pairs(&mixed),
        pairs(&interleave(base, noise, 1)),
        "deterministic"
    );
}

const POOL: &str = r#"
[[note]]
pair = "calc-add"
kind = "attempt"
path = "calc.py"
symbol = "add"
summary = "Swapping the operands of the subtraction does not fix add()."
body = "Tried it first."
"#;

/// Two relay tasks on the same repo and base, sharing one pool.
fn relay_specs(dir: &Path) -> Vec<TaskSpec> {
    let repo = dir.join("repo");
    std::fs::create_dir_all(&repo).unwrap();
    let (base, fix) = fixture_repo(&repo);
    std::fs::write(dir.join("notes.toml"), POOL).unwrap();
    ["calc-add", "calc-again"]
        .iter()
        .map(|name| {
            let text = spec_toml(&repo, &base, &fix)
                .replace("name = \"calc-add\"", &format!("name = \"{name}\""))
                .replace(
                    "setup_cmd = \"\"\n",
                    "setup_cmd = \"\"\nrelay_pool = \"notes.toml\"\nrelay_pair = \"calc-add\"\n",
                );
            let p = dir.join(format!("{name}.toml"));
            std::fs::write(&p, text).unwrap();
            load_spec(&p).unwrap()
        })
        .collect()
}

#[test]
fn every_task_on_one_base_gets_the_same_noise() {
    let dir = TempDir::new().unwrap();
    let mut specs = relay_specs(dir.path());
    add_noise(&mut specs, 40, 1).unwrap();
    for s in &specs {
        assert_eq!(s.relay_notes.len(), 41, "{}", s.name);
        assert_eq!(s.relay_notes.iter().filter(|n| n.noise).count(), 40);
    }
    assert_eq!(texts(&specs[0].relay_notes), texts(&specs[1].relay_notes));
}

#[test]
fn zero_noise_leaves_the_pool_as_it_was() {
    let dir = TempDir::new().unwrap();
    let mut specs = relay_specs(dir.path());
    let before = texts(&specs[0].relay_notes);
    add_noise(&mut specs, 0, 1).unwrap();
    assert_eq!(texts(&specs[0].relay_notes), before);
}

#[test]
fn noise_needs_a_relay_task() {
    let repo = TempDir::new().unwrap();
    let (base, fix) = fixture_repo(repo.path());
    let p = repo.path().join("task.toml");
    std::fs::write(&p, spec_toml(repo.path(), &base, &fix)).unwrap();
    let mut specs = vec![load_spec(&p).unwrap()];
    let err = add_noise(&mut specs, 10, 1).unwrap_err();
    assert!(err.contains("relay_pool"), "{err}");
    add_noise(&mut specs, 0, 1).expect("no noise asked, nothing to refuse");
}
