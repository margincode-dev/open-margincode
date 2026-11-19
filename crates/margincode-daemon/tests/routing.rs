use crate::common::*;
use margincode_core::IntentStore as _;

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

/// claude starts a task directly (no backlog item, so no tags) and grok can do it.
async fn started_by_claude(e: &Env) -> Value {
    put_agent(e, "claude", &["code"], &[], 4).await;
    put_agent(e, "grok", &["image"], &["code"], 2).await;
    start_task(e, "Make hello louder").await
}

/// `started_by_claude`, then claude goes quiet past its lease.
async fn abandoned(e: &Env) -> Value {
    let t = started_by_claude(e).await;
    expire_lease(e, &t).await;
    t
}

#[tokio::test]
async fn an_abandoned_task_is_offered_after_its_lease() {
    let e = env().await;
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
    let e = env().await;
    started_by_claude(&e).await;
    assert_eq!(peek(&e, "grok").await["would"], Value::Null);
    let (_, body) = next(&e, "grok").await;
    assert_eq!(body["task"], Value::Null, "{body}");
}

#[tokio::test]
async fn an_abandoned_task_goes_back_to_its_restarted_owner() {
    let e = env().await;
    let t = abandoned(&e).await;
    let (_, body) = next(&e, "claude").await;
    assert_eq!(body["from"], "abandoned", "{body}");
    assert_eq!(body["task"]["id"], t["task"]["id"]);
    assert_eq!(body["task"]["owner_agent"], "claude");
}

