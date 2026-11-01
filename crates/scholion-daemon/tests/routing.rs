//! Plan 8 Task 4: agent profiles, the backlog and `next` over the HTTP API.

mod common;
use common::*;

async fn put_agent(e: &Env, name: &str, best: &[&str], can: &[&str], cost: i32) {
    let (status, body) = call(
        &e.app,
        "PUT",
        &format!("/v1/agents/{name}"),
        Some(json!({"best": best, "can": can, "cost_rank": cost})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
}

async fn post_item(e: &Env, repo: &str, title: &str, needs: &[&str], reserve: i64) -> Value {
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/backlog",
        Some(
            json!({"repo": repo, "title": title, "goal": "Make hello louder",
                    "needs": needs, "reserve_secs": reserve}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body
}

async fn next(e: &Env, agent: &str) -> (StatusCode, Value) {
    call(
        &e.app,
        "POST",
        "/v1/backlog/next",
        Some(json!({"agent": agent, "session": "s-next"})),
    )
    .await
}

async fn backlog(e: &Env, query: &str) -> Vec<Value> {
    let (status, body) = call(&e.app, "GET", &format!("/v1/backlog{query}"), None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body["items"].as_array().unwrap().clone()
}

#[tokio::test]
async fn profiles_are_set_and_listed_cheapest_first() {
    let e = env().await;
    put_agent(&e, "claude", &["code"], &["docs"], 4).await;
    put_agent(&e, "muse", &[], &["code"], 1).await;
    let (status, body) = call(&e.app, "GET", "/v1/agents", None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let names: Vec<&str> = body["agents"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a["name"].as_str().unwrap())
        .collect();
    assert_eq!(names, ["muse", "claude"]);
}

#[tokio::test]
async fn a_blank_tag_in_a_profile_is_a_400() {
    let e = env().await;
    let (status, _) = call(
        &e.app,
        "PUT",
        "/v1/agents/grok",
        Some(json!({"best": [" "], "cost_rank": 2})),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn backlog_items_are_posted_listed_and_dropped() {
    let e = env().await;
    let a = post_item(&e, "demo", "first", &["Image"], 600).await;
    post_item(&e, "demo", "second", &[], 600).await;
    assert_eq!(a["state"], "queued");
    assert_eq!(a["needs"], json!(["image"]));
    assert_eq!(backlog(&e, "").await.len(), 2);
    assert_eq!(backlog(&e, "?repo=other").await.len(), 0);

    let id = a["id"].as_str().unwrap();
    let (status, _) = call(&e.app, "DELETE", &format!("/v1/backlog/{id}"), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(backlog(&e, "?state=queued").await.len(), 1);
    assert_eq!(backlog(&e, "?state=dropped").await.len(), 1);
    // dropping twice, or an unknown id
    let (status, _) = call(&e.app, "DELETE", &format!("/v1/backlog/{id}"), None).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    let (status, _) = call(
        &e.app,
        "DELETE",
        "/v1/backlog/00000000-0000-0000-0000-000000000000",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn next_with_nothing_queued_says_so() {
    let e = env().await;
    let (status, body) = next(&e, "claude").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body, json!({"task": null, "queued": 0}));
}

#[tokio::test]
async fn next_gives_an_agent_the_item_that_fits_it() {
    let e = env().await;
    put_agent(&e, "muse", &[], &["code"], 1).await;
    put_agent(&e, "claude", &["code"], &[], 4).await;
    let item = post_item(&e, "demo", "louder hello", &["code"], 0).await;

    // reserve 0: both can take it, but claude ranks first (best at code)
    let (status, body) = next(&e, "claude").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["from"], "backlog");
    assert_eq!(body["task"]["owner_agent"], "claude");
    assert_eq!(body["task"]["title"], "louder hello");
    assert!(body["pack"].is_object(), "{body}");
    assert!(ws_dir(&e, &body).is_dir());

    let items = backlog(&e, "").await;
    assert_eq!(items[0]["state"], "taken");
    assert_eq!(items[0]["taken_by"], "claude");
    assert_eq!(items[0]["task_id"], body["task"]["id"]);
    assert_eq!(items[0]["id"], item["id"]);

    // nothing is left for the next agent
    let (_, again) = next(&e, "muse").await;
    assert_eq!(again, json!({"task": null, "queued": 0}));
}

#[tokio::test]
async fn inside_the_reserve_window_only_the_top_agent_takes_an_item() {
    let e = env().await;
    put_agent(&e, "muse", &[], &["code"], 1).await;
    put_agent(&e, "claude", &["code"], &[], 4).await;
    post_item(&e, "demo", "untagged", &[], 600).await;
    // untagged work: muse is the cheapest, so claude has to wait
    let (_, body) = next(&e, "claude").await;
    assert_eq!(body, json!({"task": null, "queued": 1}));
    let (_, body) = next(&e, "muse").await;
    assert_eq!(body["task"]["owner_agent"], "muse");
}

#[tokio::test]
async fn an_item_nobody_can_do_stays_queued() {
    let e = env().await;
    put_agent(&e, "muse", &[], &["code"], 1).await;
    put_agent(&e, "claude", &["code"], &[], 4).await;
    post_item(&e, "demo", "draw it", &["image"], 0).await;
    for agent in ["muse", "claude"] {
        let (_, body) = next(&e, agent).await;
        assert_eq!(body, json!({"task": null, "queued": 1}), "{agent}");
    }
    assert_eq!(backlog(&e, "?state=queued").await.len(), 1);
}

#[tokio::test]
async fn an_item_that_cannot_start_is_requeued_and_does_not_block_the_others() {
    let e = env().await;
    put_agent(&e, "muse", &[], &["code"], 1).await;
    // older, so it is tried first, but the repo does not exist
    post_item(&e, "no-such-repo", "broken", &[], 0).await;
    post_item(&e, "demo", "fine", &[], 0).await;
    let (status, body) = next(&e, "muse").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["task"]["title"], "fine");
    let queued = backlog(&e, "?state=queued").await;
    assert_eq!(queued.len(), 1);
    assert_eq!(queued[0]["title"], "broken");
    assert_eq!(queued[0]["taken_by"], Value::Null);
}

#[tokio::test]
async fn an_alias_resolves_to_the_profile_name_that_owns_the_task() {
    let e = env().await;
    let (status, body) = call(
        &e.app,
        "PUT",
        "/v1/agents/grok",
        Some(json!({"best": ["image"], "can": [], "cost_rank": 2,
                    "aliases": ["grok-shell-av"]})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    post_item(&e, "demo", "look at it", &["image"], 600).await;
    let (_, body) = next(&e, "grok-shell-av").await;
    assert_eq!(body["task"]["owner_agent"], "grok");
}

async fn hand_off_with(e: &Env, t: &Value, extra: Value) -> (StatusCode, Value) {
    let mut b = json!({"agent": "claude", "done": "hello shouts", "remaining": "draw the logo",
                       "broken": "", "questions": ""});
    for (k, v) in extra.as_object().unwrap() {
        b[k] = v.clone();
    }
    call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/handoff", id(t)),
        Some(b),
    )
    .await
}

#[tokio::test]
async fn a_handoff_with_tags_goes_to_the_agent_that_fits() {
    let e = env().await;
    put_agent(&e, "muse", &[], &["code"], 1).await;
    put_agent(&e, "grok", &["image"], &[], 2).await;
    let t = start_task(&e, "Make hello louder").await;
    edit_greeter(&e, &t);
    record_decision(&e, &t).await;
    let (status, body) = hand_off_with(&e, &t, json!({"needs": ["Image"]})).await;
    assert_eq!(status, StatusCode::OK, "{body}");

    // muse cannot do image work: the handed-off task is not offered to it
    let (_, body) = next(&e, "muse").await;
    assert_eq!(body["task"], Value::Null);

    let (status, body) = next(&e, "grok").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["from"], "handoff");
    assert_eq!(body["task"]["owner_agent"], "grok");
    assert_eq!(body["task"]["id"], t["task"]["id"]);
    assert_eq!(body["generation"], 2);
    assert!(body["bundle"].is_object(), "{body}");

    // claimed: the tags are gone, nothing is left to pull
    let (_, again) = next(&e, "grok").await;
    assert_eq!(again["task"], Value::Null);
}

#[tokio::test]
async fn a_handoff_with_a_blank_tag_is_refused_and_leaves_the_task_open() {
    let e = env().await;
    let t = start_task(&e, "Make hello louder").await;
    edit_greeter(&e, &t);
    record_decision(&e, &t).await;
    let (status, body) = hand_off_with(&e, &t, json!({"needs": [" "]})).await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
    let (_, task) = call(&e.app, "GET", &format!("/v1/tasks/{}", id(&t)), None).await;
    assert_eq!(task["task"]["status"], "open", "{task}");
}

#[tokio::test]
async fn an_untagged_handoff_is_offered_to_another_agent() {
    let e = env().await;
    put_agent(&e, "muse", &[], &["code"], 1).await;
    put_agent(&e, "claude", &["code"], &[], 4).await;
    let t = start_task(&e, "Make hello louder").await;
    edit_greeter(&e, &t);
    record_decision(&e, &t).await;
    let (status, _) = handoff(&e, &t, "claude").await;
    assert_eq!(status, StatusCode::OK);
    // claude handed it off: it is not offered back to claude, and does not hold its window
    let (_, body) = next(&e, "claude").await;
    assert_eq!(body["task"], Value::Null, "{body}");
    let (status, body) = next(&e, "muse").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["from"], "handoff", "{body}");
    assert_eq!(body["task"]["id"], t["task"]["id"]);
    assert_eq!(body["task"]["owner_agent"], "muse");
    assert_eq!(body["generation"], 2);
}

#[tokio::test]
async fn the_backlog_names_the_agent_that_ranks_first_for_each_item() {
    let e = env().await;
    put_agent(&e, "muse", &[], &["code"], 1).await;
    put_agent(&e, "claude", &["code"], &[], 4).await;
    post_item(&e, "demo", "untagged", &[], 600).await;
    post_item(&e, "demo", "code", &["code"], 600).await;
    post_item(&e, "demo", "image", &["image"], 600).await;
    let items = backlog(&e, "").await;
    let best = |title: &str| {
        items
            .iter()
            .find(|i| i["title"] == title)
            .map(|i| i["best_agent"].clone())
            .unwrap()
    };
    assert_eq!(best("untagged"), "muse"); // cheapest
    assert_eq!(best("code"), "claude"); // best at code
    assert_eq!(best("image"), Value::Null); // nobody can do it
}

// ---- Plan 9 Task 2: next offers reviews and merges before new work ----

async fn post_for(e: &Env, title: &str, needs: &[&str], agent: &str) {
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/backlog",
        Some(
            json!({"repo": "demo", "title": title, "goal": "Make hello louder",
                    "needs": needs, "prefer": agent, "reserve_secs": 600}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
}

async fn task_op(e: &Env, task_id: &str, op: &str, body: Value) -> (StatusCode, Value) {
    call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{task_id}/{op}"),
        Some(body),
    )
    .await
}

/// `agent` takes the backlog item titled `title` (pinned to it) and submits it.
async fn take_and_submit(e: &Env, agent: &str, title: &str, needs: &[&str]) -> Value {
    post_for(e, title, needs, agent).await;
    let (status, t) = next(e, agent).await;
    assert_eq!(status, StatusCode::OK, "{t}");
    assert_eq!(t["from"], "backlog", "{t}");
    assert_eq!(t["task"]["title"], title, "{t}");
    edit_greeter(e, &t);
    record_decision(e, &t).await;
    let (status, b) = task_op(e, &id(&t), "submit", json!({"agent": agent})).await;
    assert_eq!(status, StatusCode::OK, "{b}");
    t
}

async fn approve(e: &Env, task_id: &str, reviewer: &str) {
    let (status, b) = task_op(
        e,
        task_id,
        "validate",
        json!({"agent": reviewer, "command": "pytest", "exit_code": 0}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{b}");
    let (status, b) = task_op(
        e,
        task_id,
        "review",
        json!({"reviewer": reviewer, "verdict": "approve"}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{b}");
}

async fn three_agents(e: &Env) {
    put_agent(e, "claude", &["code"], &[], 4).await;
    put_agent(e, "grok", &["image"], &["code"], 2).await;
    put_agent(e, "codex", &[], &["code"], 3).await;
}

#[tokio::test]
async fn next_offers_another_agents_task_in_review_before_backlog_work() {
    let e = env().await;
    three_agents(&e).await;
    let t = take_and_submit(&e, "claude", "louder hello", &["code"]).await;
    post_item(&e, "demo", "draw it", &["image"], 0).await;
    let (status, body) = next(&e, "grok").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["from"], "review", "{body}");
    assert_eq!(body["task"]["id"], t["task"]["id"]);
    assert_eq!(body["approved"], false);
    assert_eq!(
        body["next"],
        "Review it: review_task, run its tests in its workspace and validate, then post_review. If you approve, merge it."
    );
    // the image item is still queued
    assert_eq!(backlog(&e, "?state=queued").await.len(), 1);
}

#[tokio::test]
async fn next_never_offers_an_agent_its_own_review() {
    let e = env().await;
    three_agents(&e).await;
    let t = take_and_submit(&e, "claude", "louder hello", &["code"]).await;
    let (_, body) = next(&e, "claude").await;
    assert_ne!(body["task"]["id"], t["task"]["id"], "{body}");
    assert_eq!(body["task"], Value::Null, "{body}");
}

#[tokio::test]
async fn a_review_needs_the_tags_of_the_item() {
    let e = env().await;
    three_agents(&e).await;
    // painter can do image work too, so the review waits for an agent that can
    put_agent(&e, "painter", &[], &["image"], 5).await;
    let t = take_and_submit(&e, "grok", "draw it", &["image"]).await;
    let (_, body) = next(&e, "claude").await;
    assert_eq!(
        body["task"],
        Value::Null,
        "claude cannot do image work: {body}"
    );
    let (_, body) = next(&e, "painter").await;
    assert_eq!(body["from"], "review", "{body}");
    assert_eq!(body["task"]["id"], t["task"]["id"]);
}

/// Only grok can do image work, and grok cannot review its own task: rather than leave it in review
/// for good, any other agent may review it.
#[tokio::test]
async fn a_review_nobody_else_can_do_goes_to_any_other_agent() {
    let e = env().await;
    put_agent(&e, "claude", &["code"], &[], 4).await;
    put_agent(&e, "grok", &["image"], &["code"], 2).await;
    let t = take_and_submit(&e, "grok", "draw it", &["image"]).await;
    let (_, body) = next(&e, "claude").await;
    assert_eq!(body["from"], "review", "{body}");
    assert_eq!(body["task"]["id"], t["task"]["id"]);
}

/// claude takes a `code` backlog item and hands it off without tags.
async fn take_and_hand_off_untagged(e: &Env) -> Value {
    post_for(e, "louder hello", &["code"], "claude").await;
    let (status, t) = next(e, "claude").await;
    assert_eq!(status, StatusCode::OK, "{t}");
    edit_greeter(e, &t);
    record_decision(e, &t).await;
    let (status, b) = hand_off_with(e, &t, json!({})).await;
    assert_eq!(status, StatusCode::OK, "{b}");
    t
}

/// A handoff without tags keeps the tags of the item it was made from.
#[tokio::test]
async fn an_untagged_handoff_keeps_the_tags_of_its_item() {
    let e = env().await;
    three_agents(&e).await;
    put_agent(&e, "grok", &["image"], &[], 2).await; // grok cannot code here
    let t = take_and_hand_off_untagged(&e).await;
    let (_, body) = next(&e, "grok").await;
    assert_eq!(body["task"], Value::Null, "{body}");
    let (_, body) = next(&e, "codex").await;
    assert_eq!(body["from"], "handoff", "{body}");
    assert_eq!(body["task"]["id"], t["task"]["id"]);
}

/// ... unless nobody but the agent that handed it off can do them: then any other agent may take it,
/// rather than leave it handed off for good.
#[tokio::test]
async fn an_untagged_handoff_nobody_else_can_do_goes_to_any_other_agent() {
    let e = env().await;
    put_agent(&e, "claude", &["code"], &[], 4).await;
    put_agent(&e, "grok", &["image"], &[], 2).await;
    let t = take_and_hand_off_untagged(&e).await;
    let (_, body) = next(&e, "grok").await;
    assert_eq!(body["from"], "handoff", "{body}");
    assert_eq!(body["task"]["id"], t["task"]["id"]);
}

#[tokio::test]
async fn two_agents_are_not_offered_the_same_review() {
    let e = env().await;
    three_agents(&e).await;
    let t = take_and_submit(&e, "claude", "louder hello", &["code"]).await;
    let (_, first) = next(&e, "codex").await;
    assert_eq!(first["task"]["id"], t["task"]["id"], "{first}");
    let (_, second) = next(&e, "grok").await;
    assert_eq!(second["task"], Value::Null, "reserved for codex: {second}");
    let (_, again) = next(&e, "codex").await;
    assert_eq!(again["task"]["id"], t["task"]["id"], "{again}");
}

#[tokio::test]
async fn an_approved_task_is_offered_as_merge() {
    let e = env().await;
    three_agents(&e).await;
    let t = take_and_submit(&e, "claude", "louder hello", &["code"]).await;
    approve(&e, &id(&t), "grok").await;
    // grok holds the reservation; another agent sees the task once grok's turn is over, but grok
    // itself is told to merge it
    let (_, body) = next(&e, "grok").await;
    assert_eq!(body["from"], "review", "{body}");
    assert_eq!(body["approved"], true);
    assert_eq!(body["next"], "It is approved: merge it.");
    let (status, b) = task_op(&e, &id(&t), "merge", json!({"agent": "grok"})).await;
    assert_eq!(status, StatusCode::OK, "{b}");
    let (_, body) = next(&e, "codex").await;
    assert_eq!(
        body["task"],
        Value::Null,
        "merged work is not offered: {body}"
    );
}

#[tokio::test]
async fn a_resubmitted_task_is_offered_for_review_again_not_as_approved() {
    let e = env().await;
    three_agents(&e).await;
    let t = take_and_submit(&e, "claude", "louder hello", &["code"]).await;
    let (status, b) = task_op(
        &e,
        &id(&t),
        "review",
        json!({"reviewer": "grok", "verdict": "request_changes",
               "findings": [{"summary": "say why it shouts"}]}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{b}");
    let (_, body) = next(&e, "grok").await;
    assert_eq!(body["task"], Value::Null, "back with its owner: {body}");
    std::fs::write(
        ws_dir(&e, &t).join("app/greeter.py"),
        "class Greeter:\n    def hello(self, name):\n        return name.upper()\n",
    )
    .unwrap();
    let (status, b) = task_op(&e, &id(&t), "submit", json!({"agent": "claude"})).await;
    assert_eq!(status, StatusCode::OK, "{b}");
    let (_, body) = next(&e, "grok").await;
    assert_eq!(body["from"], "review", "{body}");
    assert_eq!(body["approved"], false);
}

// ---- Plan 9 Task 3: next rescues tasks whose owner went quiet ----

/// claude starts a task directly (no backlog item, so no tags) under a 1 s lease, then goes
/// quiet past it.
async fn abandoned(e: &Env) -> Value {
    put_agent(e, "claude", &["code"], &[], 4).await;
    put_agent(e, "grok", &["image"], &["code"], 2).await;
    let t = start_task(e, "Make hello louder").await;
    tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
    t
}

#[tokio::test]
async fn an_abandoned_task_is_offered_after_its_lease() {
    let e = env_with(|s| s.with_lease_secs(1)).await;
    let t = abandoned(&e).await;
    let (status, body) = next(&e, "grok").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["from"], "abandoned", "{body}");
    assert_eq!(body["task"]["id"], t["task"]["id"]);
    assert_eq!(body["task"]["owner_agent"], "grok");
    assert_eq!(body["generation"], 2);
}

#[tokio::test]
async fn a_task_inside_its_lease_is_not_offered() {
    let e = env_with(|s| s.with_lease_secs(3600)).await;
    abandoned(&e).await;
    let (_, body) = next(&e, "grok").await;
    assert_eq!(body["task"], Value::Null, "{body}");
}

#[tokio::test]
async fn an_abandoned_task_goes_back_to_its_restarted_owner() {
    let e = env_with(|s| s.with_lease_secs(1)).await;
    let t = abandoned(&e).await;
    let (_, body) = next(&e, "claude").await;
    assert_eq!(body["from"], "abandoned", "{body}");
    assert_eq!(body["task"]["id"], t["task"]["id"]);
    assert_eq!(body["task"]["owner_agent"], "claude");
}

#[tokio::test]
async fn a_rescued_task_refuses_the_old_owners_writes() {
    let e = env_with(|s| s.with_lease_secs(1)).await;
    let t = abandoned(&e).await;
    let (_, body) = next(&e, "grok").await;
    assert_eq!(body["from"], "abandoned", "{body}");
    let uri = format!("/v1/tasks/{}/checkpoint", id(&t));
    let (status, resp) = call_as(&e.app, "POST", &uri, None, Some(1)).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{resp}");
    assert!(
        resp["error"].as_str().unwrap().contains("claim lost"),
        "{resp}"
    );
}

// ---- Plan 9 Task 6: a dry-run peek ----

async fn peek(e: &Env, agent: &str) -> Value {
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/backlog/next",
        Some(json!({"agent": agent, "session": "s-peek", "dry_run": true})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body
}

#[tokio::test]
async fn a_peek_takes_nothing() {
    let e = env().await;
    put_agent(&e, "muse", &[], &["code"], 1).await;
    let item = post_item(&e, "demo", "louder hello", &[], 0).await;
    let p = peek(&e, "muse").await;
    assert_eq!(p["would"]["from"], "backlog", "{p}");
    assert_eq!(p["would"]["backlog_id"], item["id"]);
    assert_eq!(p["would"]["title"], "louder hello");
    assert_eq!(p["would"]["repo"], "demo");
    assert_eq!(backlog(&e, "?state=queued").await.len(), 1, "still queued");
    let (_, body) = next(&e, "muse").await;
    assert_eq!(body["backlog_id"], item["id"], "{body}");
    assert_eq!(peek(&e, "muse").await, json!({"would": null, "queued": 0}));
}

#[tokio::test]
async fn a_peek_names_what_next_would_return() {
    let e = env().await;
    three_agents(&e).await;
    // a review
    let t = take_and_submit(&e, "claude", "louder hello", &["code"]).await;
    let p = peek(&e, "grok").await;
    assert_eq!(p["would"]["from"], "review", "{p}");
    assert_eq!(p["would"]["task_id"], t["task"]["id"]);
    let (_, body) = next(&e, "grok").await;
    assert_eq!(body["from"], "review");
    assert_eq!(body["task"]["id"], p["would"]["task_id"]);
    // a handoff
    let h = start_task(&e, "Make goodbye louder").await;
    edit_greeter(&e, &h);
    record_decision(&e, &h).await;
    let (status, _) = hand_off_with(&e, &h, json!({"needs": ["code"], "prefer": "codex"})).await;
    assert_eq!(status, StatusCode::OK);
    let p = peek(&e, "codex").await;
    assert_eq!(p["would"]["from"], "handoff", "{p}");
    assert_eq!(p["would"]["task_id"], h["task"]["id"]);
    let (_, body) = next(&e, "codex").await;
    assert_eq!(body["from"], "handoff", "{body}");
    assert_eq!(body["task"]["id"], h["task"]["id"]);
}

#[tokio::test]
async fn a_peek_does_not_reserve_a_review() {
    let e = env().await;
    three_agents(&e).await;
    let t = take_and_submit(&e, "claude", "louder hello", &["code"]).await;
    assert_eq!(peek(&e, "codex").await["would"]["task_id"], t["task"]["id"]);
    let (_, body) = next(&e, "grok").await;
    assert_eq!(body["task"]["id"], t["task"]["id"], "{body}");
}

// ---- Plan 9 Task 7: next waits for work ----

async fn next_waiting(app: &Router, agent: &str, wait: u64) -> (StatusCode, Value) {
    call(
        app,
        "POST",
        "/v1/backlog/next",
        Some(json!({"agent": agent, "session": "s-wait", "wait_secs": wait})),
    )
    .await
}

#[tokio::test(flavor = "multi_thread")]
async fn next_waits_for_work_that_arrives() {
    let e = env().await;
    put_agent(&e, "grok", &["image"], &[], 2).await;
    let app = e.app.clone();
    let started = std::time::Instant::now();
    let waiting = tokio::spawn(async move { next_waiting(&app, "grok", 10).await });
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    post_item(&e, "demo", "draw it", &["image"], 0).await;
    let (status, body) = waiting.await.unwrap();
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["from"], "backlog", "{body}");
    assert_eq!(body["task"]["title"], "draw it");
    assert!(started.elapsed() < std::time::Duration::from_secs(5));
}

#[tokio::test(flavor = "multi_thread")]
async fn wait_is_clamped_and_zero_returns_at_once() {
    let e = env_with(|s| s.with_max_wait(std::time::Duration::from_secs(1))).await;
    let t = std::time::Instant::now();
    let (_, body) = next_waiting(&e.app, "grok", 0).await;
    assert_eq!(body, json!({"task": null, "queued": 0}));
    assert!(
        t.elapsed() < std::time::Duration::from_millis(500),
        "{:?}",
        t.elapsed()
    );
    let t = std::time::Instant::now();
    let (_, body) = next_waiting(&e.app, "grok", 100_000).await;
    assert_eq!(body, json!({"task": null, "queued": 0}));
    let took = t.elapsed();
    assert!(
        took >= std::time::Duration::from_millis(900) && took < std::time::Duration::from_secs(4),
        "{took:?}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn a_peek_never_waits() {
    let e = env().await;
    let t = std::time::Instant::now();
    let (_, body) = call(
        &e.app,
        "POST",
        "/v1/backlog/next",
        Some(json!({"agent": "grok", "session": "s", "dry_run": true, "wait_secs": 30})),
    )
    .await;
    assert_eq!(body, json!({"would": null, "queued": 0}));
    assert!(t.elapsed() < std::time::Duration::from_millis(500));
}

// ---- a team's limit on open tasks (`max_open`)

async fn put_limited(e: &Env, name: &str, best: &[&str], cost: i32, max_open: i32) {
    let (status, body) = call(
        &e.app,
        "PUT",
        &format!("/v1/agents/{name}"),
        Some(json!({"best": best, "cost_rank": cost, "max_open": max_open})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["agent"]["max_open"], max_open, "{body}");
}

/// An agent CLI that runs subagents is a team: it may take work in parallel, but only up to its
/// limit, so one team cannot hold the whole backlog. Reviews and merges finish work, so they are
/// still offered at the limit.
#[tokio::test]
async fn a_team_at_its_limit_gets_reviews_but_no_new_work() {
    let e = env().await;
    put_limited(&e, "claude", &["code"], 4, 1).await;
    put_agent(&e, "grok", &["image"], &["code"], 2).await;
    post_item(&e, "demo", "first", &["code"], 600).await;
    let (_, first) = next(&e, "claude").await;
    assert_eq!(first["task"]["title"], "first", "{first}");
    post_item(&e, "demo", "second", &["code"], 600).await;

    let (status, body) = next(&e, "claude").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["task"], Value::Null, "{body}");
    assert_eq!(body["at_capacity"], json!({"open": 1, "max": 1}), "{body}");
    let (_, peek) = call(
        &e.app,
        "POST",
        "/v1/backlog/next",
        Some(json!({"agent": "claude", "session": "s", "dry_run": true})),
    )
    .await;
    assert_eq!(peek["would"], Value::Null, "{peek}");
    assert_eq!(peek["at_capacity"]["max"], 1, "{peek}");

    // grok's image work needs a reviewer, and claude is offered it at the limit
    let g = take_and_submit(&e, "grok", "draw it", &["image"]).await;
    let (_, body) = next(&e, "claude").await;
    assert_eq!(body["from"], "review", "{body}");
    approve(&e, &id(&g), "claude").await;
    let (status, b) = task_op(&e, &id(&g), "merge", json!({"agent": "claude"})).await;
    assert_eq!(status, StatusCode::OK, "{b}");

    // claude submits its task: below the limit again, it takes the next item
    edit_greeter(&e, &first);
    record_decision(&e, &first).await;
    let (status, b) = task_op(&e, &id(&first), "submit", json!({"agent": "claude"})).await;
    assert_eq!(status, StatusCode::OK, "{b}");
    let (_, body) = next(&e, "claude").await;
    assert_eq!(body["task"]["title"], "second", "{body}");
    assert!(body.get("at_capacity").is_none(), "{body}");
}

/// Its own abandoned task is work it already holds: resuming it adds nothing.
#[tokio::test]
async fn a_team_at_its_limit_still_resumes_its_own_abandoned_task() {
    let e = env_with(|s| s.with_lease_secs(1)).await;
    put_limited(&e, "claude", &["code"], 4, 1).await;
    let t = start_task(&e, "Make hello louder").await;
    tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
    post_item(&e, "demo", "other", &["code"], 600).await;
    let (_, body) = next(&e, "claude").await;
    assert_eq!(body["from"], "abandoned", "{body}");
    assert_eq!(body["task"]["id"], t["task"]["id"]);
}

#[tokio::test]
async fn a_limit_below_one_is_refused() {
    let e = env().await;
    let (status, body) = call(
        &e.app,
        "PUT",
        "/v1/agents/claude",
        Some(json!({"best": ["code"], "max_open": 0})),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
}
