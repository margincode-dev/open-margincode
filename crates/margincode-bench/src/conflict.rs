//! The check the dream's `conflict` verdict passes before it is switched on anywhere (its
//! design, section 8). A planted set (`bench/conflict-check`: a fixture repository, three
//! articles, three conventions, 24 records, and a key that says which record is which) is
//! seeded twice alike, as two repositories of one bench daemon; `av dream` makes one review
//! run on each, one with `--conflicts` and one without; [`score`] reads conditions (a) to (d)
//! from the key. The agent is the last freeze's dreamer on its model, or, in a dry run, this
//! binary's [`fake_agent`], which calls no model.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::Result;
use crate::runner::{Daemon, git_ok};

/// No run starts that could take the check's spend past this (section 8).
pub const CEILING_USD: f64 = 2.0;
/// One agent call's cap, the `M2_RUN_CAP` that `dreamer.sh` passes to Claude Code as
/// `--max-budget-usd`: above the last freeze's $0.25, since this bundle holds 24 records with
/// their code and rules. A call that left no cost is charged this.
pub const CALL_CAP_USD: f64 = 0.45;
/// The last freeze's dreamer model.
pub const MODEL: &str = "claude-sonnet-5-5";
/// `av dream --fix-rounds`: the last freeze passed none, so it ran at the default.
pub const FIX_ROUNDS: u32 = 1;
const LIMIT: u32 = 50;
const TIMEOUT_SECS: u64 = 1800;
/// sha256 of `DREAM.md` as a run without `--conflicts` is handed it (pinned in `margincode-cli`).
pub const PLAIN_SHA: &str = "f66dc2ec655cf8747df585d35c4dc8d0e7205f14ebd3220da50fc220aa6973aa";
const MAX_SUMMARY: usize = 200;
const MAX_ARTICLE_TITLE: usize = 60;
const MAX_ARTICLE_TEXT: usize = 400;