#[tokio::test]
async fn a_rescued_task_refuses_the_old_owners_writes() {
    let e = env().await;
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

/// Dogfood 2026-11-03: claude's lease ran out while its task waited in review; grok requested
/// changes and its very next `next` took the task back as abandoned, reviewing its own work after.
/// The change request renews claude's lease, and once that runs out too the reviewer who sent the
/// task back still may not rescue it, while another agent may.
#[tokio::test]
async fn the_reviewer_who_requested_changes_does_not_rescue_the_task() {
    let e = env().await;
    three_agents(&e).await;
    let t = take_and_submit(&e, "claude", "louder hello", &["code"]).await;
    expire_lease(&e, &t).await;
    let (status, b) = task_op(
        &e,
        &id(&t),
        "review",
        json!({"reviewer": "grok", "verdict": "request_changes",
               "findings": [{"summary": "say why it shouts"}]}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{b}");

    // the owner has a fresh lease: nobody is offered the task now
    for agent in ["grok", "codex"] {
        let (_, body) = next(&e, agent).await;
        assert_eq!(body["task"], Value::Null, "{agent}: {body}");
    }

    expire_lease(&e, &t).await;
    assert_eq!(peek(&e, "grok").await["would"], Value::Null);
    let (_, body) = next(&e, "grok").await;
    assert_eq!(body["task"], Value::Null, "{body}");
    let (_, body) = next(&e, "codex").await;
    assert_eq!(body["from"], "abandoned", "{body}");
    assert_eq!(body["task"]["id"], t["task"]["id"]);
    assert_eq!(body["task"]["owner_agent"], "codex");
}

/// Dogfood 2026-11-16 (F16): a headless owner (`av watch`) asked `next` right after a change
/// request and was told there was nothing for it, because its task sat inside the lease the
/// change request had just renewed. The owner gets its task back at once, under the same
/// generation, with the findings; the reviewer and other agents still get nothing.
#[tokio::test]
async fn a_task_sent_back_is_offered_to_its_owner_at_once() {
    let e = env_with(|s| s.with_lease_secs(3600)).await;
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

    let would = &peek(&e, "claude").await["would"];
    assert_eq!(would["from"], "changes_requested", "{would}");
    assert_eq!(would["task_id"], t["task"]["id"]);

    let (status, body) = next(&e, "claude").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["from"], "changes_requested", "{body}");
    assert_eq!(body["task"]["id"], t["task"]["id"]);
    assert_eq!(body["task"]["owner_agent"], "claude");
    assert_eq!(
        body["generation"], t["task"]["generation"],
        "resumed, not claimed"
    );
    assert!(
        body["bundle"]["markdown"]
            .as_str()
            .unwrap()
            .contains("say why it shouts"),
        "the findings come with it: {body}"
    );
    assert!(
        body["next"].as_str().unwrap().contains("av submit"),
        "{body}"
    );

    for agent in ["grok", "codex"] {
        let (_, body) = next(&e, agent).await;
        assert_eq!(body["task"], Value::Null, "{agent}: {body}");
    }
}

/// Once the owner has moved the head after the change request it is fixing the task, so a second
/// session under the same agent name is not handed the same task.
#[tokio::test]
async fn a_task_its_owner_is_already_fixing_is_not_offered_again() {
    let e = env_with(|s| s.with_lease_secs(3600)).await;
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
    std::fs::write(
        ws_dir(&e, &t).join("app/greeter.py"),
        "class Greeter:\n    def hello(self, name):\n        return name.upper()  # shouts\n",
    )
    .unwrap();
    let uri = format!("/v1/tasks/{}/checkpoint", id(&t));
    let (status, b) = call_as(&e.app, "POST", &uri, None, Some(1)).await;
    assert_eq!(status, StatusCode::OK, "{b}");

    assert_eq!(peek(&e, "claude").await["would"], Value::Null);
}

async fn next_with(e: &Env, agent: &str, extra: Value) -> Value {
    let mut b = json!({"agent": agent, "session": "s-next"});
    b.as_object_mut()
        .unwrap()
        .extend(extra.as_object().unwrap().clone());
    let (status, body) = call(&e.app, "POST", "/v1/backlog/next", Some(b)).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body
}

/// Dogfood 2026-11-16 (F31): the Cloud runner handles its owners' sent-back tasks itself (it
/// counts review rounds and escalates), so after F16 `next` handed it back a task it had
/// escalated, ahead of its backlog. A caller that says `skip_returned` is not offered its own
/// sent-back task, in a peek or a take, and gets its other work; one that does not still is.
#[tokio::test]
async fn a_caller_that_skips_returned_tasks_gets_its_other_work() {
    let e = env_with(|s| s.with_lease_secs(3600)).await;
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
    post_for(&e, "quieter hello", &["code"], "claude").await;
    assert_eq!(
        peek(&e, "claude").await["would"]["from"],
        "changes_requested"
    );

    let skip = json!({"take": ["work"], "skip_returned": true});
    let mut peek_skip = skip.clone();
    peek_skip["dry_run"] = json!(true);
    let would = &next_with(&e, "claude", peek_skip).await["would"];
    assert_eq!(would["from"], "backlog", "{would}");
    let body = next_with(&e, "claude", skip).await;
    assert_eq!(body["from"], "backlog", "{body}");
    assert_eq!(body["task"]["title"], "quieter hello", "{body}");
    assert_ne!(body["task"]["id"], t["task"]["id"]);
}

/// Past the owner's lease a sent-back task is still its owner's returned task, not an abandoned
/// one, so a `skip_returned` owner is not handed it as `abandoned` either.
#[tokio::test]
async fn a_caller_that_skips_returned_tasks_does_not_get_them_back_after_the_lease() {
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
    expire_lease(&e, &t).await;

    let skip = json!({"take": ["work"], "skip_returned": true});
    let mut peek_skip = skip.clone();
    peek_skip["dry_run"] = json!(true);
    assert_eq!(
        next_with(&e, "claude", peek_skip).await["would"],
        Value::Null
    );
    let body = next_with(&e, "claude", skip).await;
    assert_eq!(body["task"], Value::Null, "{body}");
}

/// At its `max_open` limit an agent is offered only what finishes work, its sent-back task
/// among them; with `skip_returned` that is nothing.
#[tokio::test]
async fn at_its_limit_a_caller_that_skips_returned_tasks_gets_nothing() {
    let e = env_with(|s| s.with_lease_secs(3600)).await;
    put_limited(&e, "claude", &["code"], 4, 1).await;
    put_agent(&e, "grok", &["image"], &["code"], 2).await;
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
    post_for(&e, "quieter hello", &["code"], "claude").await;
    assert_eq!(
        peek(&e, "claude").await["would"]["from"],
        "changes_requested"
    );

    let body = next_with(&e, "claude", json!({"skip_returned": true})).await;
    assert_eq!(body["task"], Value::Null, "{body}");
    assert_eq!(body["at_capacity"]["max"], 1, "{body}");
}

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
    let t = take_and_submit(&e, "claude", "louder hello", &["code"]).await;
    let p = peek(&e, "grok").await;
    assert_eq!(p["would"]["from"], "review", "{p}");
    assert_eq!(p["would"]["task_id"], t["task"]["id"]);
    let (_, body) = next(&e, "grok").await;
    assert_eq!(body["from"], "review");
    assert_eq!(body["task"]["id"], p["would"]["task_id"]);
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

async fn next_waiting(app: &Router, agent: &str, wait: u64) -> (StatusCode, Value) {
    call(
        app,
        "POST",
        "/v1/backlog/next",
        Some(json!({"agent": agent, "session": "s-wait", "wait_secs": wait})),
    )
    .await
}

/// How long a waiting `next` asks to wait, and its tick and clamp in `env_no_tick`: an hour, far
/// past `HANG`, so a call that answers at all was woken.
const NO_TICK: std::time::Duration = std::time::Duration::from_secs(3600);

/// A waiting `next` whose timer never fires within the test: only a wake can answer it.
async fn env_no_tick() -> Env {
    env_with(|s| s.with_wait_tick(NO_TICK).with_max_wait(NO_TICK)).await
}

/// Waits until the item titled `title` is taken. A waiting `next` takes it before it makes the
/// task's workspace and first pack, so this waits for the wake, not the start.
async fn taken_on_wake(e: &Env, title: &str) {
    eventually(&format!("{title:?} to be taken on a wake"), async || {
        backlog(e, "?state=taken")
            .await
            .iter()
            .any(|i| i["title"] == title)
    })
    .await;
}

/// A spawned waiting `next`'s answer, or a panic once it has waited `HANG`.
async fn answer(waiting: tokio::task::JoinHandle<(StatusCode, Value)>) -> (StatusCode, Value) {
    tokio::time::timeout(HANG, waiting)
        .await
        .expect("hung: the waiting call was not woken")
        .unwrap()
}

#[tokio::test(flavor = "multi_thread")]
async fn next_waits_for_work_that_arrives() {
    let e = env_no_tick().await;
    put_agent(&e, "grok", &["image"], &[], 2).await;
    let app = e.app.clone();
    let waiting = tokio::spawn(async move { next_waiting(&app, "grok", NO_TICK.as_secs()).await });
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    post_item(&e, "demo", "draw it", &["image"], 0).await;
    taken_on_wake(&e, "draw it").await;
    let (status, body) = answer(waiting).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["from"], "backlog", "{body}");
    assert_eq!(body["task"]["title"], "draw it");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_submit_wakes_an_agent_waiting_for_a_review() {
    let e = env_no_tick().await;
    three_agents(&e).await;
    post_for(&e, "louder hello", &["code"], "claude").await;
    let (_, t) = next(&e, "claude").await;
    assert_eq!(t["from"], "backlog", "{t}");
    edit_greeter(&e, &t);
    record_decision(&e, &t).await;
    let app = e.app.clone();
    let waiting = tokio::spawn(async move { next_waiting(&app, "grok", NO_TICK.as_secs()).await });
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let (status, b) = task_op(&e, &id(&t), "submit", json!({"agent": "claude"})).await;
    assert_eq!(status, StatusCode::OK, "{b}");
    let (status, body) = answer(waiting).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["from"], "review", "{body}");
    assert_eq!(body["task"]["id"], t["task"]["id"]);
}

#[tokio::test(flavor = "multi_thread")]
async fn a_profile_change_wakes_an_agent_that_now_fits() {
    let e = env_no_tick().await;
    put_agent(&e, "grok", &["image"], &[], 2).await;
    post_item(&e, "demo", "louder hello", &["code"], 0).await;
    let app = e.app.clone();
    let waiting = tokio::spawn(async move { next_waiting(&app, "grok", NO_TICK.as_secs()).await });
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    put_agent(&e, "grok", &["image"], &["code"], 2).await;
    taken_on_wake(&e, "louder hello").await;
    let (status, body) = answer(waiting).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["task"]["title"], "louder hello", "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_zero_wait_returns_at_once() {
    let e = env_no_tick().await;
    let (_, body) = tokio::time::timeout(HANG, next_waiting(&e.app, "grok", 0))
        .await
        .expect("hung: a zero wait waited");
    assert_eq!(body, json!({"task": null, "queued": 0}));
}

/// Unclamped, the call would wait 27 hours; `HANG` catches that.
#[tokio::test(flavor = "multi_thread")]
async fn a_wait_is_clamped_to_max_wait() {
    let e = env_with(|s| s.with_max_wait(std::time::Duration::from_secs(1))).await;
    let t = std::time::Instant::now();
    let (_, body) = tokio::time::timeout(HANG, next_waiting(&e.app, "grok", 100_000))
        .await
        .expect("hung: the wait was not clamped");
    assert_eq!(body, json!({"task": null, "queued": 0}));
    assert!(
        t.elapsed() >= std::time::Duration::from_millis(900),
        "{:?}",
        t.elapsed()
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn a_peek_never_waits() {
    let e = env_no_tick().await;
    let peek = call(
        &e.app,
        "POST",
        "/v1/backlog/next",
        Some(json!({"agent": "grok", "session": "s", "dry_run": true,
                    "wait_secs": NO_TICK.as_secs()})),
    );
    let (_, body) = tokio::time::timeout(HANG, peek)
        .await
        .expect("hung: a peek waited");
    assert_eq!(body, json!({"would": null, "queued": 0}));
}

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

    let g = take_and_submit(&e, "grok", "draw it", &["image"]).await;
    let (_, body) = next(&e, "claude").await;
    assert_eq!(body["from"], "review", "{body}");
    approve(&e, &id(&g), "claude").await;
    let (status, b) = task_op(&e, &id(&g), "merge", json!({"agent": "claude"})).await;
    assert_eq!(status, StatusCode::OK, "{b}");

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
    let e = env().await;
    put_limited(&e, "claude", &["code"], 4, 1).await;
    let t = start_task(&e, "Make hello louder").await;
    expire_lease(&e, &t).await;
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

#[tokio::test]
async fn next_routes_twelve_agents_with_no_cap() {
    let e = env().await;
    for i in 0..12 {
        let name = format!("agent-{i:02}");
        let tag = format!("skill-{i:02}");
        put_agent(&e, &name, &[&tag], &[], 1).await;
        post_item(&e, "demo", &format!("job {i:02}"), &[&tag], 600).await;
    }
    let mut ids = Vec::new();
    for i in 0..12 {
        let name = format!("agent-{i:02}");
        let (status, body) = next(&e, &name).await;
        assert_eq!(status, StatusCode::OK, "{name}: {body}");
        assert_eq!(body["from"], "backlog", "{name}: {body}");
        assert_eq!(body["task"]["owner_agent"], name, "{body}");
        assert_eq!(body["task"]["title"], format!("job {i:02}"), "{body}");
        assert!(
            body.get("at_capacity").is_none(),
            "{name} hit a cap: {body}"
        );
        ids.push(body["task"]["id"].as_str().unwrap().to_owned());
    }
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), 12, "two agents were given the same task");

    put_agent(&e, "agent-12", &["skill-12"], &[], 1).await;
    let (status, body) = next(&e, "agent-12").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body, json!({"task": null, "queued": 0}));
    assert!(body.get("at_capacity").is_none(), "{body}");
}

