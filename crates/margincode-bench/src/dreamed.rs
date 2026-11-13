//! Plan 16 M2 stage B: one dreamed memory, frozen once and given to every session of the arm.
//!
//! [`freeze`] seeds each task's store as a session would ([`runner::seed_store`]), runs
//! `av dream` and then `av dream --verify` as a second agent, and writes what the two passes
//! ended with: every verdict, its state, and the verify run's review with its evidence. A
//! session's store is seeded afresh, so its record ids, event ids and trunk commit are new; the
//! file therefore names a record by its earlier task's role, kind, summary and place, an event
//! by its kind, task role and place, and drops a quote's commit when it was the run's own.
//!
//! [`replay`] writes those verdicts and reviews into a freshly seeded store through the
//! daemon's own dream routes (a review run that proposes, a verify run that reviews), so the
//! daemon checks every quote again, and then checks that each verdict ended in the state it had
//! when it was frozen. Every session of the arm reads the same verdicts: the arm measures one
//! dreamed memory, not the spread between dreams.

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::Result;
use crate::runner::{Daemon, RunConfig, Seeded, seed_store};
use crate::spec::{TaskSpec, git_in};

/// The file's format. A file of another version is refused.
pub const VERSION: u32 = 1;

/// A review run is handed at most this many records (the daemon's `WORKLIST_MAX`).
const RUN_LIMIT: u32 = 50;

/// A record as it is found again in another seed of the same task: the role of its earlier task
/// ([`crate::runner::OPEN_TASK`] or [`crate::runner::REVERTED_TASK`]), its kind, and either its
/// place in the pool (`note`) or, for a record the daemon wrote (the revert's note), which of
/// that task's other records of that kind it is (`nth`, from 0, in the task's order). Not its
/// summary: the daemon's own records name commits, which differ in every seed. `summary` is
/// kept for a person reading the file and is not matched.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RecKey {
    pub task: String,
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<usize>,
    pub nth: usize,
    pub summary: String,
}

impl RecKey {
    fn ident(&self) -> Ident {
        (self.task.clone(), self.kind.clone(), self.note, self.nth)
    }
}

/// What a [`RecKey`] is matched by.
type Ident = (String, String, Option<usize>, usize);

/// An event as it is found again: its kind, the role of its task (none for an event of no
/// earlier task), and which of those it is.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventKey {
    pub kind: String,
    pub task: Option<String>,
    pub nth: usize,
}

/// The verify run's review of a verdict.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrozenReview {
    pub reviewer: String,
    pub agree: bool,
    /// As the run gave it, with records, events and the run's commit renamed (module doc).
    pub evidence: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrozenVerdict {
    pub record: RecKey,
    /// The record's place in the task's pool, when it is one of the pool's notes.
    pub note: Option<usize>,
    /// That note's pair, its `no_commit` (a wrong note) and its `noise`, for the tally.
    pub pair: Option<String>,
    #[serde(default)]
    pub wrong: bool,
    #[serde(default)]
    pub noise: bool,
    /// `confirmed`, `disputed`, `duplicate` or `outdated`.
    pub verdict: String,
    pub reason: String,
    pub canonical: Option<RecKey>,
    pub evidence: Vec<Value>,
    /// The verdict's state after both passes: `effective`, `rejected` or `proposed`. A replay
    /// must end in the same one.
    pub state: String,
    pub review: Option<FrozenReview>,
}

