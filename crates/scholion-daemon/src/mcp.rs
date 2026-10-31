//! MCP server (Plan 3 Task 8): JSON-RPC 2.0 over streamable HTTP at `/mcp`, answering every
//! request with plain JSON (no SSE stream). Only what agents need is implemented: `initialize`,
//! `ping`, `tools/list`, `tools/call` and notifications. Each tool is an in-process request to the
//! daemon's own `/v1` routes, so it behaves exactly like the HTTP API and the CLI.
//!
//! The MCP session is the ledger session (each connection dedups its own packs) and remembers the
//! task the agent started or claimed, with the claim generation its writes carry. The agent name is
//! the `X-Av-Agent` header of `initialize`, else the client's `clientInfo.name`.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use axum::body::{Body, Bytes};
use axum::extract::State;
use axum::http::{HeaderMap, Request, StatusCode};
use axum::response::{IntoResponse, Response};
use serde_json::{Value, json};
use tower::ServiceExt;
use uuid::Uuid;

use crate::AppState;

const PROTOCOL_VERSIONS: [&str; 3] = ["2025-06-18", "2025-03-26", "2024-11-05"];
const SESSION_HEADER: &str = "mcp-session-id";
/// Largest request axum accepts on `/mcp`; the real limits below answer with JSON-RPC errors.
pub(crate) const MAX_REQUEST_BYTES: usize = 8 << 20;
const MAX_ARGUMENTS_BYTES: usize = 1 << 20;
const MAX_TEXT_BYTES: usize = 64 << 10;
const MAX_SUMMARY_CHARS: usize = 200;
/// Sessions unused this long are forgotten (on the next `initialize`).
const SESSION_IDLE: Duration = Duration::from_secs(24 * 3600);

const INVALID_REQUEST: i64 = -32600;
const METHOD_NOT_FOUND: i64 = -32601;
const INVALID_PARAMS: i64 = -32602;
const PARSE_ERROR: i64 = -32700;

#[derive(Clone)]
pub(crate) struct McpSession {
    agent: String,
    task: Option<String>,
    generation: Option<i64>,
    last_used: Instant,
}

pub(crate) type Sessions = Arc<Mutex<HashMap<String, McpSession>>>;

const INSTRUCTIONS: &str = "Scholion (av): context-first version control. `start_task` (or \
`claim_task`), work in the workspace it names, `record` decisions and dead ends when they happen \
(kind=attempt with why, right after an approach fails), then `submit` with any last records inline; \
submit checkpoints for you. `get_context` gives more context on demand. Call `checkpoint` only to \
refresh `get_context` mid-task. Workspace edits are snapshotted into commits, so `git status` shows \
nothing. `start_task` with no title or goal gives your next job; to review: `review_task`, run its \
tests in its workspace, `validate`, `post_review`, then `merge` if approved.";

