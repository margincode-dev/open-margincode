//! CI regression gate: the pack's retrieval scores on the checked-in `bench/retrieval/ci/` repository may not fall
//! below checked-in floors. No semantic layer (a hash embedder carries no meaning), so it guards the lexical,
//! structural and budgeting parts of the packer only; `bench/retrieval/README.md` says how to update the floors.

use std::path::{Path, PathBuf};

use crate::common::*;
use margincode_bench::retrieval::{RetrievalArgs, RetrievalTask, Row, Scores, run};
use margincode_bench::runner::Daemon;
use tempfile::TempDir;

/// A pack mean may fall this far below what was measured when the floors were set before the gate
/// fails: the fixture is small (one gold symbol is worth about 0.1), and a deliberate change to
/// the packer moves several tasks at once.
const MARGIN: f64 = 0.03;

/// Tight on purpose: in this 12-file fixture everything fits from about 1000 tokens and any ranking scores 1.0.
const BUDGETS: [u32; 3] = [300, 450, 600];

/// Each mean over the 9 fixture tasks as measured when the gate was added, minus `MARGIN`,
/// rounded down: (budget, files_named, files_code, symbols_sig).
///
/// measured: 300 -> 1.000 / 0.556 / 0.444; 450 -> 1.000 / 0.833 / 0.556; 600 -> 1.000 / 1.000 / 0.778
/// (`files_named` stays at 1.0 because the pack's map lists every path of a repo this small).
/// Re-measured when the task item's handle began to carry the goal and was reserved (F17): 300 rose
/// from 0.444 / 0.333, and 600's `symbols_sig` fell from 0.833 (one task's signature, half of it).
#[rustfmt::skip]
const FLOORS: [(u32, f64, f64, f64); 3] = [
    (300, 0.97, 0.52, 0.41),
    (450, 0.97, 0.80, 0.52),
    (600, 0.97, 0.97, 0.74),
];

fn ci_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../bench/retrieval/ci")
}

fn copy_tree(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for e in std::fs::read_dir(from).unwrap() {
        let e = e.unwrap();
        let dest = to.join(e.file_name());
        if e.file_type().unwrap().is_dir() {
            copy_tree(&e.path(), &dest);
        } else {
            std::fs::copy(e.path(), dest).unwrap();
        }
    }
}

fn tasks(dir: &Path) -> Vec<RetrievalTask> {
    let ci = ci_dir();
    copy_tree(&ci.join("repo"), dir);
    git(dir, &["init", "-q", "-b", "main"]);
    git(dir, &["add", "-A"]);
    git(dir, &["commit", "-q", "-m", "base"]);
    let base = git(dir, &["rev-parse", "HEAD"]);
    let lines = std::fs::read_to_string(ci.join("tasks.jsonl")).unwrap();
    let mut out = vec![];
    for l in lines.lines().filter(|l| !l.trim().is_empty()) {
        let v: serde_json::Value = serde_json::from_str(l).unwrap();
        let name = v["name"].as_str().unwrap();
        git(dir, &["checkout", "-q", "-b", name, &base]);
        let patch = ci.join("fixes").join(format!("{name}.patch"));
        git(dir, &["apply", patch.to_str().unwrap()]);
        git(dir, &["commit", "-q", "-am", name]);
        out.push(RetrievalTask {
            name: name.to_owned(),
            repo_path: dir.to_path_buf(),
            base_rev: base.clone(),
            fix_rev: git(dir, &["rev-parse", "HEAD"]),
            goal: v["goal"].as_str().unwrap().to_owned(),
        });
    }
    git(dir, &["checkout", "-q", "main"]);
    out
}

fn mean(rows: &[&Row], f: impl Fn(&Row) -> f64) -> f64 {
    rows.iter().map(|r| f(r)).sum::<f64>() / rows.len() as f64
}

