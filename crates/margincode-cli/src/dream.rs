//! `av dream`: the agent side of a second look at records (hivemind Phase 2). The daemon chooses
//! what a run may judge and checks every quote; this starts the agent that judges. It asks the
//! daemon for a run, writes what the run is handed (`bundle.json`) and the instructions
//! (`DREAM.md`) into `$AV_HOME/dreams/<id>/`, runs the given command there, and posts what the
//! agent wrote: `verdicts.json` of a review run, `reviews.json` of a verify run. What the
//! daemon refuses goes back to the agent once (`--fix-rounds`). The agent has no workspace and
//! this writes nothing but those files; the daemon still never starts a process.

use std::path::{Path, PathBuf};
#[cfg(unix)]
use std::process::Command;
use std::time::Duration;

use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::client::{CliError, Client, Result};
use margincode_agentrun::slots;

/// What the dreaming agent reads. Its hash is stored on the run, so a result can be traced to
/// the instructions that produced it.
pub const INSTRUCTIONS: &str = include_str!("../../../integrations/dream/DREAM.md");

pub fn instructions_sha() -> String {
    Sha256::digest(INSTRUCTIONS.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

pub struct Dream<'a> {
    pub agent: &'a str,
    /// Read only where the agent command runs (Unix); `av dream` refuses to start elsewhere.
    #[cfg_attr(not(unix), allow(dead_code))]
    pub url: &'a str,
    pub repo: &'a str,
    pub verify: bool,
    pub area: Option<&'a str>,
    pub limit: Option<u32>,
    /// How often the agent is run again on what the daemon refused.
    pub fix_rounds: u32,
    /// The agent command is killed after this long.
    #[cfg_attr(not(unix), allow(dead_code))]
    pub timeout: Duration,
    /// Agents allowed to run at once on this host; `None` = no limit.
    pub max_agents: Option<u32>,
    pub slots_dir: PathBuf,
    /// `$AV_HOME/dreams`.
    pub dreams_dir: PathBuf,
    pub cmd: &'a [String],
}

impl Dream<'_> {
    fn mode(&self) -> &'static str {
        if self.verify { "verify" } else { "review" }
    }

    /// The file the agent writes, and the key of its list.
    fn output(&self) -> (&'static str, &'static str) {
        if self.verify {
            ("reviews.json", "reviews")
        } else {
            ("verdicts.json", "verdicts")
        }
    }
}

fn local(e: impl std::fmt::Display) -> CliError {
    CliError::Local(e.to_string())
}

fn write(path: &Path, text: &str) -> Result<()> {
    std::fs::write(path, text).map_err(|e| local(format!("{}: {e}", path.display())))
}

fn short(id: &str) -> &str {
    &id[..id.len().min(8)]
}

/// The agent's list from its output file: `{"<key>": [...]}` or the bare list.
fn read_list(path: &Path, key: &str) -> Result<Vec<Value>> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| local(format!("the agent wrote no {}: {e}", path.display())))?;
    let v: Value =
        serde_json::from_str(&text).map_err(|e| local(format!("{}: {e}", path.display())))?;
    let list = if v.is_array() { &v } else { &v[key] };
    list.as_array()
        .cloned()
        .ok_or_else(|| local(format!("{}: expected {{\"{key}\": [...]}}", path.display())))
}

/// What the daemon refused, to hand back to the agent.
struct Refused {
    item: Value,
    why: String,
}

/// Runs the agent command in the run's directory. `Ok(false)`: it failed or ran out of time.
#[cfg(unix)]
fn run_agent(d: &Dream, dir: &Path, id: &str, round: u32) -> Result<bool> {
    let (program, args) = d.cmd.split_first().expect("checked by run");
    let mut cmd = Command::new(program);
    cmd.args(args)
        .current_dir(dir)
        .env("AV_AGENT", d.agent)
        .env("AV_URL", d.url)
        .env("AV_REPO", d.repo)
        .env("AV_DREAM", id)
        .env("AV_DREAM_DIR", dir)
        .env("AV_DREAM_MODE", d.mode())
        .env("AV_DREAM_ROUND", round.to_string())
        // a dream is on no task, whatever the shell that started it was on
        .env_remove("AV_TASK")
        .env_remove("AV_SESSION")
        .env_remove("AV_GENERATION");
    let shown = d.cmd.join(" ");
    match margincode_agentrun::process::run_with_timeout(cmd, d.timeout) {
        Ok(w) if w.exit == Some(0) => Ok(true),
        Ok(w) if w.killed => {
            eprintln!(
                "dream: {shown} was stopped after {}s (--timeout)",
                d.timeout.as_secs()
            );
            Ok(false)
        }
        Ok(w) => {
            eprintln!("dream: {shown} failed (exit {:?})", w.exit);
            Ok(false)
        }
        Err(e) => {
            eprintln!("dream: {shown} failed to start ({e})");
            Ok(false)
        }
    }
}

#[cfg(not(unix))]
fn run_agent(_: &Dream, _: &Path, _: &str, _: u32) -> Result<bool> {
    Err(CliError::Local(
        "av dream starts an agent process, which this build does only on Unix".into(),
    ))
}

