mod common;

use common::*;
use scholion_core::IntentStore as _;
use scholion_core::Vcs as _;

#[tokio::test(flavor = "multi_thread")]
async fn post_tasks_returns_workspace_pack_and_host_path() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    let path = t["task"]["workspace_path"].as_str().unwrap();
    assert_eq!(path, format!("{HOST_WS}/demo/improve-greeting"));
    assert!(
        ws_dir(&e, &t).join("app/greeter.py").exists(),
        "workspace materialized under the container root"
    );
    let md = t["pack"]["markdown"].as_str().unwrap();
    assert!(
        md.contains("## Task") && md.contains("Make hello louder"),
        "{md}"
    );
    assert!(
        md.contains("repo map") && md.contains("app/"),
        "cold start orients the agent:\n{md}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn checkpoint_then_context_reflects_edit() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    std::fs::write(
        ws_dir(&e, &t).join("app/greeter.py"),
        "from util import format_name\n\nclass Greeter:\n    def hello(self, name):\n        return format_name(name)\n\n    def shout_loudly(self, name):\n        return self.hello(name).upper()\n",
    )
    .unwrap();
    let (status, cp) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/checkpoint", id(&t)),
        Some(json!({})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{cp}");
    assert_eq!(cp["changed"], json!(["app/greeter.py"]));
    let (status, pack) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/context?budget=3000&session=s-2", id(&t)),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let md = pack["markdown"].as_str().unwrap();
    assert!(md.contains("Greeter.shout_loudly"), "{md}");
    let (_, task) = call(&e.app, "GET", &format!("/v1/tasks/{}", id(&t)), None).await;
    assert_eq!(task["changed"], json!(["app/greeter.py"]));
}