async fn post_after(e: &Env, title: &str, after: &[&str]) -> (StatusCode, Value) {
    call(
        &e.app,
        "POST",
        "/v1/backlog",
        Some(
            json!({"repo": "demo", "title": title, "goal": "Make hello louder",
                    "after": after, "reserve_secs": 0}),
        ),
    )
    .await
}

#[tokio::test]
async fn an_item_is_not_offered_until_the_items_it_lists_have_merged() {
    let e = env().await;
    put_agent(&e, "claude", &["code"], &[], 1).await;
    put_agent(&e, "grok", &["code"], &[], 2).await;
    post_item(&e, "demo", "stop hook", &["code"], 0).await;
    // by title slug; the dependent is posted before A is taken
    let (status, b) = post_after(&e, "keep agents busy", &["stop-hook"]).await;
    assert_eq!(status, StatusCode::OK, "{b}");
    assert_eq!(b["waiting_on"], json!(["stop hook"]), "{b}");

    let (_, t) = next(&e, "grok").await;
    assert_eq!(t["task"]["title"], "stop hook", "{t}");
    let (_, body) = next(&e, "grok").await;
    assert_eq!(body["task"], Value::Null, "B waits while A is open: {body}");
    let items = backlog(&e, "?state=queued").await;
    assert_eq!(items[0]["waiting_on"], json!(["stop hook"]));

    edit_greeter(&e, &t);
    record_decision(&e, &t).await;
    let (status, b) = task_op(&e, &id(&t), "submit", json!({"agent": "grok"})).await;
    assert_eq!(status, StatusCode::OK, "{b}");
    let (_, body) = next(&e, "grok").await;
    assert_eq!(
        body["task"],
        Value::Null,
        "B waits while A is in review: {body}"
    );
    let (_, body) = next(&e, "claude").await;
    assert_eq!(
        body["from"], "review",
        "claude gets the review, not B: {body}"
    );

    approve(&e, &id(&t), "claude").await;
    let (status, b) = task_op(&e, &id(&t), "merge", json!({"agent": "claude"})).await;
    assert_eq!(status, StatusCode::OK, "{b}");
    assert_eq!(
        backlog(&e, "?state=queued").await[0]["waiting_on"],
        json!([])
    );
    let (_, body) = next(&e, "grok").await;
    assert_eq!(body["from"], "backlog", "{body}");
    assert_eq!(body["task"]["title"], "keep agents busy", "{body}");
}

#[tokio::test]
async fn after_takes_an_item_id_and_refuses_an_unknown_item() {
    let e = env().await;
    let a = post_item(&e, "demo", "first", &[], 0).await;
    let id = a["id"].as_str().unwrap();
    let (status, b) = post_after(&e, "second", &[id]).await;
    assert_eq!(status, StatusCode::OK, "{b}");
    assert_eq!(b["after"], json!([id]), "{b}");
    let (status, b) = post_after(&e, "third", &["nothing-like-it"]).await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{b}");
}

const SYNC_ACTION: &str = "Its merge conflicted because main moved: av sync it onto main, resolve \
                           the conflict markers, re-run the check, validate and submit it again.";

async fn start_edit_submit(e: &Env, agent: &str, title: &str, text: &str) -> Value {
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
    let (status, b) = task_op(e, &id(&t), "submit", json!({"agent": agent})).await;
    assert_eq!(status, StatusCode::OK, "{b}");
    t
}

const GREETER_LOUD: &str =
    "class Greeter:\n    def hello(self, name):\n        return name.upper()\n";
const GREETER_SHOUT: &str =
    "class Greeter:\n    def hello(self, name):\n        return name + \"!\"\n";

/// claude's task and codex's change the same lines. Both are approved and codex's merges first, so
/// claude's merge comes out conflicted. Returns claude's task.
async fn conflicted_task(e: &Env) -> Value {
    three_agents(e).await;
    let mine = start_edit_submit(e, "claude", "Make hello loud", GREETER_LOUD).await;
    let theirs = start_edit_submit(e, "codex", "Make hello shout", GREETER_SHOUT).await;
    approve(e, &id(&theirs), "grok").await;
    let (status, b) = task_op(e, &id(&theirs), "merge", json!({"agent": "grok"})).await;
    assert_eq!(status, StatusCode::OK, "{b}");
    approve(e, &id(&mine), "grok").await;
    let (status, b) = task_op(e, &id(&mine), "merge", json!({"agent": "grok"})).await;
    assert_eq!(status, StatusCode::OK, "{b}");
    assert_eq!(b["status"], "conflicted", "{b}");
    mine
}

#[tokio::test]
async fn a_conflicted_task_is_offered_to_its_owner_with_the_sync_action() {
    let e = env().await;
    let t = conflicted_task(&e).await;

    let p = peek(&e, "claude").await;
    assert_eq!(p["would"]["from"], "conflicted", "{p}");
    assert_eq!(p["would"]["task_id"], t["task"]["id"], "{p}");

    let (status, body) = next(&e, "claude").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["from"], "conflicted", "{body}");
    assert_eq!(body["task"]["id"], t["task"]["id"]);
    assert_eq!(body["task"]["owner_agent"], "claude");
    assert_eq!(body["task"]["status"], "conflicted");
    assert_eq!(body["generation"], 1, "the owner keeps it: {body}");
    assert_eq!(body["next"], SYNC_ACTION);
    assert!(body["bundle"]["markdown"].is_string(), "{body}");
}

#[tokio::test]
async fn another_agent_gets_a_conflicted_task_only_after_the_owners_lease() {
    let e = env().await;
    let t = conflicted_task(&e).await;

    // the owner takes it back, which renews its lease: nobody else is offered it now
    let (_, body) = next(&e, "claude").await;
    assert_eq!(body["from"], "conflicted", "{body}");
    for other in ["grok", "codex"] {
        assert_eq!(peek(&e, other).await["would"], Value::Null, "{other}");
        let (_, body) = next(&e, other).await;
        assert_eq!(body["task"], Value::Null, "{other}: {body}");
    }

    expire_lease(&e, &t).await;
    let (_, body) = next(&e, "grok").await;
    assert_eq!(body["from"], "conflicted", "{body}");
    assert_eq!(body["task"]["id"], t["task"]["id"]);
    assert_eq!(body["task"]["owner_agent"], "grok");
    assert_eq!(body["task"]["status"], "conflicted");
    assert_eq!(body["generation"], 2);
    assert_eq!(body["next"], SYNC_ACTION);
    let (_, body) = next(&e, "claude").await;
    assert_eq!(body["task"], Value::Null, "{body}");
}

#[tokio::test]
async fn a_review_comes_before_the_owners_conflicted_task() {
    let e = env_with(|s| s.with_lease_secs(3600)).await;
    let t = conflicted_task(&e).await;
    let other = take_and_submit(&e, "codex", "louder hello", &["code"]).await;
    let (_, body) = next(&e, "claude").await;
    assert_eq!(body["from"], "review", "{body}");
    assert_eq!(body["task"]["id"], other["task"]["id"]);
    // once the review is out of the way (reserved for claude, who holds it), the owner asks again
    // for its own task: the review is still its to do, so merge it away first
    approve(&e, &id(&other), "claude").await;
    let (status, b) = task_op(&e, &id(&other), "merge", json!({"agent": "claude"})).await;
    assert_eq!(status, StatusCode::OK, "{b}");
    let (_, body) = next(&e, "claude").await;
    assert_eq!(body["from"], "conflicted", "{body}");
    assert_eq!(body["task"]["id"], t["task"]["id"]);
}

