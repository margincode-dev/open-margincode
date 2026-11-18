//! `av dream`: the agent side of a second look at records (hivemind Phase 2). The daemon chooses
//! what a run may judge and checks every quote; this starts the agent that judges. It asks the
//! daemon for a run, writes what the run is handed (`bundle.json`) and the instructions
//! (`DREAM.md`) into `$AV_HOME/dreams/<id>/`, runs the given command there, and posts what the
//! agent wrote: `verdicts.json` of a review run, `reviews.json` of a verify run. What the
//! daemon refuses goes back to the agent once (`--fix-rounds`). The agent has no workspace and
//! this writes nothing but those files; the daemon still never starts a process.
//!
//! `--also-read` directories are named to the agent (`also-read.txt`, `AV_DREAM_ALSO_READ` and a
//! section appended to that run's `DREAM.md`); nothing under them is read or sent here.
//! `--every` is [`repeat`] around [`run`].

use std::path::{Path, PathBuf};
#[cfg(unix)]
use std::process::Command;
use std::time::Duration;

use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::client::{CliError, Client, Result};
use margincode_agentrun::slots;

/// What the dreaming agent reads; the hash of what a run was handed (this, plus `PROMOTE` with
/// `--promote` and `ALSO_READ` with `--also-read`) is stored on the run, so a result traces to
/// its instructions.
pub const INSTRUCTIONS: &str = include_str!("../../../integrations/dream/DREAM.md");

/// Appended to `DREAM.md` only in a run given `--also-read`, so every other run is handed
/// `INSTRUCTIONS` as it is.
const ALSO_READ: &str = "
## More to read

The person who started this run lets you read the directories listed in `also-read.txt`, in
this directory, one absolute path per line (`$AV_DREAM_ALSO_READ` holds them too), for example
an agent CLI's own transcripts. Read them for context when you judge an item: why a record was
written, what happened after it. They are not evidence. The daemon cannot see them, so a quote
from them is refused: quote only what \"Evidence\" above allows. Change nothing in them.
";

/// Appended to `DREAM.md` only in a review run given `--promote`.
const PROMOTE: &str = r#"
## `promote`: a decision as a convention

This run may also propose conventions. A `decision` that states a rule for a set of paths (how
all code under them is written), not a one-off choice, may get `promote` instead of another
verdict: a proposed repository convention, with `text` (the rule, in a line or two, at most 400
characters), `scope_glob` (the paths it is for) and a `record` quote of the decision itself.
It goes in `verdicts.json` with the others:

```json
{"record": "rec:1a2b3c4d", "verdict": "promote", "text": "handlers never log a request body",
 "scope_glob": "app/api/**",
 "evidence": [{"record": "rec:1a2b3c4d", "quote": "we never log request bodies"}]}
```

A convention is shown to every agent working on those paths and is never dropped, so a person
decides whether it becomes one; promote only a decision you checked holds.
"#;

/// What the agent reads as `DREAM.md`: `INSTRUCTIONS` as it is, unless the run asked for more.
pub fn instructions(promote: bool, also_read: bool) -> String {
    let mut text = INSTRUCTIONS.to_owned();
    if promote {
        text.push_str(PROMOTE);
    }
    if also_read {
        text.push_str(ALSO_READ);
    }
    text
}

