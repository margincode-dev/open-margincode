mod common;

use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use common::*;
use scholion_bench::metrics::SessionMetrics;
use scholion_bench::orchestrate::{RunArgs, plan, run};
use scholion_bench::report::{Meta, Row, Status, read_rows, render_report};
use scholion_bench::runner::{Arm, RunConfig};
use scholion_bench::spec::{TaskSpec, load_spec};
use tempfile::TempDir;

fn m(first: Option<u64>, total: u64, cost: f64, dead: u32) -> SessionMetrics {
    SessionMetrics {
        tokens_to_first_edit: first,
        tokens_total: total,
        cost_usd: cost,
        turns: 5,
        tool_calls: 9,
        repeated_dead_ends: dead,
        truncated: false,
    }
}

fn row(task: &str, arm: &str, rep: u32, success: bool, metrics: SessionMetrics) -> Row {
    Row {
        task: task.into(),
        arm: arm.into(),
        rep,
        status: Status::Ok,
        reason: None,
        success,
        cost_usd: metrics.cost_usd,
        wall_secs: 60.0 * rep as f64,
        metrics: Some(metrics),
    }
}

fn fixture_rows() -> Vec<Row> {
    let mut rows = vec![
        row("alpha", "baseline", 1, true, m(Some(9000), 40000, 0.80, 1)),
        row("alpha", "baseline", 2, false, m(None, 52000, 1.10, 2)),
        row(
            "alpha",
            "av_no_packs",
            1,
            true,
            m(Some(7000), 36000, 0.70, 0),
        ),
        row(
            "alpha",
            "av_no_packs",
            2,
            true,
            m(Some(7600), 38000, 0.75, 1),
        ),
        row("alpha", "av", 1, true, m(Some(3000), 25000, 0.50, 0)),
        row("alpha", "av", 2, true, m(Some(3400), 27000, 0.55, 0)),
        row("beta", "baseline", 1, true, m(Some(12000), 61000, 1.20, 0)),
        row(
            "beta",
            "av_no_packs",
            1,
            false,
            m(Some(11000), 60000, 1.15, 0),
        ),
        row("beta", "av", 1, true, m(Some(5000), 41000, 0.90, 0)),
    ];
    rows.push(Row {
        status: Status::Invalid,
        reason: Some("the av hooks never fired".into()),
        ..row("beta", "av", 2, false, m(None, 1000, 0.10, 0))
    });
    rows.push(Row {
        status: Status::SkippedBudget,
        metrics: None,
        cost_usd: 0.0,
        wall_secs: 0.0,
        ..row("beta", "baseline", 2, false, m(None, 0, 0.0, 0))
    });
    rows
}

fn meta() -> Meta {
    Meta {
        date: "2026-10-22 20:00 UTC".into(),
        claude_version: "2.1.308 (Claude Code)".into(),
        av_rev: "abc1234".into(),
        models: vec!["claude-sonnet-5-5".into()],
        seed: 1,
        av_mcp: Some(true),
        hook_args: vec![],
    }
}

#[test]
fn report_names_the_session_start_hook_arguments() {
    let lean = Meta {
        hook_args: vec!["--no-map".into(), "--max-code=3".into()],
        ..meta()
    };
    assert!(
        render_report(&fixture_rows(), &lean).contains(" · hook: --no-map --max-code=3 · "),
        "{}",
        render_report(&fixture_rows(), &lean)
    );
    assert!(!render_report(&fixture_rows(), &meta()).contains("hook:"));
}

#[test]
fn report_says_whether_the_av_arms_had_the_mcp_server() {
    let hooks_only = Meta {
        av_mcp: Some(false),
        ..meta()
    };
    assert!(render_report(&fixture_rows(), &hooks_only).contains("av arms: hooks only"));
    assert!(render_report(&fixture_rows(), &meta()).contains("av arms: hooks + MCP"));
    let older = Meta {
        av_mcp: None,
        ..meta()
    };
    assert!(!render_report(&fixture_rows(), &older).contains("av arms:"));
}

#[test]
fn report_matches_golden_for_fixture_rows() {
    let md = render_report(&fixture_rows(), &meta());
    let golden = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/golden/report.md");
    if std::env::var_os("AV_BLESS").is_some() {
        std::fs::create_dir_all(golden.parent().unwrap()).unwrap();
        std::fs::write(&golden, &md).unwrap();
    }
    let want = std::fs::read_to_string(&golden).expect("run with AV_BLESS=1 to create the golden");
    assert_eq!(md, want);
}

#[test]
fn median_skips_none_and_prints_none_count() {
    let md = render_report(&fixture_rows(), &meta());
    // baseline: first edits 9000 and 12000 (one session without) -> median 10500, 1 none
    let line = md.lines().find(|l| l.starts_with("| baseline |")).unwrap();
    assert!(line.contains("| 10500 (1) |"), "{line}");
    assert!(md.contains("**No significance claim**"));
    assert!(md.contains("- beta · av · rep 2: the av hooks never fired"));
    assert!(md.contains("## Skipped (budget)\n\n- beta · baseline · rep 2"));
}

