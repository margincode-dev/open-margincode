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
    /// `AV_DATABASE_URL`. Unset = lite mode: the daemon runs its own Postgres in `pg_dir`.
    pub database_url: Option<String>,
    /// `AV_PG_DIR` (default `$AV_HOME/pg`, `AV_HOME` defaulting to `~/.scholion`): data, password
    /// and binaries of the daemon's own Postgres.
    pub pg_dir: PathBuf,
    /// `AV_PG_INSTALL_DIR`: an existing PostgreSQL installation (the directory holding `bin/`) for
    /// lite mode, instead of the bundled or downloaded one.
    pub pg_install_dir: Option<PathBuf>,
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
    /// `AV_INDEX_MAX_FANOUT`: a call whose name more than this many other symbols define links
    /// to none of them in the code graph; unset = every one. Stored edges keep the cap they were
    /// built with: set it before a repo is imported (reindex skips unchanged files).
    pub index_max_fanout: Option<u32>,
    /// `AV_PG_MAX_CONNECTIONS` (default 16): Postgres pool size. Every concurrent agent request
    /// holds a connection while it runs, so raise it with the number of agents.
    pub pg_max_connections: u32,
    /// `AV_MAX_CONCURRENT_STARTS` (default a quarter of the pool): task starts that copy an index
    /// at once. The others wait their turn instead of failing.
    pub max_concurrent_starts: Option<usize>,
    /// `AV_LEASE_SECS` (default 1800): how long a task's owner holds it without writing; after
    /// that another agent may claim it.
    pub lease_secs: u64,
    /// `AV_ATTN_REVIEW_SECS`, `AV_ATTN_APPROVED_SECS`, `AV_ATTN_HANDOFF_SECS`,
    /// `AV_ATTN_FOOTPRINT_BYTES`, `AV_ATTN_FOOTPRINT_SECS`: when `GET /v1/attention` lists a
    /// waiting task or a footprint (defaults in `AttentionLimits::default`).
    pub attention: crate::AttentionLimits,
    /// `AV_WS_RETAIN_SECS` (default 3600; `never` = `None`): how long the workspace of a merged
    /// or abandoned task stays on disk before the daemon removes it. `0` removes it at once.
    pub ws_retain_secs: Option<u64>,
    /// `AV_BUILD_CACHE` (default on; `off`, `0` or `false` turn it off): a repo with a root
    /// `Cargo.toml` gets one cargo target dir for all its task workspaces.
    pub build_cache: bool,
    /// `AV_BACKUP_REMOTES`: whitespace-separated `repo=url` pairs, pushed after every merge.
    /// `POST /v1/repos/{name}/backup` sets one at runtime.
    pub backup_remotes: Vec<(String, String)>,
    /// `AV_TOKEN`: shared bearer token for every route but `/healthz` and `/ui`; unset = open.
    pub token: Option<String>,
}

const DEFAULT_PG_MAX_CONNECTIONS: u32 = 16;
pub(crate) const DEFAULT_LEASE_SECS: u64 = 1800;
pub(crate) const DEFAULT_WS_RETAIN_SECS: u64 = 3600;

