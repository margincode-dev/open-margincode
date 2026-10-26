use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use axum::Json;
use axum::Router;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::routing::{get, post};
use scholion_core::AvError;
use scholion_semantic::{Embedder, ScriptedEmbedder, TeiEmbedder};
use serde_json::{Value, json};
use tokio::net::TcpListener;

/// The fake lives on its own runtime. `TeiEmbedder::dimension` blocks the caller, so the
/// listener cannot share a current-thread test runtime.

#[derive(Clone)]
struct Fake {
    calls: Arc<AtomicUsize>,
    bodies: Arc<Mutex<Vec<Value>>>,
    authorizations: Arc<Mutex<Vec<Option<String>>>>,
    kind: FakeKind,
}

#[derive(Clone, Copy)]
enum FakeKind {
    /// One vector per input. Each vector is `[index, 0.5, 1.5]`.
    Mirror,
    /// First request returns `first_dim`; later requests return `later_dim`.
    DimThen { first_dim: usize, later_dim: usize },
    /// Always one vector, however many inputs were posted.
    OneVector,
}

fn spawn_fake(kind: FakeKind) -> (String, Fake) {
    let fake = Fake {
        calls: Arc::new(AtomicUsize::new(0)),
        bodies: Arc::new(Mutex::new(Vec::new())),
        authorizations: Arc::new(Mutex::new(Vec::new())),
        kind,
    };
    let state = fake.clone();
    let (tx, rx) = std::sync::mpsc::sync_channel(1);
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async move {
            let app = Router::new()
                .route("/embed", post(embed_route))
                .route("/health", get(health_route))
                .with_state(state);
            let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
            tx.send(listener.local_addr().unwrap()).unwrap();
            axum::serve(listener, app).await.unwrap();
        });
    });
    let addr = rx.recv().unwrap();
    (format!("http://{addr}"), fake)
}

async fn health_route(State(fake): State<Fake>, headers: HeaderMap) -> &'static str {
    record_authorization(&fake, &headers);
    "ok"
}

fn record_authorization(fake: &Fake, headers: &HeaderMap) {
    let header = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .map(str::to_string);
    fake.authorizations.lock().unwrap().push(header);
}

