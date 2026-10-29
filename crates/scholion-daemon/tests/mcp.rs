//! The MCP endpoint (`/mcp`, Plan 3 Task 8): JSON-RPC over streamable HTTP, plain JSON responses.

mod common;

use axum::body::Body;
use common::*;
use tower::ServiceExt;

struct Reply {
    status: StatusCode,
    session: Option<String>,
    body: Value,
}

async fn send(e: &Env, session: Option<&str>, msg: Value) -> Reply {
    let mut req = Request::builder()
        .method("POST")
        .uri("/mcp")
        .header("content-type", "application/json")
        .header("accept", "application/json, text/event-stream");
    if let Some(s) = session {
        req = req.header("mcp-session-id", s);
    }
    let resp = e
        .app
        .clone()
        .oneshot(req.body(Body::from(msg.to_string())).unwrap())
        .await
        .unwrap();
    let status = resp.status();
    let session = resp
        .headers()
        .get("mcp-session-id")
        .map(|v| v.to_str().unwrap().to_owned());
    let bytes = axum::body::to_bytes(resp.into_body(), 64 * 1024 * 1024)
        .await
        .unwrap();
    let body = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    Reply {
        status,
        session,
        body,
    }
}

/// `initialize` as `client`; returns the session id.
async fn connect(e: &Env, client: &str) -> String {
    let r = send(
        e,
        None,
        json!({"jsonrpc": "2.0", "id": 0, "method": "initialize", "params": {
            "protocolVersion": "2025-06-18", "capabilities": {},
            "clientInfo": {"name": client, "version": "1"}}}),
    )
    .await;
    assert_eq!(r.status, StatusCode::OK, "{}", r.body);
    assert_eq!(r.body["result"]["protocolVersion"], "2025-06-18");
    assert!(
        r.body["result"]["capabilities"]["tools"].is_object(),
        "{}",
        r.body
    );
    let session = r.session.expect("initialize returns Mcp-Session-Id");
    let n = send(
        e,
        Some(&session),
        json!({"jsonrpc": "2.0", "method": "notifications/initialized"}),
    )
    .await;
    assert_eq!(n.status, StatusCode::ACCEPTED);
    session
}

async fn tool(e: &Env, session: &str, name: &str, args: Value) -> Value {
    let r = send(
        e,
        Some(session),
        json!({"jsonrpc": "2.0", "id": 7, "method": "tools/call",
               "params": {"name": name, "arguments": args}}),
    )
    .await;
    assert_eq!(r.status, StatusCode::OK, "{}", r.body);
    assert_eq!(r.body["id"], 7);
    r.body
}

/// The text of a successful tool result.
fn text(reply: &Value) -> String {
    assert_eq!(reply["result"]["isError"], false, "{reply}");
    reply["result"]["content"][0]["text"]
        .as_str()
        .unwrap()
        .to_owned()
}

