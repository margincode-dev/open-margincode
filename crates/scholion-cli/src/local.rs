//! `av up`, `av down` and `av init`: a daemon on this machine (lite mode: it runs its own
//! Postgres unless `AV_DATABASE_URL` is set) and a repository wired to it in one command.

use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use serde_json::{Value, json};

use crate::agents::{self, Agent};
use crate::client::{CliError, Client, Result};

/// First start may unpack Postgres and run initdb.
const UP_TIMEOUT: Duration = Duration::from_secs(180);
/// The daemon stops its Postgres before it exits.
const DOWN_TIMEOUT: Duration = Duration::from_secs(60);

/// `$AV_HOME`, else `~/.scholion` (the daemon uses the same default).
pub fn av_home() -> PathBuf {
    env_path("AV_HOME").unwrap_or_else(|| home().join(".scholion"))
}

fn home() -> PathBuf {
    env_path("HOME").unwrap_or_else(|| PathBuf::from("."))
}

fn env_path(key: &str) -> Option<PathBuf> {
    std::env::var_os(key)
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
}

fn local(m: impl Into<String>) -> CliError {
    CliError::Local(m.into())
}

fn healthy(client: &Client) -> bool {
    client.get_text("/healthz").is_ok()
}

/// Port of a daemon URL on this machine; `None` for any other host.
fn local_port(url: &str) -> Option<u16> {
    let rest = url.split_once("://").map_or(url, |(_, r)| r);
    let authority = rest.split('/').next().unwrap_or_default();
    let (host, port) = match authority.rsplit_once(':') {
        Some((h, p)) => (h, p.parse().ok()?),
        None => (authority, 80),
    };
    matches!(host, "localhost" | "127.0.0.1" | "[::1]").then_some(port)
}

