use scholion_core::{AvError, Result};

pub const DEFAULT_OLLAMA_EMBED_FAMILY: &str = "qwen3-embedding";
pub const DEFAULT_OLLAMA_URL: &str = "http://127.0.0.1:11434";

/// Where embedding requests go.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmbedChoice {
    /// Local `ollama` `/api/embed`. No API key.
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
    pub model_family: &'a str,
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
        return Err(AvError::Invalid(
            "AV_EMBED_PROVIDER=ollama but the ollama CLI is not on PATH".into(),
        ));
    }
    let models = detect.models.map_err(|err| {
        AvError::Invalid(format!(
            "ollama is installed but `ollama list` failed: {err}. Start ollama, or set AV_EMBED_PROVIDER=api"
        ))
    })?;
    match pick_embedding_model(models, detect.model_family) {
        Some(model) => Ok(Some(EmbedChoice::Ollama {
            base_url: detect.ollama_url.trim_end_matches('/').to_owned(),
            model: model.to_owned(),
        })),
        None => Err(AvError::Invalid(format!(
            "ollama is installed but no model matching {} is pulled. Run `ollama pull {}`, or set AV_EMBED_URL and AV_EMBED_PROVIDER=api",
            detect.model_family, detect.model_family
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
            model_family: DEFAULT_OLLAMA_EMBED_FAMILY,
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
    fn ollama_with_qwen3_embedding_is_the_default() {
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
    fn missing_qwen3_model_falls_back_to_the_api() {
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
    fn missing_qwen3_model_without_an_api_explains_the_pull() {
        let models = Ok(vec!["llama3.2:latest".into()]);
        let err = choose_embedder(&detect(true, &models, None, None, None)).unwrap_err();
        let AvError::Invalid(msg) = err else {
            panic!("invalid")
        };
        assert!(msg.contains("ollama pull qwen3-embedding"), "{msg}");
    }

    #[test]
    fn a_host_with_ollama_and_qwen3_embedding_selects_that_model() {
        let (cli, models) = detect_ollama();
        if !cli {
            return;
        }
        let Ok(list) = models.as_ref() else {
            return;
        };
        if pick_embedding_model(list, DEFAULT_OLLAMA_EMBED_FAMILY).is_none() {
            return;
        }
        let choice = choose_embedder(&detect(true, &models, None, None, None)).unwrap();
        match choice {
            Some(EmbedChoice::Ollama { model, .. }) => {
                assert!(model.starts_with(DEFAULT_OLLAMA_EMBED_FAMILY), "{model}");
            }
            other => panic!("expected ollama, got {other:?}"),
        }
    }

    #[test]
    fn provider_api_wins_even_when_qwen3_embedding_is_installed() {
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
