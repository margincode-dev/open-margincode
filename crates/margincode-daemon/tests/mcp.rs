//! The MCP endpoint (`/mcp`): JSON-RPC over streamable HTTP, plain JSON responses.

use crate::common::*;
use axum::body::Body;
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
            "ask",
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
            "post_backlog",
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
    // a caller with no task (a dream, a reviewer) is told to name the repository, not to start
    // a task: those tools may not be in its catalogue
    let no_repo = tool(&e, &s, "read_file", json!({"path": "app/util.py"})).await;
    let msg = no_repo["error"]["message"].as_str().unwrap_or_default();
    assert!(
        msg.contains("pass `repo`") && !msg.contains("start_task"),
        "{no_repo}"
    );
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
async fn the_record_tool_corrects_a_record_by_the_handle_a_pack_prints() {
    let e = env().await;
    let earlier = start_task(&e, "Earlier work").await;
    let (status, wrong) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/records", id(&earlier)),
        Some(
            json!({"kind": "decision", "summary": "names stay lower case", "agent": "claude",
                    "anchors": [{"path": "app/util.py"}]}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{wrong}");
    let handle = format!(
        "rec:{}",
        &wrong["id"].as_str().unwrap().replace('-', "")[..8]
    );

    let s = connect(&e, "grok").await;
    let list = send(
        &e,
        Some(&s),
        json!({"jsonrpc": "2.0", "id": 1, "method": "tools/list"}),
    )
    .await;
    let record = list.body["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .find(|t| t["name"] == "record")
        .unwrap();
    let property = &record["inputSchema"]["properties"]["supersedes"];
    assert_eq!(property["type"], "string", "{record}");
    assert!(
        property["description"].as_str().unwrap().contains("rec:"),
        "says what to pass: {record}"
    );

    tool(
        &e,
        &s,
        "start_task",
        json!({"repo": "demo", "title": "Later work", "goal": "Make hello louder"}),
    )
    .await;
    let said = text(
        &tool(
            &e,
            &s,
            "record",
            json!({"kind": "note", "summary": "names are title case", "supersedes": handle}),
        )
        .await,
    );
    assert!(said.starts_with("recorded rec:"), "{said}");
    assert!(
        said.ends_with(&format!("(note), supersedes {handle}")),
        "names what was superseded: {said}"
    );
    let (_, records) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/records", id(&earlier)),
        None,
    )
    .await;
    let old = &records["records"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["id"] == wrong["id"])
        .unwrap();
    assert!(old["superseded_by"].is_string(), "{old}");

    let r = tool(
        &e,
        &s,
        "record",
        json!({"kind": "note", "summary": "again", "supersedes": handle}),
    )
    .await;
    assert_eq!(r["result"]["isError"], true, "{r}");
    assert!(
        r["result"]["content"][0]["text"]
            .as_str()
            .unwrap()
            .contains("already superseded"),
        "{r}"
    );
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
    // How to take the next job and do a review, and why git status looks empty. Not a
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

fn refusal(reply: &Value) -> &str {
    reply["result"]["content"][0]["text"].as_str().unwrap()
}

#[tokio::test(flavor = "multi_thread")]
async fn submit_lists_supersedes_for_its_inline_records_and_takes_it() {
    let e = env().await;
    let earlier = start_task(&e, "Earlier work").await;
    let mut handles = vec![];
    for summary in ["names stay lower case", "hello stays quiet"] {
        let (status, wrong) = call(
            &e.app,
            "POST",
            &format!("/v1/tasks/{}/records", id(&earlier)),
            Some(
                json!({"kind": "decision", "summary": summary, "agent": "claude",
                        "anchors": [{"path": "app/util.py"}]}),
            ),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{wrong}");
        handles.push(format!(
            "rec:{}",
            &wrong["id"].as_str().unwrap().replace('-', "")[..8]
        ));
    }

    let s = connect(&e, "claude-code").await;
    let list = send(
        &e,
        Some(&s),
        json!({"jsonrpc": "2.0", "id": 1, "method": "tools/list"}),
    )
    .await;
    let tools = list.body["result"]["tools"].as_array().unwrap();
    let named = |name: &str| tools.iter().find(|t| t["name"] == name).unwrap();
    // `submit` says its records take the same fields as `record`: every one `record` lists but
    // the task is listed for them too
    let inline = &named("submit")["inputSchema"]["properties"]["records"]["items"]["properties"];
    assert_eq!(inline["supersedes"], json!({"type": "string"}), "{inline}");
    for (field, _) in named("record")["inputSchema"]["properties"]
        .as_object()
        .unwrap()
    {
        assert!(
            field == "task_id" || inline[field].is_object(),
            "`record` takes {field}, submit's records do not list it: {inline}"
        );
    }

    let t = start_task(&e, "Improve greeting").await;
    edit_greeter(&e, &t);
    let r = tool(
        &e,
        &s,
        "record",
        json!({"task_id": id(&t), "kind": "intent", "summary": "Make hello louder",
               "supersedes": handles[0]}),
    )
    .await;
    assert_eq!(r["result"]["isError"], true, "{r}");
    assert!(refusal(&r).contains("decision, attempt or note"), "{r}");
    let r = tool(
        &e,
        &s,
        "submit",
        json!({"task_id": id(&t), "records": [
            {"kind": "intent", "summary": "Make hello louder", "supersedes": handles[0]}]}),
    )
    .await;
    assert_eq!(r["result"]["isError"], true, "{r}");
    assert!(refusal(&r).contains("decision, attempt or note"), "{r}");

    let r = tool(
        &e,
        &s,
        "submit",
        json!({"task_id": id(&t), "records": [
            {"kind": "decision", "summary": "names are title case", "supersedes": handles[0]}]}),
    )
    .await;
    assert_ne!(r["result"]["isError"], true, "{r}");
    let (_, records) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/records", id(&earlier)),
        None,
    )
    .await;
    let superseded: Vec<&Value> = records["records"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["superseded_by"].is_string())
        .collect();
    assert_eq!(superseded.len(), 1, "{records}");
    assert_eq!(superseded[0]["summary"], "names stay lower case");
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

    let status = text(&tool(&e, &s, "status", json!({})).await);
    assert!(status.contains("\"owner_agent\": \"grok\""), "{status}");
}

#[tokio::test(flavor = "multi_thread")]
async fn post_backlog_queues_or_proposes_follow_up_work() {
    let e = env().await;
    let s = connect(&e, "claude-code").await;
    let out = text(
        &tool(
            &e,
            &s,
            "post_backlog",
            json!({"repo": "demo", "title": "Add a farewell", "goal": "Say goodbye"}),
        )
        .await,
    );
    assert!(out.starts_with("queued: "), "{out}");
    assert!(out.contains("Add a farewell"), "{out}");
    text(
        &tool(
            &e,
            &s,
            "start_task",
            json!({"repo": "demo", "title": "Louder", "goal": "g"}),
        )
        .await,
    );
    let out = text(
        &tool(
            &e,
            &s,
            "post_backlog",
            json!({"title": "Cache greetings", "goal": "Cache them", "propose": true,
                   "why": "might not be worth it", "after": ["add-a-farewell"]}),
        )
        .await,
    );
    assert!(out.starts_with("proposed"), "{out}");
    let (_, list) = call(&e.app, "GET", "/v1/backlog?repo=demo", None).await;
    let items = list["items"].as_array().unwrap();
    assert_eq!(items.len(), 2, "{list}");
    assert_eq!(items[0]["state"], "queued");
    assert_eq!(items[0]["posted_by"], "claude-code");
    assert_eq!(items[1]["state"], "proposed");
    assert_eq!(items[1]["why"], "might not be worth it");
    assert_eq!(items[1]["waiting_on"], json!(["Add a farewell"]));

    // the repo proposes every agent's post: an MCP post counts as an agent's, profile or not
    let (status, b) = call(
        &e.app,
        "PUT",
        "/v1/repos/demo/settings",
        Some(json!({"agent_posts": "propose"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{b}");
    let out = text(
        &tool(
            &e,
            &s,
            "post_backlog",
            json!({"title": "Sure thing", "goal": "g"}),
        )
        .await,
    );
    assert!(out.starts_with("proposed"), "{out}");
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
async fn start_task_at_the_limit_says_to_finish_work_first() {
    let e = env().await;
    put_profile(
        &e,
        "claude",
        json!({"best": ["code"], "aliases": ["claude-code"], "max_open": 1}),
    )
    .await;
    post_backlog(&e, "first", &["code"], 600).await;
    post_backlog(&e, "second", &["code"], 600).await;
    let s = connect(&e, "claude-code").await;
    let out = text(&tool(&e, &s, "start_task", json!({})).await);
    assert!(out.contains("workspace:"), "{out}");
    let out = text(&tool(&e, &s, "start_task", json!({})).await);
    assert!(out.contains("You hold 1 open task, your limit"), "{out}");
    assert!(out.contains("submit or hand off"), "{out}");
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
async fn status_shows_no_backup_credentials() {
    let e = env().await;
    let remote = "https://user:s3cr3t-tok@127.0.0.1:1/r.git";
    e.store
        .set_backup_remote("demo", Some(remote), None)
        .await
        .unwrap();
    let error = format!("git [\"push\", \"--\", \"{remote}\"]: fatal");
    e.store
        .backup_failed("demo", &error, std::time::Duration::from_secs(300))
        .await
        .unwrap();
    let t = start_task(&e, "Improve greeting").await;
    let s = connect(&e, "grok").await;
    let out = text(&tool(&e, &s, "status", json!({"task_id": id(&t)})).await);
    assert!(out.contains("127.0.0.1:1"), "{out}");
    assert!(!out.contains("s3cr3t-tok"), "{out}");
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

/// An extension's MCP tool: echoes `text`, refuses "boom".
struct Echo;

#[async_trait::async_trait]
impl margincode_daemon::ToolHandler for Echo {
    async fn call(&self, agent: &str, args: Value) -> Result<String, String> {
        match args["text"].as_str() {
            Some("boom") => Err("echo refused".into()),
            Some(t) => Ok(format!("{agent} says {t}")),
            None => Err("no text".into()),
        }
    }
}

fn echo_tool() -> margincode_daemon::ExtTool {
    margincode_daemon::ExtTool::new(
        "ee_echo",
        "Echo text back (extension).",
        json!({"type": "object", "required": ["text"],
               "properties": {"text": {"type": "string"}}}),
        std::sync::Arc::new(Echo),
    )
}

#[tokio::test(flavor = "multi_thread")]
async fn the_ask_tool_answers_with_one_instruction_and_says_when_to_ask() {
    let e = env().await;
    let s = connect_named(&e, "claude").await;
    let list = send(
        &e,
        Some(&s),
        json!({"jsonrpc": "2.0", "id": 1, "method": "tools/list"}),
    )
    .await;
    let ask = list.body["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .find(|t| t["name"] == "ask")
        .cloned()
        .expect("an ask tool");
    let said = ask["description"].as_str().unwrap();
    assert!(
        said.contains(
            "when you would otherwise guess about something a person would want a say in"
        ),
        "{said}"
    );
    assert!(said.contains("follow"), "{said}");
    assert_eq!(
        ask["inputSchema"]["properties"]["undo"]["enum"],
        json!(["workspace", "outside"]),
        "{ask}"
    );

    tool(
        &e,
        &s,
        "start_task",
        json!({"repo": "demo", "title": "Cache greetings", "goal": "Make hello louder"}),
    )
    .await;
    let question = |undo: &str| {
        json!({"question": "Which database should the cache use?",
               "options": ["Postgres", "SQLite"], "recommended": "Postgres",
               "why": "the daemon already runs one", "depends": "the cache's schema",
               "undo": undo, "remaining": "its storage", "done": null, "anchors": null})
    };
    let mut two_lines = question("workspace");
    two_lines["options"] = json!(["Postgres", "SQLite\nwith WAL"]);
    let refused = tool(&e, &s, "ask", two_lines).await;
    assert_eq!(refused["error"]["code"], -32602, "{refused}");
    assert!(
        refused["error"]["message"]
            .as_str()
            .unwrap()
            .contains("one line"),
        "{refused}"
    );
    let proceed = text(&tool(&e, &s, "ask", question("workspace")).await);
    assert!(
        proceed.starts_with("Carry on with Postgres. Record what you build on it."),
        "{proceed}"
    );
    let park = text(&tool(&e, &s, "ask", question("outside")).await);
    assert!(
        park.starts_with("Finish what does not depend on this, checkpoint, then stop this task."),
        "{park}"
    );
    assert!(park.contains("rec:"), "names the question: {park}");
    tool(&e, &s, "ask", question("outside")).await;
    let refused = tool(&e, &s, "ask", question("outside")).await;
    assert_eq!(refused["result"]["isError"], true, "{refused}");
    assert!(
        refused["result"]["content"][0]["text"]
            .as_str()
            .unwrap()
            .contains("answer or withdraw one first"),
        "{refused}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn the_tool_list_stays_small() {
    // the guard counts Community tools only: an extension's tools are its own to pay for
    let e = env_with(|s| s.with_mcp_tools(vec![echo_tool()])).await;
    let s = connect(&e, "claude-code").await;
    let r = send(
        &e,
        Some(&s),
        json!({"jsonrpc": "2.0", "id": 1, "method": "tools/list"}),
    )
    .await;
    let community: Vec<&Value> = r.body["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|t| t["name"] != "ee_echo")
        .collect();
    let bytes = serde_json::to_string(&community).unwrap().len();
    // Every tool definition is sent to the model on every turn. Plan 8 added 160 bytes (about 40
    // tokens) to the 10,040 before it, Plan 9 112 (start_task's wait and its longer description),
    // propose mode 477 (`post_backlog`: before it an agent had no way to leave follow-up work for
    // another agent, or to propose optional work for a human to accept); a bigger tool list needs
    // a reason that pays for itself. `record`'s `supersedes` (85 bytes: how an agent corrects a
    // stored record that is wrong) was paid for inside the limit: `start_task` lost its empty
    // `required` list and `submit`'s inline records the receipt description `record` already has.
    // The inline records' own `supersedes` (31 bytes, so they list every field `record` takes)
    // was paid for by `records` losing a description that repeated the tool's (52 bytes).
    // `ask` (questions for a person, the owner's go-ahead of 2026-11-26) added 1,106 bytes to the
    // 10,779 before it, which raised the limit from 10,800 to 11,900: without it an agent that is
    // unsure can only guess or stop, and no answer reaches it.
    assert!(
        bytes <= 11_900,
        "tools/list is {bytes} bytes; it was 11,900 after `ask`"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn extension_tools_are_listed_and_called_like_community_ones() {
    let plain = env().await;
    let s = connect(&plain, "claude-code").await;
    let r = send(
        &plain,
        Some(&s),
        json!({"jsonrpc": "2.0", "id": 1, "method": "tools/list"}),
    )
    .await;
    let names = |r: &Reply| -> Vec<String> {
        r.body["result"]["tools"]
            .as_array()
            .unwrap()
            .iter()
            .map(|t| t["name"].as_str().unwrap().to_owned())
            .collect()
    };
    assert!(!names(&r).contains(&"ee_echo".to_owned()));
    let community = names(&r).len();

    let e = env_with(|s| s.with_mcp_tools(vec![echo_tool()])).await;
    let s = connect(&e, "claude-code").await;
    let r = send(
        &e,
        Some(&s),
        json!({"jsonrpc": "2.0", "id": 1, "method": "tools/list"}),
    )
    .await;
    assert_eq!(names(&r).len(), community + 1);
    assert!(names(&r).contains(&"ee_echo".to_owned()));

    let out = text(&tool(&e, &s, "ee_echo", json!({"text": "hi"})).await);
    assert!(out.ends_with("says hi"), "{out}");

    let refused = tool(&e, &s, "ee_echo", json!({"text": "boom"})).await;
    assert_eq!(refused["result"]["isError"], true, "{refused}");
    assert_eq!(refused["result"]["content"][0]["text"], "echo refused");

    let bad = tool(&e, &s, "ee_echo", json!({})).await;
    assert!(bad["error"]["message"].is_string(), "{bad}");

    // and Community tools still work beside it
    let unknown = tool(&e, &s, "no_such_tool", json!({})).await;
    assert!(
        unknown["error"]["message"]
            .as_str()
            .unwrap()
            .contains("unknown tool")
    );
}

/// A clash with a Community tool is a bug in the extension, refused when the tool is registered.
#[tokio::test(flavor = "multi_thread")]
#[should_panic(expected = "MCP tool name already taken: start_task")]
async fn an_extension_tool_cannot_take_a_community_tool_name() {
    let tool = margincode_daemon::ExtTool::new(
        "start_task",
        "not allowed",
        json!({"type": "object"}),
        std::sync::Arc::new(Echo),
    );
    env_with(|s| s.with_mcp_tools(vec![tool])).await;
}

fn record_count(status: &str) -> i64 {
    serde_json::from_str::<Value>(status).unwrap()["record_count"]
        .as_i64()
        .unwrap()
}

/// A subagent that starts its own task shares its parent's MCP session (checked with Claude
/// Code). The session must not then send the parent's writes to the subagent's task: with two
/// open tasks, a call without `task_id` asks which one.
#[tokio::test(flavor = "multi_thread")]
async fn a_session_with_two_open_tasks_asks_which_one() {
    let e = env().await;
    let s = connect(&e, "claude-code").await;
    for title in ["First job", "Second job"] {
        let out = text(
            &tool(
                &e,
                &s,
                "start_task",
                json!({"repo": "demo", "title": title, "goal": "Make hello louder"}),
            )
            .await,
        );
        assert!(out.contains("workspace:"), "{out}");
    }
    let note = json!({"kind": "note", "summary": "hello is fine"});
    let r = tool(&e, &s, "record", note.clone()).await;
    assert_eq!(r["error"]["code"], -32602, "{r}");
    let msg = r["error"]["message"].as_str().unwrap();
    assert!(
        msg.contains("demo/first-job") && msg.contains("demo/second-job"),
        "{msg}"
    );
    assert!(msg.contains("task_id"), "{msg}");

    let count = |id: &'static str| {
        let (e, s) = (&e, &s);
        async move { record_count(&text(&tool(e, s, "status", json!({"task_id": id})).await)) }
    };
    let first = count("first-job").await;
    let mut named = note.clone();
    named["task_id"] = json!("first-job");
    let r = tool(&e, &s, "record", named).await;
    assert_eq!(r["result"]["isError"], false, "{r}");
    assert_eq!(count("first-job").await, first + 1);

    let r = tool(&e, &s, "abandon", json!({"task_id": "second-job"})).await;
    assert_eq!(r["result"]["isError"], false, "{r}");
    let second = count("second-job").await;
    let r = tool(&e, &s, "record", note).await;
    assert_eq!(r["result"]["isError"], false, "{r}");
    assert_eq!(count("first-job").await, first + 2);
    assert_eq!(count("second-job").await, second);
}

#[tokio::test(flavor = "multi_thread")]
async fn abandon_says_when_the_workspace_was_removed() {
    let e = env_with(|s| s.with_ws_retain(Some(0))).await;
    let s = connect(&e, "claude-code").await;
    text(
        &tool(
            &e,
            &s,
            "start_task",
            json!({"repo": "demo", "title": "Never mind", "goal": "Nothing"}),
        )
        .await,
    );
    let abandoned = text(&tool(&e, &s, "abandon", json!({})).await);
    assert_eq!(abandoned, "task abandoned; its workspace was removed");
}

// Claude Code resets a tool call's socket 60 s after sending it, sends `notifications/cancelled`
// only when MCP_TOOL_TIMEOUT is shorter than that, and retries about 1 s after giving up with the
// same `Mcp-Session-Id`, while the original start may still be running.

async fn paused_env() -> (Env, std::sync::Arc<margincode_daemon::StartPackPause>) {
    let pause = std::sync::Arc::new(margincode_daemon::StartPackPause {
        ready: tokio::sync::Notify::new(),
        proceed: tokio::sync::Notify::new(),
        armed: std::sync::atomic::AtomicBool::new(false),
    });
    let p = pause.clone();
    let e = env_with(move |s| s.with_extension(p)).await;
    put_profile(&e, "claude", json!({"best": ["code"], "cost_rank": 1})).await;
    (e, pause)
}

fn arm(pause: &margincode_daemon::StartPackPause) {
    pause.armed.store(true, std::sync::atomic::Ordering::SeqCst);
}

/// Lets a paused start go on; disarmed first, so a second start (a bug) does not wait forever.
fn release(pause: &margincode_daemon::StartPackPause) {
    pause
        .armed
        .store(false, std::sync::atomic::Ordering::SeqCst);
    pause.proceed.notify_waiters();
}

/// `tools/call` of `name` under JSON-RPC id `id`, on a task of its own (so a test can cancel or
/// drop it while it runs); the reply body.
fn call_with_id(
    e: &Env,
    session: &str,
    id: i64,
    name: &str,
    args: Value,
) -> tokio::task::JoinHandle<Value> {
    let app = e.app.clone();
    let session = session.to_owned();
    let msg = json!({"jsonrpc": "2.0", "id": id, "method": "tools/call",
                     "params": {"name": name, "arguments": args}});
    tokio::spawn(async move {
        let req = Request::builder()
            .method("POST")
            .uri("/mcp")
            .header("content-type", "application/json")
            .header("accept", "application/json, text/event-stream")
            .header("mcp-session-id", session)
            .body(Body::from(msg.to_string()))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(resp.into_body(), 64 * 1024 * 1024)
            .await
            .unwrap();
        serde_json::from_slice(&bytes).unwrap()
    })
}

async fn started(e: &Env, session: &str, id: i64, args: Value) -> String {
    text(
        &call_with_id(e, session, id, "start_task", args)
            .await
            .unwrap(),
    )
}

async fn cancel(e: &Env, session: &str, id: i64) {
    let r = send(
        e,
        Some(session),
        json!({"jsonrpc": "2.0", "method": "notifications/cancelled",
               "params": {"requestId": id, "reason": "SdkError: Request timed out"}}),
    )
    .await;
    assert_eq!(r.status, StatusCode::ACCEPTED);
}

async fn open_tasks(e: &Env) -> Vec<Value> {
    let (status, out) = call(&e.app, "GET", "/v1/tasks?status=open", None).await;
    assert_eq!(status, StatusCode::OK, "{out}");
    out["tasks"].as_array().unwrap().clone()
}

#[tokio::test(flavor = "multi_thread")]
async fn a_start_cancelled_in_flight_is_handed_to_the_retry_once_it_is_done() {
    let (e, pause) = paused_env().await;
    post_backlog(&e, "louder hello", &["code"], 0).await;
    let s = connect(&e, "claude").await;
    arm(&pause);
    let ready = pause.ready.notified();
    let first = call_with_id(&e, &s, 11, "start_task", json!({}));
    ready.await;
    cancel(&e, &s, 11).await;

    let retry = call_with_id(&e, &s, 12, "start_task", json!({}));
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    assert!(
        !retry.is_finished(),
        "the retry waits for the start in flight"
    );
    release(&pause);

    let first = text(&first.await.unwrap());
    let retry = retry.await.unwrap();
    assert_eq!(retry["id"], 12);
    assert_eq!(
        text(&retry),
        first,
        "the retry gets the answer the client never read"
    );
    assert!(first.contains("workspace:"), "{first}");
    assert_eq!(open_tasks(&e).await.len(), 1);
}

#[tokio::test(flavor = "multi_thread")]
async fn a_start_cancelled_after_its_answer_was_written_is_handed_to_the_retry() {
    let (e, _) = paused_env().await;
    post_backlog(&e, "louder hello", &["code"], 0).await;
    let s = connect(&e, "claude").await;
    let first = started(&e, &s, 21, json!({})).await;
    cancel(&e, &s, 21).await;
    assert_eq!(started(&e, &s, 22, json!({})).await, first);
    assert_eq!(open_tasks(&e).await.len(), 1);
}

/// A hand-back is tracked like a start: the probe's own retry was cancelled too.
#[tokio::test(flavor = "multi_thread")]
async fn a_retry_that_is_cancelled_too_leaves_the_task_to_the_next_retry() {
    let (e, pause) = paused_env().await;
    post_backlog(&e, "louder hello", &["code"], 0).await;
    let s = connect(&e, "claude").await;
    arm(&pause);
    let ready = pause.ready.notified();
    let first = call_with_id(&e, &s, 31, "start_task", json!({}));
    ready.await;
    cancel(&e, &s, 31).await;
    let second = call_with_id(&e, &s, 32, "start_task", json!({}));
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    cancel(&e, &s, 32).await;
    release(&pause);
    let first = text(&first.await.unwrap());
    second.await.unwrap();

    assert_eq!(started(&e, &s, 33, json!({})).await, first);
    assert_eq!(open_tasks(&e).await.len(), 1);
}

/// A client that closes its socket drops the call, and its start stops as before: a start
/// still waiting for work takes nothing once its client is gone.
#[tokio::test(flavor = "multi_thread")]
async fn a_waiting_start_whose_socket_closed_takes_nothing() {
    let (e, _) = paused_env().await;
    let s = connect(&e, "claude").await;
    let first = call_with_id(&e, &s, 41, "start_task", json!({"wait": 30}));
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    assert!(
        !first.is_finished(),
        "nothing is queued, so the start waits"
    );
    first.abort();
    let _ = first.await;

    post_backlog(&e, "louder hello", &["code"], 0).await;
    tokio::time::sleep(TAKE_WINDOW).await;
    assert!(
        open_tasks(&e).await.is_empty(),
        "the gone call took the item"
    );
    let next = started(&e, &s, 42, json!({})).await;
    assert!(next.contains("workspace:"), "{next}");
}

/// Claude Code resets a cancelled call's socket 60 s after it sent the call, while the start
/// may still run: a start that a retry took over goes on for the retry.
#[tokio::test(flavor = "multi_thread")]
async fn a_start_taken_over_by_a_retry_goes_on_when_its_socket_closes() {
    let (e, pause) = paused_env().await;
    post_backlog(&e, "louder hello", &["code"], 0).await;
    let s = connect(&e, "claude").await;
    arm(&pause);
    let ready = pause.ready.notified();
    let first = call_with_id(&e, &s, 91, "start_task", json!({}));
    ready.await;
    cancel(&e, &s, 91).await;
    let retry = call_with_id(&e, &s, 92, "start_task", json!({}));
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    first.abort();
    let _ = first.await;
    release(&pause);

    let retry = text(&retry.await.unwrap());
    assert!(retry.contains("workspace:"), "{retry}");
    assert_eq!(open_tasks(&e).await.len(), 1);
}

/// At Claude Code's default 60 s timeout the socket reset comes with the cancel, before any
/// retry: a start that is past waiting goes on and is the retry's.
#[tokio::test(flavor = "multi_thread")]
async fn a_start_whose_socket_closed_before_any_retry_is_handed_to_the_retry() {
    let (e, pause) = paused_env().await;
    post_backlog(&e, "louder hello", &["code"], 0).await;
    let s = connect(&e, "claude").await;
    arm(&pause);
    let ready = pause.ready.notified();
    let first = call_with_id(&e, &s, 95, "start_task", json!({}));
    ready.await;
    first.abort(); // no cancel: Claude Code sends none at its default timeout
    let _ = first.await;

    let retry = call_with_id(&e, &s, 96, "start_task", json!({}));
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    assert!(
        !retry.is_finished(),
        "the retry waits for the start in flight"
    );
    release(&pause);
    let retry = text(&retry.await.unwrap());
    assert!(retry.contains("workspace:"), "{retry}");
    assert_eq!(open_tasks(&e).await.len(), 1);
}

/// How long the tests below give a start that still waits to take an item posted meanwhile.
const TAKE_WINDOW: std::time::Duration = std::time::Duration::from_secs(2);

/// The control for the tests below: a start that waits takes an item posted meanwhile.
#[tokio::test(flavor = "multi_thread")]
async fn a_waiting_start_takes_an_item_posted_while_it_waits() {
    let (e, _) = paused_env().await;
    let s = connect(&e, "claude").await;
    let first = call_with_id(&e, &s, 97, "start_task", json!({"wait": 30}));
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    post_backlog(&e, "louder hello", &["code"], 0).await;
    eventually("the waiting start to take the item", async || {
        open_tasks(&e).await.len() == 1
    })
    .await;
    let got = first.await.unwrap();
    assert!(text(&got).contains("workspace:"), "{got}");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_waiting_start_of_a_deleted_session_takes_nothing() {
    let (e, _) = paused_env().await;
    let s = connect(&e, "claude").await;
    let _first = call_with_id(&e, &s, 98, "start_task", json!({"wait": 30}));
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    let req = Request::builder()
        .method("DELETE")
        .uri("/mcp")
        .header("mcp-session-id", s.as_str())
        .body(Body::empty())
        .unwrap();
    let resp = e.app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    post_backlog(&e, "louder hello", &["code"], 0).await;
    tokio::time::sleep(TAKE_WINDOW).await;
    assert!(
        open_tasks(&e).await.is_empty(),
        "a gone session took the item"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn a_waiting_start_whose_retry_closed_too_takes_nothing() {
    let (e, _) = paused_env().await;
    let s = connect(&e, "claude").await;
    let first = call_with_id(&e, &s, 93, "start_task", json!({"wait": 30}));
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    cancel(&e, &s, 93).await;
    let retry = call_with_id(&e, &s, 94, "start_task", json!({"wait": 30}));
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    first.abort();
    let _ = first.await;
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    assert!(
        !retry.is_finished(),
        "the retry waits for the start it holds"
    );
    retry.abort();
    let _ = retry.await;

    post_backlog(&e, "louder hello", &["code"], 0).await;
    tokio::time::sleep(TAKE_WINDOW).await;
    assert!(open_tasks(&e).await.is_empty(), "a gone call took the item");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_cancelled_start_is_handed_only_to_a_retry_with_the_same_repo() {
    let (e, _) = paused_env().await;
    post_backlog(&e, "louder hello", &["code"], 0).await;
    let s = connect(&e, "claude").await;
    let first = started(&e, &s, 81, json!({"repo": "demo"})).await;
    assert!(first.contains("workspace:"), "{first}");
    cancel(&e, &s, 81).await;
    assert_ne!(started(&e, &s, 82, json!({})).await, first);
    assert_eq!(started(&e, &s, 83, json!({"repo": "demo"})).await, first);
    assert_eq!(open_tasks(&e).await.len(), 1);
}

/// A start the client read is never handed back: the session's next start is other work, and
/// two calls at once (subagents share their parent's session) get a task each.
#[tokio::test(flavor = "multi_thread")]
async fn a_delivered_start_is_not_handed_back() {
    let (e, _) = paused_env().await;
    for title in ["louder hello", "quieter hello", "plain hello", "warm hello"] {
        post_backlog(&e, title, &["code"], 0).await;
    }
    let s = connect(&e, "claude").await;
    let one = started(&e, &s, 51, json!({})).await;
    let two = started(&e, &s, 52, json!({})).await;
    assert_ne!(one, two);
    let (a, b) = (
        call_with_id(&e, &s, 53, "start_task", json!({})),
        call_with_id(&e, &s, 54, "start_task", json!({})),
    );
    let (a, b) = (text(&a.await.unwrap()), text(&b.await.unwrap()));
    assert!(
        a.contains("workspace:") && b.contains("workspace:"),
        "{a}\n{b}"
    );
    assert_ne!(a, b);
    assert_eq!(open_tasks(&e).await.len(), 4);
}

/// An explicit start is handed back only to a retry that asks for the same task, so a retry
/// makes no `slug-2`; another title is a new task.
#[tokio::test(flavor = "multi_thread")]
async fn a_cancelled_explicit_start_is_handed_only_to_the_same_ask() {
    let (e, _) = paused_env().await;
    let s = connect(&e, "claude").await;
    let ask = json!({"repo": "demo", "title": "Louder hello", "goal": "Make hello louder"});
    let first = started(&e, &s, 61, ask.clone()).await;
    cancel(&e, &s, 61).await;
    let other = json!({"repo": "demo", "title": "Quieter hello", "goal": "Make hello quieter"});
    assert_ne!(started(&e, &s, 62, other).await, first);
    assert_eq!(started(&e, &s, 63, ask).await, first);
    let slugs: Vec<String> = open_tasks(&e)
        .await
        .iter()
        .map(|t| t["slug"].as_str().unwrap().to_owned())
        .collect();
    assert_eq!(slugs.len(), 2, "{slugs:?}");
    assert!(!slugs.iter().any(|s| s.ends_with("-2")), "{slugs:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn only_a_start_that_gave_a_task_still_open_is_handed_back() {
    let (e, _) = paused_env().await;
    let s = connect(&e, "claude").await;
    let none = started(&e, &s, 71, json!({})).await;
    assert!(none.starts_with("No task is waiting"), "{none}");
    cancel(&e, &s, 71).await;
    post_backlog(&e, "louder hello", &["code"], 0).await;
    let got = started(&e, &s, 72, json!({})).await;
    assert!(got.contains("workspace:"), "{got}");

    cancel(&e, &s, 72).await;
    let task = open_tasks(&e).await[0]["id"].as_str().unwrap().to_owned();
    op(&e, &task, "abandon", json!({"agent": "claude"})).await;
    let after = started(&e, &s, 73, json!({})).await;
    assert!(after.starts_with("No task is waiting"), "{after}");
}

// F14: the daemon writes `.av-task` into a workspace it hands out, so hooks and `av` commands in
// a workspace started over MCP find their task (only the CLI wrote it before).

#[tokio::test(flavor = "multi_thread")]
async fn start_task_over_mcp_writes_the_task_file() {
    let e = env().await;
    let s = connect(&e, "claude-code").await;
    let said = text(
        &tool(
            &e,
            &s,
            "start_task",
            json!({"repo": "demo", "title": "Louder", "goal": "Make hello louder"}),
        )
        .await,
    );
    let line = said.lines().find(|l| l.starts_with("task: ")).unwrap();
    let (task, slug) = line["task: ".len()..].split_once(" (").unwrap();
    let t = json!({"task": {"id": task, "slug": slug.trim_end_matches(')')}});
    let tf = task_file(&e, &t);
    assert_eq!(tf["task"], task, "{tf}");
    assert_eq!(tf["session"], s.as_str(), "the MCP session: {tf}");
    assert_eq!(tf["generation"], 1, "{tf}");
    // Claude Code's hooks act as `agent`, not as its MCP name: a file naming the owner would
    // fence the owner's own writes from its workspace
    assert!(tf.get("agent").is_none(), "names no agent: {tf}");
}

// The daemon names the owner in `.av-task` only when the agent named itself (`X-Av-Agent`), with its
// profile's aliases, so a hook acting under that name or an alias is not fenced from its own
// workspace. A session named only by its client program gets a file with no name.

async fn connect_named(e: &Env, name: &str) -> String {
    let (status, sid, body) = initialize_with(e, ("x-av-agent", name)).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let sid = sid.expect("initialize returns Mcp-Session-Id");
    let n = send(
        e,
        Some(&sid),
        json!({"jsonrpc": "2.0", "method": "notifications/initialized"}),
    )
    .await;
    assert_eq!(n.status, StatusCode::ACCEPTED);
    sid
}

fn started_task(said: &str) -> Value {
    let line = said.lines().find(|l| l.starts_with("task: ")).unwrap();
    let (task, slug) = line["task: ".len()..].split_once(" (").unwrap();
    json!({"task": {"id": task, "slug": slug.trim_end_matches(')')}})
}

async fn started_and_handed_off(e: &Env, session: &str, owner: &str) -> Value {
    let said = text(
        &tool(
            e,
            session,
            "start_task",
            json!({"repo": "demo", "title": "Louder", "goal": "Make hello louder"}),
        )
        .await,
    );
    let t = started_task(&said);
    edit_greeter(e, &t);
    record_decision(e, &t).await;
    assert_eq!(handoff(e, &t, owner).await.0, StatusCode::OK);
    t
}

#[tokio::test(flavor = "multi_thread")]
async fn a_session_named_by_header_gets_its_name_in_the_task_file() {
    let e = env().await;
    put_profile(
        &e,
        "claude-opus",
        json!({"best": ["code"], "aliases": ["opus"]}),
    )
    .await;
    let s = connect_named(&e, "opus").await;
    let t = started_and_handed_off(&e, &s, "claude-opus").await;
    let tf = task_file(&e, &t);
    assert_eq!(tf["agent"], "claude-opus", "{tf}");
    assert_eq!(tf["aliases"], json!(["opus"]), "{tf}");
    assert_eq!(tf["generation"], 1, "{tf}");

    let g = connect_named(&e, "grok").await;
    let claimed = text(&tool(&e, &g, "claim_task", json!({"task_id": t["task"]["id"]})).await);
    assert!(claimed.contains("generation: 2"), "{claimed}");
    let tf = task_file(&e, &t);
    assert_eq!(tf["agent"], "grok", "{tf}");
    assert_eq!(tf["generation"], 2, "{tf}");
    assert!(tf.get("aliases").is_none(), "{tf}");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_session_named_only_by_its_client_gets_none() {
    let e = env().await;
    let s = connect_named(&e, "grok").await;
    let t = started_and_handed_off(&e, &s, "grok").await;
    assert_eq!(task_file(&e, &t)["agent"], "grok");
    let c = connect(&e, "codex").await;
    let claimed = text(&tool(&e, &c, "claim_task", json!({"task_id": t["task"]["id"]})).await);
    assert!(claimed.contains("generation: 2"), "{claimed}");
    let tf = task_file(&e, &t);
    assert!(tf.get("agent").is_none(), "{tf}");
    assert_eq!(tf["generation"], 2, "{tf}");
}

/// A claim names its owner in the same write that hands out the new generation: a reader that saw
/// generation 2 saw the name too, so a named previous owner never reads it from a file that names
/// nobody and lands a write under it.
#[tokio::test(flavor = "multi_thread")]
async fn a_claims_file_is_named_in_the_write_that_hands_out_its_generation() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    let e = env().await;
    let s = connect_named(&e, "claude").await;
    let t = started_and_handed_off(&e, &s, "claude").await;
    let path = ws_dir(&e, &t).join(".av-task");
    let done = Arc::new(AtomicBool::new(false));
    let watcher = {
        let done = done.clone();
        std::thread::spawn(move || {
            let (mut seen, mut unnamed) = (0u32, Vec::new());
            while !done.load(Ordering::SeqCst) {
                let Ok(text) = std::fs::read_to_string(&path) else {
                    continue;
                };
                let Ok(tf) = serde_json::from_str::<Value>(&text) else {
                    continue;
                };
                if tf["generation"] == 2 {
                    seen += 1;
                    if tf["agent"] != "grok" {
                        unnamed.push(text);
                    }
                }
            }
            (seen, unnamed)
        })
    };
    let g = connect_named(&e, "grok").await;
    let claimed = text(&tool(&e, &g, "claim_task", json!({"task_id": t["task"]["id"]})).await);
    assert!(claimed.contains("generation: 2"), "{claimed}");
    // keep reading a moment after, as a hook would
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    done.store(true, Ordering::SeqCst);
    let (seen, unnamed) = watcher.join().unwrap();
    assert!(seen > 0, "the watcher never read generation 2");
    assert!(unnamed.is_empty(), "generation 2 named nobody: {unnamed:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn read_file_and_read_symbol_note_what_the_sessions_task_opened() {
    let e = env().await;
    let s = connect(&e, "claude-code").await;
    let reviewer = connect(&e, "reviewer").await;
    text(
        &tool(
            &e,
            &reviewer,
            "read_file",
            json!({"repo": "demo", "path": "app/greeter.py"}),
        )
        .await,
    );
    text(
        &tool(
            &e,
            &s,
            "start_task",
            json!({"repo": "demo", "title": "Improve greeting", "goal": "Make hello louder"}),
        )
        .await,
    );
    text(&tool(&e, &s, "read_file", json!({"path": "app/util.py"})).await);
    let pack = text(&tool(&e, &s, "get_context", json!({"budget": 3000})).await);
    let n: i64 = pack[pack.find("sym:").expect("a code item") + 4..]
        .chars()
        .take_while(char::is_ascii_digit)
        .collect::<String>()
        .parse()
        .unwrap();
    text(&tool(&e, &s, "read_symbol", json!({"id": n})).await);
    let (path,): (String,) = sqlx::query_as("SELECT path FROM symbols WHERE id = $1")
        .bind(n)
        .fetch_one(e.store.pool())
        .await
        .unwrap();
    let want = if path == "app/util.py" {
        vec![("app/util.py".to_owned(), 2, true)]
    } else {
        let mut w = vec![("app/util.py".to_owned(), 1, false), (path, 1, true)];
        w.sort();
        w
    };
    eventually("the reads to be noted", async || {
        let rows: Vec<(String, i32, bool)> =
            sqlx::query_as("SELECT r.path, r.reads, r.expanded FROM task_reads r ORDER BY r.path")
                .fetch_all(e.store.pool())
                .await
                .unwrap();
        rows == want
    })
    .await;
}
