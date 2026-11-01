//! `AgentCommand`: launch, environment, budget flag, deadline and output capture, with a shell
//! script standing in for the agent CLI.

use std::path::PathBuf;
use std::time::Duration;

use scholion_agentrun::AgentCommand;

fn sh(script: &str, dir: &std::path::Path) -> AgentCommand {
    AgentCommand::new("sh")
        .args(["-c", script, "agent"])
        .cwd(dir)
        .timeout(Duration::from_secs(20))
        .events_path(dir.join("events.jsonl"))
}

fn events(dir: &std::path::Path) -> String {
    std::fs::read_to_string(dir.join("events.jsonl")).unwrap()
}

#[test]
fn stdout_is_the_events_file_and_usage_is_totalled_over_messages() {
    let dir = tempfile::tempdir().unwrap();
    let script = r#"
echo '{"type":"system","subtype":"init","cwd":"/w"}'
echo '{"type":"assistant","message":{"id":"a","content":[],"usage":{"input_tokens":10,"cache_creation_input_tokens":1,"cache_read_input_tokens":100,"output_tokens":5}}}'
echo '{"type":"assistant","message":{"id":"b","content":[],"usage":{"input_tokens":20,"cache_creation_input_tokens":2,"cache_read_input_tokens":200,"output_tokens":7}}}'
echo '{"type":"result","total_cost_usd":0.25,"num_turns":2,"is_error":false,"usage":{}}'
exit 4
"#;
    let r = sh(script, dir.path()).run().unwrap();
    assert_eq!((r.exit, r.killed), (Some(4), false));
    assert_eq!(r.events_path, dir.path().join("events.jsonl"));
    assert_eq!(events(dir.path()).lines().count(), 4);
    assert_eq!(
        (
            r.usage.input,
            r.usage.cache_creation,
            r.usage.cache_read,
            r.usage.output
        ),
        (30, 3, 300, 12),
        "an empty result usage falls back to the sum over the two messages"
    );
    assert!(!r.session.truncated());
    assert_eq!(r.session.result.as_ref().unwrap().cost_usd, 0.25);
    assert!(r.wall > Duration::ZERO);
}

#[test]
fn the_result_events_usage_is_the_total_when_it_differs_from_the_message_sum() {
    let dir = tempfile::tempdir().unwrap();
    let script = r#"
echo '{"type":"assistant","message":{"id":"a","content":[],"usage":{"input_tokens":10,"cache_creation_input_tokens":1,"cache_read_input_tokens":100,"output_tokens":5}}}'
echo '{"type":"assistant","message":{"id":"b","content":[],"usage":{"input_tokens":20,"cache_creation_input_tokens":2,"cache_read_input_tokens":200,"output_tokens":7}}}'
echo '{"type":"result","total_cost_usd":0.25,"num_turns":2,"is_error":false,"usage":{"input_tokens":40,"cache_creation_input_tokens":4,"cache_read_input_tokens":400,"output_tokens":90}}'
"#;
    let r = sh(script, dir.path()).run().unwrap();
    assert_eq!(
        (
            r.usage.input,
            r.usage.cache_creation,
            r.usage.cache_read,
            r.usage.output
        ),
        (40, 4, 400, 90),
        "the result event's total, not the message sum (30, 3, 300, 12)"
    );
}

#[test]
fn stderr_goes_to_its_own_file_when_asked() {
    let dir = tempfile::tempdir().unwrap();
    let err = dir.path().join("agent.stderr");
    sh("echo out; echo oops >&2", dir.path())
        .stderr_path(&err)
        .run()
        .unwrap();
    assert_eq!(events(dir.path()).trim(), "out");
    assert_eq!(std::fs::read_to_string(err).unwrap().trim(), "oops");
}

#[test]
fn the_budget_flag_comes_last_and_only_when_set() {
    let dir = tempfile::tempdir().unwrap();
    let script = r#"echo "$@""#;
    sh(script, dir.path())
        .args(["-p", "goal"])
        .max_budget_usd(Some(1.5))
        .run()
        .unwrap();
    assert_eq!(events(dir.path()).trim(), "-p goal --max-budget-usd 1.50");
    sh(script, dir.path())
        .args(["-p", "goal"])
        .max_budget_usd(None)
        .run()
        .unwrap();
    assert_eq!(events(dir.path()).trim(), "-p goal");
}

#[test]
fn env_is_applied_in_order_removals_included_and_the_cwd_is_used() {
    let dir = tempfile::tempdir().unwrap();
    // SAFETY-free: set in the child only; the parent value is what env_remove must hide
    let script = r#"echo "${FOO-unset}|${BAR-unset}|${BAZ-unset}|$(pwd)""#;
    let r = sh(script, dir.path())
        .env("FOO", "1")
        .env("BAR", "2")
        .env_remove("BAR")
        .env("BAZ", "3")
        .env_remove("BAZ")
        .env("BAZ", "4")
        .run()
        .unwrap();
    assert_eq!(r.exit, Some(0));
    let here = dir.path().canonicalize().unwrap();
    assert_eq!(
        events(dir.path()).trim(),
        format!("1|unset|4|{}", here.display())
    );
}

#[test]
fn path_prepend_puts_the_directories_first() {
    let dir = tempfile::tempdir().unwrap();
    let a = PathBuf::from("/first/bin");
    let b = PathBuf::from("/second/bin");
    sh("echo $PATH", dir.path())
        .path_prepend(vec![a, b])
        .run()
        .unwrap();
    let out = events(dir.path());
    assert!(out.trim().starts_with("/first/bin:/second/bin:"), "{out}");
}

#[test]
fn a_session_past_its_deadline_is_killed_and_reads_as_truncated() {
    let dir = tempfile::tempdir().unwrap();
    let r = sh(
        r#"echo '{"type":"system","subtype":"init","cwd":"/w"}'
echo '{"type":"assistant","message":{"id":"a","content":[],"usage":{"input_tokens":10,"cache_creation_input_tokens":1,"cache_read_input_tokens":100,"output_tokens":5}}}'
sleep 60"#,
        dir.path(),
    )
    .timeout(Duration::from_millis(400))
    .run()
    .unwrap();
    assert_eq!((r.exit, r.killed), (None, true));
    assert!(r.session.truncated());
    assert_eq!(
        r.usage.counted(),
        16,
        "no result event: the message sum is kept"
    );
    assert_eq!(
        r.session.cwd.as_deref(),
        Some("/w"),
        "what it printed first is kept"
    );
}

#[test]
fn a_program_that_does_not_exist_is_an_error_naming_the_spawn() {
    let dir = tempfile::tempdir().unwrap();
    let err = AgentCommand::new("/no/such/agent-cli")
        .cwd(dir.path())
        .events_path(dir.path().join("events.jsonl"))
        .run()
        .unwrap_err();
    assert!(err.contains("spawn"), "{err}");
}

#[test]
fn an_events_path_that_cannot_be_created_names_it() {
    let dir = tempfile::tempdir().unwrap();
    let err = AgentCommand::new("true")
        .events_path(dir.path().join("missing/dir/events.jsonl"))
        .run()
        .unwrap_err();
    assert!(err.contains("missing/dir/events.jsonl"), "{err}");
}
