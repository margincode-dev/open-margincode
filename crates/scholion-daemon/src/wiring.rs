use std::path::PathBuf;
use std::sync::Arc;

use scholion_core::{AvError, Result};
use scholion_index::PgIndex;
use scholion_semantic::{Embedder, OllamaEmbedder, QdrantIndex, TeiEmbedder};
use scholion_store::PgStore;
use scholion_vcs::JjVcs;

use crate::AppState;
use crate::embed_choice::{
    DEFAULT_OLLAMA_EMBED_FAMILY, DEFAULT_OLLAMA_URL, EmbedChoice, EmbedDetect, choose_embedder,
    detect_ollama,
};

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub store_root: PathBuf,
    pub ws_root: PathBuf,
    pub ws_host_root: PathBuf,
    pub listen: String,
    /// `AV_EMBED_API_KEY`, trimmed. `None` means the embedder sends no `Authorization` header.
    pub embed_api_key: Option<String>,
    /// Ollama `qwen3-embedding` when the CLI has that model. Otherwise the API-key embedder, or off.
    pub embed: Option<EmbedChoice>,
    /// `AV_QDRANT_URL` (gRPC). The semantic layer is on only when this and `embed` are both set.
    pub qdrant_url: Option<String>,
    /// `AV_EXPAND_CAP_TOKENS`: per-session cap on tokens pulled through `expand`; unset = unlimited.
    pub expand_cap: Option<u64>,
    /// `AV_PG_MAX_CONNECTIONS` (default 16): Postgres pool size. Every concurrent agent request
    /// holds a connection while it runs, so raise it with the number of agents.
    pub pg_max_connections: u32,
    /// `AV_LEASE_SECS` (default 1800): how long a task's owner holds it without writing; after
    /// that another agent may claim it.
    pub lease_secs: u64,
    /// `AV_BACKUP_REMOTES`: whitespace-separated `repo=url` pairs, pushed after every merge.
    /// `POST /v1/repos/{name}/backup` sets one at runtime.
    pub backup_remotes: Vec<(String, String)>,
    /// `AV_TOKEN`: shared bearer token for every route but `/healthz`; unset = open.
    pub token: Option<String>,
}

const DEFAULT_PG_MAX_CONNECTIONS: u32 = 16;
pub(crate) const DEFAULT_LEASE_SECS: u64 = 1800;

impl Config {
    /// `AV_DATABASE_URL` (required), `AV_STORE_ROOT`, `AV_WS_ROOT`, `AV_WS_HOST_ROOT` (defaults to
    /// `AV_WS_ROOT`), `AV_LISTEN` (default `0.0.0.0:7420`), `AV_EMBED_API_KEY` (optional).
    ///
    /// Embedding: an `ollama` CLI with a `qwen3-embedding` model is used by default.
    /// With no CLI, `AV_EMBED_URL` plus an optional `AV_EMBED_API_KEY` is used.
    /// `AV_EMBED_PROVIDER=api` forces the API path. `AV_OLLAMA_URL` defaults to `http://127.0.0.1:11434`.
    pub fn from_env() -> Result<Config> {
        let get = |k: &str| std::env::var(k).ok().filter(|v| !v.is_empty());
        let ws_root = PathBuf::from(get("AV_WS_ROOT").unwrap_or_else(|| "./ws".into()));
        let embed_api_key = normalize_secret(get("AV_EMBED_API_KEY"));
        let ollama_url = get("AV_OLLAMA_URL").unwrap_or_else(|| DEFAULT_OLLAMA_URL.to_owned());
        let model_family =
            get("AV_OLLAMA_EMBED_MODEL").unwrap_or_else(|| DEFAULT_OLLAMA_EMBED_FAMILY.to_owned());
        let embed_url = get("AV_EMBED_URL");
        let provider = get("AV_EMBED_PROVIDER");
        let (cli_installed, models) = detect_ollama();
        let embed = choose_embedder(&EmbedDetect {
            cli_installed,
            models: models.as_ref().map(Vec::as_slice).map_err(String::as_str),
            ollama_url: &ollama_url,
            model_family: &model_family,
            embed_url: embed_url.as_deref(),
            api_key: embed_api_key.as_deref(),
            provider: provider.as_deref(),
        })?;
        Ok(Config {
            database_url: get("AV_DATABASE_URL")
                .ok_or_else(|| AvError::Invalid("AV_DATABASE_URL is required".into()))?,
            store_root: PathBuf::from(get("AV_STORE_ROOT").unwrap_or_else(|| "./store".into())),
            ws_host_root: get("AV_WS_HOST_ROOT")
                .map(PathBuf::from)
                .unwrap_or_else(|| ws_root.clone()),
            ws_root,
            listen: get("AV_LISTEN").unwrap_or_else(|| "0.0.0.0:7420".into()),
            embed_api_key,
            embed,
            qdrant_url: get("AV_QDRANT_URL"),
            expand_cap: match get("AV_EXPAND_CAP_TOKENS") {
                Some(v) => Some(v.trim().parse().map_err(|_| {
                    AvError::Invalid(format!("AV_EXPAND_CAP_TOKENS must be a number, got {v}"))
                })?),
                None => None,
            },
            pg_max_connections: match get("AV_PG_MAX_CONNECTIONS") {
                Some(v) => v.trim().parse().ok().filter(|n| *n > 0).ok_or_else(|| {
                    AvError::Invalid(format!(
                        "AV_PG_MAX_CONNECTIONS must be a positive number, got {v}"
                    ))
                })?,
                None => DEFAULT_PG_MAX_CONNECTIONS,
            },
            lease_secs: match get("AV_LEASE_SECS") {
                Some(v) => v.trim().parse().map_err(|_| {
                    AvError::Invalid(format!("AV_LEASE_SECS must be a number, got {v}"))
                })?,
                None => DEFAULT_LEASE_SECS,
            },
            backup_remotes: parse_backup_remotes(get("AV_BACKUP_REMOTES").as_deref())?,
            token: normalize_secret(get("AV_TOKEN")),
        })
    }
}

