//! The stream-json parser (moved from `scholion-bench`; its other tests still run there through
//! the re-export).

use scholion_agentrun::events::{Usage, parse_stream};

const STREAM: &str = r#"{"type":"system","subtype":"init","cwd":"/w"}
{"type":"assistant","message":{"id":"m1","content":[{"type":"text","text":"hi"}],"usage":{"input_tokens":10,"cache_creation_input_tokens":2,"cache_read_input_tokens":100,"output_tokens":5}}}
{"type":"assistant","message":{"id":"m1","content":[{"type":"tool_use","name":"Read","input":{"file_path":"a.py"}}],"usage":{"input_tokens":10,"cache_creation_input_tokens":2,"cache_read_input_tokens":100,"output_tokens":7}}}
this line is not json
{"type":"result","total_cost_usd":0.5,"num_turns":2,"is_error":false,"duration_ms":1234,"usage":{"input_tokens":10,"output_tokens":7}}
"#;

#[test]
fn one_message_is_merged_over_its_events() {
    let s = parse_stream(STREAM);
    assert_eq!(s.cwd.as_deref(), Some("/w"));
    assert_eq!(s.messages.len(), 1);
    let m = &s.messages[0];
    assert_eq!(m.id, "m1");
    assert_eq!(m.usage.output, 7, "the larger of the repeated usages");
    assert_eq!(m.usage.counted(), 10 + 2 + 7, "cache reads are not counted");
    assert_eq!(m.usage.all_input(), 112);
    assert_eq!(m.tool_uses.len(), 1);
    assert_eq!(m.tool_uses[0].name, "Read");
    assert_eq!(s.bad_lines, 1);
    let r = s.result.as_ref().unwrap();
    assert_eq!((r.cost_usd, r.num_turns, r.is_error), (0.5, Some(2), false));
    assert_eq!(r.duration_ms, Some(1234));
    assert_eq!(
        r.usage,
        Usage {
            input: 10,
            output: 7,
            ..Usage::default()
        }
    );
    assert!(!s.truncated());
}

#[test]
fn a_killed_session_has_no_result_and_a_cut_last_line_is_counted_not_fatal() {
    let cut = STREAM.lines().take(3).collect::<Vec<_>>().join("\n") + "\n{\"type\":\"assis";
    let s = parse_stream(&cut);
    assert!(s.truncated());
    assert_eq!(s.bad_lines, 1);
    assert_eq!(s.messages.len(), 1);
}