/// What the two passes did to the task's pool: stage A's bars read these.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Tally {
    pub verdicts: usize,
    pub effective: usize,
    /// The pool's `no_commit` notes, and how many of them ended `disputed` and effective.
    pub wrong_notes: usize,
    pub wrong_disputed: usize,
    /// The pool's other notes that are not noise (the true lessons), and how many of them
    /// ended `disputed` or `outdated` and effective (stage A's bar: none).
    pub true_notes: usize,
    pub true_hit: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrozenTask {
    /// `pair: summary` of every note of the pool, in order: a replay refuses another pool (a
    /// different `--noise` or `--noise-seed`).
    pub pool: Vec<String>,
    pub wrong_reverted: bool,
    pub dreamer: String,
    pub verifier: String,
    pub verdicts: Vec<FrozenVerdict>,
    pub tally: Tally,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Frozen {
    pub version: u32,
    pub tasks: BTreeMap<String, FrozenTask>,
}

impl Frozen {
    pub fn load(path: &Path) -> Result<Frozen> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let f: Frozen =
            serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
        if f.version != VERSION {
            return Err(format!(
                "{}: version {}, this harness reads {VERSION}",
                path.display(),
                f.version
            ));
        }
        Ok(f)
    }

    /// This task's frozen dream, refused unless it was made from the same pool and with the
    /// same `--wrong-reverted`.
    pub fn task(&self, spec: &TaskSpec, wrong_reverted: bool) -> Result<&FrozenTask> {
        let t = self.tasks.get(&spec.name).ok_or_else(|| {
            format!(
                "the dreamed file has no task {}: run dream-freeze on it",
                spec.name
            )
        })?;
        if t.pool != pool_of(spec) {
            return Err(format!(
                "task {}: the dreamed file was frozen from another pool of notes (a different \
                 --noise or --noise-seed?)",
                spec.name
            ));
        }
        if t.wrong_reverted != wrong_reverted {
            return Err(format!(
                "task {}: the dreamed file was frozen {} --wrong-reverted, this run is {} it",
                spec.name,
                if t.wrong_reverted { "with" } else { "without" },
                if wrong_reverted { "with" } else { "without" },
            ));
        }
        Ok(t)
    }
}

fn pool_of(spec: &TaskSpec) -> Vec<String> {
    spec.relay_notes
        .iter()
        .map(|n| format!("{}: {}", n.pair, n.summary))
        .collect()
}

/// The agents of the two passes and how they run.
#[derive(Debug, Clone)]
pub struct FreezeArgs {
    /// The name the review pass dreams as, and its command (run with `sh -c` in the run's
    /// directory, beside `DREAM.md` and `bundle.json`).
    pub dreamer: String,
    pub dream_cmd: String,
    /// The name and command of the verify pass: another agent.
    pub verifier: String,
    pub verify_cmd: String,
    /// `av dream --limit`; the daemon's default when `None`.
    pub limit: Option<u32>,
    pub timeout: Duration,
    pub fix_rounds: u32,
    /// `$AV_HOME` of the two passes: their `dreams/<id>/` directories (bundle, instructions,
    /// what the agent wrote) are kept under it.
    pub home: PathBuf,
}

/// What [`crate::report::Meta::memory`] says for a run made with `cfg`.
pub fn memory_of(cfg: &RunConfig) -> Result<String> {
    use sha2::{Digest, Sha256};
    let mut parts = vec![];
    if cfg.wrong_reverted {
        parts.push("wrong-reverted".to_owned());
    }
    if let Some(file) = &cfg.dreamed {
        let bytes = std::fs::read(file).map_err(|e| format!("{}: {e}", file.display()))?;
        let hex: String = Sha256::digest(&bytes)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        parts.push(format!("dreamed sha256:{hex}"));
    }
    Ok(if parts.is_empty() {
        "raw".to_owned()
    } else {
        parts.join(", ")
    })
}

/// The ids a seeded store gave its records and events, and their keys.
struct Names {
    records: HashMap<String, RecKey>,
    by_key: HashMap<Ident, String>,
    events: HashMap<i64, EventKey>,
    events_by_key: HashMap<EventKey, i64>,
}

fn bare(id: &str) -> String {
    id.trim()
        .trim_start_matches("rec:")
        .replace('-', "")
        .to_lowercase()
}

