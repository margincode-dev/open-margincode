use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use scholion_core::{AvError, Result};
use serde::Serialize;

const EMBED_TIMEOUT: Duration = Duration::from_secs(10);

/// Embeds text. `dimension()` is the vector length every successful `embed` must return; it may
/// have to ask the embedder, so it is async and fails with `AvError::Semantic` when that is down.
#[async_trait]
pub trait Embedder: Send + Sync {
    async fn dimension(&self) -> Result<usize>;
    async fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>>;
}

/// Lets shared trait objects (`Arc<dyn Embedder>`) be used wherever an `Embedder` is expected.
#[async_trait]
impl<T: Embedder + ?Sized> Embedder for Arc<T> {
    async fn dimension(&self) -> Result<usize> {
        (**self).dimension().await
    }
    async fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        (**self).embed(texts).await
    }
}

/// HTTP client for a text-embeddings-inference `/embed` endpoint.
///
/// `new` does not connect. `dimension` posts `["ping"]` on first use and caches that vector's length.
#[derive(Clone)]
pub struct TeiEmbedder {
    inner: Arc<TeiInner>,
}

struct TeiInner {
    base: String,
    /// `None` sends no `Authorization` header. The value is never logged.
    api_key: Option<String>,
    client: reqwest::Client,
    dim: Mutex<Option<usize>>,
}

#[derive(Serialize)]
struct EmbedRequest<'a> {
    inputs: &'a [String],
    truncate: bool,
}

impl TeiEmbedder {
    /// Does not connect. No `Authorization` header.
    pub fn new(base_url: impl Into<String>) -> Self {
        Self::with_api_key(base_url, None)
    }

    /// `api_key` `None` or blank: no `Authorization` header. Otherwise `Bearer <key>`
    /// on `/embed` and `/health`. Does not connect.
    pub fn with_api_key(base_url: impl Into<String>, api_key: Option<String>) -> Self {
        let base = base_url.into().trim_end_matches('/').to_string();
        let api_key = api_key.and_then(|key| {
            let key = key.trim();
            if key.is_empty() {
                None
            } else {
                Some(key.to_string())
            }
        });
        Self {
            inner: Arc::new(TeiInner {
                base,
                api_key,
                client: http_client().expect("reqwest client"),
                dim: Mutex::new(None),
            }),
        }
    }

    /// `GET {base}/health`. Non-2xx is [`AvError::Semantic`].
    pub async fn health(&self) -> Result<()> {
        let response = authorized(
            self.inner.client.get(format!("{}/health", self.inner.base)),
            self.inner.api_key.as_deref(),
        )
        .send()
        .await
        .map_err(|err| AvError::Semantic(format!("embedder health request failed: {err}")))?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            let body: String = body.chars().take(500).collect();
            return Err(AvError::Semantic(format!(
                "embedder health HTTP {status}: {body}"
            )));
        }
        Ok(())
    }
}

#[async_trait]
impl Embedder for TeiEmbedder {
    async fn dimension(&self) -> Result<usize> {
        if let Some(dim) = cached_dim(&self.inner.dim) {
            return Ok(dim);
        }
        let texts = vec!["ping".to_string()];
        let vectors = post_embed(
            &self.inner.client,
            &self.inner.base,
            self.inner.api_key.as_deref(),
            &texts,
        )
        .await?;
        validate_dim(&self.inner.dim, texts.len(), &vectors)?;
        cached_dim(&self.inner.dim)
            .ok_or_else(|| AvError::Semantic("embedder returned no vector".into()))
    }

    async fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }
        let vectors = post_embed(
            &self.inner.client,
            &self.inner.base,
            self.inner.api_key.as_deref(),
            texts,
        )
        .await?;
        validate_dim(&self.inner.dim, texts.len(), &vectors)?;
        Ok(vectors)
    }
}

/// Test double. `embed` of `n` texts returns `n` vectors of `dimension()`, derived from the text,
/// unless `fail` is set (then [`AvError::Semantic`]) or `wrong_dim` is set (then length
/// `dimension() - 1`), or any text contains `poison` (then the whole call fails, as a server
/// rejecting one input would). `calls` counts non-empty `embed` invocations and `texts` the texts
/// they carried. Clones share the flags and the counters.
#[derive(Clone, Debug)]
pub struct ScriptedEmbedder {
    pub dimension: usize,
    pub fail: Arc<AtomicBool>,
    pub wrong_dim: Arc<AtomicBool>,
    pub calls: Arc<AtomicUsize>,
    pub texts: Arc<AtomicUsize>,
    pub poison: Arc<std::sync::Mutex<Option<String>>>,
}