async fn next_taking(e: &Env, agent: &str, take: &[&str]) -> (StatusCode, Value) {
    call(
        &e.app,
        "POST",
        "/v1/backlog/next",
        Some(json!({"agent": agent, "session": "s-next", "take": take})),
    )
    .await
}

#[tokio::test]
async fn a_worker_that_takes_only_work_skips_reviews_and_a_reviewer_skips_work() {
    let e = env().await;
    three_agents(&e).await;
    let t = take_and_submit(&e, "claude", "louder hello", &["code"]).await;
    post_item(&e, "demo", "quieter hello", &["code"], 0).await;

    let (_, peek) = call(
        &e.app,
        "POST",
        "/v1/backlog/next",
        Some(json!({"agent": "grok", "session": "s", "dry_run": true})),
    )
    .await;
    assert_eq!(peek["would"]["from"], "review", "{peek}");

    let (status, r) = next_taking(&e, "codex", &["review", "merge"]).await;
    assert_eq!(status, StatusCode::OK, "{r}");
    assert_eq!(r["from"], "review", "{r}");
    assert_eq!(r["task"]["id"], t["task"]["id"]);
    let (_, r) = next_taking(&e, "codex", &["review"]).await;
    // its own reserved review again; never the item
    assert_ne!(r["from"], "backlog", "{r}");

    let (status, w) = next_taking(&e, "grok", &["work"]).await;
    assert_eq!(status, StatusCode::OK, "{w}");
    assert_eq!(w["from"], "backlog", "{w}");
    assert_eq!(w["task"]["title"], "quieter hello");

    approve(&e, &id(&t), "codex").await;
    let (_, r) = next_taking(&e, "codex", &["review"]).await;
    assert_eq!(r["task"], Value::Null, "{r}");
    let (_, r) = next_taking(&e, "codex", &["merge"]).await;
    assert_eq!(r["from"], "review", "{r}");
    assert_eq!(r["approved"], true, "{r}");
}

#[tokio::test]
async fn an_unknown_take_is_a_client_error() {
    let e = env().await;
    let (status, _) = next_taking(&e, "claude", &["everything"]).await;
    assert!(status.is_client_error(), "{status}");
}

#[tokio::test]
async fn next_routes_a_dozen_agents_at_once_with_no_cap() {
    // Community never caps the number of agents: twelve CLIs, each best at its own thing,
    // all holding a task at the same time.
    let e = env().await;
    let names: Vec<String> = (1..=12).map(|i| format!("agent-{i:02}")).collect();
    for (i, name) in names.iter().enumerate() {
        put_agent(&e, name, &[&format!("skill-{i:02}")], &[], 1 + i as i32 % 4).await;
    }
    for i in 0..names.len() {
        post_item(
            &e,
            "demo",
            &format!("job {i:02}"),
            &[&format!("skill-{i:02}")],
            600,
        )
        .await;
    }

    let (status, body) = call(&e.app, "GET", "/v1/agents", None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["agents"].as_array().unwrap().len(), names.len());

    // in reverse, so the order of asking does not decide who gets what
    for (i, name) in names.iter().enumerate().rev() {
        let (status, body) = next(&e, name).await;
        assert_eq!(status, StatusCode::OK, "{body}");
        assert_eq!(body["task"]["owner_agent"], name.as_str(), "{body}");
        assert_eq!(body["task"]["title"], format!("job {i:02}"), "{body}");
    }
    let items = backlog(&e, "").await;
    assert_eq!(items.len(), names.len());
    assert!(items.iter().all(|it| it["state"] == "taken"), "{items:?}");
}

