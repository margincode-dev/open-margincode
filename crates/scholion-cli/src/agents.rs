//! `av init --for NAME`: wires an agent CLI to the daemon's MCP server (`/mcp`). Claude Code
//! also gets hooks (`local::install_claude_hooks`); Codex, Grok Build and Gemini CLI get the MCP
//! server only, and their agent calls `start_task` and `context` itself (or runs under
//! `integrations/generic/av-wrap.sh`). Each writer merges into the CLI's own config file, keeps
//! everything else in it, replaces an earlier `av` entry, and keeps the previous file as `*.bak`.

use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use crate::client::{CliError, Result};

/// The MCP server's name in every agent's config.
pub const SERVER: &str = "av";

#[derive(Clone, Copy, Debug, PartialEq, Eq, clap::ValueEnum)]
pub enum Agent {
    /// Claude Code: hooks in ~/.claude/settings.json, MCP through `claude mcp add`.
    Claude,
    /// OpenAI Codex CLI: MCP in $CODEX_HOME/config.toml (default ~/.codex).
    Codex,
    /// xAI Grok Build: MCP in ~/.grok/config.toml.
    Grok,
    /// Google Gemini CLI: MCP in ~/.gemini/settings.json.
    Gemini,
}

impl Agent {
    pub fn name(self) -> &'static str {
        match self {
            Agent::Claude => "Claude Code",
            Agent::Codex => "Codex",
            Agent::Grok => "Grok Build",
            Agent::Gemini => "Gemini CLI",
        }
    }

    /// The config file `av init` writes for this agent, under `home`.
    pub fn config_path(self, home: &Path, codex_home: Option<PathBuf>) -> PathBuf {
        match self {
            Agent::Claude => home.join(".claude/settings.json"),
            Agent::Codex => codex_home
                .unwrap_or_else(|| home.join(".codex"))
                .join("config.toml"),
            Agent::Grok => home.join(".grok/config.toml"),
            Agent::Gemini => home.join(".gemini/settings.json"),
        }
    }
}

fn local(m: impl Into<String>) -> CliError {
    CliError::Local(m.into())
}

/// Codex: `[mcp_servers.av] url = …`; with a token, Codex reads it from `AV_TOKEN` at start
/// (`bearer_token_env_var`), so the file never holds the secret.
pub fn install_codex(path: &Path, mcp_url: &str, token: bool) -> Result<()> {
    edit_toml(path, |doc| {
        let mut server = toml_edit::Table::new();
        server["url"] = toml_edit::value(mcp_url);
        if token {
            server["bearer_token_env_var"] = toml_edit::value("AV_TOKEN");
        }
        mcp_servers(doc)?.insert(SERVER, toml_edit::Item::Table(server));
        Ok(())
    })
}

/// Grok Build: `[mcp_servers.av] url = …`; it expands `${AV_TOKEN}` in headers when it starts.
pub fn install_grok(path: &Path, mcp_url: &str, token: bool) -> Result<()> {
    edit_toml(path, |doc| {
        let mut server = toml_edit::Table::new();
        server["url"] = toml_edit::value(mcp_url);
        if token {
            let mut headers = toml_edit::InlineTable::new();
            headers.insert("Authorization", "Bearer ${AV_TOKEN}".into());
            server["headers"] = toml_edit::value(headers);
        }
        mcp_servers(doc)?.insert(SERVER, toml_edit::Item::Table(server));
        Ok(())
    })
}

/// Gemini CLI: `mcpServers.av.httpUrl` (streamable HTTP); it expands `${AV_TOKEN}` in headers.
pub fn install_gemini(path: &Path, mcp_url: &str, token: bool) -> Result<()> {
    edit_json(path, |settings| {
        let mut server = json!({ "httpUrl": mcp_url });
        if token {
            server["headers"] = json!({ "Authorization": "Bearer ${AV_TOKEN}" });
        }
        let servers = settings
            .as_object_mut()
            .expect("checked by edit_json")
            .entry("mcpServers")
            .or_insert_with(|| json!({}));
        if !servers.is_object() {
            *servers = json!({});
        }
        servers[SERVER] = server;
        Ok(())
    })
}

fn mcp_servers(doc: &mut toml_edit::DocumentMut) -> Result<&mut toml_edit::Table> {
    let item = doc
        .entry("mcp_servers")
        .or_insert_with(|| toml_edit::Item::Table(toml_edit::Table::new()));
    let table = item
        .as_table_mut()
        .ok_or_else(|| local("`mcp_servers` is not a table; nothing changed"))?;
    table.set_implicit(true);
    Ok(table)
}

fn edit_toml(
    path: &Path,
    change: impl FnOnce(&mut toml_edit::DocumentMut) -> Result<()>,
) -> Result<()> {
    let text = std::fs::read_to_string(path).unwrap_or_default();
    let mut doc: toml_edit::DocumentMut = text.parse().map_err(|e| {
        local(format!(
            "{} is not valid TOML ({e}); nothing changed",
            path.display()
        ))
    })?;
    change(&mut doc).map_err(|e| local(format!("{}: {e}", path.display())))?;
    write_with_backup(path, &doc.to_string())
}

