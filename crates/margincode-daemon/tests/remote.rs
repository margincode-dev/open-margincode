//! Plan 14, remote workspaces by checkpoint-by-patch: `POST /v1/tasks/{id}/checkpoint?base=` with a
//! git-format patch body, and `GET /v1/tasks/{id}/patch?from=` to start and resync a checkout.

use std::collections::BTreeMap;

use axum::Router;
use axum::body::Body;
use axum::http::{HeaderMap, Request, StatusCode};
use margincode_vcs::patch::{self, Blob, FILE, FileDiff};
use serde_json::{Value, json};
use tower::ServiceExt as _;

use crate::common::*;

const GREETER: &str = "from util import format_name\n\nclass Greeter:\n    def hello(self, name):\n        return format_name(name)\n";
const LOUDER: &str = "from util import format_name\n\nclass Greeter:\n    def hello(self, name):\n        return format_name(name).upper()\n";

/// A request with a raw body; returns status, headers and the body bytes.
async fn raw(
    app: &Router,
    method: &str,
    uri: &str,
    body: Vec<u8>,
) -> (StatusCode, HeaderMap, Vec<u8>) {
    let req = Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "text/x-diff")
        .body(Body::from(body))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    let (status, headers) = (resp.status(), resp.headers().clone());
    let bytes = axum::body::to_bytes(resp.into_body(), 256 * 1024 * 1024)
        .await
        .unwrap();
    (status, headers, bytes.to_vec())
}

fn json_of(bytes: &[u8]) -> Value {
    serde_json::from_slice(bytes).unwrap_or_else(|_| json!(String::from_utf8_lossy(bytes)))
}

fn text(s: &str) -> Option<Blob> {
    Some(Blob {
        mode: FILE,
        bytes: s.as_bytes().to_vec(),
    })
}

fn greeter_patch(before: &str, after: &str) -> Vec<u8> {
    patch::render(&[FileDiff {
        path: "app/greeter.py".into(),
        before: text(before),
        after: text(after),
    }])
}

async fn head_of(e: &Env, t: &Value) -> String {
    let (status, body) = call(&e.app, "GET", &format!("/v1/tasks/{}", id(t)), None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body["task"]["head_change"].as_str().unwrap().to_owned()
}

async fn post_patch(e: &Env, t: &Value, base: &str, p: Vec<u8>) -> (StatusCode, HeaderMap, Value) {
    let (status, headers, body) = raw(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/checkpoint?base={base}", id(t)),
        p,
    )
    .await;
    (status, headers, json_of(&body))
}

#[tokio::test(flavor = "multi_thread")]
async fn a_patch_checkpoint_lands_like_a_local_one() {
    let e = env().await;
    let t = start_task(&e, "Remote greeting").await;
    let base = head_of(&e, &t).await;
    let (status, _, body) = post_patch(&e, &t, &base, greeter_patch(GREETER, LOUDER)).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let head = body["head_change"].as_str().unwrap();
    assert_ne!(head, base);
    assert_eq!(body["changed"], json!(["app/greeter.py"]));
    assert_eq!(body["applied"], json!(["app/greeter.py"]));
    assert!(body["warnings"].is_array(), "{body}");
    let tree = body["tree"].as_str().unwrap();
    assert!(
        tree.len() == 40 && tree.bytes().all(|b| b.is_ascii_hexdigit()),
        "{tree}"
    );
    // the daemon's own workspace holds the file, and the task's head is the new checkpoint
    assert_eq!(
        std::fs::read_to_string(ws_dir(&e, &t).join("app/greeter.py")).unwrap(),
        LOUDER
    );
    assert_eq!(head_of(&e, &t).await, head);
    let slug = t["task"]["slug"].as_str().unwrap();
    let (_, file) = call(
        &e.app,
        "GET",
        &format!("/v1/repos/demo/files/app/greeter.py?at=task:{slug}"),
        None,
    )
    .await;
    assert_eq!(file["content"], LOUDER, "{file}");
    // a local checkpoint after it changes nothing: the snapshot already has it
    let (_, local) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/checkpoint", id(&t)),
        Some(json!({})),
    )
    .await;
    assert_eq!(local["head_change"], head, "{local}");
    // the checkpoint is an event like any other
    let (_, events) = call(&e.app, "GET", "/v1/events?repo=demo&since=0", None).await;
    let kinds: Vec<&str> = events["events"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|ev| ev["kind"].as_str())
        .collect();
    assert!(kinds.contains(&"checkpoint"), "{kinds:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn an_empty_patch_on_a_fresh_task_is_accepted() {
    // the first base a client sees is the task's base on trunk; the workspace's commit is a child
    // of it with the same files, so that base is not stale
    let e = env().await;
    let t = start_task(&e, "Nothing yet").await;
    let base = head_of(&e, &t).await;
    let (status, _, body) = post_patch(&e, &t, &base, vec![]).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["changed"], json!([]));
}

