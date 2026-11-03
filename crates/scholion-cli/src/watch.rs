//! `av watch`: start an agent CLI only when there is work for it (Plan 9). It asks the daemon
//! what `next` would give the agent (a dry run: nothing is taken) every `--every` seconds, and
//! when something fits, runs the given command with `AV_AGENT` and `AV_URL` set and waits for it.
//! The command is expected to take the work itself (`start_task` with no arguments, or
//! `av next`). This runs on the user's machine; the daemon still never starts a process.
//!
//! `--max-agents N` caps the agents running at once on this host, across every watch that shares
//! `$AV_HOME`: an agent runs only while its watch holds one of N slots
//! (`scholion_agentrun::slots`, shared with anything else that starts agents on the host).

use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

use serde_json::{Value, json};

use crate::client::{CliError, Client, Result};
use scholion_agentrun::slots;

/// The longest pause after failures (the command, or the daemon).
const MAX_BACKOFF: Duration = Duration::from_secs(300);

pub struct Watch<'a> {
    pub agent: &'a str,
    pub url: &'a str,
    pub repo: Option<&'a str>,
    pub every: Duration,
    pub max_runs: Option<u32>,
    pub once: bool,
    /// Agents allowed to run at once on this host; `None` = no limit.
    pub max_agents: Option<u32>,
    /// Where the slot files live (`$AV_HOME/agents`).
    pub slots_dir: PathBuf,
    pub cmd: &'a [String],
}

/// What a dry run of `next` says: `None` when nothing fits the agent.
fn peek(client: &Client, w: &Watch) -> Result<Option<Value>> {
    let r = client.post(
        "/v1/backlog/next",
        &json!({"agent": w.agent, "session": "av-watch", "repo": w.repo, "dry_run": true}),
    )?;
    Ok(Some(r["would"].clone()).filter(|v| !v.is_null()))
}

pub fn run(client: &Client, w: &Watch) -> Result<()> {
    let Some((program, args)) = w.cmd.split_first() else {
        return Err(CliError::Local("watch needs a command after --".into()));
    };
    let shown = w.cmd.join(" ");
    let mut runs = 0u32;
    let mut pause = w.every;
    let mut said_full = false;
    loop {
        // a slot first: while the host is full there is nothing to ask the daemon
        let slot = match w.max_agents {
            None => None,
            Some(n) => match slots::take(&w.slots_dir, n) {
                Ok(Some(slot)) => {
                    said_full = false;
                    Some(slot)
                }
                Ok(None) => {
                    let full = format!("watch: all {n} agent slots on this host are in use");
                    if w.once {
                        println!("{full}");
                        return Ok(());
                    }
                    if !said_full {
                        println!("{full}; waiting");
                        said_full = true;
                    }
                    std::thread::sleep(w.every);
                    continue;
                }
                Err(e) => {
                    return Err(CliError::Local(format!(
                        "agent slots in {}: {e}",
                        w.slots_dir.display()
                    )));
                }
            },
        };
        let would = match peek(client, w) {
            Ok(v) => v,
            Err(e) if w.once => return Err(e),
            Err(e) => {
                drop(slot);
                pause = (pause * 2).min(MAX_BACKOFF);
                eprintln!("watch: {e}; looking again in {}s", pause.as_secs());
                std::thread::sleep(pause);
                continue;
            }
        };
        let Some(work) = would else {
            drop(slot); // nothing to run: free the slot while waiting
            if w.once {
                println!("watch: nothing for {} now", w.agent);
                return Ok(());
            }
            std::thread::sleep(w.every);
            continue;
        };
        println!(
            "watch: {} {}/{} -> running {shown}",
            work["from"].as_str().unwrap_or_default(),
            work["repo"].as_str().unwrap_or_default(),
            work["title"].as_str().unwrap_or_default()
        );
        let status = Command::new(program)
            .args(args)
            .env("AV_AGENT", w.agent)
            .env("AV_URL", w.url)
            .status();
        drop(slot); // the agent is done: another watch may start one
        runs += 1;
        let ok = match &status {
            Ok(s) if s.success() => true,
            Ok(s) => {
                eprintln!("watch: {shown} failed ({s})");
                false
            }
            Err(e) => {
                eprintln!("watch: {shown} failed to start ({e})");
                false
            }
        };
        if w.once || w.max_runs.is_some_and(|m| runs >= m) {
            return Ok(());
        }
        // after a failure wait longer each time, so a broken command does not spin
        pause = if ok {
            w.every
        } else {
            (pause * 2).min(MAX_BACKOFF)
        };
        std::thread::sleep(pause);
    }
}