async fn add_target(e: &Env, repo: &str, title: &str) -> Value {
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/targets",
        Some(json!({"repo": repo, "title": title, "detail": "why", "due": "2026-12-01"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body
}

async fn targets(e: &Env, query: &str) -> Vec<Value> {
    let (status, body) = call(&e.app, "GET", &format!("/v1/targets{query}"), None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body["targets"].as_array().unwrap().clone()
}

#[tokio::test]
async fn targets_are_added_listed_and_closed_over_http() {
    let e = env().await;
    let t = add_target(&e, "demo", "Ship v1").await;
    assert_eq!(t["status"], "active");
    assert_eq!(t["due"], "2026-12-01");
    assert_eq!(t["created_by"], "user");
    add_target(&e, "other", "Elsewhere").await;
    assert_eq!(targets(&e, "?repo=demo").await.len(), 1);
    assert_eq!(targets(&e, "").await.len(), 2);

    let (status, body) = call(
        &e.app,
        "PUT",
        "/v1/targets/ship-v1",
        Some(json!({"repo": "demo", "status": "done"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["status"], "done");
    assert!(targets(&e, "?repo=demo&status=active").await.is_empty());
    let id = t["id"].as_str().unwrap();
    let (status, body) = call(
        &e.app,
        "PUT",
        &format!("/v1/targets/{id}"),
        Some(json!({"status": "active"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");

    let (status, _) = call(
        &e.app,
        "PUT",
        "/v1/targets/ship-v1",
        Some(json!({"repo": "demo", "status": "finished"})),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    let (status, _) = call(
        &e.app,
        "PUT",
        "/v1/targets/nope",
        Some(json!({"repo": "demo", "status": "done"})),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let (status, _) = call(
        &e.app,
        "POST",
        "/v1/targets",
        Some(json!({"repo": "demo", "title": " "})),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn backlog_items_count_toward_a_target_and_merges_move_it() {
    let e = env().await;
    put_agent(&e, "claude", &["code"], &[], 1).await;
    add_target(&e, "demo", "Ship v1").await;
    let (status, a) = call(
        &e.app,
        "POST",
        "/v1/backlog",
        Some(
            json!({"repo": "demo", "title": "Louder hello", "goal": "Make hello louder",
                    "target": "ship-v1"}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{a}");
    assert_eq!(a["target"], "Ship v1");
    let second = post_item(&e, "demo", "Second", &[], 0).await;
    let (status, body) = call(
        &e.app,
        "PUT",
        &format!("/v1/backlog/{}/target", second["id"].as_str().unwrap()),
        Some(json!({"target": "ship-v1"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["target"], "Ship v1");
    let listed = backlog(&e, "?repo=demo").await;
    assert!(
        listed.iter().all(|i| i["target"] == "Ship v1"),
        "{listed:?}"
    );
    let t = &targets(&e, "?repo=demo").await[0];
    assert_eq!(
        (t["items"].as_i64(), t["queued"].as_i64()),
        (Some(2), Some(2))
    );

    let (status, t) = next(&e, "claude").await;
    assert_eq!(status, StatusCode::OK, "{t}");
    assert_eq!(t["task"]["title"], "Louder hello", "{t}");
    edit_greeter(&e, &t);
    record_decision(&e, &t).await;
    let task = id(&t);
    let (status, b) = task_op(&e, &task, "submit", json!({"agent": "claude"})).await;
    assert_eq!(status, StatusCode::OK, "{b}");
    approve(&e, &task, "grok").await;
    let (status, b) = task_op(&e, &task, "merge", json!({"agent": "claude"})).await;
    assert_eq!(status, StatusCode::OK, "{b}");
    let t = &targets(&e, "?repo=demo").await[0];
    assert_eq!(
        (
            t["items"].as_i64(),
            t["queued"].as_i64(),
            t["merged"].as_i64()
        ),
        (Some(2), Some(1), Some(1)),
        "{t}"
    );

    let (status, body) = call(
        &e.app,
        "PUT",
        &format!("/v1/backlog/{}/target", second["id"].as_str().unwrap()),
        Some(json!({"target": null})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert!(body["target"].is_null());
}

#[tokio::test]
async fn a_closed_target_takes_no_items() {
    let e = env().await;
    add_target(&e, "demo", "Ship v1").await;
    call(
        &e.app,
        "PUT",
        "/v1/targets/ship-v1",
        Some(json!({"repo": "demo", "status": "dropped"})),
    )
    .await;
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/backlog",
        Some(json!({"repo": "demo", "title": "x", "goal": "y", "target": "ship-v1"})),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(
        body["error"].as_str().unwrap().contains("dropped"),
        "{body}"
    );
}

/// The state the env was built from, so a test can write what an extension writes.
async fn env_and_state() -> (Env, AppState) {
    let keep: Arc<std::sync::Mutex<Option<AppState>>> = Arc::default();
    let k = keep.clone();
    let e = env_with(move |s| {
        *k.lock().unwrap() = Some(s.clone());
        s
    })
    .await;
    let st = keep.lock().unwrap().clone().unwrap();
    (e, st)
}

fn managed(name: &str, by: Option<&str>) -> margincode_core::routing::Profile {
    margincode_core::routing::Profile {
        name: name.into(),
        best: vec!["review".into()],
        can: vec![],
        cost_rank: 2,
        aliases: vec![],
        max_open: None,
        managed_by: by.map(str::to_owned),
    }
}

async fn agent(e: &Env, name: &str) -> Value {
    let (status, body) = call(&e.app, "GET", "/v1/agents", None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body["agents"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["name"] == name)
        .cloned()
        .unwrap_or_else(|| panic!("no agent {name} in {body}"))
}

#[tokio::test]
async fn a_managed_profile_refuses_the_api_put_and_names_its_manager() {
    let (e, st) = env_and_state().await;
    st.put_profile(&managed("reviewer", Some("ee")))
        .await
        .unwrap();
    assert_eq!(agent(&e, "reviewer").await["managed_by"], "ee");

    let (status, body) = call(
        &e.app,
        "PUT",
        "/v1/agents/reviewer",
        Some(json!({"best": ["code"], "cost_rank": 9})),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT, "{body}");
    assert!(
        body["error"]
            .as_str()
            .unwrap()
            .contains("agent reviewer is managed by ee; change it there"),
        "{body}"
    );
    let a = agent(&e, "reviewer").await;
    assert_eq!(a["best"], json!(["review"]), "{a}");
    assert_eq!(a["cost_rank"], 2, "{a}");
    assert_eq!(a["managed_by"], "ee", "{a}");

    st.put_profile(&managed("reviewer", None)).await.unwrap();
    assert_eq!(agent(&e, "reviewer").await["managed_by"], Value::Null);
    let (status, body) = call(
        &e.app,
        "PUT",
        "/v1/agents/reviewer",
        Some(json!({"best": ["code"], "cost_rank": 9})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["agent"]["cost_rank"], 9, "{body}");
}

#[tokio::test]
async fn a_profile_put_through_the_api_is_not_managed_and_can_be_overwritten() {
    let e = env().await;
    put_agent(&e, "claude", &["code"], &[], 4).await;
    let a = agent(&e, "claude").await;
    assert!(a.get("managed_by").is_some(), "{a}");
    assert_eq!(a["managed_by"], Value::Null, "{a}");
    put_agent(&e, "claude", &["docs"], &[], 1).await;
    let a = agent(&e, "claude").await;
    assert_eq!(a["best"], json!(["docs"]), "{a}");
    assert_eq!(a["cost_rank"], 1, "{a}");
}

fn names(body: &Value) -> Vec<String> {
    body["agents"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a["name"].as_str().unwrap().to_owned())
        .collect()
}

async fn start_as(e: &Env, agent_name: &str, title: &str) -> Value {
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/tasks",
        Some(
            json!({"repo": "demo", "title": title, "goal": "Make hello louder",
                    "agent": agent_name, "session": format!("s-{agent_name}")}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body
}

#[tokio::test]
async fn an_agent_profile_can_be_removed_unless_it_owns_work_or_an_extension_manages_it() {
    let (e, st) = env_and_state().await;

    let (status, body) = call(&e.app, "DELETE", "/v1/agents/nobody", None).await;
    assert_eq!(status, StatusCode::NOT_FOUND, "{body}");
    assert!(
        body["error"].as_str().unwrap().contains("agent nobody"),
        "{body}"
    );

    put_agent(&e, "muse", &[], &["docs"], 1).await;
    let (status, body) = call(&e.app, "DELETE", "/v1/agents/muse", None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["removed"], "muse", "{body}");
    let (status, listed) = call(&e.app, "GET", "/v1/agents", None).await;
    assert_eq!(status, StatusCode::OK, "{listed}");
    assert!(!names(&listed).contains(&"muse".to_owned()), "{listed}");

    put_agent(&e, "claude", &["code"], &[], 4).await;
    let open = start_task(&e, "Make hello louder").await;
    let (status, body) = call(&e.app, "DELETE", "/v1/agents/claude", None).await;
    assert_eq!(status, StatusCode::CONFLICT, "{body}");
    assert!(
        body["error"].as_str().unwrap().contains("force=true"),
        "{body}"
    );
    assert_eq!(agent(&e, "claude").await["name"], "claude");
    let (status, body) = call(&e.app, "DELETE", "/v1/agents/claude?force=true", None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (_, still) = call(&e.app, "GET", &format!("/v1/tasks/{}", id(&open)), None).await;
    assert_eq!(still["task"]["status"], "open", "{still}");
    assert_eq!(still["task"]["owner_agent"], "claude", "{still}");

    // in review blocks. Before the handoff below: `next` would claim that handed-off task
    // ahead of the backlog item this uses to reach in_review.
    put_agent(&e, "bea", &["code"], &[], 4).await;
    let _review = take_and_submit(&e, "bea", "louder hello", &["code"]).await;
    let (status, body) = call(&e.app, "DELETE", "/v1/agents/bea", None).await;
    assert_eq!(status, StatusCode::CONFLICT, "{body}");

    put_agent(&e, "ada", &["code"], &[], 3).await;
    let handed = start_as(&e, "ada", "Pass it on").await;
    edit_greeter(&e, &handed);
    record_decision(&e, &handed).await;
    let (status, body) = handoff(&e, &handed, "ada").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = call(&e.app, "DELETE", "/v1/agents/ada", None).await;
    assert_eq!(status, StatusCode::CONFLICT, "{body}");

    put_agent(&e, "cy", &["code"], &[], 2).await;
    let done = start_as(&e, "cy", "Never mind").await;
    let (status, body) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/abandon", id(&done)),
        Some(json!({"agent": "cy"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = call(&e.app, "DELETE", "/v1/agents/cy", None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["removed"], "cy", "{body}");

    st.put_profile(&managed("reviewer", Some("ee")))
        .await
        .unwrap();
    let (status, body) = call(&e.app, "DELETE", "/v1/agents/reviewer?force=true", None).await;
    assert_eq!(status, StatusCode::CONFLICT, "{body}");
    assert!(
        body["error"]
            .as_str()
            .unwrap()
            .contains("agent reviewer is managed by ee; change it there"),
        "{body}"
    );
    assert_eq!(agent(&e, "reviewer").await["managed_by"], "ee");
}

/// A conflicted task goes back to its owner (`next` offers it first), so removing the owner's
/// profile would strand it: 409 unless forced, and the profile is still there after the refusal.
#[tokio::test]
async fn a_conflicted_task_keeps_its_owners_profile_unless_forced() {
    let e = env().await;
    let t = conflicted_task(&e).await;
    let owner = t["task"]["owner_agent"].as_str().unwrap().to_owned();
    let (status, body) = call(&e.app, "DELETE", &format!("/v1/agents/{owner}"), None).await;
    assert_eq!(status, StatusCode::CONFLICT, "{body}");
    assert!(
        body["error"].as_str().unwrap().contains("conflicted"),
        "{body}"
    );
    assert_eq!(agent(&e, &owner).await["name"], owner.as_str());
    let uri = format!("/v1/agents/{owner}?force=true");
    let (status, body) = call(&e.app, "DELETE", &uri, None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
}

/// The check and the delete are one statement: with a task the owner holds, `delete_profile`
/// refuses without touching the row, and an unknown name is told apart from a held one.
#[tokio::test]
async fn delete_profile_refuses_a_held_profile_and_names_an_unknown_one_in_one_step() {
    let e = env().await;
    put_agent(&e, "claude", &["code"], &[], 4).await;
    let _open = start_task(&e, "Make hello louder").await;
    let err = e.store.delete_profile("claude", false).await.unwrap_err();
    assert!(
        matches!(err, margincode_core::AvError::Conflict(_)),
        "{err}"
    );
    assert_eq!(agent(&e, "claude").await["name"], "claude");
    let err = e.store.delete_profile("nobody", false).await.unwrap_err();
    assert!(
        matches!(err, margincode_core::AvError::NotFound(_)),
        "{err}"
    );
    let err = e.store.delete_profile("nobody", true).await.unwrap_err();
    assert!(
        matches!(err, margincode_core::AvError::NotFound(_)),
        "{err}"
    );
    e.store.delete_profile("claude", true).await.unwrap();
}

async fn post_json(e: &Env, body: Value) -> (StatusCode, Value) {
    call(&e.app, "POST", "/v1/backlog", Some(body)).await
}

async fn accept(e: &Env, id: &str, by: &str) -> (StatusCode, Value) {
    call(
        &e.app,
        "POST",
        &format!("/v1/backlog/{id}/accept"),
        Some(json!({ "by": by })),
    )
    .await
}

async fn set_agent_posts(e: &Env, repo: &str, mode: &str) -> Value {
    let (status, body) = call(
        &e.app,
        "PUT",
        &format!("/v1/repos/{repo}/settings"),
        Some(json!({ "agent_posts": mode })),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body
}

#[tokio::test]
async fn a_proposed_item_is_not_offered_until_it_is_accepted() {
    let e = env().await;
    let (status, p) = post_json(
        &e,
        json!({"repo": "demo", "title": "Cache packs", "goal": "Cache them",
               "reserve_secs": 0, "propose": true, "why": "unsure it pays off"}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{p}");
    assert_eq!(p["state"], "proposed");
    assert_eq!(p["why"], "unsure it pays off");
    let id = p["id"].as_str().unwrap().to_owned();

    assert_eq!(
        peek(&e, "claude").await,
        json!({"would": null, "queued": 0})
    );
    let (_, body) = next(&e, "claude").await;
    assert_eq!(body, json!({"task": null, "queued": 0}));
    assert_eq!(backlog(&e, "?state=proposed").await.len(), 1);

    let (status, a) = accept(&e, &id, "vinh").await;
    assert_eq!(status, StatusCode::OK, "{a}");
    assert_eq!(a["state"], "queued");
    assert_eq!(a["accepted_by"], "vinh");
    let (_, events) = call(&e.app, "GET", "/v1/events?repo=demo&since=0", None).await;
    let accepted: Vec<&Value> = events["events"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|v| v["kind"] == "backlog:accepted")
        .collect();
    assert_eq!(accepted.len(), 1, "{events}");
    assert_eq!(accepted[0]["payload"]["by"], "vinh");
    assert_eq!(accepted[0]["payload"]["backlog_id"], id.as_str());
    let (status, body) = next(&e, "claude").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["from"], "backlog", "{body}");
    assert_eq!(body["backlog_id"], id.as_str());

    let (status, body) = accept(&e, &id, "vinh").await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    let (status, _) = accept(&e, "00000000-0000-0000-0000-000000000000", "vinh").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let (status, _) = accept(&e, &id, " ").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn a_proposed_item_is_dropped_like_a_queued_one() {
    let e = env().await;
    let (_, p) = post_json(
        &e,
        json!({"repo": "demo", "title": "Maybe", "goal": "g", "propose": true}),
    )
    .await;
    let id = p["id"].as_str().unwrap();
    let (status, _) = call(&e.app, "DELETE", &format!("/v1/backlog/{id}"), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(backlog(&e, "?state=dropped").await.len(), 1);
    let (status, _) = accept(&e, id, "vinh").await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test(flavor = "multi_thread")]
async fn an_accept_wakes_an_agent_waiting_for_work() {
    let e = env_no_tick().await;
    let (_, p) = post_json(
        &e,
        json!({"repo": "demo", "title": "draw it", "goal": "g", "reserve_secs": 0,
               "propose": true}),
    )
    .await;
    let app = e.app.clone();
    let waiting = tokio::spawn(async move { next_waiting(&app, "grok", NO_TICK.as_secs()).await });
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let (status, body) = accept(&e, p["id"].as_str().unwrap(), "vinh").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    taken_on_wake(&e, "draw it").await;
    let (status, body) = answer(waiting).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["task"]["title"], "draw it", "{body}");
}

#[tokio::test]
async fn an_accepted_item_starts_its_reserve_window_when_accepted() {
    let e = env().await;
    put_agent(&e, "claude", &["code"], &[], 4).await;
    put_agent(&e, "muse", &[], &["code"], 1).await;
    let (_, p) = post_json(
        &e,
        json!({"repo": "demo", "title": "louder", "goal": "g", "needs": ["code"],
               "reserve_secs": 3600, "propose": true}),
    )
    .await;
    let id = p["id"].as_str().unwrap();
    // proposed long ago: its window would be over if it counted from the post
    sqlx::query("UPDATE backlog SET created_at = now() - interval '2 hours' WHERE id = $1::uuid")
        .bind(id)
        .execute(e.store.pool())
        .await
        .unwrap();
    accept(&e, id, "vinh").await;
    // inside the window only claude (best at code) may take it
    let (_, body) = next(&e, "muse").await;
    assert!(body["task"].is_null(), "{body}");
    let (_, body) = next(&e, "claude").await;
    assert_eq!(body["from"], "backlog", "{body}");
}

#[tokio::test]
async fn agent_posts_propose_makes_every_agents_post_a_proposal() {
    let e = env().await;
    put_agent(&e, "claude", &["code"], &[], 4).await;
    let (status, body) = call(
        &e.app,
        "PUT",
        "/v1/agents/grok",
        Some(json!({"best": ["image"], "cost_rank": 2, "aliases": ["grok-cli"]})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let s = set_agent_posts(&e, "demo", "propose").await;
    assert_eq!(s["agent_posts"], "propose");
    let (_, s) = call(&e.app, "GET", "/v1/repos/demo/settings", None).await;
    assert_eq!(s["agent_posts"], "propose");

    let post = |by: &str, from_agent: bool| {
        json!({"repo": "demo", "title": format!("by {by}"), "goal": "g", "posted_by": by,
               "from_agent": from_agent})
    };
    // a profiled agent (by name or alias), or a post marked as an agent's: proposed
    for (by, from_agent) in [("claude", false), ("grok-cli", false), ("someone", true)] {
        let (status, b) = post_json(&e, post(by, from_agent)).await;
        assert_eq!(status, StatusCode::OK, "{b}");
        assert_eq!(b["state"], "proposed", "{by}: {b}");
    }
    // a person (no profile, no marker): queued
    let (_, b) = post_json(&e, post("vinh", false)).await;
    assert_eq!(b["state"], "queued", "{b}");
    let (_, b) = post_json(&e, json!({"repo": "demo", "title": "anon", "goal": "g"})).await;
    assert_eq!(b["state"], "queued", "{b}");
    assert_eq!(b["posted_by"], "user");

    // another repo keeps the default: an agent's post queues
    let (_, b) = post_json(
        &e,
        json!({"repo": "other", "title": "x", "goal": "g", "posted_by": "claude"}),
    )
    .await;
    assert_eq!(b["state"], "queued", "{b}");

    set_agent_posts(&e, "demo", "queue").await;
    let (_, b) = post_json(&e, post("claude", true)).await;
    assert_eq!(b["state"], "queued", "{b}");
    let (status, _) = call(
        &e.app,
        "PUT",
        "/v1/repos/demo/settings",
        Some(json!({ "agent_posts": "sometimes" })),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn a_why_over_300_characters_is_refused() {
    let e = env().await;
    let (status, body) = post_json(
        &e,
        json!({"repo": "demo", "title": "t", "goal": "g", "propose": true,
               "why": "x".repeat(301)}),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
}

#[tokio::test]
async fn an_item_after_a_proposal_waits_for_it_to_be_accepted_and_merged() {
    let e = env().await;
    let (_, p) = post_json(
        &e,
        json!({"repo": "demo", "title": "Maybe first", "goal": "g", "propose": true}),
    )
    .await;
    let (status, b) = post_after(&e, "then this", &["maybe-first"]).await;
    assert_eq!(status, StatusCode::OK, "{b}");
    assert_eq!(b["waiting_on"], json!(["Maybe first"]), "{b}");
    let (_, body) = next(&e, "claude").await;
    assert!(body["task"].is_null(), "{body}");
    // accepted, the proposal itself is offered first; the later item still waits for its merge
    accept(&e, p["id"].as_str().unwrap(), "vinh").await;
    let (_, body) = next(&e, "claude").await;
    assert_eq!(body["task"]["title"], "Maybe first", "{body}");
}

/// A guard that refuses to let anyone accept their own proposal, and remembers what it was asked.
#[derive(Default)]
struct NoSelfAccept(std::sync::Mutex<Vec<String>>);

#[async_trait::async_trait]
impl margincode_daemon::MergeGuard for NoSelfAccept {
    async fn check(&self, action: margincode_daemon::Guarded<'_>) -> Result<(), String> {
        if let margincode_daemon::Guarded::AcceptBacklog { item, by } = action {
            self.0
                .lock()
                .unwrap()
                .push(format!("{} by {by}", item.item.title));
            if item.posted_by == by {
                return Err(format!("{by} proposed it"));
            }
        }
        Ok(())
    }
}

#[tokio::test]
async fn a_merge_guard_can_refuse_an_accept_and_the_item_stays_proposed() {
    let guard = std::sync::Arc::new(NoSelfAccept::default());
    let g = guard.clone();
    let e = env_with(move |s| s.with_merge_guard(g)).await;
    let (_, p) = post_json(
        &e,
        json!({"repo": "demo", "title": "Mine", "goal": "g", "propose": true,
               "posted_by": "claude"}),
    )
    .await;
    let id = p["id"].as_str().unwrap();
    let (status, body) = accept(&e, id, "claude").await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(
        body["error"]
            .as_str()
            .unwrap()
            .contains("cannot accept: claude proposed it"),
        "{body}"
    );
    assert_eq!(backlog(&e, "?state=proposed").await.len(), 1);
    let (status, body) = accept(&e, id, "vinh").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(*guard.0.lock().unwrap(), ["Mine by claude", "Mine by vinh"]);
    // a queued item is refused before the guard is asked
    let (status, _) = accept(&e, id, "vinh").await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(guard.0.lock().unwrap().len(), 2);
}

// Every route that hands out a workspace writes its `.av-task`: the task, a session and the
// generation, no agent (the daemon does not know the name the hooks there act as).

#[tokio::test]
async fn a_task_started_from_the_backlog_gets_its_task_file() {
    let e = env().await;
    three_agents(&e).await;
    post_for(&e, "louder hello", &["code"], "claude").await;
    let (status, t) = next(&e, "claude").await;
    assert_eq!(status, StatusCode::OK, "{t}");
    assert_eq!(t["from"], "backlog", "{t}");
    let tf = task_file(&e, &t);
    assert_eq!(tf["task"], t["task"]["id"], "{tf}");
    assert_eq!(tf["session"], "s-next", "{tf}");
    assert_eq!(tf["generation"], 1, "{tf}");
    assert!(tf.get("agent").is_none(), "{tf}");
}

#[tokio::test]
async fn a_claimed_task_gets_a_fresh_task_file_of_its_new_generation() {
    let e = env().await;
    let t = start_task(&e, "Louder").await;
    assert_eq!(task_file(&e, &t)["generation"], 1);
    record_decision(&e, &t).await;
    let (status, b) = handoff(&e, &t, "claude").await;
    assert_eq!(status, StatusCode::OK, "{b}");
    // the previous owner's, as the CLI and its Stop hook leave it
    std::fs::write(
        ws_dir(&e, &t).join(".av-task"),
        json!({"task": id(&t), "session": "old", "generation": 1, "agent": "claude",
               "continue_session": "cc-1", "continues": 3})
        .to_string(),
    )
    .unwrap();
    let (status, c) = claim(&e, &t, "codex").await;
    assert_eq!(status, StatusCode::OK, "{c}");
    let tf = task_file(&e, &t);
    assert!(tf.get("agent").is_none(), "{tf}");
    assert_eq!(tf["generation"], c["generation"], "{tf}");
    assert_eq!(tf["generation"], 2, "{tf}");
    assert_eq!(tf["session"], c["session"], "the claim's session: {tf}");
    assert!(tf.get("continues").is_none(), "counters start afresh: {tf}");
    assert!(tf.get("continue_session").is_none(), "{tf}");
}

#[tokio::test]
async fn a_task_sent_back_to_its_owner_gets_a_fresh_task_file() {
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
    let _ = std::fs::remove_file(ws_dir(&e, &t).join(".av-task"));
    let (status, r) = next(&e, "claude").await;
    assert_eq!(status, StatusCode::OK, "{r}");
    assert_eq!(r["from"], "changes_requested", "{r}");
    let tf = task_file(&e, &t);
    assert!(tf.get("agent").is_none(), "{tf}");
    assert_eq!(tf["generation"], 1, "{tf}");
    assert_eq!(tf["session"], r["session"], "{tf}");
}

#[tokio::test]
async fn a_resume_keeps_what_the_task_file_already_holds() {
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
    let file = ws_dir(&e, &t).join(".av-task");
    std::fs::write(
        &file,
        json!({"task": id(&t), "session": "old", "generation": 1, "agent": "claude",
               "continue_session": "cc-1", "continues": 3, "nudged_session": "cc-1"})
        .to_string(),
    )
    .unwrap();
    let (status, r) = next(&e, "claude").await;
    assert_eq!(status, StatusCode::OK, "{r}");
    assert_eq!(r["from"], "changes_requested", "{r}");
    let tf = task_file(&e, &t);
    assert_eq!(tf["session"], r["session"], "a new session: {tf}");
    assert_eq!(tf["continues"], 3, "the Stop hook's cap still counts: {tf}");
    assert_eq!(tf["continue_session"], "cc-1", "{tf}");
    assert_eq!(tf["nudged_session"], "cc-1", "{tf}");
    assert_eq!(tf["agent"], "claude", "the CLI's name stays: {tf}");
    assert_eq!(tf["generation"], 1, "{tf}");
}

#[tokio::test]
async fn a_review_offer_leaves_the_owners_task_file_alone() {
    let e = env().await;
    three_agents(&e).await;
    let t = take_and_submit(&e, "claude", "louder hello", &["code"]).await;
    let before = task_file(&e, &t);
    let (status, r) = next(&e, "grok").await;
    assert_eq!(status, StatusCode::OK, "{r}");
    assert_eq!(r["from"], "review", "{r}");
    assert_eq!(
        task_file(&e, &t),
        before,
        "a reviewer must not fence the owner"
    );
}

#[tokio::test]
async fn a_task_file_the_daemon_cannot_write_does_not_fail_the_claim() {
    let e = env().await;
    let t = start_task(&e, "Louder").await;
    record_decision(&e, &t).await;
    let (status, b) = handoff(&e, &t, "claude").await;
    assert_eq!(status, StatusCode::OK, "{b}");
    let file = ws_dir(&e, &t).join(".av-task");
    let _ = std::fs::remove_file(&file);
    std::fs::create_dir(&file).unwrap();
    std::fs::write(file.join("keep"), "x").unwrap();
    let (status, c) = claim(&e, &t, "codex").await;
    assert_eq!(status, StatusCode::OK, "{c}");
    assert_eq!(c["generation"], 2, "{c}");
    assert!(file.join("keep").is_file(), "left as it was");
}

/// A link planted at `.av-task` is not read through (it could block the claim or fill memory):
/// the resume writes a fresh file in its place and leaves the link's target alone.
#[cfg(unix)]
#[tokio::test]
async fn a_link_at_the_task_file_is_not_followed() {
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
    let elsewhere = tempfile::TempDir::new().unwrap();
    let target = elsewhere.path().join("planted");
    let planted = json!({"task": id(&t), "session": "x", "generation": 1, "agent": "mallory",
                         "continues": 7})
    .to_string();
    std::fs::write(&target, &planted).unwrap();
    let file = ws_dir(&e, &t).join(".av-task");
    let _ = std::fs::remove_file(&file);
    std::os::unix::fs::symlink(&target, &file).unwrap();
    let (status, r) = next(&e, "claude").await;
    assert_eq!(status, StatusCode::OK, "{r}");
    assert_eq!(r["from"], "changes_requested", "{r}");
    assert!(
        !std::fs::symlink_metadata(&file)
            .unwrap()
            .file_type()
            .is_symlink()
    );
    let tf = task_file(&e, &t);
    assert!(
        tf.get("agent").is_none(),
        "nothing read through the link: {tf}"
    );
    assert!(tf.get("continues").is_none(), "{tf}");
    assert_eq!(
        std::fs::read_to_string(&target).unwrap(),
        planted,
        "the target is untouched"
    );
}

// ---- F12: a backlog item whose task was abandoned ----

/// muse takes the item titled `title` and abandons its task; returns the item and the task.
async fn take_and_abandon(e: &Env, title: &str) -> (Value, Value) {
    put_agent(e, "muse", &[], &["code"], 1).await;
    let item = post_item(e, "demo", title, &[], 0).await;
    let (status, t) = next(e, "muse").await;
    assert_eq!(status, StatusCode::OK, "{t}");
    assert_eq!(t["from"], "backlog", "{t}");
    let (status, b) = task_op(e, &id(&t), "abandon", json!({"agent": "muse"})).await;
    assert_eq!(status, StatusCode::OK, "{b}");
    (item, t)
}

async fn requeue(e: &Env, item: &Value) -> (StatusCode, Value) {
    let item_id = item["id"].as_str().unwrap();
    call(
        &e.app,
        "POST",
        &format!("/v1/backlog/{item_id}/requeue"),
        None,
    )
    .await
}

#[tokio::test]
async fn an_item_whose_task_was_abandoned_can_be_requeued_or_dropped() {
    let e = env().await;
    let (item, t) = take_and_abandon(&e, "louder hello").await;
    let listed = backlog(&e, "").await;
    assert_eq!(listed[0]["state"], "taken");
    assert_eq!(listed[0]["task_status"], "abandoned", "{}", listed[0]);

    let (status, body) = requeue(&e, &item).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["state"], "queued", "{body}");
    assert_eq!(body["task_id"], Value::Null, "{body}");
    assert_eq!(body["taken_by"], Value::Null, "{body}");
    assert_eq!(body["title"], "louder hello", "the title is kept: {body}");
    let (_, events) = call(&e.app, "GET", "/v1/events?repo=demo&since=0", None).await;
    let requeued: Vec<&Value> = events["events"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|v| v["kind"] == "backlog:requeued")
        .collect();
    assert_eq!(requeued.len(), 1, "{events}");
    assert_eq!(requeued[0]["task_id"], t["task"]["id"], "{events}");
    assert_eq!(requeued[0]["payload"]["abandoned_task"], t["task"]["id"]);
    assert_eq!(requeued[0]["payload"]["backlog_id"], item["id"]);

    let (status, again) = next(&e, "muse").await;
    assert_eq!(status, StatusCode::OK, "{again}");
    assert_eq!(again["from"], "backlog", "{again}");
    assert_eq!(again["task"]["title"], "louder hello", "{again}");
    assert_ne!(again["task"]["id"], t["task"]["id"], "a new task: {again}");
    let listed = backlog(&e, "").await;
    assert_eq!(listed[0]["task_id"], again["task"]["id"]);
    assert_eq!(listed[0]["task_status"], "open", "{}", listed[0]);

    let (_, second) = take_and_abandon(&e, "quieter hello").await;
    let second_item = backlog(&e, "")
        .await
        .into_iter()
        .find(|i| i["task_id"] == second["task"]["id"])
        .unwrap();
    let second_id = second_item["id"].as_str().unwrap();
    let (status, body) = call(&e.app, "DELETE", &format!("/v1/backlog/{second_id}"), None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(backlog(&e, "?state=dropped").await.len(), 1);
    let (status, body) = requeue(&e, &second_item).await;
    assert_eq!(
        status,
        StatusCode::UNPROCESSABLE_ENTITY,
        "a dropped item stays dropped: {body}"
    );
}

#[tokio::test]
async fn an_item_whose_task_is_open_is_still_refused() {
    let e = env().await;
    put_agent(&e, "muse", &[], &["code"], 1).await;
    let item = post_item(&e, "demo", "louder hello", &[], 0).await;
    let (_, t) = next(&e, "muse").await;
    assert_eq!(t["from"], "backlog", "{t}");
    let item_id = item["id"].as_str().unwrap();

    let (status, body) = requeue(&e, &item).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    let (status, body) = call(&e.app, "DELETE", &format!("/v1/backlog/{item_id}"), None).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    let listed = backlog(&e, "").await;
    assert_eq!(listed[0]["state"], "taken");
    assert_eq!(listed[0]["task_id"], t["task"]["id"]);

    let (status, body) = requeue(&e, &post_item(&e, "demo", "queued", &[], 0).await).await;
    assert_eq!(
        status,
        StatusCode::UNPROCESSABLE_ENTITY,
        "a queued item is not requeued: {body}"
    );
    let (status, _) = call(
        &e.app,
        "POST",
        "/v1/backlog/00000000-0000-0000-0000-000000000000/requeue",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn an_item_whose_task_is_in_review_is_refused() {
    let e = env().await;
    put_agent(&e, "claude", &["code"], &[], 4).await;
    let t = take_and_submit(&e, "claude", "louder hello", &[]).await;
    let item = backlog(&e, "")
        .await
        .into_iter()
        .find(|i| i["task_id"] == t["task"]["id"])
        .unwrap();
    let item_id = item["id"].as_str().unwrap();
    let (status, body) = requeue(&e, &item).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    let (status, body) = call(&e.app, "DELETE", &format!("/v1/backlog/{item_id}"), None).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert_eq!(backlog(&e, "?state=taken").await.len(), 1);
}

#[tokio::test(flavor = "multi_thread")]
async fn a_requeue_wakes_an_agent_waiting_for_work() {
    let e = env_no_tick().await;
    let (item, _) = take_and_abandon(&e, "draw it").await;
    let app = e.app.clone();
    let waiting = tokio::spawn(async move { next_waiting(&app, "muse", NO_TICK.as_secs()).await });
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let (status, body) = requeue(&e, &item).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    taken_on_wake(&e, "draw it").await;
    let (status, body) = answer(waiting).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["task"]["title"], "draw it", "{body}");
}