impl Names {
    fn read(daemon: &Daemon, repo: &str, seeded: &Seeded) -> Result<Names> {
        let mut records = HashMap::new();
        let mut by_key = HashMap::new();
        let mut roles = HashMap::new();
        let note_of: HashMap<String, usize> = seeded
            .notes
            .iter()
            .enumerate()
            .map(|(i, (_, id))| (bare(id), i))
            .collect();
        for (role, task) in &seeded.tasks {
            roles.insert(task.clone(), role.clone());
            let list = daemon.get(&format!("/v1/tasks/{task}/records"))?;
            let mut seen: HashMap<String, usize> = HashMap::new();
            for r in list["records"].as_array().into_iter().flatten() {
                let id = bare(r["id"].as_str().ok_or("a record without an id")?);
                let kind = r["kind"].as_str().unwrap_or_default().to_owned();
                let note = note_of.get(&id).copied();
                let nth = match note {
                    Some(_) => 0,
                    None => {
                        let n = seen.entry(kind.clone()).or_default();
                        *n += 1;
                        *n - 1
                    }
                };
                let key = RecKey {
                    task: role.clone(),
                    kind,
                    note,
                    nth,
                    summary: r["summary"].as_str().unwrap_or_default().to_owned(),
                };
                by_key.insert(key.ident(), id.clone());
                records.insert(id, key);
            }
        }
        let mut events = HashMap::new();
        let mut events_by_key = HashMap::new();
        let mut seen: HashMap<(String, Option<String>), usize> = HashMap::new();
        let mut since = 0;
        loop {
            let page = daemon.get(&format!("/v1/events?repo={repo}&since={since}&limit=500"))?;
            let list = page["events"].as_array().cloned().unwrap_or_default();
            if list.is_empty() {
                break;
            }
            for ev in &list {
                let id = ev["id"].as_i64().ok_or("an event without an id")?;
                since = since.max(id);
                let kind = ev["kind"].as_str().unwrap_or_default().to_owned();
                let task = ev["task_id"].as_str().and_then(|t| roles.get(t)).cloned();
                let n = seen.entry((kind.clone(), task.clone())).or_default();
                let key = EventKey {
                    kind,
                    task,
                    nth: *n,
                };
                *n += 1;
                events_by_key.insert(key.clone(), id);
                events.insert(id, key);
            }
        }
        Ok(Names {
            records,
            by_key,
            events,
            events_by_key,
        })
    }

    /// The key of a record named by its id or a `rec:` handle (a prefix of it).
    fn key_of(&self, named: &str) -> Result<RecKey> {
        let want = bare(named);
        let mut hits = self.records.iter().filter(|(id, _)| id.starts_with(&want));
        match (hits.next(), hits.next()) {
            (Some((_, k)), None) if !want.is_empty() => Ok(k.clone()),
            (Some(_), Some(_)) => Err(format!("{named} names more than one seeded record")),
            _ => Err(format!(
                "{named} is not a record of the seeded earlier tasks, so no other seed has it"
            )),
        }
    }

    fn id_of(&self, key: &RecKey) -> Result<String> {
        self.by_key.get(&key.ident()).cloned().ok_or_else(|| {
            let which = match key.note {
                Some(i) => format!("note {}", ordinal(i)),
                None => format!("{} {}", key.kind, ordinal(key.nth)),
            };
            format!(
                "no seeded record is {which} of the {} task ({:?})",
                key.task, key.summary
            )
        })
    }
}

fn ordinal(n: usize) -> String {
    format!("#{}", n + 1)
}

/// A quote's evidence as the file keeps it: records and events by key, the run's own commit
/// dropped (a quote without one is checked at the run's commit). A commit of the task's own
/// repository stays; any other is refused, since no other seed has it.
fn freeze_evidence(e: &Value, names: &Names, run_commit: &str, spec: &TaskSpec) -> Result<Value> {
    if e.get("code").is_some() {
        let mut out = e.clone();
        if let Some(c) = e["commit"].as_str() {
            if c == run_commit {
                out.as_object_mut().map(|o| o.remove("commit"));
            } else if git_in(
                &spec.repo_path,
                &["cat-file", "-e", &format!("{c}^{{commit}}")],
            )
            .is_err()
            {
                return Err(format!(
                    "a quote names commit {c}, which only this seed has; no session could \
                     check it"
                ));
            }
        }
        return Ok(out);
    }
    if let Some(named) = e["record"].as_str() {
        return Ok(json!({"record_key": names.key_of(named)?, "quote": e["quote"]}));
    }
    if let Some(id) = e["event"].as_i64() {
        let key = names
            .events
            .get(&id)
            .ok_or_else(|| format!("event {id} is not an event of the seeded repository"))?;
        return Ok(json!({"event_key": key}));
    }
    Err(format!("evidence the harness does not know: {e}"))
}

