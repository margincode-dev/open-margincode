//! Parsing `claude -p --output-format stream-json --verbose` transcripts.
//!
//! Shapes, from a real capture (Claude Code 2.1.308; see `bench/README.md`):
//! - `{"type":"system","subtype":"init","cwd":…}` once at the start;
//! - `{"type":"assistant","message":{"id":…,"content":[…],"usage":{…}}}`, one event per content
//!   block, each repeating the message's id and usage;
//! - `{"type":"user",…}` tool results; `stream_event`, `rate_limit_event` and other `system`
//!   events, which carry nothing measured here;
//! - `{"type":"result","total_cost_usd":…,"num_turns":…,"usage":{…}}` last, unless the process
//!   was killed.

use serde_json::Value;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Usage {
    pub input: u64,
    pub cache_creation: u64,
    pub cache_read: u64,
    pub output: u64,
}

impl Usage {
    fn from_json(u: &Value) -> Usage {
        let n = |k: &str| u[k].as_u64().unwrap_or(0);
        Usage {
            input: n("input_tokens"),
            cache_creation: n("cache_creation_input_tokens"),
            cache_read: n("cache_read_input_tokens"),
            output: n("output_tokens"),
        }
    }

    fn max(self, o: Usage) -> Usage {
        Usage {
            input: self.input.max(o.input),
            cache_creation: self.cache_creation.max(o.cache_creation),
            cache_read: self.cache_read.max(o.cache_read),
            output: self.output.max(o.output),
        }
    }

    /// The tokens the metrics count: everything but cache reads.
    pub fn counted(self) -> u64 {
        self.input + self.cache_creation + self.output
    }

    /// Every input token the model saw, cache reads included (calibration).
    pub fn all_input(self) -> u64 {
        self.input + self.cache_creation + self.cache_read
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToolUse {
    pub name: String,
    pub input: Value,
}

/// One assistant message, merged over all the events that carry its id.
#[derive(Debug, Clone)]
pub struct Message {
    pub id: String,
    pub usage: Usage,
    pub tool_uses: Vec<ToolUse>,
}

#[derive(Debug, Clone)]
pub struct ResultEvent {
    pub cost_usd: f64,
    pub num_turns: Option<u32>,
    pub is_error: bool,
    pub usage: Usage,
    pub duration_ms: Option<u64>,
}

#[derive(Debug, Clone, Default)]
pub struct ParsedSession {
    /// The session's working directory (`init.cwd`), for relativizing tool paths.
    pub cwd: Option<String>,
    /// In order of first appearance.
    pub messages: Vec<Message>,
    pub result: Option<ResultEvent>,
    /// Lines that are not JSON objects (a partial last line of a killed session, for example).
    pub bad_lines: u32,
}

impl ParsedSession {
    /// No `result` event: the session was killed or its output cut off.
    pub fn truncated(&self) -> bool {
        self.result.is_none()
    }
}

/// Tolerant: bad or partial lines are counted, never fatal.
pub fn parse_stream(jsonl: &str) -> ParsedSession {
    let mut s = ParsedSession::default();
    for line in jsonl.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let Ok(e) = serde_json::from_str::<Value>(line) else {
            s.bad_lines += 1;
            continue;
        };
        if !e.is_object() {
            s.bad_lines += 1;
            continue;
        }
        match e["type"].as_str() {
            Some("system") if e["subtype"] == "init" => {
                if s.cwd.is_none() {
                    s.cwd = e["cwd"].as_str().map(str::to_owned);
                }
            }
            Some("assistant") => {
                let m = &e["message"];
                let Some(id) = m["id"].as_str() else {
                    s.bad_lines += 1;
                    continue;
                };
                let usage = Usage::from_json(&m["usage"]);
                let tools: Vec<ToolUse> = m["content"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter(|b| b["type"] == "tool_use")
                    .map(|b| ToolUse {
                        name: b["name"].as_str().unwrap_or_default().to_owned(),
                        input: b["input"].clone(),
                    })
                    .collect();
                match s.messages.iter_mut().find(|x| x.id == id) {
                    Some(x) => {
                        x.usage = x.usage.max(usage);
                        x.tool_uses.extend(tools);
                    }
                    None => s.messages.push(Message {
                        id: id.to_owned(),
                        usage,
                        tool_uses: tools,
                    }),
                }
            }
            Some("result") => {
                s.result = Some(ResultEvent {
                    cost_usd: e["total_cost_usd"].as_f64().unwrap_or(0.0),
                    num_turns: e["num_turns"].as_u64().map(|n| n as u32),
                    is_error: e["is_error"].as_bool().unwrap_or(false),
                    usage: Usage::from_json(&e["usage"]),
                    duration_ms: e["duration_ms"].as_u64(),
                })
            }
            _ => {}
        }
    }
    s
}