fn tools() -> Value {
    let task_id = json!({"type": "string", "description": "Task id (default: the task this session started or claimed)."});
    let repo = json!({"type": "string", "description": "Default: the task's repo."});
    let at =
        json!({"type": "string", "description": "main (default), task:<slug> or a commit id."});
    let anchors = json!({"type": "array", "description": "Code the record is about.", "items": {
        "type": "object", "required": ["path"],
        "properties": {"path": {"type": "string"}, "symbol": {"type": "string"}}}});
    let record_item = json!({"type": "object", "required": ["kind", "summary"], "properties": {
        "kind": {"type": "string", "enum": ["intent", "decision", "attempt", "note"]},
        "summary": {"type": "string"}, "body": {"type": "string"}, "anchors": anchors,
        "receipt": {"type": "string", "description": "The rcp:… id of the pack you worked from."}}});
    json!([
        {"name": "start_task", "description": "Start a task: creates its isolated workspace and returns the workspace path and a first context pack. Omit title and goal for your next job: a review or merge of others' work, a handoff, else a backlog task that fits you. wait: seconds to wait for one.",
         "inputSchema": {"type": "object", "required": [], "properties": {
            "repo": {"type": "string"}, "title": {"type": "string"}, "goal": {"type": "string"},
            "budget": {"type": "integer", "description": "Token budget of the pack (default 800)."},
            "wait": {"type": "integer"}}}},
        {"name": "checkpoint", "description": "Snapshot the workspace and refresh the code index. Only needed to refresh `get_context` mid-task; `submit` checkpoints for you.",
         "inputSchema": {"type": "object", "properties": {"task_id": task_id}}},
        {"name": "record", "description": "Record a decision, attempt (dead end), note or intent, anchored to code. Call `record` with kind=attempt immediately when an approach fails, including why. Summary at most 200 characters.",
         "inputSchema": {"type": "object", "required": ["kind", "summary"], "properties": {
            "kind": {"type": "string", "enum": ["intent", "decision", "attempt", "note"]},
            "summary": {"type": "string"}, "body": {"type": "string"}, "anchors": anchors,
            "receipt": {"type": "string", "description": "The rcp:… id of the pack you worked from."},
            "task_id": task_id}}},
        {"name": "get_context", "description": "A token-budgeted context pack for the task (small by default; pass a larger budget for more): code, decisions, dead ends, conventions, other agents' live work. Items sent earlier in this session are referenced, not resent.",
         "inputSchema": {"type": "object", "properties": {
            "budget": {"type": "integer", "description": "Token budget (default 800)."},
            "code": {"type": "string", "enum": ["signatures", "full"], "description": "`signatures` (default): path, lines and signature of each code item; `full`: with its source."},
            "focus": {"type": "string"}, "task_id": task_id}}},
        {"name": "expand", "description": "Full text of an item id shown in a pack (rec:… or sym:…).",
         "inputSchema": {"type": "object", "required": ["item_id"], "properties": {
            "item_id": {"type": "string"}, "task_id": task_id}}},
        {"name": "read_symbol", "description": "Full source of a symbol by its numeric id (the N of sym:N).",
         "inputSchema": {"type": "object", "required": ["id"], "properties": {
            "id": {"type": "integer"}, "task_id": task_id}}},
        {"name": "search_history", "description": "Full-text search over the repo's records: earlier decisions, dead ends and notes.",
         "inputSchema": {"type": "object", "required": ["query"], "properties": {
            "query": {"type": "string"}, "limit": {"type": "integer", "description": "At most 20."},
            "repo": {"type": "string", "description": "Default: the task's repo."}}}},
        {"name": "handoff", "description": "Hand the task to another agent: what is done, what remains, what is broken, open questions. Needs a recorded decision, attempt or note.",
         "inputSchema": {"type": "object", "required": ["done", "remaining"], "properties": {
            "done": {"type": "string"}, "remaining": {"type": "string"},
            "broken": {"type": "string"}, "questions": {"type": "string"}, "task_id": task_id,
            "needs": {"type": "array", "items": {"type": "string"}, "description": "Tags the next agent must have, e.g. [\"image\"]."}}}},
        {"name": "claim_task", "description": "Take over a handed-off task (or an open one whose lease expired); returns the workspace and the handoff bundle.",
         "inputSchema": {"type": "object", "required": ["task_id"], "properties": {
            "task_id": {"type": "string"}, "budget": {"type": "integer"}}}},
        {"name": "submit", "description": "Send the task to review; checkpoints for you. Needs changes and a recorded decision, attempt or note. Pass the last ones as `records` (same fields as `record`) to record and submit in one call; if any is invalid nothing is written and the task stays open.",
         "inputSchema": {"type": "object", "properties": {
            "records": {"type": "array", "description": "Records to write before submitting.", "items": record_item},
            "task_id": task_id}}},
        {"name": "review_task", "description": "The review bundle of a task: tests at its head, risks, a conventions checklist, changes and records.",
         "inputSchema": {"type": "object", "required": ["task_id"], "properties": {
            "task_id": {"type": "string"}, "budget": {"type": "integer"}}}},
        {"name": "post_review", "description": "Approve a task in review (needs a passing validation at its head) or request changes with findings. The reviewer must not be its owner.",
         "inputSchema": {"type": "object", "required": ["task_id", "verdict"], "properties": {
            "task_id": {"type": "string"},
            "verdict": {"type": "string", "enum": ["approve", "request_changes"]},
            "findings": {"type": "array", "items": {"type": "object", "required": ["summary"], "properties": {
                "summary": {"type": "string"}, "body": {"type": "string"}, "anchors": anchors}}}}}},
        {"name": "merge", "description": "Merge an approved task into main. A conflict leaves main untouched and names the paths.",
         "inputSchema": {"type": "object", "properties": {"task_id": task_id}}},
        {"name": "undo", "description": "Undo this task's merge while it is still the newest change on main (only the agent that merged).",
         "inputSchema": {"type": "object", "properties": {"task_id": task_id}}},
        {"name": "status", "description": "The task's status, owner, generation, changed files, verdict, merge and backup state.",
         "inputSchema": {"type": "object", "properties": {"task_id": task_id}}},
        {"name": "validate", "description": "Report that a check command ran in the workspace, with its exit code, bound to the task's current head (approval needs a passing one). This is an attestation: run the command yourself first; the daemon does not run it.",
         "inputSchema": {"type": "object", "required": ["command", "exit_code"], "properties": {
            "command": {"type": "string"}, "exit_code": {"type": "integer"},
            "output_tail": {"type": "string", "description": "The last lines of the output."},
            "checks": {"type": "array", "description": "Results of the task's required convention checks (each convention's check command, run by you in the workspace); approval needs every one to exit 0.",
                "items": {"type": "object", "required": ["convention_id", "exit_code"], "properties": {
                    "convention_id": {"type": "integer"}, "exit_code": {"type": "integer"},
                    "output_tail": {"type": "string"}}}},
            "task_id": task_id}}},
        {"name": "sync", "description": "Rebase the task onto the current main (after a merge conflict, or to pick up merged work). Conflicts are left as markers in the workspace files.",
         "inputSchema": {"type": "object", "properties": {"task_id": task_id}}},
        {"name": "abandon", "description": "End the task for good (owner only); its records stay searchable.",
         "inputSchema": {"type": "object", "properties": {"task_id": task_id}}},
        {"name": "list_conventions", "description": "Every convention of the repo: path glob, whether it is required, its check command, and the rule.",
         "inputSchema": {"type": "object", "properties": {"repo": repo.clone()}}},
        {"name": "why", "description": "Why a line of main is the way it is: the merge that last changed it, that task's goal, and its decisions, dead ends and notes.",
         "inputSchema": {"type": "object", "required": ["path", "line"], "properties": {
            "path": {"type": "string"}, "line": {"type": "integer", "description": "1-based."},
            "repo": repo.clone()}}},
        {"name": "read_file", "description": "A file's content at a commit, without a workspace. `at`: main (default), task:<slug> (that task's last checkpoint) or a commit id.",
         "inputSchema": {"type": "object", "required": ["path"], "properties": {
            "path": {"type": "string"}, "at": at.clone(), "repo": repo.clone()}}},
        {"name": "list_files", "description": "Every file at a commit, without a workspace.",
         "inputSchema": {"type": "object", "properties": {"at": at.clone(), "repo": repo.clone()}}},
        {"name": "diff", "description": "Unified diff between two commits (each main, task:<slug> or a commit id; `to` defaults to main).",
         "inputSchema": {"type": "object", "required": ["from"], "properties": {
            "from": {"type": "string"}, "to": {"type": "string"}, "repo": repo.clone()}}},
        {"name": "log", "description": "Main's history, newest first; with `path`, only the commits that changed it.",
         "inputSchema": {"type": "object", "properties": {
            "path": {"type": "string"}, "limit": {"type": "integer"}, "repo": repo}}}
    ])
}