/// The other way: evidence a replay's run can post.
fn thaw_evidence(e: &Value, names: &Names) -> Result<Value> {
    if let Some(key) = e.get("record_key") {
        let key: RecKey = serde_json::from_value(key.clone()).map_err(|e| e.to_string())?;
        return Ok(json!({"record": names.id_of(&key)?, "quote": e["quote"]}));
    }
    if let Some(key) = e.get("event_key") {
        let key: EventKey = serde_json::from_value(key.clone()).map_err(|e| e.to_string())?;
        let id = names.events_by_key.get(&key).ok_or_else(|| {
            format!(
                "no {} event {} of the {:?} task in this seed",
                key.kind,
                ordinal(key.nth),
                key.task
            )
        })?;
        return Ok(json!({"event": id}));
    }
    Ok(e.clone())
}

fn freeze_all(
    evidence: &Value,
    names: &Names,
    run_commit: &str,
    spec: &TaskSpec,
) -> Result<Vec<Value>> {
    evidence
        .as_array()
        .into_iter()
        .flatten()
        .map(|e| freeze_evidence(e, names, run_commit, spec))
        .collect()
}

fn thaw_all(evidence: &[Value], names: &Names) -> Result<Vec<Value>> {
    evidence.iter().map(|e| thaw_evidence(e, names)).collect()
}

/// Runs `av dream` (`verify`: `--verify`) on `repo` as `agent`.
fn av_dream(cfg: &RunConfig, args: &FreezeArgs, repo: &str, verify: bool) -> Result<()> {
    let (agent, cmd) = if verify {
        (&args.verifier, &args.verify_cmd)
    } else {
        (&args.dreamer, &args.dream_cmd)
    };
    let url = cfg
        .av_url
        .as_deref()
        .ok_or("dream-freeze needs the bench daemon (--av-url)")?;
    let mut c = Command::new(&cfg.av_bin);
    c.arg("dream").arg("--repo").arg(repo);
    if verify {
        c.arg("--verify");
    }
    if let Some(n) = args.limit {
        c.arg("--limit").arg(n.to_string());
    }
    c.arg("--timeout")
        .arg(args.timeout.as_secs().to_string())
        .arg("--fix-rounds")
        .arg(args.fix_rounds.to_string())
        .arg("--")
        .args(["sh", "-c", cmd])
        .env("AV_URL", url)
        .env("AV_AGENT", agent)
        .env("AV_HOME", &args.home)
        .env_remove("AV_TASK")
        .env_remove("AV_SESSION")
        .env_remove("AV_GENERATION");
    match &cfg.av_token {
        Some(t) => c.env("AV_TOKEN", t),
        None => c.env_remove("AV_TOKEN"),
    };
    std::fs::create_dir_all(&args.home).map_err(|e| format!("{}: {e}", args.home.display()))?;
    let out = c.output().map_err(|e| format!("{}: {e}", cfg.av_bin))?;
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    if !out.status.success() {
        return Err(format!(
            "av dream{} on {repo} as {agent} did not finish: {}",
            if verify { " --verify" } else { "" },
            said.trim()
        ));
    }
    Ok(())
}

