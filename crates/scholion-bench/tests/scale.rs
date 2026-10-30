//! `scale` (Plan 6 E2) against a real daemon on a small repo: every phase runs, every agent's
//! task merges, and the report says so.

mod common;

use common::*;
use scholion_bench::runner::Daemon;
use scholion_bench::scale::{self, ScaleArgs};
use tempfile::TempDir;

fn repo(dir: &std::path::Path) {
    git(dir, &["init", "-q", "-b", "main"]);
    std::fs::create_dir_all(dir.join("pkg")).unwrap();
    for i in 0..6 {
        std::fs::write(
            dir.join(format!("pkg/mod_{i}.py")),
            format!("def handle_{i}(x):\n    return x + {i}\n"),
        )
        .unwrap();
    }
    std::fs::write(dir.join("README.md"), "# demo\n").unwrap();
    git(dir, &["add", "-A"]);
    git(dir, &["commit", "-q", "-m", "init"]);
}

#[test]
fn scale_runs_every_phase_and_merges_every_agent() {
    let src = TempDir::new().unwrap();
    repo(src.path());
    let out = TempDir::new().unwrap();
    let d = start_daemon(None);
    let daemon = Daemon::new(&d.url, None);
    let args = ScaleArgs {
        name: "demo".into(),
        repo: src.path().to_path_buf(),
        git_source: src.path().display().to_string(),
        out: out.path().to_path_buf(),
        agents: vec![1, 4],
        samples: 3,
        reps: 2,
        daemon_pid: Some(std::process::id()),
        database_url: None,
    };
    let r = scale::run(&daemon, &args).unwrap();

    assert_eq!(r.files, 7);
    assert_eq!(r.code_files, 6);
    assert_eq!(r.indexed_files, 6);
    for op in [
        "start",
        "context",
        "checkpoint",
        "checkpoint_again",
        "why",
        "why_cached",
    ] {
        let s = r.latency.get(op).unwrap_or_else(|| panic!("no {op}"));
        assert!(s.n > 0 && s.p50_ms <= s.p95_ms, "{op}: {s:?}");
    }
    assert_eq!(r.latency["context"].n, 6);
    assert_eq!(r.levels.len(), 2);
    for l in &r.levels {
        assert_eq!(l.merged, l.agents, "{l:?}");
        assert!(l.errors.is_empty(), "{l:?}");
        assert!(l.ops["merge"].n == l.agents);
    }
    assert!(r.daemon_rss_kb.is_some_and(|kb| kb > 0));
    assert_eq!(r.workspaces, 8);
    assert!(r.workspaces_mb.is_some_and(|mb| mb > 0.0));

    let report = std::fs::read_to_string(out.path().join("report.md")).unwrap();
    assert!(report.contains("| merge |"), "{report}");
    assert!(out.path().join("scale.json").exists());
}
