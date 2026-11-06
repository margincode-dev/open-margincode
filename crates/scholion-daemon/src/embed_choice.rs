use scholion_core::{AvError, Result};

/// Ollama embedding models tried in order when `AV_OLLAMA_EMBED_MODEL` is unset. `embeddinggemma-2`
/// (271M, 512 dimensions) scored the same as `qwen3-embedding` (7.6B, 4096) on the E1 retrieval
/// set (`bench/retrieval`, 2026-11-05) at a fraction of the size; a host that has only
/// `qwen3-embedding`, the earlier default, keeps it, so its vectors stay valid.
pub const DEFAULT_OLLAMA_EMBED_FAMILIES: &[&str] = &["embeddinggemma-2", "qwen3-embedding"];
pub const DEFAULT_OLLAMA_URL: &str = "http://127.0.0.1:11434";

/// Where embedding requests go.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmbedChoice {
    /// Ollama's `/api/embed`, on this host or another (`AV_OLLAMA_URL`). No API key.
    Ollama { base_url: String, model: String },
    /// Remote embedder. `api_key` is sent as `Authorization: Bearer` when set.
    Api {
        base_url: String,
        api_key: Option<String>,
    },
}

/// Facts the selector needs. `models` is the error from `ollama list` when the CLI exists but the server did not answer.
pub struct EmbedDetect<'a> {
    pub cli_installed: bool,
    pub models: std::result::Result<&'a [String], &'a str>,
    pub ollama_url: &'a str,
    /// Tried in order; the first one `ollama list` has wins.
    pub model_families: &'a [&'a str],
    /// `AV_OLLAMA_EMBED_MODEL`. With `provider` `ollama` and no CLI, the model is sent to
    /// `ollama_url` exactly as given, without listing it first.
    pub named_model: Option<&'a str>,
    pub embed_url: Option<&'a str>,
    pub api_key: Option<&'a str>,
    /// `None` is automatic. `ollama` or `api` forces that path (`AV_EMBED_PROVIDER`).
    pub provider: Option<&'a str>,
}

/// First column of `ollama list`. Skips the `NAME` header.
pub fn parse_ollama_list(stdout: &str) -> Vec<String> {
    stdout
        .lines()
        .filter_map(|line| {
            let name = line.split_whitespace().next()?;
            if name.eq_ignore_ascii_case("NAME") {
                None
            } else {
                Some(name.to_owned())
            }
        })
        .collect()
}

/// Prefer the bare family name, then `:latest`, then the first other tag.
pub fn pick_embedding_model<'a>(models: &'a [String], family: &str) -> Option<&'a str> {
    if let Some(exact) = models.iter().find(|name| name.as_str() == family) {
        return Some(exact);
    }
    let latest = format!("{family}:latest");
    if let Some(tagged) = models.iter().find(|name| name.as_str() == latest) {
        return Some(tagged);
    }
    let prefix = format!("{family}:");
    models
        .iter()
        .find(|name| name.starts_with(&prefix))
        .map(String::as_str)
}

pub fn choose_embedder(detect: &EmbedDetect<'_>) -> Result<Option<EmbedChoice>> {
    match detect.provider.map(str::trim).filter(|p| !p.is_empty()) {
        Some("ollama") => ollama_choice(detect),
        Some("api") => api_choice(detect, true),
        Some(other) => Err(AvError::Invalid(format!(
            "AV_EMBED_PROVIDER must be ollama or api, got {other}"
        ))),
        None if detect.cli_installed => match ollama_choice(detect) {
            Ok(choice) => Ok(choice),
            Err(_) => api_choice(detect, false),
        },
        None => api_choice(detect, false),
    }
}

fn ollama_choice(detect: &EmbedDetect<'_>) -> Result<Option<EmbedChoice>> {
    if !detect.cli_installed {
        return remote_ollama_choice(detect);
    }
    let models = detect.models.map_err(|err| {
        AvError::Invalid(format!(
            "ollama is installed but `ollama list` failed: {err}. Start ollama, or set AV_EMBED_PROVIDER=api"
        ))
    })?;
    let picked = detect
        .model_families
        .iter()
        .find_map(|family| pick_embedding_model(models, family));
    match picked {
        Some(model) => Ok(Some(EmbedChoice::Ollama {
            base_url: detect.ollama_url.trim_end_matches('/').to_owned(),
            model: model.to_owned(),
        })),
        None => {
            let first = detect.model_families.first().copied().unwrap_or_default();
            Err(AvError::Invalid(format!(
                "ollama is installed but no model matching {} is pulled. Run `ollama pull {first}`, or set AV_EMBED_URL and AV_EMBED_PROVIDER=api",
                detect.model_families.join(" or ")
            )))
        }
    }
}

