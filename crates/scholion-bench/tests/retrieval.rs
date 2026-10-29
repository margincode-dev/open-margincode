mod common;

use std::path::Path;

use common::*;
use scholion_bench::calibrate::cl100k_tokens;
use scholion_bench::mine::{MineArgs, goal_from_message, housekeeping, mine};
use scholion_bench::retrieval::{
    Repo, RetrievalArgs, RetrievalTask, bm25, fit, goal_terms, gold, hunk_ranges, is_test_path,
    repo_map, run, score, words,
};
use scholion_bench::runner::Daemon;
use tempfile::TempDir;

const GREETER: &str = "from app.util import format_name\n\n\nclass Greeter:\n    def hello(self, name):\n        return \"Hello, \" + format_name(name)\n\n    def goodbye(self, name):\n        return \"Goodbye, \" + format_name(name)\n";
const GREETER_FIX: &str = "from app.util import format_name\n\n\nclass Greeter:\n    def hello(self, name):\n        return \"Hello, \" + format_name(name) + \"!\"\n\n    def goodbye(self, name):\n        return \"Goodbye, \" + format_name(name)\n";
const UTIL: &str = "def format_name(n):\n    return n.strip().title()\n";
const BILLING: &str = "def invoice_total(lines):\n    return sum(l.amount for l in lines)\n\n\ndef apply_discount(total, pct):\n    return total * (1 - pct / 100)\n";

/// base: greeter, util, billing. fix: `Greeter.hello` gains a "!" and a test file appears.
/// Then a docs-only commit. Returns (base, fix).
fn repo(dir: &Path) -> (String, String) {
    git(dir, &["init", "-q", "-b", "main"]);
    std::fs::create_dir_all(dir.join("app")).unwrap();
    std::fs::write(dir.join("app/greeter.py"), GREETER).unwrap();
    std::fs::write(dir.join("app/util.py"), UTIL).unwrap();
    std::fs::write(dir.join("app/billing.py"), BILLING).unwrap();
    std::fs::write(dir.join("README.md"), "# demo\n").unwrap();
    git(dir, &["add", "-A"]);
    git(dir, &["commit", "-q", "-m", "base"]);
    let base = git(dir, &["rev-parse", "HEAD"]);
    std::fs::write(dir.join("app/greeter.py"), GREETER_FIX).unwrap();
    std::fs::create_dir_all(dir.join("tests")).unwrap();
    std::fs::write(
        dir.join("tests/test_greeter.py"),
        "def test_bang():\n    assert True\n",
    )
    .unwrap();
    git(dir, &["add", "-A"]);
    git(
        dir,
        &[
            "commit",
            "-q",
            "-m",
            "Greetings end with an exclamation mark\n\nCustomers asked for warmer greetings.\n\nCo-Authored-By: Someone <a@b.c>",
        ],
    );
    let fix = git(dir, &["rev-parse", "HEAD"]);
    std::fs::write(dir.join("README.md"), "# demo\n\nmore\n").unwrap();
    git(dir, &["commit", "-q", "-am", "docs: readme"]);
    (base, fix)
}

#[test]
fn words_split_identifiers_and_goal_terms_drop_stop_words() {
    let w = words("parseHTTPRequest snake_case");
    for want in [
        "parsehttprequest",
        "parse",
        "http",
        "request",
        "snake",
        "case",
    ] {
        assert!(w.contains(&want.to_owned()), "{want} missing from {w:?}");
    }
    assert_eq!(
        goal_terms("Make the greeter add an exclamation mark, the greeter!"),
        vec!["greeter", "exclamation", "mark"]
    );
}

#[test]
fn test_paths_are_recognised() {
    for p in [
        "tests/x.py",
        "src/app_test.py",
        "web/foo.test.ts",
        "a/__tests__/b.tsx",
        "test_x.py",
    ] {
        assert!(is_test_path(p), "{p}");
    }
    for p in ["src/latest.py", "src/testing_utils.py", "src/contest.rs"] {
        assert!(!is_test_path(p), "{p}");
    }
}