#[derive(Debug, Clone, Deserialize)]
pub struct TaskDef {
    pub agent: String,
    pub title: String,
    pub goal: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ArticleDef {
    pub name: String,
    pub title: String,
    pub text: String,
    pub reason: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConventionDef {
    pub name: String,
    pub scope_glob: String,
    pub text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnchorDef {
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecordDef {
    pub id: String,
    pub agent: String,
    pub kind: String,
    pub summary: String,
    #[serde(default)]
    pub body: String,
    pub anchors: Vec<AnchorDef>,
    #[serde(default)]
    pub before_rules: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetFile {
    pub person: String,
    pub task: Vec<TaskDef>,
    pub article: Vec<ArticleDef>,
    pub convention: Vec<ConventionDef>,
    pub record: Vec<RecordDef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Category {
    Planted,
    Overlap,
    Silent,
    Attempt,
    Untouched,
    CodeDisputes,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KeyEntry {
    pub category: Category,
    /// A planted record's rule; a control's, the rule it is close to.
    #[serde(default)]
    pub rule: Option<String>,
    #[serde(default)]
    pub older: bool,
    #[serde(default)]
    pub exception: bool,
    pub why: String,
}

pub struct Set {
    pub dir: PathBuf,
    pub file: SetFile,
    pub key: BTreeMap<String, KeyEntry>,
}

fn read(path: &Path) -> Result<String> {
    std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))
}

fn need(bad: &mut Vec<String>, ok: bool, what: impl FnOnce() -> String) {
    if !ok {
        bad.push(what());
    }
}

impl Set {
    pub fn load(dir: &Path) -> Result<Set> {
        Set::parse(
            dir,
            &read(&dir.join("set.toml"))?,
            &read(&dir.join("key.toml"))?,
        )
    }

    pub fn parse(dir: &Path, set: &str, key: &str) -> Result<Set> {
        Ok(Set {
            dir: dir.to_path_buf(),
            file: toml::from_str(set).map_err(|e| format!("set.toml: {e}"))?,
            key: toml::from_str(key).map_err(|e| format!("key.toml: {e}"))?,
        })
    }

    pub fn is_article(&self, rule: &str) -> bool {
        self.file.article.iter().any(|a| a.name == rule)
    }

    fn rule_text(&self, rule: &str) -> Option<&str> {
        let article = self.file.article.iter().find(|a| a.name == rule);
        let convention = self.file.convention.iter().find(|c| c.name == rule);
        article
            .map(|a| a.text.as_str())
            .or(convention.map(|c| c.text.as_str()))
    }

    pub fn ids_of(&self, category: Category) -> Vec<&str> {
        self.key
            .iter()
            .filter(|(_, k)| k.category == category)
            .map(|(id, _)| id.as_str())
            .collect()
    }

    pub fn controls(&self) -> Vec<&str> {
        self.key
            .iter()
            .filter(|(_, k)| k.category != Category::Planted)
            .map(|(id, _)| id.as_str())
            .collect()
    }

    /// The set is what section 8 fixes, and the daemon can take it as written.
    pub fn check(&self) -> Result<()> {
        let f = &self.file;
        let mut bad = vec![];
        need(&mut bad, f.article.len() == 3, || {
            format!("{} articles, not 3", f.article.len())
        });
        need(&mut bad, f.convention.len() == 3, || {
            format!("{} conventions, not 3", f.convention.len())
        });
        need(&mut bad, f.record.len() == 24, || {
            format!("{} records, not 24", f.record.len())
        });
        let rules: BTreeSet<&str> = f
            .article
            .iter()
            .map(|a| a.name.as_str())
            .chain(f.convention.iter().map(|c| c.name.as_str()))
            .collect();
        need(&mut bad, rules.len() == 6, || {
            "two rules share a name".into()
        });
        let agents: BTreeSet<&str> = f.task.iter().map(|t| t.agent.as_str()).collect();
        need(
            &mut bad,
            !f.person.trim().is_empty() && !agents.contains(f.person.as_str()),
            || "the person who adds the articles must be named and must not be an agent".into(),
        );
        for a in &f.article {
            need(
                &mut bad,
                a.title.chars().count() <= MAX_ARTICLE_TITLE,
                || format!("{}: title over {MAX_ARTICLE_TITLE} characters", a.name),
            );
            need(&mut bad, a.text.chars().count() <= MAX_ARTICLE_TEXT, || {
                format!("{}: text over {MAX_ARTICLE_TEXT} characters", a.name)
            });
        }
        let mut globs = BTreeMap::new();
        for c in &f.convention {
            match globset::Glob::new(&c.scope_glob) {
                Ok(g) => {
                    globs.insert(c.name.as_str(), g.compile_matcher());
                }
                Err(e) => bad.push(format!("{}: {e}", c.name)),
            }
        }
        let ids: BTreeSet<&str> = f.record.iter().map(|r| r.id.as_str()).collect();
        need(&mut bad, ids.len() == f.record.len(), || {
            "two records share an id".into()
        });
        let keyed: BTreeSet<&str> = self.key.keys().map(String::as_str).collect();
        need(&mut bad, ids == keyed, || {
            "key.toml and set.toml do not name the same records".into()
        });
        for r in &f.record {
            let id = &r.id;
            need(&mut bad, agents.contains(r.agent.as_str()), || {
                format!("{id}: agent {} has no task", r.agent)
            });
            need(
                &mut bad,
                matches!(r.kind.as_str(), "decision" | "attempt" | "note"),
                || format!("{id}: kind {}", r.kind),
            );
            need(&mut bad, r.summary.chars().count() <= MAX_SUMMARY, || {
                format!("{id}: summary over {MAX_SUMMARY} characters")
            });
            need(&mut bad, !r.anchors.is_empty(), || {
                format!("{id}: no anchor")
            });
            for a in &r.anchors {
                need(
                    &mut bad,
                    self.dir.join("repo").join(&a.path).is_file(),
                    || format!("{id}: {} is not a file of repo/", a.path),
                );
            }
            let Some(k) = self.key.get(id) else { continue };
            need(
                &mut bad,
                (r.kind == "attempt") == (k.category == Category::Attempt),
                || format!("{id}: an attempt control is an attempt, and only it"),
            );
            if let Some(rule) = &k.rule {
                need(&mut bad, rules.contains(rule.as_str()), || {
                    format!("{id}: no rule {rule}")
                });
            }
            need(&mut bad, r.before_rules == k.older, || {
                format!("{id}: only the record older than its rule is seeded before the rules")
            });
            if k.category == Category::Planted {
                match &k.rule {
                    None => bad.push(format!("{id}: a planted conflict names its rule")),
                    Some(rule) => {
                        if let Some(g) = globs.get(rule.as_str()) {
                            need(
                                &mut bad,
                                r.anchors.iter().any(|a| g.is_match(&a.path)),
                                || format!("{id}: {rule} covers none of its anchors"),
                            );
                        }
                    }
                }
            } else {
                need(&mut bad, !k.older && !k.exception, || {
                    format!("{id}: only a planted conflict is older or an exception")
                });
            }
        }
        for (category, n) in [
            (Category::Planted, 8),
            (Category::Overlap, 4),
            (Category::Silent, 4),
            (Category::Attempt, 2),
            (Category::Untouched, 3),
            (Category::CodeDisputes, 3),
        ] {
            let got = self.ids_of(category).len();
            need(&mut bad, got == n, || {
                format!("{got} {category:?}, not {n}")
            });
        }
        let planted: Vec<&KeyEntry> = self
            .key
            .values()
            .filter(|k| k.category == Category::Planted)
            .collect();
        let against_articles = planted
            .iter()
            .filter(|k| k.rule.as_deref().is_some_and(|r| self.is_article(r)))
            .count();
        need(&mut bad, against_articles == 4, || {
            format!("{against_articles} planted against an article, not 4")
        });
        let older = planted.iter().filter(|k| k.older).count();
        let exception = planted.iter().filter(|k| k.exception).count();
        need(&mut bad, older == 1, || {
            format!("{older} planted older than its rule, not 1")
        });
        need(&mut bad, exception == 1, || {
            format!("{exception} planted claiming an exception, not 1")
        });
        if bad.is_empty() {
            Ok(())
        } else {
            Err(format!("the set does not hold: {}", bad.join("; ")))
        }
    }

    /// sha256 of `set.toml` and `key.toml` as read, so a result names the set it was scored on.
    pub fn sha(&self) -> Result<String> {
        use sha2::{Digest, Sha256};
        let mut h = Sha256::new();
        for f in ["set.toml", "key.toml"] {
            h.update(read(&self.dir.join(f))?.as_bytes());
        }
        Ok(h.finalize().iter().map(|b| format!("{b:02x}")).collect())
    }
}

/// What one run ended with, by the set's record ids.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct RunResult {
    /// The verdict the daemon took for each record (`conflict`: with its rule, by the set's
    /// name). A record the agent left out, or whose verdict was refused, is not here.
    pub accepted: BTreeMap<String, (String, Option<String>)>,
    /// Records the agent sent as `conflict`, whether the daemon took them or refused them.
    pub proposed_conflict: BTreeSet<String>,
    /// Records refused at least once because a rule quote was not in the rule.
    pub rule_quote_refused: BTreeSet<String>,
    /// Every refusal, in order.
    pub refused: BTreeMap<String, Vec<String>>,
}

impl RunResult {
    pub fn kind(&self, id: &str) -> &str {
        self.accepted.get(id).map_or("none", |(k, _)| k.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Condition {
    pub name: &'static str,
    pub says: &'static str,
    pub count: usize,
    pub of: usize,
    /// The bound: at least (a, c) or at most (b, d).
    pub bound: usize,
    pub at_least: bool,
    pub pass: bool,
    pub ids: Vec<String>,
}

impl Condition {
    fn new(
        name: &'static str,
        says: &'static str,
        ids: Vec<String>,
        of: usize,
        bound: usize,
        at_least: bool,
    ) -> Condition {
        let count = ids.len();
        Condition {
            name,
            says,
            count,
            of,
            bound,
            at_least,
            pass: if at_least {
                count >= bound
            } else {
                count <= bound
            },
            ids,
        }
    }

    pub fn line(&self) -> String {
        format!(
            "({}) {}: {} of {} ({} {}): {}",
            self.name,
            self.says,
            self.count,
            self.of,
            if self.at_least { "at least" } else { "at most" },
            self.bound,
            if self.pass { "pass" } else { "FAIL" }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Score {
    pub conditions: Vec<Condition>,
    pub pass: bool,
}

/// Section 8's four conditions. A record with no verdict has the kind `none` in (c); (b)
/// counts a control sent as `conflict` even when the daemon refused it; (d) counts records, a
/// record refused twice for it once, in the run with `--conflicts`.
pub fn score(set: &Set, with: &RunResult, without: &RunResult) -> Score {
    let planted = set.ids_of(Category::Planted);
    let controls = set.controls();
    let a: Vec<String> = planted
        .iter()
        .filter(|id| {
            let want = set.key[**id].rule.as_deref();
            matches!(with.accepted.get(**id), Some((k, rule)) if k == "conflict" && rule.as_deref() == want)
        })
        .map(|id| id.to_string())
        .collect();
    let b: Vec<String> = controls
        .iter()
        .filter(|id| with.proposed_conflict.contains(**id))
        .map(|id| id.to_string())
        .collect();
    let c: Vec<String> = controls
        .iter()
        .filter(|id| with.kind(id) == without.kind(id))
        .map(|id| id.to_string())
        .collect();
    let d: Vec<String> = with.rule_quote_refused.iter().cloned().collect();
    let records = set.file.record.len();
    let conditions = vec![
        Condition::new(
            "a",
            "planted proposed as conflict against the right rule and accepted",
            a,
            planted.len(),
            6,
            true,
        ),
        Condition::new(
            "b",
            "controls proposed as conflict",
            b,
            controls.len(),
            1,
            false,
        ),
        Condition::new(
            "c",
            "controls given the same verdict kind in both runs",
            c,
            controls.len(),
            14,
            true,
        ),
        Condition::new(
            "d",
            "items refused for a rule quote that is not in the rule",
            d,
            records,
            2,
            false,
        ),
    ];
    let pass = conditions.iter().all(|c| c.pass);
    Score { conditions, pass }
}

/// The records `av dream` printed as refused, round by round, with why.
pub fn refusals(log: &str) -> Vec<Vec<(String, String)>> {
    let mut rounds = vec![vec![]];
    for line in log.lines() {
        if line.contains("refused; asking the agent to correct them") {
            rounds.push(vec![]);
        } else if let Some((rec, why)) = line.split_once(" refused: ")
            && !rec.contains(' ')
        {
            rounds
                .last_mut()
                .expect("one round at least")
                .push((rec.to_owned(), why.to_owned()));
        }
    }
    rounds
}

/// The daemon's refusal of a `{rule, quote}` whose quote is not in that rule.
pub fn rule_quote_refusal(why: &str) -> bool {
    why.split_once("the quote is not in ")
        .is_some_and(|(_, rest)| {
            ["art:", "org:", "conv:"]
                .iter()
                .any(|p| rest.starts_with(p))
                && rest.contains("word for word")
        })
}

fn bare(id: &str) -> String {
    id.trim()
        .trim_start_matches("rec:")
        .replace('-', "")
        .to_lowercase()
}

/// One seeded copy: its repository, the set's record ids by the daemon's, and the set's rule
/// names by handle.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Store {
    pub repo: String,
    /// daemon record id (bare hex) -> the set's id
    pub records: BTreeMap<String, String>,
    /// `art:N` / `conv:N` -> the set's rule name
    pub rules: BTreeMap<String, String>,
}

impl Store {
    /// The set's id of a record an agent or `av dream` named (`rec:` and a prefix, or the id).
    pub fn resolve(&self, named: &str) -> Option<String> {
        let want = bare(named);
        if want.len() < 8 {
            return None;
        }
        let mut hits = self.records.iter().filter(|(id, _)| id.starts_with(&want));
        match (hits.next(), hits.next()) {
            (Some((_, set_id)), None) => Some(set_id.clone()),
            _ => None,
        }
    }
}

fn copy_dir(from: &Path, to: &Path) -> Result<()> {
    std::fs::create_dir_all(to).map_err(|e| format!("{}: {e}", to.display()))?;
    for entry in std::fs::read_dir(from).map_err(|e| format!("{}: {e}", from.display()))? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let dest = to.join(entry.file_name());
        if path.is_dir() {
            copy_dir(&path, &dest)?;
        } else {
            std::fs::copy(&path, &dest).map_err(|e| format!("{}: {e}", path.display()))?;
        }
    }
    Ok(())
}

fn write_record(
    daemon: &Daemon,
    store: &mut Store,
    tasks: &BTreeMap<String, (String, i64)>,
    r: &RecordDef,
) -> Result<()> {
    let (task, generation) = &tasks[&r.agent];
    let rec = daemon.post(
        &format!("/v1/tasks/{task}/records"),
        json!({"kind": r.kind, "summary": r.summary, "body": r.body, "anchors": r.anchors,
               "agent": r.agent}),
        Some(*generation),
    )?;
    let id = rec["id"].as_str().ok_or("a record without an id")?;
    store.records.insert(bare(id), r.id.clone());
    Ok(())
}

/// Seeds one copy as `name`: the fixture repository as one commit, imported; the agents'
/// tasks; the record older than its rule; the articles and conventions, added by the set's
/// person as `av constitution add` and `av convention add` add them; then the other records.
pub fn build_store(
    set: &Set,
    daemon: &Daemon,
    import_dir: &Path,
    import_mount: &str,
    name: &str,
) -> Result<Store> {
    let src = import_dir.join(name);
    if src.exists() {
        return Err(format!("{} exists already", src.display()));
    }
    copy_dir(&set.dir.join("repo"), &src)?;
    git_ok(&src, &["init", "-q", "-b", "main"])?;
    git_ok(&src, &["add", "-A"])?;
    git_ok(&src, &["commit", "-q", "-m", "tidepool"])?;
    daemon.add_repo(
        name,
        &format!("{}/{name}", import_mount.trim_end_matches('/')),
    )?;
    let mut store = Store {
        repo: name.to_owned(),
        ..Store::default()
    };
    let mut tasks = BTreeMap::new();
    for t in &set.file.task {
        let started = daemon.post(
            "/v1/tasks",
            json!({"repo": name, "title": t.title, "goal": t.goal, "agent": t.agent,
                   "session": format!("cc-{}", uuid::Uuid::new_v4())}),
            None,
        )?;
        let id = started["task"]["id"].as_str().ok_or("no task id")?;
        let generation = started["task"]["generation"].as_i64().unwrap_or(1);
        tasks.insert(t.agent.clone(), (id.to_owned(), generation));
    }
    for r in set.file.record.iter().filter(|r| r.before_rules) {
        write_record(daemon, &mut store, &tasks, r)?;
    }
    for a in &set.file.article {
        let r = daemon.post(
            &format!("/v1/repos/{name}/constitution/articles"),
            json!({"title": a.title, "text": a.text, "by": set.file.person, "reason": a.reason}),
            None,
        )?;
        let handle = r["amendment"]["handle"]
            .as_str()
            .ok_or("an article without a handle")?;
        store.rules.insert(handle.to_owned(), a.name.clone());
    }
    for c in &set.file.convention {
        let r = daemon.post(
            &format!("/v1/repos/{name}/conventions"),
            json!({"scope_glob": c.scope_glob, "text": c.text, "required": false,
                   "by": set.file.person}),
            None,
        )?;
        let id = r["id"].as_i64().ok_or("a convention without an id")?;
        store.rules.insert(format!("conv:{id}"), c.name.clone());
    }
    for r in set.file.record.iter().filter(|r| !r.before_rules) {
        write_record(daemon, &mut store, &tasks, r)?;
    }
    Ok(store)
}

/// Exactly the set's rules are in force in `store`: no organisation article of the daemon, no
/// other repository rule.
fn check_rules(daemon: &Daemon, store: &Store) -> Result<()> {
    let repo = &store.repo;
    let c = daemon.get(&format!("/v1/repos/{repo}/constitution"))?;
    let articles: Vec<&str> = c["articles"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|a| a["handle"].as_str())
        .collect();
    let conventions: Vec<String> =
        daemon.get(&format!("/v1/repos/{repo}/conventions"))?["conventions"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|c| c["id"].as_i64())
            .map(|id| format!("conv:{id}"))
            .collect();
    let mut found: Vec<String> = articles.iter().map(|a| a.to_string()).collect();
    found.extend(conventions);
    found.sort();
    let want: Vec<String> = store.rules.keys().cloned().collect();
    if found != want {
        return Err(format!(
            "{repo}: the rules in force are {found:?}, the set's are {want:?} (an organisation \
             article on this daemon?)"
        ));
    }
    Ok(())
}

/// The agent a run starts.
#[derive(Debug, Clone)]
pub enum Agent {
    /// The last freeze's `dreamer.sh` (Claude Code on [`MODEL`], `--max-budget-usd` the
    /// [`CALL_CAP_USD`]), on the key in `CLAUDE_API_KEY`.
    Model { dreamer: PathBuf },
    /// `EXE conflict-fake SET MODE`: no model, no network.
    Fake { exe: PathBuf, mode: FakeMode },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FakeMode {
    /// Proposes what the key says: every planted record a conflict against its rule.
    Keyed,
    /// Names the wrong article for three planted records.
    BreakA,
    /// Also proposes two controls as conflicts.
    BreakB,
    /// Leaves three controls out of the run with `--conflicts`.
    BreakC,
    /// Quotes words that are not in the rule for three planted records, then corrects them.
    BreakD,
}

impl FakeMode {
    pub const ALL: [FakeMode; 5] = [
        FakeMode::Keyed,
        FakeMode::BreakA,
        FakeMode::BreakB,
        FakeMode::BreakC,
        FakeMode::BreakD,
    ];

    pub fn name(self) -> &'static str {
        match self {
            FakeMode::Keyed => "keyed",
            FakeMode::BreakA => "break-a",
            FakeMode::BreakB => "break-b",
            FakeMode::BreakC => "break-c",
            FakeMode::BreakD => "break-d",
        }
    }

    pub fn parse(s: &str) -> Result<FakeMode> {
        FakeMode::ALL
            .into_iter()
            .find(|m| m.name() == s)
            .ok_or_else(|| format!("dry mode {s:?}: keyed, break-a, break-b, break-c or break-d"))
    }
}

fn sh_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', r"'\''"))
}

impl Agent {
    /// The command `av dream` runs in the run's directory; `set_dir` is absolute.
    fn command(&self, set_dir: &Path) -> String {
        match self {
            Agent::Model { dreamer } => format!("sh {}", sh_quote(&dreamer.to_string_lossy())),
            Agent::Fake { exe, mode } => format!(
                "{} conflict-fake {} --mode {}",
                sh_quote(&exe.to_string_lossy()),
                sh_quote(&set_dir.to_string_lossy()),
                mode.name()
            ),
        }
    }

    pub fn describe(&self) -> String {
        match self {
            Agent::Model { .. } => format!(
                "dreamer.sh on {MODEL}, at most ${CALL_CAP_USD:.2} a call, {} fix round",
                FIX_ROUNDS
            ),
            Agent::Fake { mode, .. } => format!("dry: the scripted agent, {}", mode.name()),
        }
    }
}

pub struct CheckArgs {
    pub set_dir: PathBuf,
    pub out: PathBuf,
    pub av_bin: String,
    pub av_url: String,
    pub av_token: Option<String>,
    pub import_dir: PathBuf,
    pub import_mount: String,
    pub agent: Agent,
}

/// What every call of the check's agents cost so far: `total_cost_usd` of each
/// `claude-result-*.json` under `out`, and [`CALL_CAP_USD`] for one that holds none.
pub fn spent(out: &Path) -> f64 {
    fn walk(dir: &Path, total: &mut f64) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().into_owned();
            if path.is_dir() {
                walk(&path, total);
            } else if name.starts_with("claude-result-") && name.ends_with(".json") {
                let cost = std::fs::read_to_string(&path)
                    .ok()
                    .and_then(|t| serde_json::from_str::<Value>(&t).ok())
                    .and_then(|v| v["total_cost_usd"].as_f64());
                *total += cost.unwrap_or(CALL_CAP_USD);
            }
        }
    }
    let mut total = 0.0;
    walk(out, &mut total);
    total
}

/// Whether a run may start: what it can spend at most, every round at its cap, stays inside
/// the ceiling.
pub fn may_start(spent: f64) -> bool {
    spent + CALL_CAP_USD * f64::from(FIX_ROUNDS + 1) <= CEILING_USD + 1e-9
}

const KEEP_ENV: &str = "AV_BENCH_CONFLICT_KEEP";

fn av_dream(args: &CheckArgs, repo: &str, conflicts: bool, dir: &Path) -> Result<String> {
    let home = dir.join("home");
    let keep = dir.join("rounds");
    std::fs::create_dir_all(&keep).map_err(|e| format!("{}: {e}", keep.display()))?;
    // what the agent wrote in each round, kept outside its directory: `av dream` removes it
    // before a fix round
    let cmd = format!(
        "{}; s=$?; cp verdicts.json \"${KEEP_ENV}/round-$AV_DREAM_ROUND.json\" 2>/dev/null; exit $s",
        args.agent.command(&args.set_dir)
    );
    let mut c = Command::new(&args.av_bin);
    c.arg("dream")
        .arg("--repo")
        .arg(repo)
        .arg("--limit")
        .arg(LIMIT.to_string())
        .arg("--timeout")
        .arg(TIMEOUT_SECS.to_string())
        .arg("--fix-rounds")
        .arg(FIX_ROUNDS.to_string());
    if conflicts {
        c.arg("--conflicts");
    }
    c.args(["--", "sh", "-c", &cmd])
        .current_dir(dir)
        .env("AV_URL", &args.av_url)
        .env("AV_AGENT", "dreamer")
        .env("AV_HOME", &home)
        .env(KEEP_ENV, &keep)
        .env_remove("AV_TASK")
        .env_remove("AV_SESSION")
        .env_remove("AV_GENERATION")
        .env_remove("ANTHROPIC_BASE_URL")
        .env_remove("ANTHROPIC_AUTH_TOKEN")
        .env_remove("DEEPSEEK_API_KEY")
        .env_remove("ANTHROPIC_API_KEY")
        .env_remove("CLAUDE_API_KEY");
    match &args.av_token {
        Some(t) => c.env("AV_TOKEN", t),
        None => c.env_remove("AV_TOKEN"),
    };
    if let Agent::Model { .. } = args.agent {
        let key = std::env::var("CLAUDE_API_KEY").map_err(|_| "CLAUDE_API_KEY is not set")?;
        c.env("ANTHROPIC_API_KEY", key)
            .env("M2_MODEL", MODEL)
            .env("M2_RUN_CAP", format!("{CALL_CAP_USD}"))
            .env("DISABLE_AUTOUPDATER", "1");
    }
    let out = c.output().map_err(|e| format!("{}: {e}", args.av_bin))?;
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let log = dir.join("dream.log");
    std::fs::write(&log, &said).map_err(|e| format!("{}: {e}", log.display()))?;
    if !out.status.success() {
        return Err(format!(
            "av dream on {repo} did not finish (its output is in {})",
            log.display()
        ));
    }
    Ok(said)
}

fn read_items(path: &Path) -> Vec<Value> {
    let Some(v) = std::fs::read_to_string(path)
        .ok()
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
    else {
        return vec![];
    };
    let list = if v.is_array() { &v } else { &v["verdicts"] };
    list.as_array().cloned().unwrap_or_default()
}

/// What a run ended with: the daemon's verdicts, `av dream`'s refusals, and what the agent
/// sent in each round (the items `av dream` posted: all of round 1, then the refused ones).
pub fn collect(daemon: &Daemon, store: &Store, dir: &Path, log: &str) -> Result<RunResult> {
    let mut r = RunResult::default();
    let list = daemon.get(&format!("/v1/repos/{}/verdicts?limit=1000", store.repo))?;
    for v in list["verdicts"].as_array().into_iter().flatten() {
        if v["state"] == "retracted" {
            continue;
        }
        let Some(id) = v["record_id"].as_str().and_then(|i| store.resolve(i)) else {
            continue;
        };
        let rule = v["against"]
            .as_str()
            .map(|h| store.rules.get(h).cloned().unwrap_or_else(|| h.to_owned()));
        let kind = v["verdict"].as_str().unwrap_or_default().to_owned();
        r.accepted.insert(id, (kind, rule));
    }
    let rounds = refusals(log);
    let mut refused_in: Vec<BTreeSet<String>> = vec![];
    for round in &rounds {
        let mut ids = BTreeSet::new();
        for (named, why) in round {
            let Some(id) = store.resolve(named) else {
                continue;
            };
            if rule_quote_refusal(why) {
                r.rule_quote_refused.insert(id.clone());
            }
            r.refused.entry(id.clone()).or_default().push(why.clone());
            ids.insert(id);
        }
        refused_in.push(ids);
    }
    for n in 1..=FIX_ROUNDS + 1 {
        for item in read_items(&dir.join("rounds").join(format!("round-{n}.json"))) {
            let Some(id) = item["record"].as_str().and_then(|i| store.resolve(i)) else {
                continue;
            };
            let posted = n == 1
                || refused_in
                    .get(n as usize - 2)
                    .is_some_and(|ids| ids.contains(&id));
            if posted && item["verdict"] == "conflict" {
                r.proposed_conflict.insert(id);
            }
        }
    }
    Ok(r)
}

/// The run's one review dream, from the `bundle.json` `av dream` kept under `dir/home`.
fn bundle_of(dir: &Path) -> Result<Value> {
    let runs = dir.join("home").join("dreams");
    let mut found = vec![];
    for entry in std::fs::read_dir(&runs).map_err(|e| format!("{}: {e}", runs.display()))? {
        let path = entry.map_err(|e| e.to_string())?.path().join("bundle.json");
        if let Ok(text) = std::fs::read_to_string(&path) {
            found.push(serde_json::from_str::<Value>(&text).map_err(|e| e.to_string())?);
        }
    }
    match found.len() {
        1 => Ok(found.remove(0)),
        n => Err(format!("{}: {n} dreams, not 1", runs.display())),
    }
}

/// The run was handed all 24 records with their code, and the rules only when it asked.
fn check_bundle(set: &Set, b: &Value, conflicts: bool) -> Result<String> {
    let items = b["items"].as_array().cloned().unwrap_or_default();
    let handed: BTreeSet<&str> = items
        .iter()
        .filter_map(|i| i["record"]["summary"].as_str())
        .collect();
    let seeded: BTreeSet<&str> = set.file.record.iter().map(|r| r.summary.as_str()).collect();
    if handed != seeded || items.len() != seeded.len() {
        return Err(format!(
            "the run was handed {} records, not the set's {}",
            items.len(),
            seeded.len()
        ));
    }
    for i in &items {
        let summary = i["record"]["summary"].as_str().unwrap_or_default();
        if i["code"]
            .as_array()
            .into_iter()
            .flatten()
            .any(|c| c["text"].is_null())
        {
            return Err(format!("no code was found for an anchor of {summary:?}"));
        }
        if i.get("rules").is_some() != conflicts {
            return Err(format!(
                "{summary:?} was handed rules in a run {} --conflicts",
                if conflicts { "with" } else { "without" }
            ));
        }
    }
    Ok(b["dream"]["id"].as_str().unwrap_or_default().to_owned())
}

#[derive(Debug, Clone, Serialize)]
pub struct Outcome {
    pub score: Score,
    pub spent_usd: f64,
    pub report: String,
}

/// The whole check: seed two copies, one review run on each, score. Refuses an `out` that
/// holds a finished check; one that holds an interrupted one counts what it spent.
pub fn check(args: &CheckArgs) -> Result<Outcome> {
    let set = Set::load(&args.set_dir)?;
    set.check()?;
    if args.out.join("result.json").exists() {
        return Err(format!(
            "{} holds a finished check: give another --out",
            args.out.display()
        ));
    }
    if matches!(args.agent, Agent::Model { .. }) && std::env::var_os("CLAUDE_API_KEY").is_none() {
        return Err("a run on the model needs CLAUDE_API_KEY".into());
    }
    std::fs::create_dir_all(&args.out).map_err(|e| format!("{}: {e}", args.out.display()))?;
    let daemon = Daemon::new(&args.av_url, args.av_token.as_deref());
    daemon.preflight()?;
    let run = format!("cc{}", &uuid::Uuid::new_v4().simple().to_string()[..8]);
    let copies = [("conflicts", true), ("plain", false)];
    let mut stores = vec![];
    for (copy, _) in copies {
        let store = build_store(
            &set,
            &daemon,
            &args.import_dir,
            &args.import_mount,
            &format!("{run}-{copy}"),
        )?;
        check_rules(&daemon, &store)?;
        stores.push(store);
    }
    let mut results = vec![];
    let mut shas = vec![];
    for ((copy, conflicts), store) in copies.iter().zip(&stores) {
        let spent_now = spent(&args.out);
        if !may_start(spent_now) {
            return Err(format!(
                "stopped: ${spent_now:.4} spent, and the {copy} run could take it past \
                 ${CEILING_USD:.2}"
            ));
        }
        let dir = args.out.join(&run).join(copy);
        let log = av_dream(args, &store.repo, *conflicts, &dir)?;
        let dream = check_bundle(&set, &bundle_of(&dir)?, *conflicts)?;
        let sha = daemon.get(&format!("/v1/dreams/{dream}"))?["dream"]["instructions_sha"]
            .as_str()
            .unwrap_or_default()
            .to_owned();
        if (sha == PLAIN_SHA) == *conflicts {
            return Err(format!(
                "the {copy} run was handed instructions {sha}; the plain DREAM.md is {PLAIN_SHA}"
            ));
        }
        shas.push(sha);
        results.push(collect(&daemon, store, &dir, &log)?);
    }
    let score = score(&set, &results[0], &results[1]);
    let spent_usd = spent(&args.out);
    let mut report = format!(
        "conflict check {run} ({}), set sha256 {}\n",
        args.agent.describe(),
        set.sha()?
    );
    for c in &score.conditions {
        report.push_str(&c.line());
        report.push('\n');
    }
    report.push_str(&format!(
        "overall: {}\nspent: ${spent_usd:.4} of ${CEILING_USD:.2}\n",
        if score.pass { "pass" } else { "FAIL" }
    ));
    let rows: Vec<Value> = set
        .file
        .record
        .iter()
        .map(|r| {
            let k = &set.key[&r.id];
            json!({"id": r.id, "category": k.category, "rule": k.rule,
                   "with": results[0].accepted.get(&r.id), "without": results[1].accepted.get(&r.id),
                   "proposed_conflict": results[0].proposed_conflict.contains(&r.id),
                   "refused_with": results[0].refused.get(&r.id),
                   "refused_without": results[1].refused.get(&r.id)})
        })
        .collect();
    let result = json!({"run": run, "agent": args.agent.describe(), "set_sha256": set.sha()?,
                        "stores": stores, "instructions_sha": shas, "records": rows,
                        "score": score, "spent_usd": spent_usd});
    let text = serde_json::to_string_pretty(&result).map_err(|e| e.to_string())?;
    std::fs::write(args.out.join("result.json"), text).map_err(|e| e.to_string())?;
    std::fs::write(args.out.join("report.txt"), &report).map_err(|e| e.to_string())?;
    Ok(Outcome {
        score,
        spent_usd,
        report,
    })
}

fn code_quote(item: &Value) -> Option<Value> {
    item["code"].as_array().into_iter().flatten().find_map(|c| {
        let text = c["text"].as_str()?;
        let line = text
            .lines()
            .map(str::trim)
            .find(|l| l.chars().filter(|c| !c.is_whitespace()).count() >= 12)?;
        Some(json!({"code": c["path"], "quote": line}))
    })
}

/// The scripted agent of a dry run, in the run's directory: reads `bundle.json`, writes
/// `verdicts.json`. Every record is `confirmed` with a line of its code, except what the key
/// and `mode` make a `conflict` in a run that was handed rules.
pub fn fake_agent(set: &Set, mode: FakeMode, dir: &Path) -> Result<()> {
    let bundle: Value = serde_json::from_str(&read(&dir.join("bundle.json"))?)
        .map_err(|e| format!("bundle.json: {e}"))?;
    let conflicts = bundle["dream"]["worklist"]["rules"] == true;
    let round: u32 = std::env::var("AV_DREAM_ROUND")
        .ok()
        .and_then(|r| r.parse().ok())
        .unwrap_or(1);
    let by_summary: BTreeMap<&str, &str> = set
        .file
        .record
        .iter()
        .map(|r| (r.summary.as_str(), r.id.as_str()))
        .collect();
    let planted = set.ids_of(Category::Planted);
    let first = |ids: &[&str], n: usize| -> Vec<String> {
        ids.iter().take(n).map(|s| s.to_string()).collect()
    };
    let wrong_rule = if mode == FakeMode::BreakA {
        first(&planted, 3)
    } else {
        vec![]
    };
    let bad_quote = if mode == FakeMode::BreakD && round == 1 {
        first(&planted, 3)
    } else {
        vec![]
    };
    let extra = if mode == FakeMode::BreakB {
        first(&set.ids_of(Category::Silent), 2)
    } else {
        vec![]
    };
    let left_out = if mode == FakeMode::BreakC {
        first(&set.ids_of(Category::Untouched), 3)
    } else {
        vec![]
    };
    let mut out = vec![];
    for item in bundle["items"].as_array().into_iter().flatten() {
        let rec = item["record"]["id"].as_str().unwrap_or_default();
        let summary = item["record"]["summary"].as_str().unwrap_or_default();
        let Some(id) = by_summary.get(summary).map(|s| s.to_string()) else {
            continue;
        };
        let k = &set.key[&id];
        if conflicts && left_out.contains(&id) {
            continue;
        }
        let conflict = conflicts && (k.category == Category::Planted || extra.contains(&id));
        if !conflict {
            let Some(quote) = code_quote(item) else {
                continue;
            };
            out.push(json!({"record": rec, "verdict": "confirmed", "evidence": [quote]}));
            continue;
        }
        let rules: Vec<&Value> = item["rules"].as_array().into_iter().flatten().collect();
        let rule_name = k.rule.as_deref().unwrap_or_default();
        let text = set.rule_text(rule_name).unwrap_or_default();
        let keyed = rules.iter().find(|r| r["text"] == text);
        let rule = if wrong_rule.contains(&id) {
            rules
                .iter()
                .find(|r| r["text"] != text && r["title"].is_string())
        } else {
            keyed
        };
        let Some(rule) = rule else {
            return Err(format!("{id}: its rule is not among the item's rules"));
        };
        let quote = if bad_quote.contains(&id) {
            "words that this rule does not contain at all".to_owned()
        } else {
            rule["text"].as_str().unwrap_or_default().to_owned()
        };
        out.push(
            json!({"record": rec, "verdict": "conflict", "against": rule["handle"],
                        "reason": k.why,
                        "evidence": [{"record": rec, "quote": summary},
                                     {"rule": rule["handle"], "quote": quote}]}),
        );
    }
    let text =
        serde_json::to_string_pretty(&json!({ "verdicts": out })).map_err(|e| e.to_string())?;
    std::fs::write(dir.join("verdicts.json"), text).map_err(|e| format!("verdicts.json: {e}"))
}