/// `scholion-daemon`: `$AV_DAEMON_BIN`, else next to this executable, else on `PATH`.
fn daemon_bin() -> Result<PathBuf> {
    if let Some(bin) = env_path("AV_DAEMON_BIN") {
        return Ok(bin);
    }
    let name = format!("scholion-daemon{}", std::env::consts::EXE_SUFFIX);
    let sibling = std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|d| d.join(&name)));
    let on_path = std::env::var_os("PATH")
        .map(|p| {
            std::env::split_paths(&p)
                .map(|d| d.join(&name))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    sibling
        .into_iter()
        .chain(on_path)
        .find(|p| p.is_file())
        .ok_or_else(|| {
            local(format!(
                "cannot find {name} next to this program or on PATH (set AV_DAEMON_BIN)"
            ))
        })
}

/// Starts a daemon for `url` in the background unless one already answers there.
pub fn up(client: &Client, url: &str) -> Result<()> {
    if healthy(client) {
        println!("daemon already running at {url} (dashboard: {url}/ui)");
        return Ok(());
    }
    let port = local_port(url).ok_or_else(|| {
        local(format!(
            "av up starts a daemon on this machine, but AV_URL is {url}; unset it or use localhost"
        ))
    })?;
    let bin = daemon_bin()?;
    let home_dir = av_home();
    std::fs::create_dir_all(&home_dir)
        .map_err(|e| local(format!("cannot create {}: {e}", home_dir.display())))?;
    let log_path = home_dir.join("daemon.log");
    let log = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .map_err(|e| local(format!("cannot open {}: {e}", log_path.display())))?;
    let log_offset = log.metadata().map(|m| m.len()).unwrap_or(0);
    let mut cmd = Command::new(&bin);
    cmd.env("AV_HOME", &home_dir)
        .env("AV_LISTEN", format!("127.0.0.1:{port}"))
        .stdin(Stdio::null())
        .stdout(log.try_clone().map_err(|e| local(e.to_string()))?)
        .stderr(log);
    if env_path("AV_STORE_ROOT").is_none() {
        cmd.env("AV_STORE_ROOT", home_dir.join("store"));
    }
    if env_path("AV_WS_ROOT").is_none() {
        cmd.env("AV_WS_ROOT", home().join("ws"));
    }
    #[cfg(unix)]
    {
        // its own process group: Ctrl-C in this terminal must not stop it
        use std::os::unix::process::CommandExt;
        cmd.process_group(0);
    }
    let mut child = cmd
        .spawn()
        .map_err(|e| local(format!("cannot start {}: {e}", bin.display())))?;
    let pid_path = home_dir.join("daemon.pid");
    std::fs::write(&pid_path, child.id().to_string())
        .map_err(|e| local(format!("cannot write {}: {e}", pid_path.display())))?;
    let since = Instant::now();
    loop {
        if healthy(client) {
            break;
        }
        if let Ok(Some(status)) = child.try_wait() {
            let _ = std::fs::remove_file(&pid_path);
            return Err(local(format!(
                "the daemon exited ({status}) before it was ready; {}:\n{}",
                log_path.display(),
                log_tail(&log_path, log_offset)
            )));
        }
        if since.elapsed() > UP_TIMEOUT {
            return Err(local(format!(
                "the daemon (pid {}) is not answering at {url} after {}s; see {}",
                child.id(),
                UP_TIMEOUT.as_secs(),
                log_path.display()
            )));
        }
        std::thread::sleep(Duration::from_millis(200));
    }
    println!(
        "daemon running at {url} (pid {}; dashboard: {url}/ui; log: {})",
        child.id(),
        log_path.display()
    );
    Ok(())
}

/// The last 20 lines written to the log since `offset`.
fn log_tail(path: &Path, offset: u64) -> String {
    let text = std::fs::read(path).unwrap_or_default();
    let new = String::from_utf8_lossy(text.get(offset as usize..).unwrap_or_default()).into_owned();
    let lines: Vec<&str> = new.lines().collect();
    lines[lines.len().saturating_sub(20)..].join("\n")
}

/// Stops the daemon `av up` started.
pub fn down(client: &Client, url: &str) -> Result<()> {
    let pid_path = av_home().join("daemon.pid");
    let Ok(text) = std::fs::read_to_string(&pid_path) else {
        if healthy(client) {
            return Err(local(format!(
                "a daemon answers at {url}, but `av up` did not start it (no {}); stop it where it was started",
                pid_path.display()
            )));
        }
        println!("no daemon running");
        return Ok(());
    };
    let pid = text.trim().to_owned();
    if !alive(&pid) {
        let _ = std::fs::remove_file(&pid_path);
        println!("no daemon running (removed a stale {})", pid_path.display());
        return Ok(());
    }
    signal(&pid, "-TERM")?;
    let since = Instant::now();
    while alive(&pid) {
        if since.elapsed() > DOWN_TIMEOUT {
            return Err(local(format!(
                "daemon (pid {pid}) still running {}s after SIGTERM",
                DOWN_TIMEOUT.as_secs()
            )));
        }
        std::thread::sleep(Duration::from_millis(200));
    }
    let _ = std::fs::remove_file(&pid_path);
    println!("daemon stopped (pid {pid})");
    Ok(())
}

#[cfg(unix)]
fn alive(pid: &str) -> bool {
    Command::new("kill")
        .args(["-0", pid])
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}

#[cfg(unix)]
fn signal(pid: &str, sig: &str) -> Result<()> {
    let status = Command::new("kill")
        .args([sig, pid])
        .status()
        .map_err(|e| local(format!("kill {sig} {pid}: {e}")))?;
    if status.success() {
        Ok(())
    } else {
        Err(local(format!("kill {sig} {pid} failed")))
    }
}

#[cfg(not(unix))]
fn alive(_pid: &str) -> bool {
    false
}

#[cfg(not(unix))]
fn signal(pid: &str, _sig: &str) -> Result<()> {
    Err(local(format!(
        "av down is not supported on this platform; stop process {pid} yourself"
    )))
}

/// A repo name from a directory name: one path component of letters, digits, `.`, `_`, `-`.
pub fn repo_name(dir: &Path) -> String {
    let raw = dir
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let cleaned: String = raw
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-') {
                c
            } else {
                '-'
            }
        })
        .collect();
    let trimmed = cleaned.trim_start_matches(['.', '-']).to_owned();
    if trimmed.is_empty() {
        "repo".to_owned()
    } else {
        trimmed
    }
}

