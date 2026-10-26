use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use scholion_core::AvError;
use scholion_semantic::{Embedder, OllamaEmbedder};
use serde_json::{Value, json};
use tokio::net::TcpListener;

#[derive(Clone)]
struct Fake {
    calls: Arc<AtomicUsize>,
    bodies: Arc<Mutex<Vec<Value>>>,
    /// Vector length per call: `dims[n]` for the n-th call (last entry repeats).
    dims: Arc<Vec<usize>>,
    /// Return one vector fewer than the number of inputs.
    short: bool,
}

async fn embed_route(State(f): State<Fake>, Json(body): Json<Value>) -> Json<Value> {
    let n = f.calls.fetch_add(1, Ordering::SeqCst);
    let inputs = body["input"].as_array().map(Vec::len).unwrap_or(0);
    f.bodies.lock().unwrap().push(body);
    let dim = *f.dims.get(n).unwrap_or_else(|| f.dims.last().unwrap());
    let count = if f.short {
        inputs.saturating_sub(1)
    } else {
        inputs
    };
    let embeddings: Vec<Vec<f32>> = (0..count).map(|i| vec![i as f32; dim]).collect();
    Json(json!({ "model": "qwen3-embedding:8b", "embeddings": embeddings }))
}

async fn spawn_fake(dims: Vec<usize>, short: bool) -> (String, Fake) {
    let fake = Fake {
        calls: Arc::new(AtomicUsize::new(0)),
        bodies: Arc::new(Mutex::new(vec![])),
        dims: Arc::new(dims),
        short,
    };
    let app = Router::new()
        .route("/api/embed", post(embed_route))
        .with_state(fake.clone());
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    (format!("http://{addr}"), fake)
}

#[tokio::test]
async fn posts_model_input_and_truncate_and_returns_embeddings() {
    let (url, fake) = spawn_fake(vec![3], false).await;
    let e = OllamaEmbedder::new(&url, "qwen3-embedding:8b");
    let out = e.embed(&["a".into(), "b".into()]).await.unwrap();
    assert_eq!(out, vec![vec![0.0; 3], vec![1.0; 3]]);
    let body = fake.bodies.lock().unwrap()[0].clone();
    assert_eq!(
        body,
        json!({ "model": "qwen3-embedding:8b", "input": ["a", "b"], "truncate": true, "keep_alive": "30m" })
    );
    assert_eq!(
        e.dimension().await.unwrap(),
        3,
        "dimension cached from the first embed"
    );
    assert_eq!(fake.calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn empty_input_makes_no_request() {
    let (url, fake) = spawn_fake(vec![3], false).await;
    let e = OllamaEmbedder::new(&url, "m");
    assert!(e.embed(&[]).await.unwrap().is_empty());
    assert_eq!(fake.calls.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn dimension_probes_once_then_rejects_a_changed_length() {
    let (url, fake) = spawn_fake(vec![4, 3], false).await;
    let e = OllamaEmbedder::new(&url, "m");
    assert_eq!(e.dimension().await.unwrap(), 4);
    assert_eq!(e.dimension().await.unwrap(), 4);
    assert_eq!(fake.calls.load(Ordering::SeqCst), 1);
    assert!(matches!(
        e.embed(&["x".into()]).await,
        Err(AvError::Semantic(_))
    ));
}

#[tokio::test]
async fn count_mismatch_is_an_error() {
    let (url, _fake) = spawn_fake(vec![3], true).await;
    let e = OllamaEmbedder::new(&url, "m");
    assert!(matches!(
        e.embed(&["a".into(), "b".into()]).await,
        Err(AvError::Semantic(_))
    ));
}

#[tokio::test]
async fn unreachable_server_is_a_semantic_error() {
    let e = OllamaEmbedder::new("http://127.0.0.1:1", "m");
    assert!(matches!(e.dimension().await, Err(AvError::Semantic(_))));
    assert!(matches!(
        e.embed(&["a".into()]).await,
        Err(AvError::Semantic(_))
    ));
}