#[test]
fn hunk_ranges_read_the_old_side() {
    let d = "@@ -5,2 +5,3 @@ x\n-a\n@@ -9 +9 @@\n@@ -3,0 +4,2 @@\n";
    assert_eq!(hunk_ranges(d), vec![(5, 6), (9, 9), (3, 4)]);
}

#[test]
fn gold_is_the_innermost_changed_symbol_of_modified_source_files() {
    let d = TempDir::new().unwrap();
    let (base, fix) = repo(d.path());
    let g = gold(
        d.path(),
        &base,
        &fix,
        "Greetings end with an exclamation mark",
    )
    .unwrap();
    assert_eq!(
        g.files,
        vec!["app/greeter.py"],
        "the added test file is not gold"
    );
    assert_eq!(g.symbols.len(), 1, "{:?}", g.symbols);
    assert_eq!(g.symbols[0].qname, "Greeter.hello");
    assert_eq!(g.symbols[0].signature, "def hello(self, name):");
    assert_eq!(
        g.symbols[0].body,
        vec!["return \"Hello, \" + format_name(name)"]
    );
    assert!(!g.leak);
    let leaky = gold(d.path(), &base, &fix, "hello() should end with a bang").unwrap();
    assert!(leaky.leak, "the goal names the gold symbol");
}

#[test]
fn scores_count_shown_code_not_mere_mentions() {
    let d = TempDir::new().unwrap();
    let (base, fix) = repo(d.path());
    let g = gold(d.path(), &base, &fix, "x").unwrap();
    git(d.path(), &["checkout", "-q", &base]);
    let r = Repo::load(d.path()).unwrap();

    let full = score(GREETER, &g, &r);
    assert_eq!(full.files_code, 1.0);
    assert_eq!(full.symbols_sig, 1.0);
    assert_eq!(full.symbols_body, 1.0);
    assert_eq!(full.files_named, 0.0, "the path itself is not in the text");
    assert!(full.gold_share > 0.0 && full.gold_share <= 1.0);

    let mention = score("see app/greeter.py for the greeting", &g, &r);
    assert_eq!(mention.files_named, 1.0);
    assert_eq!(mention.files_code, 0.0);
    assert_eq!(mention.symbols_sig, 0.0);

    let map = score("app/greeter.py:\n  L5: def hello(self, name):\n", &g, &r);
    assert_eq!(map.symbols_sig, 1.0, "a repo-map line shows the signature");
    assert_eq!(
        map.symbols_body, 1.0,
        "a one-line body counts through its signature"
    );
    assert_eq!(map.first_gold, Some(0.0));

    // the index drops the opener (`:` here, ` {` in Rust and TS) from signatures
    let sig = score(
        "- sym:9 Greeter.hello (app/greeter.py:5-6)\n  def hello(self, name)\n",
        &g,
        &r,
    );
    assert_eq!(sig.symbols_sig, 1.0, "a pack's signature line shows it");

    let none = score(BILLING, &g, &r);
    assert_eq!(none.files_code + none.symbols_sig + none.gold_share, 0.0);
    assert_eq!(none.first_gold, None);
}

#[test]
fn fit_never_exceeds_the_budget_and_cuts_the_first_item_that_overflows() {
    let items: Vec<String> = (0..50)
        .map(|i| format!("line {i} of some reasonably long text here\n").repeat(5))
        .collect();
    let out = fit(&items, 120);
    assert!(cl100k_tokens(&out) <= 120, "{}", cl100k_tokens(&out));
    assert!(cl100k_tokens(&out) > 80, "the budget is mostly used");
    assert_eq!(fit(&items[..1], 10_000), items[0]);
}

#[test]
fn bm25_ranks_by_term_weight_and_drops_non_matches() {
    let docs = vec![
        vec!["apple".to_owned(), "pie".to_owned()],
        vec!["banana".to_owned()],
        vec!["apple".to_owned(), "apple".to_owned()],
    ];
    let r = bm25(&docs, &["apple".to_owned()]);
    assert_eq!(r.iter().map(|(i, _)| *i).collect::<Vec<_>>(), vec![2, 0]);
}