/// Freezes one task: a store seeded as `name`, both passes, and what they ended with.
pub fn freeze(
    spec: &TaskSpec,
    cfg: &RunConfig,
    daemon: &Daemon,
    name: &str,
    args: &FreezeArgs,
) -> Result<FrozenTask> {
    if spec.relay_notes.is_empty() {
        return Err(format!(
            "task {} has no relay pool: there is nothing for a dream to look at",
            spec.name
        ));
    }
    if args.dreamer == args.verifier {
        return Err("the verify pass is a second look: give it another agent name".into());
    }
    let (_src, seeded) = seed_store(spec, cfg, daemon, name)?;
    av_dream(cfg, args, name, false)?;
    av_dream(cfg, args, name, true)?;

    let names = Names::read(daemon, name, &seeded)?;
    let list = daemon.get(&format!("/v1/repos/{name}/verdicts?limit=1000"))?;
    let mut commits: HashMap<String, String> = HashMap::new();
    let mut run_commit = |dream: Option<&str>| -> Result<String> {
        let Some(d) = dream else {
            return Ok(String::new());
        };
        if let Some(c) = commits.get(d) {
            return Ok(c.clone());
        }
        let c = daemon.get(&format!("/v1/dreams/{d}"))?["dream"]["worklist"]["commit"]
            .as_str()
            .unwrap_or_default()
            .to_owned();
        commits.insert(d.to_owned(), c.clone());
        Ok(c)
    };
    let mut verdicts = vec![];
    // the list is newest first; the file keeps the order they were proposed in
    for v in list["verdicts"].as_array().into_iter().flatten().rev() {
        let state = v["state"].as_str().unwrap_or_default();
        if !matches!(state, "effective" | "rejected" | "proposed") {
            continue; // retracted: not part of the memory
        }
        let id = v["id"].as_str().ok_or("a verdict without an id")?;
        let one = daemon.get(&format!("/v1/verdicts/{id}"))?;
        let record = names.key_of(v["record_id"].as_str().unwrap_or_default())?;
        let note = record.note;
        let n = note.map(|i| &spec.relay_notes[i]);
        let commit = run_commit(v["dream_id"].as_str())?;
        let review = one["reviews"]
            .as_array()
            .into_iter()
            .flatten()
            .rev()
            .find(|r| !r["dream"].is_null());
        if one["reviews"]
            .as_array()
            .is_some_and(|rs| rs.iter().any(|r| r["dream"].is_null()))
        {
            return Err(format!(
                "verdict {id} has a person's review; a frozen dream holds only the two runs'"
            ));
        }
        let review = match review {
            None => None,
            Some(r) => {
                let c = run_commit(r["dream"].as_str())?;
                Some(FrozenReview {
                    reviewer: r["reviewer"].as_str().unwrap_or_default().to_owned(),
                    agree: r["agree"] == true,
                    evidence: freeze_all(&r["evidence"], &names, &c, spec)?,
                })
            }
        };
        verdicts.push(FrozenVerdict {
            record,
            note,
            pair: n.map(|n| n.pair.clone()),
            wrong: n.is_some_and(|n| n.no_commit),
            noise: n.is_some_and(|n| n.noise),
            verdict: v["verdict"].as_str().unwrap_or_default().to_owned(),
            reason: v["reason"].as_str().unwrap_or_default().to_owned(),
            canonical: match v["canonical"].as_str() {
                Some(c) => Some(names.key_of(c)?),
                None => None,
            },
            evidence: freeze_all(&v["evidence"], &names, &commit, spec)?,
            state: state.to_owned(),
            review,
        });
    }
    let tally = tally(spec, &verdicts);
    Ok(FrozenTask {
        pool: pool_of(spec),
        wrong_reverted: cfg.wrong_reverted,
        dreamer: args.dreamer.clone(),
        verifier: args.verifier.clone(),
        verdicts,
        tally,
    })
}

pub fn tally(spec: &TaskSpec, verdicts: &[FrozenVerdict]) -> Tally {
    let effective = |v: &&FrozenVerdict| v.state == "effective";
    let notes = &spec.relay_notes;
    Tally {
        verdicts: verdicts.len(),
        effective: verdicts.iter().filter(effective).count(),
        wrong_notes: notes.iter().filter(|n| n.no_commit).count(),
        wrong_disputed: verdicts
            .iter()
            .filter(effective)
            .filter(|v| v.wrong && v.verdict == "disputed")
            .count(),
        true_notes: notes.iter().filter(|n| !n.no_commit && !n.noise).count(),
        true_hit: verdicts
            .iter()
            .filter(effective)
            .filter(|v| v.note.is_some() && !v.wrong && !v.noise)
            .filter(|v| matches!(v.verdict.as_str(), "disputed" | "outdated"))
            .count(),
    }
}

