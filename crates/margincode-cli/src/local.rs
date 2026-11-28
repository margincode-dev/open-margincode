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

pub fn av_home() -> PathBuf {
    margincode_agentrun::slots::av_home()
}

fn home() -> PathBuf {
    home_dir().unwrap_or_else(|| PathBuf::from("."))
}

/// The user's home: `$HOME`, else `%USERPROFILE%` (Windows sets no `HOME` outside Git Bash).
pub fn home_dir() -> Option<PathBuf> {
    margincode_agentrun::slots::home_dir()
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

fn local_port(url: &str) -> Option<u16> {
    let rest = url.split_once("://").map_or(url, |(_, r)| r);
    let authority = rest.split('/').next().unwrap_or_default();
    let (host, port) = match authority.rsplit_once(':') {
        Some((h, p)) => (h, p.parse().ok()?),
        None => (authority, 80),
    };
    matches!(host, "localhost" | "127.0.0.1" | "[::1]").then_some(port)
}

fn daemon_bin() -> Result<PathBuf> {
    if let Some(bin) = env_path("AV_DAEMON_BIN") {
        return Ok(bin);
    }
    let name = format!("margincode-daemon{}", std::env::consts::EXE_SUFFIX);
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
    if let Err(e) = rotate_log(&log_path, LOG_MAX_BYTES) {
        eprintln!("av: could not rotate {}: {e}", log_path.display());
    }
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

const LOG_MAX_BYTES: u64 = 50 * 1024 * 1024;

fn rotate_log(path: &Path, max: u64) -> std::io::Result<bool> {
    match std::fs::metadata(path) {
        Ok(m) if m.len() > max => {
            let mut old = path.as_os_str().to_owned();
            old.push(".1");
            let old = PathBuf::from(old);
            if old.exists() {
                std::fs::remove_file(&old)?;
            }
            std::fs::rename(path, &old)?;
            Ok(true)
        }
        Ok(_) => Ok(false),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e),
    }
}

fn log_tail(path: &Path, offset: u64) -> String {
    let text = std::fs::read(path).unwrap_or_default();
    let new = String::from_utf8_lossy(text.get(offset as usize..).unwrap_or_default()).into_owned();
    let lines: Vec<&str> = new.lines().collect();
    lines[lines.len().saturating_sub(20)..].join("\n")
}

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

/// The repo `av init` adds: its name and the path the daemon imports (defaults: from the git root).
pub struct Repo<'a> {
    pub name: Option<&'a str>,
    pub source: Option<&'a str>,
}