#[tokio::test(flavor = "multi_thread")]
async fn tools_list_contains_every_tool_name() {
    let e = env().await;
    let s = connect(&e, "claude-code").await;
    let r = send(
        &e,
        Some(&s),
        json!({"jsonrpc": "2.0", "id": 1, "method": "tools/list"}),
    )
    .await;
    let mut names: Vec<&str> = r.body["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["name"].as_str().unwrap())
        .collect();
    names.sort();
    assert_eq!(
        names,
        [
            "abandon",
            "checkpoint",
            "claim_task",
            "diff",
            "expand",
            "get_context",
            "handoff",
            "list_conventions",
            "list_files",
            "log",
            "merge",
            "post_review",
            "read_file",
            "read_symbol",
            "record",
            "review_task",
            "search_history",
            "start_task",
            "status",
            "submit",
            "sync",
            "undo",
            "validate",
            "why"
        ]
    );
    let record = r.body["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .find(|t| t["name"] == "record")
        .unwrap();
    assert!(
        record["description"]
            .as_str()
            .unwrap()
            .contains("kind=attempt immediately when an approach fails"),
        "{record}"
    );
    assert_eq!(record["inputSchema"]["type"], "object");
}

#[tokio::test(flavor = "multi_thread")]
async fn an_mcp_only_agent_can_validate_sync_abandon_and_list_conventions() {
    let e = env().await;
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/repos/demo/conventions",
        Some(
            json!({"scope_glob": "app/**/*.py", "text": "type hints everywhere", "required": true}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let s = connect(&e, "claude-code").await;
    text(
        &tool(
            &e,
            &s,
            "start_task",
            json!({"repo": "demo", "title": "Improve greeting", "goal": "Make hello louder"}),
        )
        .await,
    );
    let conventions = text(&tool(&e, &s, "list_conventions", json!({})).await);
    assert!(
        conventions.contains("type hints everywhere") && conventions.contains("required"),
        "{conventions}"
    );
    let validated = text(
        &tool(
            &e,
            &s,
            "validate",
            json!({"command": "pytest -q", "exit_code": 0, "output_tail": "3 passed"}),
        )
        .await,
    );
    assert!(validated.contains("exit 0"), "{validated}");
    let synced = text(&tool(&e, &s, "sync", json!({})).await);
    assert!(
        synced.contains("no conflicts") || synced.contains("open"),
        "{synced}"
    );
    let abandoned = text(&tool(&e, &s, "abandon", json!({})).await);
    assert!(abandoned.contains("abandoned"), "{abandoned}");
}

#[tokio::test(flavor = "multi_thread")]
async fn read_tools_need_no_workspace() {
    let e = env().await;
    let s = connect(&e, "reviewer").await;
    let file = text(
        &tool(
            &e,
            &s,
            "read_file",
            json!({"repo": "demo", "path": "app/util.py"}),
        )
        .await,
    );
    assert!(file.contains("return n.title()"), "{file}");
    let files = text(&tool(&e, &s, "list_files", json!({"repo": "demo"})).await);
    assert!(
        files.contains("app/greeter.py") && files.contains("app/util.py"),
        "{files}"
    );
    let log = text(
        &tool(
            &e,
            &s,
            "log",
            json!({"repo": "demo", "path": "app/util.py"}),
        )
        .await,
    );
    assert!(log.contains("init"), "{log}");
    let diff = text(
        &tool(
            &e,
            &s,
            "diff",
            json!({"repo": "demo", "from": "main", "to": "main"}),
        )
        .await,
    );
    assert!(diff.contains("no changes"), "{diff}");
    let missing = tool(
        &e,
        &s,
        "read_file",
        json!({"repo": "demo", "path": "nope.py"}),
    )
    .await;
    assert_eq!(missing["result"]["isError"], true, "{missing}");
}

#[tokio::test(flavor = "multi_thread")]
async fn start_task_then_get_context_roundtrip_over_mcp() {
    let e = env().await;
    let s = connect(&e, "claude-code").await;
    let started = text(
        &tool(
            &e,
            &s,
            "start_task",
            json!({"repo": "demo", "title": "Improve greeting", "goal": "Make hello louder"}),
        )
        .await,
    );
    assert!(
        started.contains(&format!("workspace: {HOST_WS}/demo/improve-greeting")),
        "{started}"
    );
    assert!(started.contains("## Task"), "{started}");
    let status = text(&tool(&e, &s, "status", json!({})).await);
    assert!(
        status.contains("\"owner_agent\": \"claude-code\""),
        "agent from clientInfo: {status}"
    );
    let pack = text(&tool(&e, &s, "get_context", json!({"budget": 2000})).await);
    assert!(pack.contains("## Task"), "{pack}");
}

#[tokio::test(flavor = "multi_thread")]
async fn second_get_context_in_same_session_marks_items_sent_earlier() {
    let e = env().await;
    let s = connect(&e, "claude-code").await;
    tool(
        &e,
        &s,
        "start_task",
        json!({"repo": "demo", "title": "Improve greeting", "goal": "Make hello louder"}),
    )
    .await;
    let again = text(&tool(&e, &s, "get_context", json!({})).await);
    assert!(again.contains("(sent earlier)"), "{again}");
    // another MCP session sees everything in full
    let other = connect(&e, "claude-code").await;
    let task = text(&tool(&e, &s, "status", json!({})).await);
    let id: Value = serde_json::from_str(&task).unwrap();
    let fresh = text(
        &tool(
            &e,
            &other,
            "get_context",
            json!({"task_id": id["task"]["id"]}),
        )
        .await,
    );
    assert!(!fresh.contains("(sent earlier)"), "{fresh}");
}

#[tokio::test(flavor = "multi_thread")]
async fn malformed_arguments_return_jsonrpc_error_and_write_nothing() {
    let e = env().await;
    let s = connect(&e, "claude-code").await;
    tool(
        &e,
        &s,
        "start_task",
        json!({"repo": "demo", "title": "Improve greeting", "goal": "Make hello louder"}),
    )
    .await;
    let status: Value =
        serde_json::from_str(&text(&tool(&e, &s, "status", json!({})).await)).unwrap();
    let before = status["record_count"].clone();
    let missing = tool(&e, &s, "record", json!({"kind": "note"})).await;
    assert_eq!(missing["error"]["code"], -32602, "{missing}");
    assert!(
        missing["error"]["message"]
            .as_str()
            .unwrap()
            .contains("summary"),
        "{missing}"
    );
    let huge = tool(
        &e,
        &s,
        "record",
        json!({"kind": "note", "summary": "big", "body": "x".repeat(5 * 1024 * 1024)}),
    )
    .await;
    assert_eq!(huge["error"]["code"], -32602, "{}", huge["error"]);
    let long_body = tool(
        &e,
        &s,
        "record",
        json!({"kind": "note", "summary": "big", "body": "x".repeat(70 * 1024)}),
    )
    .await;
    assert_eq!(
        long_body["error"]["code"], -32602,
        "body over 64 KiB: {}",
        long_body["error"]
    );
    let long_summary = tool(
        &e,
        &s,
        "record",
        json!({"kind": "note", "summary": "s".repeat(201)}),
    )
    .await;
    assert_eq!(
        long_summary["error"]["code"], -32602,
        "{}",
        long_summary["error"]
    );
    let status: Value =
        serde_json::from_str(&text(&tool(&e, &s, "status", json!({})).await)).unwrap();
    assert_eq!(status["record_count"], before, "nothing written");
}

#[tokio::test(flavor = "multi_thread")]
async fn unknown_tool_is_a_jsonrpc_error() {
    let e = env().await;
    let s = connect(&e, "claude-code").await;
    let r = tool(&e, &s, "rm_rf", json!({})).await;
    assert_eq!(r["error"]["code"], -32602, "{r}");
    let r = send(
        &e,
        Some(&s),
        json!({"jsonrpc": "2.0", "id": 3, "method": "resources/list"}),
    )
    .await;
    assert_eq!(r.body["error"]["code"], -32601, "{}", r.body);
}

#[tokio::test(flavor = "multi_thread")]
async fn a_refused_operation_is_a_tool_error_not_a_protocol_error() {
    let e = env().await;
    let s = connect(&e, "claude-code").await;
    tool(
        &e,
        &s,
        "start_task",
        json!({"repo": "demo", "title": "Improve greeting", "goal": "Make hello louder"}),
    )
    .await;
    let r = tool(&e, &s, "submit", json!({})).await;
    assert_eq!(r["result"]["isError"], true, "{r}");
    assert!(
        r["result"]["content"][0]["text"]
            .as_str()
            .unwrap()
            .contains("nothing to submit"),
        "{r}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn instructions_do_not_ask_for_manual_checkpoints() {
    let e = env().await;
    let r = send(
        &e,
        None,
        json!({"jsonrpc": "2.0", "id": 0, "method": "initialize", "params": {
            "protocolVersion": "2025-06-18", "capabilities": {},
            "clientInfo": {"name": "claude-code", "version": "1"}}}),
    )
    .await;
    let instructions = r.body["result"]["instructions"].as_str().unwrap();
    assert!(
        !instructions.contains("checkpoint` after edits"),
        "{instructions}"
    );
    assert!(instructions.contains("submit checkpoints for you"));

    // get_context without a budget asks for 800 tokens
    let s = r.session.unwrap();
    let t = start_task(&e, "Improve greeting").await;
    let out = text(&tool(&e, &s, "get_context", json!({"task_id": id(&t)})).await);
    let rcp = out.rsplit("receipt: ").next().unwrap().trim().to_owned();
    let (_, receipt) = call(&e.app, "GET", &format!("/v1/receipts/{rcp}"), None).await;
    assert_eq!(receipt["budget"], 800, "{receipt}");
}

#[tokio::test(flavor = "multi_thread")]
async fn submit_tool_accepts_records() {
    let e = env().await;
    let s = connect(&e, "claude-code").await;
    let list = send(
        &e,
        Some(&s),
        json!({"jsonrpc": "2.0", "id": 1, "method": "tools/list"}),
    )
    .await;
    let submit = list.body["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .find(|t| t["name"] == "submit")
        .unwrap()
        .clone();
    let records = &submit["inputSchema"]["properties"]["records"];
    assert_eq!(records["type"], "array", "{submit}");
    assert_eq!(records["items"]["required"], json!(["kind", "summary"]));
    assert!(records["items"]["properties"]["anchors"].is_object());

    let t = start_task(&e, "Improve greeting").await;
    edit_greeter(&e, &t);
    let r = tool(
        &e,
        &s,
        "submit",
        json!({"task_id": id(&t), "records": [
            {"kind": "decision", "summary": "Greet by name",
             "anchors": [{"path": "app/greeter.py"}]}]}),
    )
    .await;
    assert_ne!(r["result"]["isError"], true, "{r}");
    let out = text(&r);
    assert!(out.contains("in_review") || out.contains("review"), "{out}");
    let status: Value = serde_json::from_str(&text(
        &tool(&e, &s, "status", json!({"task_id": id(&t)})).await,
    ))
    .unwrap();
    assert_eq!(
        status["record_count"], 2,
        "intent + inline decision: {status}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn requests_need_the_session_from_initialize() {
    let e = env().await;
    let list = json!({"jsonrpc": "2.0", "id": 1, "method": "tools/list"});
    assert_eq!(
        send(&e, None, list.clone()).await.status,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        send(&e, Some("no-such-session"), list.clone()).await.status,
        StatusCode::NOT_FOUND
    );
    let s = connect(&e, "x").await;
    let ping = send(
        &e,
        Some(&s),
        json!({"jsonrpc": "2.0", "id": 2, "method": "ping"}),
    )
    .await;
    assert_eq!(ping.body["result"], json!({}));
}

#[tokio::test(flavor = "multi_thread")]
async fn claim_over_mcp_writes_under_the_new_generation_and_fences_the_old_owner() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    edit_greeter(&e, &t);
    record_decision(&e, &t).await;
    assert_eq!(handoff(&e, &t, "claude").await.0, StatusCode::OK);
    let s = connect(&e, "codex").await;
    let claimed = text(&tool(&e, &s, "claim_task", json!({"task_id": id(&t)})).await);
    assert!(claimed.contains("generation: 2"), "{claimed}");
    assert!(
        claimed.contains("Remaining: make goodbye louder"),
        "{claimed}"
    );
    let ok = tool(&e, &s, "checkpoint", json!({})).await;
    text(&ok);
    let (status, body) = call_as(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/checkpoint", id(&t)),
        None,
        Some(1),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn mcp_needs_the_token_when_one_is_set() {
    let e = env_with(|s| s.with_token(Some("sekrit".into()))).await;
    let r = send(
        &e,
        None,
        json!({"jsonrpc": "2.0", "id": 0, "method": "initialize", "params": {
            "protocolVersion": "2025-06-18", "capabilities": {},
            "clientInfo": {"name": "x", "version": "1"}}}),
    )
    .await;
    assert_eq!(r.status, StatusCode::UNAUTHORIZED);
}

async fn initialize_with(e: &Env, header: (&str, &str)) -> (StatusCode, Option<String>, Value) {
    let req = Request::builder()
        .method("POST")
        .uri("/mcp")
        .header("content-type", "application/json")
        .header(header.0, header.1)
        .body(Body::from(
            json!({"jsonrpc": "2.0", "id": 0, "method": "initialize", "params": {
                "protocolVersion": "2025-06-18", "capabilities": {},
                "clientInfo": {"name": "claude-code", "version": "1"}}})
            .to_string(),
        ))
        .unwrap();
    let resp = e.app.clone().oneshot(req).await.unwrap();
    let status = resp.status();
    let sid = resp
        .headers()
        .get("mcp-session-id")
        .map(|v| v.to_str().unwrap().to_owned());
    let bytes = axum::body::to_bytes(resp.into_body(), 1 << 20)
        .await
        .unwrap();
    (
        status,
        sid,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

#[tokio::test(flavor = "multi_thread")]
async fn x_av_task_header_pins_the_session_to_a_task() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    let (status, sid, body) = initialize_with(&e, ("x-av-task", &id(&t))).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let s = sid.expect("session id");
    let pack = text(&tool(&e, &s, "get_context", json!({})).await);
    assert!(pack.contains("Improve greeting"), "{pack}");
    let rec = tool(
        &e,
        &s,
        "record",
        json!({"kind": "note", "summary": "pinned"}),
    )
    .await;
    text(&rec);
    let (status, _, body) =
        initialize_with(&e, ("x-av-task", &uuid::Uuid::new_v4().to_string())).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["error"]["code"], -32602, "{body}");
    let (_, _, body) = initialize_with(&e, ("x-av-task", "not-a-uuid")).await;
    assert_eq!(body["error"]["code"], -32602, "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn mcp_packs_are_800_tokens_of_signatures_unless_asked() {
    let e = env().await;
    let s = connect(&e, "claude-code").await;
    let started = text(
        &tool(
            &e,
            &s,
            "start_task",
            json!({"repo": "demo", "title": "Lean start", "goal": "format_name"}),
        )
        .await,
    );
    assert!(started.contains("/800\n"), "{started}");
    assert!(!started.contains("n.title()"), "no bodies: {started}");
    let full = text(
        &tool(
            &e,
            &s,
            "get_context",
            json!({"code": "full", "focus": "format_name"}),
        )
        .await,
    );
    assert!(full.contains("n.title()"), "code=full has bodies: {full}");
}