/// The gate's printed report and the floors it fell below.
fn gate_report(packs: &[&Row]) -> (String, Vec<String>) {
    let mut report = String::new();
    let mut failed = vec![];
    for (budget, named, code, sig) in FLOORS {
        let at: Vec<&Row> = packs
            .iter()
            .copied()
            .filter(|r| r.budget == budget)
            .collect();
        for (metric, floor, got) in [
            ("files_named", named, mean(&at, |r| r.scores.files_named)),
            ("files_code", code, mean(&at, |r| r.scores.files_code)),
            ("symbols_sig", sig, mean(&at, |r| r.scores.symbols_sig)),
        ] {
            report += &format!("pack {metric} at {budget}: {got:.3} (floor {floor:.2})\n");
            if got < floor {
                failed.push(format!("{metric} at {budget}: {got:.3} < {floor:.2}"));
            }
        }
        let mut by_task = at;
        by_task.sort_by(|a, b| a.task.cmp(&b.task));
        for r in by_task {
            report += &format!(
                "  {budget} {}: files_named {:.3} files_code {:.3} symbols_sig {:.3} of {}\n",
                r.task,
                r.scores.files_named,
                r.scores.files_code,
                r.scores.symbols_sig,
                r.gold_symbols
            );
        }
    }
    (report, failed)
}

#[test]
fn the_pack_keeps_its_retrieval_scores_on_the_fixture_repo() {
    let repo = TempDir::new().unwrap();
    let ts = tasks(repo.path());
    let out = TempDir::new().unwrap();
    let import = TempDir::new().unwrap();
    let td = start_daemon(None);
    let daemon = Daemon::new(&td.url, None);
    let args = RetrievalArgs {
        budgets: BUDGETS.to_vec(),
        out: out.path().to_path_buf(),
        import_dir: import.path().to_path_buf(),
        import_mount: import.path().to_string_lossy().into(),
        allow_no_semantic: true,
        index_wait_secs: 30,
        no_pack: false,
        reuse_run: None,
    };
    let rows = run(&ts, Some(&daemon), &args).unwrap();
    let packs: Vec<&Row> = rows.iter().filter(|r| r.method == "pack").collect();
    assert_eq!(
        packs.len(),
        ts.len() * BUDGETS.len(),
        "every fixture task must be scored"
    );

    let (report, failed) = gate_report(&packs);
    eprintln!("{report}");
    assert!(
        failed.is_empty(),
        "the pack's retrieval fell below the E1 CI floors:\n  {}\n\n\
         If this change should lower retrieval on purpose, re-measure with\n  \
         cargo test -p margincode-bench --test it retrieval_gate:: -- --nocapture\n\
         and set FLOORS in {} to each printed value minus {MARGIN}, rounded down, and say why in the\n\
         commit message. Otherwise fix the regression. The indented lines of the report name each\n\
         task's scores; per-task texts: run `margincode-bench retrieval` (bench/retrieval/README.md)\n\
         on the fixture.\n",
        failed.join("\n  "),
        file!()
    );
}

fn row(task: &str, budget: u32, sig: f64) -> Row {
    Row {
        task: task.into(),
        method: "pack".into(),
        budget,
        leak: false,
        gold_files: 1,
        gold_symbols: 2,
        degraded: vec![],
        scores: Scores {
            files_named: 1.0,
            files_code: sig,
            symbols_sig: sig,
            ..Scores::default()
        },
    }
}

#[test]
fn the_gate_report_names_each_task_and_its_lines_add_up_to_the_means() {
    let mut rows = vec![];
    for b in BUDGETS {
        for (task, sig) in [("c", 0.5), ("a", 1.0), ("b", 0.0)] {
            rows.push(row(task, b, sig));
        }
    }
    let packs: Vec<&Row> = rows.iter().collect();
    let (report, _) = gate_report(&packs);
    let value = |line: &str, metric: &str| -> f64 {
        let words: Vec<&str> = line.split_whitespace().collect();
        let at = words.iter().position(|w| *w == metric).unwrap();
        words[at + 1].parse().unwrap()
    };
    for b in BUDGETS {
        let prefix = format!("  {b} ");
        let lines: Vec<&str> = report
            .lines()
            .filter_map(|l| l.strip_prefix(prefix.as_str()))
            .collect();
        let names: Vec<&str> = lines.iter().map(|l| l.split(':').next().unwrap()).collect();
        assert_eq!(
            names,
            ["a", "b", "c"],
            "one line per task at {b}, sorted:\n{report}"
        );
        for metric in ["files_named", "files_code", "symbols_sig"] {
            let per_task = lines.iter().map(|l| value(l, metric)).sum::<f64>() / lines.len() as f64;
            let head = format!("pack {metric} at {b}:");
            let line = report.lines().find(|l| l.starts_with(&head)).unwrap();
            let reported: f64 = line.split_whitespace().nth(4).unwrap().parse().unwrap();
            assert!(
                (per_task - reported).abs() < 0.001,
                "{metric} at {b}: tasks average {per_task:.3}, the report says {reported:.3}\n{report}"
            );
        }
    }
}