#[tokio::test(flavor = "multi_thread")]
async fn a_stale_base_is_409_with_the_current_head() {
    let e = env().await;
    let t = start_task(&e, "Stale base").await;
    let base = head_of(&e, &t).await;
    // someone on the daemon's host edits and checkpoints
    edit_greeter(&e, &t);
    let (_, local) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/checkpoint", id(&t)),
        Some(json!({})),
    )
    .await;
    let h1 = local["head_change"].as_str().unwrap().to_owned();
    let (status, headers, body) = post_patch(&e, &t, &base, greeter_patch(GREETER, LOUDER)).await;
    assert_eq!(status, StatusCode::CONFLICT, "{body}");
    assert_eq!(body["head_change"], h1);
    assert_eq!(body["base"], base);
    assert_eq!(headers["x-av-head"], h1.as_str());
    assert!(
        body["error"].as_str().unwrap().contains("/patch?from="),
        "{body}"
    );
    assert_ne!(
        std::fs::read_to_string(ws_dir(&e, &t).join("app/greeter.py")).unwrap(),
        LOUDER,
        "nothing was applied"
    );

    // an edit on disk that no checkpoint has seen yet makes the base stale too
    std::fs::write(
        ws_dir(&e, &t).join("app/util.py"),
        "def format_name(n):\n    return n\n",
    )
    .unwrap();
    let (status, _, body) = post_patch(&e, &t, &h1, vec![]).await;
    assert_eq!(status, StatusCode::CONFLICT, "{body}");
    let h2 = body["head_change"].as_str().unwrap();
    assert_ne!(h2, h1, "the drift was snapshotted first");
    assert_eq!(head_of(&e, &t).await, h2);
}