#[test]
fn repo_map_puts_the_goal_file_first() {
    let d = TempDir::new().unwrap();
    let (base, _) = repo(d.path());
    git(d.path(), &["checkout", "-q", &base]);
    let r = Repo::load(d.path()).unwrap();
    let map = repo_map(&r, "invoice discount is applied twice", 2000);
    assert!(map.starts_with("app/billing.py:\n"), "{map}");
    assert!(
        map.contains("  L5: def apply_discount(total, pct):"),
        "{map}"
    );
}

#[test]
fn commit_messages_become_goals_without_trailers_or_links() {
    let g = goal_from_message(
        "Fix crash on empty input",
        "Seen in https://example.com/issue/1 today.\n\n\nSigned-off-by: A <a@b.c>\nCo-Authored-By: B <b@c.d>\n",
    );
    assert_eq!(g, "Fix crash on empty input\n\nSeen in  today.");
    assert!(housekeeping("Bump serde to 1.0.200"));
    assert!(housekeeping("docs: explain packs"));
    assert!(housekeeping("Upgrade Python formatter ruff (#3651)"));
    assert!(!housekeeping("Fix crash in parser"));
}

#[test]
fn mine_keeps_small_source_commits_only() {
    let d = TempDir::new().unwrap();
    let (base, fix) = repo(d.path());
    let tasks = mine(
        d.path(),
        &MineArgs {
            repo_path: "~/eval-repos/demo".into(),
            prefix: "demo".into(),
            since: None,
            max_files: 3,
            max_lines: 200,
            limit: 10,
        },
    )
    .unwrap();
    assert_eq!(
        tasks.len(),
        1,
        "base has no parent, docs is housekeeping: {tasks:?}"
    );
    let t = &tasks[0];
    assert_eq!(t.name, format!("demo-{}", &fix[..7]));
    assert_eq!(t.fix_rev, fix);
    assert_eq!(git(d.path(), &["rev-parse", &t.base_rev]), base);
    assert_eq!(
        t.goal,
        "Greetings end with an exclamation mark\n\nCustomers asked for warmer greetings."
    );
}

fn task(dir: &Path, base: &str, fix: &str, goal: &str) -> RetrievalTask {
    RetrievalTask {
        name: "greet".into(),
        repo_path: dir.to_path_buf(),
        base_rev: base.into(),
        fix_rev: fix.into(),
        goal: goal.into(),
    }
}

fn args(out: &Path, import: &Path, no_pack: bool) -> RetrievalArgs {
    RetrievalArgs {
        budgets: vec![300, 2000],
        out: out.to_path_buf(),
        import_dir: import.to_path_buf(),
        import_mount: import.to_string_lossy().into(),
        allow_no_semantic: true,
        index_wait_secs: 30,
        no_pack,
        reuse_run: None,
    }
}

#[test]
fn baselines_run_without_a_daemon_and_write_results() {
    let d = TempDir::new().unwrap();
    let (base, fix) = repo(d.path());
    let out = TempDir::new().unwrap();
    let import = TempDir::new().unwrap();
    let t = task(
        d.path(),
        &base,
        &fix,
        "Greetings end with an exclamation mark",
    );
    let rows = run(&[t], None, &args(out.path(), import.path(), true)).unwrap();
    assert_eq!(rows.len(), 6, "3 baselines x 2 budgets");
    assert!(rows.iter().all(|r| r.scores.tokens <= u64::from(r.budget)));
    for f in [
        "rows.jsonl",
        "gold.jsonl",
        "meta.json",
        "report.md",
        "texts/greet/repo_map-300.md",
    ] {
        assert!(out.path().join(f).exists(), "{f}");
    }
    let report = std::fs::read_to_string(out.path().join("report.md")).unwrap();
    assert!(report.contains("| bm25_files |"), "{report}");
    assert!(
        std::fs::read_dir(import.path()).unwrap().next().is_none(),
        "the scrubbed copy is removed"
    );
}