pub fn sha_hex(text: &str) -> String {
    Sha256::digest(text.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// `--budget-usd`: a positive amount that does not print as `0.00` (the flag has two decimals).
pub fn parse_budget(s: &str) -> std::result::Result<f64, String> {
    match s.trim().parse::<f64>() {
        Ok(x) if x.is_finite() && x >= 0.01 => Ok(x),
        _ => Err("a number of US dollars, at least 0.01".into()),
    }
}

/// `--every`: a whole number with `s`, `m`, `h` or `d` (seconds without one), never zero.
pub fn parse_every(s: &str) -> std::result::Result<Duration, String> {
    let s = s.trim();
    let (num, unit) = match s.char_indices().last() {
        Some((i, c)) if c.is_ascii_alphabetic() => (&s[..i], c),
        _ => (s, 's'),
    };
    let per = match unit {
        's' => 1,
        'm' => 60,
        'h' => 3600,
        'd' => 86400,
        _ => 0,
    };
    let n: u64 = num.parse().unwrap_or(0);
    match n.checked_mul(per) {
        Some(secs) if secs > 0 && num.bytes().all(|b| b.is_ascii_digit()) => {
            Ok(Duration::from_secs(secs))
        }
        _ => Err("a time above zero, such as 30s, 90m, 24h or 1d".into()),
    }
}

/// A wait as `1d2h3m4s`, whole seconds rounded up, zero parts left out.
pub fn span(d: Duration) -> String {
    let mut left = d.as_secs() + u64::from(d.subsec_nanos() > 0);
    if left == 0 {
        return "0s".into();
    }
    let mut out = String::new();
    for (unit, secs) in [("d", 86400), ("h", 3600), ("m", 60), ("s", 1)] {
        if left >= secs {
            out.push_str(&format!("{}{unit}", left / secs));
            left %= secs;
        }
    }
    out
}

/// `--also-read`: each one must be a directory; answered absolute, symlinks resolved. Nothing
/// in them is read.
pub fn also_read_dirs(given: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut dirs = vec![];
    for p in given {
        let bad = |why: String| local(format!("--also-read {}: {why}", p.display()));
        let meta = std::fs::metadata(p).map_err(|e| bad(e.to_string()))?;
        if !meta.is_dir() {
            return Err(bad("not a directory".into()));
        }
        let abs = std::fs::canonicalize(p).map_err(|e| bad(e.to_string()))?;
        if abs.to_string_lossy().contains('\n') {
            return Err(bad("a path with a line break cannot be listed".into()));
        }
        dirs.push(abs);
    }
    // the same check `run_agent` makes when it sets AV_DREAM_ALSO_READ
    std::env::join_paths(&dirs).map_err(|e| local(format!("--also-read: {e}")))?;
    Ok(dirs)
}

/// The agent command is required; checked once, before a run or a loop of them.
pub fn check_command(cmd: &[String]) -> Result<()> {
    if cmd.is_empty() {
        return Err(CliError::Local(
            "dream needs an agent command after --, e.g. `av dream -- claude -p \"Read DREAM.md \
             and do what it says\"`"
                .into(),
        ));
    }
    Ok(())
}

/// `--every`: `run` once, then again `every` after each start, until `wait` (given how long is
/// left) answers false. A run that fails is printed and the next one comes all the same. Runs
/// follow one another, so they never overlap; one that took longer than `every` is followed at
/// once. A signal that stopped an agent ([`interrupted`]) ends the loop with that run's error.
pub fn repeat(
    every: Duration,
    mut run: impl FnMut() -> Result<bool>,
    mut wait: impl FnMut(Duration) -> bool,
) -> Result<()> {
    loop {
        let started = std::time::Instant::now();
        match run() {
            Ok(true) => {}
            Ok(false) => eprintln!("dream: the agent command failed; trying again next time"),
            Err(e) if interrupted() => return Err(e),
            Err(e) => eprintln!("dream: {e}; trying again next time"),
        }
        if !wait(every.saturating_sub(started.elapsed())) {
            return Ok(());
        }
    }
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
    pub fix_rounds: u32,
    #[cfg_attr(not(unix), allow(dead_code))]
    pub timeout: Duration,
    /// Agents allowed to run at once on this host; `None` = no limit.
    pub max_agents: Option<u32>,
    /// Appended to the agent command as `--max-budget-usd`.
    #[cfg_attr(not(unix), allow(dead_code))]
    pub budget_usd: Option<f64>,
    /// `--promote`: the agent is told it may propose conventions (review runs only).
    pub promote: bool,
    /// Absolute directories the agent may read for context, from [`also_read_dirs`].
    pub also_read: &'a [PathBuf],
    pub slots_dir: PathBuf,
    pub dreams_dir: PathBuf,
    pub cmd: &'a [String],
}

impl Dream<'_> {
    fn mode(&self) -> &'static str {
        if self.verify { "verify" } else { "review" }
    }

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

struct Refused {
    item: Value,
    why: String,
}

/// SIGINT and SIGTERM while an agent runs: the agent runs in a process group of its own, so a
/// Ctrl-C reaches only `av`, and `av` must end that group before it goes.
#[cfg(unix)]
mod interrupt {
    use std::sync::atomic::{AtomicBool, Ordering};

    pub static STOP: AtomicBool = AtomicBool::new(false);

    extern "C" fn on_signal(_: libc::c_int) {
        STOP.store(true, Ordering::SeqCst);
    }

    /// Catches both signals until it is dropped; outside it they end `av` as usual (no agent
    /// runs then).
    pub struct Caught;

    pub fn catch() -> Caught {
        let handler = on_signal as extern "C" fn(libc::c_int) as libc::sighandler_t;
        // SAFETY: the handler only stores into an atomic, which is async-signal-safe
        unsafe {
            libc::signal(libc::SIGINT, handler);
            libc::signal(libc::SIGTERM, handler);
        }
        Caught
    }

    impl Drop for Caught {
        fn drop(&mut self) {
            // SAFETY: restoring the default disposition
            unsafe {
                libc::signal(libc::SIGINT, libc::SIG_DFL);
                libc::signal(libc::SIGTERM, libc::SIG_DFL);
            }
        }
    }
}

/// Whether a SIGINT or SIGTERM stopped an agent: `av` should end now, `--every` too.
pub fn interrupted() -> bool {
    #[cfg(unix)]
    return interrupt::STOP.load(std::sync::atomic::Ordering::SeqCst);
    #[cfg(not(unix))]
    false
}

/// Runs the agent command in the run's directory. `Ok(false)`: it failed or ran out of time.
/// `Err` when a signal stopped it; its process group is ended by then.
#[cfg(unix)]
fn run_agent(d: &Dream, dir: &Path, id: &str, round: u32) -> Result<bool> {
    let (program, args) = d.cmd.split_first().expect("checked by run");
    let mut cmd = Command::new(program);
    cmd.args(args);
    if let Some(cap) = d.budget_usd {
        cmd.args(margincode_agentrun::command::max_budget_args(cap));
    }
    if !d.also_read.is_empty() {
        let joined = std::env::join_paths(d.also_read).map_err(local)?;
        cmd.env("AV_DREAM_ALSO_READ", joined);
    }
    cmd.current_dir(dir)
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
    let caught = interrupt::catch();
    let waited = margincode_agentrun::process::run_until(cmd, d.timeout, &interrupt::STOP);
    drop(caught);
    match waited {
        Ok(w) if w.stopped => Err(local(format!(
            "interrupted; {shown} and its processes were ended"
        ))),
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
    check_command(d.cmd)?;
    let told = instructions(d.promote, !d.also_read.is_empty());
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
                "instructions_sha": sha_hex(&told)}),
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
    write(&dir.join("DREAM.md"), &told).map_err(fail)?;
    if !d.also_read.is_empty() {
        let list: String = d
            .also_read
            .iter()
            .map(|p| format!("{}\n", p.display()))
            .collect();
        write(&dir.join("also-read.txt"), &list).map_err(fail)?;
    }
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

    let mut waiting: Option<Vec<Refused>> = None;
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
        if done["dream"]["counts"]["promote"].as_u64().unwrap_or(0) > 0 {
            println!(
                "a promote becomes a convention only when a person accepts it: \
                 `av verdict accept vd:…`"
            );
        }
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
    if let Some(c) = v["convention"].as_object() {
        let field = |k: &str| c.get(k).and_then(Value::as_str).unwrap_or_default();
        let decided = match s("state") {
            "proposed" => "waits for a person",
            "effective" => "a person accepted it",
            "rejected" => "a person rejected it",
            _ => "taken back",
        };
        line.push_str(&format!(
            "  convention for {}: {} ({decided})",
            field("scope_glob"),
            field("text")
        ));
    }
    line
}

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
    /// `INSTRUCTIONS` is compiled in from outside `crates/`, so the image build must copy the
    /// file (deploy/Dockerfile) and must not ignore it (.dockerignore drops `**/*.md`).
    #[test]
    fn image_build_sees_dream_md() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let file = "integrations/dream/DREAM.md";
        let dockerfile = std::fs::read_to_string(root.join("deploy/Dockerfile")).unwrap();
        assert!(
            dockerfile.lines().any(|l| {
                let w: Vec<&str> = l.split_whitespace().collect();
                w.len() == 3 && w[0] == "COPY" && file.starts_with(w[1].trim_end_matches('/'))
            }),
            "deploy/Dockerfile copies no directory holding {file}"
        );
        let ignore = std::fs::read_to_string(root.join(".dockerignore")).unwrap();
        // The last matching line wins; only the two shapes this file uses are understood.
        let ignored = ignore.lines().fold(false, |ignored, l| match l.trim() {
            "**/*.md" => true,
            l if l.strip_prefix('!') == Some(file) => false,
            _ => ignored,
        });
        assert!(!ignored, ".dockerignore drops {file}");
    }

    use super::*;

    #[test]
    fn the_instructions_hash_is_sha256_hex() {
        let sha = sha_hex(INSTRUCTIONS);
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

    /// DREAM.md as it was before `promote` existed (commit 8244812): what a run asked for
    /// neither `--promote` nor `--also-read` is handed, byte for byte.
    #[test]
    fn a_plain_run_is_handed_dream_md_as_it_was() {
        assert_eq!(
            sha_hex(&instructions(false, false)),
            "f66dc2ec655cf8747df585d35c4dc8d0e7205f14ebd3220da50fc220aa6973aa"
        );
        let promote = instructions(true, false);
        assert!(
            promote.starts_with(INSTRUCTIONS),
            "DREAM.md's own text is kept"
        );
        assert!(promote.contains("\"verdict\": \"promote\"") && promote.contains("at most 400"));
        assert!(!promote.contains("also-read.txt"));
        let both = instructions(true, true);
        assert!(both.starts_with(&promote) && both.contains("also-read.txt"));
    }

    #[test]
    fn a_run_without_also_read_is_handed_dream_md_as_it_is() {
        assert_eq!(instructions(false, false), INSTRUCTIONS);
        let more = instructions(false, true);
        assert!(more.starts_with(INSTRUCTIONS) && more.contains("also-read.txt"));
        assert!(more.contains("not evidence"));
    }

    #[test]
    fn every_takes_seconds_minutes_hours_and_days_and_never_zero() {
        let s = |n| Ok(Duration::from_secs(n));
        assert_eq!(parse_every("30s"), s(30));
        assert_eq!(parse_every("90m"), s(5400));
        assert_eq!(parse_every("24h"), s(86400));
        assert_eq!(parse_every("1d"), s(86400));
        assert_eq!(parse_every("45"), s(45));
        for bad in [
            "", "0", "0s", "0d", "h", "-1h", "1.5h", "5w", "1h30m", "soon",
        ] {
            assert!(parse_every(bad).is_err(), "{bad:?}");
        }
        assert_eq!(span(Duration::from_secs(86400)), "1d");
        assert_eq!(span(Duration::from_millis(5_399_200)), "1h30m");
        assert_eq!(span(Duration::from_secs(90061)), "1d1h1m1s");
    }

    #[test]
    fn a_budget_is_a_positive_finite_number() {
        assert_eq!(parse_budget("2.5"), Ok(2.5));
        assert_eq!(parse_budget("0.01"), Ok(0.01));
        for bad in ["0", "-1", "0.001", "nan", "inf", "-inf", "two", ""] {
            assert!(parse_budget(bad).is_err(), "{bad:?}");
        }
    }

    #[test]
    fn repeat_goes_on_after_a_failed_run_and_stops_when_told() {
        let every = Duration::from_secs(3600);
        let mut answers = vec![
            Ok(true),
            Err(CliError::Api {
                status: 409,
                message: "another run holds the area".into(),
                body: Value::Null,
            }),
            Ok(false),
        ]
        .into_iter();
        let mut runs = 0;
        let mut waits = vec![];
        repeat(
            every,
            || {
                runs += 1;
                answers.next().unwrap()
            },
            |left| {
                waits.push(left);
                waits.len() < 3
            },
        )
        .unwrap();
        assert_eq!(runs, 3, "a failed run does not end the loop; the stop does");
        assert_eq!(waits.len(), 3);
        assert!(
            waits.iter().all(|w| *w <= every && *w > every / 2),
            "{waits:?}"
        );
    }

    #[test]
    fn also_read_takes_directories_only_and_makes_them_absolute() {
        let dir = std::env::temp_dir().join(format!("av-also-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(dir.join("t")).unwrap();
        std::fs::write(dir.join("f"), "x").unwrap();
        let got = also_read_dirs(&[dir.join("t")]).unwrap();
        assert_eq!(got, vec![std::fs::canonicalize(dir.join("t")).unwrap()]);
        for bad in [dir.join("f"), dir.join("missing")] {
            let e = also_read_dirs(std::slice::from_ref(&bad))
                .unwrap_err()
                .to_string();
            assert!(
                e.contains("--also-read") && e.contains(&*bad.to_string_lossy()),
                "{e}"
            );
        }
        std::fs::remove_dir_all(&dir).unwrap();
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