/// `mcp_json` is the file for [`Agent::McpJson`]; `agent_name` is written as `X-Av-Agent`.
pub fn init(
    client: &Client,
    url: &str,
    token: Option<&str>,
    Repo { name, source }: Repo,
    agents: &[Agent],
    mcp_json: Option<&Path>,
    agent_name: &str,
) -> Result<String> {
    let root = git_root()?;
    let source = source.map_or_else(|| root.to_string_lossy().into_owned(), str::to_owned);
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
        client
            .post(
                "/v1/repos",
                &json!({"name": name, "git_source": source, "wait": false}),
            )
            .map_err(|e| crate::with_msys_hint(e, &source))?;
        let r = crate::wait_import(client, &name)?;
        println!(
            "repo {name} added from {source} (main {}, {} files indexed)",
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
    let xdg_config_home = env_path("XDG_CONFIG_HOME");
    for &agent in agents {
        let path = match agent {
            Agent::McpJson => mcp_json
                .ok_or_else(|| local("--for mcp-json needs --path FILE"))?
                .to_path_buf(),
            other => other
                .config_path(&home(), codex_home.clone(), xdg_config_home.clone())
                .expect("every agent but mcp-json has a home config"),
        };
        match agent {
            Agent::Claude => {
                install_claude_hooks(&path)?;
                println!("Claude Code hooks: {}", path.display());
                // the default name is no one's: MCP then names the session after the client
                let named = (agent_name != DEFAULT_AGENT).then_some(agent_name);
                claude_mcp(&mcp_url, token, named);
            }
            Agent::Codex => agents::install_codex(&path, &mcp_url, token.is_some())?,
            Agent::Grok => {
                agents::install_grok(&path, &mcp_url, token.is_some())?;
                let home_dir = home();
                let hooks_path = home_dir.join(".grok/hooks/av.json");
                if install_grok_hooks(&home_dir, agents.contains(&Agent::Claude))? {
                    println!("Grok Build hooks: {}", hooks_path.display());
                } else {
                    println!(
                        "Grok Build hooks: skipped, already reachable from {}",
                        home_dir.join(".claude/settings.json").display()
                    );
                }
            }
            Agent::Gemini => agents::install_gemini(&path, &mcp_url, token.is_some())?,
            Agent::Cursor => agents::install_cursor(&path, &mcp_url, token.is_some())?,
            Agent::Antigravity => agents::install_antigravity(&path, &mcp_url, token)?,
            Agent::Opencode => {
                // first, so a config av cannot edit (the entry then added by hand) has the hooks
                let av_bin = std::env::current_exe()
                    .map_err(|e| local(format!("cannot find av's own path: {e}")))?;
                let config_dir = path.parent().expect("the config file is in a directory");
                let plugin = agents::install_opencode_plugin(config_dir, &av_bin)?;
                println!("OpenCode plugin: {}", plugin.display());
                agents::install_opencode(&path, &mcp_url, token)?;
            }
            Agent::McpJson => {
                agents::install_mcp_json(&path, &mcp_url, agent_name, token.is_some())?
            }
        }
        if agent != Agent::Claude {
            let note = match (agent, token.is_some()) {
                (_, false) => "",
                (Agent::Antigravity | Agent::Opencode, true) => {
                    " (the token is written into the file: run `av init` again when it changes)"
                }
                (_, true) => " (reads the token from AV_TOKEN when it starts)",
            };
            println!(
                "{} MCP server `{}`: {}{note}",
                agent.name(),
                agents::SERVER,
                path.display(),
            );
        }
        // OpenCode loads opencode.jsonc after opencode.json, and the later file's server wins
        let jsonc = path.with_extension("jsonc");
        if agent == Agent::Opencode && jsonc.exists() {
            println!(
                "OpenCode also reads {}: an mcp.servers.av entry there overrides this one",
                jsonc.display()
            );
        }
    }
    println!(
        "Other MCP clients: streamable HTTP at {mcp_url}, or\n  \
         av --agent NAME init --for mcp-json --path FILE\n\
         \n\
         Next: start a task and work in its workspace (or let the agent call the `start_task` tool):\n  \
         av task start --repo {name} --title \"Add retry\" --goal \"retry failed calls with backoff\"\n  \
         cd <the workspace it prints> && claude   (or codex, grok, gemini, cursor-agent, agy, opencode)\n\
         Dashboard: {}/ui",
        url.trim_end_matches('/')
    );
    Ok(name)
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

/// Our hook command (`av hook …`), or one `integrations/claude-code/install.sh` installed.
fn our_command(hook: &Value) -> bool {
    hook["command"]
        .as_str()
        .is_some_and(|c| c.contains("av hook ") || c.contains("/av-hooks/"))
}

fn ours(entry: &Value) -> bool {
    entry["hooks"]
        .as_array()
        .is_some_and(|hooks| hooks.iter().any(our_command))
}

/// Hook commands never fail the agent and do nothing where `av` is not installed.
fn hook(sub: &str) -> Value {
    json!({ "type": "command", "command": format!("command -v av >/dev/null 2>&1 && av hook {sub}; exit 0") })
}

/// The Stop hook waits for work (`AV_HOOK_WAIT_SECS`), so its timeout must exceed the wait: Claude
/// Code discards the output of a hook it cancels, and its default timeout is 600 s.
fn stop_hook() -> Value {
    let mut h = hook("stop");
    h["timeout"] = json!(crate::hook::stop_timeout_secs());
    h
}

/// Replaces earlier Margincode hooks; the previous file is kept as `settings.json.bak`.
pub fn install_claude_hooks(path: &Path) -> Result<()> {
    let mut settings = agents::read_json_object(path)?;
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
        ("Stop", json!({ "hooks": [stop_hook()] })),
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
    std::fs::write(path, text)
        .map_err(|e| local(format!("cannot write {}: {e}", path.display())))?;
    // Grok reads this file and ~/.grok/hooks/*.json. Drop the copy `av init --for grok` wrote
    // so the same hook does not fire twice.
    if let Some(home_dir) = claude_config_home(path) {
        strip_our_hooks_file(&home_dir.join(".grok/hooks/av.json"))?;
    }
    Ok(())
}

fn claude_config_home(path: &Path) -> Option<PathBuf> {
    let dir = path.parent()?;
    if dir.file_name().and_then(|s| s.to_str()) != Some(".claude") {
        return None;
    }
    dir.parent().map(PathBuf::from)
}

/// Grok reads that file, so a second copy in `~/.grok/hooks` would fire twice.
fn claude_settings_have_ours(path: &Path) -> bool {
    let Ok(text) = std::fs::read_to_string(path) else {
        return false;
    };
    let Ok(settings) = serde_json::from_str::<Value>(&text) else {
        return false;
    };
    settings
        .get("hooks")
        .and_then(Value::as_object)
        .is_some_and(|events| {
            events.values().any(|list| {
                list.as_array()
                    .is_some_and(|entries| entries.iter().any(ours))
            })
        })
}

/// Writes SessionStart (the pack) and Stop into `~/.grok/hooks/av.json`. Returns false, and
/// removes a previous copy of that file, when Claude's settings already load the av hooks or
/// this `av init` is about to install them (`claude_in_this_run`).
pub fn install_grok_hooks(home_dir: &Path, claude_in_this_run: bool) -> Result<bool> {
    let path = home_dir.join(".grok/hooks/av.json");
    let settings = home_dir.join(".claude/settings.json");
    if claude_in_this_run || claude_settings_have_ours(&settings) {
        strip_our_hooks_file(&path)?;
        return Ok(false);
    }
    let add = [
        ("SessionStart", json!({ "hooks": [hook("session-start")] })),
        ("Stop", json!({ "hooks": [stop_hook()] })),
    ];
    merge_hook_entries(&path, &add)?;
    Ok(true)
}

fn merge_hook_entries(path: &Path, add: &[(&str, Value)]) -> Result<()> {
    let mut settings = agents::read_json_object(path)?;
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
            .entry(*event)
            .or_insert_with(|| json!([]));
        let mut kept: Vec<Value> = list
            .as_array()
            .map(|a| a.iter().filter(|e| !ours(e)).cloned().collect())
            .unwrap_or_default();
        kept.push(entry.clone());
        *list = Value::Array(kept);
    }
    let text = serde_json::to_string_pretty(&settings).expect("serializable") + "\n";
    agents::write_with_backup(path, &text)
}

/// A missing or non-JSON file is left as it is.
fn strip_our_hooks_file(path: &Path) -> Result<()> {
    match hooks_without_ours(path) {
        Ok(undo) => agents::apply_undo(path, &undo),
        Err(_) => Ok(()),
    }
}

/// Without our hook commands: an entry, event list or `hooks` object left empty by that goes
/// too, and the file when nothing else is left. Another command in an entry of ours stays.
fn hooks_without_ours(path: &Path) -> Result<agents::Undo> {
    let mut settings = agents::read_json_object(path)?;
    let Some(hooks) = settings.get_mut("hooks").and_then(Value::as_object_mut) else {
        return Ok(agents::Undo::Unchanged);
    };
    let mut changed = false;
    hooks.retain(|_, list| {
        let Some(entries) = list.as_array_mut() else {
            return true;
        };
        let before = entries.len();
        entries.retain_mut(|entry| {
            let Some(commands) = entry.get_mut("hooks").and_then(Value::as_array_mut) else {
                return true;
            };
            let n = commands.len();
            commands.retain(|h| !our_command(h));
            if commands.len() == n {
                return true;
            }
            changed = true;
            !commands.is_empty()
        });
        before == entries.len() || !entries.is_empty()
    });
    if !changed {
        return Ok(agents::Undo::Unchanged);
    }
    if hooks.is_empty() {
        settings
            .as_object_mut()
            .expect("an object")
            .retain(|k, _| k != "hooks");
    }
    Ok(agents::json_undo(&settings))
}

/// The scripts `integrations/claude-code/install.sh` copies into `~/.claude/av-hooks/`.
const INSTALL_SH_HOOKS: [&str; 4] = ["session-start.sh", "pre-read.sh", "post-edit.sh", "stop.sh"];

/// One file `av uninstall` checks, and what it would do to it.
pub struct Step {
    pub path: PathBuf,
    pub what: &'static str,
    pub undo: Result<agents::Undo>,
}

/// What `av uninstall` would change for `agents`, under `home`; nothing is written. The order is
/// the order to apply it in (a directory after the files in it).
pub fn plan_uninstall(
    home: &Path,
    codex_home: Option<PathBuf>,
    xdg_config_home: Option<PathBuf>,
    agents: &[Agent],
    mcp_json: Option<&Path>,
) -> Vec<Step> {
    let mut steps = Vec::new();
    let server = "MCP server `av`";
    for &agent in agents {
        let path = match agent {
            Agent::McpJson => match mcp_json {
                Some(p) => p.to_path_buf(),
                None => continue,
            },
            other => other
                .config_path(home, codex_home.clone(), xdg_config_home.clone())
                .expect("every agent but mcp-json has a home config"),
        };
        match agent {
            Agent::Claude => {
                let undo = hooks_without_ours(&path);
                steps.push(Step {
                    path,
                    what: "av's hooks",
                    undo,
                });
                let dir = home.join(".claude/av-hooks");
                for name in INSTALL_SH_HOOKS {
                    let script = dir.join(name);
                    if script.is_file() {
                        steps.push(Step {
                            path: script,
                            what: "install.sh's hook",
                            undo: Ok(agents::Undo::Delete),
                        });
                    }
                }
                let only_ours = std::fs::read_dir(&dir).is_ok_and(|entries| {
                    entries.into_iter().all(|e| {
                        e.is_ok_and(|e| INSTALL_SH_HOOKS.iter().any(|n| e.file_name() == *n))
                    })
                });
                if only_ours {
                    steps.push(Step {
                        path: dir,
                        what: "install.sh's hook directory",
                        undo: Ok(agents::Undo::Delete),
                    });
                }
            }
            Agent::Codex | Agent::Grok => {
                let undo = agents::toml_without_server(&path);
                steps.push(Step {
                    path,
                    what: server,
                    undo,
                });
                if agent == Agent::Grok {
                    let path = home.join(".grok/hooks/av.json");
                    let undo = hooks_without_ours(&path);
                    steps.push(Step {
                        path,
                        what: "av's hooks",
                        undo,
                    });
                }
            }
            Agent::Gemini | Agent::Cursor | Agent::Antigravity | Agent::McpJson => {
                let undo = agents::json_without_server(&path, &["mcpServers"]);
                steps.push(Step {
                    path,
                    what: server,
                    undo,
                });
            }
            Agent::Opencode => {
                let dir = path.parent().expect("the config file is in a directory");
                let (plugin, undo) = agents::opencode_plugin_undo(dir);
                steps.push(Step {
                    path: plugin,
                    what: "av's OpenCode plugin",
                    undo: Ok(undo),
                });
                let undo = agents::json_without_server(&path, &["mcp", "servers"]);
                steps.push(Step {
                    path,
                    what: server,
                    undo,
                });
            }
        }
    }
    // an agent named twice, or a --path that is also a home config: plan each file once
    let mut seen = std::collections::HashSet::new();
    steps.retain(|s| seen.insert(s.path.clone()));
    steps
}

/// `av uninstall`: takes out what `av init` added for `agents`, entry by entry, and says what it
/// left in place. A file it cannot read is named and makes the command fail after the others.
pub fn uninstall(agents: &[Agent], mcp_json: Option<&Path>, dry_run: bool) -> Result<()> {
    let home = home();
    let steps = plan_uninstall(
        &home,
        env_path("CODEX_HOME"),
        env_path("XDG_CONFIG_HOME"),
        agents,
        mcp_json,
    );
    let mut changed = 0;
    let mut failed = 0;
    let mut backups = Vec::new();
    for step in &steps {
        let mut bak = step.path.as_os_str().to_owned();
        bak.push(".bak");
        let bak = PathBuf::from(bak);
        if bak.is_file() && !backups.contains(&bak) {
            backups.push(bak);
        }
        let undo = match &step.undo {
            Ok(agents::Undo::Unchanged) => continue,
            Ok(undo) => undo,
            Err(e) => {
                failed += 1;
                eprintln!(
                    "av: cannot check {}: {e}; remove its `av` entry by hand",
                    step.path.display()
                );
                continue;
            }
        };
        changed += 1;
        let (verb, path) = (step.what, step.path.display());
        match (undo, dry_run) {
            (agents::Undo::Delete, true) => println!("would delete {path} ({verb})"),
            (_, true) => println!("would remove {verb} from {path}"),
            (agents::Undo::Delete, false) => {
                agents::apply_undo(&step.path, undo)?;
                println!("deleted {path} ({verb})");
            }
            (_, false) => {
                agents::apply_undo(&step.path, undo)?;
                println!("removed {verb} from {path}");
            }
        }
    }
    if agents.contains(&Agent::Claude) {
        match claude_mcp_remove(dry_run) {
            Ok(true) => changed += 1,
            Ok(false) => {}
            Err(e) => {
                failed += 1;
                eprintln!("av: {e}");
            }
        }
    }
    if changed == 0 {
        println!("nothing to remove: no av entries in the agent configs checked");
    } else if dry_run {
        println!("dry run: nothing changed");
    }
    let ws = env_path("AV_WS_ROOT").unwrap_or_else(|| home.join("ws"));
    println!(
        "Left in place: av's data (records, repositories) in {}, task workspaces in {}{}.",
        av_home().display(),
        ws.display(),
        if backups.is_empty() {
            String::new()
        } else {
            let names: Vec<String> = backups.iter().map(|b| b.display().to_string()).collect();
            format!(", and the backups av init kept ({})", names.join(", "))
        }
    );
    if av_home().join("daemon.pid").exists() {
        println!("A daemon `av up` started may still be running: `av down` stops it.");
    }
    println!("To remove those and the binaries: {UNINSTALL_DOCS}");
    if failed > 0 {
        return Err(local(format!(
            "{failed} config(s) could not be checked; see above"
        )));
    }
    Ok(())
}

const UNINSTALL_DOCS: &str =
    "https://github.com/margincode-dev/open-margincode/blob/main/docs/install.md#uninstall";

/// Removes Claude Code's user-scope `av` MCP server through its CLI; true when there was one.
fn claude_mcp_remove(dry_run: bool) -> Result<bool> {
    let remove = ["mcp", "remove", "--scope", "user", "av"];
    match claude_quiet(&["mcp", "get", "av"]) {
        Err(_) => {
            println!(
                "Claude Code CLI not found; if it has an `av` MCP server: claude {}",
                remove.join(" ")
            );
            Ok(false)
        }
        Ok(s) if !s.success() => Ok(false),
        Ok(_) if dry_run => {
            println!("would run: claude {}", remove.join(" "));
            Ok(true)
        }
        Ok(_) => match claude_quiet(&remove) {
            Ok(s) if s.success() => {
                println!(
                    "removed Claude Code MCP server `av` (claude {})",
                    remove.join(" ")
                );
                Ok(true)
            }
            _ => Err(local(format!(
                "`claude {}` failed; Claude Code may hold `av` in another scope: claude mcp get av",
                remove.join(" ")
            ))),
        },
    }
}

fn claude_quiet(args: &[&str]) -> std::io::Result<std::process::ExitStatus> {
    Command::new("claude")
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
}

pub const DEFAULT_AGENT: &str = "agent";

/// `agent` is sent as `X-Av-Agent`, so MCP calls are made under the name `av next` and the hooks
/// use, not under Claude Code's client name.
fn claude_mcp(mcp_url: &str, token: Option<&str>, agent: Option<&str>) {
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
    if let Some(a) = agent {
        args.push("--header".into());
        args.push(format!("X-Av-Agent: {a}"));
    }
    match claude_quiet(&["mcp", "get", "av"]) {
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
        Ok(s) if s.success() => match agent {
            None => println!("Claude Code MCP server `av`: already configured"),
            Some(a) => println!(
                "Claude Code MCP server `av`: already configured, left as it is; for it to send \
                 X-Av-Agent: {a}, run `claude mcp remove --scope user av` and this again"
            ),
        },
        Ok(_) => {
            let refs: Vec<&str> = args.iter().map(String::as_str).collect();
            match claude_quiet(&refs) {
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
    fn a_daemon_log_over_the_limit_moves_to_dot_one() {
        let dir = tempfile::TempDir::new().unwrap();
        let log = dir.path().join("daemon.log");
        let old = dir.path().join("daemon.log.1");
        assert!(!rotate_log(&log, 10).unwrap());

        std::fs::write(&log, "small").unwrap();
        assert!(!rotate_log(&log, 10).unwrap());
        assert_eq!(std::fs::read_to_string(&log).unwrap(), "small");

        std::fs::write(&old, "older").unwrap();
        std::fs::write(&log, "more than ten bytes").unwrap();
        assert!(rotate_log(&log, 10).unwrap());
        assert!(!log.exists());
        assert_eq!(
            std::fs::read_to_string(&old).unwrap(),
            "more than ten bytes"
        );
    }

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
        let timeout = stop[1]["hooks"][0]["timeout"].as_u64().unwrap();
        assert_eq!(timeout, crate::hook::stop_timeout_secs());
        assert_eq!(crate::hook::stop_timeout_for(600), 660);
        assert!(timeout > 60);
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

    #[test]
    fn empty_hook_files_are_read_as_empty_objects() {
        let claude = tempfile::tempdir().unwrap();
        let settings = claude.path().join(".claude/settings.json");
        std::fs::create_dir_all(settings.parent().unwrap()).unwrap();
        std::fs::write(&settings, "").unwrap();
        install_claude_hooks(&settings).unwrap();
        let v: Value = serde_json::from_str(&std::fs::read_to_string(&settings).unwrap()).unwrap();
        assert!(v["hooks"]["SessionStart"].is_array(), "{v}");

        let grok = tempfile::tempdir().unwrap();
        let hooks = grok.path().join(".grok/hooks/av.json");
        std::fs::create_dir_all(hooks.parent().unwrap()).unwrap();
        std::fs::write(&hooks, " \n").unwrap();
        assert!(install_grok_hooks(grok.path(), false).unwrap());
        assert!(grok_hooks(grok.path())["hooks"]["Stop"].is_array());
    }

    fn grok_hooks(home: &Path) -> Value {
        let path = home.join(".grok/hooks/av.json");
        serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap()
    }

    #[test]
    fn grok_hooks_file_is_session_start_and_stop_and_a_second_write_does_not_duplicate() {
        let dir = tempfile::tempdir().unwrap();
        assert!(install_grok_hooks(dir.path(), false).unwrap());
        assert!(install_grok_hooks(dir.path(), false).unwrap());
        let v = grok_hooks(dir.path());
        let start = v["hooks"]["SessionStart"][0]["hooks"][0]["command"]
            .as_str()
            .unwrap();
        assert!(start.contains("av hook session-start"), "{start}");
        let stop = &v["hooks"]["Stop"][0]["hooks"][0];
        assert!(
            stop["command"].as_str().unwrap().contains("av hook stop"),
            "{stop}"
        );
        assert_eq!(stop["timeout"], json!(crate::hook::stop_timeout_secs()));
        assert_eq!(crate::hook::stop_timeout_for(600), 660);
        assert!(v["hooks"].get("PreToolUse").is_none(), "{v}");
        assert_eq!(v["hooks"]["Stop"].as_array().unwrap().len(), 1);
        let path = dir.path().join(".grok/hooks/av.json");
        let mut edited = v.clone();
        edited["hooks"]["Notification"] =
            json!([{ "hooks": [{ "type": "command", "command": "say hi" }] }]);
        std::fs::write(&path, serde_json::to_string(&edited).unwrap()).unwrap();
        assert!(install_grok_hooks(dir.path(), false).unwrap());
        let v = grok_hooks(dir.path());
        assert_eq!(
            v["hooks"]["Notification"][0]["hooks"][0]["command"],
            "say hi"
        );
        assert_eq!(v["hooks"]["Stop"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn grok_hooks_are_left_out_when_claude_settings_already_load_them() {
        let dir = tempfile::tempdir().unwrap();
        assert!(install_grok_hooks(dir.path(), false).unwrap());
        let settings = dir.path().join(".claude/settings.json");
        std::fs::create_dir_all(settings.parent().unwrap()).unwrap();
        std::fs::write(
            &settings,
            r#"{"model":"x","hooks":{"Stop":[{"hooks":[{"type":"command","command":"av hook stop"}]}]}}"#,
        )
        .unwrap();
        assert!(!install_grok_hooks(dir.path(), false).unwrap());
        assert!(
            !dir.path().join(".grok/hooks/av.json").exists(),
            "a second copy would fire twice"
        );
        assert!(
            std::fs::read_to_string(&settings)
                .unwrap()
                .contains("\"model\"")
        );
        let bare = tempfile::tempdir().unwrap();
        assert!(!install_grok_hooks(bare.path(), true).unwrap());
        assert!(!bare.path().join(".grok/hooks/av.json").exists());
    }

    const URL: &str = "http://127.0.0.1:7420/mcp";

    const ALL: [Agent; 8] = [
        Agent::Claude,
        Agent::Codex,
        Agent::Grok,
        Agent::Gemini,
        Agent::Cursor,
        Agent::Antigravity,
        Agent::Opencode,
        Agent::McpJson,
    ];

    fn mcp_json(home: &Path) -> PathBuf {
        home.join("clients/mcp.json")
    }

    fn config(home: &Path, agent: Agent) -> PathBuf {
        agent
            .config_path(home, None, None)
            .unwrap_or_else(|| mcp_json(home))
    }

    /// What `av init --for agent` writes, with a token.
    fn init_files(home: &Path, agent: Agent) {
        let path = config(home, agent);
        match agent {
            Agent::Claude => install_claude_hooks(&path).unwrap(),
            Agent::Codex => agents::install_codex(&path, URL, true).unwrap(),
            Agent::Grok => {
                agents::install_grok(&path, URL, true).unwrap();
                // false, and no hooks file, when Claude's settings already load av's hooks
                install_grok_hooks(home, false).unwrap();
            }
            Agent::Gemini => agents::install_gemini(&path, URL, true).unwrap(),
            Agent::Cursor => agents::install_cursor(&path, URL, true).unwrap(),
            Agent::Antigravity => agents::install_antigravity(&path, URL, Some("t0k3n")).unwrap(),
            Agent::Opencode => {
                agents::install_opencode_plugin(path.parent().unwrap(), Path::new("/opt/av"))
                    .unwrap();
                agents::install_opencode(&path, URL, Some("t0k3n")).unwrap();
            }
            Agent::McpJson => agents::install_mcp_json(&path, URL, "muse", true).unwrap(),
        }
    }

    /// The files of `av uninstall` changed (would change, with `dry_run`).
    fn uninstall_in(home: &Path, agents: &[Agent], dry_run: bool) -> usize {
        let steps = plan_uninstall(home, None, None, agents, Some(&mcp_json(home)));
        let mut changed = 0;
        for step in steps {
            let undo = step.undo.unwrap();
            if undo != agents::Undo::Unchanged {
                changed += 1;
                if !dry_run {
                    agents::apply_undo(&step.path, &undo).unwrap();
                }
            }
        }
        changed
    }

    fn snapshot(dir: &Path) -> std::collections::BTreeMap<PathBuf, Vec<u8>> {
        let mut files = std::collections::BTreeMap::new();
        let mut todo = vec![dir.to_path_buf()];
        while let Some(d) = todo.pop() {
            for e in std::fs::read_dir(&d).unwrap() {
                let p = e.unwrap().path();
                if p.is_dir() {
                    todo.push(p);
                } else {
                    files.insert(p.clone(), std::fs::read(&p).unwrap());
                }
            }
        }
        files
    }

    fn write(path: &Path, text: &str) {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }

    /// The person's own files for `agent`, before `av init`.
    fn seed(home: &Path, agent: Agent) -> Vec<PathBuf> {
        let path = config(home, agent);
        match agent {
            Agent::Claude => write(
                &path,
                r#"{"model":"x","hooks":{"Stop":[{"hooks":[{"type":"command","command":"say done"}]}],"PostToolUse":[{"matcher":"Bash","hooks":[{"type":"command","command":"echo mine"}]}]}}"#,
            ),
            Agent::Codex | Agent::Grok => write(
                &path,
                "model = \"gpt-5\" # mine\n\n[mcp_servers.other]\ncommand = \"x\"\n",
            ),
            Agent::Opencode => write(
                &path,
                r#"{"model":"x/y","mcp":{"servers":{"other":{"type":"local","command":["x"]}}}}"#,
            ),
            _ => write(
                &path,
                r#"{"theme":"dark","mcpServers":{"other":{"command":"x"}}}"#,
            ),
        }
        let mut seeded = vec![path];
        if agent == Agent::Grok {
            let hooks = home.join(".grok/hooks/av.json");
            write(
                &hooks,
                r#"{"hooks":{"Notification":[{"hooks":[{"type":"command","command":"say hi"}]}]}}"#,
            );
            seeded.push(hooks);
        }
        if agent == Agent::Opencode {
            let mine = home.join(".config/opencode/plugins/mine.js");
            write(&mine, "export default {}\n");
            seeded.push(mine);
        }
        seeded
    }

    /// A new top-level setting, as the person adds it after `av init`; a plugin is left alone.
    fn with_later_edit(path: &Path, text: &str) -> String {
        match path.extension().and_then(|e| e.to_str()) {
            Some("toml") => format!("later = true\n{text}"),
            Some("json") => {
                let mut v: Value = serde_json::from_str(text).unwrap();
                v["later"] = json!(true);
                v.to_string()
            }
            _ => text.to_owned(),
        }
    }

    fn same_config(path: &Path, expected: &str) {
        let now = std::fs::read_to_string(path).unwrap();
        if path.extension().is_some_and(|e| e == "json") {
            let a: Value = serde_json::from_str(&now).unwrap();
            let b: Value = serde_json::from_str(expected).unwrap();
            assert_eq!(a, b, "{}", path.display());
        } else {
            assert_eq!(now, expected, "{}", path.display());
        }
    }

    #[test]
    fn uninstall_takes_out_what_init_added_and_keeps_the_persons_entries_and_later_edits() {
        for agent in ALL {
            let home = tempfile::tempdir().unwrap();
            let seeded = seed(home.path(), agent);
            let originals: Vec<String> = seeded
                .iter()
                .map(|p| std::fs::read_to_string(p).unwrap())
                .collect();
            init_files(home.path(), agent);
            for p in &seeded {
                let now = std::fs::read_to_string(p).unwrap();
                std::fs::write(p, with_later_edit(p, &now)).unwrap();
            }
            assert!(uninstall_in(home.path(), &ALL, false) > 0, "{agent:?}");
            for (p, original) in seeded.iter().zip(&originals) {
                same_config(p, &with_later_edit(p, original));
                let text = std::fs::read_to_string(p).unwrap();
                assert!(!text.contains("X-Av-Agent"), "{agent:?} {text}");
                assert!(!text.contains("av hook"), "{agent:?} {text}");
            }
            let left: std::collections::BTreeSet<PathBuf> = snapshot(home.path())
                .into_keys()
                .filter(|p| !p.to_string_lossy().ends_with(".bak"))
                .collect();
            let seeded_set: std::collections::BTreeSet<PathBuf> = seeded.iter().cloned().collect();
            assert_eq!(left, seeded_set, "{agent:?}");
            let after = snapshot(home.path());
            assert_eq!(
                uninstall_in(home.path(), &ALL, false),
                0,
                "{agent:?}: a second run"
            );
            assert_eq!(snapshot(home.path()), after, "{agent:?}: a second run");
        }
    }

    #[test]
    fn uninstall_deletes_a_config_init_made_from_nothing_and_leaves_its_backups() {
        for agent in ALL {
            let home = tempfile::tempdir().unwrap();
            init_files(home.path(), agent);
            init_files(home.path(), agent);
            let baks: Vec<(PathBuf, Vec<u8>)> = snapshot(home.path())
                .into_iter()
                .filter(|(p, _)| p.to_string_lossy().ends_with(".bak"))
                .collect();
            assert!(!baks.is_empty(), "{agent:?}");
            assert!(uninstall_in(home.path(), &[agent], false) > 0, "{agent:?}");
            let left: Vec<(PathBuf, Vec<u8>)> = snapshot(home.path()).into_iter().collect();
            assert_eq!(left, baks, "{agent:?}: only the backups init kept");
            assert_eq!(uninstall_in(home.path(), &ALL, false), 0, "{agent:?}");
        }
    }

    #[test]
    fn uninstall_dry_run_writes_nothing() {
        let home = tempfile::tempdir().unwrap();
        for agent in ALL {
            seed(home.path(), agent);
            init_files(home.path(), agent);
        }
        let before = snapshot(home.path());
        assert!(uninstall_in(home.path(), &ALL, true) >= ALL.len());
        assert_eq!(snapshot(home.path()), before);
    }

    #[test]
    fn uninstall_for_one_agent_leaves_the_others() {
        let home = tempfile::tempdir().unwrap();
        init_files(home.path(), Agent::Codex);
        init_files(home.path(), Agent::Gemini);
        let twice = [Agent::Codex, Agent::Gemini, Agent::Codex];
        let steps = plan_uninstall(home.path(), None, None, &twice, None);
        assert_eq!(steps.len(), 2, "each file once");
        assert_eq!(uninstall_in(home.path(), &[Agent::Codex], false), 1);
        assert!(!config(home.path(), Agent::Codex).exists());
        let gemini = std::fs::read_to_string(config(home.path(), Agent::Gemini)).unwrap();
        assert!(gemini.contains("X-Av-Agent"), "{gemini}");
    }

    #[test]
    fn uninstall_keeps_a_command_the_person_added_to_an_av_hook_entry() {
        let home = tempfile::tempdir().unwrap();
        let path = config(home.path(), Agent::Claude);
        install_claude_hooks(&path).unwrap();
        let mut v: Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        v["hooks"]["Stop"][0]["hooks"]
            .as_array_mut()
            .unwrap()
            .push(json!({ "type": "command", "command": "say bye" }));
        std::fs::write(&path, v.to_string()).unwrap();
        assert_eq!(uninstall_in(home.path(), &[Agent::Claude], false), 1);
        let v: Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(
            v,
            json!({ "hooks": { "Stop": [{ "hooks": [{ "type": "command", "command": "say bye" }] }] } })
        );
    }

    #[test]
    fn uninstall_takes_out_install_sh_hooks_and_their_scripts() {
        let home = tempfile::tempdir().unwrap();
        let dir = home.path().join(".claude/av-hooks");
        for name in INSTALL_SH_HOOKS {
            write(&dir.join(name), "#!/bin/sh\n");
        }
        let settings = config(home.path(), Agent::Claude);
        let command = dir.join("stop.sh").display().to_string();
        write(
            &settings,
            &json!({ "model": "x", "hooks": { "Stop": [{ "hooks": [{ "type": "command", "command": command }] }] } })
                .to_string(),
        );
        assert_eq!(uninstall_in(home.path(), &[Agent::Claude], false), 6);
        assert!(!dir.exists());
        same_config(&settings, r#"{"model":"x"}"#);

        write(&dir.join("stop.sh"), "#!/bin/sh\n");
        write(&dir.join("mine.sh"), "#!/bin/sh\n");
        assert_eq!(uninstall_in(home.path(), &[Agent::Claude], false), 1);
        assert!(dir.join("mine.sh").is_file());
        assert!(!dir.join("stop.sh").exists());
    }

    #[test]
    fn uninstall_leaves_an_opencode_plugin_that_is_not_avs() {
        let home = tempfile::tempdir().unwrap();
        let plugin = home.path().join(".config/opencode/plugins/av.js");
        write(&plugin, "export default {}\n");
        assert_eq!(uninstall_in(home.path(), &ALL, false), 0);
        assert!(plugin.is_file());
    }

    #[test]
    fn uninstall_names_a_config_it_cannot_read_and_still_plans_the_others() {
        let home = tempfile::tempdir().unwrap();
        init_files(home.path(), Agent::Gemini);
        let codex = config(home.path(), Agent::Codex);
        write(&codex, "[[[ nope");
        let steps = plan_uninstall(
            home.path(),
            None,
            None,
            &[Agent::Codex, Agent::Gemini],
            None,
        );
        assert!(steps[0].undo.is_err());
        assert_eq!(steps[0].path, codex);
        assert_eq!(steps[1].undo.as_ref().unwrap(), &agents::Undo::Delete);
        assert_eq!(std::fs::read_to_string(&codex).unwrap(), "[[[ nope");
    }

    #[test]
    fn installing_claude_hooks_removes_the_grok_copy() {
        let dir = tempfile::tempdir().unwrap();
        assert!(install_grok_hooks(dir.path(), false).unwrap());
        let settings = dir.path().join(".claude/settings.json");
        install_claude_hooks(&settings).unwrap();
        assert!(!dir.path().join(".grok/hooks/av.json").exists());
        let v: Value = serde_json::from_str(&std::fs::read_to_string(&settings).unwrap()).unwrap();
        assert!(
            v["hooks"]["Stop"][0]["hooks"][0]["command"]
                .as_str()
                .unwrap()
                .contains("av hook stop")
        );
        assert!(
            v["hooks"]["SessionStart"][0]["hooks"][0]["command"]
                .as_str()
                .unwrap()
                .contains("av hook session-start")
        );
    }
}
