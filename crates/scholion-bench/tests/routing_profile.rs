use scholion_bench::routing_profile::{Call, commands, decide, parse_results, report};

const FIXTURE: &str = include_str!("fixtures/routing-profile.jsonl");

fn call(text: &str, tag: &str, agent: &str) -> Call {
    let cells = parse_results(text).unwrap();
    decide(&cells)
        .into_iter()
        .find(|d| d.tag == tag && d.agent == agent)
        .unwrap()
        .call
}

fn av_lines(text: &str) -> Vec<String> {
    let cells = parse_results(text).unwrap();
    commands(&cells, &decide(&cells))
        .into_iter()
        .filter(|l| l.starts_with("av "))
        .collect()
}

#[test]
fn fixture_picks_a_best_a_can_an_out_and_keeps_ties_and_short_cells() {
    assert_eq!(call(FIXTURE, "image", "grok"), Call::Best);
    assert_eq!(call(FIXTURE, "image", "claude"), Call::Can);
    assert_eq!(call(FIXTURE, "image", "codex"), Call::Out);
    assert_eq!(call(FIXTURE, "image", "muse"), Call::Inconclusive);
    assert_eq!(call(FIXTURE, "code", "grok"), Call::Inconclusive);
    assert_eq!(call(FIXTURE, "code", "claude"), Call::Inconclusive);
    assert_eq!(call(FIXTURE, "code", "codex"), Call::Can);
    assert_eq!(call(FIXTURE, "code", "muse"), Call::Can);
    assert_eq!(
        av_lines(FIXTURE),
        vec!["av agent set codex --can code".to_string()]
    );
    let text = report(FIXTURE).unwrap();
    assert!(text.contains("av agent set codex --can code"), "{text}");
    assert!(text.contains("90.0%"), "{text}");
    assert!(text.contains("wilson 95%"), "{text}");
    assert!(text.contains("profile left unchanged"), "{text}");
    assert!(!text.lines().any(|l| l.starts_with("av agent set grok")));
    assert!(!text.lines().any(|l| l.starts_with("av agent set claude")));
    assert!(!text.lines().any(|l| l.starts_with("av agent set muse")));
}

#[test]
fn a_json_array_is_the_same_shape_as_jsonl() {
    let text =
        r#"[{"tag":"math","agent":"grok","trials":10,"accepted":10,"tokens_per_accepted":100}]"#;
    assert_eq!(
        av_lines(text),
        vec!["av agent set grok --best math".to_string()]
    );
}

#[test]
fn fewer_than_ten_trials_emits_no_command() {
    let text = r#"{"tag":"math","agent":"grok","trials":9,"accepted":9,"tokens_per_accepted":100}"#;
    assert_eq!(call(text, "math", "grok"), Call::Inconclusive);
    assert!(av_lines(text).is_empty());
}

#[test]
fn an_equal_token_cost_inside_the_band_is_a_tie() {
    let text = r#"
{"tag":"ui","agent":"a","trials":10,"accepted":10,"tokens_per_accepted":5}
{"tag":"ui","agent":"b","trials":10,"accepted":10,"tokens_per_accepted":5}
"#;
    assert_eq!(call(text, "ui", "a"), Call::Inconclusive);
    assert_eq!(call(text, "ui", "b"), Call::Inconclusive);
    assert!(av_lines(text).is_empty());
}

#[test]
fn exactly_ten_points_stays_in_the_band_and_the_cheaper_one_is_best() {
    let text = r#"
{"tag":"docs","agent":"hi","trials":20,"accepted":18,"tokens_per_accepted":9}
{"tag":"docs","agent":"lo","trials":20,"accepted":16,"tokens_per_accepted":4}
"#;
    assert_eq!(call(text, "docs", "lo"), Call::Best);
    assert_eq!(call(text, "docs", "hi"), Call::Can);
}

#[test]
fn just_over_ten_points_is_not_best_even_when_cheaper() {
    let text = r#"
{"tag":"docs","agent":"hi","trials":20,"accepted":18,"tokens_per_accepted":9}
{"tag":"docs","agent":"lo","trials":20,"accepted":15,"tokens_per_accepted":1}
"#;
    assert_eq!(call(text, "docs", "hi"), Call::Best);
    assert_eq!(call(text, "docs", "lo"), Call::Can);
}

#[test]
fn exactly_half_the_highest_is_can_and_just_under_is_out() {
    let text = r#"
{"tag":"math","agent":"hi","trials":20,"accepted":16,"tokens_per_accepted":1}
{"tag":"math","agent":"mid","trials":20,"accepted":8,"tokens_per_accepted":1}
{"tag":"math","agent":"lo","trials":20,"accepted":7,"tokens_per_accepted":1}
"#;
    assert_eq!(call(text, "math", "hi"), Call::Best);
    assert_eq!(call(text, "math", "mid"), Call::Can);
    assert_eq!(call(text, "math", "lo"), Call::Out);
    assert_eq!(
        av_lines(text),
        vec![
            "av agent set hi --best math".to_string(),
            "av agent set lo".to_string(),
            "av agent set mid --can math".to_string(),
        ]
    );
}

#[test]
fn the_only_agent_in_the_band_is_best_even_without_a_token_count() {
    let text =
        r#"{"tag":"image","agent":"grok","trials":10,"accepted":7,"tokens_per_accepted":null}"#;
    assert_eq!(call(text, "image", "grok"), Call::Best);
}

#[test]
fn a_finite_cost_beats_a_missing_one_inside_the_band() {
    let text = r#"
{"tag":"image","agent":"a","trials":10,"accepted":9,"tokens_per_accepted":null}
{"tag":"image","agent":"b","trials":10,"accepted":9,"tokens_per_accepted":40}
"#;
    assert_eq!(call(text, "image", "b"), Call::Best);
    assert_eq!(call(text, "image", "a"), Call::Can);
}

#[test]
fn results_reject_an_empty_file_a_duplicate_and_accepted_past_trials() {
    assert!(parse_results("").is_err());
    assert!(parse_results("# only a comment\n").is_err());
    assert!(
        parse_results(
            r#"{"tag":"math","agent":"a","trials":1,"accepted":2,"tokens_per_accepted":1}"#
        )
        .is_err()
    );
    assert!(
        parse_results(
            r#"
{"tag":"math","agent":"a","trials":10,"accepted":1,"tokens_per_accepted":1}
{"tag":"math","agent":"a","trials":10,"accepted":1,"tokens_per_accepted":1}
"#
        )
        .is_err()
    );
}