#[tokio::test(flavor = "multi_thread")]
async fn the_patch_route_starts_and_resyncs_a_checkout() {
    let e = env().await;
    let t = start_task(&e, "Resync").await;
    let base = head_of(&e, &t).await;
    // from nothing: every file of the task's head
    let (status, headers, full) = raw(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/patch", id(&t)),
        vec![],
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(headers["content-type"], "text/x-diff");
    assert_eq!(headers["x-av-head"], base.as_str());
    let files: BTreeMap<String, Blob> = patch::apply(&full, |_| Ok(None))
        .unwrap()
        .into_iter()
        .map(|a| (a.path, a.after.unwrap()))
        .collect();
    let paths: Vec<&str> = files.keys().map(String::as_str).collect();
    assert_eq!(paths, vec![".gitignore", "app/greeter.py", "app/util.py"]);
    assert_eq!(files["app/greeter.py"].bytes, GREETER.as_bytes());
    let tree = headers["x-av-tree"].to_str().unwrap().to_owned();
    assert_eq!(tree.len(), 40);

    // after a change on the daemon, from the old base: only what changed
    edit_greeter(&e, &t);
    let (_, local) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/checkpoint", id(&t)),
        Some(json!({})),
    )
    .await;
    let (status, headers, step) = raw(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/patch?from={base}", id(&t)),
        vec![],
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(headers["x-av-head"], local["head_change"].as_str().unwrap());
    assert_ne!(headers["x-av-tree"].to_str().unwrap(), tree);
    let applied = patch::apply(&step, |p| Ok(files.get(p).cloned())).unwrap();
    assert_eq!(applied.len(), 1);
    assert_eq!(applied[0].path, "app/greeter.py");

    // a patch from the head to itself is empty
    let head = local["head_change"].as_str().unwrap();
    let (_, _, none) = raw(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/patch?from={head}", id(&t)),
        vec![],
    )
    .await;
    assert!(none.is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn a_bad_patch_is_400_and_writes_nothing() {
    let e = env().await;
    let t = start_task(&e, "Bad patch").await;
    let base = head_of(&e, &t).await;
    let escape = b"diff --git a/.jj/repo/x b/.jj/repo/x\nnew file mode 100644\n--- /dev/null\n+++ b/.jj/repo/x\n@@ -0,0 +1 @@\n+x\n".to_vec();
    let (status, _, body) = post_patch(&e, &t, &base, escape).await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
    // made against other content
    let (status, _, body) = post_patch(&e, &t, &base, greeter_patch(LOUDER, GREETER)).await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
    assert_eq!(
        std::fs::read_to_string(ws_dir(&e, &t).join("app/greeter.py")).unwrap(),
        GREETER
    );
    // an unknown base
    let (status, _, body) = post_patch(&e, &t, &"ab".repeat(20), vec![]).await;
    assert_eq!(status, StatusCode::NOT_FOUND, "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_patch_over_the_limit_is_413() {
    let e = env().await;
    let t = start_task(&e, "Too big").await;
    let base = head_of(&e, &t).await;
    let big = vec![b'x'; margincode_daemon::MAX_PATCH_BYTES + 1];
    let (status, _, _) = post_patch(&e, &t, &base, big).await;
    assert_eq!(status, StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test(flavor = "multi_thread")]
async fn a_patch_from_a_lost_claim_is_fenced() {
    let e = env().await;
    let t = start_task(&e, "Fenced").await;
    let base = head_of(&e, &t).await;
    let req = Request::builder()
        .method("POST")
        .uri(format!("/v1/tasks/{}/checkpoint?base={base}", id(&t)))
        .header("x-av-generation", "99")
        .body(Body::from(greeter_patch(GREETER, LOUDER)))
        .unwrap();
    let resp = e.app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        std::fs::read_to_string(ws_dir(&e, &t).join("app/greeter.py")).unwrap(),
        GREETER
    );
}

/// A remote checkout's checkpoint gets the extension's additions too (`with_checkpoint_hook`),
/// beside `applied` and `tree`.
#[tokio::test(flavor = "multi_thread")]
async fn a_patch_checkpoint_carries_the_hooks_additions() {
    struct Hook;
    #[async_trait::async_trait]
    impl margincode_daemon::CheckpointHook for Hook {
        async fn after_checkpoint(
            &self,
            _: &margincode_core::Task,
            _: &margincode_core::ChangeId,
            changed: &[String],
        ) -> Result<Value, String> {
            Ok(json!({"notices": [format!("seen {}", changed.join(","))], "applied": "mine"}))
        }
    }
    let e = env_with(|st| st.with_checkpoint_hook(std::sync::Arc::new(Hook))).await;
    let t = start_task(&e, "Remote greeting").await;
    let base = head_of(&e, &t).await;
    let (status, _, body) = post_patch(&e, &t, &base, greeter_patch(GREETER, LOUDER)).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["notices"], json!(["seen app/greeter.py"]));
    // the daemon's own field wins over the hook's
    assert_eq!(body["applied"], json!(["app/greeter.py"]));
}