#[tokio::test(flavor = "multi_thread")]
async fn checkpoint_removes_deleted_files_from_the_index() {
    let e = env().await;
    let t = start_task(&e, "Cleanup").await;
    // positive control: before the delete the symbol IS served, so the later absence means something
    let (_, before) = call(
        &e.app,
        "GET",
        &format!(
            "/v1/tasks/{}/context?budget=3000&focus=format_name&session=s-0",
            id(&t)
        ),
        None,
    )
    .await;
    assert!(
        before["markdown"].as_str().unwrap().contains("format_name"),
        "{before}"
    );
    std::fs::remove_file(ws_dir(&e, &t).join("app/util.py")).unwrap();
    let (status, cp) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/checkpoint", id(&t)),
        Some(json!({})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{cp}");
    assert_eq!(cp["changed"], json!(["app/util.py"]));
    let (_, pack) = call(
        &e.app,
        "GET",
        &format!(
            "/v1/tasks/{}/context?budget=3000&focus=format_name&session=s-3",
            id(&t)
        ),
        None,
    )
    .await;
    // other symbols may still be served (the goal's words find Greeter.hello), but nothing from
    // the deleted file
    let md = pack["markdown"].as_str().unwrap();
    assert!(
        !md.contains("app/util.py:") && !md.contains("def format_name"),
        "deleted symbol must not be served: {pack}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn record_then_context_contains_attempt() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    let (status, rec) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/records", id(&t)),
        Some(json!({"kind": "attempt", "summary": "tried caching hello; breaks tests", "agent": "claude",
                    "anchors": [{"path": "app/greeter.py", "symbol": "Greeter.hello"}]})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{rec}");
    let (_, pack) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/context?budget=3000&session=s-4", id(&t)),
        None,
    )
    .await;
    assert!(
        pack["markdown"]
            .as_str()
            .unwrap()
            .contains("tried caching hello"),
        "{pack}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn record_with_201_char_summary_is_400() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    let (status, body) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/records", id(&t)),
        Some(json!({"kind": "note", "summary": "x".repeat(201), "agent": "claude"})),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
    assert!(body["error"].as_str().unwrap().contains("201"));
}

#[tokio::test(flavor = "multi_thread")]
async fn two_tasks_same_title_get_distinct_workspaces() {
    let e = env().await;
    let a = start_task(&e, "Same title").await;
    let b = start_task(&e, "Same title").await;
    assert_ne!(a["task"]["workspace_path"], b["task"]["workspace_path"]);
    assert_eq!(b["task"]["slug"], "same-title-2");
    assert!(ws_dir(&e, &a).exists() && ws_dir(&e, &b).exists());
}

#[tokio::test(flavor = "multi_thread")]
async fn checkpoint_with_binary_file_succeeds_with_warning() {
    let e = env().await;
    let t = start_task(&e, "Binary").await;
    std::fs::write(
        ws_dir(&e, &t).join("blob.py"),
        [0xffu8, 0xfe, 0x00, 0x01, 0x80],
    )
    .unwrap();
    std::fs::write(
        ws_dir(&e, &t).join("app/ok.py"),
        "def fine():\n    return 1\n",
    )
    .unwrap();
    let (status, cp) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/checkpoint", id(&t)),
        Some(json!({})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{cp}");
    let warnings = cp["warnings"].as_array().unwrap();
    assert!(
        warnings
            .iter()
            .any(|w| w.as_str().unwrap().contains("blob.py")),
        "{cp}"
    );
    let (_, pack) = call(
        &e.app,
        "GET",
        &format!(
            "/v1/tasks/{}/context?budget=3000&focus=fine&session=s-5",
            id(&t)
        ),
        None,
    )
    .await;
    assert!(
        pack["markdown"].as_str().unwrap().contains("fine"),
        "the good file is still indexed"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn concurrent_checkpoints_from_two_tasks_succeed() {
    let e = env().await;
    let a = start_task(&e, "Task A").await;
    let b = start_task(&e, "Task B").await;
    std::fs::write(
        ws_dir(&e, &a).join("a_new.py"),
        "def from_a():\n    return 'a'\n",
    )
    .unwrap();
    std::fs::write(
        ws_dir(&e, &b).join("b_new.py"),
        "def from_b():\n    return 'b'\n",
    )
    .unwrap();
    let (ua, ub) = (
        format!("/v1/tasks/{}/checkpoint", id(&a)),
        format!("/v1/tasks/{}/checkpoint", id(&b)),
    );
    let ((sa, ca), (sb, cb)) = tokio::join!(
        call(&e.app, "POST", &ua, Some(json!({}))),
        call(&e.app, "POST", &ub, Some(json!({})))
    );
    assert_eq!((sa, sb), (StatusCode::OK, StatusCode::OK), "{ca} {cb}");
    assert_eq!(ca["changed"], json!(["a_new.py"]));
    assert_eq!(cb["changed"], json!(["b_new.py"]));
}

#[tokio::test(flavor = "multi_thread")]
async fn expand_route_returns_full_text() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    std::fs::write(
        ws_dir(&e, &t).join("app/util.py"),
        "def format_name(n):\n    return n.title().strip()\n",
    )
    .unwrap();
    call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/checkpoint", id(&t)),
        Some(json!({})),
    )
    .await;
    let (_, pack) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/context?budget=3000&session=s-6", id(&t)),
        None,
    )
    .await;
    let md = pack["markdown"].as_str().unwrap();
    let start = md.find("sym:").expect("a symbol id in the pack");
    let sym: String = md[start..]
        .chars()
        .take_while(|c| !c.is_whitespace())
        .collect();
    let (status, full) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/items/{sym}", id(&t)),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{full}");
    assert!(full["full"].as_str().unwrap().contains("def "), "{full}");
    let (status, _) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/items/bogus:1", id(&t)),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test(flavor = "multi_thread")]
async fn unknown_task_is_404() {
    let e = env().await;
    let ghost = "00000000-0000-4000-8000-000000000000";
    for uri in [
        format!("/v1/tasks/{ghost}"),
        format!("/v1/tasks/{ghost}/context"),
    ] {
        let (status, body) = call(&e.app, "GET", &uri, None).await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{uri}: {body}");
    }
    let (status, _) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{ghost}/checkpoint"),
        Some(json!({})),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let (status, _) = call(
        &e.app,
        "POST",
        "/v1/tasks",
        Some(json!({"repo": "nope", "title": "t", "goal": "g", "agent": "a", "session": "s"})),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND, "unknown repo");
}

#[tokio::test(flavor = "multi_thread")]
async fn path_traversal_in_repo_name_is_a_400_and_creates_nothing() {
    let e = env().await;
    let outside = e.ws.path().parent().unwrap().to_path_buf();
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/repos",
        Some(json!({"name": "../../escape", "git_source": e.src_path})),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
    assert!(!outside.join("escape").exists());
    let (status, _) = call(
        &e.app,
        "POST",
        "/v1/tasks",
        Some(json!({"repo": "../demo", "title": "t", "goal": "g", "agent": "a", "session": "s"})),
    )
    .await;
    assert_eq!(
        status,
        StatusCode::BAD_REQUEST,
        "a task for a traversal repo name is rejected too"
    );
}

// ---- Plan 2 Task 8: semantic wiring and POST /v1/reindex ----

struct FailingSemantic;

#[async_trait::async_trait]
impl scholion_core::Semantic for FailingSemantic {
    async fn search(
        &self,
        _repo: &str,
        _query: &str,
        _limit: usize,
    ) -> scholion_core::Result<Vec<scholion_core::SemanticHit>> {
        Err(scholion_core::AvError::Semantic("down".into()))
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn reindex_without_qdrant_enqueues_and_reports_semantic() {
    let e = env().await;
    start_task(&e, "Improve greeting").await;
    let (status, body) = call(&e.app, "POST", "/v1/reindex", Some(json!({"repo": "demo"}))).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let enqueued = body["enqueued"].as_i64().unwrap();
    assert!(enqueued >= 1, "{body}");
    assert_eq!(body["upserted"], 0, "{body}");
    assert_eq!(body["degraded"], json!(["semantic"]), "{body}");
    assert_eq!(e.store.outbox_count(Some("demo")).await.unwrap(), enqueued);
}

#[tokio::test(flavor = "multi_thread")]
async fn context_with_failing_semantic_is_200_and_marks_degraded() {
    let e = env_with(|s| s.with_semantic(Arc::new(FailingSemantic))).await;
    let t = start_task(&e, "Improve greeting").await;
    let (status, body) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/context?budget=2000&session=s-2", id(&t)),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert!(
        body["degraded"]
            .as_array()
            .unwrap()
            .contains(&json!("semantic")),
        "{body}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn reindex_with_memory_index_upserts_and_clears_outbox() {
    use scholion_semantic::{Embedder, MemoryIndex, ScriptedEmbedder, VectorIndex};
    let embedder = ScriptedEmbedder::new(8);
    let memory = MemoryIndex::new();
    let (emb, mem) = (embedder.clone(), memory.clone());
    let e = env_with(move |s| s.with_index_backend(Arc::new(emb), Arc::new(mem))).await;
    start_task(&e, "Improve greeting").await;
    let (status, body) = call(&e.app, "POST", "/v1/reindex", Some(json!({"repo": "demo"}))).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["degraded"], json!([]), "{body}");
    assert!(body["enqueued"].as_i64().unwrap() >= 1, "{body}");
    assert_eq!(body["upserted"], body["enqueued"], "{body}");
    assert_eq!(e.store.outbox_count(Some("demo")).await.unwrap(), 0);
    // The task's intent record ("Make hello louder") is now searchable.
    let v = embedder
        .embed(&["Make hello louder".to_string()])
        .await
        .unwrap()
        .remove(0);
    let hits = memory
        .search(scholion_core::SemanticKind::Record, "demo", &v, 5)
        .await
        .unwrap();
    assert!(!hits.is_empty(), "records were upserted");
    let chunks = memory
        .search(scholion_core::SemanticKind::CodeChunk, "demo", &v, 50)
        .await
        .unwrap();
    assert!(
        chunks.iter().all(|h| h.task_id.is_none()),
        "a fresh task's base import is not embedded, only main: {chunks:?}"
    );
    assert!(!chunks.is_empty(), "main's code chunks were upserted");
}

// ---- Plan 2b Task 1: receipts ----

#[tokio::test(flavor = "multi_thread")]
async fn context_returns_a_receipt_that_records_can_cite() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    let rcp = t["pack"]["receipt"]
        .as_str()
        .expect("start_task pack has a receipt")
        .to_owned();
    let (status, body) = call(&e.app, "GET", &format!("/v1/receipts/{rcp}"), None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["id"], rcp.as_str());
    assert!(!body["items"].as_array().unwrap().is_empty(), "{body}");

    let (status, rec) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/records", id(&t)),
        Some(json!({"kind": "note", "summary": "worked from that pack", "agent": "claude", "receipt": rcp})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{rec}");
    assert_eq!(rec["receipt"], rcp.as_str(), "{rec}");

    let (status, _) = call(&e.app, "GET", "/v1/receipts/ffffffff", None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

// ---- Plan 2b Task 2: freshness ----

#[tokio::test(flavor = "multi_thread")]
async fn knowledge_on_code_edited_later_is_flagged_after_checkpoint() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    let (status, body) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/records", id(&t)),
        Some(
            json!({"kind": "decision", "summary": "format_name stays title-case", "agent": "claude",
                    "anchors": [{"path": "app/util.py", "symbol": "format_name"}]}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let ctx = |s: &'static str| {
        let app = e.app.clone();
        let tid = id(&t);
        async move {
            let (_, b) = call(
                &app,
                "GET",
                &format!("/v1/tasks/{tid}/context?budget=3000&session={s}"),
                None,
            )
            .await;
            b["markdown"].as_str().unwrap().to_owned()
        }
    };
    assert!(!ctx("before").await.contains("changed since recorded"));
    std::fs::write(
        ws_dir(&e, &t).join("app/util.py"),
        "def format_name(n):\n    return n.upper()\n",
    )
    .unwrap();
    let (status, body) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/checkpoint", id(&t)),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let md = ctx("after").await;
    let line = md
        .lines()
        .find(|l| l.contains("format_name stays title-case"))
        .unwrap_or_else(|| panic!("{md}"));
    assert!(line.contains("(changed since recorded)"), "{line}");
}

#[tokio::test(flavor = "multi_thread")]
async fn record_without_checkpoint_is_not_flagged_changed_after_submit() {
    // MCP-only clients have no hook to checkpoint: the record itself must see the edit.
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    edit_greeter(&e, &t);
    let (status, body) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/records", id(&t)),
        Some(
            json!({"kind": "decision", "summary": "hello shouts with a bang", "agent": "claude",
                    "anchors": [{"path": "app/greeter.py", "symbol": "Greeter.hello"}]}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = submit(&e, &t).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, ctx) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/context?budget=3000&session=after", id(&t)),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{ctx}");
    let md = ctx["markdown"].as_str().unwrap();
    let line = md
        .lines()
        .find(|l| l.contains("hello shouts with a bang"))
        .unwrap_or_else(|| panic!("{md}"));
    assert!(!line.contains("changed since recorded"), "{line}");
}

// ---- Plan 2b Task 3: conventions over HTTP, required ones survive tiny budgets ----

#[tokio::test(flavor = "multi_thread")]
async fn required_convention_added_over_http_survives_a_tiny_pack() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/repos/demo/conventions",
        Some(json!({"scope_glob": "app/**", "text": "never log user names", "required": true})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let cid = body["id"].as_i64().unwrap();
    let (status, bad) = call(
        &e.app,
        "POST",
        "/v1/repos/demo/conventions",
        Some(json!({"scope_glob": "[", "text": "x"})),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{bad}");
    let (_, pack) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/context?budget=150&session=tiny", id(&t)),
        None,
    )
    .await;
    assert!(
        pack["markdown"]
            .as_str()
            .unwrap()
            .contains(&format!("conv:{cid}")),
        "{pack}"
    );
}

// ---- Plan 2b Task 4: charged expand ----

#[tokio::test(flavor = "multi_thread")]
async fn expand_with_session_is_charged_and_capped() {
    let e = env_with(|s| s.with_expand_cap(Some(60))).await;
    let t = start_task(&e, "Improve greeting").await;
    let (_, r) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/records", id(&t)),
        Some(
            json!({"kind": "note", "summary": "a note worth expanding", "agent": "claude",
                    "body": "Some longer body text that costs a few tokens to expand each time."}),
        ),
    )
    .await;
    let rec = format!("rec:{}", &r["id"].as_str().unwrap().replace('-', "")[..8]);
    let url = format!("/v1/tasks/{}/items/{rec}?session=s-1", id(&t));
    let (status, body) = call(&e.app, "GET", &url, None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let used = body["session_expand_tokens"].as_u64().unwrap();
    assert!(used > 0, "{body}");
    // keep expanding until the 60-token cap refuses
    let mut refused = None;
    for _ in 0..20 {
        let (status, body) = call(&e.app, "GET", &url, None).await;
        if status != StatusCode::OK {
            refused = Some((status, body));
            break;
        }
    }
    let (status, body) = refused.expect("the cap never refused");
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
    assert!(
        body["error"].as_str().unwrap().contains("expand cap"),
        "{body}"
    );
    // without a session, expand stays uncharged (Plan 1 behavior)
    let (status, _) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/items/{rec}", id(&t)),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
}

// ---- Plan 3 Task 2: record discipline and the submit gate ----

#[tokio::test(flavor = "multi_thread")]
async fn submit_refused_when_nothing_changed() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    let (status, body) = submit(&e, &t).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(
        body["error"]
            .as_str()
            .unwrap()
            .contains("nothing to submit"),
        "{body}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn submit_refused_without_decision_or_note_and_names_the_symbols() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    edit_greeter(&e, &t);
    let (status, body) = submit(&e, &t).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    let msg = body["error"].as_str().unwrap();
    assert!(msg.contains("record a decision"), "{msg}");
    assert!(
        msg.contains("av submit --record"),
        "names the one-call form: {msg}"
    );
    assert!(msg.contains("Greeter.wave"), "{msg}");
    let (_, task) = call(&e.app, "GET", &format!("/v1/tasks/{}", id(&t)), None).await;
    assert_eq!(task["task"]["status"], "open");
}

#[tokio::test(flavor = "multi_thread")]
async fn submit_ok_moves_to_in_review_and_lists_unrecorded_symbols() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    edit_greeter(&e, &t);
    let (status, body) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/records", id(&t)),
        Some(
            json!({"kind": "decision", "summary": "hello shouts", "agent": "claude",
                    "anchors": [{"path": "app/greeter.py", "symbol": "Greeter.hello"}]}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = submit(&e, &t).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["status"], "in_review");
    let unrecorded: Vec<&str> = body["unrecorded_symbols"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(Value::as_str)
        .collect();
    assert!(unrecorded.contains(&"Greeter.wave"), "{body}");
    assert!(!unrecorded.contains(&"Greeter.hello"), "{body}");
    // a second submit is an illegal transition
    let (status, body) = submit(&e, &t).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
}

async fn submit_with(e: &Env, t: &Value, records: Value) -> (StatusCode, Value) {
    call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/submit", id(t)),
        Some(json!({"agent": "claude", "records": records})),
    )
    .await
}

async fn task_records(e: &Env, t: &Value) -> Vec<Value> {
    let (status, body) = call(&e.app, "GET", &format!("/v1/tasks/{}/records", id(t)), None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body["records"].as_array().unwrap().clone()
}

#[tokio::test(flavor = "multi_thread")]
async fn submit_with_inline_records_passes_the_gate() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    edit_greeter(&e, &t);
    let (status, body) = submit_with(
        &e,
        &t,
        json!([{"kind": "decision", "summary": "Greet by name",
                "anchors": [{"path": "app/greeter.py"}]}]),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["status"], "in_review");
    assert_eq!(body["recorded"].as_array().unwrap().len(), 1, "{body}");
    let recs = task_records(&e, &t).await;
    let rec = recs
        .iter()
        .find(|r| r["summary"] == "Greet by name")
        .unwrap_or_else(|| panic!("inline record not stored: {recs:?}"));
    assert_eq!(rec["author_agent"], "claude");
    assert_eq!(rec["id"], body["recorded"][0]);
}

#[tokio::test(flavor = "multi_thread")]
async fn submit_with_an_invalid_inline_record_writes_nothing() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    edit_greeter(&e, &t);
    let (status, body) = submit_with(
        &e,
        &t,
        json!([
            {"kind": "decision", "summary": "Greet by name"},
            {"kind": "decision", "summary": "x".repeat(201)}
        ]),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
    let (_, task) = call(&e.app, "GET", &format!("/v1/tasks/{}", id(&t)), None).await;
    assert_eq!(task["task"]["status"], "open", "{task}");
    let recs = task_records(&e, &t).await;
    assert!(
        !recs.iter().any(|r| r["kind"] == "decision"),
        "no inline record may be written: {recs:?}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn submit_with_a_malformed_inline_record_writes_nothing() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    edit_greeter(&e, &t);
    let (status, body) = submit_with(
        &e,
        &t,
        json!([
            {"kind": "decision", "summary": "Greet by name"},
            {"kind": "bogus", "summary": "nope"}
        ]),
    )
    .await;
    assert!(status.is_client_error(), "{status} {body}");
    let (_, task) = call(&e.app, "GET", &format!("/v1/tasks/{}", id(&t)), None).await;
    assert_eq!(task["task"]["status"], "open", "{task}");
    let recs = task_records(&e, &t).await;
    assert!(!recs.iter().any(|r| r["kind"] == "decision"), "{recs:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn submit_says_what_comes_next() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    edit_greeter(&e, &t);
    let (status, body) = submit_with(
        &e,
        &t,
        json!([{"kind": "note", "summary": "Greeting now shouts"}]),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let next = body["next"].as_str().unwrap_or_default();
    assert!(next.contains("av") && next.contains("review"), "{body}");
    assert_eq!(
        next,
        "in review: a reviewer runs av review to approve or request changes; nothing else to do",
        "{body}"
    );
    assert!(!next.contains("av next"), "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn concurrent_submit_only_one_wins() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    edit_greeter(&e, &t);
    call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/records", id(&t)),
        Some(json!({"kind": "note", "summary": "ready", "agent": "claude"})),
    )
    .await;
    let (a, b) = tokio::join!(submit(&e, &t), submit(&e, &t));
    let oks = [a.0, b.0].iter().filter(|s| **s == StatusCode::OK).count();
    assert_eq!(oks, 1, "{a:?} {b:?}");
}

// ---- Plan 7: task start opens an overlay, does not copy main ----

#[tokio::test(flavor = "multi_thread")]
async fn start_opens_an_empty_overlay_without_reading_the_repo() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let reads = Arc::new(AtomicUsize::new(0));
    let counter = reads.clone();
    let e = env_with_vcs(
        move |inner| {
            Arc::new(CountingVcs {
                inner,
                read_files: counter,
            })
        },
        |s| s,
    )
    .await;

    reads.store(0, Ordering::SeqCst);
    let t = start_task(&e, "overlay start").await;
    assert_eq!(
        reads.load(Ordering::SeqCst),
        0,
        "start must not re-read the repo"
    );
    let slug = t["task"]["slug"].as_str().unwrap();
    assert!(overlay_row(&e, slug).await.is_some());
    assert!(overlay_paths(&e, slug).await.is_empty());
    // the start pack still sees main's symbols through the empty overlay
    let md = t["pack"]["markdown"].as_str().unwrap();
    assert!(
        md.contains("Greeter") || md.contains("format_name") || md.contains("app/"),
        "{md}"
    );

    edit_greeter(&e, &t);
    let before = reads.load(Ordering::SeqCst);
    let (status, body) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/checkpoint", id(&t)),
        Some(json!({})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert!(
        reads.load(Ordering::SeqCst) > before,
        "checkpoint reads the changed path"
    );
    assert_eq!(overlay_paths(&e, slug).await, vec!["app/greeter.py"]);
    let qnames: Vec<String> = sqlx::query_scalar(
        "SELECT qname FROM symbols WHERE repo = 'demo' AND scope = $1 ORDER BY qname",
    )
    .bind(slug)
    .fetch_all(e.store.pool())
    .await
    .unwrap();
    assert!(qnames.iter().any(|q| q == "Greeter.wave"), "{qnames:?}");
}

// ---- Plan 3 Tasks 4-5: validate, review, merge, sync, undo ----

fn write_file(e: &Env, t: &Value, path: &str, text: &str) {
    std::fs::write(ws_dir(e, t).join(path), text).unwrap();
}

async fn post(e: &Env, t: &Value, op: &str, body: Value) -> (StatusCode, Value) {
    call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/{op}", id(t)),
        Some(body),
    )
    .await
}

async fn decide(e: &Env, t: &Value) {
    let (status, body) = post(
        e,
        t,
        "records",
        json!({"kind": "decision", "summary": "format names in upper case", "agent": "claude",
               "anchors": [{"path": "app/util.py", "symbol": "format_name"}]}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
}

async fn validate(e: &Env, t: &Value, exit_code: i32) -> Value {
    let (status, body) = post(
        e,
        t,
        "validate",
        json!({"agent": "claude", "command": "pytest", "exit_code": exit_code}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body
}

async fn review(e: &Env, t: &Value, reviewer: &str, verdict: &str) -> (StatusCode, Value) {
    post(
        e,
        t,
        "review",
        json!({"reviewer": reviewer, "verdict": verdict}),
    )
    .await
}

/// Edits util.py with `body`, records, submits, validates and gets an approval.
async fn ready_to_merge(e: &Env, title: &str, body: &str) -> Value {
    let t = start_task(e, title).await;
    write_file(e, &t, "app/util.py", body);
    decide(e, &t).await;
    let (status, b) = submit(e, &t).await;
    assert_eq!(status, StatusCode::OK, "{b}");
    validate(e, &t, 0).await;
    let (status, b) = review(e, &t, "reviewer", "approve").await;
    assert_eq!(status, StatusCode::OK, "{b}");
    t
}

async fn main_file(e: &Env, path: &str) -> String {
    let t = start_task(e, "peek at main").await;
    std::fs::read_to_string(ws_dir(e, &t).join(path)).unwrap()
}

const UPPER: &str = "def format_name(n):\n    return n.upper()\n";
const LOWER: &str = "def format_name(n):\n    return n.lower()\n";

#[tokio::test(flavor = "multi_thread")]
async fn validate_review_merge_advances_main_for_later_tasks() {
    let e = env().await;
    let t = ready_to_merge(&e, "Upper names", UPPER).await;
    let (status, body) = post(&e, &t, "merge", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["status"], "merged");
    assert_eq!(body["changed"], json!(["app/util.py"]));
    let (_, task) = call(&e.app, "GET", &format!("/v1/tasks/{}", id(&t)), None).await;
    assert_eq!(task["task"]["status"], "merged");
    assert_eq!(task["merge"]["merge_commit"], body["main"]);
    assert_eq!(main_file(&e, "app/util.py").await, UPPER);
}

#[tokio::test(flavor = "multi_thread")]
async fn approve_requires_a_passing_validation_at_head() {
    let e = env().await;
    let t = start_task(&e, "Upper names").await;
    write_file(&e, &t, "app/util.py", UPPER);
    decide(&e, &t).await;
    submit(&e, &t).await;
    let (status, body) = review(&e, &t, "reviewer", "approve").await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(
        body["error"].as_str().unwrap().contains("tests not run"),
        "{body}"
    );
    validate(&e, &t, 1).await;
    let (status, body) = review(&e, &t, "reviewer", "approve").await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(body["error"].as_str().unwrap().contains("failed"), "{body}");
    validate(&e, &t, 0).await;
    let (status, body) = review(&e, &t, "reviewer", "approve").await;
    assert_eq!(status, StatusCode::OK, "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn owner_cannot_review_and_merge_needs_an_approval() {
    let e = env().await;
    let t = start_task(&e, "Upper names").await;
    write_file(&e, &t, "app/util.py", UPPER);
    decide(&e, &t).await;
    submit(&e, &t).await;
    validate(&e, &t, 0).await;
    let (status, body) = review(&e, &t, "claude", "approve").await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    let (status, body) = post(&e, &t, "merge", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(
        body["error"].as_str().unwrap().contains("no approval"),
        "{body}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn request_changes_reopens_the_task_with_a_review_record() {
    let e = env().await;
    let t = start_task(&e, "Upper names").await;
    write_file(&e, &t, "app/util.py", UPPER);
    decide(&e, &t).await;
    submit(&e, &t).await;
    let (status, body) = post(
        &e,
        &t,
        "review",
        json!({"reviewer": "reviewer", "verdict": "request_changes",
               "findings": [{"summary": "keep title case for names", "anchors": [{"path": "app/util.py", "symbol": "format_name"}]}]}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["status"], "open");
    let records = e
        .store
        .task_records(scholion_core::TaskId(id(&t).parse().unwrap()))
        .await
        .unwrap();
    assert!(
        records
            .iter()
            .any(|r| r.kind == scholion_core::RecordKind::Review && r.author_agent == "reviewer")
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn approval_is_void_after_the_workspace_changes() {
    let e = env().await;
    let t = ready_to_merge(&e, "Upper names", UPPER).await;
    write_file(&e, &t, "app/util.py", LOWER);
    let (status, body) = post(&e, &t, "merge", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(
        body["error"].as_str().unwrap().contains("changed since"),
        "{body}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn conflict_then_sync_fix_resubmit_and_merge() {
    let e = env().await;
    let first = ready_to_merge(&e, "Upper names", UPPER).await;
    let second = ready_to_merge(&e, "Lower names", LOWER).await;
    let (status, body) = post(&e, &first, "merge", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let main_after_first = body["main"].clone();

    let (status, body) = post(&e, &second, "merge", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["status"], "conflicted");
    assert_eq!(body["conflicts"], json!(["app/util.py"]));
    let (_, task) = call(&e.app, "GET", &format!("/v1/tasks/{}", id(&second)), None).await;
    assert_eq!(task["merge"]["conflicts"], json!(["app/util.py"]));

    // submit rebases, leaves markers and refuses
    let (status, body) = submit(&e, &second).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(
        body["error"].as_str().unwrap().contains("app/util.py"),
        "{body}"
    );
    let text = std::fs::read_to_string(ws_dir(&e, &second).join("app/util.py")).unwrap();
    assert!(text.contains("<<<<<<<"), "{text}");

    let fixed = "def format_name(n):\n    return n.upper().strip()\n";
    write_file(&e, &second, "app/util.py", fixed);
    let (status, body) = submit(&e, &second).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["status"], "in_review");
    assert_eq!(body["changed"], json!(["app/util.py"]));
    validate(&e, &second, 0).await;
    let (status, body) = review(&e, &second, "reviewer", "approve").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = post(&e, &second, "merge", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["status"], "merged");
    assert_ne!(body["main"], main_after_first);
    assert_eq!(main_file(&e, "app/util.py").await, fixed);
}

#[tokio::test(flavor = "multi_thread")]
async fn sync_brings_main_into_an_open_task() {
    let e = env().await;
    let other = start_task(&e, "Add helper").await;
    let t = ready_to_merge(&e, "Upper names", UPPER).await;
    post(&e, &t, "merge", json!({"agent": "merger"})).await;
    let (status, body) = post(&e, &other, "sync", json!({"agent": "claude"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["conflicts"], json!([]));
    let text = std::fs::read_to_string(ws_dir(&e, &other).join("app/util.py")).unwrap();
    assert_eq!(text, UPPER);
}

#[tokio::test(flavor = "multi_thread")]
async fn undo_returns_main_and_the_task_to_review() {
    let e = env().await;
    let t = ready_to_merge(&e, "Upper names", UPPER).await;
    post(&e, &t, "merge", json!({"agent": "merger"})).await;
    let (status, body) = post(&e, &t, "undo", json!({"agent": "someone-else"})).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    let (status, body) = post(&e, &t, "undo", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["status"], "in_review");
    assert_eq!(
        main_file(&e, "app/util.py").await,
        "def format_name(n):\n    return n.title()\n"
    );
}

// ---- Plan 3 Task 3: handoff and claim ----

async fn task_json(e: &Env, t: &Value) -> Value {
    let (status, body) = call(&e.app, "GET", &format!("/v1/tasks/{}", id(t)), None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body
}

#[tokio::test(flavor = "multi_thread")]
async fn handoff_requires_gate_and_moves_to_handed_off() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    edit_greeter(&e, &t);
    let (status, body) = handoff(&e, &t, "claude").await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(
        body["error"]
            .as_str()
            .unwrap()
            .contains("record a decision"),
        "{body}"
    );
    assert!(
        !body["error"].as_str().unwrap().contains("av submit"),
        "a handoff is not told to submit: {body}"
    );
    assert_eq!(task_json(&e, &t).await["task"]["status"], "open");

    record_decision(&e, &t).await;
    let (status, body) = handoff(&e, &t, "claude").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["status"], "handed_off");
    let md = body["bundle"]["markdown"].as_str().unwrap();
    assert!(
        md.contains("## Goal") && md.contains("Remaining: make goodbye louder"),
        "{md}"
    );
    assert!(
        md.contains("Greeter.wave"),
        "the bundle sees the snapshot taken by handoff:\n{md}"
    );
    assert!(body["bundle"]["tokens"].as_u64().unwrap() <= 4000);
}

#[tokio::test(flavor = "multi_thread")]
async fn failed_handoff_gate_leaves_no_handoff_record() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    edit_greeter(&e, &t);
    let (status, _) = handoff(&e, &t, "claude").await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    let task = scholion_core::TaskId(id(&t).parse().unwrap());
    let records = e.store.task_records(task).await.unwrap();
    assert!(
        records
            .iter()
            .all(|r| r.kind != scholion_core::RecordKind::Handoff),
        "{records:?}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn only_the_owner_hands_off() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    edit_greeter(&e, &t);
    record_decision(&e, &t).await;
    let (status, body) = handoff(&e, &t, "codex").await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(body["error"].as_str().unwrap().contains("owner"), "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn claim_changes_owner_and_reopens() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    assert_eq!(t["task"]["generation"], 1);
    edit_greeter(&e, &t);
    record_decision(&e, &t).await;
    assert_eq!(handoff(&e, &t, "claude").await.0, StatusCode::OK);
    let (status, body) = claim(&e, &t, "codex").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["task"]["status"], "open");
    assert_eq!(body["task"]["owner_agent"], "codex");
    assert_eq!(body["task"]["generation"], 2);
    assert_eq!(body["generation"], 2);
    assert_eq!(
        body["task"]["workspace_path"],
        format!("{HOST_WS}/demo/improve-greeting")
    );
    let session = body["session"].as_str().unwrap();
    assert!(
        !session.is_empty() && session != "s-1",
        "a claim starts a new session: {session}"
    );
    let md = body["bundle"]["markdown"].as_str().unwrap();
    assert!(md.contains("Remaining: make goodbye louder"), "{md}");
    // the new owner can carry on to submit
    let (status, body) = call_as(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/submit", id(&t)),
        Some(json!({"agent": "codex"})),
        Some(2),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn claim_by_current_owner_is_refused() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    edit_greeter(&e, &t);
    record_decision(&e, &t).await;
    assert_eq!(handoff(&e, &t, "claude").await.0, StatusCode::OK);
    let (status, body) = claim(&e, &t, "claude").await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(
        body["error"].as_str().unwrap().contains("already yours"),
        "{body}"
    );
    assert_eq!(task_json(&e, &t).await["task"]["status"], "handed_off");
}

#[tokio::test(flavor = "multi_thread")]
async fn handoff_with_missing_workspace_errors_cleanly() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    edit_greeter(&e, &t);
    record_decision(&e, &t).await;
    std::fs::remove_dir_all(ws_dir(&e, &t)).unwrap();
    let (status, body) = handoff(&e, &t, "claude").await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    let msg = body["error"].as_str().unwrap();
    assert!(
        msg.contains(&format!("{HOST_WS}/demo/improve-greeting")),
        "names the host path: {msg}"
    );
    let task = task_json(&e, &t).await;
    assert_eq!(task["task"]["status"], "open");
    assert_eq!(task["record_count"], 2, "records stay readable: {task}");
}

#[tokio::test(flavor = "multi_thread")]
async fn stale_generation_write_is_refused() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    edit_greeter(&e, &t);
    record_decision(&e, &t).await;
    assert_eq!(handoff(&e, &t, "claude").await.0, StatusCode::OK);
    assert_eq!(claim(&e, &t, "codex").await.0, StatusCode::OK);
    let tid = id(&t);
    let checkpoint = format!("/v1/tasks/{tid}/checkpoint");
    let records = format!("/v1/tasks/{tid}/records");
    let note = json!({"kind": "note", "summary": "still here", "agent": "claude"});
    for (uri, body) in [
        (&checkpoint, None),
        (&records, Some(note.clone())),
        (
            &format!("/v1/tasks/{tid}/submit"),
            Some(json!({"agent": "claude"})),
        ),
    ] {
        let (status, resp) = call_as(&e.app, "POST", uri, body, Some(1)).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{uri}: {resp}");
        assert!(
            resp["error"].as_str().unwrap().contains("claim lost"),
            "{resp}"
        );
    }
    let (status, resp) = call_as(&e.app, "POST", &checkpoint, None, Some(2)).await;
    assert_eq!(status, StatusCode::OK, "the current claim writes: {resp}");
    let (status, resp) = call(&e.app, "POST", &records, Some(note)).await;
    assert_eq!(
        status,
        StatusCode::OK,
        "a write without a generation is not fenced: {resp}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn expired_lease_can_be_claimed_by_another_agent() {
    let held = env().await;
    let t = start_task(&held, "Improve greeting").await;
    let (status, body) = claim(&held, &t, "codex").await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(body["error"].as_str().unwrap().contains("lease"), "{body}");

    let e = env_with(|s| s.with_lease_secs(0)).await;
    let t = start_task(&e, "Improve greeting").await;
    let (status, body) = claim(&e, &t, "codex").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["task"]["owner_agent"], "codex");
    assert_eq!(body["task"]["status"], "open");
    assert_eq!(body["generation"], 2);
}

// ---- merge crash recovery, bundles ----

/// Merges for real, then fails the first `merge_task` call as if the daemon died before it could
/// record the merge.
struct CrashAfterMerge {
    inner: Arc<dyn scholion_core::Vcs>,
    crashed: std::sync::atomic::AtomicBool,
}

#[async_trait::async_trait]
impl scholion_core::Vcs for CrashAfterMerge {
    async fn init_repo(&self, r: &str, p: &Path) -> scholion_core::Result<scholion_core::ChangeId> {
        self.inner.init_repo(r, p).await
    }
    async fn create_workspace(
        &self,
        r: &str,
        s: &str,
    ) -> scholion_core::Result<scholion_core::WorkspaceInfo> {
        self.inner.create_workspace(r, s).await
    }
    async fn snapshot(&self, r: &str, s: &str) -> scholion_core::Result<scholion_core::ChangeId> {
        self.inner.snapshot(r, s).await
    }
    async fn changed_paths(
        &self,
        r: &str,
        b: &scholion_core::ChangeId,
        h: &scholion_core::ChangeId,
    ) -> scholion_core::Result<Vec<String>> {
        self.inner.changed_paths(r, b, h).await
    }
    async fn list_files(
        &self,
        r: &str,
        c: &scholion_core::ChangeId,
    ) -> scholion_core::Result<Vec<String>> {
        self.inner.list_files(r, c).await
    }
    async fn read_file(
        &self,
        r: &str,
        c: &scholion_core::ChangeId,
        p: &str,
    ) -> scholion_core::Result<Vec<u8>> {
        self.inner.read_file(r, c, p).await
    }
    async fn read_files(
        &self,
        r: &str,
        c: &scholion_core::ChangeId,
        p: &[String],
    ) -> scholion_core::Result<Vec<(String, scholion_core::Result<Vec<u8>>)>> {
        self.inner.read_files(r, c, p).await
    }
    async fn trunk(&self, r: &str) -> scholion_core::Result<scholion_core::ChangeId> {
        self.inner.trunk(r).await
    }
    async fn trunk_log(
        &self,
        r: &str,
        n: usize,
    ) -> scholion_core::Result<Vec<scholion_core::LogEntry>> {
        self.inner.trunk_log(r, n).await
    }
    async fn merge_task(
        &self,
        r: &str,
        s: &str,
        h: &scholion_core::ChangeId,
        m: &str,
    ) -> scholion_core::Result<scholion_core::MergeOutcome> {
        let out = self.inner.merge_task(r, s, h, m).await?;
        if !self.crashed.swap(true, std::sync::atomic::Ordering::SeqCst) {
            return Err(scholion_core::AvError::Vcs("daemon died here".into()));
        }
        Ok(out)
    }
    async fn rebase_workspace(
        &self,
        r: &str,
        s: &str,
    ) -> scholion_core::Result<scholion_core::RebaseOutcome> {
        self.inner.rebase_workspace(r, s).await
    }
    async fn undo_merge(
        &self,
        r: &str,
        p: &scholion_core::ChangeId,
        m: &scholion_core::ChangeId,
    ) -> scholion_core::Result<()> {
        self.inner.undo_merge(r, p, m).await
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn a_merge_interrupted_after_main_moved_completes_without_merging_twice() {
    let inner: std::sync::Mutex<Option<Arc<dyn scholion_core::Vcs>>> = Default::default();
    let e = env_with_vcs(
        |v| {
            *inner.lock().unwrap() = Some(v.clone());
            Arc::new(CrashAfterMerge {
                inner: v,
                crashed: Default::default(),
            })
        },
        |s| s,
    )
    .await;
    let vcs = inner.lock().unwrap().clone().unwrap();
    let t = ready_to_merge(&e, "Upper names", UPPER).await;
    let (status, body) = post(&e, &t, "merge", json!({"agent": "merger"})).await;
    assert_ne!(status, StatusCode::OK, "the first attempt dies: {body}");
    assert_eq!(task_json(&e, &t).await["task"]["status"], "in_review");

    let (status, body) = post(&e, &t, "merge", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["status"], "merged");
    assert_eq!(body["recovered"], true, "{body}");
    let marker = format!("av task {}", id(&t));
    let log = vcs.trunk_log("demo", 20).await.unwrap();
    assert_eq!(
        log.iter()
            .filter(|c| c.description.contains(&marker))
            .count(),
        1,
        "{log:?}"
    );
    assert_eq!(body["main"], log[0].change.0);
    let state = task_json(&e, &t).await;
    assert_eq!(state["merge"]["merge_commit"], log[0].change.0, "{state}");
    assert_eq!(state["merge"]["pre_merge"], log[1].change.0, "{state}");
    assert_eq!(main_file(&e, "app/util.py").await, UPPER);
}

#[tokio::test(flavor = "multi_thread")]
async fn bundle_route_serves_handoff_and_review_bundles() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    edit_greeter(&e, &t);
    record_decision(&e, &t).await;
    call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/checkpoint", id(&t)),
        None,
    )
    .await;
    let get = |kind: &'static str| {
        let uri = format!("/v1/tasks/{}/bundle?kind={kind}&budget=2000", id(&t));
        let app = e.app.clone();
        async move { call(&app, "GET", &uri, None).await }
    };
    let (status, pr) = get("pr").await;
    assert_eq!(status, StatusCode::OK, "{pr}");
    let md = pr["markdown"].as_str().unwrap();
    assert!(
        md.contains("## Tests") && md.contains("not run at this head"),
        "{md}"
    );
    assert!(pr["tokens"].as_u64().unwrap() <= 2000);
    let (status, handoff) = get("handoff").await;
    assert_eq!(status, StatusCode::OK, "{handoff}");
    assert!(!handoff["markdown"].as_str().unwrap().contains("## Tests"));
    let (status, bad) = get("poem").await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{bad}");
}

// ---- Plan 3 Task 6: backup push ----

async fn backup(e: &Env) -> Value {
    let (status, body) = call(&e.app, "GET", "/v1/repos/demo/backup", None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body
}

async fn set_remote(e: &Env, remote: &str) -> Value {
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/repos/demo/backup",
        Some(json!({"remote": remote})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body
}

fn bare_repo(path: &Path) {
    std::fs::create_dir_all(path).unwrap();
    git(path, &["init", "-q", "--bare", "-b", "main"]);
}

/// The background push after `set_remote` or a merge, done: waits until no attempt is pending.
async fn settled(e: &Env) -> Value {
    for _ in 0..100 {
        let b = backup(e).await;
        if b["failures"].as_u64().unwrap() > 0 || b["last_pushed"].is_string() {
            return b;
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    panic!("backup never ran: {}", backup(e).await);
}

#[tokio::test(flavor = "multi_thread")]
async fn merge_succeeds_when_remote_unreachable_and_records_failure() {
    let e = env().await;
    let dir = TempDir::new().unwrap();
    let missing = dir.path().join("nowhere.git");
    set_remote(&e, missing.to_str().unwrap()).await;
    settled(&e).await;
    let t = ready_to_merge(&e, "Upper names", UPPER).await;
    let (status, body) = post(&e, &t, "merge", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["status"], "merged");
    let mut state = backup(&e).await;
    for _ in 0..100 {
        if state["failures"].as_u64().unwrap() >= 2 {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        state = backup(&e).await;
    }
    assert!(state["failures"].as_u64().unwrap() >= 2, "{state}");
    assert!(
        state["last_error"].as_str().unwrap().contains("git"),
        "{state}"
    );
    assert!(state["next_retry"].is_string(), "{state}");
    let status = task_json(&e, &t).await;
    assert_eq!(status["backup"]["failures"], state["failures"], "{status}");
}

#[tokio::test(flavor = "multi_thread")]
async fn successful_retry_clears_failure_state_and_pushes_main() {
    let e = env().await;
    let dir = TempDir::new().unwrap();
    let remote = dir.path().join("backup.git");
    set_remote(&e, remote.to_str().unwrap()).await;
    let failed = settled(&e).await;
    assert_eq!(failed["failures"], 1, "{failed}");
    let t = ready_to_merge(&e, "Upper names", UPPER).await;
    let (_, merged) = post(&e, &t, "merge", json!({"agent": "merger"})).await;
    bare_repo(&remote);
    let (status, state) = call(&e.app, "POST", "/v1/repos/demo/backup/push", None).await;
    assert_eq!(status, StatusCode::OK, "{state}");
    assert_eq!(state["failures"], 0, "{state}");
    assert_eq!(state["last_error"], Value::Null);
    assert_eq!(state["next_retry"], Value::Null);
    assert_eq!(state["last_pushed"], merged["main"], "{state}");
    assert_eq!(
        git_out(&remote, &["rev-parse", "refs/heads/main"]),
        merged["main"].as_str().unwrap()
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn backup_can_push_main_to_a_named_branch() {
    let e = env().await;
    let dir = TempDir::new().unwrap();
    let remote = dir.path().join("backup.git");
    bare_repo(&remote);
    let t = ready_to_merge(&e, "Upper names", UPPER).await;
    let (_, merged) = post(&e, &t, "merge", json!({"agent": "merger"})).await;
    let (status, state) = call(
        &e.app,
        "POST",
        "/v1/repos/demo/backup",
        Some(json!({"remote": remote.to_str().unwrap(), "branch": "av-main"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{state}");
    assert_eq!(state["branch"], "av-main", "{state}");
    let done = settled(&e).await;
    assert_eq!(done["failures"], 0, "{done}");
    assert_eq!(done["last_pushed"], merged["main"], "{done}");
    assert_eq!(
        done["branch"], "av-main",
        "the target survives the push: {done}"
    );
    assert_eq!(
        git_out(&remote, &["rev-parse", "refs/heads/av-main"]),
        merged["main"].as_str().unwrap()
    );
    let (status, _) = call(&e.app, "POST", "/v1/repos/demo/backup/push", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(backup(&e).await["branch"], "av-main");

    // setting only the remote goes back to the trunk name
    let (_, state) = call(
        &e.app,
        "POST",
        "/v1/repos/demo/backup",
        Some(json!({"remote": remote.to_str().unwrap()})),
    )
    .await;
    assert_eq!(state["branch"], Value::Null, "{state}");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_bad_backup_branch_is_rejected_and_changes_nothing() {
    let e = env().await;
    for bad in ["", "-x", "a..b", "a b", "x.lock", "/x"] {
        let (status, body) = call(
            &e.app,
            "POST",
            "/v1/repos/demo/backup",
            Some(json!({"remote": "/srv/b.git", "branch": bad})),
        )
        .await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{bad:?}: {body}");
    }
    assert_eq!(backup(&e).await["remote"], Value::Null);
}

#[tokio::test(flavor = "multi_thread")]
async fn option_like_backup_remote_is_rejected() {
    let e = env().await;
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/repos/demo/backup",
        Some(json!({"remote": "--upload-pack=sh"})),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
    assert_eq!(backup(&e).await["remote"], Value::Null);
}

// ---- Plan 3 Task 7: search, abandon, log ----

#[tokio::test(flavor = "multi_thread")]
async fn search_route_finds_records_of_the_repo() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    record_decision(&e, &t).await;
    let (status, body) = call(&e.app, "GET", "/v1/repos/demo/search?q=shouts", None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let hits = body["records"].as_array().unwrap();
    assert_eq!(hits.len(), 1, "{body}");
    assert_eq!(hits[0]["summary"], "hello shouts");
    assert_eq!(hits[0]["task_id"], id(&t));
    let (_, other) = call(&e.app, "GET", "/v1/repos/elsewhere/search?q=shouts", None).await;
    assert_eq!(other["records"], json!([]));
    let (_, blank) = call(&e.app, "GET", "/v1/repos/demo/search?q=%20", None).await;
    assert_eq!(blank["records"], json!([]));
    let (status, _) = call(&e.app, "GET", "/v1/repos/demo/search?q=x&limit=5000", None).await;
    assert_eq!(
        status,
        StatusCode::OK,
        "a large limit is capped, not refused"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn abandon_ends_the_task_and_only_the_owner_may() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    let (status, body) = post(&e, &t, "abandon", json!({"agent": "codex"})).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(body["error"].as_str().unwrap().contains("owner"), "{body}");
    let (status, body) = post(&e, &t, "abandon", json!({"agent": "claude"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["status"], "abandoned");
    let (status, body) = post(&e, &t, "abandon", json!({"agent": "claude"})).await;
    assert_eq!(
        status,
        StatusCode::UNPROCESSABLE_ENTITY,
        "abandoned is final: {body}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn illegal_transition_returns_422_with_reason() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    let (status, body) = post(&e, &t, "merge", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    let msg = body["error"].as_str().unwrap();
    assert!(msg.contains("open") && msg.contains("MergeOk"), "{msg}");
}

#[tokio::test(flavor = "multi_thread")]
async fn log_route_lists_main_newest_first() {
    let e = env().await;
    let t = ready_to_merge(&e, "Upper names", UPPER).await;
    let (_, merged) = post(&e, &t, "merge", json!({"agent": "merger"})).await;
    let (status, body) = call(&e.app, "GET", "/v1/repos/demo/log?limit=5", None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let log = body["commits"].as_array().unwrap();
    assert_eq!(log.len(), 2, "the merge and the imported commit: {body}");
    assert_eq!(log[0]["change"], merged["main"]);
    assert!(
        log[0]["description"]
            .as_str()
            .unwrap()
            .starts_with("Upper names"),
        "{body}"
    );
}

// ---- Plan 3 Task 9: token, scope cleanup, expand repo check, prune ----

#[tokio::test(flavor = "multi_thread")]
async fn token_required_when_configured_and_healthz_open() {
    let e = env_with(|s| s.with_token(Some("sekrit".into()))).await;
    let (status, _) = call(&e.app, "GET", "/healthz", None).await;
    assert_eq!(status, StatusCode::OK);
    let (status, body) = call(&e.app, "GET", "/v1/repos/demo/backup", None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED, "{body}");
    assert!(body["error"].as_str().unwrap().contains("Bearer"), "{body}");
    let (status, body) = call_with(
        &e.app,
        "GET",
        "/v1/repos/demo/backup",
        None,
        &[("authorization", "Bearer sekrit")],
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn wrong_token_is_401() {
    let e = env_with(|s| s.with_token(Some("sekrit".into()))).await;
    for value in [
        "Bearer sekri",
        "Bearer sekrit2",
        "sekrit",
        "Basic c2Vrcml0",
        "Bearer ",
    ] {
        let (status, _) = call_with(
            &e.app,
            "GET",
            "/v1/repos/demo/backup",
            None,
            &[("authorization", value)],
        )
        .await;
        assert_eq!(status, StatusCode::UNAUTHORIZED, "{value}");
    }
}

async fn scope_symbols(e: &Env, scope: &str) -> i64 {
    sqlx::query_scalar("SELECT count(*) FROM symbols WHERE repo = 'demo' AND scope = $1")
        .bind(scope)
        .fetch_one(e.store.pool())
        .await
        .unwrap()
}

#[tokio::test(flavor = "multi_thread")]
async fn merge_and_abandon_drop_the_task_scope_and_undo_restores_it() {
    let e = env().await;
    let t = ready_to_merge(&e, "Upper names", UPPER).await;
    let slug = t["task"]["slug"].as_str().unwrap().to_owned();
    assert!(scope_symbols(&e, &slug).await > 0, "overlay holds the edit");
    assert!(overlay_row(&e, &slug).await.is_some());
    let (status, body) = post(&e, &t, "merge", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(scope_symbols(&e, &slug).await, 0, "merged: overlay dropped");
    assert!(overlay_row(&e, &slug).await.is_none());
    assert!(scope_symbols(&e, "main").await > 0, "main untouched");
    let (status, body) = post(&e, &t, "undo", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert!(
        scope_symbols(&e, &slug).await > 0,
        "back in review: overlay rebuilt for the task's diff"
    );
    assert!(overlay_row(&e, &slug).await.is_some());

    let a = start_task(&e, "Give up").await;
    let gone = a["task"]["slug"].as_str().unwrap().to_owned();
    assert!(overlay_row(&e, &gone).await.is_some());
    assert_eq!(scope_symbols(&e, &gone).await, 0, "start: O empty");
    let (status, body) = post(&e, &a, "abandon", json!({"agent": "claude"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert!(
        overlay_row(&e, &gone).await.is_none(),
        "abandoned: overlay dropped"
    );
    assert_eq!(scope_symbols(&e, &gone).await, 0);
}

#[tokio::test(flavor = "multi_thread")]
async fn expand_rejects_items_from_another_repo() {
    let e = env().await;
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/repos",
        Some(json!({"name": "other", "git_source": e.src_path})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, theirs) = call(
        &e.app,
        "POST",
        "/v1/tasks",
        Some(
            json!({"repo": "other", "title": "Other work", "goal": "Make hello louder",
                    "agent": "claude", "session": "s-2"}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{theirs}");
    let (_, rec) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/records", id(&theirs)),
        Some(json!({"kind": "note", "summary": "private to other", "agent": "claude"})),
    )
    .await;
    let rec_id = format!("rec:{}", &rec["id"].as_str().unwrap().replace('-', "")[..8]);
    // Task scopes are overlays: symbols live on main. Take a main symbol of the other repo.
    let their_sym: i64 =
        sqlx::query_scalar("SELECT min(id) FROM symbols WHERE repo = 'other' AND scope = 'main'")
            .fetch_one(e.store.pool())
            .await
            .unwrap();
    let ours = start_task(&e, "Improve greeting").await;
    for item in [rec_id.clone(), format!("sym:{their_sym}")] {
        let (status, body) = call(
            &e.app,
            "GET",
            &format!("/v1/tasks/{}/items/{item}", id(&ours)),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{item}: {body}");
        let (status, body) = call(
            &e.app,
            "GET",
            &format!("/v1/tasks/{}/items/{item}", id(&theirs)),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{item} in its own repo: {body}");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn prune_route_reports_what_it_removed() {
    let e = env().await;
    let (status, body) = call(&e.app, "POST", "/v1/maintenance/prune", None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["sessions"], 0, "s-1 is not idle: {body}");
    assert!(body["parse_cache"].is_u64(), "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn task_only_pack_mode_keeps_just_the_task_section() {
    let e = env().await;
    let full = start_task(&e, "Full packs").await;
    edit_greeter(&e, &full);
    record_decision(&e, &full).await;
    let (status, lean) = call(
        &e.app,
        "POST",
        "/v1/tasks",
        Some(
            json!({"repo": "demo", "title": "Lean packs", "goal": "Make hello louder",
                    "agent": "claude", "session": "s-2", "pack_mode": "task_only"}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{lean}");
    assert_eq!(lean["task"]["pack_mode"], "task_only");
    assert_eq!(full["task"]["pack_mode"], "full");
    for pack in [
        lean["pack"]["markdown"].as_str().unwrap().to_owned(),
        call(
            &e.app,
            "GET",
            &format!("/v1/tasks/{}/context?budget=4000", id(&lean)),
            None,
        )
        .await
        .1["markdown"]
            .as_str()
            .unwrap()
            .to_owned(),
    ] {
        assert!(pack.contains("## Task"), "{pack}");
        assert!(pack.contains("Make hello louder"), "{pack}");
        for other in [
            "## Map",
            "## Decisions & dead ends",
            "## Code",
            "## Live work",
            "hello shouts",
        ] {
            assert!(
                !pack.contains(other),
                "{other} in a task_only pack:\n{pack}"
            );
        }
    }
    let (_, fullpack) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/context?budget=4000", id(&full)),
        None,
    )
    .await;
    assert!(
        fullpack["markdown"]
            .as_str()
            .unwrap()
            .contains("hello shouts"),
        "{fullpack}"
    );
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/tasks",
        Some(
            json!({"repo": "demo", "title": "Bad", "goal": "g", "agent": "a", "session": "s",
                    "pack_mode": "tiny"}),
        ),
    )
    .await;
    assert!(status.is_client_error(), "{status} {body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn outbox_route_reports_pending_rows_and_whether_semantic_is_on() {
    let e = env().await;
    let (status, body) = call(&e.app, "GET", "/v1/repos/demo/outbox", None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["semantic"], false);
    assert!(
        body["pending"].as_i64().unwrap() > 0,
        "main's chunks are queued: {body}"
    );
    let (_, other) = call(&e.app, "GET", "/v1/repos/nope/outbox", None).await;
    assert_eq!(other["pending"], 0);
    let e = env_with(|s| s.with_semantic(Arc::new(FailingSemantic))).await;
    let (_, body) = call(&e.app, "GET", "/v1/repos/demo/outbox", None).await;
    assert_eq!(body["semantic"], true);
}

// ---- Plan 5 Task 2: workspace-free read API ----

#[tokio::test(flavor = "multi_thread")]
async fn read_api_serves_files_and_trees_at_main_a_change_and_a_task() {
    let e = env().await;
    let (status, body) = call(&e.app, "GET", "/v1/repos/demo/files/app/util.py", None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(
        body["content"],
        "def format_name(n):\n    return n.title()\n"
    );
    let imported = body["at"].as_str().unwrap().to_owned();

    let t = ready_to_merge(&e, "Upper names", UPPER).await;
    let slug = t["task"]["slug"].as_str().unwrap();
    let (_, body) = call(
        &e.app,
        "GET",
        &format!("/v1/repos/demo/files/app/util.py?at=task:{slug}"),
        None,
    )
    .await;
    assert_eq!(
        body["content"], UPPER,
        "a task read serves its last checkpoint"
    );
    post(&e, &t, "merge", json!({"agent": "merger"})).await;

    let (_, body) = call(
        &e.app,
        "GET",
        "/v1/repos/demo/files/app/util.py?at=main",
        None,
    )
    .await;
    assert_eq!(body["content"], UPPER);
    let (_, body) = call(
        &e.app,
        "GET",
        &format!("/v1/repos/demo/files/app/util.py?at={imported}"),
        None,
    )
    .await;
    assert_eq!(
        body["content"],
        "def format_name(n):\n    return n.title()\n"
    );

    let (status, body) = call(&e.app, "GET", "/v1/repos/demo/files/nope.py", None).await;
    assert_eq!(status, StatusCode::NOT_FOUND, "{body}");
    let (status, body) = call(
        &e.app,
        "GET",
        "/v1/repos/demo/files/app/util.py?at=task:no-such-task",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND, "{body}");

    let (status, body) = call(&e.app, "GET", "/v1/repos/demo/tree", None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(
        body["files"],
        json!([".gitignore", "app/greeter.py", "app/util.py"])
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn read_api_diffs_two_changes_and_logs_one_path() {
    let e = env().await;
    let (_, before) = call(&e.app, "GET", "/v1/repos/demo/tree", None).await;
    let before = before["at"].as_str().unwrap().to_owned();
    let t = ready_to_merge(&e, "Upper names", UPPER).await;
    let (_, merged) = post(&e, &t, "merge", json!({"agent": "merger"})).await;
    let after = merged["main"].as_str().unwrap();

    let (status, body) = call(
        &e.app,
        "GET",
        &format!("/v1/repos/demo/diff?from={before}&to={after}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let files = body["files"].as_array().unwrap();
    assert_eq!(files.len(), 1, "{body}");
    assert_eq!(files[0]["path"], "app/util.py");
    assert_eq!(files[0]["status"], "modified");
    let diff = files[0]["diff"].as_str().unwrap();
    assert!(
        diff.contains("-    return n.title()") && diff.contains("+    return n.upper()"),
        "{diff}"
    );

    let (_, body) = call(&e.app, "GET", "/v1/repos/demo/log?path=app/util.py", None).await;
    let commits = body["commits"].as_array().unwrap();
    assert_eq!(
        commits.len(),
        2,
        "the merge and the import both touch it: {body}"
    );
    assert_eq!(commits[0]["change"], merged["main"]);
    let (_, body) = call(
        &e.app,
        "GET",
        "/v1/repos/demo/log?path=app/greeter.py",
        None,
    )
    .await;
    let commits = body["commits"].as_array().unwrap();
    assert_eq!(
        commits.len(),
        1,
        "only the import touches greeter.py: {body}"
    );
}

// ---- Plan 5 Task 3: event feed ----

fn kinds(body: &Value) -> Vec<String> {
    body["events"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| e["kind"].as_str().unwrap().to_owned())
        .collect()
}

#[tokio::test(flavor = "multi_thread")]
async fn events_feed_reports_the_lifecycle_in_order() {
    let e = env().await;
    let t = ready_to_merge(&e, "Upper names", UPPER).await;
    let (_, merged) = post(&e, &t, "merge", json!({"agent": "merger"})).await;
    let (status, body) = call(&e.app, "GET", "/v1/events?repo=demo&since=0", None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(
        kinds(&body),
        ["started", "submit", "approve", "merged"],
        "{body}"
    );
    let events = body["events"].as_array().unwrap();
    assert!(
        events.iter().all(|ev| ev["task_id"] == t["task"]["id"]),
        "{body}"
    );
    assert_eq!(
        events[3]["change"], merged["main"],
        "merged carries the merge commit"
    );
    let ids: Vec<i64> = events.iter().map(|ev| ev["id"].as_i64().unwrap()).collect();
    assert!(ids.windows(2).all(|w| w[0] < w[1]), "{ids:?}");
    assert_eq!(body["next"], ids[3]);

    let (_, rest) = call(
        &e.app,
        "GET",
        &format!("/v1/events?repo=demo&since={}", ids[1]),
        None,
    )
    .await;
    assert_eq!(kinds(&rest), ["approve", "merged"]);
    let (_, other) = call(&e.app, "GET", "/v1/events?repo=elsewhere&since=0", None).await;
    assert!(kinds(&other).is_empty(), "{other}");
}

#[tokio::test(flavor = "multi_thread")]
async fn events_long_poll_returns_as_soon_as_an_event_lands() {
    let e = env().await;
    let t = start_task(&e, "Upper names").await;
    write_file(&e, &t, "app/util.py", UPPER);
    decide(&e, &t).await;
    let (_, now) = call(&e.app, "GET", "/v1/events?repo=demo", None).await;
    let since = now["next"].as_i64().unwrap();
    assert!(
        kinds(&now).is_empty(),
        "without since the feed starts at its head: {now}"
    );

    let app = e.app.clone();
    let started = std::time::Instant::now();
    let waiter = tokio::spawn(async move {
        call(
            &app,
            "GET",
            &format!("/v1/events?repo=demo&since={since}&wait=20"),
            None,
        )
        .await
    });
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    let (status, b) = submit(&e, &t).await;
    assert_eq!(status, StatusCode::OK, "{b}");
    let (status, body) = waiter.await.unwrap();
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(kinds(&body), ["submit"], "{body}");
    assert!(started.elapsed() < std::time::Duration::from_secs(5));
}

// ---- Plan 5 Task 4: carried approvals ----

const BASE_UTIL: &str = "def format_name(n):\n    return n.title()\n";

/// Task A (approved, appends `a_line` to util.py and writes `a_extra`) and task B (appends
/// `b_line`, writes `b_extra`) touch the same spot; B merges first, so A's merge conflicts.
/// Returns A, synced onto main with its conflict markers still in place.
async fn conflicted_after_approval(
    e: &Env,
    a_line: &str,
    b_line: &str,
    extra: Option<(&str, &str, &str)>,
) -> Value {
    let a = start_task(e, "Feature A").await;
    write_file(e, &a, "app/util.py", &format!("{BASE_UTIL}{a_line}"));
    if let Some((path, a_text, _)) = extra {
        write_file(e, &a, path, a_text);
    }
    decide(e, &a).await;
    assert_eq!(submit(e, &a).await.0, StatusCode::OK);
    validate(e, &a, 0).await;
    assert_eq!(review(e, &a, "reviewer", "approve").await.0, StatusCode::OK);

    let b = start_task(e, "Feature B").await;
    write_file(e, &b, "app/util.py", &format!("{BASE_UTIL}{b_line}"));
    if let Some((path, _, b_text)) = extra {
        write_file(e, &b, path, b_text);
    }
    decide(e, &b).await;
    assert_eq!(submit(e, &b).await.0, StatusCode::OK);
    validate(e, &b, 0).await;
    assert_eq!(review(e, &b, "reviewer", "approve").await.0, StatusCode::OK);
    let (status, body) = post(e, &b, "merge", json!({"agent": "merger"})).await;
    assert_eq!(body["status"], "merged", "{status} {body}");

    let (_, body) = post(e, &a, "merge", json!({"agent": "merger"})).await;
    assert_eq!(body["status"], "conflicted", "{body}");
    let (status, body) = post(e, &a, "sync", json!({"agent": "claude"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    a
}

/// Resolves A's conflicts by writing `files`, then checkpoints, validates and submits.
async fn resolve_and_resubmit(e: &Env, a: &Value, files: &[(&str, &str)]) -> Value {
    for (path, text) in files {
        write_file(e, a, path, text);
    }
    let (status, body) = post(e, a, "checkpoint", json!({})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    validate(e, a, 0).await;
    let (status, body) = submit(e, a).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body
}

async fn verdict_of(e: &Env, t: &Value) -> Value {
    let (_, got) = call(&e.app, "GET", &format!("/v1/tasks/{}", id(t)), None).await;
    got["verdict"].clone()
}

#[tokio::test(flavor = "multi_thread")]
async fn approval_carries_when_the_resolution_keeps_both_sides() {
    let e = env().await;
    let a = conflicted_after_approval(&e, "A = 1\n", "B = 2\n", None).await;
    resolve_and_resubmit(
        &e,
        &a,
        &[("app/util.py", &format!("{BASE_UTIL}B = 2\nA = 1\n"))],
    )
    .await;
    let v = verdict_of(&e, &a).await;
    assert_eq!(v["verdict"], "approve", "{v}");
    assert_eq!(v["reviewer"], "av", "carried, not re-reviewed: {v}");
    let (status, body) = post(&e, &a, "merge", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["status"], "merged", "{body}");
    assert_eq!(
        main_file(&e, "app/util.py").await,
        format!("{BASE_UTIL}B = 2\nA = 1\n")
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn approval_does_not_carry_when_the_resolution_changes_the_task_code() {
    let e = env().await;
    let a = conflicted_after_approval(&e, "A = 1\n", "B = 2\n", None).await;
    resolve_and_resubmit(
        &e,
        &a,
        &[("app/util.py", &format!("{BASE_UTIL}B = 2\nA = 100\n"))],
    )
    .await;
    assert!(
        verdict_of(&e, &a).await.is_null(),
        "a code change needs a new review"
    );
    let (status, body) = post(&e, &a, "merge", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(
        body["error"].as_str().unwrap().contains("no approval"),
        "{body}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn approval_carries_over_a_reworded_markdown_resolution() {
    let e = env().await;
    let extra = Some(("NOTES.md", "from a\n", "from b\n"));
    let a = conflicted_after_approval(&e, "A = 1\n", "", extra).await;
    resolve_and_resubmit(
        &e,
        &a,
        &[
            ("app/util.py", &format!("{BASE_UTIL}A = 1\n")),
            ("NOTES.md", "from b\nand from a, reworded\n"),
        ],
    )
    .await;
    let v = verdict_of(&e, &a).await;
    assert_eq!(v["reviewer"], "av", "NOTES.md is in carry_globs: {v}");
    let (_, body) = post(&e, &a, "merge", json!({"agent": "merger"})).await;
    assert_eq!(body["status"], "merged", "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_request_for_changes_after_the_approval_blocks_the_carry() {
    let e = env().await;
    let a = conflicted_after_approval(&e, "A = 1\n", "B = 2\n", None).await;
    // The reviewer changes their mind before the resolution lands.
    let (_, task) = call(&e.app, "GET", &format!("/v1/tasks/{}", id(&a)), None).await;
    let head = task["task"]["head_change"].as_str().unwrap().to_owned();
    e.store
        .add_verdict(
            scholion_core::TaskId(uuid::Uuid::parse_str(&id(&a)).unwrap()),
            "reviewer",
            scholion_core::Verdict::RequestChanges,
            &scholion_core::ChangeId(head),
            None,
            None,
        )
        .await
        .unwrap();
    resolve_and_resubmit(
        &e,
        &a,
        &[("app/util.py", &format!("{BASE_UTIL}B = 2\nA = 1\n"))],
    )
    .await;
    assert!(verdict_of(&e, &a).await.is_null());
}

#[tokio::test(flavor = "multi_thread")]
async fn repo_settings_default_then_update_and_reject_bad_globs() {
    let e = env().await;
    let (status, body) = call(&e.app, "GET", "/v1/repos/demo/settings", None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(
        body,
        json!({"carry_globs": ["**/*.md"], "check_command": null})
    );
    let (status, body) = call(
        &e.app,
        "PUT",
        "/v1/repos/demo/settings",
        Some(json!({"carry_globs": ["docs/**", "*.md"], "check_command": "pytest -q"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (_, body) = call(&e.app, "GET", "/v1/repos/demo/settings", None).await;
    assert_eq!(body["carry_globs"], json!(["docs/**", "*.md"]));
    assert_eq!(body["check_command"], "pytest -q");
    let (status, body) = call(
        &e.app,
        "PUT",
        "/v1/repos/demo/settings",
        Some(json!({"carry_globs": ["[unclosed"]})),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
}

// ---- Plan 5 Task 5: enforced conventions ----

async fn add_conv(e: &Env, glob: &str, text: &str, check: Option<&str>) -> i64 {
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/repos/demo/conventions",
        Some(json!({"scope_glob": glob, "text": text, "required": true, "check": check})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body["id"].as_i64().unwrap()
}

async fn validate_with(e: &Env, t: &Value, checks: Value) {
    let (status, body) = post(
        e,
        t,
        "validate",
        json!({"agent": "claude", "command": "pytest", "exit_code": 0, "checks": checks}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_required_convention_check_must_pass_before_approval() {
    let e = env().await;
    let conv = add_conv(
        &e,
        "app/**/*.py",
        "docstrings everywhere",
        Some("python lint.py"),
    )
    .await;
    add_conv(&e, "docs/**", "short docs", Some("true")).await; // does not match the change
    add_conv(&e, "app/**", "no check, never blocks", None).await;
    let t = start_task(&e, "Upper names").await;
    write_file(&e, &t, "app/util.py", UPPER);
    decide(&e, &t).await;
    assert_eq!(submit(&e, &t).await.0, StatusCode::OK);

    let (status, body) = call(&e.app, "GET", &format!("/v1/tasks/{}/checks", id(&t)), None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let checks = body["checks"].as_array().unwrap();
    assert_eq!(checks.len(), 1, "only the matching required check: {body}");
    assert_eq!(checks[0]["convention_id"], conv);
    assert_eq!(checks[0]["command"], "python lint.py");

    validate(&e, &t, 0).await; // tests only: the check was not run
    let (status, body) = review(&e, &t, "reviewer", "approve").await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    let msg = body["error"].as_str().unwrap();
    assert!(
        msg.contains(&format!("conv:{conv}")) && msg.contains("not run"),
        "{msg}"
    );

    validate_with(
        &e,
        &t,
        json!([{"convention_id": conv, "exit_code": 1, "output_tail": "missing docstring"}]),
    )
    .await;
    let (status, body) = review(&e, &t, "reviewer", "approve").await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(body["error"].as_str().unwrap().contains("failed"), "{body}");

    validate_with(&e, &t, json!([{"convention_id": conv, "exit_code": 0}])).await;
    let (status, body) = review(&e, &t, "reviewer", "approve").await;
    assert_eq!(status, StatusCode::OK, "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_removed_convention_is_no_longer_listed_or_enforced() {
    let e = env().await;
    let conv = add_conv(
        &e,
        "app/**/*.py",
        "docstrings everywhere",
        Some("python lint.py"),
    )
    .await;
    let (status, body) = call(
        &e.app,
        "DELETE",
        &format!("/v1/repos/demo/conventions/{conv}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (_, list) = call(&e.app, "GET", "/v1/repos/demo/conventions", None).await;
    assert!(list["conventions"].as_array().unwrap().is_empty(), "{list}");
    let t = ready_to_merge(&e, "Upper names", UPPER).await; // approve succeeds without the check
    let (_, body) = post(&e, &t, "merge", json!({"agent": "merger"})).await;
    assert_eq!(body["status"], "merged", "{body}");
}

// ---- Plan 5 Task 6: post-merge checks ----

#[tokio::test(flavor = "multi_thread")]
async fn main_health_follows_posted_checks_and_names_the_culprit() {
    let e = env().await;
    let (_, body) = call(&e.app, "GET", "/v1/repos/demo/health", None).await;
    assert_eq!(body["state"], "unknown", "{body}");

    let t = ready_to_merge(&e, "Upper names", UPPER).await;
    let (_, merged) = post(&e, &t, "merge", json!({"agent": "merger"})).await;
    let main = merged["main"].as_str().unwrap().to_owned();
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/repos/demo/checks",
        Some(
            json!({"change": main, "command": "pytest -q", "exit_code": 1,
                    "output_tail": "1 failed", "runner": "runner-1"}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (_, body) = call(&e.app, "GET", "/v1/repos/demo/health", None).await;
    assert_eq!(body["state"], "red", "{body}");
    assert_eq!(body["since"], main.as_str(), "{body}");
    assert_eq!(
        body["task"], t["task"]["slug"],
        "the merge's task is the culprit: {body}"
    );

    let (_, log) = call(&e.app, "GET", "/v1/repos/demo/log?limit=5", None).await;
    assert_eq!(log["commits"][0]["check"]["exit_code"], 1, "{log}");
    assert!(log["commits"][1]["check"].is_null(), "{log}");

    // a re-run that passes turns main green (latest result per change and command wins)
    call(
        &e.app,
        "POST",
        "/v1/repos/demo/checks",
        Some(json!({"change": main, "command": "pytest -q", "exit_code": 0, "runner": "runner-1"})),
    )
    .await;
    let (_, body) = call(&e.app, "GET", "/v1/repos/demo/health", None).await;
    assert_eq!(body["state"], "green", "{body}");

    let (status, raw) = call(
        &e.app,
        "GET",
        "/v1/repos/demo/files/app/util.py?raw=1",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(raw, json!(UPPER), "raw bytes, not JSON");
}

#[tokio::test(flavor = "multi_thread")]
async fn packs_warn_when_main_is_red() {
    let e = env().await;
    let t = ready_to_merge(&e, "Upper names", UPPER).await;
    let (_, merged) = post(&e, &t, "merge", json!({"agent": "merger"})).await;
    call(
        &e.app,
        "POST",
        "/v1/repos/demo/checks",
        Some(json!({"change": merged["main"], "command": "pytest -q", "exit_code": 2})),
    )
    .await;
    let later = start_task(&e, "Later work").await;
    let (_, pack) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/context?budget=2000", id(&later)),
        None,
    )
    .await;
    let md = pack["markdown"].as_str().unwrap();
    assert!(
        md.contains("main is red") && md.contains("upper-names") && md.contains("pytest -q"),
        "{md}"
    );
}

// ---- Plan 5 Task 7: revert any merge ----

const LOUD_GREETER: &str = "from util import format_name\n\nclass Greeter:\n    def hello(self, name):\n        return format_name(name) + \"!\"\n";

async fn merged_task(e: &Env, title: &str, path: &str, text: &str) -> (Value, Value) {
    let t = start_task(e, title).await;
    write_file(e, &t, path, text);
    decide(e, &t).await;
    assert_eq!(submit(e, &t).await.0, StatusCode::OK);
    validate(e, &t, 0).await;
    assert_eq!(review(e, &t, "reviewer", "approve").await.0, StatusCode::OK);
    let (_, merged) = post(e, &t, "merge", json!({"agent": "merger"})).await;
    assert_eq!(merged["status"], "merged", "{merged}");
    (t, merged)
}

#[tokio::test(flavor = "multi_thread")]
async fn revert_undoes_an_older_merge_and_the_task_can_merge_again() {
    let e = env().await;
    let (a, _) = merged_task(&e, "Upper names", "app/util.py", UPPER).await;
    let (_b, _) = merged_task(&e, "Loud greeter", "app/greeter.py", LOUD_GREETER).await;

    let (status, body) = post(
        &e,
        &a,
        "revert",
        json!({"agent": "merger", "reason": "broke main"}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["status"], "open", "back to its owner to fix: {body}");
    assert_eq!(
        main_file(&e, "app/util.py").await,
        BASE_UTIL,
        "A's change is gone"
    );
    assert_eq!(
        main_file(&e, "app/greeter.py").await,
        LOUD_GREETER,
        "B's change stays"
    );
    let (_, log) = call(&e.app, "GET", "/v1/repos/demo/log?limit=1", None).await;
    let msg = log["commits"][0]["description"].as_str().unwrap();
    assert!(
        msg.starts_with("Revert Upper names") && msg.contains(&format!("av revert {}", id(&a))),
        "{msg}"
    );
    assert!(
        !msg.contains(&format!("av task {}", id(&a))),
        "a revert never looks like the merge"
    );
    let (_, feed) = call(
        &e.app,
        "GET",
        "/v1/events?repo=demo&since=0&limit=100",
        None,
    )
    .await;
    assert!(
        feed["events"]
            .as_array()
            .unwrap()
            .iter()
            .any(|ev| ev["kind"] == "revert"),
        "{feed}"
    );

    // The owner resubmits; the merge applies the change again instead of "recovering" the old one.
    assert_eq!(submit(&e, &a).await.0, StatusCode::OK);
    validate(&e, &a, 0).await;
    assert_eq!(
        review(&e, &a, "reviewer", "approve").await.0,
        StatusCode::OK
    );
    let (_, merged) = post(&e, &a, "merge", json!({"agent": "merger"})).await;
    assert_eq!(merged["status"], "merged", "{merged}");
    assert_eq!(merged["recovered"], false, "{merged}");
    assert_eq!(main_file(&e, "app/util.py").await, UPPER);
}

#[tokio::test(flavor = "multi_thread")]
async fn a_revert_that_conflicts_leaves_main_untouched() {
    let e = env().await;
    let (a, _) = merged_task(&e, "Upper names", "app/util.py", UPPER).await;
    merged_task(&e, "Lower names", "app/util.py", LOWER).await;
    let (status, body) = post(&e, &a, "revert", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(
        body["error"].as_str().unwrap().contains("app/util.py"),
        "{body}"
    );
    assert_eq!(main_file(&e, "app/util.py").await, LOWER);
    let (_, t) = call(&e.app, "GET", &format!("/v1/tasks/{}", id(&a)), None).await;
    assert_eq!(t["task"]["status"], "merged");
}

// ---- Plan 5 Task 8: derived data, `why` ----

#[tokio::test(flavor = "multi_thread")]
async fn why_names_the_task_and_decisions_behind_a_line() {
    let e = env().await;
    // line 2 of util.py changes in A; greeter.py changes in B
    let (a, _) = merged_task(&e, "Upper names", "app/util.py", UPPER).await;
    let (b, _) = merged_task(&e, "Loud greeter", "app/greeter.py", LOUD_GREETER).await;

    let (status, body) = call(
        &e.app,
        "GET",
        "/v1/repos/demo/why?path=app/util.py&line=2",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["task"]["slug"], a["task"]["slug"], "{body}");
    assert_eq!(body["line_text"], "    return n.upper()", "{body}");
    let records = body["records"].as_array().unwrap();
    assert!(
        records.iter().any(|r| r["kind"] == "decision"),
        "the decision recorded on that task comes along: {body}"
    );

    // line 1 was never touched by a task: it comes from the imported history
    let (_, body) = call(
        &e.app,
        "GET",
        "/v1/repos/demo/why?path=app/util.py&line=1",
        None,
    )
    .await;
    assert!(body["task"].is_null(), "{body}");
    assert_eq!(body["commit_description"], "init", "{body}");

    // the last line of greeter.py is B's
    let (_, body) = call(
        &e.app,
        "GET",
        "/v1/repos/demo/why?path=app/greeter.py&line=5",
        None,
    )
    .await;
    assert_eq!(body["task"]["slug"], b["task"]["slug"], "{body}");

    let (status, _) = call(
        &e.app,
        "GET",
        "/v1/repos/demo/why?path=app/util.py&line=99",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    // computed once, then served from the derived cache (keyed by the immutable commit)
    let cached: i64 = sqlx::query_scalar("SELECT count(*) FROM derived WHERE kind = 'blame_v1'")
        .fetch_one(e.store.pool())
        .await
        .unwrap();
    assert!(cached >= 1, "blame is cached");
}

#[tokio::test(flavor = "multi_thread")]
async fn why_follows_a_line_through_later_changes_from_the_cache() {
    let e = env().await;
    merged_task(&e, "Upper names", "app/util.py", UPPER).await;
    let (_, first) = call(
        &e.app,
        "GET",
        "/v1/repos/demo/why?path=app/util.py&line=2",
        None,
    )
    .await;
    assert_eq!(first["task"]["slug"], "upper-names", "{first}");
    let (lower, _) = merged_task(&e, "Lower names", "app/util.py", LOWER).await;
    let (_, second) = call(
        &e.app,
        "GET",
        "/v1/repos/demo/why?path=app/util.py&line=2",
        None,
    )
    .await;
    assert_eq!(second["task"]["slug"], lower["task"]["slug"], "{second}");
    assert_eq!(second["line_text"], "    return n.lower()");
    let (_, line1) = call(
        &e.app,
        "GET",
        "/v1/repos/demo/why?path=app/util.py&line=1",
        None,
    )
    .await;
    assert_eq!(
        line1["commit_description"], "init",
        "untouched lines keep their origin: {line1}"
    );
}

// ---- Plan 5 Task 10: conflict markers block submit ----

#[tokio::test(flavor = "multi_thread")]
async fn submit_refuses_leftover_conflict_markers() {
    let e = env().await;
    let t = start_task(&e, "Upper names").await;
    write_file(
        &e,
        &t,
        "app/util.py",
        "def format_name(n):\n<<<<<<< Conflict 1 of 1\n%%%%%%% Changes from base to side #1\n-    return n.title()\n+    return n.upper()\n+++++++ Contents of side #2\n    return n.lower()\n>>>>>>> Conflict 1 of 1 ends\n",
    );
    decide(&e, &t).await;
    let (status, body) = submit(&e, &t).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    let msg = body["error"].as_str().unwrap();
    assert!(
        msg.contains("app/util.py:2") && msg.contains("conflict marker"),
        "{msg}"
    );

    // a markdown underline or a diff in a docstring is not a marker
    write_file(
        &e,
        &t,
        "app/util.py",
        "def format_name(n):\n    \"\"\"Title\n    =======\n    \"\"\"\n    return n.upper()\n",
    );
    let (status, body) = submit(&e, &t).await;
    assert_eq!(status, StatusCode::OK, "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn context_with_code_signatures_shows_where_code_is_but_not_its_body() {
    let e = env().await;
    let t = start_task(&e, "Sigs").await;
    let uri = |extra: &str, s: &str| {
        format!(
            "/v1/tasks/{}/context?budget=3000&focus=format_name&session={s}{extra}",
            id(&t)
        )
    };
    let (_, full) = call(&e.app, "GET", &uri("&code=full", "sig-a"), None).await;
    assert!(
        full["markdown"].as_str().unwrap().contains("n.title()"),
        "{full}"
    );
    let (status, sigs) = call(&e.app, "GET", &uri("&code=signatures", "sig-b"), None).await;
    assert_eq!(status, StatusCode::OK, "{sigs}");
    let md = sigs["markdown"].as_str().unwrap();
    assert!(md.contains("def format_name(n):"), "{md}");
    assert!(!md.contains("n.title()"), "{md}");
}

#[tokio::test]
async fn context_can_leave_out_the_map_and_cap_the_code_items() {
    let e = env().await;
    let t = start_task(&e, "Lean").await;
    let uri = |extra: &str, s: &str| {
        format!(
            "/v1/tasks/{}/context?budget=3000&focus=format_name&session={s}{extra}",
            id(&t)
        )
    };
    let (_, all) = call(&e.app, "GET", &uri("", "lean-a"), None).await;
    let all = all["markdown"].as_str().unwrap();
    assert!(all.contains("## Map"), "{all}");
    let (status, lean) = call(&e.app, "GET", &uri("&map=false&max_code=1", "lean-b"), None).await;
    assert_eq!(status, StatusCode::OK, "{lean}");
    let md = lean["markdown"].as_str().unwrap();
    assert!(!md.contains("## Map"), "{md}");
    assert_eq!(md.matches("- sym:").count(), 1, "{md}");
}

// ---- Dashboard (`/ui`) and the listings it reads ----

#[tokio::test(flavor = "multi_thread")]
async fn list_tasks_returns_newest_first_with_record_counts_and_filters() {
    let e = env().await;
    let a = start_task(&e, "First task").await;
    let b = start_task(&e, "Second task").await;
    call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/records", id(&a)),
        Some(json!({"kind": "note", "summary": "a note", "agent": "claude"})),
    )
    .await;
    call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/abandon", id(&b)),
        Some(json!({"agent": "claude"})),
    )
    .await;

    let (status, body) = call(&e.app, "GET", "/v1/tasks?repo=demo", None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let tasks = body["tasks"].as_array().unwrap();
    assert_eq!(tasks.len(), 2, "{body}");
    assert_eq!(tasks[0]["id"], b["task"]["id"], "newest first: {body}");
    assert_eq!(tasks[1]["title"], "First task");
    assert!(tasks[1]["created_at"].is_string(), "{body}");
    assert!(tasks[1]["record_count"].as_u64().unwrap() >= 1, "{body}");

    let (_, open) = call(&e.app, "GET", "/v1/tasks?repo=demo&status=open", None).await;
    let open = open["tasks"].as_array().unwrap();
    assert_eq!(open.len(), 1, "{open:?}");
    assert_eq!(open[0]["id"], a["task"]["id"]);

    let (_, other) = call(&e.app, "GET", "/v1/tasks?repo=nope", None).await;
    assert!(other["tasks"].as_array().unwrap().is_empty(), "{other}");

    let (status, body) = call(&e.app, "GET", "/v1/tasks?status=bogus", None).await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn list_records_returns_them_in_order_with_anchors() {
    let e = env().await;
    let t = start_task(&e, "Records listing").await;
    for (kind, summary) in [
        ("decision", "keep title case"),
        ("attempt", "tried caching"),
    ] {
        let (status, body) = call(
            &e.app,
            "POST",
            &format!("/v1/tasks/{}/records", id(&t)),
            Some(json!({"kind": kind, "summary": summary, "agent": "claude",
                        "anchors": [{"path": "app/greeter.py", "symbol": "Greeter.hello"}]})),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
    }
    let (status, body) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/records", id(&t)),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let recs = body["records"].as_array().unwrap();
    let kinds: Vec<&str> = recs.iter().map(|r| r["kind"].as_str().unwrap()).collect();
    let n = kinds.len();
    assert_eq!(&kinds[n - 2..], ["decision", "attempt"], "{body}");
    assert_eq!(
        recs[n - 1]["anchors"][0]["symbol"],
        "Greeter.hello",
        "{body}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn ui_page_is_served_without_a_token_but_its_data_is_not() {
    use tower::ServiceExt as _;
    let e = env_with(|s| s.with_token(Some("sekrit".into()))).await;
    let resp = e
        .app
        .clone()
        .oneshot(Request::get("/ui").body(axum::body::Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let ct = resp.headers()["content-type"].to_str().unwrap().to_owned();
    assert!(ct.starts_with("text/html"), "{ct}");
    assert!(resp.headers().contains_key("content-security-policy"));
    let html = axum::body::to_bytes(resp.into_body(), 1 << 20)
        .await
        .unwrap();
    let html = String::from_utf8_lossy(&html);
    assert!(html.contains("<title>"), "{html}");
    assert!(html.contains("/v1/tasks"), "the page reads the task list");
    assert!(
        html.contains("/v1/backlog"),
        "the page shows the backlog and who each item is reserved for"
    );
    assert!(
        html.contains("/unrecorded"),
        "the task drawer shows what no record explains"
    );
    // the pack preview starts at the budget agents get by default (`av context`, MCP)
    assert!(
        html.contains("[800, 2000, 4000, 8000]") && html.contains("selected: b === 800"),
        "the preview defaults to the agents' 800-token budget"
    );
    assert!(!html.contains("|| 4000"), "no stale 4000 fallback");

    let (status, _) = call(&e.app, "GET", "/v1/tasks", None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test(flavor = "multi_thread")]
async fn ui_page_reads_the_team_profiles() {
    use tower::ServiceExt as _;
    let e = env().await;
    let resp = e
        .app
        .clone()
        .oneshot(Request::get("/ui").body(axum::body::Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let html = axum::body::to_bytes(resp.into_body(), 1 << 20)
        .await
        .unwrap();
    let html = String::from_utf8_lossy(&html);
    assert!(html.contains("/v1/agents"), "the page reads team profiles");
    assert!(
        html.contains("max_open"),
        "the page shows a team's open-task limit"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn reindex_refreshes_mains_code_index_and_skips_unchanged_files() {
    let e = env().await;
    // what migration 0020 does to an index built before constants were symbols
    sqlx::query("UPDATE index_files SET content_hash = ''")
        .execute(e.store.pool())
        .await
        .unwrap();
    let (status, body) = call(&e.app, "POST", "/v1/reindex", Some(json!({"repo": "demo"}))).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let refreshed = body["code_files_reindexed"].as_u64().unwrap();
    assert!(refreshed >= 2, "both .py files are read again: {body}");

    let (_, body) = call(&e.app, "POST", "/v1/reindex", Some(json!({"repo": "demo"}))).await;
    assert_eq!(
        body["code_files_reindexed"], 0,
        "nothing changed since: {body}"
    );

    let (_, body) = call(&e.app, "POST", "/v1/reindex", None).await;
    assert_eq!(body["code_files_reindexed"], 0, "all repos, same: {body}");
}

// ---- just-in-time records (handoff B6): what others recorded about the file an agent opens ----

async fn record_on(e: &Env, t: &Value, kind: &str, summary: &str, path: &str) -> Value {
    let (status, body) = post(
        e,
        t,
        "records",
        json!({"kind": kind, "summary": summary, "agent": "claude",
               "anchors": [{"path": path}]}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body
}

async fn records_at(e: &Env, t: &Value, path: &str, session: &str) -> Value {
    let (status, body) = call(
        &e.app,
        "GET",
        &format!(
            "/v1/tasks/{}/records/at?path={path}&session={session}",
            id(t)
        ),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body
}

#[tokio::test(flavor = "multi_thread")]
async fn records_at_a_path_are_other_tasks_records_anchored_there() {
    let e = env().await;
    let a = start_task(&e, "Earlier work").await;
    record_on(
        &e,
        &a,
        "attempt",
        "tried caching format_name; names went stale",
        "app/util.py",
    )
    .await;
    record_on(
        &e,
        &a,
        "decision",
        "greeter keeps its own punctuation",
        "app/greeter.py",
    )
    .await;
    let b = start_task(&e, "Later work").await;
    record_on(&e, &b, "note", "my own note on util", "app/util.py").await;

    let body = records_at(&e, &b, "app/util.py", "jit-1").await;
    let md = body["markdown"].as_str().unwrap();
    assert!(md.contains("tried caching format_name"), "{md}");
    assert!(md.contains("attempt"), "{md}");
    assert!(!md.contains("greeter keeps"), "other path: {md}");
    assert!(!md.contains("my own note"), "the task's own records: {md}");
    assert!(md.contains("av"), "says how to see more: {md}");
    assert_eq!(body["records"].as_array().unwrap().len(), 1, "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn records_at_a_path_leave_out_superseded_records() {
    let e = env().await;
    let a = start_task(&e, "Earlier work").await;
    let old = record_on(&e, &a, "decision", "names stay lower case", "app/util.py").await;
    let (status, body) = post(
        &e,
        &a,
        "records",
        json!({"kind": "decision", "summary": "names are upper case now", "agent": "claude",
               "anchors": [{"path": "app/util.py"}], "supersedes": old["id"]}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let b = start_task(&e, "Later work").await;
    let md = records_at(&e, &b, "app/util.py", "jit-1").await["markdown"]
        .as_str()
        .unwrap()
        .to_owned();
    assert!(md.contains("upper case now"), "{md}");
    assert!(!md.contains("stay lower case"), "{md}");
}

#[tokio::test(flavor = "multi_thread")]
async fn records_at_a_path_are_sent_once_per_session() {
    let e = env().await;
    let a = start_task(&e, "Earlier work").await;
    record_on(
        &e,
        &a,
        "attempt",
        "tried caching format_name",
        "app/util.py",
    )
    .await;
    let b = start_task(&e, "Later work").await;
    assert_ne!(
        records_at(&e, &b, "app/util.py", "jit-1").await["markdown"],
        ""
    );
    let again = records_at(&e, &b, "app/util.py", "jit-1").await;
    assert_eq!(again["markdown"], "", "{again}");
    assert_ne!(
        records_at(&e, &b, "app/util.py", "jit-2").await["markdown"],
        "",
        "another session has not seen it"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn records_at_a_path_skip_what_a_pack_already_sent() {
    let e = env().await;
    let a = start_task(&e, "Earlier work").await;
    let (status, body) = post(
        &e,
        &a,
        "records",
        json!({"kind": "attempt", "summary": "tried caching format_name", "agent": "claude",
               "anchors": [{"path": "app/util.py", "symbol": "format_name"}]}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let b = start_task(&e, "Later work").await;
    write_file(&e, &b, "app/util.py", UPPER);
    let (status, body) = post(&e, &b, "checkpoint", json!({})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, pack) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/context?budget=4000&session=jit-3", id(&b)),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{pack}");
    assert!(
        pack["markdown"].as_str().unwrap().contains("tried caching"),
        "precondition: the pack carries it: {pack}"
    );
    let body = records_at(&e, &b, "app/util.py", "jit-3").await;
    assert_eq!(body["markdown"], "", "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn records_at_a_path_without_records_is_empty() {
    let e = env().await;
    let b = start_task(&e, "Later work").await;
    let body = records_at(&e, &b, "app/util.py", "jit-1").await;
    assert_eq!(body["markdown"], "");
    assert_eq!(body["records"], json!([]));
}

// ---- record-writing nudge (handoff B7): what a Stop hook asks the agent to record ----

async fn unrecorded(e: &Env, t: &Value) -> Value {
    let (status, body) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/unrecorded", id(t)),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body
}

#[tokio::test(flavor = "multi_thread")]
async fn unrecorded_names_changed_symbols_no_record_explains() {
    let e = env().await;
    let t = start_task(&e, "Upper names").await;
    let body = unrecorded(&e, &t).await;
    assert_eq!(body["status"], "open");
    assert_eq!(body["unrecorded_symbols"], json!([]), "nothing changed yet");

    write_file(&e, &t, "app/util.py", UPPER);
    let (status, body) = post(&e, &t, "checkpoint", json!({})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let body = unrecorded(&e, &t).await;
    assert_eq!(body["explained"], false, "{body}");
    assert_eq!(body["unrecorded_symbols"], json!(["format_name"]), "{body}");

    decide(&e, &t).await; // anchored on app/util.py:format_name
    let body = unrecorded(&e, &t).await;
    assert_eq!(body["explained"], true, "{body}");
    assert_eq!(body["unrecorded_symbols"], json!([]), "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn submit_still_reports_the_same_unrecorded_symbols() {
    let e = env().await;
    let t = start_task(&e, "Upper names").await;
    write_file(&e, &t, "app/util.py", UPPER);
    let (status, body) = post(
        &e,
        &t,
        "records",
        json!({"kind": "note", "summary": "a note on the greeter", "agent": "claude",
               "anchors": [{"path": "app/greeter.py"}]}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = submit(&e, &t).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["unrecorded_symbols"], json!(["format_name"]), "{body}");
}

const GREETER_TEST: &str = "from greeter import Greeter\n\n\ndef test_greets():\n    assert Greeter().hello(\"ada\") == \"Ada\"\n";

fn write_greeter_test(e: &Env, t: &Value) {
    std::fs::create_dir_all(ws_dir(e, t).join("tests")).unwrap();
    write_file(e, t, "tests/test_greeter.py", GREETER_TEST);
}

#[tokio::test(flavor = "multi_thread")]
async fn unrecorded_leaves_out_test_functions() {
    let e = env().await;
    let t = start_task(&e, "Test the greeter").await;
    write_greeter_test(&e, &t);
    let (status, body) = post(&e, &t, "checkpoint", json!({})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let body = unrecorded(&e, &t).await;
    assert_eq!(body["unrecorded_symbols"], json!([]), "{body}");
    assert_eq!(
        body["explained"], false,
        "the submit gate is unchanged: {body}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn unrecorded_names_real_code_but_not_its_tests() {
    let e = env().await;
    let t = start_task(&e, "Louder greeter and its test").await;
    write_file(
        &e,
        &t,
        "app/greeter.py",
        "from util import format_name\n\nclass Greeter:\n    def hello(self, name):\n        return format_name(name) + \"!\"\n",
    );
    write_greeter_test(&e, &t);
    let (status, body) = post(&e, &t, "checkpoint", json!({})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let body = unrecorded(&e, &t).await;
    // the class counts as changed too (its range covers the edited method)
    assert_eq!(
        body["unrecorded_symbols"],
        json!(["Greeter", "Greeter.hello"]),
        "{body}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn unrecorded_leaves_out_rust_unit_tests_inside_a_source_file() {
    let e = env().await;
    let t = start_task(&e, "Rust sums").await;
    std::fs::create_dir_all(ws_dir(&e, &t).join("src")).unwrap();
    write_file(
        &e,
        &t,
        "src/sum.rs",
        "pub fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n\n#[test]\nfn adds() {\n    assert_eq!(add(1, 2), 3);\n}\n",
    );
    let (status, body) = post(&e, &t, "checkpoint", json!({})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let body = unrecorded(&e, &t).await;
    assert_eq!(body["unrecorded_symbols"], json!(["add"]), "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn submit_report_leaves_out_rust_unit_tests_without_a_checkpoint() {
    // the gate must read source at the head the submit just snapshotted, not the stale one
    let e = env().await;
    let t = start_task(&e, "Rust sums").await;
    std::fs::create_dir_all(ws_dir(&e, &t).join("src")).unwrap();
    write_file(
        &e,
        &t,
        "src/sum.rs",
        "pub fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n\n#[test]\nfn adds() {\n    assert_eq!(add(1, 2), 3);\n}\n",
    );
    let (status, body) = submit_with(
        &e,
        &t,
        json!([{"kind": "note", "summary": "a note on nothing in particular"}]),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["unrecorded_symbols"], json!(["add"]), "{body}");
}

// ---- one budget law: work packs are 800 tokens of signatures unless asked ----

#[tokio::test(flavor = "multi_thread")]
async fn work_packs_default_to_800_tokens_of_signatures() {
    let e = env().await;
    let t = start_task(&e, "Defaults").await;
    let ctx = |q: String| format!("/v1/tasks/{}/context?focus=format_name{q}", id(&t));
    let (status, default) = call(&e.app, "GET", &ctx("&session=d-a".into()), None).await;
    assert_eq!(status, StatusCode::OK, "{default}");
    let (_, explicit) = call(
        &e.app,
        "GET",
        &ctx("&session=d-b&budget=800&code=signatures".into()),
        None,
    )
    .await;
    assert_eq!(default["markdown"], explicit["markdown"]);
    let (_, larger) = call(&e.app, "GET", &ctx("&session=d-c&budget=4000".into()), None).await;
    assert_ne!(
        default["markdown"], larger["markdown"],
        "the default is not 4000"
    );

    let (status, started) = call(
        &e.app,
        "POST",
        "/v1/tasks",
        Some(
            json!({"repo": "demo", "title": "Started", "goal": "Make hello louder",
                    "agent": "claude", "session": "d-start"}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{started}");
    let (_, same) = call(
        &e.app,
        "GET",
        &format!(
            "/v1/tasks/{}/context?session=d-again&budget=800&code=signatures",
            id(&started)
        ),
        None,
    )
    .await;
    assert_eq!(started["pack"]["markdown"], same["markdown"]);
}

/// A guard that refuses one kind of action and remembers what it was asked.
struct Refuse {
    approve: bool,
    asked: std::sync::Mutex<Vec<String>>,
}

#[async_trait::async_trait]
impl scholion_daemon::MergeGuard for Refuse {
    async fn check(&self, action: scholion_daemon::Guarded<'_>) -> Result<(), String> {
        let (what, refuse) = match action {
            scholion_daemon::Guarded::Approve { reviewer, .. } => {
                (format!("approve by {reviewer}"), self.approve)
            }
            scholion_daemon::Guarded::Merge { agent, .. } => {
                (format!("merge by {agent}"), !self.approve)
            }
            // this guard is about approvals and merges only
            scholion_daemon::Guarded::Submit { .. } => return Ok(()),
        };
        self.asked.lock().unwrap().push(what);
        if refuse {
            Err("held for a human".into())
        } else {
            Ok(())
        }
    }
}

async fn ready_for_review(e: &Env) -> Value {
    let t = start_task(e, "Upper names").await;
    write_file(e, &t, "app/util.py", UPPER);
    decide(e, &t).await;
    submit(e, &t).await;
    validate(e, &t, 0).await;
    t
}

#[tokio::test(flavor = "multi_thread")]
async fn a_merge_guard_can_refuse_an_approval() {
    let guard = std::sync::Arc::new(Refuse {
        approve: true,
        asked: Default::default(),
    });
    let g = guard.clone();
    let e = env_with(move |s| s.with_merge_guard(g)).await;
    let t = ready_for_review(&e).await;
    let (status, body) = review(&e, &t, "reviewer", "approve").await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(
        body["error"]
            .as_str()
            .unwrap()
            .contains("cannot approve: held for a human"),
        "{body}"
    );
    assert_eq!(*guard.asked.lock().unwrap(), ["approve by reviewer"]);
    let (_, task) = call(&e.app, "GET", &format!("/v1/tasks/{}", id(&t)), None).await;
    assert_eq!(task["task"]["status"], "in_review", "{task}");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_merge_guard_can_refuse_a_merge_and_main_stays() {
    let guard = std::sync::Arc::new(Refuse {
        approve: false,
        asked: Default::default(),
    });
    let g = guard.clone();
    let e = env_with(move |s| s.with_merge_guard(g)).await;
    let t = ready_for_review(&e).await;
    let (status, body) = review(&e, &t, "reviewer", "approve").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = post(&e, &t, "merge", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(
        body["error"]
            .as_str()
            .unwrap()
            .contains("cannot merge: held for a human"),
        "{body}"
    );
    assert_eq!(
        *guard.asked.lock().unwrap(),
        ["approve by reviewer", "merge by merger"]
    );
    let (_, task) = call(&e.app, "GET", &format!("/v1/tasks/{}", id(&t)), None).await;
    assert_eq!(task["task"]["status"], "in_review", "{task}");
}

/// A guard that keeps tasks out of one directory and remembers every submit it was asked about.
struct Scope {
    forbidden: &'static str,
    asked: std::sync::Mutex<Vec<Vec<String>>>,
}

#[async_trait::async_trait]
impl scholion_daemon::MergeGuard for Scope {
    async fn check(&self, action: scholion_daemon::Guarded<'_>) -> Result<(), String> {
        let scholion_daemon::Guarded::Submit { changed_paths, .. } = action else {
            return Ok(());
        };
        self.asked.lock().unwrap().push(changed_paths.to_vec());
        match changed_paths.iter().find(|p| p.starts_with(self.forbidden)) {
            Some(p) => Err(format!("{p} is outside this persona's paths")),
            None => Ok(()),
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn a_guard_can_refuse_a_submit_by_its_changed_paths() {
    let guard = std::sync::Arc::new(Scope {
        forbidden: "app/",
        asked: Default::default(),
    });
    let g = guard.clone();
    let e = env_with(move |s| s.with_merge_guard(g)).await;

    let t = start_task(&e, "Upper names").await;
    write_file(&e, &t, "app/util.py", UPPER);
    decide(&e, &t).await;
    let (status, body) = submit(&e, &t).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(
        body["error"]
            .as_str()
            .unwrap()
            .contains("cannot submit: app/util.py is outside this persona's paths"),
        "{body}"
    );
    let (_, task) = call(&e.app, "GET", &format!("/v1/tasks/{}", id(&t)), None).await;
    assert_eq!(task["task"]["status"], "open", "{task}");

    let t = start_task(&e, "Notes").await;
    std::fs::create_dir_all(ws_dir(&e, &t).join("docs")).unwrap();
    write_file(&e, &t, "docs/note.md", "hello\n");
    decide(&e, &t).await;
    let (status, body) = submit(&e, &t).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(
        *guard.asked.lock().unwrap(),
        [
            vec!["app/util.py".to_owned()],
            vec!["docs/note.md".to_owned()]
        ]
    );
}

/// A sink that keeps what it is told, and refuses reports for one model.
#[derive(Default)]
struct Tally(std::sync::Mutex<Vec<scholion_daemon::UsageReport>>);

#[async_trait::async_trait]
impl scholion_daemon::UsageSink for Tally {
    async fn report(&self, report: scholion_daemon::UsageReport) -> Result<(), String> {
        if report.model.as_deref() == Some("pricey") {
            return Err("over budget".into());
        }
        self.0.lock().unwrap().push(report);
        Ok(())
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn usage_is_accepted_and_dropped_without_an_extension() {
    let e = env().await;
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/agents/claude/usage",
        Some(json!({"model": "opus", "input_tokens": 10, "output_tokens": 2})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["accepted"], true);
    // a malformed report is still refused
    let (status, _) = call(
        &e.app,
        "POST",
        "/v1/agents/claude/usage",
        Some(json!({"input_tokens": -1})),
    )
    .await;
    assert!(status.is_client_error(), "{status}");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_usage_sink_gets_the_reports_and_can_refuse_one() {
    let sink = std::sync::Arc::new(Tally::default());
    let s = sink.clone();
    let e = env_with(move |st| st.with_usage_sink(s)).await;
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/agents/claude/usage",
        Some(
            json!({"task_id": "t-1", "model": "opus", "input_tokens": 100,
                    "output_tokens": 20, "cache_read_tokens": 5, "cost_usd": 0.5}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    {
        let got = sink.0.lock().unwrap();
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].agent, "claude");
        assert_eq!(got[0].task_id.as_deref(), Some("t-1"));
        assert_eq!(got[0].model.as_deref(), Some("opus"));
        assert_eq!(
            (
                got[0].input_tokens,
                got[0].output_tokens,
                got[0].cache_read_tokens
            ),
            (100, 20, 5)
        );
        assert_eq!(got[0].cost_usd, Some(0.5));
    }
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/agents/claude/usage",
        Some(json!({"model": "pricey"})),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
    assert!(
        body["error"].as_str().unwrap().contains("over budget"),
        "{body}"
    );
    assert_eq!(sink.0.lock().unwrap().len(), 1);
}

#[tokio::test(flavor = "multi_thread")]
async fn an_extension_upserts_agent_profiles_that_routing_reads() {
    let state = std::sync::Arc::new(std::sync::Mutex::new(None::<AppState>));
    let keep = state.clone();
    let e = env_with(move |s| {
        *keep.lock().unwrap() = Some(s.clone());
        s
    })
    .await;
    let st = state.lock().unwrap().clone().unwrap();
    let mut p = scholion_core::routing::Profile {
        name: "ee-reviewer".into(),
        best: vec!["review".into()],
        can: vec![],
        cost_rank: 2,
        aliases: vec!["reviewer-bot".into()],
        max_open: Some(3),
    };
    st.put_profile(&p).await.unwrap();
    p.cost_rank = 4; // the same persona edited: an upsert, not a second row
    st.put_profile(&p).await.unwrap();

    let (status, body) = call(&e.app, "GET", "/v1/agents", None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let mine: Vec<&Value> = body["agents"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|a| a["name"] == "ee-reviewer")
        .collect();
    assert_eq!(mine.len(), 1, "{body}");
    assert_eq!(mine[0]["cost_rank"], 4);
    assert_eq!(mine[0]["best"], json!(["review"]));
    assert_eq!(mine[0]["max_open"], 3);

    p.name = "  ".into();
    let err = st.put_profile(&p).await.unwrap_err();
    assert!(matches!(err, scholion_core::AvError::Invalid(_)), "{err:?}");
}

/// What an extension keeps in the state.
struct Greeting(&'static str);

async fn greet(axum::extract::State(st): axum::extract::State<AppState>) -> axum::Json<Value> {
    let g = st.extension::<Greeting>().map(|g| g.0);
    axum::Json(json!({ "greeting": g }))
}

#[tokio::test(flavor = "multi_thread")]
async fn extra_routes_find_their_extension_and_sit_behind_the_token() {
    let extra = axum::Router::new().route("/v1/ee/greet", axum::routing::get(greet));
    let e = env_with_routes(
        |s| {
            s.with_extension(std::sync::Arc::new(Greeting("hello")))
                .with_token(Some("secret".into()))
        },
        extra,
    )
    .await;
    let (status, _) = call(&e.app, "GET", "/v1/ee/greet", None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    let (status, body) = call_with(
        &e.app,
        "GET",
        "/v1/ee/greet",
        None,
        &[("authorization", "Bearer secret")],
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["greeting"], "hello");
}

static BOARD: sqlx::migrate::Migrator = sqlx::migrate!("./tests/ext_migrations");

async fn board_notes(
    axum::extract::State(st): axum::extract::State<AppState>,
) -> axum::Json<Value> {
    let notes: Vec<String> = sqlx::query_scalar("SELECT note FROM agentvcs.board ORDER BY note")
        .fetch_all(st.pool())
        .await
        .unwrap();
    axum::Json(json!({ "notes": notes }))
}

#[tokio::test(flavor = "multi_thread")]
async fn an_extension_migrates_its_own_schema_and_reads_it_from_its_routes() {
    let state = std::sync::Arc::new(std::sync::Mutex::new(None::<AppState>));
    let keep = state.clone();
    let extra = axum::Router::new().route("/v1/ee/board", axum::routing::get(board_notes));
    let e = env_with_routes(
        move |s| {
            *keep.lock().unwrap() = Some(s.clone());
            s
        },
        extra,
    )
    .await;
    let st = state.lock().unwrap().clone().unwrap();
    st.migrate_extension("agentvcs", &BOARD).await.unwrap();
    st.migrate_extension("agentvcs", &BOARD).await.unwrap(); // every start-up runs it again

    let t = start_task(&e, "Board note").await;
    sqlx::query("INSERT INTO agentvcs.board (task, note) VALUES ($1, 'first')")
        .bind(uuid::Uuid::parse_str(id(&t).as_str()).unwrap())
        .execute(st.pool())
        .await
        .unwrap();
    let (status, body) = call(&e.app, "GET", "/v1/ee/board", None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["notes"], json!(["first"]));

    let err = st.migrate_extension("public", &BOARD).await.unwrap_err();
    assert!(matches!(err, scholion_core::AvError::Invalid(_)), "{err:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn subscribers_receive_lifecycle_events_in_process() {
    let state = std::sync::Arc::new(std::sync::Mutex::new(None::<AppState>));
    let keep = state.clone();
    let e = env_with(move |s| {
        *keep.lock().unwrap() = Some(s.clone());
        s
    })
    .await;
    let st = state.lock().unwrap().clone().unwrap();
    let mut rx = st.subscribe_events().await.unwrap();
    let mut rx2 = st.subscribe_events().await.unwrap();
    let t = start_task(&e, "Upper names").await;
    for rx in [&mut rx, &mut rx2] {
        let ev = tokio::time::timeout(std::time::Duration::from_secs(10), rx.recv())
            .await
            .expect("an event within 10 s")
            .unwrap();
        assert_eq!(ev.kind, "started");
        assert_eq!(ev.task_id.unwrap().0.to_string(), id(&t));
    }
}

async fn import_state(e: &Env, name: &str) -> Value {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);
    loop {
        let (status, body) = call(&e.app, "GET", &format!("/v1/repos/{name}/import"), None).await;
        assert_eq!(status, StatusCode::OK, "{body}");
        if body["state"] != "running" {
            return body;
        }
        assert!(std::time::Instant::now() < deadline, "import still running");
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn an_import_without_wait_answers_at_once_and_is_polled() {
    let e = env().await;
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/repos",
        Some(json!({"name": "second", "git_source": e.src_path, "wait": false})),
    )
    .await;
    assert_eq!(status, StatusCode::ACCEPTED, "{body}");
    assert_eq!(body["state"], "running");
    let done = import_state(&e, "second").await;
    assert_eq!(done["state"], "done", "{done}");
    assert!(done["main_change"].as_str().is_some_and(|c| !c.is_empty()));
    assert!(done["indexed_files"].as_u64().unwrap() > 0, "{done}");
    // the waiting form answers with the same job, done
    let (_, first) = call(&e.app, "GET", "/v1/repos/demo/import", None).await;
    assert_eq!(first["state"], "done", "{first}");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_failed_import_is_reported_and_the_name_is_free_again() {
    let e = env().await;
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/repos",
        Some(json!({"name": "broken", "git_source": "/nonexistent/repo", "wait": false})),
    )
    .await;
    assert_eq!(status, StatusCode::ACCEPTED, "{body}");
    let failed = import_state(&e, "broken").await;
    assert_eq!(failed["state"], "failed", "{failed}");
    assert!(failed["error"].as_str().is_some_and(|m| !m.is_empty()));
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/repos",
        Some(json!({"name": "broken", "git_source": e.src_path})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["state"], "done");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_client_that_disconnects_does_not_stop_its_import() {
    let e = env().await;
    let app = e.app.clone();
    let src = e.src_path.clone();
    let request = tokio::spawn(async move {
        call(
            &app,
            "POST",
            "/v1/repos",
            Some(json!({"name": "dropped", "git_source": src})),
        )
        .await
    });
    // disconnect as soon as the job exists
    loop {
        let (status, _) = call(&e.app, "GET", "/v1/repos/dropped/import", None).await;
        if status == StatusCode::OK {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    }
    request.abort();
    let done = import_state(&e, "dropped").await;
    assert_eq!(done["state"], "done", "{done}");
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/tasks",
        Some(json!({"repo": "dropped", "title": "t", "goal": "g", "agent": "a", "session": "s"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
}

// ---- Plan 7 Task 4: overlay writes ----

async fn overlay_paths(e: &Env, slug: &str) -> Vec<String> {
    sqlx::query_scalar(
        "SELECT path FROM index_files WHERE repo = 'demo' AND scope = $1 ORDER BY path",
    )
    .bind(slug)
    .fetch_all(e.store.pool())
    .await
    .unwrap()
}

async fn overlay_row(e: &Env, slug: &str) -> Option<String> {
    sqlx::query_scalar("SELECT main_at FROM scope_overlays WHERE repo = 'demo' AND scope = $1")
        .bind(slug)
        .fetch_optional(e.store.pool())
        .await
        .unwrap()
}

async fn task_symbol_count(e: &Env, slug: &str) -> i64 {
    sqlx::query_scalar("SELECT count(*) FROM symbols WHERE repo = 'demo' AND scope = $1")
        .bind(slug)
        .fetch_one(e.store.pool())
        .await
        .unwrap()
}

#[tokio::test(flavor = "multi_thread")]
async fn start_leaves_empty_overlay_and_no_cloned_symbols() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    let slug = t["task"]["slug"].as_str().unwrap();
    assert!(overlay_row(&e, slug).await.is_some(), "scope_overlays row");
    assert!(
        overlay_paths(&e, slug).await.is_empty(),
        "O empty when B = T"
    );
    assert_eq!(task_symbol_count(&e, slug).await, 0, "no clone_scope");
}

#[tokio::test(flavor = "multi_thread")]
async fn checkpoint_puts_edits_in_overlay_and_undo_clears_them() {
    let e = env().await;
    let t = start_task(&e, "Improve greeting").await;
    let slug = t["task"]["slug"].as_str().unwrap().to_owned();
    edit_greeter(&e, &t);
    let (status, cp) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/checkpoint", id(&t)),
        Some(json!({})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{cp}");
    assert_eq!(overlay_paths(&e, &slug).await, vec!["app/greeter.py"]);
    assert!(task_symbol_count(&e, &slug).await > 0);

    // restore to main's content: path leaves O
    let original = std::fs::read_to_string(e.src_path.join("app/greeter.py")).unwrap();
    std::fs::write(ws_dir(&e, &t).join("app/greeter.py"), original).unwrap();
    let (status, cp) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/checkpoint", id(&t)),
        Some(json!({})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{cp}");
    assert!(
        overlay_paths(&e, &slug).await.is_empty(),
        "undo leaves empty O: {:?}",
        overlay_paths(&e, &slug).await
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn idle_task_reconciles_when_main_merges_before_next_read() {
    let e = env().await;
    let idle = start_task(&e, "Idle reader").await;
    let idle_slug = idle["task"]["slug"].as_str().unwrap().to_owned();
    assert!(overlay_paths(&e, &idle_slug).await.is_empty());

    let editor = ready_to_merge(&e, "Upper names", UPPER).await;
    let (status, body) = post(&e, &editor, "merge", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");

    // before a read, idle O is still empty (reconcile is lazy)
    assert!(overlay_paths(&e, &idle_slug).await.is_empty());

    let (status, pack) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/context?budget=800&session=idle-1", id(&idle)),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{pack}");
    let paths = overlay_paths(&e, &idle_slug).await;
    assert_eq!(
        paths,
        vec!["app/util.py"],
        "reconcile masks main's merge with H's old content"
    );
    // pack still serves format_name from the overlay (old title()), not main's upper()
    let md = pack["markdown"].as_str().unwrap();
    assert!(md.contains("format_name") || md.contains("util"), "{md}");
}

/// Bundle (and start pack) must reconcile under `lock_task` before reading the task view —
/// same lazy path as context, not only GET …/context.
#[tokio::test(flavor = "multi_thread")]
async fn idle_task_reconciles_before_bundle_read() {
    let e = env().await;
    let idle = start_task(&e, "Idle bundler").await;
    let idle_slug = idle["task"]["slug"].as_str().unwrap().to_owned();

    let editor = ready_to_merge(&e, "Upper names", UPPER).await;
    let (status, body) = post(&e, &editor, "merge", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert!(overlay_paths(&e, &idle_slug).await.is_empty());

    let (status, bundle) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/bundle?kind=pr&budget=2000", id(&idle)),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{bundle}");
    assert_eq!(
        overlay_paths(&e, &idle_slug).await,
        vec!["app/util.py"],
        "bundle reconciles before in_degree / changed_symbols on the task view"
    );
}

/// Start's pack must re-reconcile under `lock_task` after overlay init, so a main merge in the
/// gap between init_task_overlay and pack build is not missed.
#[tokio::test(flavor = "multi_thread")]
async fn start_pack_reconciles_when_main_moves_after_overlay_init() {
    use scholion_daemon::StartPackPause;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};

    let pause = Arc::new(StartPackPause {
        ready: tokio::sync::Notify::new(),
        proceed: tokio::sync::Notify::new(),
        armed: AtomicBool::new(false),
    });
    let pause_for_env = pause.clone();
    let e = env_with(move |s| s.with_extension(pause_for_env)).await;

    // setup without pausing (armed = false)
    let editor = ready_to_merge(&e, "Upper names", UPPER).await;

    pause.armed.store(true, Ordering::SeqCst);
    let ready = pause.ready.notified();
    let start = tokio::spawn({
        let app = e.app.clone();
        async move {
            call(
                &app,
                "POST",
                "/v1/tasks",
                Some(json!({
                    "repo": "demo",
                    "title": "Started during merge",
                    "goal": "Make hello louder",
                    "agent": "claude",
                    "session": "start-race",
                })),
            )
            .await
        }
    });
    ready.await;
    let (status, body) = post(&e, &editor, "merge", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    pause.proceed.notify_waiters();

    let (status, started) = start.await.unwrap();
    assert_eq!(status, StatusCode::OK, "{started}");
    let slug = started["task"]["slug"].as_str().unwrap();
    assert_eq!(
        overlay_paths(&e, slug).await,
        vec!["app/util.py"],
        "start pack reconciles after main moved between overlay init and pack"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn sync_rebuilds_overlay_as_diff_against_trunk() {
    let e = env().await;
    let first = ready_to_merge(&e, "Upper names", UPPER).await;
    let second = start_task(&e, "Also touch greeter").await;
    let slug = second["task"]["slug"].as_str().unwrap().to_owned();
    edit_greeter(&e, &second);
    call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/checkpoint", id(&second)),
        Some(json!({})),
    )
    .await;
    assert_eq!(overlay_paths(&e, &slug).await, vec!["app/greeter.py"]);

    let (status, body) = post(&e, &first, "merge", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");

    // sync onto trunk: O is only the task's remaining edit (greeter), not util
    let (status, body) = post(&e, &second, "sync", json!({"agent": "claude"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(
        overlay_paths(&e, &slug).await,
        vec!["app/greeter.py"],
        "sync replaces O with changed(T, H')"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn merge_and_abandon_drop_overlay_rows() {
    let e = env().await;
    let t = ready_to_merge(&e, "Upper names", UPPER).await;
    let slug = t["task"]["slug"].as_str().unwrap().to_owned();
    assert!(!overlay_paths(&e, &slug).await.is_empty());
    assert!(overlay_row(&e, &slug).await.is_some());

    let (status, body) = post(&e, &t, "merge", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert!(overlay_paths(&e, &slug).await.is_empty());
    assert!(overlay_row(&e, &slug).await.is_none());
    assert_eq!(task_symbol_count(&e, &slug).await, 0);

    let abandoned = start_task(&e, "Abandon me").await;
    let a_slug = abandoned["task"]["slug"].as_str().unwrap().to_owned();
    edit_greeter(&e, &abandoned);
    call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/checkpoint", id(&abandoned)),
        Some(json!({})),
    )
    .await;
    assert!(!overlay_paths(&e, &a_slug).await.is_empty());
    let (status, body) = post(&e, &abandoned, "abandon", json!({"agent": "claude"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert!(overlay_paths(&e, &a_slug).await.is_empty());
    assert!(overlay_row(&e, &a_slug).await.is_none());
}

#[tokio::test(flavor = "multi_thread")]
async fn repos_lists_every_imported_repo_with_its_main_change() {
    let e = env().await;
    let (status, body) = call(&e.app, "GET", "/v1/repos", None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let repos = body.as_array().expect("an array");
    assert_eq!(repos.len(), 1, "{body}");
    assert_eq!(repos[0]["name"], "demo");
    let (_, import) = call(&e.app, "GET", "/v1/repos/demo/import", None).await;
    assert_eq!(repos[0]["main_change"], import["main_change"], "{body}");
    // the import recorded the repo in Postgres, with its source
    let rows = e.store.list_repos().await.unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].name, "demo");
    assert_eq!(
        rows[0].git_source.as_deref(),
        Some(e.src_path.canonicalize().unwrap().to_str().unwrap())
    );
    assert!(repos[0]["added_at"].is_string(), "{body}");
}

/// A repo imported before the `repos` table existed is only on disk; the daemon adds its row at
/// start-up (`sync_repos`), so it is listed too.
#[tokio::test(flavor = "multi_thread")]
async fn sync_repos_backfills_repos_that_are_only_on_disk() {
    let db = scholion_testutil::TestDb::new().await;
    let store = std::sync::Arc::new(scholion_store::PgStore::connect(&db.url).await.unwrap());
    let src = common::fixture_git_repo();
    let (store_dir, ws) = (
        tempfile::TempDir::new().unwrap(),
        tempfile::TempDir::new().unwrap(),
    );
    let vcs = std::sync::Arc::new(scholion_vcs::JjVcs::new(
        store_dir.path().to_path_buf(),
        ws.path().to_path_buf(),
    ));
    vcs.init_repo("old", src.path()).await.unwrap();
    let state = scholion_daemon::AppState::new(
        vcs,
        store.clone(),
        std::sync::Arc::new(scholion_index::PgIndex::new(store.pool().clone())),
        ws.path().to_path_buf(),
        ws.path().to_path_buf(),
    );
    assert_eq!(scholion_daemon::sync_repos(&state).await.unwrap(), 1);
    assert_eq!(
        scholion_daemon::sync_repos(&state).await.unwrap(),
        0,
        "only once"
    );
    let rows = store.list_repos().await.unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].name, "old");
}
