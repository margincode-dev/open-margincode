//! Ollama `/api/embed` client (the default embedder when the host has `qwen3-embedding`).

use std::sync::Mutex;

use async_trait::async_trait;
use scholion_core::{AvError, Result};
use serde::{Deserialize, Serialize};

use crate::embed::{Embedder, cached_dim, http_client, validate_dim};

/// How long Ollama keeps the embedding model loaded after a request.
const KEEP_ALIVE: &str = "30m";

/// `POST {base}/api/embed` with `{"model", "input", "truncate": true}`. No API key.
/// `new` does not connect; the dimension is learned from the first successful call.
pub struct OllamaEmbedder {
    base: String,
    model: String,
    client: reqwest::Client,
    dim: Mutex<Option<usize>>,
}

#[derive(Serialize)]
struct Request<'a> {
    model: &'a str,
    input: &'a [String],
    truncate: bool,
    /// Ollama unloads an idle model after ~5 min; a cold load blows the pack's 150 ms search
    /// budget, so ask it to stay resident.
    keep_alive: &'a str,
}

#[derive(Deserialize)]
struct Response {
    embeddings: Vec<Vec<f32>>,
}

impl OllamaEmbedder {
    pub fn new(base_url: &str, model: &str) -> Self {
        Self {
            base: base_url.trim_end_matches('/').to_string(),
            model: model.to_string(),
            client: http_client().expect("reqwest client"),
            dim: Mutex::new(None),
        }
    }

    async fn post(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        let response = self
            .client
            .post(format!("{}/api/embed", self.base))
            .json(&Request {
                model: &self.model,
                input: texts,
                truncate: true,
                keep_alive: KEEP_ALIVE,
            })
            .send()
            .await
            .map_err(|e| AvError::Semantic(format!("ollama request failed: {e}")))?;
        let status = response.status();
        if !status.is_success() {
            let body: String = response
                .text()
                .await
                .unwrap_or_default()
                .chars()
                .take(500)
                .collect();
            return Err(AvError::Semantic(format!("ollama HTTP {status}: {body}")));
        }
        let parsed: Response = response
            .json()
            .await
            .map_err(|e| AvError::Semantic(format!("ollama response had no embeddings: {e}")))?;
        validate_dim(&self.dim, texts.len(), &parsed.embeddings)?;
        Ok(parsed.embeddings)
    }
}

#[async_trait]
impl Embedder for OllamaEmbedder {
    async fn dimension(&self) -> Result<usize> {
        if let Some(dim) = cached_dim(&self.dim) {
            return Ok(dim);
        }
        self.post(&["ping".to_string()]).await?;
        cached_dim(&self.dim).ok_or_else(|| AvError::Semantic("ollama returned no vector".into()))
    }

    async fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }
        self.post(texts).await
    }
}