/// Posts a review run's verdicts in one call; prints each answer and returns the refused.
fn post_verdicts(client: &Client, id: &str, items: &[Value]) -> Result<Vec<Refused>> {
    if items.is_empty() {
        return Ok(vec![]);
    }
    let r = client.post(
        &format!("/v1/dreams/{id}/verdicts"),
        &json!({ "verdicts": items }),
    )?;
    let mut refused = vec![];
    for (item, res) in items
        .iter()
        .zip(r["results"].as_array().into_iter().flatten())
    {
        let record = item["record"].as_str().unwrap_or("?");
        if res["accepted"] == true {
            println!(
                "{record} {} -> {} proposed",
                res["verdict"]["verdict"].as_str().unwrap_or_default(),
                res["verdict"]["handle"].as_str().unwrap_or_default()
            );
        } else {
            let why = res["why"].as_str().unwrap_or_default().to_owned();
            println!("{record} refused: {why}");
            refused.push(Refused {
                item: item.clone(),
                why,
            });
        }
    }
    Ok(refused)
}

/// Posts a verify run's reviews, one call each; prints each answer and returns the refused.
fn post_reviews(client: &Client, d: &Dream, id: &str, items: &[Value]) -> Result<Vec<Refused>> {
    let mut refused = vec![];
    for item in items {
        let mut refuse = |why: String| {
            println!("{} refused: {why}", item["verdict"].as_str().unwrap_or("?"));
            refused.push(Refused {
                item: item.clone(),
                why,
            });
        };
        let (Some(vd), Some(agree)) = (item["verdict"].as_str(), item["agree"].as_bool()) else {
            refuse("a review needs `verdict` (vd:…) and `agree` (true or false)".into());
            continue;
        };
        let body = json!({"reviewer": d.agent, "agree": agree, "dream": id,
                          "evidence": item["evidence"].as_array().cloned().unwrap_or_default()});
        // the id is the agent's text: it goes into the path escaped, and names nothing else
        let path = format!("/v1/verdicts/{}/review", crate::urlencode(vd.trim()));
        match client.post(&path, &body) {
            Ok(r) => println!(
                "{vd} {} -> {}",
                if agree { "agreed" } else { "disagreed" },
                r["verdict"]["state"].as_str().unwrap_or_default()
            ),
            // the daemon's refusal of one review is that review's answer, not the run's end
            Err(CliError::Api { message, .. }) => refuse(message),
            Err(e) => return Err(e),
        }
    }
    Ok(refused)
}

fn key_of(item: &Value, verify: bool) -> Option<&str> {
    item[if verify { "verdict" } else { "record" }].as_str()
}

/// Runs one dream. `Ok(false)` when the agent failed: the run is marked failed and nothing it
/// wrote in that round is posted.
pub fn run(client: &Client, d: &Dream) -> Result<bool> {
    if d.cmd.is_empty() {
        return Err(CliError::Local(
            "dream needs an agent command after --, e.g. `av dream -- claude -p \"Read DREAM.md \
             and do what it says\"`"
                .into(),
        ));
    }
    // the slot before the run: a run holds its area, and should not while it only waits
    let _slot = match d.max_agents {
        None => None,
        Some(n) => Some(
            slots::wait_for(&d.slots_dir, n, Duration::from_secs(5), || {
                println!("dream: all {n} agent slots on this host are in use; waiting")
            })
            .map_err(|e| local(format!("agent slots in {}: {e}", d.slots_dir.display())))?,
        ),
    };
    let started = client.post(
        &format!("/v1/repos/{}/dreams", d.repo),
        &json!({"agent": d.agent, "mode": d.mode(), "area": d.area, "limit": d.limit,
                "instructions_sha": instructions_sha()}),
    )?;
    let Some(id) = started["dream"]["id"].as_str() else {
        println!(
            "dream: nothing to {} in {}",
            if d.verify { "verify" } else { "review" },
            d.repo
        );
        return Ok(true);
    };
    let fail = |e: CliError| {
        let _ = client.post(&format!("/v1/dreams/{id}/finish"), &json!({"failed": true}));
        e
    };
    let dir = d.dreams_dir.join(id);
    std::fs::create_dir_all(&dir).map_err(|e| fail(local(format!("{}: {e}", dir.display()))))?;
    let pretty = serde_json::to_string_pretty(&started).map_err(local)?;
    write(&dir.join("bundle.json"), &pretty).map_err(fail)?;
    write(&dir.join("DREAM.md"), INSTRUCTIONS).map_err(fail)?;
    let (file, key) = d.output();
    let out = dir.join(file);
    let _ = std::fs::remove_file(&out);
    println!(
        "dream {} {} {}: {} to look at in {} -> running {}",
        short(id),
        d.mode(),
        d.repo,
        started["items"].as_array().map_or(0, Vec::len),
        dir.display(),
        d.cmd.join(" ")
    );

    let mut waiting: Option<Vec<Refused>> = None; // refused by the round before
    let mut refused_total = 0;
    for round in 1..=d.fix_rounds + 1 {
        if !run_agent(d, &dir, id, round).map_err(fail)? {
            client.post(&format!("/v1/dreams/{id}/finish"), &json!({"failed": true}))?;
            println!(
                "dream {}: failed; nothing of this round was posted",
                short(id)
            );
            return Ok(false);
        }
        let mut items = read_list(&out, key).map_err(fail)?;
        if let Some(before) = &waiting {
            // a fix round answers for what was refused, and nothing else
            items.retain(|i| {
                key_of(i, d.verify)
                    .is_some_and(|k| before.iter().any(|r| key_of(&r.item, d.verify) == Some(k)))
            });
        }
        let refused = if d.verify {
            post_reviews(client, d, id, &items)
        } else {
            post_verdicts(client, id, &items)
        }
        .map_err(fail)?;
        refused_total = refused.len();
        if refused.is_empty() || round > d.fix_rounds {
            break;
        }
        let list: Vec<Value> = refused
            .iter()
            .map(|r| json!({"item": r.item, "why": r.why}))
            .collect();
        let text = serde_json::to_string_pretty(&json!({ "refused": list })).map_err(local)?;
        write(&dir.join("refused.json"), &text).map_err(fail)?;
        let _ = std::fs::remove_file(&out);
        println!(
            "dream {}: {} refused; asking the agent to correct them",
            short(id),
            refused.len()
        );
        waiting = Some(refused);
    }

    let done = client.post(&format!("/v1/dreams/{id}/finish"), &json!({}))?;
    if d.verify {
        println!("dream {} done: {refused_total} refused", short(id));
    } else {
        let proposed: u64 = done["dream"]["counts"]
            .as_object()
            .map(|c| c.values().filter_map(Value::as_u64).sum())
            .unwrap_or(0);
        println!(
            "dream {} done: {proposed} proposed, {refused_total} refused",
            short(id)
        );
        if proposed > 0 {
            println!(
                "nothing changes until a second look agrees: `av dream --verify --repo {} -- \
                 <agent command>`, or `av verdict accept vd:…`; `av dream undo {id}` takes \
                 the run back",
                d.repo
            );
        }
    }
    Ok(true)
}

