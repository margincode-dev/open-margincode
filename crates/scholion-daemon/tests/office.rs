//! Plan 10 A3: the office (`GET /v1/office`) and agents' timelines
//! (`GET /v1/agents/{name}/timeline`), read by the dashboard.

mod common;
use common::*;

async fn put_agent(e: &Env, name: &str, cost: i32) {
    let (status, body) = call(
        &e.app,
        "PUT",
        &format!("/v1/agents/{name}"),
        Some(json!({"best": ["code"], "cost_rank": cost})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
}

async fn timeline(e: &Env, agent: &str, query: &str) -> Vec<Value> {
    let (status, body) = call(
        &e.app,
        "GET",
        &format!("/v1/agents/{agent}/timeline{query}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["agent"], agent);
    body["items"].as_array().unwrap().clone()
}

fn kinds(items: &[Value]) -> Vec<&str> {
    items.iter().map(|i| i["kind"].as_str().unwrap()).collect()
}

async fn office(e: &Env) -> Value {
    let (status, body) = call(&e.app, "GET", "/v1/office", None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body
}

fn desk<'a>(o: &'a Value, name: &str) -> &'a Value {
    o["agents"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["name"] == name)
        .unwrap_or_else(|| panic!("{name} not in the office: {o}"))
}

#[tokio::test]
async fn a_timeline_lists_what_an_agent_did_newest_first() {
    let e = env().await;
    let t = start_task(&e, "Louder hello").await;
    edit_greeter(&e, &t);
    record_decision(&e, &t).await;
    let (status, body) = submit(&e, &t).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let task = id(&t);
    let (status, body) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{task}/validate"),
        Some(json!({"agent": "grok", "command": "pytest", "exit_code": 0})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{task}/review"),
        Some(json!({"reviewer": "grok", "verdict": "approve", "summary": "looks right"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{task}/merge"),
        Some(json!({"agent": "grok"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");

    let mine = timeline(&e, "claude", "").await;
    let k = kinds(&mine);
    for want in ["started", "intent", "decision", "submit"] {
        assert!(k.contains(&want), "{want} missing from {k:?}");
    }
    assert!(!k.contains(&"merged"), "grok merged it, not claude: {k:?}");
    // newest first: the submit came after the start
    let pos = |kind: &str| k.iter().position(|x| *x == kind).unwrap();
    assert!(pos("submit") < pos("started"), "{k:?}");
    let ats: Vec<&str> = mine.iter().map(|i| i["at"].as_str().unwrap()).collect();
    let mut sorted = ats.clone();
    sorted.sort_by(|a, b| b.cmp(a));
    assert_eq!(ats, sorted);
    // each entry names its task
    assert!(
        mine.iter()
            .all(|i| i["slug"] == t["task"]["slug"] && i["repo"] == "demo")
    );
    let decision = mine.iter().find(|i| i["kind"] == "decision").unwrap();
    assert_eq!(decision["text"], "hello shouts");

    let theirs = kinds(&timeline(&e, "grok", "").await).join(",");
    assert!(theirs.contains("merged"), "{theirs}");
    assert!(theirs.contains("approve"), "{theirs}");
    assert!(theirs.contains("validation"), "{theirs}");

    // limit and repo
    assert_eq!(timeline(&e, "claude", "?limit=2").await.len(), 2);
    assert!(timeline(&e, "claude", "?repo=other").await.is_empty());
    assert!(timeline(&e, "nobody", "").await.is_empty());
}

#[tokio::test]
async fn a_submit_after_a_claim_belongs_to_the_claimer() {
    let e = env().await;
    let t = start_task(&e, "Louder hello").await;
    edit_greeter(&e, &t);
    record_decision(&e, &t).await;
    let (status, body) = handoff(&e, &t, "claude").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = claim(&e, &t, "codex").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/submit", id(&t)),
        Some(json!({"agent": "codex"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");

    let claude = kinds(&timeline(&e, "claude", "").await).join(",");
    assert!(claude.contains("handoff"), "{claude}");
    assert!(!claude.contains("submit"), "{claude}");
    let codex = kinds(&timeline(&e, "codex", "").await).join(",");
    assert!(codex.contains("claim"), "{codex}");
    assert!(codex.contains("submit"), "{codex}");
}

#[tokio::test]
async fn the_office_shows_each_agent_at_work_or_idle() {
    let e = env().await;
    put_agent(&e, "grok", 2).await;
    put_agent(&e, "claude", 4).await;
    let t = start_task(&e, "Louder hello").await;
    record_decision(&e, &t).await;

    let o = office(&e).await;
    let names: Vec<&str> = o["agents"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a["name"].as_str().unwrap())
        .collect();
    assert_eq!(names, ["grok", "claude"], "profiles first, cheapest first");
    let claude = desk(&o, "claude");
    assert_eq!(claude["state"], "working", "{claude}");
    assert_eq!(claude["open"][0]["slug"], t["task"]["slug"]);
    assert_eq!(claude["last"]["kind"], "decision", "{claude}");
    assert_eq!(claude["profile"]["cost_rank"], 4);
    let grok = desk(&o, "grok");
    assert_eq!(grok["state"], "idle", "{grok}");
    assert!(grok["last"].is_null());

    // submitted work waits for review: the owner is idle with it in `in_review`
    edit_greeter(&e, &t);
    let (status, body) = submit(&e, &t).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let o = office(&e).await;
    let claude = desk(&o, "claude");
    assert_eq!(claude["state"], "idle", "{claude}");
    assert_eq!(claude["in_review"][0]["slug"], t["task"]["slug"]);

    // `next` reserves the review for grok: grok is reviewing it
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/backlog/next",
        Some(json!({"agent": "grok", "session": "s-grok"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["from"], "review", "{body}");
    let o = office(&e).await;
    let grok = desk(&o, "grok");
    assert_eq!(grok["state"], "reviewing", "{grok}");
    assert_eq!(grok["reviewing"][0]["slug"], t["task"]["slug"]);
}

#[tokio::test]
async fn an_agent_without_a_profile_shows_and_a_task_past_its_lease_is_quiet() {
    let e = env_with(|s| s.with_lease_secs(0)).await;
    put_agent(&e, "grok", 2).await;
    start_task(&e, "Louder hello").await; // claude has no profile
    let o = office(&e).await;
    let names: Vec<&str> = o["agents"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a["name"].as_str().unwrap())
        .collect();
    assert_eq!(names, ["grok", "claude"]);
    let claude = desk(&o, "claude");
    assert!(claude["profile"].is_null());
    assert_eq!(claude["state"], "quiet", "{claude}");
}
