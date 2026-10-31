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
    // Plan 9: how to take the next job and do a review, and why git status looks empty. Not a
    // loop: an agent the user opened for their own work must not start pulling backlog work.
    assert!(
        instructions.contains("with no title or goal"),
        "{instructions}"
    );
    for step in ["`review_task`", "`validate`", "`post_review`", "`merge`"] {
        assert!(instructions.contains(step), "{step}: {instructions}");
    }
    assert!(instructions.contains("`git status` shows nothing"));
    assert!(!instructions.to_lowercase().contains("loop"));
    assert!(!instructions.contains("again"));
    // sent on every turn: about 160 tokens at most
    assert!(
        instructions.len() <= 640,
        "instructions are {} bytes",
        instructions.len()
    );

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

// ---- Plan 8 Task 5: routing over MCP

async fn put_profile(e: &Env, name: &str, body: Value) {
    let (status, out) = call(&e.app, "PUT", &format!("/v1/agents/{name}"), Some(body)).await;
    assert_eq!(status, StatusCode::OK, "{out}");
}

async fn post_backlog(e: &Env, title: &str, needs: &[&str], reserve: i64) {
    let (status, out) = call(
        &e.app,
        "POST",
        "/v1/backlog",
        Some(
            json!({"repo": "demo", "title": title, "goal": "Make hello louder",
                    "needs": needs, "reserve_secs": reserve}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{out}");
}

#[tokio::test(flavor = "multi_thread")]
async fn start_task_without_title_and_goal_takes_the_next_backlog_task() {
    let e = env().await;
    // Grok introduces itself as `grok-shell-av`; the profile's alias maps it to `grok`.
    put_profile(
        &e,
        "grok",
        json!({"best": ["image"], "cost_rank": 2, "aliases": ["grok-shell-av"]}),
    )
    .await;
    post_backlog(&e, "look at the logo", &["image"], 600).await;
    let s = connect(&e, "grok-shell-av").await;
    let out = text(&tool(&e, &s, "start_task", json!({})).await);
    assert!(out.contains("workspace:"), "{out}");
    assert!(out.contains("generation: 1"), "{out}");

    // the session now has a task and its owner is the profile name, so later calls just work
    let status = text(&tool(&e, &s, "status", json!({})).await);
    assert!(status.contains("\"owner_agent\": \"grok\""), "{status}");
}

#[tokio::test(flavor = "multi_thread")]
async fn start_task_without_arguments_says_when_nothing_is_waiting() {
    let e = env().await;
    post_backlog(&e, "draw it", &["image"], 0).await;
    let s = connect(&e, "claude-code").await;
    let out = text(&tool(&e, &s, "start_task", json!({})).await);
    assert!(out.contains("No task is waiting for you"), "{out}");
    assert!(out.contains("1 queued"), "{out}");
    // no task was remembered
    let r = tool(&e, &s, "status", json!({})).await;
    assert_eq!(r["error"]["code"], -32602, "{r}");
}

#[tokio::test(flavor = "multi_thread")]
async fn start_task_with_a_title_still_needs_repo_and_goal() {
    let e = env().await;
    let s = connect(&e, "claude-code").await;
    let r = tool(&e, &s, "start_task", json!({"repo": "demo", "title": "x"})).await;
    assert_eq!(r["error"]["code"], -32602, "{r}");
    assert!(
        r["error"]["message"].as_str().unwrap().contains("goal"),
        "{r}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn a_handoff_with_needs_is_claimed_by_start_task_without_arguments() {
    let e = env().await;
    put_profile(&e, "grok", json!({"best": ["image"], "cost_rank": 2})).await;
    let t = start_task(&e, "Make hello louder").await;
    edit_greeter(&e, &t);
    record_decision(&e, &t).await;

    // the owner hands off over MCP, naming the tag the next agent must have
    let first = connect(&e, "claude").await;
    let out = text(
        &tool(
            &e,
            &first,
            "handoff",
            json!({"task_id": id(&t), "done": "shouts", "remaining": "draw the logo",
                   "needs": ["Image"]}),
        )
        .await,
    );
    assert!(out.starts_with("handed_off"), "{out}");

    let second = connect(&e, "grok").await;
    let out = text(&tool(&e, &second, "start_task", json!({})).await);
    assert!(out.contains("generation: 2"), "{out}");
    assert!(out.contains("draw the logo"), "{out}");
}

#[tokio::test(flavor = "multi_thread")]
async fn needs_must_be_a_list_of_strings() {
    let e = env().await;
    let s = connect(&e, "claude-code").await;
    let r = tool(
        &e,
        &s,
        "handoff",
        json!({"task_id": "00000000-0000-0000-0000-000000000000", "done": "a",
               "remaining": "b", "needs": "image"}),
    )
    .await;
    assert_eq!(r["error"]["code"], -32602, "{r}");
}

// ---- Plan 9 Task 4: slugs for task ids; review offers over MCP

async fn op(e: &Env, task: &str, op: &str, body: Value) {
    let (status, out) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{task}/{op}"),
        Some(body),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{op}: {out}");
}

/// A task of `demo` that claude submitted, ready for review.
async fn submitted(e: &Env, title: &str) -> Value {
    let t = start_task(e, title).await;
    edit_greeter(e, &t);
    record_decision(e, &t).await;
    op(e, &id(&t), "submit", json!({"agent": "claude"})).await;
    t
}

fn slug(t: &Value) -> String {
    t["task"]["slug"].as_str().unwrap().to_owned()
}

#[tokio::test(flavor = "multi_thread")]
async fn status_takes_a_slug() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    let s = connect(&e, "grok").await;
    let by_id = text(&tool(&e, &s, "status", json!({"task_id": id(&t)})).await);
    let by_slug = text(&tool(&e, &s, "status", json!({"task_id": slug(&t)})).await);
    assert_eq!(by_id, by_slug);
}

#[tokio::test(flavor = "multi_thread")]
async fn review_task_takes_repo_slash_slug() {
    let e = env().await;
    let t = submitted(&e, "Improve greeting").await;
    let s = connect(&e, "grok").await;
    let out = text(
        &tool(
            &e,
            &s,
            "review_task",
            json!({"task_id": format!("demo/{}", slug(&t))}),
        )
        .await,
    );
    assert!(out.contains("## Tests"), "{out}");
}

#[tokio::test(flavor = "multi_thread")]
async fn an_ambiguous_slug_names_every_match() {
    let e = env().await;
    let (status, out) = call(
        &e.app,
        "POST",
        "/v1/repos",
        Some(json!({"name": "other", "git_source": e.src_path})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{out}");
    let t = start_task(&e, "Improve greeting").await;
    let (status, out) = call(
        &e.app,
        "POST",
        "/v1/tasks",
        Some(
            json!({"repo": "other", "title": "Improve greeting", "goal": "g",
                    "agent": "claude", "session": "s-2"}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{out}");
    let s = connect(&e, "grok").await;
    let r = tool(&e, &s, "status", json!({"task_id": slug(&t)})).await;
    assert_eq!(r["error"]["code"], -32602, "{r}");
    let msg = r["error"]["message"].as_str().unwrap();
    assert!(msg.contains(&format!("demo/{}", slug(&t))), "{msg}");
    assert!(msg.contains(&format!("other/{}", slug(&t))), "{msg}");
    // naming the repo picks one
    let out = text(
        &tool(
            &e,
            &s,
            "status",
            json!({"task_id": format!("other/{}", slug(&t))}),
        )
        .await,
    );
    assert!(out.contains("\"repo\": \"other\""), "{out}");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_merged_task_resolves_by_slug() {
    let e = env().await;
    let t = submitted(&e, "Improve greeting").await;
    op(
        &e,
        &id(&t),
        "validate",
        json!({"agent": "grok", "command": "pytest", "exit_code": 0}),
    )
    .await;
    op(
        &e,
        &id(&t),
        "review",
        json!({"reviewer": "grok", "verdict": "approve"}),
    )
    .await;
    op(&e, &id(&t), "merge", json!({"agent": "grok"})).await;
    let s = connect(&e, "grok").await;
    let out = text(&tool(&e, &s, "status", json!({"task_id": slug(&t)})).await);
    assert!(out.contains("\"status\": \"merged\""), "{out}");
}

#[tokio::test(flavor = "multi_thread")]
async fn an_unknown_slug_says_so() {
    let e = env().await;
    let s = connect(&e, "grok").await;
    let r = tool(&e, &s, "status", json!({"task_id": "no-such-task"})).await;
    assert_eq!(r["error"]["code"], -32602, "{r}");
    let msg = r["error"]["message"].as_str().unwrap();
    assert!(
        msg.contains("no task") && msg.contains("no-such-task"),
        "{msg}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn start_task_without_arguments_offers_a_review_and_does_not_adopt_it() {
    let e = env().await;
    let t = submitted(&e, "Improve greeting").await;
    let s = connect(&e, "grok").await;
    let out = text(&tool(&e, &s, "start_task", json!({})).await);
    assert!(out.starts_with("review:"), "{out}");
    assert!(
        out.contains(&id(&t)) && out.contains(&format!("demo/{}", slug(&t))),
        "{out}"
    );
    assert!(out.contains("owner: claude"), "{out}");
    assert!(out.contains("Review it: review_task"), "{out}");
    // the reviewer does not own it: the session has no task of its own
    let r = tool(&e, &s, "status", json!({})).await;
    assert_eq!(r["error"]["code"], -32602, "{r}");
}

#[tokio::test(flavor = "multi_thread")]
async fn start_task_passes_wait() {
    let e = env_with(|s| s.with_max_wait(std::time::Duration::from_secs(1))).await;
    let s = connect(&e, "grok").await;
    let t = std::time::Instant::now();
    let out = text(&tool(&e, &s, "start_task", json!({"wait": 30})).await);
    assert!(out.contains("No task is waiting for you"), "{out}");
    assert!(
        t.elapsed() >= std::time::Duration::from_millis(900),
        "it waited"
    );
    assert!(out.contains("wait"), "says it can wait: {out}");
}

#[tokio::test(flavor = "multi_thread")]
async fn the_tool_list_stays_small() {
    let e = env().await;
    let s = connect(&e, "claude-code").await;
    let r = send(
        &e,
        Some(&s),
        json!({"jsonrpc": "2.0", "id": 1, "method": "tools/list"}),
    )
    .await;
    let bytes = serde_json::to_string(&r.body["result"]["tools"])
        .unwrap()
        .len();
    // Every tool definition is sent to the model on every turn. Plan 8 added 160 bytes (about 40
    // tokens) to the 10,040 before it, Plan 9 112 (start_task's wait and its longer description);
    // a bigger tool list needs a reason that pays for itself.
    assert!(
        bytes <= 10_320,
        "tools/list is {bytes} bytes; it was 10,312 after Plan 9"
    );
}