/// `AV_EMBED_PROVIDER=ollama` with no CLI (the only way here: automatic selection tries Ollama only
/// when the CLI is present): an Ollama server elsewhere, such as another container or host, at
/// `AV_OLLAMA_URL`. Nothing here can list its models, so `AV_OLLAMA_EMBED_MODEL` must name one; it
/// is sent as given. Nothing connects now: the daemon boots while that server is down, and the
/// first embedding reports a server that is down or lacks the model.
fn remote_ollama_choice(detect: &EmbedDetect<'_>) -> Result<Option<EmbedChoice>> {
    match detect.named_model.map(str::trim).filter(|m| !m.is_empty()) {
        Some(model) => Ok(Some(EmbedChoice::Ollama {
            base_url: detect.ollama_url.trim().trim_end_matches('/').to_owned(),
            model: model.to_owned(),
        })),
        None => Err(AvError::Invalid(format!(
            "AV_EMBED_PROVIDER=ollama with no ollama CLI on PATH needs AV_OLLAMA_EMBED_MODEL, the model to ask the server at {} for (for example {})",
            detect.ollama_url, DEFAULT_OLLAMA_EMBED_FAMILIES[0]
        ))),
    }
}

/// `required` is set when the caller forced the API. Otherwise a missing URL with no key leaves embedding off,
/// and a missing URL after Ollama was not usable falls through to the Ollama error.
fn api_choice(detect: &EmbedDetect<'_>, required: bool) -> Result<Option<EmbedChoice>> {
    match detect
        .embed_url
        .map(str::trim)
        .filter(|url| !url.is_empty())
    {
        Some(url) => Ok(Some(EmbedChoice::Api {
            base_url: url.trim_end_matches('/').to_owned(),
            api_key: detect
                .api_key
                .map(str::trim)
                .filter(|key| !key.is_empty())
                .map(str::to_owned),
        })),
        None if detect
            .api_key
            .map(str::trim)
            .is_some_and(|key| !key.is_empty()) =>
        {
            Err(AvError::Invalid(
                "AV_EMBED_API_KEY is set but AV_EMBED_URL is empty".into(),
            ))
        }
        None if required => Err(AvError::Invalid(
            "AV_EMBED_PROVIDER=api but AV_EMBED_URL is empty".into(),
        )),
        None => {
            if detect.cli_installed {
                ollama_choice(detect)
            } else {
                Ok(None)
            }
        }
    }
}