/// `av init`: daemon up (when it is local), this git repository added, `agents` wired.
pub fn init(
    client: &Client,
    url: &str,
    token: Option<&str>,
    name: Option<&str>,
    agents: &[Agent],
) -> Result<()> {
    let root = git_root()?;
    if !healthy(client) {
        if local_port(url).is_none() {
            return Err(CliError::Unreachable {
                url: url.to_owned(),
                detail: "no answer on /healthz".into(),
            });
        }
        up(client, url)?;
    }
    let name = name.map(str::to_owned).unwrap_or_else(|| repo_name(&root));
    let exists = client
        .get(&format!("/v1/repos/{name}/log"), &[("limit", "1")])
        .is_ok();
    if exists {
        println!("repo {name} already added");
    } else {
        client.post(
            "/v1/repos",
            &json!({"name": name, "git_source": root.to_string_lossy(), "wait": false}),
        )?;
        let r = crate::wait_import(client, &name)?;
        println!(
            "repo {name} added from {} (main {}, {} files indexed)",
            root.display(),
            r["main_change"]
                .as_str()
                .unwrap_or_default()
                .chars()
                .take(12)
                .collect::<String>(),
            r["indexed_files"]
        );
    }
    let mcp_url = format!("{}/mcp", url.trim_end_matches('/'));
    let codex_home = env_path("CODEX_HOME");
    for &agent in agents {
        let path = agent.config_path(&home(), codex_home.clone());
        match agent {
            Agent::Claude => {
                install_claude_hooks(&path)?;
                println!("Claude Code hooks: {}", path.display());
                claude_mcp(&mcp_url, token);
            }
            Agent::Codex => agents::install_codex(&path, &mcp_url, token.is_some())?,
            Agent::Grok => agents::install_grok(&path, &mcp_url, token.is_some())?,
            Agent::Gemini => agents::install_gemini(&path, &mcp_url, token.is_some())?,
        }
        if agent != Agent::Claude {
            println!(
                "{} MCP server `{}`: {}{}",
                agent.name(),
                agents::SERVER,
                path.display(),
                if token.is_some() {
                    " (reads the token from AV_TOKEN when it starts)"
                } else {
                    ""
                }
            );
        }
    }
    println!(
        "Other MCP clients: streamable HTTP at {mcp_url}\n\
         \n\
         Next: start a task and work in its workspace (or let the agent call the `start_task` tool):\n  \
         av task start --repo {name} --title \"Add retry\" --goal \"retry failed calls with backoff\"\n  \
         cd <the workspace it prints> && claude   (or codex, grok, gemini)\n\
         Dashboard: {}/ui",
        url.trim_end_matches('/')
    );
    Ok(())
}

fn git_root() -> Result<PathBuf> {
    let out = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .map_err(|e| local(format!("cannot run git: {e}")))?;
    if !out.status.success() {
        return Err(local("av init runs inside a git repository"));
    }
    Ok(PathBuf::from(
        String::from_utf8_lossy(&out.stdout).trim().to_owned(),
    ))
}

/// Our hooks (`av hook …`), or ones `integrations/claude-code/install.sh` installed.
fn ours(entry: &Value) -> bool {
    entry["hooks"].as_array().is_some_and(|hooks| {
        hooks.iter().any(|h| {
            h["command"]
                .as_str()
                .is_some_and(|c| c.contains("av hook ") || c.contains("/av-hooks/"))
        })
    })
}

/// Hook commands never fail the agent and do nothing where `av` is not installed.
fn hook(sub: &str) -> Value {
    json!({ "type": "command", "command": format!("command -v av >/dev/null 2>&1 && av hook {sub}; exit 0") })
}

/// Merges the `SessionStart`, `PreToolUse`, `PostToolUse` and `Stop` hooks into Claude Code's settings,
/// replacing earlier Scholion hooks; the previous file is kept as `settings.json.bak`.
pub fn install_claude_hooks(path: &Path) -> Result<()> {
    let mut settings: Value = match std::fs::read_to_string(path) {
        Ok(text) => serde_json::from_str(&text).map_err(|e| {
            local(format!(
                "{} is not valid JSON ({e}); nothing changed",
                path.display()
            ))
        })?,
        Err(_) => json!({}),
    };
    if !settings.is_object() {
        return Err(local(format!(
            "{} is not a JSON object; nothing changed",
            path.display()
        )));
    }
    let add = [
        ("SessionStart", json!({ "hooks": [hook("session-start")] })),
        (
            "PostToolUse",
            json!({ "matcher": "Edit|Write|MultiEdit", "hooks": [hook("post-edit")] }),
        ),
        (
            "PreToolUse",
            json!({ "matcher": "Read|Edit|MultiEdit", "hooks": [hook("pre-read")] }),
        ),
        ("Stop", json!({ "hooks": [hook("stop")] })),
    ];
    let hooks = settings
        .as_object_mut()
        .expect("checked above")
        .entry("hooks")
        .or_insert_with(|| json!({}));
    if !hooks.is_object() {
        *hooks = json!({});
    }
    for (event, entry) in add {
        let list = hooks
            .as_object_mut()
            .expect("an object")
            .entry(event)
            .or_insert_with(|| json!([]));
        let mut kept: Vec<Value> = list
            .as_array()
            .map(|a| a.iter().filter(|e| !ours(e)).cloned().collect())
            .unwrap_or_default();
        kept.push(entry);
        *list = Value::Array(kept);
    }
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)
            .map_err(|e| local(format!("cannot create {}: {e}", dir.display())))?;
    }
    if path.exists() {
        std::fs::copy(path, path.with_extension("json.bak"))
            .map_err(|e| local(format!("cannot back up {}: {e}", path.display())))?;
    }
    let text = serde_json::to_string_pretty(&settings).expect("serializable") + "\n";
    std::fs::write(path, text).map_err(|e| local(format!("cannot write {}: {e}", path.display())))
}