/// A JSON-RPC error.
struct RpcError {
    code: i64,
    message: String,
}

fn invalid(message: impl Into<String>) -> RpcError {
    RpcError {
        code: INVALID_PARAMS,
        message: message.into(),
    }
}

fn reply(id: &Value, outcome: Result<Value, RpcError>) -> Value {
    match outcome {
        Ok(result) => json!({"jsonrpc": "2.0", "id": id, "result": result}),
        Err(e) => {
            json!({"jsonrpc": "2.0", "id": id, "error": {"code": e.code, "message": e.message}})
        }
    }
}

fn http(status: StatusCode, body: Value) -> Response {
    (status, axum::Json(body)).into_response()
}

fn json_type_ok(kind: &str, v: &Value) -> bool {
    match kind {
        "string" => v.is_string(),
        "integer" => v.is_i64() || v.is_u64(),
        "array" => v.is_array(),
        "object" => v.is_object(),
        _ => true,
    }
}

/// Required properties present, declared types right (one level, plus array items' required keys).
fn check_args(schema: &Value, args: &Value) -> Result<(), RpcError> {
    let Some(obj) = args.as_object() else {
        return Err(invalid("arguments must be an object"));
    };
    for key in schema["required"].as_array().into_iter().flatten() {
        let key = key.as_str().unwrap_or_default();
        if obj.get(key).is_none_or(Value::is_null) {
            return Err(invalid(format!("missing required argument `{key}`")));
        }
    }
    for (key, prop) in schema["properties"].as_object().into_iter().flatten() {
        let Some(v) = obj.get(key).filter(|v| !v.is_null()) else {
            continue;
        };
        let kind = prop["type"].as_str().unwrap_or_default();
        if !json_type_ok(kind, v) {
            return Err(invalid(format!("argument `{key}` must be of type {kind}")));
        }
        if let Some(allowed) = prop["enum"].as_array()
            && !allowed.contains(v)
        {
            return Err(invalid(format!(
                "argument `{key}` must be one of {allowed:?}"
            )));
        }
        if kind == "array"
            && let Some(item_kind) = prop["items"]["type"].as_str()
            && item_kind != "object"
            && let Some(bad) = v
                .as_array()
                .into_iter()
                .flatten()
                .find(|i| !json_type_ok(item_kind, i))
        {
            return Err(invalid(format!(
                "`{key}` must be a list of {item_kind}s, got {bad}"
            )));
        }
        if kind == "array" && prop["items"]["type"] == "object" {
            for item in v.as_array().into_iter().flatten() {
                check_args(&prop["items"], item)
                    .map_err(|e| invalid(format!("in `{key}`: {}", e.message)))?;
            }
        }
    }
    Ok(())
}

/// The size limits: summaries at most 200 characters, bodies at most 64 KiB, anywhere in the
/// arguments (findings included).
fn check_sizes(v: &Value) -> Result<(), RpcError> {
    match v {
        Value::Object(map) => {
            for (key, val) in map {
                if let Some(s) = val.as_str() {
                    if key == "summary" && s.chars().count() > MAX_SUMMARY_CHARS {
                        return Err(invalid(format!(
                            "`summary` has {} characters; at most {MAX_SUMMARY_CHARS}",
                            s.chars().count()
                        )));
                    }
                    if s.len() > MAX_TEXT_BYTES {
                        return Err(invalid(format!(
                            "`{key}` is {} bytes; at most {MAX_TEXT_BYTES}",
                            s.len()
                        )));
                    }
                }
                check_sizes(val)?;
            }
            Ok(())
        }
        Value::Array(items) => items.iter().try_for_each(check_sizes),
        _ => Ok(()),
    }
}

/// An in-process request to the daemon's `/v1` routes, as (status, JSON body).
async fn inner(
    st: &AppState,
    method: &str,
    uri: &str,
    body: Option<Value>,
    generation: Option<i64>,
) -> (StatusCode, Value) {
    let mut req = Request::builder().method(method).uri(uri);
    if let Some(g) = generation {
        req = req.header(crate::flow::GENERATION_HEADER, g.to_string());
    }
    let body = match body {
        Some(v) => {
            req = req.header("content-type", "application/json");
            Body::from(v.to_string())
        }
        None => Body::empty(),
    };
    let router = crate::api::routes().with_state(st.clone());
    let resp = match router.oneshot(req.body(body).expect("valid request")).await {
        Ok(r) => r,
        Err(never) => match never {},
    };
    let status = resp.status();
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap_or_default();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

fn enc(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

fn str_arg<'a>(args: &'a Value, key: &str) -> Option<&'a str> {
    args[key].as_str().filter(|s| !s.is_empty())
}