#[test]
fn relative_change_is_na_without_a_baseline() {
    let rows: Vec<Row> = fixture_rows()
        .into_iter()
        .filter(|r| r.arm == "av")
        .collect();
    let md = render_report(&rows, &meta());
    assert!(!md.contains("## Attribution"), "{md}");
    let mut rows = fixture_rows();
    for r in rows.iter_mut().filter(|r| r.arm == "baseline") {
        r.success = false;
    }
    let md = render_report(&rows, &meta());
    let line = md
        .lines()
        .find(|l| l.starts_with("| total: av vs baseline"))
        .unwrap();
    assert!(
        line.starts_with("| total: av vs baseline | n/a |"),
        "{line}"
    );
}

#[test]
fn plan_is_rep_major_with_seeded_arm_order() {
    let p = plan(2, &Arm::ALL, 2, 5);
    assert_eq!(p.len(), 12);
    assert!(p[..6].iter().all(|x| x.rep == 1) && p[6..].iter().all(|x| x.rep == 2));
    assert!(p[..3].iter().all(|x| x.task == 0) && p[3..6].iter().all(|x| x.task == 1));
    let mut arms: Vec<&str> = p[..3].iter().map(|x| x.arm.as_str()).collect();
    arms.sort();
    assert_eq!(arms, ["av", "av_no_packs", "baseline"]);
    assert_eq!(plan(2, &Arm::ALL, 2, 5), p, "same seed, same order");
}

const FAKE_CLAUDE: &str = r#"#!/bin/sh
echo run >> "$FAKE_MARKER"
printf 'def add(a, b):\n    return a + b\n' > calc.py
echo "{\"type\":\"system\",\"subtype\":\"init\",\"cwd\":\"$PWD\"}"
echo "{\"type\":\"result\",\"subtype\":\"success\",\"num_turns\":1,\"total_cost_usd\":0.4,\"usage\":{\"input_tokens\":1,\"output_tokens\":1}}"
"#;

struct Setup {
    _dirs: Vec<TempDir>,
    specs: Vec<TaskSpec>,
    cfg: RunConfig,
    out: PathBuf,
    marker: PathBuf,
}

fn setup() -> Setup {
    let repo = TempDir::new().unwrap();
    let (base, fix) = fixture_repo(repo.path());
    let specs_dir = TempDir::new().unwrap();
    let p = specs_dir.path().join("calc.toml");
    std::fs::write(&p, spec_toml(repo.path(), &base, &fix)).unwrap();
    let spec = load_spec(&p).unwrap();
    let mut spec2 = spec.clone();
    spec2.name = "calc-again".into();
    let tools = TempDir::new().unwrap();
    let claude = tools.path().join("claude");
    std::fs::write(&claude, FAKE_CLAUDE).unwrap();
    std::fs::set_permissions(&claude, std::fs::Permissions::from_mode(0o755)).unwrap();
    let marker = tools.path().join("runs");
    let work = TempDir::new().unwrap();
    let cfg = RunConfig {
        work_root: work.path().to_path_buf(),
        claude_bin: claude.to_string_lossy().into(),
        av_bin: "av".into(),
        av_url: None,
        av_token: None,
        import_dir: work.path().join("src"),
        import_mount: "/import".into(),
        allowed_tools: vec!["Read".into()],
        permission_mode: "acceptEdits".into(),
        session_cap_usd: 0.5,
        claude_home: None,
        hook_budget: 800,
        allow_no_semantic: true,
        index_wait_secs: 60,
        av_mcp: true,
        hook_code: None,
        hook_args: vec![],
        jit_hooks: false,
        prices: None,
    };
    let out = work.path().join("results");
    Setup {
        _dirs: vec![repo, specs_dir, tools, work],
        specs: vec![spec, spec2],
        cfg,
        out,
        marker,
    }
}

fn args(s: &Setup, budget: f64, confirm: bool) -> RunArgs {
    RunArgs {
        out: s.out.clone(),
        arms: vec![Arm::Baseline],
        reps: 2,
        budget_usd: budget,
        seed: 1,
        confirm,
        dry_run: false,
        est_session_usd: Some(0.3),
        pilot: None,
        meta: meta(),
    }
}

