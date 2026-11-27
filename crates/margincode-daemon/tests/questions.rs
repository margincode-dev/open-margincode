use axum::http::StatusCode;
use serde_json::{Value, json};

use crate::common::{call, env, id, start_task};

const PROCEED: &str = "Carry on with Postgres. Record what you build on it.";
const PARK: &str = "Finish what does not depend on this, checkpoint, then stop this task.";

fn question(undo: &str) -> Value {
    json!({
        "question": "Which database should the cache use?",
        "options": ["Postgres", "SQLite"],
        "recommended": "Postgres",
        "why": "the daemon already runs one",
        "depends": "the cache's schema and its tests",
        "undo": undo,
        "state": {"done": "the cache's interface", "remaining": "its storage"},
        "anchors": [{"path": "app/greeter.py"}],
        "agent": "claude",
    })
}

async fn ask(e: &crate::common::Env, t: &Value, body: Value) -> (StatusCode, Value) {
    call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/questions", id(t)),
        Some(body),
    )
    .await
}

async fn answer(e: &crate::common::Env, question: &str, body: Value) -> (StatusCode, Value) {
    call(
        &e.app,
        "POST",
        &format!("/v1/questions/{question}/answer"),
        Some(body),
    )
    .await
}

async fn records(e: &crate::common::Env, t: &Value) -> Vec<Value> {
    let (status, body) = call(&e.app, "GET", &format!("/v1/tasks/{}/records", id(t)), None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body["records"].as_array().cloned().unwrap_or_default()
}

#[tokio::test(flavor = "multi_thread")]
async fn balanced_proceeds_when_a_wrong_guess_stays_in_the_workspace_and_parks_otherwise() {
    let e = env().await;
    let t = start_task(&e, "Cache greetings").await;
    let (status, inside) = ask(&e, &t, question("workspace")).await;
    assert_eq!(status, StatusCode::OK, "{inside}");
    assert_eq!(inside["instruction"], "proceed", "{inside}");
    assert_eq!(inside["level"], "balanced", "{inside}");
    assert_eq!(inside["told"], PROCEED, "{inside}");
    let (status, outside) = ask(&e, &t, question("outside")).await;
    assert_eq!(status, StatusCode::OK, "{outside}");
    assert_eq!(outside["instruction"], "park", "{outside}");
    assert_eq!(outside["told"], PARK, "{outside}");

    let asked: Vec<Value> = records(&e, &t)
        .await
        .into_iter()
        .filter(|r| r["kind"] == "question")
        .collect();
    assert_eq!(asked.len(), 2, "{asked:?}");
    assert_eq!(asked[0]["summary"], "Which database should the cache use?");
    assert_eq!(asked[0]["anchors"][0]["path"], "app/greeter.py");
    let body: Value = serde_json::from_str(asked[0]["body"].as_str().unwrap()).unwrap();
    assert_eq!(body["options"], json!(["Postgres", "SQLite"]));
    assert_eq!(body["recommended"], "Postgres");
    assert_eq!(body["undo"], "workspace");
    assert_eq!(body["state"]["remaining"], "its storage");
    assert_eq!(body["instruction"], "proceed");
}

#[tokio::test(flavor = "multi_thread")]
async fn the_asking_agents_profile_level_decides_and_an_alias_finds_it() {
    let e = env().await;
    let t = start_task(&e, "Cache greetings").await;
    let (status, body) = call(
        &e.app,
        "PUT",
        "/v1/agents/careful",
        Some(json!({"aliases": ["claude"], "ask_level": "ask_first"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["agent"]["ask_level"], "ask_first", "{body}");
    let (_, parked) = ask(&e, &t, question("workspace")).await;
    assert_eq!(parked["instruction"], "park", "{parked}");
    assert_eq!(parked["level"], "ask_first", "{parked}");

    let (status, body) = call(
        &e.app,
        "PUT",
        "/v1/agents/careful",
        Some(json!({"aliases": ["claude"], "ask_level": "carry_on"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (_, on) = ask(&e, &t, question("outside")).await;
    assert_eq!(on["instruction"], "proceed", "{on}");
    assert_eq!(on["told"], PROCEED, "{on}");

    let (_, listed) = call(&e.app, "GET", "/v1/agents", None).await;
    assert_eq!(listed["agents"][0]["ask_level"], "carry_on", "{listed}");

    // a put that leaves the level out goes back to the daemon's default
    let (status, body) = call(
        &e.app,
        "PUT",
        "/v1/agents/careful",
        Some(json!({"aliases": ["claude"]})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert!(body["agent"]["ask_level"].is_null(), "{body}");
    let (_, back) = ask(&e, &t, question("outside")).await;
    assert_eq!(back["level"], "balanced", "{back}");

    let (status, body) = call(
        &e.app,
        "PUT",
        "/v1/agents/careful",
        Some(json!({"ask_level": "sometimes"})),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
    assert!(body.to_string().contains("carry_on"), "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_question_is_checked_before_anything_is_written() {
    let e = env().await;
    let t = start_task(&e, "Cache greetings").await;
    let cases: Vec<(&str, Value, &str)> = vec![
        (
            "one option",
            json!({"options": ["Postgres"]}),
            "two to four",
        ),
        (
            "five options",
            json!({"options": ["a", "b", "c", "d", "e"]}),
            "two to four",
        ),
        (
            "twice the same option",
            json!({"options": ["Postgres", "postgres"]}),
            "twice",
        ),
        (
            "a recommendation that is no option",
            json!({"recommended": "Redis"}),
            "one of the options",
        ),
        ("no why", json!({"why": " "}), "why"),
        ("no depends", json!({"depends": ""}), "depends"),
        ("no question", json!({"question": ""}), "question"),
        ("an unknown undo", json!({"undo": "somewhere"}), "undo"),
        (
            "a long option",
            json!({"options": ["Postgres", "x".repeat(61)]}),
            "60",
        ),
        (
            "an option of two lines",
            json!({"options": ["Postgres", "SQLite\nwith WAL"]}),
            "one line",
        ),
        (
            "an option with a carriage return",
            json!({"options": ["Postgres", "SQLite\rin memory"]}),
            "one line",
        ),
    ];
    for (what, change, says) in cases {
        let mut body = question("workspace");
        for (k, v) in change.as_object().unwrap() {
            body[k] = v.clone();
        }
        let (status, answer) = ask(&e, &t, body).await;
        assert!(
            status == StatusCode::BAD_REQUEST || status == StatusCode::UNPROCESSABLE_ENTITY,
            "{what}: {status} {answer}"
        );
        assert!(
            answer.to_string().contains(says),
            "{what}: says {answer}, wanted {says:?}"
        );
    }
    assert!(
        records(&e, &t)
            .await
            .iter()
            .all(|r| r["kind"] != "question"),
        "a refused question was written"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn a_fourth_open_question_waits_for_an_answer_or_a_withdrawal() {
    let e = env().await;
    let t = start_task(&e, "Cache greetings").await;
    let mut ids = vec![];
    for _ in 0..3 {
        let (status, body) = ask(&e, &t, question("workspace")).await;
        assert_eq!(status, StatusCode::OK, "{body}");
        ids.push(body["id"].as_str().unwrap().to_owned());
    }
    let (status, refused) = ask(&e, &t, question("workspace")).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{refused}");
    let said = refused.to_string();
    assert!(said.contains("answer or withdraw one first"), "{said}");

    // the asking agent withdraws one with a note that supersedes it
    let (status, body) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/records", id(&t)),
        Some(
            json!({"kind": "note", "summary": "found it in the docs", "agent": "claude",
                    "supersedes": ids[0]}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = ask(&e, &t, question("workspace")).await;
    assert_eq!(status, StatusCode::OK, "{body}");

    let (status, body) = answer(&e, &ids[1], json!({"option": "SQLite", "by": "alice"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = ask(&e, &t, question("workspace")).await;
    assert_eq!(status, StatusCode::OK, "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn an_answer_is_a_decision_that_supersedes_the_question() {
    let e = env().await;
    let t = start_task(&e, "Cache greetings").await;
    let (_, asked) = ask(&e, &t, question("outside")).await;
    let q = asked["id"].as_str().unwrap().to_owned();

    let (status, open) = call(&e.app, "GET", "/v1/questions?repo=demo", None).await;
    assert_eq!(status, StatusCode::OK, "{open}");
    assert_eq!(open["questions"].as_array().unwrap().len(), 1, "{open}");
    assert_eq!(open["questions"][0]["id"], q.as_str(), "{open}");
    assert_eq!(
        open["questions"][0]["task"]["slug"], t["task"]["slug"],
        "{open}"
    );

    let (status, refused) = answer(&e, &q, json!({"option": "Redis", "by": "alice"})).await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{refused}");
    assert!(refused.to_string().contains("Postgres"), "{refused}");
    let (status, own) = answer(&e, &q, json!({"option": "SQLite", "by": "claude"})).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{own}");
    assert!(own.to_string().contains("its own question"), "{own}");

    // an option is named by its text or its number
    let (status, answered) = answer(
        &e,
        &format!("rec:{}", &q.replace('-', "")[..8]),
        json!({"option": "2", "comment": "keep it on one disk", "by": "alice"}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{answered}");
    let decision = &answered["decision"];
    assert_eq!(decision["kind"], "decision", "{answered}");
    assert_eq!(decision["supersedes"], q.as_str(), "{answered}");
    assert_eq!(decision["author_agent"], "alice", "{answered}");
    assert_eq!(
        decision["summary"],
        "Which database should the cache use? → SQLite"
    );
    assert_eq!(decision["anchors"][0]["path"], "app/greeter.py");
    let body = decision["body"].as_str().unwrap();
    assert!(body.starts_with("Chosen: SQLite\n"), "{body}");
    assert!(
        body.contains("not the recommended option (Postgres)"),
        "{body}"
    );
    assert!(body.contains("keep it on one disk"), "{body}");

    let (status, again) = answer(&e, &q, json!({"option": "Postgres", "by": "bob"})).await;
    assert_eq!(status, StatusCode::CONFLICT, "{again}");
    assert!(again.to_string().contains("already answered"), "{again}");
    let (_, open) = call(&e.app, "GET", "/v1/questions?repo=demo", None).await;
    assert_eq!(open["questions"], json!([]), "{open}");
}

#[tokio::test(flavor = "multi_thread")]
async fn the_record_route_neither_asks_nor_answers_a_question() {
    let e = env().await;
    let t = start_task(&e, "Cache greetings").await;
    let (status, body) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/records", id(&t)),
        Some(json!({"kind": "question", "summary": "Postgres?", "agent": "claude"})),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(body.to_string().contains("av ask"), "{body}");

    let (_, asked) = ask(&e, &t, question("workspace")).await;
    let (status, body) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/records", id(&t)),
        Some(
            json!({"kind": "decision", "summary": "Postgres it is", "agent": "claude",
                    "supersedes": asked["id"]}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(body.to_string().contains("av answer"), "{body}");

    let (status, body) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/submit", id(&t)),
        Some(json!({"agent": "claude",
                    "records": [{"kind": "question", "summary": "Postgres?"}]})),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(body.to_string().contains("av ask"), "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn why_names_the_answer_like_any_decision_of_the_merged_task() {
    let e = env().await;
    let t = start_task(&e, "Cache greetings").await;
    crate::common::edit_greeter(&e, &t);
    let (_, asked) = ask(&e, &t, question("workspace")).await;
    let (status, body) = answer(
        &e,
        asked["id"].as_str().unwrap(),
        json!({"option": "Postgres", "by": "alice"}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = crate::common::submit(&e, &t).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/validate", id(&t)),
        Some(json!({"agent": "bob", "command": "pytest", "exit_code": 0})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/review", id(&t)),
        Some(json!({"reviewer": "bob", "verdict": "approve"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/merge", id(&t)),
        Some(json!({"agent": "bob"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, why) = call(
        &e.app,
        "GET",
        "/v1/repos/demo/why?path=app/greeter.py&line=7",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{why}");
    let records = why["records"].as_array().unwrap();
    assert!(
        records.iter().any(|r| r["kind"] == "decision"
            && r["summary"] == "Which database should the cache use? → Postgres"),
        "{why}"
    );
    assert!(records.iter().all(|r| r["kind"] != "question"), "{why}");
}

async fn record(e: &crate::common::Env, t: &Value, body: Value) -> (StatusCode, Value) {
    call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/records", id(t)),
        Some(body),
    )
    .await
}

#[tokio::test(flavor = "multi_thread")]
async fn an_agent_does_not_answer_its_own_question_under_another_case_or_an_alias() {
    let e = env().await;
    let t = start_task(&e, "Cache greetings").await;
    let (status, body) = call(
        &e.app,
        "PUT",
        "/v1/agents/careful",
        Some(json!({"aliases": ["claude"]})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (_, asked) = ask(&e, &t, question("workspace")).await;
    let q = asked["id"].as_str().unwrap();
    for by in ["Claude", "careful", "CAREFUL"] {
        let (status, own) = answer(&e, q, json!({"option": "SQLite", "by": by})).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{by}: {own}");
        assert!(own.to_string().contains("its own question"), "{by}: {own}");
    }
    let (status, body) = answer(&e, q, json!({"option": "SQLite", "by": "alice"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_persons_answer_is_corrected_only_by_answering_again() {
    let e = env().await;
    let t = start_task(&e, "Cache greetings").await;
    let (_, asked) = ask(&e, &t, question("workspace")).await;
    let q = asked["id"].as_str().unwrap();
    let (_, answered) = answer(&e, q, json!({"option": "SQLite", "by": "alice"})).await;
    let first = answered["decision"]["id"].as_str().unwrap().to_owned();

    let (status, body) = record(
        &e,
        &t,
        json!({"kind": "decision", "summary": "Use Postgres", "agent": "claude",
                "supersedes": first}),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(body.to_string().contains("av answer"), "{body}");
    let (status, body) = record(
        &e,
        &t,
        json!({"kind": "note", "summary": "never mind", "agent": "alice", "supersedes": first}),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");

    let (status, again) = answer(&e, q, json!({"option": "Postgres", "by": "bob"})).await;
    assert_eq!(status, StatusCode::CONFLICT, "{again}");
    assert!(again.to_string().contains("--correct"), "{again}");
    let (status, corrected) = answer(
        &e,
        q,
        json!({"option": "Postgres", "by": "bob", "correct": true}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{corrected}");
    assert_eq!(
        corrected["decision"]["supersedes"],
        first.as_str(),
        "{corrected}"
    );
    let second = corrected["decision"]["id"].as_str().unwrap();
    let (status, body) = record(
        &e,
        &t,
        json!({"kind": "decision", "summary": "SQLite after all", "agent": "claude",
                "supersedes": second}),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    let (status, own) = answer(
        &e,
        q,
        json!({"option": "SQLite", "by": "claude", "correct": true}),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{own}");
}

#[tokio::test(flavor = "multi_thread")]
async fn an_answer_is_corrected_at_most_64_times_and_each_correction_stays_a_persons_answer() {
    let e = env().await;
    let t = start_task(&e, "Cache greetings").await;
    let (_, asked) = ask(&e, &t, question("workspace")).await;
    let q = asked["id"].as_str().unwrap();
    let (status, body) = answer(&e, q, json!({"option": "SQLite", "by": "alice"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let mut live = body["decision"]["id"].as_str().unwrap().to_owned();
    for n in 1..=64 {
        let option = if n % 2 == 0 { "SQLite" } else { "Postgres" };
        let (status, body) = answer(
            &e,
            q,
            json!({"option": option, "by": "alice", "correct": true}),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "correction {n}: {body}");
        assert_eq!(body["decision"]["supersedes"], live.as_str(), "{body}");
        live = body["decision"]["id"].as_str().unwrap().to_owned();
    }

    let (status, body) = record(
        &e,
        &t,
        json!({"kind": "note", "summary": "never mind", "agent": "claude", "supersedes": live}),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(body.to_string().contains("a person's answer"), "{body}");

    let (status, body) = answer(
        &e,
        q,
        json!({"option": "Postgres", "by": "alice", "correct": true}),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(body.to_string().contains("64 times"), "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn only_the_asking_agent_withdraws_a_question() {
    let e = env().await;
    let t = start_task(&e, "Cache greetings").await;
    let (_, asked) = ask(&e, &t, question("workspace")).await;
    let q = asked["id"].as_str().unwrap();
    let (status, body) = record(
        &e,
        &t,
        json!({"kind": "note", "summary": "not needed", "agent": "grok", "supersedes": q}),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(body.to_string().contains("claude"), "{body}");
    let (status, body) = record(
        &e,
        &t,
        json!({"kind": "note", "summary": "not needed", "agent": "CLAUDE", "supersedes": q}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_finished_tasks_questions_are_neither_open_nor_answered() {
    let e = env().await;
    let t = start_task(&e, "Cache greetings").await;
    let (_, asked) = ask(&e, &t, question("workspace")).await;
    let q = asked["id"].as_str().unwrap();
    let (status, body) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/abandon", id(&t)),
        Some(json!({"agent": "claude"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (_, open) = call(&e.app, "GET", "/v1/questions", None).await;
    assert_eq!(open["questions"], json!([]), "{open}");
    let (status, refused) = answer(&e, q, json!({"option": "SQLite", "by": "alice"})).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{refused}");
    let said = refused.to_string();
    assert!(
        said.contains("finished") && said.contains("new task"),
        "{said}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn a_question_on_the_record_route_is_refused_before_any_snapshot() {
    let e = env().await;
    let t = start_task(&e, "Cache greetings").await;
    crate::common::edit_greeter(&e, &t);
    let (_, before) = call(&e.app, "GET", &format!("/v1/tasks/{}", id(&t)), None).await;
    let (status, body) = record(
        &e,
        &t,
        json!({"kind": "question", "summary": "Postgres?", "agent": "claude"}),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    let (_, after) = call(&e.app, "GET", &format!("/v1/tasks/{}", id(&t)), None).await;
    assert_eq!(
        before["task"]["head_change"], after["task"]["head_change"],
        "{after}"
    );
}

/// Says how each supersede it is asked about was meant: an answer or not.
#[derive(Default)]
struct Answers(std::sync::Mutex<Vec<String>>);

#[async_trait::async_trait]
impl margincode_daemon::MergeGuard for Answers {
    async fn check(&self, action: margincode_daemon::Guarded<'_>) -> Result<(), String> {
        if let margincode_daemon::Guarded::MemoryWrite {
            action: margincode_daemon::MemoryAction::Supersede { record, answer },
            by,
            ..
        } = action
        {
            self.0
                .lock()
                .unwrap()
                .push(format!("{} {} answer={answer}", by, record.kind.as_str()));
        }
        Ok(())
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn a_guard_tells_an_answer_from_a_withdrawal() {
    let guard = std::sync::Arc::new(Answers::default());
    let g = guard.clone();
    let e = crate::common::env_with(move |s| s.with_merge_guard(g)).await;
    let t = start_task(&e, "Cache greetings").await;
    let (_, first) = ask(&e, &t, question("workspace")).await;
    let (_, second) = ask(&e, &t, question("workspace")).await;
    let (status, body) = answer(
        &e,
        first["id"].as_str().unwrap(),
        json!({"option": "SQLite", "by": "alice"}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = answer(
        &e,
        first["id"].as_str().unwrap(),
        json!({"option": "Postgres", "by": "bob", "correct": true}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = record(
        &e,
        &t,
        json!({"kind": "note", "summary": "not needed", "agent": "claude",
                "supersedes": second["id"]}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(
        *guard.0.lock().unwrap(),
        [
            "alice question answer=true",
            "bob decision answer=true",
            "claude question answer=false"
        ]
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn an_answer_names_an_option_by_its_text_before_its_number() {
    let e = env().await;
    let t = start_task(&e, "Cache greetings").await;
    let mut retries = question("workspace");
    retries["question"] = json!("How many retries?");
    retries["options"] = json!(["2", "5"]);
    retries["recommended"] = json!("2");
    let (status, asked) = ask(&e, &t, retries.clone()).await;
    assert_eq!(status, StatusCode::OK, "{asked}");
    assert_eq!(
        asked["told"], "Carry on with 2. Record what you build on it.",
        "{asked}"
    );
    let (status, answered) = answer(
        &e,
        asked["id"].as_str().unwrap(),
        json!({"option": "2", "by": "alice"}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{answered}");
    assert_eq!(
        answered["decision"]["summary"], "How many retries? → 2",
        "{answered}"
    );
    retries["recommended"] = json!("3");
    let (status, refused) = ask(&e, &t, retries).await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{refused}");
}

#[tokio::test(flavor = "multi_thread")]
async fn the_owner_a_task_was_handed_to_withdraws_its_open_question() {
    let e = env().await;
    let t = start_task(&e, "Cache greetings").await;
    let (_, asked) = ask(&e, &t, question("workspace")).await;
    let q = asked["id"].as_str().unwrap();
    crate::common::edit_greeter(&e, &t);
    crate::common::record_decision(&e, &t).await;
    let (status, body) = crate::common::handoff(&e, &t, "claude").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = crate::common::claim(&e, &t, "codex").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = record(
        &e,
        &t,
        json!({"kind": "note", "summary": "not needed", "agent": "grok", "supersedes": q}),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    let (status, body) = record(
        &e,
        &t,
        json!({"kind": "note", "summary": "not needed", "agent": "Codex", "supersedes": q}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
}

async fn set_level(e: &crate::common::Env, name: &str, body: Value) {
    let (status, body) = call(&e.app, "PUT", &format!("/v1/agents/{name}"), Some(body)).await;
    assert_eq!(status, StatusCode::OK, "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_question_is_asked_by_the_tasks_owner_and_takes_the_owners_level() {
    let e = env().await;
    let t = start_task(&e, "Cache greetings").await;
    set_level(&e, "claude", json!({"ask_level": "ask_first"})).await;
    set_level(&e, "grok", json!({"ask_level": "carry_on"})).await;

    let mut forged = question("outside");
    forged["agent"] = json!("grok");
    let (status, refused) = ask(&e, &t, forged).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{refused}");
    let said = refused.to_string();
    assert!(said.contains("claude") && said.contains("owner"), "{said}");
    assert!(
        records(&e, &t)
            .await
            .iter()
            .all(|r| r["kind"] != "question"),
        "a refused question was written"
    );

    // the owner's name in another case is the owner, and its level is the owner's
    let mut cased = question("workspace");
    cased["agent"] = json!("CLAUDE");
    let (status, parked) = ask(&e, &t, cased).await;
    assert_eq!(status, StatusCode::OK, "{parked}");
    assert_eq!(parked["level"], "ask_first", "{parked}");
    assert_eq!(parked["instruction"], "park", "{parked}");

    // no name asks as the owner
    let mut unnamed = question("workspace");
    unnamed.as_object_mut().unwrap().remove("agent");
    let (status, parked) = ask(&e, &t, unnamed).await;
    assert_eq!(status, StatusCode::OK, "{parked}");
    assert_eq!(parked["instruction"], "park", "{parked}");

    let asked: Vec<Value> = records(&e, &t)
        .await
        .into_iter()
        .filter(|r| r["kind"] == "question")
        .collect();
    assert_eq!(asked.len(), 2, "{asked:?}");
    assert!(
        asked.iter().all(|r| r["author_agent"] == "claude"),
        "{asked:?}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn an_alias_of_the_owner_asks_as_the_owner() {
    let e = env().await;
    let t = start_task(&e, "Cache greetings").await;
    set_level(
        &e,
        "careful",
        json!({"aliases": ["claude"], "ask_level": "ask_first"}),
    )
    .await;
    let mut alias = question("workspace");
    alias["agent"] = json!("careful");
    let (status, parked) = ask(&e, &t, alias).await;
    assert_eq!(status, StatusCode::OK, "{parked}");
    assert_eq!(parked["level"], "ask_first", "{parked}");
    assert_eq!(parked["instruction"], "park", "{parked}");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_task_with_no_owner_asks_nothing() {
    let e = env().await;
    let (status, t) = call(
        &e.app,
        "POST",
        "/v1/tasks",
        Some(
            json!({"repo": "demo", "title": "Cache greetings", "goal": "Make hello louder",
                    "agent": " ", "session": "s-1"}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{t}");
    let (status, refused) = ask(&e, &t, question("workspace")).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{refused}");
    assert!(refused.to_string().contains("no owner"), "{refused}");
}
