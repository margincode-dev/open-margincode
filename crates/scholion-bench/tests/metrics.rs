mod common;

use common::*;
use scholion_bench::events::parse_stream;
use scholion_bench::metrics::compute;
use scholion_bench::spec::load_spec;
use tempfile::TempDir;

fn spec() -> (TempDir, scholion_bench::spec::TaskSpec) {
    let repo = TempDir::new().unwrap();
    let (base, fix) = fixture_repo(repo.path());
    let p = repo.path().join("task.toml");
    std::fs::write(&p, spec_toml(repo.path(), &base, &fix)).unwrap();
    let s = load_spec(&p).unwrap();
    (repo, s)
}

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "{}/tests/fixtures/{name}",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap()
}

#[test]
fn first_edit_counts_only_tokens_before_it_excluding_cache_reads() {
    let (_r, s) = spec();
    let m = compute(&s, &parse_stream(&fixture("edit_first.jsonl")), 2.0);
    // m1 (10+100+5, counted once despite two events) + m2 (1+20+7) + m3 (2+30+8)
    assert_eq!(m.tokens_to_first_edit, Some(115 + 28 + 40));
    // the result event's usage: 14 + 155 + 90
    assert_eq!(m.tokens_total, 259);
    assert_eq!(m.cost_usd, 0.12);
    assert_eq!(m.turns, 4);
    assert_eq!(m.tool_calls, 3);
    assert!(!m.truncated);
}

#[test]
fn edit_to_non_relevant_file_does_not_count_as_first_edit() {
    let (_r, s) = spec();
    let parsed = parse_stream(&fixture("edit_first.jsonl"));
    let m = compute(&s, &parsed, 2.0);
    assert_ne!(
        m.tokens_to_first_edit,
        Some(115 + 28),
        "README.md is not relevant"
    );
}

#[test]
fn no_relevant_edit_gives_none_but_totals_are_counted() {
    let (_r, s) = spec();
    let m = compute(&s, &parse_stream(&fixture("no_edit.jsonl")), 2.0);
    assert_eq!(m.tokens_to_first_edit, None);
    assert_eq!(m.tokens_total, 14 + 111 + 20);
    assert_eq!(m.cost_usd, 0.05);
}

#[test]
fn truncated_stream_returns_prefix_metrics_and_flag() {
    let (_r, s) = spec();
    let parsed = parse_stream(&fixture("truncated.jsonl"));
    assert_eq!(parsed.bad_lines, 1, "the cut-off last line");
    let m = compute(&s, &parsed, 1.5);
    assert!(m.truncated);
    assert_eq!(m.tokens_to_first_edit, Some(115 + 19));
    assert_eq!(m.tokens_total, 115 + 19, "summed over the parsed messages");
    assert_eq!(m.cost_usd, 1.5, "charged the per-session cap");
    assert_eq!(m.turns, 2);
}

#[test]
fn dead_end_pattern_match_counts_once_per_tool_call() {
    let (_r, s) = spec();
    let m = compute(&s, &parse_stream(&fixture("dead_end.jsonl")), 2.0);
    // Edit new_string (1), MultiEdit with two matching edits (1), Write to a relative calc.py (1);
    // not: Write to notes.txt (glob), Edit whose old_string matches (removal)
    assert_eq!(m.repeated_dead_ends, 3);
}

#[test]
fn write_and_multiedit_count_as_edits() {
    let (_r, s) = spec();
    let m = compute(&s, &parse_stream(&fixture("write_created.jsonl")), 2.0);
    assert_eq!(
        m.tokens_to_first_edit,
        Some(56 + 68),
        "created file test_calc.py"
    );
    let m = compute(&s, &parse_stream(&fixture("multiedit.jsonl")), 2.0);
    assert_eq!(m.tokens_to_first_edit, Some(80));
}

#[test]
fn real_capture_parses_with_partial_message_events() {
    let (_r, mut s) = spec();
    s.relevant_files = vec!["app.py".into()];
    let parsed = parse_stream(&fixture("real_edit.jsonl"));
    assert_eq!(parsed.cwd.as_deref(), Some("/work/ws"));
    assert_eq!(parsed.bad_lines, 0);
    assert_eq!(
        parsed.messages.len(),
        3,
        "thinking and tool_use events share one id"
    );
    let m = compute(&s, &parsed, 2.0);
    assert!(m.tokens_to_first_edit.is_some());
    assert!(m.tokens_to_first_edit.unwrap() <= m.tokens_total);
    assert_eq!(m.tokens_total, 6 + 5845 + 469);
    assert_eq!(m.tool_calls, 2);
    assert_eq!(m.turns, 3);
    assert!((m.cost_usd - 0.0373412).abs() < 1e-6);
}