// FAKE_MARKER is process-wide.
static SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[test]
fn budget_cap_skips_remaining_sessions_and_exits_nonzero() {
    let _g = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let s = setup();
    // SAFETY: SERIAL is held.
    unsafe { std::env::set_var("FAKE_MARKER", &s.marker) };
    // 4 sessions at $0.40 each, cap $0.50: 0+.5, .4+.5 fit in $1.00; .8+.5 does not
    let r = run(&s.specs, &args(&s, 1.0, true), &s.cfg).unwrap();
    assert_eq!(r.exit_code, 2);
    let launched = std::fs::read_to_string(&s.marker).unwrap().lines().count();
    assert_eq!(launched, 2);
    let rows = read_rows(&s.out.join("rows.jsonl")).unwrap();
    assert_eq!(rows.len(), 4);
    assert_eq!(
        rows.iter()
            .filter(|r| r.status == Status::SkippedBudget)
            .count(),
        2
    );
    assert!(
        rows.iter()
            .filter(|r| r.status == Status::Ok)
            .all(|r| r.success)
    );
    let report = std::fs::read_to_string(s.out.join("report.md")).unwrap();
    assert!(report.contains("## Skipped (budget)"), "{report}");

    // a rerun with more budget resumes: only the two skipped sessions launch
    let r = run(&s.specs, &args(&s, 5.0, true), &s.cfg).unwrap();
    assert_eq!(r.exit_code, 0);
    let launched = std::fs::read_to_string(&s.marker).unwrap().lines().count();
    assert_eq!(launched, 4);
    let rows = read_rows(&s.out.join("rows.jsonl")).unwrap();
    assert_eq!(rows.len(), 4);
    assert!(rows.iter().all(|r| r.status == Status::Ok), "{rows:?}");
}

#[test]
fn dry_run_prints_plan_with_session_count_and_estimated_cost_without_running() {
    let _g = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let s = setup();
    // SAFETY: SERIAL is held.
    unsafe { std::env::set_var("FAKE_MARKER", &s.marker) };
    for a in [
        RunArgs {
            dry_run: true,
            ..args(&s, 10.0, true)
        },
        args(&s, 10.0, false),
    ] {
        let r = run(&s.specs, &a, &s.cfg).unwrap();
        assert_eq!(r.exit_code, 3);
        assert!(
            r.plan_text
                .contains("2 tasks × 1 arms × 2 reps = 4 sessions"),
            "{}",
            r.plan_text
        );
        assert!(
            r.plan_text.contains("estimated cost: $1.20"),
            "{}",
            r.plan_text
        );
        assert!(r.plan_text.contains("worst case $2.00"), "{}", r.plan_text);
        assert!(r.plan_text.contains("below 30"), "{}", r.plan_text);
    }
    assert!(!s.marker.exists(), "nothing launched");
    assert!(!s.out.exists());
}

#[test]
fn av_arms_check_the_daemon_before_the_first_session() {
    let _g = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let mut s = setup();
    // SAFETY: SERIAL is held.
    unsafe { std::env::set_var("FAKE_MARKER", &s.marker) };
    s.cfg.av_url = Some("http://127.0.0.1:9".into());
    let a = RunArgs {
        arms: vec![Arm::Baseline, Arm::Av],
        ..args(&s, 10.0, true)
    };
    let err = run(&s.specs, &a, &s.cfg).err().unwrap();
    assert!(err.contains("not up"), "{err}");
    assert!(!s.marker.exists(), "not even the baseline sessions ran");
}

#[test]
fn a_relay_report_with_a_cold_arm_compares_every_channel_with_it() {
    let rows = vec![
        row("relay-a", "cold", 1, false, m(Some(9000), 40000, 0.02, 1)),
        row(
            "relay-a",
            "baseline",
            1,
            true,
            m(Some(9000), 40000, 0.02, 1),
        ),
        row("relay-a", "notes", 1, true, m(Some(8000), 38000, 0.02, 1)),
        row("relay-a", "av", 1, true, m(Some(7000), 36000, 0.02, 0)),
    ];
    let md = render_report(&rows, &meta());
    assert!(md.contains("| cold | 0/1 |"), "{md}");
    for label in [
        "relay: baseline vs cold",
        "relay: notes vs cold",
        "relay: av vs cold",
    ] {
        assert!(md.contains(&format!("| {label} |")), "{label}: {md}");
    }
    assert!(md.contains("`cold`"), "explains the cold arm: {md}");
    let cold = md.find("| cold |").unwrap();
    assert!(cold < md.find("| baseline |").unwrap(), "cold first: {md}");
}

#[test]
fn a_relay_report_compares_av_with_notes_and_says_how_the_notes_arrived() {
    let rows = vec![
        row(
            "relay-a",
            "baseline",
            1,
            false,
            m(Some(9000), 40000, 0.02, 1),
        ),
        row("relay-a", "notes", 1, true, m(Some(8000), 38000, 0.02, 1)),
        row("relay-a", "av", 1, true, m(Some(7000), 36000, 0.02, 0)),
    ];
    let md = render_report(&rows, &meta());
    assert!(md.contains("| notes | 1/1 |"), "{md}");
    assert!(md.contains("| relay: av vs notes |"), "{md}");
    assert!(md.contains("| relay: notes vs baseline |"), "{md}");
    assert!(md.contains("NOTES.md"), "explains the channels: {md}");
    assert!(
        !md.contains("single-commit tree"),
        "the E3 note does not apply: {md}"
    );
}