/// The id of the task `given` names: an id as is, else `repo/slug`, else a slug in any repo (an
/// error that lists the matches when it is in several).
async fn task_ref(st: &AppState, given: &str) -> Result<String, RpcError> {
    let given = given.trim();
    if Uuid::parse_str(given).is_ok() {
        return Ok(given.to_owned());
    }
    let found = match given.split_once('/') {
        Some((repo, slug)) => match st.store.task_by_slug(repo, slug).await {
            Ok(t) => vec![t],
            Err(scholion_core::AvError::NotFound(_)) => vec![],
            Err(e) => return Err(invalid(e.to_string())),
        },
        None => st
            .store
            .tasks_with_slug(given)
            .await
            .map_err(|e| invalid(e.to_string()))?,
    };
    match found.as_slice() {
        [t] => Ok(t.id.to_string()),
        [] => Err(invalid(format!(
            "no task {given:?}: pass a task id, a slug or repo/slug"
        ))),
        many => Err(invalid(format!(
            "{given:?} names several tasks; pass one of: {}",
            many.iter()
                .map(|t| format!("{}/{}", t.repo, t.slug))
                .collect::<Vec<_>>()
                .join(", ")
        ))),
    }
}

fn session(st: &AppState, sid: &str) -> Option<McpSession> {
    st.mcp
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get(sid)
        .cloned()
}

fn remember(st: &AppState, sid: &str, task: &str, generation: Option<i64>) {
    if let Some(s) = st
        .mcp
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get_mut(sid)
    {
        s.task = Some(task.to_owned());
        s.generation = generation;
    }
}

/// The task a tool acts on and the generation to write under (only for the session's own task).
/// The `repo` argument, else the repo of the session's task.
async fn repo_arg(st: &AppState, s: &McpSession, args: &Value) -> Result<String, RpcError> {
    if let Some(r) = str_arg(args, "repo") {
        return Ok(r.to_owned());
    }
    let (task, _) = target(s, args)?;
    let (_, t) = inner(st, "GET", &format!("/v1/tasks/{}", enc(&task)), None, None).await;
    Ok(t["task"]["repo"].as_str().unwrap_or_default().to_owned())
}

fn target(s: &McpSession, args: &Value) -> Result<(String, Option<i64>), RpcError> {
    match str_arg(args, "task_id") {
        Some(t) if s.task.as_deref() == Some(t) => Ok((t.to_owned(), s.generation)),
        Some(t) => Ok((t.to_owned(), None)),
        None => match &s.task {
            Some(t) => Ok((t.clone(), s.generation)),
            None => Err(invalid(
                "no task: pass `task_id`, or call start_task or claim_task first",
            )),
        },
    }
}

fn pretty(v: &Value) -> String {
    serde_json::to_string_pretty(v).unwrap_or_default()
}

/// How a tool reaches the API: method, URI, JSON body, generation header, and how the response
/// becomes the tool's text.
type Call = (
    &'static str,
    String,
    Option<Value>,
    Option<i64>,
    fn(&Value) -> String,
);