/// `ollama --version`, then `ollama list` when the CLI is present.
pub fn detect_ollama() -> (bool, std::result::Result<Vec<String>, String>) {
    let version = std::process::Command::new("ollama")
        .arg("--version")
        .output();
    let installed = matches!(&version, Ok(out) if out.status.success());
    if !installed {
        return (false, Ok(Vec::new()));
    }
    match std::process::Command::new("ollama").arg("list").output() {
        Ok(out) if out.status.success() => (
            true,
            Ok(parse_ollama_list(&String::from_utf8_lossy(&out.stdout))),
        ),
        Ok(out) => (
            true,
            Err(String::from_utf8_lossy(&out.stderr).trim().to_owned()),
        ),
        Err(err) => (true, Err(err.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn detect<'a>(
        cli: bool,
        models: &'a std::result::Result<Vec<String>, String>,
        url: Option<&'a str>,
        key: Option<&'a str>,
        provider: Option<&'a str>,
    ) -> EmbedDetect<'a> {
        EmbedDetect {
            cli_installed: cli,
            models: match models {
                Ok(list) => Ok(list.as_slice()),
                Err(err) => Err(err.as_str()),
            },
            ollama_url: DEFAULT_OLLAMA_URL,
            model_families: DEFAULT_OLLAMA_EMBED_FAMILIES,
            named_model: None,
            embed_url: url,
            api_key: key,
            provider,
        }
    }

    #[test]
    fn parses_ollama_list_and_prefers_the_bare_name() {
        let stdout = "\
NAME                  ID              SIZE      MODIFIED
qwen3-embedding:8b    64b933495768    4.7 GB    6 minutes ago
llama3.2:latest       abc             2.0 GB    yesterday
";
        let models = parse_ollama_list(stdout);
        assert_eq!(
            models,
            vec![
                "qwen3-embedding:8b".to_owned(),
                "llama3.2:latest".to_owned()
            ]
        );
        assert_eq!(
            pick_embedding_model(&models, "qwen3-embedding"),
            Some("qwen3-embedding:8b")
        );
        let with_bare = vec!["qwen3-embedding:8b".into(), "qwen3-embedding".into()];
        assert_eq!(
            pick_embedding_model(&with_bare, "qwen3-embedding"),
            Some("qwen3-embedding")
        );
    }

    #[test]
    fn ollama_with_embeddinggemma_2_is_the_default() {
        let models = Ok(vec!["embeddinggemma-2:latest".into()]);
        let choice = choose_embedder(&detect(true, &models, None, None, None)).unwrap();
        assert_eq!(
            choice,
            Some(EmbedChoice::Ollama {
                base_url: DEFAULT_OLLAMA_URL.into(),
                model: "embeddinggemma-2:latest".into(),
            })
        );
    }

    #[test]
    fn embeddinggemma_2_wins_over_qwen3_embedding() {
        let models = Ok(vec![
            "qwen3-embedding:8b".into(),
            "embeddinggemma-2:latest".into(),
        ]);
        let choice = choose_embedder(&detect(true, &models, None, None, None)).unwrap();
        assert!(
            matches!(&choice, Some(EmbedChoice::Ollama { model, .. }) if model == "embeddinggemma-2:latest"),
            "{choice:?}"
        );
    }

    #[test]
    fn a_host_with_only_qwen3_embedding_keeps_using_it() {
        let models = Ok(vec!["qwen3-embedding:8b".into()]);
        let choice = choose_embedder(&detect(true, &models, None, None, None)).unwrap();
        assert_eq!(
            choice,
            Some(EmbedChoice::Ollama {
                base_url: DEFAULT_OLLAMA_URL.into(),
                model: "qwen3-embedding:8b".into(),
            })
        );
    }

    #[test]
    fn a_named_model_is_the_only_one_tried() {
        let models = Ok(vec![
            "embeddinggemma-2:latest".into(),
            "qwen3-embedding:8b".into(),
        ]);
        let mut d = detect(true, &models, None, None, None);
        d.model_families = &["qwen3-embedding"];
        let choice = choose_embedder(&d).unwrap();
        assert!(
            matches!(&choice, Some(EmbedChoice::Ollama { model, .. }) if model == "qwen3-embedding:8b"),
            "{choice:?}"
        );
        let only_gemma = Ok(vec!["embeddinggemma-2:latest".into()]);
        let mut d = detect(true, &only_gemma, None, None, None);
        d.model_families = &["qwen3-embedding"];
        let err = choose_embedder(&d).unwrap_err();
        assert!(
            err.to_string().contains("ollama pull qwen3-embedding"),
            "{err}"
        );
    }

    #[test]
    fn no_ollama_uses_the_api_key() {
        let models = Ok(vec![]);
        let choice = choose_embedder(&detect(
            false,
            &models,
            Some("https://embed.example/v1"),
            Some("sk-test"),
            None,
        ))
        .unwrap();
        assert_eq!(
            choice,
            Some(EmbedChoice::Api {
                base_url: "https://embed.example/v1".into(),
                api_key: Some("sk-test".into()),
            })
        );
    }

    #[test]
    fn no_ollama_and_no_url_leaves_embedding_off() {
        let models = Ok(vec![]);
        let choice = choose_embedder(&detect(false, &models, None, None, None)).unwrap();
        assert_eq!(choice, None);
    }

    #[test]
    fn api_key_without_a_url_is_an_error() {
        let models = Ok(vec![]);
        let err =
            choose_embedder(&detect(false, &models, None, Some("sk-test"), None)).unwrap_err();
        assert!(matches!(err, AvError::Invalid(_)), "{err}");
    }

    #[test]
    fn no_embedding_model_falls_back_to_the_api() {
        let models = Ok(vec!["llama3.2:latest".into()]);
        let choice = choose_embedder(&detect(
            true,
            &models,
            Some("https://embed.example"),
            Some("sk-test"),
            None,
        ))
        .unwrap();
        assert!(matches!(choice, Some(EmbedChoice::Api { .. })));
    }

    #[test]
    fn no_embedding_model_without_an_api_explains_the_pull() {
        let models = Ok(vec!["llama3.2:latest".into()]);
        let err = choose_embedder(&detect(true, &models, None, None, None)).unwrap_err();
        let AvError::Invalid(msg) = err else {
            panic!("invalid")
        };
        assert!(msg.contains("ollama pull embeddinggemma-2"), "{msg}");
    }

    #[test]
    fn a_host_with_ollama_and_a_default_model_selects_it() {
        let (cli, models) = detect_ollama();
        if !cli {
            return;
        }
        let Ok(list) = models.as_ref() else {
            return;
        };
        let Some(family) = DEFAULT_OLLAMA_EMBED_FAMILIES
            .iter()
            .find(|family| pick_embedding_model(list, family).is_some())
        else {
            return;
        };
        let choice = choose_embedder(&detect(true, &models, None, None, None)).unwrap();
        match choice {
            Some(EmbedChoice::Ollama { model, .. }) => {
                assert!(model.starts_with(family), "{model}");
            }
            other => panic!("expected ollama, got {other:?}"),
        }
    }

    #[test]
    fn provider_ollama_without_a_cli_uses_the_server_at_the_url() {
        // another container or host: nothing to list, the named model is sent as given
        let models = Ok(vec![]);
        let mut d = detect(false, &models, None, None, Some("ollama"));
        d.ollama_url = "http://ollama:11434/";
        d.named_model = Some(" embeddinggemma-2 ");
        assert_eq!(
            choose_embedder(&d).unwrap(),
            Some(EmbedChoice::Ollama {
                base_url: "http://ollama:11434".into(),
                model: "embeddinggemma-2".into(),
            })
        );
        // a tag is kept as written
        d.named_model = Some("qwen3-embedding:8b");
        assert!(
            matches!(choose_embedder(&d).unwrap(), Some(EmbedChoice::Ollama { model, .. }) if model == "qwen3-embedding:8b")
        );
        // an API URL set too does not win over the forced provider
        let mut d = detect(
            false,
            &models,
            Some("https://embed.example"),
            None,
            Some("ollama"),
        );
        d.named_model = Some("embeddinggemma-2");
        assert!(matches!(
            choose_embedder(&d).unwrap(),
            Some(EmbedChoice::Ollama { .. })
        ));
    }

    #[test]
    fn provider_ollama_without_a_cli_needs_the_model_named() {
        let models = Ok(vec![]);
        for named in [None, Some(""), Some("  ")] {
            let mut d = detect(false, &models, None, None, Some("ollama"));
            d.named_model = named;
            let err = choose_embedder(&d).unwrap_err();
            let AvError::Invalid(msg) = err else {
                panic!("invalid")
            };
            assert!(msg.contains("AV_OLLAMA_EMBED_MODEL"), "{msg}");
            assert!(msg.contains(DEFAULT_OLLAMA_URL), "{msg}");
        }
    }

    #[test]
    fn a_named_model_without_the_provider_or_with_a_cli_changes_nothing() {
        // automatic selection with no CLI: the named model alone does not reach for a server
        let models = Ok(vec![]);
        let mut d = detect(false, &models, None, None, None);
        d.named_model = Some("embeddinggemma-2");
        assert_eq!(choose_embedder(&d).unwrap(), None);
        // with the CLI, `ollama list` still resolves the name to a pulled tag
        let models = Ok(vec!["qwen3-embedding:8b".into()]);
        let mut d = detect(true, &models, None, None, Some("ollama"));
        d.model_families = &["qwen3-embedding"];
        d.named_model = Some("qwen3-embedding");
        assert!(
            matches!(choose_embedder(&d).unwrap(), Some(EmbedChoice::Ollama { model, .. }) if model == "qwen3-embedding:8b")
        );
    }

    #[test]
    fn provider_api_wins_even_when_an_embedding_model_is_installed() {
        let models = Ok(vec!["qwen3-embedding:8b".into()]);
        let choice = choose_embedder(&detect(
            true,
            &models,
            Some("https://embed.example"),
            Some("sk-test"),
            Some("api"),
        ))
        .unwrap();
        assert!(matches!(choice, Some(EmbedChoice::Api { .. })));
    }
}