/// `vd:1a2b3c4d  proposed  disputed  rec:5e6f7a8b  by dreamer  the reason`.
pub fn verdict_line(v: &Value) -> String {
    let s = |k: &str| v[k].as_str().unwrap_or_default();
    let mut line = format!(
        "{}  {}  {}  {}  by {}",
        s("handle"),
        s("state"),
        s("verdict"),
        s("record"),
        s("author_agent")
    );
    if !s("reason").is_empty() {
        line.push_str(&format!("  {}", s("reason")));
    }
    line
}

/// One verdict in full, with the record it judges (`GET /v1/verdicts/{id}`).
pub fn verdict_text(r: &Value) -> String {
    let v = &r["verdict"];
    let rec = &r["record"];
    let s = |v: &Value| v.as_str().unwrap_or_default().to_owned();
    let mut out = format!("{}\n", verdict_line(v));
    out.push_str(&format!(
        "record {} {} by {}: {}\n",
        s(&rec["id"]),
        s(&rec["kind"]),
        s(&rec["author"]),
        s(&rec["summary"])
    ));
    let body = s(&rec["body"]);
    if !body.trim().is_empty() {
        for l in body.lines() {
            out.push_str(&format!("    {l}\n"));
        }
    }
    if let Some(c) = v["canonical"].as_str() {
        out.push_str(&format!("canonical: {c}\n"));
    }
    for e in v["evidence"].as_array().into_iter().flatten() {
        if let Some(path) = e["code"].as_str() {
            out.push_str(&format!("evidence: {path}: {}\n", s(&e["quote"])));
        } else if let Some(rec) = e["record"].as_str() {
            out.push_str(&format!("evidence: {rec}: {}\n", s(&e["quote"])));
        } else if !e["event"].is_null() {
            out.push_str(&format!("evidence: event {}\n", e["event"]));
        }
    }
    if let Some(d) = v["dream_id"].as_str() {
        out.push_str(&format!("dream: {d}\n"));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_instructions_hash_is_sha256_hex() {
        let sha = instructions_sha();
        assert_eq!(sha.len(), 64);
        assert!(sha.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn the_instructions_name_both_output_files() {
        for name in [
            "verdicts.json",
            "reviews.json",
            "refused.json",
            "bundle.json",
        ] {
            assert!(INSTRUCTIONS.contains(name), "{name}");
        }
    }

    #[test]
    fn the_agents_list_is_read_wrapped_or_bare() {
        let dir = std::env::temp_dir().join(format!("av-dream-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        let f = dir.join("verdicts.json");
        std::fs::write(&f, r#"{"verdicts": [{"record": "rec:1"}]}"#).unwrap();
        assert_eq!(read_list(&f, "verdicts").unwrap().len(), 1);
        std::fs::write(&f, r#"[{"record": "rec:1"}, {"record": "rec:2"}]"#).unwrap();
        assert_eq!(read_list(&f, "verdicts").unwrap().len(), 2);
        std::fs::write(&f, r#"{"other": 1}"#).unwrap();
        assert!(read_list(&f, "verdicts").is_err());
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
