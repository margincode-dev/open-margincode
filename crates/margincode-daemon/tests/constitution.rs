use std::sync::Mutex;

use crate::common::*;
use margincode_core::{ArticleChange, ConventionId, IntentStore as _};
use margincode_daemon::{Guarded, MemoryAction, RuleChange};

async fn article(
    e: &Env,
    title: &str,
    text: &str,
    check: Option<&str>,
    by: &str,
) -> (StatusCode, Value) {
    call(
        &e.app,
        "POST",
        "/v1/repos/demo/constitution/articles",
        Some(
            json!({"title": title, "text": text, "check": check, "by": by, "reason": "we agreed"}),
        ),
    )
    .await
}

async fn show(e: &Env, query: &str) -> Value {
    let (status, body) = call(
        &e.app,
        "GET",
        &format!("/v1/repos/demo/constitution{query}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body
}

#[tokio::test(flavor = "multi_thread")]
async fn a_person_adds_amends_and_repeals_articles_over_http() {
    let e = env().await;
    let (status, body) = article(
        &e,
        "Never log a key",
        "No provider key in a log line.",
        None,
        "user",
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["amendment"]["handle"], "art:1");
    assert_eq!(body["amendment"]["version"], 1);
    let (status, body) = call(
        &e.app,
        "PUT",
        "/v1/repos/demo/constitution/articles/1",
        Some(json!({"check": "true", "by": "user", "reason": "now it is checked"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["amendment"]["action"], "amend");
    assert_eq!(body["constitution"]["articles"][0]["check"], "true");
    assert_eq!(
        body["constitution"]["articles"][0]["title"],
        "Never log a key"
    );
    article(&e, "Second", "Another rule.", None, "user").await;
    let (status, body) = call(
        &e.app,
        "DELETE",
        "/v1/repos/demo/constitution/articles/2",
        Some(json!({"by": "user", "reason": "not needed"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let c = show(&e, "?history=1").await;
    assert_eq!(c["version"], 4);
    assert_eq!(c["articles"].as_array().unwrap().len(), 1);
    let actions: Vec<&str> = c["history"]
        .as_array()
        .unwrap()
        .iter()
        .map(|h| h["action"].as_str().unwrap())
        .collect();
    assert_eq!(actions, ["add", "amend", "add", "repeal"]);
    assert!(show(&e, "").await.get("history").is_none());

    let (status, body) = article(&e, &"t".repeat(61), "x", None, "user").await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/repos/demo/constitution/articles",
        Some(json!({"title": "t", "text": "x", "by": "user", "reason": ""})),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    let (status, body) = call(
        &e.app,
        "PUT",
        "/v1/repos/demo/constitution/articles/9",
        Some(json!({"text": "x", "by": "user", "reason": "why"})),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND, "{body}");
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/repos/nope/constitution/articles",
        Some(json!({"title": "t", "text": "x", "by": "user", "reason": "why"})),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND, "{body}");
    assert_eq!(show(&e, "").await["version"], 4);
}

#[tokio::test(flavor = "multi_thread")]
async fn an_agents_name_cannot_amend_the_constitution() {
    let e = env().await;
    start_task(&e, "Improve greeting").await; // its agent is `claude`
    e.store
        .put_profile(&margincode_core::routing::Profile {
            name: "muse".into(),
            best: vec![],
            can: vec![],
            cost_rank: 1,
            aliases: vec!["muse-cli".into()],
            max_open: None,
            managed_by: None,
        })
        .await
        .unwrap();
    for by in ["claude", "Claude", "muse", "muse-cli"] {
        let (status, body) = article(&e, "Rule", "text", None, by).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{by}: {body}");
        assert!(
            body["error"]
                .as_str()
                .unwrap()
                .contains("the constitution is amended by a person; an agent proposes a rule with a decision record"),
            "{body}"
        );
    }
    let (status, body) = article(&e, "Rule", "text", None, "vinh").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = call(
        &e.app,
        "DELETE",
        "/v1/repos/demo/constitution/articles/1",
        Some(json!({"by": "claude", "reason": "gone"})),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert_eq!(show(&e, "").await["articles"].as_array().unwrap().len(), 1);
}

/// Refuses every rule change and remembers what it was asked.
#[derive(Default)]
struct Board(Mutex<Vec<String>>);

#[async_trait::async_trait]
impl margincode_daemon::MergeGuard for Board {
    async fn check(&self, action: Guarded<'_>) -> Result<(), String> {
        let Guarded::MemoryWrite {
            action: MemoryAction::Rule { kind, change },
            by,
            ..
        } = action
        else {
            return Ok(());
        };
        let what = match change {
            RuleChange::Add {
                number,
                title,
                scope_glob,
                check,
                ..
            } => {
                format!("add {number:?} {title:?} {scope_glob} {check:?}")
            }
            RuleChange::Amend {
                number,
                title,
                check,
                ..
            } => format!("amend {number} {title} {check:?}"),
            RuleChange::Remove { number, convention } => {
                format!("remove {number:?} {}", convention.handle())
            }
        };
        self.0
            .lock()
            .unwrap()
            .push(format!("{kind:?} {what} by {by}"));
        Err("rules are written on the board".into())
    }
}

fn board(e: &Env) -> Vec<String> {
    let guard = e.state.extension::<Board>().unwrap();
    std::mem::take(&mut *guard.0.lock().unwrap())
}

async fn with_board() -> Env {
    env_with(|s| {
        let b = Arc::new(Board::default());
        s.with_merge_guard(b.clone()).with_extension(b)
    })
    .await
}

#[tokio::test(flavor = "multi_thread")]
async fn the_guard_is_asked_before_an_amendment_and_its_refusal_writes_nothing() {
    let e = with_board().await;
    let (status, body) = article(&e, "Rule", "text", Some("make lint"), "user").await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(
        body["error"]
            .as_str()
            .unwrap()
            .contains("rules are written on the board"),
        "{body}"
    );
    assert_eq!(
        board(&e),
        [r#"Article add Some(1) Some("Rule") ** Some("make lint") by user"#]
    );
    assert_eq!(show(&e, "").await["version"], 0);

    // an extension writes through `AppState`, which asks no guard and knows no agent names
    let add = |title: &str| ArticleChange::Add {
        title: title.into(),
        text: "text".into(),
        check: None,
    };
    e.state
        .amend_constitution("demo", &add("Own"), "claude", "from the board")
        .await
        .unwrap();
    e.state
        .amend_shared_constitution(&add("Shared"), "owner@org", "for every repo")
        .await
        .unwrap();
    assert!(board(&e).is_empty());
    let c = show(&e, "?history=1").await;
    let handles: Vec<&str> = c["articles"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a["handle"].as_str().unwrap())
        .collect();
    assert_eq!(handles, ["org:1", "art:1"]);
    assert_eq!(
        (c["version"].as_i64(), c["shared_version"].as_i64()),
        (Some(1), Some(1))
    );
    assert_eq!(c["history"].as_array().unwrap().len(), 2);

    for (method, body) in [
        ("PUT", json!({"text": "new", "by": "user", "reason": "why"})),
        ("DELETE", json!({"by": "user", "reason": "why"})),
    ] {
        let (status, _) = call(
            &e.app,
            method,
            "/v1/repos/demo/constitution/articles/1",
            Some(body),
        )
        .await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    }
    let asked = board(&e);
    assert_eq!(asked.len(), 2, "{asked:?}");
    assert!(
        asked[0].starts_with("Article amend 1 Own None"),
        "{asked:?}"
    );
    assert!(
        asked[1].starts_with("Article remove Some(1) art:1"),
        "{asked:?}"
    );
    // no route writes the shared level: its numbers are not the repository's
    let (status, _) = call(
        &e.app,
        "PUT",
        "/v1/repos/demo/constitution/articles/2",
        Some(json!({"text": "new", "by": "user", "reason": "why"})),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test(flavor = "multi_thread")]
async fn an_articles_row_is_not_removed_as_a_convention() {
    let e = env().await;
    let (_, body) = article(&e, "Rule", "text", None, "user").await;
    let row = body["amendment"]["convention_id"].as_i64().unwrap();
    let (status, body) = call(
        &e.app,
        "DELETE",
        &format!("/v1/repos/demo/conventions/{row}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(body["error"].as_str().unwrap().contains("art:1"), "{body}");
    assert_eq!(show(&e, "").await["articles"].as_array().unwrap().len(), 1);
    let (_, list) = call(&e.app, "GET", "/v1/repos/demo/conventions", None).await;
    assert!(list["conventions"].as_array().unwrap().is_empty(), "{list}");
    assert!(
        e.store
            .remove_convention("demo", ConventionId(row))
            .await
            .is_err()
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn an_articles_check_gates_approval_whatever_the_task_changed() {
    let e = env().await;
    article(
        &e,
        "Lint passes",
        "Run the linter.",
        Some("make lint"),
        "user",
    )
    .await;
    e.state
        .amend_shared_constitution(
            &ArticleChange::Add {
                title: "Org rule".into(),
                text: "No check.".into(),
                check: None,
            },
            "owner",
            "for every repo",
        )
        .await
        .unwrap();
    let t = start_task(&e, "Improve greeting").await;
    edit_greeter(&e, &t);
    record_decision(&e, &t).await;
    let (status, body) = submit(&e, &t).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (_, checks) = call(&e.app, "GET", &format!("/v1/tasks/{}/checks", id(&t)), None).await;
    let list = checks["checks"].as_array().unwrap();
    assert_eq!(list.len(), 1, "{checks}");
    assert_eq!(list[0]["article"], 1);
    assert_eq!(list[0]["handle"], "art:1");
    assert_eq!(list[0]["command"], "make lint");
    let conv = list[0]["convention_id"].as_i64().unwrap();
    let validate = |checks: Value| {
        let e = &e;
        let t = &t;
        async move {
            let (status, body) = call(
                &e.app,
                "POST",
                &format!("/v1/tasks/{}/validate", id(t)),
                Some(json!({"agent": "claude", "command": "pytest", "exit_code": 0, "checks": checks})),
            )
            .await;
            assert_eq!(status, StatusCode::OK, "{body}");
        }
    };
    let review = || async {
        call(
            &e.app,
            "POST",
            &format!("/v1/tasks/{}/review", id(&t)),
            Some(json!({"reviewer": "reviewer", "verdict": "approve"})),
        )
        .await
    };
    validate(json!([])).await;
    let (status, body) = review().await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(
        body["error"]
            .as_str()
            .unwrap()
            .contains("art:1 check `make lint` not run"),
        "{body}"
    );
    validate(json!([{"convention_id": conv, "exit_code": 0, "command": "make lint"}])).await;
    let (status, body) = review().await;
    assert_eq!(status, StatusCode::OK, "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn rules_get_free_notes_on_the_constitution_and_the_convention_list() {
    let e = env().await;
    article(
        &e,
        "Docs",
        "Keep the  docs SHORT.",
        Some("make lint"),
        "user",
    )
    .await;
    for (glob, text, check) in [
        ("docs/**", "keep the docs short.", None),
        ("app/**", "Lint it", Some("make lint")),
    ] {
        let (status, body) = call(
            &e.app,
            "POST",
            "/v1/repos/demo/conventions",
            Some(json!({"scope_glob": glob, "text": text, "required": check.is_some(), "check": check})),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
    }
    let (_, list) = call(&e.app, "GET", "/v1/repos/demo/conventions", None).await;
    let notes: Vec<&str> = list["notes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|n| n.as_str().unwrap())
        .collect();
    assert_eq!(notes.len(), 3, "{notes:?}");
    assert!(
        notes
            .iter()
            .any(|n| n.contains("(docs/**) matches no file on main")),
        "{notes:?}"
    );
    assert!(
        notes
            .iter()
            .any(|n| n.starts_with("art:1 and conv:") && n.ends_with("have the same text")),
        "{notes:?}"
    );
    assert!(
        notes
            .iter()
            .any(|n| n.contains("run the same check `make lint`")),
        "{notes:?}"
    );
    assert_eq!(show(&e, "").await["notes"], list["notes"]);
}

#[tokio::test(flavor = "multi_thread")]
async fn a_repo_with_a_blank_name_is_refused() {
    let e = env().await;
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/repos",
        Some(json!({"name": " ", "git_source": e.src_path})),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
}
