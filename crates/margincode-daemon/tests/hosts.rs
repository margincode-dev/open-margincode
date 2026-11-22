//! Without `AV_TOKEN`, `/v1` and `/mcp` refuse a `Host` or `Origin` a page on another site could
//! send (DNS rebinding); with a token nothing changes.

use crate::common::*;

async fn get_as(e: &Env, headers: &[(&str, &str)]) -> (StatusCode, Value) {
    call_with(&e.app, "GET", "/v1/repos/demo/backup", None, headers).await
}

#[tokio::test(flavor = "multi_thread")]
async fn without_a_token_hosts_no_rebinding_page_can_send_pass() {
    let e = env().await;
    let (status, body) = get_as(&e, &[]).await;
    assert_eq!(status, StatusCode::OK, "no Host: {body}");
    for host in [
        "127.0.0.1:7420",
        "127.0.0.2",
        "localhost:7420",
        "LOCALHOST.",
        "app.localhost:7420",
        "[::1]:7420",
        "10.0.0.5:7420",
        "[fd00::1]:7420",
        "daemon:7420",
        "host.docker.internal:7420",
        "gateway.docker.internal",
    ] {
        let (status, body) = get_as(&e, &[("host", host)]).await;
        assert_eq!(status, StatusCode::OK, "{host}: {body}");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn without_a_token_a_dotted_dns_name_is_refused() {
    let e = env().await;
    for host in [
        "evil.example:7420",
        "rebind.attacker.com",
        "localhost.evil.com.",
    ] {
        let (status, body) = get_as(&e, &[("host", host)]).await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{host}: {body}");
        let error = body["error"].as_str().unwrap();
        assert!(
            error.contains("AV_TOKEN") && error.contains("AV_ALLOWED_HOSTS"),
            "{error}"
        );
    }
    let (status, body) = call_with(
        &e.app,
        "POST",
        "/mcp",
        Some(json!({"jsonrpc": "2.0", "id": 0, "method": "ping"})),
        &[("host", "evil.example:7420")],
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn without_a_token_only_a_loopback_allowed_or_same_origin_passes() {
    let e = env().await;
    for (host, origin) in [
        ("127.0.0.1:7420", "http://127.0.0.1:7420"),
        ("127.0.0.1:7420", "http://localhost:5173"),
        ("127.0.0.1:7420", "http://[::1]:7420"),
        ("daemon:7420", "http://daemon:7420"),
        ("10.0.0.5:7420", "http://10.0.0.5:7420"),
    ] {
        let (status, body) = get_as(&e, &[("host", host), ("origin", origin)]).await;
        assert_eq!(status, StatusCode::OK, "{host} from {origin}: {body}");
    }
    for origin in [
        "http://evil.example",
        "https://evil.example:7420",
        "null",
        "http://10.0.0.5:7420",
        "http://daemon:7420",
    ] {
        let (status, body) = get_as(&e, &[("host", "127.0.0.1:7420"), ("origin", origin)]).await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{origin}: {body}");
        let (status, _) = get_as(&e, &[("origin", origin)]).await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{origin} without a Host");
    }
    let (status, body) = call_with(
        &e.app,
        "POST",
        "/mcp",
        Some(json!({"jsonrpc": "2.0", "id": 0, "method": "ping"})),
        &[
            ("host", "127.0.0.1:7420"),
            ("origin", "http://evil.example"),
        ],
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn av_allowed_hosts_lets_a_named_host_and_its_origin_in() {
    let e = env_with(|s| {
        s.with_allowed_hosts(vec![
            " Daemon-Host.LAN:7420 ".into(),
            " ".into(),
            "[fd00::2]".into(),
        ])
    })
    .await;
    for (host, origin) in [
        ("daemon-host.lan.", "http://DAEMON-HOST.lan:9999"),
        ("127.0.0.1:7420", "http://daemon-host.lan"),
        ("127.0.0.1:7420", "http://[fd00::2]:7420"),
    ] {
        let (status, body) = get_as(&e, &[("host", host), ("origin", origin)]).await;
        assert_eq!(status, StatusCode::OK, "{host} from {origin}: {body}");
    }
    let (status, _) = get_as(&e, &[("host", "other.lan")]).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[tokio::test(flavor = "multi_thread")]
async fn the_ui_page_and_its_same_origin_calls_work_without_a_token() {
    use tower::ServiceExt as _;
    let e = env_with(|s| s.with_allowed_hosts(vec!["av.lan".into()])).await;
    for host in ["127.0.0.1:7420", "localhost:7420", "av.lan:7420"] {
        let page = Request::get("/ui")
            .header("host", host)
            .body(axum::body::Body::empty())
            .unwrap();
        let resp = e.app.clone().oneshot(page).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK, "{host}");
        let origin = format!("http://{host}");
        let (status, body) = call_with(
            &e.app,
            "GET",
            "/v1/tasks",
            None,
            &[("host", host), ("origin", &origin)],
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{host}: {body}");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn mcp_tools_reach_the_api_in_process_from_a_checked_request() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    let headers = [
        ("host", "localhost:7420"),
        ("origin", "http://localhost:7420"),
    ];
    let init = json!({"jsonrpc": "2.0", "id": 0, "method": "initialize", "params": {
        "protocolVersion": "2025-06-18", "capabilities": {},
        "clientInfo": {"name": "grok", "version": "1"}}});
    let mut req = Request::post("/mcp")
        .header("content-type", "application/json")
        .header("accept", "application/json, text/event-stream");
    for (k, v) in headers {
        req = req.header(k, v);
    }
    use tower::ServiceExt as _;
    let resp = e
        .app
        .clone()
        .oneshot(req.body(axum::body::Body::from(init.to_string())).unwrap())
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let session = resp.headers()["mcp-session-id"]
        .to_str()
        .unwrap()
        .to_owned();
    let in_session = [
        headers[0],
        headers[1],
        ("accept", "application/json, text/event-stream"),
        ("mcp-session-id", &session),
    ];
    let note = json!({"jsonrpc": "2.0", "method": "notifications/initialized"});
    let (status, _) = call_with(&e.app, "POST", "/mcp", Some(note), &in_session).await;
    assert_eq!(status, StatusCode::ACCEPTED);
    let id = t["task"]["id"].as_str().unwrap();
    let call = json!({"jsonrpc": "2.0", "id": 1, "method": "tools/call",
        "params": {"name": "status", "arguments": {"task_id": id}}});
    let (status, body) = call_with(&e.app, "POST", "/mcp", Some(call), &in_session).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["result"]["isError"], false, "{body}");
    assert!(
        body["result"]["content"][0]["text"]
            .as_str()
            .unwrap()
            .contains(id),
        "{body}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn with_a_token_host_and_origin_are_not_checked() {
    let e = env_with(|s| s.with_token(Some("sekrit".into()))).await;
    let (status, body) = get_as(
        &e,
        &[
            ("host", "evil.example"),
            ("origin", "http://evil.example"),
            ("authorization", "Bearer sekrit"),
        ],
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, _) = get_as(&e, &[("host", "evil.example")]).await;
    assert_eq!(
        status,
        StatusCode::UNAUTHORIZED,
        "the token is still asked for"
    );
}