/// Registers the MCP server with Claude Code (user scope) when its CLI is installed; otherwise
/// prints the command.
fn claude_mcp(mcp_url: &str, token: Option<&str>) {
    let mut args = vec![
        "mcp".to_owned(),
        "add".into(),
        "--transport".into(),
        "http".into(),
        "--scope".into(),
        "user".into(),
        "av".into(),
        mcp_url.to_owned(),
    ];
    if let Some(t) = token {
        args.push("--header".into());
        args.push(format!("Authorization: Bearer {t}"));
    }
    let quiet = |a: &[&str]| {
        Command::new("claude")
            .args(a)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
    };
    match quiet(&["mcp", "get", "av"]) {
        Err(_) => {
            let shown: Vec<String> = args
                .iter()
                .map(|a| {
                    if a.contains(' ') {
                        format!("\"{a}\"")
                    } else {
                        a.clone()
                    }
                })
                .collect();
            println!(
                "Claude Code CLI not found; to add the MCP server later: claude {}",
                shown.join(" ")
            );
        }
        Ok(s) if s.success() => println!("Claude Code MCP server `av`: already configured"),
        Ok(_) => {
            let refs: Vec<&str> = args.iter().map(String::as_str).collect();
            match quiet(&refs) {
                Ok(s) if s.success() => {
                    println!("Claude Code MCP server `av`: added (user scope, {mcp_url})")
                }
                _ => println!(
                    "Claude Code MCP server: `claude mcp add` failed; run it yourself: claude {}",
                    args.join(" ")
                ),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_port_only_for_this_machine() {
        assert_eq!(local_port("http://localhost:7420"), Some(7420));
        assert_eq!(local_port("http://127.0.0.1:9000/"), Some(9000));
        assert_eq!(local_port("http://localhost"), Some(80));
        assert_eq!(local_port("http://av.example.com:7420"), None);
    }

    #[test]
    fn repo_name_is_one_safe_path_component() {
        assert_eq!(repo_name(Path::new("/home/me/my-app")), "my-app");
        assert_eq!(repo_name(Path::new("/x/My App (2)")), "My-App--2-");
        assert_eq!(repo_name(Path::new("/x/.hidden")), "hidden");
        assert_eq!(repo_name(Path::new("/")), "repo");
    }

    #[test]
    fn hooks_merge_keeps_other_hooks_and_replaces_ours() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        std::fs::write(
            &path,
            r#"{"model":"x","hooks":{"Stop":[
                {"hooks":[{"type":"command","command":"say done"}]},
                {"hooks":[{"type":"command","command":"/home/me/.claude/av-hooks/stop.sh"}]}
            ]}}"#,
        )
        .unwrap();
        install_claude_hooks(&path).unwrap();
        install_claude_hooks(&path).unwrap();
        let v: Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(v["model"], "x");
        let stop = v["hooks"]["Stop"].as_array().unwrap();
        assert_eq!(stop.len(), 2, "{stop:?}");
        assert_eq!(stop[0]["hooks"][0]["command"], "say done");
        assert!(
            stop[1]["hooks"][0]["command"]
                .as_str()
                .unwrap()
                .contains("av hook stop")
        );
        let pre = &v["hooks"]["PreToolUse"][0];
        assert_eq!(pre["matcher"], "Read|Edit|MultiEdit");
        assert_eq!(v["hooks"]["SessionStart"].as_array().unwrap().len(), 1);
        assert_eq!(
            v["hooks"]["PostToolUse"][0]["matcher"],
            "Edit|Write|MultiEdit"
        );
        assert!(path.with_extension("json.bak").exists());
    }

    #[test]
    fn invalid_settings_are_left_alone() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        std::fs::write(&path, "{ not json").unwrap();
        assert!(install_claude_hooks(&path).is_err());
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "{ not json");
    }
}