impl ScriptedEmbedder {
    pub fn new(dimension: usize) -> Self {
        Self {
            dimension,
            fail: Arc::new(AtomicBool::new(false)),
            wrong_dim: Arc::new(AtomicBool::new(false)),
            calls: Arc::new(AtomicUsize::new(0)),
            texts: Arc::new(AtomicUsize::new(0)),
            poison: Arc::default(),
        }
    }
}

#[async_trait]
impl Embedder for ScriptedEmbedder {
    async fn dimension(&self) -> Result<usize> {
        Ok(self.dimension)
    }

    async fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }
        self.calls.fetch_add(1, Ordering::SeqCst);
        self.texts.fetch_add(texts.len(), Ordering::SeqCst);
        if self.fail.load(Ordering::SeqCst) {
            return Err(AvError::Semantic("scripted embedder failed".into()));
        }
        let poison = self
            .poison
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone();
        if let Some(p) = poison.filter(|p| texts.iter().any(|t| t.contains(p.as_str()))) {
            return Err(AvError::Semantic(format!(
                "scripted embedder rejects {p:?}"
            )));
        }
        let len = if self.wrong_dim.load(Ordering::SeqCst) {
            self.dimension.saturating_sub(1)
        } else {
            self.dimension
        };
        Ok(texts
            .iter()
            .map(|text| scripted_vector(text, len))
            .collect())
    }
}

fn scripted_vector(text: &str, len: usize) -> Vec<f32> {
    let mut acc: u64 = 0x9e37_79b9_7f4a_7c15;
    for byte in text.as_bytes() {
        acc = acc
            .wrapping_mul(0x100_0000_01b3)
            .wrapping_add(u64::from(*byte));
    }
    (0..len)
        .map(|i| {
            let mixed = acc.wrapping_add(u64::try_from(i).unwrap_or(0).wrapping_mul(0x85eb_ca6b));
            let bucket = u16::try_from(mixed % 2000).unwrap_or(0);
            if bucket == 1000 {
                0.125
            } else {
                f32::from(bucket) / 1000.0 - 1.0
            }
        })
        .collect()
}

pub(crate) fn http_client() -> Result<reqwest::Client> {
    reqwest::Client::builder()
        .timeout(EMBED_TIMEOUT)
        .build()
        .map_err(|err| AvError::Semantic(format!("embedder client: {err}")))
}

pub(crate) fn cached_dim(dim: &Mutex<Option<usize>>) -> Option<usize> {
    *dim.lock().expect("embedder dimension lock")
}

fn authorized(request: reqwest::RequestBuilder, api_key: Option<&str>) -> reqwest::RequestBuilder {
    match api_key {
        Some(key) => request.header(reqwest::header::AUTHORIZATION, format!("Bearer {key}")),
        None => request,
    }
}

async fn post_embed(
    client: &reqwest::Client,
    base: &str,
    api_key: Option<&str>,
    texts: &[String],
) -> Result<Vec<Vec<f32>>> {
    let response = authorized(
        client.post(format!("{base}/embed")).json(&EmbedRequest {
            inputs: texts,
            truncate: true,
        }),
        api_key,
    )
    .send()
    .await
    .map_err(|err| AvError::Semantic(format!("embedder request failed: {err}")))?;
    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        let body: String = body.chars().take(500).collect();
        return Err(AvError::Semantic(format!("embedder HTTP {status}: {body}")));
    }
    response.json().await.map_err(|err| {
        AvError::Semantic(format!(
            "embedder response was not a JSON array of vectors: {err}"
        ))
    })
}

pub(crate) fn validate_dim(
    dim: &Mutex<Option<usize>>,
    input_count: usize,
    vectors: &[Vec<f32>],
) -> Result<()> {
    if vectors.len() != input_count {
        return Err(AvError::Semantic(format!(
            "embedder returned {} vectors for {input_count} inputs",
            vectors.len()
        )));
    }
    let Some(observed) = vectors.first().map(Vec::len) else {
        return Ok(());
    };
    if vectors.iter().any(|vector| vector.len() != observed) {
        return Err(AvError::Semantic(format!(
            "embedder returned vectors of unequal length; first is {observed}"
        )));
    }
    let mut guard = dim
        .lock()
        .map_err(|_| AvError::Semantic("embedder dimension lock poisoned".into()))?;
    if let Some(expected) = *guard {
        if observed != expected {
            return Err(AvError::Semantic(format!(
                "embedder returned dimension {observed}, probe dimension is {expected}"
            )));
        }
    } else {
        *guard = Some(observed);
    }
    Ok(())
}