/// Writes `task`'s verdicts and reviews into the store just seeded as `repo` (module doc).
pub fn replay(
    task: &FrozenTask,
    spec: &TaskSpec,
    daemon: &Daemon,
    repo: &str,
    seeded: &Seeded,
) -> Result<()> {
    if task.verdicts.is_empty() {
        return Ok(());
    }
    let names = Names::read(daemon, repo, seeded)?;
    let ids: Vec<String> = task
        .verdicts
        .iter()
        .map(|v| names.id_of(&v.record))
        .collect::<Result<_>>()?;

    // the review pass: as many runs as it takes to be handed every record
    let mut made: Vec<Option<String>> = vec![None; task.verdicts.len()];
    while made.iter().any(Option::is_none) {
        let started = daemon.post(
            &format!("/v1/repos/{repo}/dreams"),
            json!({"agent": task.dreamer, "mode": "review", "limit": RUN_LIMIT}),
            None,
        )?;
        let dream = started["dream"]["id"]
            .as_str()
            .ok_or_else(|| {
                format!(
                    "task {}: the replay's review run was handed nothing",
                    spec.name
                )
            })?
            .to_owned();
        let on_list: Vec<String> = started["dream"]["worklist"]["records"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|r| r.as_str().map(bare))
            .collect();
        let mut batch = vec![];
        let mut which = vec![];
        for (i, v) in task.verdicts.iter().enumerate() {
            if made[i].is_some() || !on_list.contains(&ids[i]) {
                continue;
            }
            let mut item = json!({"record": ids[i], "verdict": v.verdict, "reason": v.reason,
                                  "evidence": thaw_all(&v.evidence, &names)?});
            if let Some(c) = &v.canonical {
                item["canonical"] = json!(names.id_of(c)?);
            }
            batch.push(item);
            which.push(i);
        }
        if batch.is_empty() {
            let _ = daemon.post(
                &format!("/v1/dreams/{dream}/finish"),
                json!({"failed": true}),
                None,
            );
            return Err(format!(
                "task {}: a frozen verdict's record is on no review run's list",
                spec.name
            ));
        }
        let answer = daemon.post(
            &format!("/v1/dreams/{dream}/verdicts"),
            json!({ "verdicts": batch }),
            None,
        )?;
        daemon.post(&format!("/v1/dreams/{dream}/finish"), json!({}), None)?;
        for (r, i) in answer["results"]
            .as_array()
            .into_iter()
            .flatten()
            .zip(which)
        {
            if r["accepted"] != true {
                return Err(format!(
                    "task {}: the daemon refused frozen verdict {} on {:?}: {}",
                    spec.name, i, task.verdicts[i].record.summary, r["why"]
                ));
            }
            made[i] = r["verdict"]["id"].as_str().map(str::to_owned);
        }
    }
    let made: Vec<String> = made.into_iter().map(Option::unwrap_or_default).collect();

    // the verify pass, for the verdicts the frozen one reviewed
    let mut waiting: Vec<usize> = (0..task.verdicts.len())
        .filter(|&i| task.verdicts[i].review.is_some())
        .collect();
    while !waiting.is_empty() {
        let started = daemon.post(
            &format!("/v1/repos/{repo}/dreams"),
            json!({"agent": task.verifier, "mode": "verify", "limit": RUN_LIMIT}),
            None,
        )?;
        let dream = started["dream"]["id"]
            .as_str()
            .ok_or_else(|| {
                format!(
                    "task {}: the replay's verify run was handed nothing",
                    spec.name
                )
            })?
            .to_owned();
        let on_list: Vec<&str> = started["dream"]["worklist"]["verdicts"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .collect();
        let (now, later): (Vec<usize>, Vec<usize>) = waiting
            .iter()
            .partition(|&&i| on_list.contains(&made[i].as_str()));
        let result = (|| -> Result<()> {
            for &i in &now {
                let r = task.verdicts[i].review.as_ref().expect("filtered");
                daemon.post(
                    &format!("/v1/verdicts/{}/review", made[i]),
                    json!({"reviewer": r.reviewer, "agree": r.agree, "dream": dream,
                           "evidence": thaw_all(&r.evidence, &names)?}),
                    None,
                )?;
            }
            Ok(())
        })();
        let failed = result.is_err() || now.is_empty();
        let _ = daemon.post(
            &format!("/v1/dreams/{dream}/finish"),
            json!({ "failed": failed }),
            None,
        );
        result.map_err(|e| format!("task {}: a frozen review was refused: {e}", spec.name))?;
        if now.is_empty() {
            return Err(format!(
                "task {}: a frozen review's verdict is on no verify run's list",
                spec.name
            ));
        }
        waiting = later;
    }

    // every verdict ends where it ended when it was frozen
    for (v, id) in task.verdicts.iter().zip(&made) {
        let now = daemon.get(&format!("/v1/verdicts/{id}"))?;
        if now["verdict"]["state"] != v.state.as_str() {
            return Err(format!(
                "task {}: the replayed verdict on {:?} is {}, frozen as {}",
                spec.name, v.record.summary, now["verdict"]["state"], v.state
            ));
        }
    }
    Ok(())
}