/// Runs one tool. `Ok` is the tool's text (`Err` inside: the daemon refused, a tool error);
/// the outer `Err` is a protocol error (unknown tool, bad arguments).
async fn run_tool(
    st: &AppState,
    sid: &str,
    s: &McpSession,
    name: &str,
    args: &Value,
) -> Result<Result<String, String>, RpcError> {
    let agent = s.agent.as_str();
    let (method, uri, body, generation, render): Call = match name {
        // With no title and goal: the next backlog task that fits this agent (Plan 8).
        "start_task" if args["title"].is_null() && args["goal"].is_null() => (
            "POST",
            "/v1/backlog/next".into(),
            Some(json!({"repo": args["repo"], "agent": agent, "session": sid,
                        "budget": args["budget"], "wait_secs": args["wait"]})),
            None,
            |r| {
                if r["task"].is_null() {
                    return format!(
                        "No task is waiting for you ({} queued for other agents or still \
                         reserved). Call start_task again with wait (seconds) to wait for one, \
                         start your own with repo, title and goal, or ask the user.",
                        r["queued"]
                    );
                }
                if r["from"] == "review" {
                    return format!(
                        "review: task {} ({}/{}), owner: {}\nworkspace: {}\n{}",
                        r["task"]["id"].as_str().unwrap_or_default(),
                        r["task"]["repo"].as_str().unwrap_or_default(),
                        r["task"]["slug"].as_str().unwrap_or_default(),
                        r["task"]["owner_agent"].as_str().unwrap_or_default(),
                        r["task"]["workspace_path"].as_str().unwrap_or_default(),
                        r["next"].as_str().unwrap_or_default()
                    );
                }
                let md = if r["from"] == "handoff" || r["from"] == "abandoned" {
                    &r["bundle"]["markdown"]
                } else {
                    &r["pack"]["markdown"]
                };
                format!(
                    "workspace: {}\ntask: {} ({})\ngeneration: {}\n\n{}",
                    r["task"]["workspace_path"].as_str().unwrap_or_default(),
                    r["task"]["id"].as_str().unwrap_or_default(),
                    r["task"]["slug"].as_str().unwrap_or_default(),
                    r["task"]["generation"],
                    md.as_str().unwrap_or_default()
                )
            },
        ),
        "start_task" => {
            for key in ["repo", "title", "goal"] {
                if str_arg(args, key).is_none() {
                    return Err(invalid(format!(
                        "start_task needs `{key}` (repo, title and goal), or none of title and \
                         goal to take the next backlog task"
                    )));
                }
            }
            (
                "POST",
                "/v1/tasks".into(),
                Some(
                    json!({"repo": args["repo"], "title": args["title"], "goal": args["goal"],
                                "agent": agent, "session": sid, "budget": args["budget"]}),
                ),
                None,
                |r| {
                    format!(
                        "workspace: {}\ntask: {} ({})\ngeneration: {}\n\n{}",
                        r["task"]["workspace_path"].as_str().unwrap_or_default(),
                        r["task"]["id"].as_str().unwrap_or_default(),
                        r["task"]["slug"].as_str().unwrap_or_default(),
                        r["task"]["generation"],
                        r["pack"]["markdown"].as_str().unwrap_or_default()
                    )
                },
            )
        }
        "claim_task" => (
            "POST",
            format!(
                "/v1/tasks/{}/claim",
                enc(str_arg(args, "task_id").unwrap_or_default())
            ),
            Some(json!({"agent": agent, "budget": args["budget"]})),
            None,
            |r| {
                format!(
                    "workspace: {}\ntask: {} ({})\ngeneration: {}\n\n{}",
                    r["task"]["workspace_path"].as_str().unwrap_or_default(),
                    r["task"]["id"].as_str().unwrap_or_default(),
                    r["task"]["slug"].as_str().unwrap_or_default(),
                    r["generation"],
                    r["bundle"]["markdown"].as_str().unwrap_or_default()
                )
            },
        ),
        "search_history" => {
            let repo = repo_arg(st, s, args).await?;
            let limit = args["limit"].as_u64().unwrap_or(10);
            (
                "GET",
                format!(
                    "/v1/repos/{}/search?q={}&limit={limit}",
                    enc(&repo),
                    enc(str_arg(args, "query").unwrap_or_default())
                ),
                None,
                None,
                |r| {
                    let hits = r["records"].as_array().cloned().unwrap_or_default();
                    if hits.is_empty() {
                        return "no matching records".to_owned();
                    }
                    hits.iter()
                        .map(|h| {
                            format!(
                                "- rec:{} {}: {}",
                                &h["id"].as_str().unwrap_or_default().replace('-', "")[..8],
                                h["kind"].as_str().unwrap_or_default(),
                                h["summary"].as_str().unwrap_or_default()
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("\n")
                },
            )
        }
        "list_conventions" => {
            let repo = repo_arg(st, s, args).await?;
            (
                "GET",
                format!("/v1/repos/{}/conventions", enc(&repo)),
                None,
                None,
                |r| {
                    let list = r["conventions"].as_array().cloned().unwrap_or_default();
                    if list.is_empty() {
                        return "no conventions".to_owned();
                    }
                    list.iter()
                        .map(|c| {
                            let required = if c["required"] == true {
                                " (required)"
                            } else {
                                ""
                            };
                            let check = c["check_command"]
                                .as_str()
                                .map(|cmd| format!(" [check: {cmd}]"))
                                .unwrap_or_default();
                            format!(
                                "- conv:{} ({}){required}{check} {}",
                                c["id"],
                                c["scope_glob"].as_str().unwrap_or_default(),
                                c["text"].as_str().unwrap_or_default()
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("\n")
                },
            )
        }
        "why" => {
            let repo = repo_arg(st, s, args).await?;
            let path = str_arg(args, "path").ok_or_else(|| invalid("path is required"))?;
            let line = args["line"]
                .as_u64()
                .ok_or_else(|| invalid("line (1-based) is required"))?;
            (
                "GET",
                format!(
                    "/v1/repos/{}/why?path={}&line={line}",
                    enc(&repo),
                    enc(path)
                ),
                None,
                None,
                |r| {
                    let commit = r["commit"].as_str().unwrap_or_default();
                    let mut out = format!(
                        "{}\nlast changed in {} {}",
                        r["line_text"].as_str().unwrap_or_default(),
                        &commit[..commit.len().min(12)],
                        r["commit_description"].as_str().unwrap_or_default()
                    );
                    if r["task"].is_object() {
                        out.push_str(&format!(
                            "\ntask {}: {}",
                            r["task"]["slug"].as_str().unwrap_or_default(),
                            r["task"]["goal"].as_str().unwrap_or_default()
                        ));
                        for rec in r["records"].as_array().into_iter().flatten() {
                            out.push_str(&format!(
                                "\n- {} {}: {}",
                                rec["id"].as_str().unwrap_or_default(),
                                rec["kind"].as_str().unwrap_or_default(),
                                rec["summary"].as_str().unwrap_or_default()
                            ));
                        }
                    }
                    out
                },
            )
        }
        "read_file" => {
            let repo = repo_arg(st, s, args).await?;
            let path = str_arg(args, "path").ok_or_else(|| invalid("path is required"))?;
            (
                "GET",
                format!(
                    "/v1/repos/{}/files/{}?at={}",
                    enc(&repo),
                    path.split('/').map(enc).collect::<Vec<_>>().join("/"),
                    enc(str_arg(args, "at").unwrap_or("main"))
                ),
                None,
                None,
                |r| match r["content"].as_str() {
                    Some(text) => text.to_owned(),
                    None => format!("binary file, {} bytes", r["bytes"]),
                },
            )
        }
        "list_files" => {
            let repo = repo_arg(st, s, args).await?;
            (
                "GET",
                format!(
                    "/v1/repos/{}/tree?at={}",
                    enc(&repo),
                    enc(str_arg(args, "at").unwrap_or("main"))
                ),
                None,
                None,
                |r| {
                    let files = r["files"].as_array().cloned().unwrap_or_default();
                    files
                        .iter()
                        .filter_map(Value::as_str)
                        .collect::<Vec<_>>()
                        .join("\n")
                },
            )
        }
        "diff" => {
            let repo = repo_arg(st, s, args).await?;
            let from = str_arg(args, "from").ok_or_else(|| invalid("from is required"))?;
            (
                "GET",
                format!(
                    "/v1/repos/{}/diff?from={}&to={}",
                    enc(&repo),
                    enc(from),
                    enc(str_arg(args, "to").unwrap_or("main"))
                ),
                None,
                None,
                |r| {
                    let files = r["files"].as_array().cloned().unwrap_or_default();
                    if files.is_empty() {
                        return "no changes".to_owned();
                    }
                    files
                        .iter()
                        .map(|f| match f["diff"].as_str() {
                            Some(d) if !d.is_empty() => d.to_owned(),
                            _ => format!(
                                "{} {} (binary)\n",
                                f["status"].as_str().unwrap_or_default(),
                                f["path"].as_str().unwrap_or_default()
                            ),
                        })
                        .collect::<String>()
                },
            )
        }
        "log" => {
            let repo = repo_arg(st, s, args).await?;
            let limit = args["limit"].as_u64().unwrap_or(20);
            let path = str_arg(args, "path")
                .map(|p| format!("&path={}", enc(p)))
                .unwrap_or_default();
            (
                "GET",
                format!("/v1/repos/{}/log?limit={limit}{path}", enc(&repo)),
                None,
                None,
                |r| {
                    let commits = r["commits"].as_array().cloned().unwrap_or_default();
                    commits
                        .iter()
                        .map(|c| {
                            let change = c["change"].as_str().unwrap_or_default();
                            let title = c["description"]
                                .as_str()
                                .unwrap_or_default()
                                .lines()
                                .next()
                                .unwrap_or_default();
                            format!("{} {title}", &change[..change.len().min(12)])
                        })
                        .collect::<Vec<_>>()
                        .join("\n")
                },
            )
        }
        "review_task" => (
            "GET",
            format!(
                "/v1/tasks/{}/bundle?kind=pr&budget={}",
                enc(str_arg(args, "task_id").unwrap_or_default()),
                args["budget"].as_u64().unwrap_or(4000)
            ),
            None,
            None,
            |r| r["markdown"].as_str().unwrap_or_default().to_owned(),
        ),
        "post_review" => (
            "POST",
            format!(
                "/v1/tasks/{}/review",
                enc(str_arg(args, "task_id").unwrap_or_default())
            ),
            Some(json!({"reviewer": agent, "verdict": args["verdict"],
                            "findings": args.get("findings").cloned().unwrap_or(json!([]))})),
            None,
            pretty,
        ),
        _ => {
            let (task, generation) = target(s, args)?;
            let t = enc(&task);
            let sess = enc(sid);
            match name {
                "checkpoint" => (
                    "POST",
                    format!("/v1/tasks/{t}/checkpoint"),
                    None,
                    generation,
                    pretty,
                ),
                "record" => (
                    "POST",
                    format!("/v1/tasks/{t}/records"),
                    Some(json!({"kind": args["kind"], "summary": args["summary"],
                                    "body": args["body"], "anchors": args["anchors"],
                                    "receipt": args["receipt"], "agent": agent})),
                    generation,
                    |r| {
                        format!(
                            "recorded rec:{} ({})",
                            &r["id"].as_str().unwrap_or_default().replace('-', "")[..8],
                            r["kind"].as_str().unwrap_or_default()
                        )
                    },
                ),
                "get_context" => {
                    let mut uri = format!(
                        "/v1/tasks/{t}/context?session={sess}&budget={}",
                        args["budget"].as_u64().unwrap_or(800)
                    );
                    if let Some(f) = str_arg(args, "focus") {
                        uri.push_str(&format!("&focus={}", enc(f)));
                    }
                    if let Some(c) = str_arg(args, "code") {
                        uri.push_str(&format!("&code={}", enc(c)));
                    }
                    ("GET", uri, None, None, |r| {
                        let mut text = r["markdown"].as_str().unwrap_or_default().to_owned();
                        if let Some(rcp) = r["receipt"].as_str() {
                            text.push_str(&format!("\nreceipt: {rcp}"));
                        }
                        text
                    })
                }
                "expand" | "read_symbol" => {
                    let item = match name {
                        "expand" => str_arg(args, "item_id").unwrap_or_default().to_owned(),
                        _ => format!("sym:{}", args["id"]),
                    };
                    (
                        "GET",
                        format!("/v1/tasks/{t}/items/{}?session={sess}", enc(&item)),
                        None,
                        None,
                        |r| r["full"].as_str().unwrap_or_default().to_owned(),
                    )
                }
                "handoff" => (
                    "POST",
                    format!("/v1/tasks/{t}/handoff"),
                    Some(
                        json!({"agent": agent, "done": args["done"], "remaining": args["remaining"],
                                    "broken": args.get("broken").cloned().unwrap_or(json!("")),
                                    "questions": args.get("questions").cloned().unwrap_or(json!("")),
                                    "needs": args.get("needs").filter(|v| !v.is_null()).cloned()
                                        .unwrap_or(json!([]))}),
                    ),
                    generation,
                    |r| {
                        format!(
                            "{}\n\n{}",
                            r["status"].as_str().unwrap_or_default(),
                            r["bundle"]["markdown"].as_str().unwrap_or_default()
                        )
                    },
                ),
                "submit" => (
                    "POST",
                    format!("/v1/tasks/{t}/submit"),
                    Some(json!({"agent": agent,
                                "records": args.get("records").filter(|v| !v.is_null()).cloned()
                                    .unwrap_or(json!([]))})),
                    generation,
                    pretty,
                ),
                "merge" => (
                    "POST",
                    format!("/v1/tasks/{t}/merge"),
                    Some(json!({"agent": agent})),
                    None,
                    pretty,
                ),
                "undo" => (
                    "POST",
                    format!("/v1/tasks/{t}/undo"),
                    Some(json!({"agent": agent})),
                    None,
                    pretty,
                ),
                "validate" => (
                    "POST",
                    format!("/v1/tasks/{t}/validate"),
                    Some(json!({"agent": agent, "command": args["command"],
                                "exit_code": args["exit_code"],
                                "output_tail": args.get("output_tail").cloned().unwrap_or(json!("")),
                                "checks": args.get("checks").cloned().unwrap_or(json!([]))})),
                    generation,
                    |r| {
                        let head = r["head"].as_str().unwrap_or_default();
                        let head = &head[..head.len().min(12)];
                        if r["passed"] == true {
                            format!("validation passed (exit 0) at head {head}")
                        } else {
                            format!("validation failed (non-zero exit) at head {head}")
                        }
                    },
                ),
                "sync" => (
                    "POST",
                    format!("/v1/tasks/{t}/sync"),
                    Some(json!({"agent": agent})),
                    generation,
                    |r| {
                        let conflicts = r["conflicts"].as_array().cloned().unwrap_or_default();
                        let status = r["status"].as_str().unwrap_or_default();
                        if conflicts.is_empty() {
                            format!("synced onto main: no conflicts (task {status})")
                        } else {
                            let paths: Vec<&str> =
                                conflicts.iter().filter_map(Value::as_str).collect();
                            format!(
                                "synced onto main with conflicts in {}: fix the markers, then \
                                 checkpoint and submit (task {status})",
                                paths.join(", ")
                            )
                        }
                    },
                ),
                "abandon" => (
                    "POST",
                    format!("/v1/tasks/{t}/abandon"),
                    Some(json!({"agent": agent})),
                    generation,
                    |r| format!("task {}", r["status"].as_str().unwrap_or_default()),
                ),
                "status" => ("GET", format!("/v1/tasks/{t}"), None, None, pretty),
                other => return Err(invalid(format!("unknown tool: {other}"))),
            }
        }
    };
    let (status, out) = inner(st, method, &uri, body, generation).await;
    if status.is_success() {
        match name {
            "start_task" => {
                // `{"task": null}`: nothing was waiting, so there is nothing to remember; a
                // review is someone else's task, so it is not this session's either
                if let Some(id) = out["task"]["id"].as_str()
                    && out["from"] != "review"
                {
                    remember(st, sid, id, out["task"]["generation"].as_i64());
                }
            }
            "claim_task" => remember(
                st,
                sid,
                out["task"]["id"].as_str().unwrap_or_default(),
                out["generation"].as_i64(),
            ),
            _ => {}
        }
        return Ok(Ok(render(&out)));
    }
    let message = out["error"]
        .as_str()
        .map_or_else(|| out.to_string(), str::to_owned);
    if status == StatusCode::BAD_REQUEST {
        // malformed input the route itself refused: a protocol error, as for the schema checks
        return Err(invalid(message));
    }
    Ok(Err(message))
}

async fn call_tool(
    st: &AppState,
    sid: &str,
    s: &McpSession,
    params: &Value,
) -> Result<Value, RpcError> {
    let name = params["name"].as_str().unwrap_or_default();
    let args = params.get("arguments").cloned().unwrap_or(json!({}));
    let size = serde_json::to_vec(&args)
        .map(|v| v.len())
        .unwrap_or(usize::MAX);
    if size > MAX_ARGUMENTS_BYTES {
        return Err(invalid(format!(
            "arguments are {size} bytes; at most {MAX_ARGUMENTS_BYTES}"
        )));
    }
    let tools = tools();
    let Some(tool) = tools
        .as_array()
        .into_iter()
        .flatten()
        .find(|t| t["name"] == name)
    else {
        return Err(invalid(format!("unknown tool: {name}")));
    };
    check_args(&tool["inputSchema"], &args)?;
    check_sizes(&args)?;
    // a task may be named by its slug (or repo/slug): from here on it is its id
    let mut args = args;
    if let Some(given) = args["task_id"].as_str() {
        args["task_id"] = json!(task_ref(st, given).await?);
    }
    let (text, is_error) = match run_tool(st, sid, s, name, &args).await? {
        Ok(text) => (text, false),
        Err(text) => (text, true),
    };
    Ok(json!({"content": [{"type": "text", "text": text}], "isError": is_error}))
}

/// `X-Av-Task` pins a new session to an existing task, as if it had claimed it: tools default to
/// that task and write under its current generation. The benchmark's av arms use it, so the
/// agent's prompt need not name the task.
async fn pinned_task(
    st: &AppState,
    headers: &HeaderMap,
) -> Result<Option<(String, i64)>, RpcError> {
    let Some(raw) = headers.get("x-av-task") else {
        return Ok(None);
    };
    let id = raw
        .to_str()
        .ok()
        .and_then(|v| Uuid::parse_str(v.trim()).ok())
        .ok_or_else(|| invalid("X-Av-Task is not a task id"))?;
    match st.store.get_task(scholion_core::TaskId(id)).await {
        Ok(task) => Ok(Some((id.to_string(), task.generation))),
        Err(_) => Err(invalid(format!("X-Av-Task: no task {id}"))),
    }
}

async fn initialize(
    st: &AppState,
    headers: &HeaderMap,
    params: &Value,
) -> Result<(String, Value), RpcError> {
    let pinned = pinned_task(st, headers).await?;
    let agent = headers
        .get("x-av-agent")
        .and_then(|v| v.to_str().ok())
        .or(params["clientInfo"]["name"].as_str())
        .map(str::trim)
        .filter(|a| !a.is_empty())
        .unwrap_or("mcp")
        .to_owned();
    // A profile may list the names a client introduces itself with (`av agent set --alias`), so
    // routing, task ownership and fencing all use one name for the agent.
    let agent = match st.store.list_profiles().await {
        Ok(profiles) => scholion_core::routing::resolve(&profiles, &agent)
            .map(|p| p.name.clone())
            .unwrap_or(agent),
        Err(_) => agent,
    };
    let sid = Uuid::new_v4().to_string();
    {
        let mut map = st.mcp.lock().unwrap_or_else(|e| e.into_inner());
        map.retain(|_, s| s.last_used.elapsed() < SESSION_IDLE);
        map.insert(
            sid.clone(),
            McpSession {
                agent,
                task: pinned.as_ref().map(|(t, _)| t.clone()),
                generation: pinned.map(|(_, g)| g),
                last_used: Instant::now(),
            },
        );
    }
    let asked = params["protocolVersion"].as_str().unwrap_or_default();
    let version = PROTOCOL_VERSIONS
        .iter()
        .find(|v| **v == asked)
        .unwrap_or(&PROTOCOL_VERSIONS[0]);
    Ok((
        sid,
        json!({
            "protocolVersion": version,
            "capabilities": {"tools": {"listChanged": false}},
            "serverInfo": {"name": "Scholion", "version": env!("CARGO_PKG_VERSION")},
            "instructions": INSTRUCTIONS,
        }),
    ))
}

/// `POST /mcp`: one JSON-RPC request or notification (batches are not supported).
pub(crate) async fn post(State(st): State<AppState>, headers: HeaderMap, body: Bytes) -> Response {
    let Ok(msg) = serde_json::from_slice::<Value>(&body) else {
        return http(
            StatusCode::BAD_REQUEST,
            reply(
                &Value::Null,
                Err(RpcError {
                    code: PARSE_ERROR,
                    message: "not JSON".into(),
                }),
            ),
        );
    };
    let id = msg.get("id").cloned();
    let method = msg["method"].as_str().unwrap_or_default().to_owned();
    if !msg.is_object() || method.is_empty() {
        return http(
            StatusCode::BAD_REQUEST,
            reply(
                &id.unwrap_or(Value::Null),
                Err(RpcError {
                    code: INVALID_REQUEST,
                    message: "expected one JSON-RPC request object (batches are not supported)"
                        .into(),
                }),
            ),
        );
    }
    let params = msg.get("params").cloned().unwrap_or(json!({}));
    if method == "initialize" {
        let (sid, result) = match initialize(&st, &headers, &params).await {
            Ok(ok) => ok,
            Err(e) => return http(StatusCode::OK, reply(&id.unwrap_or(Value::Null), Err(e))),
        };
        let mut resp = http(
            StatusCode::OK,
            reply(&id.unwrap_or(Value::Null), Ok(result)),
        );
        resp.headers_mut()
            .insert(SESSION_HEADER, sid.parse().expect("uuid is a valid header"));
        return resp;
    }
    let Some(sid) = headers.get(SESSION_HEADER).and_then(|v| v.to_str().ok()) else {
        return http(
            StatusCode::BAD_REQUEST,
            reply(
                &id.unwrap_or(Value::Null),
                Err(RpcError {
                    code: INVALID_REQUEST,
                    message: "missing Mcp-Session-Id: call initialize first".into(),
                }),
            ),
        );
    };
    let Some(session) = session(&st, sid) else {
        return http(
            StatusCode::NOT_FOUND,
            reply(
                &id.unwrap_or(Value::Null),
                Err(RpcError {
                    code: INVALID_REQUEST,
                    message: "unknown or expired session: initialize again".into(),
                }),
            ),
        );
    };
    if let Some(s) = st
        .mcp
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get_mut(sid)
    {
        s.last_used = Instant::now();
    }
    let Some(id) = id else {
        // a notification (`notifications/initialized`, cancellations): nothing to answer
        return StatusCode::ACCEPTED.into_response();
    };
    let outcome = match method.as_str() {
        "ping" => Ok(json!({})),
        "tools/list" => Ok(json!({"tools": tools()})),
        "tools/call" => call_tool(&st, sid, &session, &params).await,
        other => Err(RpcError {
            code: METHOD_NOT_FOUND,
            message: format!("method not supported: {other}"),
        }),
    };
    http(StatusCode::OK, reply(&id, outcome))
}

/// `GET /mcp` would open a server-to-client stream; this server never sends unprompted messages.
pub(crate) async fn get() -> Response {
    StatusCode::METHOD_NOT_ALLOWED.into_response()
}

/// `DELETE /mcp` ends a session.
pub(crate) async fn delete(State(st): State<AppState>, headers: HeaderMap) -> Response {
    let removed = headers
        .get(SESSION_HEADER)
        .and_then(|v| v.to_str().ok())
        .and_then(|sid| st.mcp.lock().unwrap_or_else(|e| e.into_inner()).remove(sid));
    match removed {
        Some(_) => StatusCode::NO_CONTENT.into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}
