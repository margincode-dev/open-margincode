//! Learn from use: the outcome of a merged task's packs (`pack_outcomes`) and what its agent
//! opened (`task_reads`).

use crate::common::*;

const LOUD: &str = "from util import format_name\n\nclass Greeter:\n    def hello(self, name):\n        return format_name(name) + \"!\"\n";
const UPPER: &str = "def format_name(n):\n    return n.upper()\n";

async fn post(e: &Env, t: &Value, op: &str, body: Value) -> (StatusCode, Value) {
    call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/{op}", id(t)),
        Some(body),
    )
    .await
}

/// A started task with greeter.py and util.py changed and app/extra.py added, submitted and
/// approved.
async fn approved(e: &Env, title: &str) -> Value {
    let t = start_task(e, title).await;
    let ws = ws_dir(e, &t);
    std::fs::write(ws.join("app/greeter.py"), LOUD).unwrap();
    std::fs::write(ws.join("app/util.py"), UPPER).unwrap();
    std::fs::write(ws.join("app/extra.py"), "def extra():\n    return 1\n").unwrap();
    record_decision(e, &t).await;
    let (status, body) = submit(e, &t).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = post(
        e,
        &t,
        "validate",
        json!({"agent": "claude", "command": "pytest", "exit_code": 0}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = post(
        e,
        &t,
        "review",
        json!({"reviewer": "reviewer", "verdict": "approve"}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    t
}

async fn merge(e: &Env, t: &Value) -> Value {
    let (status, body) = post(e, t, "merge", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["status"], "merged", "{body}");
    body
}

async fn outcomes(e: &Env) -> Value {
    let (status, body) = call(
        &e.app,
        "GET",
        "/v1/repos/demo/context/outcomes?tasks=1",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body
}

async fn sql(e: &Env, q: &str) {
    sqlx::query(q).execute(e.store.pool()).await.unwrap();
}

async fn reads(e: &Env, t: &Value) -> Vec<(String, i32, bool)> {
    sqlx::query_as(
        "SELECT path, reads, expanded FROM task_reads WHERE task_id = $1::uuid ORDER BY path",
    )
    .bind(id(t))
    .fetch_all(e.store.pool())
    .await
    .unwrap()
}

/// The first `sym:N` a pack of the task shows, and that symbol's path.
async fn a_symbol(e: &Env, t: &Value) -> (String, String) {
    let (_, pack) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/context?budget=3000&session=s-2", id(t)),
        None,
    )
    .await;
    let rcp = pack["receipt"].as_str().unwrap();
    let (_, receipt) = call(&e.app, "GET", &format!("/v1/receipts/{rcp}"), None).await;
    let item = receipt["items"]
        .as_array()
        .unwrap()
        .iter()
        .find(|i| i["id"].as_str().unwrap().starts_with("sym:"))
        .expect("a code item")
        .clone();
    (
        item["id"].as_str().unwrap().to_owned(),
        item["source"][0].as_str().unwrap().to_owned(),
    )
}

#[tokio::test(flavor = "multi_thread")]
async fn a_merge_stores_the_outcome_of_the_tasks_packs() {
    let e = env().await;
    let t = approved(&e, "Louder hello").await;
    merge(&e, &t).await;

    let body = outcomes(&e).await;
    assert_eq!(body["measured"], 1, "{body}");
    assert_eq!(body["reverted"], 0, "{body}");
    assert_eq!(body["no_reads"], 1, "{body}");
    let row = &body["rows"][0];
    assert_eq!(row["task_id"], id(&t).as_str());
    assert_eq!(row["changed_source"], "vcs");
    assert_eq!(row["files_changed"], 2, "app/extra.py was added: {row}");
    assert_eq!(row["first_budget"], 800, "{row}");
    assert!(row["packs"].as_i64().unwrap() >= 1, "{row}");
    assert_eq!(row["reverted"], false);
    let (tokens, on_changed) = (
        row["code_tokens"].as_i64().unwrap(),
        row["code_tokens_changed"].as_i64().unwrap(),
    );
    assert!(tokens > 0 && on_changed <= tokens, "{row}");
    assert_eq!(
        row["code_tokens_used"], row["code_tokens_changed"],
        "nothing opened: {row}"
    );
    let missed: Vec<&str> = row["missed_paths"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| p.as_str().unwrap())
        .collect();
    assert!(
        missed
            .iter()
            .all(|p| ["app/greeter.py", "app/util.py"].contains(p)),
        "{row}"
    );
    let version: (i32,) = sqlx::query_as(
        "SELECT packer_version FROM context_receipts WHERE task_id = $1::uuid
         ORDER BY created_at LIMIT 1",
    )
    .bind(id(&t))
    .fetch_one(e.store.pool())
    .await
    .unwrap();
    assert_eq!(row["packer_version"], version.0);
    let v = &body["by_packer"][0];
    assert_eq!(v["packer_version"], version.0, "{body}");
    assert_eq!(v["tasks"], 1);
    assert_eq!(v["files_changed"], 2.0);

    let (status, plain) = call(&e.app, "GET", "/v1/repos/demo/context/outcomes", None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        plain.get("rows").is_none(),
        "rows only with tasks=1: {plain}"
    );
    let (status, _) = call(&e.app, "GET", "/v1/repos/nope/context/outcomes", None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test(flavor = "multi_thread")]
async fn a_reverted_merge_keeps_its_outcome_and_leaves_the_totals() {
    let e = env().await;
    let t = approved(&e, "Louder hello").await;
    merge(&e, &t).await;
    let (status, body) = post(
        &e,
        &t,
        "revert",
        json!({"agent": "merger", "reason": "broke main"}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let body = outcomes(&e).await;
    assert_eq!(body["measured"], 0, "{body}");
    assert_eq!(body["reverted"], 1, "{body}");
    assert_eq!(body["by_packer"], json!([]), "{body}");
    assert_eq!(body["rows"][0]["reverted"], true, "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_merge_whose_outcome_cannot_be_stored_still_merges_and_backfill_measures_it_later() {
    let e = env().await;
    let t = approved(&e, "Louder hello").await;
    sql(&e, "ALTER TABLE pack_outcomes RENAME TO pack_outcomes_away").await;
    let merged = merge(&e, &t).await;
    assert_eq!(merged["changed"].as_array().unwrap().len(), 3, "{merged}");
    let (_, task) = call(&e.app, "GET", &format!("/v1/tasks/{}", id(&t)), None).await;
    assert_eq!(task["task"]["status"], "merged");
    sql(&e, "ALTER TABLE pack_outcomes_away RENAME TO pack_outcomes").await;
    assert_eq!(outcomes(&e).await["measured"], 0);

    let (status, body) = call(&e.app, "POST", "/v1/repos/demo/context/backfill", None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["computed"], 1, "{body}");
    assert_eq!(body["failed"], json!([]));
    assert_eq!(body["more"], false);
    let body = outcomes(&e).await;
    assert_eq!(body["measured"], 1, "{body}");
    assert_eq!(body["rows"][0]["changed_source"], "vcs");
    assert_eq!(body["rows"][0]["files_changed"], 2);

    let (_, again) = call(&e.app, "POST", "/v1/repos/demo/context/backfill", None).await;
    assert_eq!(
        again["computed"], 0,
        "a task with an outcome is left alone: {again}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn backfill_falls_back_to_the_touched_paths_when_the_merge_is_not_known() {
    let e = env().await;
    let t = approved(&e, "Louder hello").await;
    merge(&e, &t).await;
    sql(&e, "DELETE FROM pack_outcomes").await;
    sql(&e, "UPDATE tasks SET pre_merge_commit = NULL").await;
    let (status, body) = call(&e.app, "POST", "/v1/repos/demo/context/backfill", None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["computed"], 1, "{body}");
    let row = &outcomes(&e).await["rows"][0];
    assert_eq!(row["changed_source"], "touched", "{row}");
    assert_eq!(row["files_changed"], 2, "{row}");
}

#[tokio::test(flavor = "multi_thread")]
async fn the_pre_read_route_and_expand_note_what_the_agent_opened() {
    let e = env().await;
    let t = start_task(&e, "Louder hello").await;
    let at =
        |path: &str, session: &str| format!("/v1/tasks/{}/records/at?path={path}{session}", id(&t));
    let (status, _) = call(&e.app, "GET", &at("./app/greeter.py", "&session=s-1"), None).await;
    assert_eq!(status, StatusCode::OK);
    call(&e.app, "GET", &at("app/greeter.py", "&session=s-1"), None).await;
    let (status, _) = call(&e.app, "GET", &at("app/notes.py", ""), None).await;
    assert_eq!(status, StatusCode::OK);
    let (sym, path) = a_symbol(&e, &t).await;
    let (status, full) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/items/{sym}?session=s-2", id(&t)),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{full}");

    let want = if path == "app/greeter.py" {
        vec![("app/greeter.py".to_owned(), 3, true)]
    } else {
        let mut w = vec![("app/greeter.py".to_owned(), 2, false), (path, 1, true)];
        w.sort();
        w
    };
    eventually("the reads to be noted", async || {
        reads(&e, &t).await == want
    })
    .await;
}

#[tokio::test(flavor = "multi_thread")]
async fn a_read_that_cannot_be_noted_changes_no_answer() {
    let e = env().await;
    let other = start_task(&e, "Other work").await;
    record_decision(&e, &other).await;
    let t = start_task(&e, "Louder hello").await;
    let uri = |session: &str| {
        format!(
            "/v1/tasks/{}/records/at?path=app/greeter.py&session={session}",
            id(&t)
        )
    };
    let (sym, _) = a_symbol(&e, &t).await;
    let expand = format!("/v1/tasks/{}/items/{sym}", id(&t));
    let before = (
        call(&e.app, "GET", &uri("s-a"), None).await,
        call(&e.app, "GET", &expand, None).await,
    );
    assert!(
        before.0.1["markdown"]
            .as_str()
            .unwrap()
            .contains("hello shouts"),
        "{}",
        before.0.1
    );
    sql(&e, "DROP TABLE task_reads").await;
    let after = (
        call(&e.app, "GET", &uri("s-b"), None).await,
        call(&e.app, "GET", &expand, None).await,
    );
    assert_eq!(after, before);
}

#[tokio::test(flavor = "multi_thread")]
async fn opened_files_count_as_used_and_as_opened_but_not_sent() {
    let e = env().await;
    let t = approved(&e, "Louder hello").await;
    e.store
        .note_read(
            margincode_core::TaskId(id(&t).parse().unwrap()),
            "app/elsewhere.py",
            false,
        )
        .await
        .unwrap();
    merge(&e, &t).await;
    let body = outcomes(&e).await;
    let row = &body["rows"][0];
    assert_eq!(row["files_opened"], 1, "{row}");
    assert_eq!(row["opened_not_sent"], 1, "{row}");
    assert_eq!(row["opened_not_sent_paths"], json!(["app/elsewhere.py"]));
    assert_eq!(body["no_reads"], 0, "{body}");
}

/// The outcome counts an item a later pack of the same session sends at a tier at or below an
/// earlier one as a back-reference: this is the ledger's rule, as the receipts show it.
#[tokio::test(flavor = "multi_thread")]
async fn a_second_pack_of_a_session_sends_earlier_items_at_no_higher_tier() {
    let e = env().await;
    let t = start_task(&e, "Louder hello").await;
    let receipt = async |rcp: &str| {
        let (_, r) = call(&e.app, "GET", &format!("/v1/receipts/{rcp}"), None).await;
        r["items"].as_array().unwrap().clone()
    };
    let first = receipt(t["pack"]["receipt"].as_str().unwrap()).await;
    let (_, second) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/context?session=s-1", id(&t)),
        None,
    )
    .await;
    let second = receipt(second["receipt"].as_str().unwrap()).await;
    let tier = |t: &Value| {
        ["handle", "summary", "full"]
            .iter()
            .position(|x| t == x)
            .unwrap()
    };
    let mut again = 0;
    for item in &second {
        if let Some(old) = first.iter().find(|o| o["id"] == item["id"]) {
            again += 1;
            assert!(
                tier(&item["tier"]) <= tier(&old["tier"]),
                "{item} after {old}"
            );
        }
    }
    assert!(
        again > 0,
        "the second pack refers back to the first: {second:?}"
    );
}