/// Reads `path` as a JSON object (an absent file is `{}`), changes it, writes it back.
pub fn edit_json(path: &Path, change: impl FnOnce(&mut Value) -> Result<()>) -> Result<()> {
    let mut v: Value = match std::fs::read_to_string(path) {
        Ok(text) => serde_json::from_str(&text).map_err(|e| {
            local(format!(
                "{} is not valid JSON ({e}); nothing changed",
                path.display()
            ))
        })?,
        Err(_) => json!({}),
    };
    if !v.is_object() {
        return Err(local(format!(
            "{} is not a JSON object; nothing changed",
            path.display()
        )));
    }
    change(&mut v)?;
    write_with_backup(
        path,
        &(serde_json::to_string_pretty(&v).expect("serializable") + "\n"),
    )
}

/// Writes `text` to `path`, keeping the previous file as `<path>.bak`.
pub fn write_with_backup(path: &Path, text: &str) -> Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)
            .map_err(|e| local(format!("cannot create {}: {e}", dir.display())))?;
    }
    if path.exists() {
        let mut bak = path.as_os_str().to_owned();
        bak.push(".bak");
        std::fs::copy(path, PathBuf::from(bak))
            .map_err(|e| local(format!("cannot back up {}: {e}", path.display())))?;
    }
    std::fs::write(path, text).map_err(|e| local(format!("cannot write {}: {e}", path.display())))
}

#[cfg(test)]
mod tests {
    use super::*;

    const URL: &str = "http://localhost:7420/mcp";

    fn read(path: &Path) -> String {
        std::fs::read_to_string(path).unwrap()
    }

    #[test]
    fn codex_keeps_other_settings_and_replaces_its_entry() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.toml");
        std::fs::write(
            &path,
            "model = \"gpt-5\" # mine\n\n[mcp_servers.other]\ncommand = \"x\"\n\n[mcp_servers.av]\nurl = \"http://old/mcp\"\n",
        )
        .unwrap();
        install_codex(&path, URL, true).unwrap();
        install_codex(&path, URL, true).unwrap();
        let text = read(&path);
        assert!(text.contains("model = \"gpt-5\" # mine"), "{text}");
        let doc: toml_edit::DocumentMut = text.parse().unwrap();
        assert_eq!(doc["mcp_servers"]["other"]["command"].as_str(), Some("x"));
        assert_eq!(doc["mcp_servers"]["av"]["url"].as_str(), Some(URL));
        assert_eq!(
            doc["mcp_servers"]["av"]["bearer_token_env_var"].as_str(),
            Some("AV_TOKEN")
        );
        assert!(!text.contains("http://old"), "{text}");
        assert!(dir.path().join("config.toml.bak").exists());
    }

    #[test]
    fn codex_without_a_token_writes_only_the_url() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("codex/config.toml");
        install_codex(&path, URL, false).unwrap();
        assert_eq!(read(&path), format!("[mcp_servers.av]\nurl = \"{URL}\"\n"));
    }

    #[test]
    fn grok_puts_the_token_in_a_header_from_the_environment() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.toml");
        install_grok(&path, URL, true).unwrap();
        let doc: toml_edit::DocumentMut = read(&path).parse().unwrap();
        assert_eq!(doc["mcp_servers"]["av"]["url"].as_str(), Some(URL));
        assert_eq!(
            doc["mcp_servers"]["av"]["headers"]["Authorization"].as_str(),
            Some("Bearer ${AV_TOKEN}")
        );
    }

    #[test]
    fn gemini_merges_into_settings_json() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        std::fs::write(
            &path,
            r#"{"theme":"dark","mcpServers":{"other":{"command":"x"}}}"#,
        )
        .unwrap();
        install_gemini(&path, URL, false).unwrap();
        let v: Value = serde_json::from_str(&read(&path)).unwrap();
        assert_eq!(v["theme"], "dark");
        assert_eq!(v["mcpServers"]["other"]["command"], "x");
        assert_eq!(v["mcpServers"]["av"], json!({ "httpUrl": URL }));
        assert!(dir.path().join("settings.json.bak").exists());
    }

    #[test]
    fn broken_config_files_are_left_alone() {
        let dir = tempfile::tempdir().unwrap();
        let toml = dir.path().join("config.toml");
        std::fs::write(&toml, "[[[ nope").unwrap();
        assert!(install_codex(&toml, URL, false).is_err());
        assert_eq!(read(&toml), "[[[ nope");
        let json = dir.path().join("settings.json");
        std::fs::write(&json, "{ nope").unwrap();
        assert!(install_gemini(&json, URL, false).is_err());
        assert_eq!(read(&json), "{ nope");
    }

    #[test]
    fn config_paths() {
        let home = Path::new("/h");
        assert_eq!(
            Agent::Codex.config_path(home, None),
            Path::new("/h/.codex/config.toml")
        );
        assert_eq!(
            Agent::Codex.config_path(home, Some("/c".into())),
            Path::new("/c/config.toml")
        );
        assert_eq!(
            Agent::Gemini.config_path(home, None),
            Path::new("/h/.gemini/settings.json")
        );
        assert_eq!(
            Agent::Grok.config_path(home, None),
            Path::new("/h/.grok/config.toml")
        );
    }
}