impl Config {
    /// `AV_DATABASE_URL` (unset = lite mode, see `pg_dir`), `AV_STORE_ROOT`, `AV_WS_ROOT`, `AV_WS_HOST_ROOT` (defaults to
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
        let av_home = get("AV_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(default_av_home);
        Ok(Config {
            database_url: get("AV_DATABASE_URL"),
            pg_dir: get("AV_PG_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|| av_home.join("pg")),
            pg_install_dir: get("AV_PG_INSTALL_DIR").map(PathBuf::from),
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
            index_max_fanout: match get("AV_INDEX_MAX_FANOUT") {
                Some(v) => Some(v.trim().parse().ok().filter(|n| *n > 0).ok_or_else(|| {
                    AvError::Invalid(format!(
                        "AV_INDEX_MAX_FANOUT must be a positive number, got {v}"
                    ))
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
            max_concurrent_starts: match get("AV_MAX_CONCURRENT_STARTS") {
                Some(v) => Some(v.trim().parse().ok().filter(|n| *n > 0).ok_or_else(|| {
                    AvError::Invalid(format!(
                        "AV_MAX_CONCURRENT_STARTS must be a positive number, got {v}"
                    ))
                })?),
                None => None,
            },
            lease_secs: match get("AV_LEASE_SECS") {
                Some(v) => v.trim().parse().map_err(|_| {
                    AvError::Invalid(format!("AV_LEASE_SECS must be a number, got {v}"))
                })?,
                None => DEFAULT_LEASE_SECS,
            },
            attention: parse_attention(&get)?,
            ws_retain_secs: parse_ws_retain(get("AV_WS_RETAIN_SECS").as_deref())?,
            build_cache: parse_switch("AV_BUILD_CACHE", get("AV_BUILD_CACHE").as_deref())?,
            backup_remotes: parse_backup_remotes(get("AV_BACKUP_REMOTES").as_deref())?,
            token: normalize_secret(get("AV_TOKEN")),
        })
    }
}

/// `~/.scholion`, or `./.scholion` when there is no home directory.
fn default_av_home() -> PathBuf {
    std::env::var_os("HOME")
        .filter(|h| !h.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".scholion")
}

/// The `AV_ATTN_*` thresholds; each unset one keeps its default.
fn parse_attention(get: &dyn Fn(&str) -> Option<String>) -> Result<crate::AttentionLimits> {
    let d = crate::AttentionLimits::default();
    let num = |name: &str, default: u64| -> Result<u64> {
        match get(name) {
            Some(v) => v
                .trim()
                .parse()
                .map_err(|_| AvError::Invalid(format!("{name} must be a whole number, got {v}"))),
            None => Ok(default),
        }
    };
    Ok(crate::AttentionLimits {
        review_secs: num("AV_ATTN_REVIEW_SECS", d.review_secs)?,
        approved_secs: num("AV_ATTN_APPROVED_SECS", d.approved_secs)?,
        handoff_secs: num("AV_ATTN_HANDOFF_SECS", d.handoff_secs)?,
        footprint_bytes: num("AV_ATTN_FOOTPRINT_BYTES", d.footprint_bytes)?,
        footprint_secs: num("AV_ATTN_FOOTPRINT_SECS", d.footprint_secs)?,
    })
}

/// `AV_WS_RETAIN_SECS`: seconds, or `never`; unset is `DEFAULT_WS_RETAIN_SECS`.
fn parse_ws_retain(value: Option<&str>) -> Result<Option<u64>> {
    match value.map(str::trim) {
        None | Some("") => Ok(Some(DEFAULT_WS_RETAIN_SECS)),
        Some(v) if v.eq_ignore_ascii_case("never") => Ok(None),
        Some(v) => v.parse().map(Some).map_err(|_| {
            AvError::Invalid(format!(
                "AV_WS_RETAIN_SECS must be a number of seconds or `never`, got {v}"
            ))
        }),
    }
}

/// An on/off setting: unset or blank is on; `off`, `0`, `false`, `no` are off; `on`, `1`, `true`,
/// `yes` are on; anything else is `Invalid`.
fn parse_switch(name: &str, value: Option<&str>) -> Result<bool> {
    match value.map(|v| v.trim().to_ascii_lowercase()).as_deref() {
        None | Some("") | Some("on" | "1" | "true" | "yes") => Ok(true),
        Some("off" | "0" | "false" | "no") => Ok(false),
        Some(v) => Err(AvError::Invalid(format!(
            "{name} must be on or off, got {v}"
        ))),
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

/// `database_url` is `cfg.database_url`, or the lite-mode server's when that is unset.
pub async fn build_state(cfg: &Config, database_url: &str) -> Result<AppState> {
    let store = Arc::new(PgStore::connect_with(database_url, cfg.pg_max_connections).await?);
    let index = Arc::new(PgIndex::new(store.pool().clone()).with_max_fanout(cfg.index_max_fanout));
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
    .with_attention(cfg.attention)
    .with_ws_retain(cfg.ws_retain_secs)
    .with_build_cache(cfg.build_cache)
    .with_disk_paths(
        Some(cfg.store_root.clone()),
        cfg.database_url.is_none().then(|| cfg.pg_dir.clone()),
    )
    .with_token(cfg.token.clone());
    let state = match cfg.max_concurrent_starts {
        Some(n) => state.with_start_limit(n),
        None => state,
    };
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
    fn build_cache_is_on_unless_turned_off() {
        assert!(parse_switch("X", None).unwrap());
        assert!(parse_switch("X", Some("")).unwrap());
        assert!(parse_switch("X", Some("on")).unwrap());
        assert!(!parse_switch("X", Some("off")).unwrap());
        assert!(!parse_switch("X", Some("0")).unwrap());
        assert!(!parse_switch("X", Some(" FALSE ")).unwrap());
        assert!(parse_switch("X", Some("maybe")).is_err());
    }

    #[test]
    fn attention_limits_default_and_parse() {
        let none = |_: &str| None;
        assert_eq!(
            parse_attention(&none).unwrap(),
            crate::AttentionLimits::default()
        );
        let some = |k: &str| (k == "AV_ATTN_REVIEW_SECS").then(|| " 60 ".to_owned());
        let l = parse_attention(&some).unwrap();
        assert_eq!(l.review_secs, 60);
        assert_eq!(l.approved_secs, 3600);
        let bad = |k: &str| (k == "AV_ATTN_HANDOFF_SECS").then(|| "1h".to_owned());
        assert!(matches!(parse_attention(&bad), Err(AvError::Invalid(_))));
    }

    #[test]
    fn ws_retain_is_seconds_or_never() {
        assert_eq!(parse_ws_retain(None).unwrap(), Some(3600));
        assert_eq!(parse_ws_retain(Some("")).unwrap(), Some(3600));
        assert_eq!(parse_ws_retain(Some("0")).unwrap(), Some(0));
        assert_eq!(parse_ws_retain(Some(" 120 ")).unwrap(), Some(120));
        assert_eq!(parse_ws_retain(Some("never")).unwrap(), None);
        assert_eq!(parse_ws_retain(Some("NEVER")).unwrap(), None);
        assert!(matches!(
            parse_ws_retain(Some("1h")),
            Err(AvError::Invalid(_))
        ));
        assert!(matches!(
            parse_ws_retain(Some("-1")),
            Err(AvError::Invalid(_))
        ));
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
