//! Plan 10 A3: the office (`GET /v1/office`) and agents' timelines
//! (`GET /v1/agents/{name}/timeline`), read by the dashboard.

use crate::common::*;

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

// ---- needs attention (`GET /v1/attention`, Plan 15 Phase 2) ----

/// Every threshold at zero: anything in a waiting status is stuck at once.
fn no_patience() -> scholion_daemon::AttentionLimits {
    scholion_daemon::AttentionLimits {
        review_secs: 0,
        approved_secs: 0,
        handoff_secs: 0,
        footprint_bytes: 1,
        footprint_secs: 3600,
    }
}

async fn attention(e: &Env, query: &str) -> Value {
    let (status, body) = call(&e.app, "GET", &format!("/v1/attention{query}"), None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert!(body["generated_at"].is_string(), "{body}");
    body
}

fn attn_kinds(body: &Value) -> Vec<&str> {
    body["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|i| i["kind"].as_str().unwrap())
        .collect()
}

fn item<'a>(body: &'a Value, kind: &str) -> &'a Value {
    body["items"]
        .as_array()
        .unwrap()
        .iter()
        .find(|i| i["kind"] == kind)
        .unwrap_or_else(|| panic!("no {kind} in {body}"))
}

/// `agent` starts a task, writes `text` to the greeter, records a decision and submits it.
async fn submitted_as(e: &Env, agent: &str, title: &str, text: &str) -> Value {
    let (status, t) = call(
        &e.app,
        "POST",
        "/v1/tasks",
        Some(
            json!({"repo": "demo", "title": title, "goal": "Make hello louder",
                    "agent": agent, "session": format!("s-{agent}")}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{t}");
    std::fs::write(ws_dir(e, &t).join("app/greeter.py"), text).unwrap();
    record_decision(e, &t).await;
    let (status, b) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/submit", id(&t)),
        Some(json!({"agent": agent})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{b}");
    t
}

/// grok validates and approves the task.
async fn approve_as_grok(e: &Env, t: &Value) {
    let task = id(t);
    let (status, b) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{task}/validate"),
        Some(json!({"agent": "grok", "command": "pytest", "exit_code": 0})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{b}");
    let (status, b) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{task}/review"),
        Some(json!({"reviewer": "grok", "verdict": "approve", "summary": "fine"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{b}");
}

async fn merge_as_grok(e: &Env, t: &Value) -> Value {
    let (status, b) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/merge", id(t)),
        Some(json!({"agent": "grok"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{b}");
    b
}

/// The feed time of the newest `kind` event of task `t`.
async fn event_at(e: &Env, t: &Value, kind: &str) -> String {
    let (status, b) = call(
        &e.app,
        "GET",
        "/v1/events?repo=demo&since=0&limit=1000",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{b}");
    b["events"]
        .as_array()
        .unwrap()
        .iter()
        .rev()
        .find(|ev| ev["kind"] == kind && ev["task_id"] == t["task"]["id"])
        .unwrap_or_else(|| panic!("no {kind} event for {t}: {b}"))["at"]
        .as_str()
        .unwrap()
        .to_owned()
}

#[tokio::test]
async fn attention_lists_each_stuck_situation_worst_first() {
    let e = env_with(|s| s.with_lease_secs(0).with_attention(no_patience())).await;
    put_agent(&e, "grok", 2).await;

    // conflicted: codex's change to the same lines merges first, claude's then conflicts
    let loud = "class Greeter:\n    def hello(self, name):\n        return name.upper()\n";
    let shout = "class Greeter:\n    def hello(self, name):\n        return name + \"!\"\n";
    let mine = submitted_as(&e, "claude", "Make hello loud", loud).await;
    let theirs = submitted_as(&e, "codex", "Make hello shout", shout).await;
    approve_as_grok(&e, &theirs).await;
    merge_as_grok(&e, &theirs).await;
    approve_as_grok(&e, &mine).await;
    let b = merge_as_grok(&e, &mine).await;
    assert_eq!(b["status"], "conflicted", "{b}");

    // main red: the post-merge check of main's newest commit failed
    let (status, log) = call(&e.app, "GET", "/v1/repos/demo/log?limit=1", None).await;
    assert_eq!(status, StatusCode::OK, "{log}");
    let main = log["commits"][0]["change"].as_str().unwrap().to_owned();
    let (status, b) = call(
        &e.app,
        "POST",
        "/v1/repos/demo/checks",
        Some(json!({"change": main, "command": "pytest", "exit_code": 1, "runner": "ci"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{b}");

    // a review nobody gave, then an approval nobody merged
    let twice = "class Greeter:\n    def hello(self, name):\n        return name * 2\n";
    let waiting = submitted_as(&e, "claude", "Wait for review", twice).await;
    let approved = submitted_as(&e, "claude", "Approved only", loud).await;
    approve_as_grok(&e, &approved).await;
    // a handoff nobody claimed
    let handed = start_task(&e, "Hand me off").await;
    edit_greeter(&e, &handed);
    record_decision(&e, &handed).await;
    let (status, b) = handoff(&e, &handed, "claude").await;
    assert_eq!(status, StatusCode::OK, "{b}");
    // an open task nobody wrote to for a whole lease (the lease is 0 s)
    let quiet = start_task(&e, "Nobody writes").await;
    // a backlog item no profile fits
    let (status, b) = call(
        &e.app,
        "POST",
        "/v1/backlog",
        Some(
            json!({"repo": "demo", "title": "Train the model", "goal": "train",
                    "needs": ["gpu"]}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{b}");
    // a target past its due day
    let (status, b) = call(
        &e.app,
        "POST",
        "/v1/targets",
        Some(json!({"repo": "demo", "title": "Release 1", "due": "2020-01-01"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{b}");
    // a session that left a big build outside any workspace
    let (status, b) = call(
        &e.app,
        "POST",
        "/v1/footprint",
        Some(
            json!({"session": "cc-1", "agent": "claude", "task": id(&quiet),
                    "added": [{"path": "/home/me/target-x", "bytes": 4096}]}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{b}");

    let a = attention(&e, "?repo=demo").await;
    assert_eq!(
        attn_kinds(&a),
        [
            "main_red",
            "conflicted",
            "stale_review",
            "approved_not_merged",
            "unclaimed_handoff",
            "quiet_lease",
            "unroutable_backlog",
            "overdue_target",
            "footprint_outside",
        ],
        "{a}"
    );
    let sev: Vec<i64> = a["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|i| i["severity"].as_i64().unwrap())
        .collect();
    assert_eq!(sev, [3, 3, 2, 2, 2, 2, 1, 1, 1], "{a}");
    assert!(
        a["items"]
            .as_array()
            .unwrap()
            .iter()
            .all(|i| i["repo"] == "demo"
                && i["since"].is_string()
                && i["detail"].is_string()
                && i["title"].is_string()),
        "{a}"
    );

    let red = item(&a, "main_red");
    assert_eq!(
        red["slug"], theirs["task"]["slug"],
        "the merge that went red: {red}"
    );
    let c = item(&a, "conflicted");
    assert_eq!(c["slug"], mine["task"]["slug"]);
    assert_eq!(c["agent"], "claude");
    assert_eq!(c["since"], event_at(&e, &mine, "conflicted").await.as_str());
    let r = item(&a, "stale_review");
    assert_eq!(r["task_id"], waiting["task"]["id"]);
    assert_eq!(r["since"], event_at(&e, &waiting, "submit").await.as_str());
    let ap = item(&a, "approved_not_merged");
    assert_eq!(ap["task_id"], approved["task"]["id"]);
    assert_eq!(
        ap["since"],
        event_at(&e, &approved, "approve").await.as_str()
    );
    let h = item(&a, "unclaimed_handoff");
    assert_eq!(h["slug"], handed["task"]["slug"]);
    assert_eq!(h["since"], event_at(&e, &handed, "handoff").await.as_str());
    let q = item(&a, "quiet_lease");
    assert_eq!(q["slug"], quiet["task"]["slug"]);
    assert_eq!(q["agent"], "claude");
    let u = item(&a, "unroutable_backlog");
    assert_eq!(u["title"], "Train the model");
    assert!(u["detail"].as_str().unwrap().contains("gpu"), "{u}");
    let o = item(&a, "overdue_target");
    assert_eq!(
        o["since"], "2020-01-02T00:00:00Z",
        "overdue from the end of its due day"
    );
    let f = item(&a, "footprint_outside");
    assert_eq!(f["agent"], "claude");
    assert_eq!(f["slug"], quiet["task"]["slug"]);

    // every repo when none is named; an unknown repo is a 404
    assert_eq!(attn_kinds(&attention(&e, "").await), attn_kinds(&a));
    let (status, _) = call(&e.app, "GET", "/v1/attention?repo=nope", None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn attention_lists_proposals_waiting_for_a_human() {
    // at the default thresholds: a proposal needs a human at once
    let e = env().await;
    let (status, p) = call(
        &e.app,
        "POST",
        "/v1/backlog",
        Some(
            json!({"repo": "demo", "title": "Cache packs", "goal": "g", "propose": true,
                    "posted_by": "claude", "why": "unsure it pays off"}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{p}");
    let a = attention(&e, "?repo=demo").await;
    assert_eq!(attn_kinds(&a), ["proposed_backlog"], "{a}");
    let it = item(&a, "proposed_backlog");
    assert_eq!(it["severity"], 1);
    assert_eq!(it["backlog_id"], p["id"]);
    assert_eq!(it["agent"], "claude");
    assert_eq!(it["title"], "Cache packs");
    assert_eq!(it["detail"], "proposed by claude: unsure it pays off");

    // accepted: no longer waiting for anyone
    let uri = format!("/v1/backlog/{}/accept", p["id"].as_str().unwrap());
    let (status, b) = call(&e.app, "POST", &uri, Some(json!({"by": "vinh"}))).await;
    assert_eq!(status, StatusCode::OK, "{b}");
    assert_eq!(attention(&e, "?repo=demo").await["items"], json!([]));
}

#[tokio::test]
async fn attention_waits_out_its_thresholds() {
    // the defaults: a review is stuck after 2 h, a lease runs 30 min, nobody has a profile
    let e = env().await;
    let t = start_task(&e, "Louder hello").await;
    edit_greeter(&e, &t);
    record_decision(&e, &t).await;
    let (status, b) = submit(&e, &t).await;
    assert_eq!(status, StatusCode::OK, "{b}");
    start_task(&e, "Still writing").await;
    // untagged work and no profiles anywhere: any agent takes it
    let (status, b) = call(
        &e.app,
        "POST",
        "/v1/backlog",
        Some(json!({"repo": "demo", "title": "Fix typos", "goal": "typos"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{b}");
    // a small footprint
    let (status, b) = call(
        &e.app,
        "POST",
        "/v1/footprint",
        Some(json!({"session": "cc-1", "agent": "claude",
                    "added": [{"path": "/tmp/x", "bytes": 4096}]})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{b}");
    let a = attention(&e, "?repo=demo").await;
    assert_eq!(a["items"], json!([]), "{a}");
}