/// `repo=url` pairs separated by whitespace.
fn parse_backup_remotes(value: Option<&str>) -> Result<Vec<(String, String)>> {
    value
        .unwrap_or_default()
        .split_whitespace()
        .map(|pair| match pair.split_once('=') {
            Some((repo, url)) if !repo.is_empty() && !url.is_empty() => {
                Ok((repo.to_owned(), url.to_owned()))
            }
            _ => Err(AvError::Invalid(format!(
                "AV_BACKUP_REMOTES: expected repo=url, got {pair:?}"
            ))),
        })
        .collect()
}

/// Blank and whitespace-only secrets are absent. Surrounding whitespace is stripped.
fn normalize_secret(value: Option<String>) -> Option<String> {
    value.and_then(|raw| {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_owned())
        }
    })
}

/// `Authorization` header value for the embedding API.
pub fn embed_authorization(api_key: Option<&str>) -> Option<String> {
    normalize_secret(api_key.map(str::to_owned)).map(|key| format!("Bearer {key}"))
}

pub async fn build_state(cfg: &Config) -> Result<AppState> {
    let store = Arc::new(PgStore::connect_with(&cfg.database_url, cfg.pg_max_connections).await?);
    let index = Arc::new(PgIndex::new(store.pool().clone()));
    let vcs = Arc::new(JjVcs::new(cfg.store_root.clone(), cfg.ws_root.clone()));
    let state = AppState::new(
        vcs,
        store,
        index,
        cfg.ws_root.clone(),
        cfg.ws_host_root.clone(),
    )
    .with_expand_cap(cfg.expand_cap)
    .with_lease_secs(cfg.lease_secs)
    .with_token(cfg.token.clone());
    // Neither backend connects here: the daemon boots even when Qdrant or the embedder is down.
    Ok(match (&cfg.embed, &cfg.qdrant_url) {
        (Some(choice), Some(qdrant)) => {
            let embedder: Arc<dyn Embedder> = match choice {
                EmbedChoice::Ollama { base_url, model } => {
                    Arc::new(OllamaEmbedder::new(base_url, model))
                }
                EmbedChoice::Api { base_url, api_key } => {
                    Arc::new(TeiEmbedder::with_api_key(base_url.clone(), api_key.clone()))
                }
            };
            state.with_index_backend(embedder, Arc::new(QdrantIndex::new(qdrant.clone())))
        }
        _ => state,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backup_remotes_parse_repo_url_pairs() {
        assert_eq!(
            parse_backup_remotes(Some(" a=/srv/a.git\n b=git@host:me/b.git ")).unwrap(),
            vec![
                ("a".to_owned(), "/srv/a.git".to_owned()),
                ("b".to_owned(), "git@host:me/b.git".to_owned())
            ]
        );
        assert!(parse_backup_remotes(None).unwrap().is_empty());
        assert!(parse_backup_remotes(Some("no-equals")).is_err());
        assert!(parse_backup_remotes(Some("=url")).is_err());
    }

    #[test]
    fn blank_embed_api_key_is_absent_and_a_real_key_is_a_bearer_token() {
        assert_eq!(normalize_secret(None), None);
        assert_eq!(normalize_secret(Some(String::new())), None);
        assert_eq!(normalize_secret(Some("   ".into())), None);
        assert_eq!(
            normalize_secret(Some("  sk-test  ".into())).as_deref(),
            Some("sk-test")
        );
        assert_eq!(embed_authorization(None), None);
        assert_eq!(embed_authorization(Some("  ")), None);
        assert_eq!(
            embed_authorization(Some("sk-test")).as_deref(),
            Some("Bearer sk-test")
        );
    }
}