#[test]
fn packs_come_from_the_daemon_within_budget() {
    let d = TempDir::new().unwrap();
    let (base, fix) = repo(d.path());
    let out = TempDir::new().unwrap();
    let import = TempDir::new().unwrap();
    let td = start_daemon(None);
    let daemon = Daemon::new(&td.url, None);
    let t = task(
        d.path(),
        &base,
        &fix,
        "Greeter.hello should end with an exclamation mark",
    );
    let rows = run(&[t], Some(&daemon), &args(out.path(), import.path(), false)).unwrap();
    let packs: Vec<_> = rows.iter().filter(|r| r.method == "pack").collect();
    assert_eq!(packs.len(), 2);
    for p in &packs {
        assert!(
            p.scores.tokens > 0 && p.scores.tokens <= u64::from(p.budget),
            "{p:?}"
        );
    }
    let text = std::fs::read_to_string(out.path().join("texts/greet/pack-2000.md")).unwrap();
    assert!(
        text.starts_with("## Task") && text.contains("Goal: Greeter.hello"),
        "the daemon's first pack for the goal: {text}"
    );
    let report = std::fs::read_to_string(out.path().join("report.md")).unwrap();
    assert!(report.contains("Pack against each baseline"), "{report}");
    // the packs agents get: signatures only, scored beside the full ones
    let sigs: Vec<_> = rows.iter().filter(|r| r.method == "pack_sig").collect();
    assert_eq!(sigs.len(), 2);
    let sig = std::fs::read_to_string(out.path().join("texts/greet/pack_sig-2000.md")).unwrap();
    assert!(sig.contains("def hello(self, name):"), "{sig}");
    assert!(!sig.contains("```"), "no code bodies: {sig}");
    assert!(text.contains("```"), "the full pack has bodies: {text}");
    assert!(report.contains("| pack_sig | bm25_symbols |"), "{report}");
}

#[test]
fn a_rerun_reuses_the_repos_an_earlier_run_imported() {
    let d = TempDir::new().unwrap();
    let (base, fix) = repo(d.path());
    let import = TempDir::new().unwrap();
    let td = start_daemon(None);
    let daemon = Daemon::new(&td.url, None);
    let goal = "Greeter.hello should end with an exclamation mark";
    let first = TempDir::new().unwrap();
    run(
        &[task(d.path(), &base, &fix, goal)],
        Some(&daemon),
        &args(first.path(), import.path(), false),
    )
    .unwrap();
    let meta: serde_json::Value =
        serde_json::from_slice(&std::fs::read(first.path().join("meta.json")).unwrap()).unwrap();
    let id = meta["run_id"]
        .as_str()
        .expect("meta names the run")
        .to_owned();

    let again = TempDir::new().unwrap();
    let mut a = args(again.path(), import.path(), false);
    // an import would fail: the daemon cannot read this mount
    a.import_mount = "/nowhere".into();
    a.reuse_run = Some(id);
    let rows = run(&[task(d.path(), &base, &fix, goal)], Some(&daemon), &a).unwrap();
    assert_eq!(rows.iter().filter(|r| r.method == "pack").count(), 2);
    let text = |dir: &Path| std::fs::read_to_string(dir.join("texts/greet/pack-2000.md")).unwrap();
    // same repo, new task: only the task's slug and its symbol ids differ
    let code = |dir: &Path| {
        let t = text(dir);
        let c = t[t.find("## Code").unwrap()..].to_owned();
        c.lines()
            .map(|l| match l.strip_prefix("- sym:") {
                Some(r) => r.split_once(' ').map_or(r, |(_, n)| n).to_owned(),
                None => l.to_owned(),
            })
            .filter(|l| !l.starts_with("tokens:"))
            .collect::<Vec<_>>()
    };
    assert!(text(again.path()).contains("Goal: Greeter.hello"));
    assert_eq!(code(again.path()), code(first.path()));
}