async fn embed_route(
    State(fake): State<Fake>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Json<Vec<Vec<f32>>> {
    record_authorization(&fake, &headers);
    let call = fake.calls.fetch_add(1, Ordering::SeqCst);
    fake.bodies.lock().unwrap().push(body.clone());
    let n = body
        .get("inputs")
        .and_then(Value::as_array)
        .map(Vec::len)
        .unwrap_or(0);
    let vectors = match fake.kind {
        FakeKind::Mirror => (0..n).map(|i| vec![i as f32, 0.5, 1.5]).collect(),
        FakeKind::DimThen {
            first_dim,
            later_dim,
        } => {
            let dim = if call == 0 { first_dim } else { later_dim };
            vec![vec![1.0; dim]; n]
        }
        FakeKind::OneVector => vec![vec![0.25, 0.5]],
    };
    Json(vectors)
}

#[tokio::test]
async fn posts_inputs_and_truncate_true() {
    let (url, fake) = spawn_fake(FakeKind::Mirror);
    let embedder = TeiEmbedder::new(url);
    let inputs = vec!["alpha".to_string(), "beta".to_string()];
    let got = embedder.embed(&inputs).await.unwrap();
    assert_eq!(got, vec![vec![0.0, 0.5, 1.5], vec![1.0, 0.5, 1.5]]);
    let bodies = fake.bodies.lock().unwrap();
    assert_eq!(bodies.len(), 1);
    assert_eq!(bodies[0]["truncate"], json!(true));
    assert_eq!(bodies[0]["inputs"], json!(inputs));
}

#[tokio::test]
async fn empty_inputs_do_not_call_http() {
    let (url, fake) = spawn_fake(FakeKind::Mirror);
    let embedder = TeiEmbedder::new(url);
    let got = embedder.embed(&[]).await.unwrap();
    assert!(got.is_empty());
    assert_eq!(fake.calls.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn embed_rejects_vector_whose_length_is_not_the_probe_dim() {
    let (url, fake) = spawn_fake(FakeKind::DimThen {
        first_dim: 4,
        later_dim: 3,
    });
    let embedder = TeiEmbedder::new(url);
    assert_eq!(embedder.dimension().await.unwrap(), 4);
    let bodies = fake.bodies.lock().unwrap().clone();
    assert_eq!(bodies.len(), 1, "dimension() probes once");
    assert_eq!(bodies[0]["truncate"], json!(true));
    assert_eq!(bodies[0]["inputs"], json!(["ping"]));
    drop(bodies);
    let err = embedder
        .embed(&["later".to_string()])
        .await
        .expect_err("3-d vector must not match the 4-d probe");
    assert!(matches!(err, AvError::Semantic(_)), "{err}");
    assert_eq!(fake.calls.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn rejects_a_count_mismatch() {
    let (url, _fake) = spawn_fake(FakeKind::OneVector);
    let embedder = TeiEmbedder::new(url);
    let err = embedder
        .embed(&["one".to_string(), "two".to_string()])
        .await
        .expect_err("two inputs and one vector");
    assert!(matches!(err, AvError::Semantic(_)), "{err}");
}

#[tokio::test]
async fn scripted_embedder_can_fail_and_lie_about_dim() {
    let embedder = ScriptedEmbedder::new(4);
    let ok = embedder
        .embed(&["alpha".to_string(), "beta".to_string()])
        .await
        .unwrap();
    assert_eq!(ok.len(), 2);
    assert!(ok.iter().all(|v| v.len() == 4));
    assert_ne!(ok[0], ok[1]);
    assert_eq!(embedder.calls.load(Ordering::SeqCst), 1);
    assert_eq!(embedder.dimension().await.unwrap(), 4);

    embedder.fail.store(true, Ordering::SeqCst);
    let err = embedder
        .embed(&["alpha".to_string()])
        .await
        .expect_err("fail is set");
    assert!(matches!(err, AvError::Semantic(_)), "{err}");

    embedder.fail.store(false, Ordering::SeqCst);
    embedder.wrong_dim.store(true, Ordering::SeqCst);
    let lied = embedder.embed(&["alpha".to_string()]).await.unwrap();
    assert_eq!(lied.len(), 1);
    assert_eq!(lied[0].len(), 3);
    assert_eq!(embedder.dimension().await.unwrap(), 4);

    let empty = embedder.embed(&[]).await.unwrap();
    assert!(empty.is_empty());
    assert_eq!(
        embedder.calls.load(Ordering::SeqCst),
        3,
        "an empty slice is not an embed call"
    );
}

#[tokio::test]
async fn sends_authorization_bearer_when_api_key_is_set() {
    let (url, fake) = spawn_fake(FakeKind::Mirror);
    let embedder = TeiEmbedder::with_api_key(url, Some("sk-test".into()));
    embedder.embed(&["alpha".to_string()]).await.unwrap();
    embedder.health().await.unwrap();
    let headers = fake.authorizations.lock().unwrap().clone();
    assert_eq!(
        headers,
        vec![
            Some("Bearer sk-test".to_string()),
            Some("Bearer sk-test".to_string()),
        ]
    );
}

#[tokio::test]
async fn omits_authorization_when_api_key_is_blank() {
    let (url, fake) = spawn_fake(FakeKind::Mirror);
    let plain = TeiEmbedder::new(url.clone());
    plain.embed(&["alpha".to_string()]).await.unwrap();
    plain.health().await.unwrap();
    let blank = TeiEmbedder::with_api_key(url, Some("  ".into()));
    blank.embed(&["beta".to_string()]).await.unwrap();
    blank.health().await.unwrap();
    let headers = fake.authorizations.lock().unwrap().clone();
    assert_eq!(headers, vec![None, None, None, None]);
}

#[tokio::test]
async fn dimension_is_a_semantic_error_when_the_embedder_is_down() {
    // Nothing listens on port 1: the probe must fail as an error, not a panic.
    let embedder = TeiEmbedder::new("http://127.0.0.1:1");
    match embedder.dimension().await {
        Err(scholion_core::AvError::Semantic(_)) => {}
        other => panic!("expected AvError::Semantic, got {other:?}"),
    }
}
