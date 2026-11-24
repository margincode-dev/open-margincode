//! Dreams (Plan 16 Phase 2): an offline look at the records agents wrote. An agent started by
//! the CLI is handed a short list of records the daemon chose, judges them, and posts
//! **verdicts**; a second run, or a person, reviews each one before it has any effect.
//!
//! The daemon runs no model here and no repository command. It chooses the list (a pure
//! ranking of stored facts), checks every piece of evidence a verdict cites (the quote must be
//! found word for word in the file, record or event it names) and applies the effects through
//! the packer. A dream writes no record, edits none and changes no code; a verdict is a new
//! row that one call retracts.

use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

use axum::extract::{Path, Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use margincode_core::standing::{Origin, VerdictKind, standing};
use margincode_core::*;
use margincode_store::{Dream, NewDream, NewVerdict, PgStore, Promoted, VerdictRow};
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use crate::AppState;
use crate::error::ApiResult;

const WORKLIST_DEFAULT: usize = 20;
const WORKLIST_MAX: usize = 50;
const WORKLIST_SCAN: i64 = 400;
/// Packs of this many days back count toward "shown".
const SHOWN_DAYS: i32 = 30;
/// A quote must have at least this many characters that are not white space: one word found
/// somewhere in a file shows nothing.
const MIN_QUOTE_CHARS: usize = 12;
/// Anchored code handed to a dream is clipped to this many characters per anchor.
const CODE_CLIP: usize = 6000;
/// A promoted convention's text: "the rule, in a line or two". Once accepted it is never
/// dropped from a pack (migration 0032 holds the store to it too).
const PROMOTE_TEXT_MAX: usize = 400;
/// `AV_DREAM_LEASE_SECS`: how long a run may take. A running dream past it is failed by the
/// next start in its repository, so one that died holds no area.
static LEASE_SECS: LazyLock<u64> = LazyLock::new(|| {
    std::env::var("AV_DREAM_LEASE_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(3600)
});

pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .route("/v1/repos/{repo}/dreams", post(start))
        .route("/v1/repos/{repo}/verdicts", get(list))
        .route("/v1/dreams/{id}", get(show))
        .route("/v1/dreams/{id}/verdicts", post(propose))
        .route("/v1/dreams/{id}/finish", post(finish))
        .route("/v1/dreams/{id}/undo", post(undo))
        .route("/v1/verdicts/{id}", get(show_verdict))
        .route("/v1/verdicts/{id}/review", post(review))
        .route("/v1/verdicts/{id}/retract", post(retract))
}

fn rec_handle(id: RecordId) -> String {
    format!("rec:{}", &id.0.simple().to_string()[..8])
}

fn vd_handle(id: Uuid) -> String {
    format!("vd:{}", &id.simple().to_string()[..8])
}

fn verdict_json(v: &VerdictRow) -> Value {
    let mut out = json!(v);
    if out["convention"].is_null() {
        out.as_object_mut().map(|o| o.remove("convention"));
    }
    out["handle"] = json!(vd_handle(v.id));
    out["record"] = json!(rec_handle(v.record_id));
    out
}

async fn feed(st: &AppState, repo: &str, kind: &str, payload: Value) {
    if let Err(e) = st.store.add_event(repo, kind, None, payload).await {
        tracing::warn!(error = %e, kind, "dream event not written");
    }
}

async fn repo_of(st: &AppState, rec: &Record) -> Result<String> {
    Ok(st.store.get_task(rec.task_id).await?.repo)
}

#[derive(Deserialize)]
struct StartBody {
    agent: String,
    /// `review` (the default) or `verify`.
    mode: Option<String>,
    /// A path prefix; the whole repository without it.
    area: Option<String>,
    limit: Option<usize>,
    /// Hash of the instructions the agent was given, so a result can be traced to them.
    instructions_sha: Option<String>,
    /// A review run that may propose `conflict` verdicts (`av dream --conflicts`): each item
    /// also carries the rules that cover its record.
    #[serde(default)]
    rules: bool,
}

/// The rules in force that cover `rec`, as a run that asked is handed them: the articles
/// (organisation first), then the conventions whose glob matches one of its anchor paths.
async fn rules_of(st: &AppState, repo: &str, rec: &Record) -> Result<Vec<Convention>> {
    let paths: Vec<String> = rec.anchors.iter().map(|a| a.path.clone()).collect();
    st.pg.rules_for(repo, &paths).await
}

fn rule_json(c: &Convention) -> Value {
    let mut out = json!({"handle": c.handle(), "text": c.text, "scope_glob": c.scope_glob});
    if let Some(title) = &c.title {
        out["title"] = json!(title);
    }
    out
}

fn clean_area(area: Option<&str>) -> Result<String> {
    let area = area.unwrap_or("").trim().trim_matches('/');
    if area.split('/').any(|seg| seg == ".." || seg == ".") {
        return Err(AvError::Invalid(format!(
            "area {area:?} must be a path inside the repository"
        )));
    }
    Ok(area.to_owned())
}

/// Whether the code a record is anchored to differs from what was hashed when it was written
/// (or, under a `confirmed` verdict, from what that verdict saw).
fn anchors_changed(then: &[Option<String>], now: &[Option<String>]) -> bool {
    then.iter()
        .zip(now)
        .any(|(then, now)| then.is_some() && then != now)
}

/// A dream's list: the candidates ranked by where a second look is most likely to pay.
/// `3·[its task's merge was reverted, or it is a decision of abandoned work]
///  + 2·[its anchored code changed since it was recorded or last confirmed]
///  + 2·[another live record of its kind shares an anchor with it]
///  + min(packs of the last 30 days that showed it, 10) / 5
///  + 1·[it never had a verdict]`, ties by id.
///
/// A review run that finished (`done`) and was handed a record without writing a verdict on
/// it has declined it, at the score the record had in that run (the run's `worklist.scores`,
/// `DreamCandidate.declined_score`). A record waits behind every other record while its score
/// is no higher than the highest score it was declined at. Among waiting records, and among
/// the others, the score decides, ties by id. So runs back to back with a small `limit` are
/// handed each record in turn instead of the same declined one every time, and a declined
/// record whose score rises (its merge reverted, its code changed) leads again.
async fn worklist(
    st: &AppState,
    repo: &str,
    area: &str,
    trunk: &ChangeId,
    limit: usize,
) -> Result<Vec<(Record, f32)>> {
    let candidates = st.pg.dream_candidates(repo, area, WORKLIST_SCAN).await?;
    let ids: Vec<RecordId> = candidates.iter().map(|c| c.record.id).collect();
    let origins = st.store.record_origins(&ids).await?;
    let shown = st.pg.records_shown(repo, SHOWN_DAYS).await?;
    let mut shared: HashMap<(RecordKind, &Anchor), usize> = HashMap::new();
    for c in &candidates {
        for a in &c.record.anchors {
            *shared.entry((c.record.kind, a)).or_default() += 1;
        }
    }
    let mut ranked = Vec::with_capacity(candidates.len());
    for c in &candidates {
        let rec = &c.record;
        let facts = origins.get(&rec.id);
        let origin = facts.map(|f| standing(f, rec.kind, rec.created_at).origin);
        // an agreed verdict has judged the failure of the record's origin
        let judged = facts.is_some_and(|f| f.verdict.is_some());
        let failed = !judged
            && (origin == Some(Origin::Reverted)
                || (origin == Some(Origin::Abandoned) && rec.kind == RecordKind::Decision));
        let now = st
            .packer
            .anchor_hashes_on_main(repo, trunk, &rec.anchors)
            .await;
        let then = match facts.and_then(|f| f.verdict.as_ref()) {
            Some(v)
                if v.kind == VerdictKind::Confirmed
                    && v.anchor_hashes.len() == rec.anchors.len() =>
            {
                &v.anchor_hashes
            }
            _ => &rec.anchor_hashes,
        };
        let clustered = rec
            .anchors
            .iter()
            .any(|a| shared.get(&(rec.kind, a)).copied().unwrap_or(0) > 1);
        let seen = shown.get(&rec_handle(rec.id)).copied().unwrap_or(0).min(10) as f32;
        let score = 3.0 * f32::from(u8::from(failed))
            + 2.0 * f32::from(u8::from(anchors_changed(then, &now)))
            + 2.0 * f32::from(u8::from(clustered))
            + seen / 5.0
            + f32::from(u8::from(!c.had_verdict));
        let waits = c.declined_score.is_some_and(|at| at >= score);
        ranked.push((waits, rec.clone(), score));
    }
    ranked.sort_by(|a, b| {
        a.0.cmp(&b.0)
            .then_with(|| b.2.total_cmp(&a.2))
            .then_with(|| a.1.id.0.cmp(&b.1.id.0))
    });
    ranked.truncate(limit);
    Ok(ranked
        .into_iter()
        .map(|(_, rec, score)| (rec, score))
        .collect())
}

fn clip(text: String) -> String {
    match text.char_indices().nth(CODE_CLIP) {
        Some((at, _)) => format!("{}\n… (clipped)", &text[..at]),
        None => text,
    }
}

/// What a dream is told about one record, word for word from the store and from `main`.
async fn bundle_item(st: &AppState, repo: &str, trunk: &ChangeId, rec: &Record) -> Result<Value> {
    let task = st.store.get_task(rec.task_id).await?;
    let facts = st.store.record_origins(&[rec.id]).await?;
    let stands = facts
        .get(&rec.id)
        .map(|f| standing(f, rec.kind, rec.created_at));
    let mut code = vec![];
    for a in &rec.anchors {
        let text = st.packer.anchored_code_on_main(repo, trunk, a).await;
        code.push(json!({"path": a.path, "symbol": a.symbol, "text": text.map(clip)}));
    }
    let paths: Vec<String> = rec.anchors.iter().map(|a| a.path.clone()).collect();
    let symbols: Vec<String> = rec
        .anchors
        .iter()
        .filter_map(|a| a.symbol.clone())
        .collect();
    let others: Vec<Value> = if paths.is_empty() {
        vec![]
    } else {
        st.store
            .records_anchored_to(repo, &paths, &symbols)
            .await?
            .into_iter()
            .filter(|o| o.id != rec.id)
            .map(|o| {
                json!({"id": rec_handle(o.id), "kind": o.kind, "summary": o.summary,
                       "body": o.body, "anchors": o.anchors, "author": o.author_agent})
            })
            .collect()
    };
    let verdicts: Vec<Value> = st
        .pg
        .list_verdicts(repo, None, None, Some(rec.id), 20)
        .await?
        .iter()
        .map(verdict_json)
        .collect();
    Ok(json!({
        "record": {"id": rec_handle(rec.id), "kind": rec.kind, "summary": rec.summary,
                   "body": rec.body, "anchors": rec.anchors, "author": rec.author_agent,
                   "created_at": rec.created_at, "standing": stands},
        "task": {"id": task.id, "slug": task.slug, "title": task.title, "goal": task.goal,
                 "status": task.status, "events": st.pg.task_events(task.id, 50).await?},
        "code": code,
        "others": others,
        "verdicts": verdicts,
    }))
}

/// Starts a run and answers what it is to look at. `review`: the worklist's records, each with
/// its task, the anchored code on `main` now, the other live records on the same anchors and
/// earlier verdicts. `verify`: the proposed verdicts that wait for a review, each with its
/// record and that code. Nothing to look at: no run is made (`dream: null`).
async fn start(
    State(st): State<AppState>,
    Path(repo): Path<String>,
    Json(b): Json<StartBody>,
) -> ApiResult<Value> {
    let trunk = st.vcs.trunk(&repo).await?;
    let mode = b.mode.as_deref().unwrap_or("review");
    let area = clean_area(b.area.as_deref())?;
    let limit = b.limit.unwrap_or(WORKLIST_DEFAULT).clamp(1, WORKLIST_MAX);
    if b.agent.trim().is_empty() {
        return Err(AvError::Invalid("agent must not be empty".into()).into());
    }
    if b.rules && mode != "review" {
        return Err(AvError::Invalid(
            "rules (`av dream --conflicts`) are for review runs only".into(),
        )
        .into());
    }
    let (worklist_json, items) = match mode {
        "review" => {
            let ranked = worklist(&st, &repo, &area, &trunk, limit).await?;
            let mut items = Vec::with_capacity(ranked.len());
            for (rec, score) in &ranked {
                let mut item = bundle_item(&st, &repo, &trunk, rec).await?;
                item["score"] = json!(score);
                if b.rules {
                    let rules = rules_of(&st, &repo, rec).await?;
                    item["rules"] = rules.iter().map(rule_json).collect();
                }
                items.push(item);
            }
            let ids: Vec<RecordId> = ranked.iter().map(|(r, _)| r.id).collect();
            let scores: serde_json::Map<String, Value> = ranked
                .iter()
                .map(|(r, score)| (r.id.0.to_string(), json!(score)))
                .collect();
            let mut worklist = json!({"commit": trunk.0, "records": ids, "scores": scores});
            if b.rules {
                worklist["rules"] = json!(true);
            }
            (worklist, items)
        }
        "verify" => {
            // a promote is reviewed by name, never by a run: no run is handed it
            let mut waiting = st.pg.proposed_for_verify(&repo, 1000).await?;
            waiting.reverse(); // oldest first
            let mut items = vec![];
            let mut ids = vec![];
            for v in waiting {
                if ids.len() == limit {
                    break;
                }
                let rec = st.store.get_record(v.record_id).await?;
                let inside = area.is_empty()
                    || rec.anchors.iter().any(|a| {
                        margincode_store::areas_overlap(&a.path, &area)
                            && a.path.len() >= area.len()
                    });
                if !inside {
                    continue;
                }
                let mut item = bundle_item(&st, &repo, &trunk, &rec).await?;
                item["verdict"] = verdict_json(&v);
                item.as_object_mut().map(|o| o.remove("verdicts"));
                ids.push(v.id);
                items.push(item);
            }
            (json!({"commit": trunk.0, "verdicts": ids}), items)
        }
        other => {
            return Err(AvError::Invalid(format!("mode {other:?} is not review or verify")).into());
        }
    };
    if items.is_empty() {
        return Ok(Json(json!({"dream": null, "items": [], "commit": trunk.0})));
    }
    let dream = st
        .pg
        .create_dream(NewDream {
            repo: &repo,
            mode,
            area: &area,
            agent: b.agent.trim(),
            instructions_sha: b.instructions_sha.as_deref(),
            worklist: worklist_json,
            lease_secs: *LEASE_SECS,
        })
        .await?;
    feed(
        &st,
        &repo,
        "dream:started",
        json!({"dream": dream.id, "mode": mode, "area": area, "agent": dream.agent,
               "items": items.len()}),
    )
    .await;
    Ok(Json(
        json!({"dream": dream, "items": items, "commit": trunk.0}),
    ))
}

enum Cited {
    Code,
    Record(RecordId),
    Rule(ConventionId),
    Event,
}

fn quoted(text: &str, quote: &str) -> bool {
    let norm = |s: &str| s.replace("\r\n", "\n");
    norm(text).contains(norm(quote).trim_matches('\n'))
}

/// Checks one evidence entry. `{code: path, commit?, quote}`: the quote is in that file at
/// the commit (the run's own when none is given). `{record: id, quote}`: in that record's
/// summary or body, a record of this repository. `{rule: handle, quote}`: in the title or text
/// of that rule as it is in force (`art:N`, `org:N`, `conv:<id>`). `{event: id}`: an event of
/// this repository. The refusal says what was not found; nothing is corrected or matched
/// loosely.
async fn check_evidence(
    st: &AppState,
    repo: &str,
    commit: &ChangeId,
    e: &Value,
    rules: bool,
) -> std::result::Result<Cited, String> {
    let quote = || -> std::result::Result<&str, String> {
        let q = e["quote"].as_str().unwrap_or("");
        if q.chars().filter(|c| !c.is_whitespace()).count() < MIN_QUOTE_CHARS {
            return Err(format!(
                "a quote needs at least {MIN_QUOTE_CHARS} characters that are not white space"
            ));
        }
        Ok(q)
    };
    if let Some(path) = e["code"].as_str() {
        let quote = quote()?;
        let at = match e["commit"].as_str() {
            Some(c) => ChangeId(c.to_owned()),
            None => commit.clone(),
        };
        let bytes = st
            .vcs
            .read_file(repo, &at, path)
            .await
            .map_err(|_| format!("{path} is not a file of commit {}", at.0))?;
        if !quoted(&String::from_utf8_lossy(&bytes), quote) {
            return Err(format!(
                "the quote is not in {path} at commit {} word for word",
                at.0
            ));
        }
        return Ok(Cited::Code);
    }
    if let Some(named) = e["record"].as_str() {
        let quote = quote()?;
        let rec = st
            .store
            .find_record(named.trim().strip_prefix("rec:").unwrap_or(named.trim()))
            .await
            .map_err(|e| e.to_string())?;
        if repo_of(st, &rec).await.map_err(|e| e.to_string())? != repo {
            return Err(format!("{named} is not a record of repo {repo}"));
        }
        if !quoted(&rec.summary, quote) && !quoted(&rec.body, quote) {
            return Err(format!("the quote is not in {named} word for word"));
        }
        return Ok(Cited::Record(rec.id));
    }
    if let Some(named) = e["rule"].as_str().filter(|_| rules) {
        let quote = quote()?;
        let rule = st
            .pg
            .rule_in_force(repo, named)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| format!("{} is not a rule in force in repo {repo}", named.trim()))?;
        let in_title = rule.title.as_deref().is_some_and(|t| quoted(t, quote));
        if !in_title && !quoted(&rule.text, quote) {
            return Err(format!(
                "the quote is not in {} word for word",
                rule.handle()
            ));
        }
        return Ok(Cited::Rule(rule.id));
    }
    if let Some(id) = e["event"].as_i64() {
        return match st.pg.event_in_repo(repo, id).await {
            Ok(true) => Ok(Cited::Event),
            Ok(false) => Err(format!("event {id} is not an event of repo {repo}")),
            Err(e) => Err(e.to_string()),
        };
    }
    Err(if rules {
        "an evidence entry is {code, quote}, {record, quote}, {rule, quote} or {event}"
    } else {
        "an evidence entry is {code, quote}, {record, quote} or {event}"
    }
    .into())
}

async fn check_all(
    st: &AppState,
    repo: &str,
    commit: &ChangeId,
    evidence: &[Value],
    rules: bool,
) -> std::result::Result<Vec<Cited>, String> {
    let mut cited = Vec::with_capacity(evidence.len());
    for (i, e) in evidence.iter().enumerate() {
        cited.push(
            check_evidence(st, repo, commit, e, rules)
                .await
                .map_err(|why| format!("evidence[{i}]: {why}"))?,
        );
    }
    Ok(cited)
}

#[derive(Deserialize)]
struct ProposeBody {
    verdicts: Vec<Value>,
}

fn worklist_ids(d: &Dream, key: &str) -> HashSet<Uuid> {
    d.worklist[key]
        .as_array()
        .map(|a| a.iter().filter_map(|v| v.as_str()?.parse().ok()).collect())
        .unwrap_or_default()
}

fn dream_commit(d: &Dream) -> ChangeId {
    ChangeId(d.worklist["commit"].as_str().unwrap_or_default().to_owned())
}

fn live(d: &Dream, mode: &str) -> Result<()> {
    if d.mode != mode {
        return Err(AvError::IllegalState(format!(
            "dream {} is a {} run, not a {mode} run",
            d.id, d.mode
        )));
    }
    if !PgStore::dream_live(d) {
        return Err(AvError::IllegalState(format!(
            "dream {} is not running (status {}, lease until {})",
            d.id, d.status, d.lease_expires
        )));
    }
    Ok(())
}

/// One proposed verdict, checked whole before it is written. The refusal is the answer for
/// this item; the other items of the call are not affected.
async fn propose_one(
    st: &AppState,
    d: &Dream,
    on_list: &HashSet<Uuid>,
    item: &Value,
) -> std::result::Result<VerdictRow, String> {
    let named = item["record"].as_str().ok_or("record is missing")?.trim();
    let rec = st
        .store
        .find_record(named.strip_prefix("rec:").unwrap_or(named))
        .await
        .map_err(|e| e.to_string())?;
    if !on_list.contains(&rec.id.0) {
        return Err(format!(
            "{} is not on this run's worklist; a dream judges only what it was handed",
            rec_handle(rec.id)
        ));
    }
    let named_kind = item["verdict"].as_str().unwrap_or("");
    let kind = VerdictKind::parse(named_kind);
    if kind.is_none() && !matches!(named_kind, "promote" | "conflict") {
        return Err(if asked_for_rules(d) {
            "verdict must be confirmed, disputed, duplicate, outdated, promote or conflict"
        } else {
            "verdict must be confirmed, disputed, duplicate, outdated or promote"
        }
        .into());
    }
    let against = match named_kind {
        "conflict" => Some(conflict_rule(st, d, &rec, item).await?),
        _ => None,
    };
    let reason = item["reason"].as_str().unwrap_or("").trim();
    if reason.chars().count() > MAX_SUMMARY_CHARS {
        return Err(format!(
            "reason is {} characters; at most {MAX_SUMMARY_CHARS}",
            reason.chars().count()
        ));
    }
    if kind == Some(VerdictKind::Disputed) && reason.is_empty() {
        return Err("a disputed verdict needs a reason: it is printed with the record".into());
    }
    if against.is_some() && reason.is_empty() {
        return Err(
            "a conflict needs a reason (what the record says against the rule): it is printed \
             with the record"
                .into(),
        );
    }
    let evidence = item["evidence"].as_array().cloned().unwrap_or_default();
    let cited = check_all(st, &d.repo, &dream_commit(d), &evidence, asked_for_rules(d)).await?;
    let has_code = cited.iter().any(|c| matches!(c, Cited::Code));
    let has_event = cited.iter().any(|c| matches!(c, Cited::Event));
    let cites = |id: RecordId| {
        cited
            .iter()
            .any(|c| matches!(c, Cited::Record(r) if *r == id))
    };
    let mut canonical = None;
    let mut hashes = json!([]);
    if let Some(rule) = &against {
        if !cites(rec.id) {
            return Err(format!(
                "a conflict needs a quote of {} itself (a `record` quote)",
                rec_handle(rec.id)
            ));
        }
        if !cited
            .iter()
            .any(|c| matches!(c, Cited::Rule(r) if *r == rule.id))
        {
            return Err(format!(
                "a conflict needs a quote of {} (a `rule` quote of the rule it names)",
                rule.handle()
            ));
        }
        return st
            .pg
            .insert_verdict(NewVerdict {
                record: rec.id,
                verdict: "conflict",
                reason,
                canonical: None,
                retracts: None,
                evidence: json!(evidence),
                anchor_hashes: json!([]),
                author_agent: &d.agent,
                dream: Some(d.id),
                convention: None,
                against: Some(rule.id),
            })
            .await
            .map_err(refusal);
    }
    let Some(kind) = kind else {
        let convention = promotion(&rec, item, cites(rec.id))?;
        return st
            .pg
            .insert_verdict(NewVerdict {
                record: rec.id,
                verdict: "promote",
                reason,
                canonical: None,
                retracts: None,
                evidence: json!(evidence),
                anchor_hashes: json!([]),
                author_agent: &d.agent,
                dream: Some(d.id),
                convention: Some(convention),
                against: None,
            })
            .await
            .map_err(refusal);
    };
    match kind {
        VerdictKind::Confirmed => {
            if !has_code {
                return Err(
                    "a confirmed verdict needs a quote of the code it was checked \
                            against"
                        .into(),
                );
            }
            let trunk = st.vcs.trunk(&d.repo).await.map_err(|e| e.to_string())?;
            hashes = json!(
                st.packer
                    .anchor_hashes_on_main(&d.repo, &trunk, &rec.anchors)
                    .await
            );
        }
        VerdictKind::Disputed | VerdictKind::Outdated => {
            if !has_code && !has_event {
                return Err(format!(
                    "a {} verdict needs a quote of code or an event: another record's word \
                     is not enough",
                    kind.as_str()
                ));
            }
        }
        VerdictKind::Duplicate => {
            let named = item["canonical"]
                .as_str()
                .ok_or("a duplicate verdict names the canonical record")?
                .trim();
            let other = st
                .store
                .find_record(named.strip_prefix("rec:").unwrap_or(named))
                .await
                .map_err(|e| e.to_string())?;
            if other.id == rec.id {
                return Err("a record is not a duplicate of itself".into());
            }
            if other.superseded_by.is_some() {
                return Err(format!("{} is superseded", rec_handle(other.id)));
            }
            if repo_of(st, &other).await.map_err(|e| e.to_string())? != d.repo {
                return Err(format!(
                    "{} is not a record of this repo",
                    rec_handle(other.id)
                ));
            }
            let hidden = st
                .store
                .record_origins(&[other.id])
                .await
                .map_err(|e| e.to_string())?
                .get(&other.id)
                .and_then(|f| f.verdict.as_ref())
                .is_some_and(|v| v.kind.hides());
            if hidden {
                return Err(format!(
                    "{} is itself hidden by a verdict; name the record that is shown",
                    rec_handle(other.id)
                ));
            }
            if other.kind != rec.kind || !other.anchors.iter().any(|a| rec.anchors.contains(a)) {
                return Err(format!(
                    "{} is not of the same kind and anchor as {}",
                    rec_handle(other.id),
                    rec_handle(rec.id)
                ));
            }
            if !cites(rec.id) || !cites(other.id) {
                return Err(
                    "a duplicate verdict needs a quote from each of the two records".into(),
                );
            }
            canonical = Some(other.id);
        }
    }
    st.pg
        .insert_verdict(NewVerdict {
            record: rec.id,
            verdict: kind.as_str(),
            reason,
            canonical,
            retracts: None,
            evidence: json!(evidence),
            anchor_hashes: hashes,
            author_agent: &d.agent,
            dream: Some(d.id),
            convention: None,
            against: None,
        })
        .await
        .map_err(refusal)
}

fn asked_for_rules(d: &Dream) -> bool {
    d.worklist["rules"] == true
}

/// The rule a `conflict` names in `against`, checked here and not taken from the list the run
/// was handed: the run asked for rules, the record is not an attempt (an attempt reports what
/// was tried and tells nobody what to do), and `against` is a rule in force that covers it.
async fn conflict_rule(
    st: &AppState,
    d: &Dream,
    rec: &Record,
    item: &Value,
) -> std::result::Result<Convention, String> {
    if !asked_for_rules(d) {
        return Err(
            "this run was not asked to check records against rules; a conflict needs a run \
             started with `av dream --conflicts`"
                .into(),
        );
    }
    if rec.kind == RecordKind::Attempt {
        return Err(format!(
            "{} is an attempt: it reports what was tried and tells nobody what to do, so it \
             does not conflict with a rule",
            rec_handle(rec.id)
        ));
    }
    let named = item["against"]
        .as_str()
        .map(str::trim)
        .filter(|a| !a.is_empty())
        .ok_or("a conflict names the rule in `against` (art:N, org:N or conv:N)")?;
    let rule = st
        .pg
        .rule_in_force(&d.repo, named)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| {
            format!(
                "{named} is not a rule in force in repo {}: name one from the item's `rules`",
                d.repo
            )
        })?;
    let covering = rules_of(st, &d.repo, rec)
        .await
        .map_err(|e| e.to_string())?;
    if !covering.iter().any(|c| c.id == rule.id) {
        return Err(format!(
            "{} does not cover {}: its paths ({}) match none of the record's anchors",
            rule.handle(),
            rec_handle(rec.id),
            rule.scope_glob
        ));
    }
    Ok(rule)
}

/// A store refusal of a proposal (`insert_verdict`'s locked rules) is the item's answer.
fn refusal(e: AvError) -> String {
    match e {
        AvError::Conflict(why) => why,
        e => e.to_string(),
    }
}

/// What a `promote` proposes: a convention from a decision, its text and path glob checked as
/// a convention written by hand is (`check_scope_glob`), with a quote of the decision itself.
fn promotion(rec: &Record, item: &Value, quotes_it: bool) -> std::result::Result<Promoted, String> {
    if rec.kind != RecordKind::Decision {
        return Err(format!(
            "a promote proposes a convention from a decision; {} is a {}",
            rec_handle(rec.id),
            rec.kind.as_str()
        ));
    }
    let text = item["text"].as_str().unwrap_or("").trim();
    if text.is_empty() {
        return Err("a promote needs the convention's text".into());
    }
    if text.chars().count() > PROMOTE_TEXT_MAX {
        return Err(format!(
            "the convention's text is {} characters; at most {PROMOTE_TEXT_MAX} (the rule, in \
             a line or two)",
            text.chars().count()
        ));
    }
    let scope_glob = item["scope_glob"]
        .as_str()
        .map(str::trim)
        .filter(|g| !g.is_empty())
        .ok_or("a promote needs the convention's scope_glob (the paths it is for)")?;
    margincode_store::check_scope_glob(scope_glob).map_err(|e| e.to_string())?;
    if !quotes_it {
        return Err("a promote needs a quote of the decision it is made from".into());
    }
    Ok(Promoted {
        text: text.to_owned(),
        scope_glob: scope_glob.to_owned(),
    })
}

/// A review run's verdicts. Each item is accepted (a row with no effect yet) or refused with
/// the reason; a refused item writes nothing.
async fn propose(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
    Json(b): Json<ProposeBody>,
) -> ApiResult<Value> {
    let d = st.pg.get_dream(id).await?;
    live(&d, "review")?;
    let on_list = worklist_ids(&d, "records");
    let mut results = Vec::with_capacity(b.verdicts.len());
    for item in &b.verdicts {
        match propose_one(&st, &d, &on_list, item).await {
            Ok(v) => {
                feed(
                    &st,
                    &d.repo,
                    "verdict",
                    json!({"verdict": v.id, "record": v.record_id, "kind": v.verdict,
                           "state": v.state, "dream": d.id, "by": d.agent}),
                )
                .await;
                results.push(json!({"record": item["record"], "accepted": true,
                                    "verdict": verdict_json(&v)}));
            }
            Err(why) => {
                results.push(json!({"record": item["record"], "accepted": false, "why": why}))
            }
        }
    }
    let accepted = results.iter().filter(|r| r["accepted"] == true).count();
    Ok(Json(json!({"results": results, "accepted": accepted,
                   "refused": results.len() - accepted})))
}

#[derive(Deserialize)]
struct ReviewBody {
    reviewer: String,
    agree: bool,
    /// The verify run that makes this review; without it, a review by name (a person or an agent).
    dream: Option<Uuid>,
    #[serde(default)]
    evidence: Vec<Value>,
}

/// Agrees or disagrees with a proposed verdict. From a verify run: the verdict is on that
/// run's list, and the review carries at least one quote of its own, checked like a verdict's.
/// Without `dream`, a decision under the reviewer's name. A verdict takes effect when its newest
/// review agrees.
async fn review(
    State(st): State<AppState>,
    Path(id): Path<String>,
    Json(b): Json<ReviewBody>,
) -> ApiResult<Value> {
    let v = st.pg.find_verdict(&id).await?;
    let rec = st.store.get_record(v.record_id).await?;
    let repo = repo_of(&st, &rec).await?;
    if b.reviewer.trim().is_empty() {
        return Err(AvError::Invalid("reviewer must not be empty".into()).into());
    }
    if v.state == "lapsed" {
        return Err(AvError::Conflict(format!(
            "{} has lapsed: the rule it names ({}) was amended, repealed or removed since, so \
             it has nothing to review; the rule changed, and a later `av dream --conflicts` \
             may propose it against the rule now in force",
            vd_handle(v.id),
            v.against.as_deref().unwrap_or("?")
        ))
        .into());
    }
    let by_name = match v.verdict.as_str() {
        "promote" => Some("proposes a convention"),
        "conflict" => Some("says a record conflicts with a rule"),
        _ => None,
    };
    if let (Some(what), Some(_)) = (by_name, b.dream) {
        return Err(AvError::IllegalState(format!(
            "{} {what}, which no verify run reviews: accept or reject it by name, without \
             `dream` (`av verdict accept` or `av verdict reject`)",
            vd_handle(v.id)
        ))
        .into());
    }
    if by_name.is_some() {
        by_name_review(&st, &v, &rec, b.reviewer.trim(), b.agree).await?;
    }
    let commit = match b.dream {
        Some(run) => {
            let d = st.pg.get_dream(run).await?;
            live(&d, "verify")?;
            if d.repo != repo || !worklist_ids(&d, "verdicts").contains(&v.id) {
                return Err(AvError::IllegalState(format!(
                    "{} is not on this run's list",
                    vd_handle(v.id)
                ))
                .into());
            }
            if b.evidence.is_empty() {
                return Err(AvError::Invalid(
                    "a verify run's review needs a quote of its own as evidence".into(),
                )
                .into());
            }
            dream_commit(&d)
        }
        None => st.vcs.trunk(&repo).await?,
    };
    check_all(&st, &repo, &commit, &b.evidence, b.dream.is_none())
        .await
        .map_err(AvError::Invalid)?;
    if v.state != "proposed" {
        return Err(AvError::Conflict(format!(
            "{} is {}: a verdict is reviewed once, while it is proposed",
            vd_handle(v.id),
            v.state
        ))
        .into());
    }
    let action = crate::Guarded::MemoryWrite {
        repo: &repo,
        action: crate::MemoryAction::VerdictReview {
            verdict: v.id,
            kind: &v.verdict,
            record: &rec,
            agree: b.agree,
        },
        by: b.reviewer.trim(),
        dream: b.dream,
    };
    if let Err(reason) = st.guard.check(action).await {
        return Err(
            AvError::IllegalState(format!("cannot review {}: {reason}", vd_handle(v.id))).into(),
        );
    }
    let v = st
        .pg
        .add_verdict_review(
            v.id,
            b.reviewer.trim(),
            b.dream,
            b.agree,
            &json!(b.evidence),
        )
        .await?;
    feed(
        &st,
        &repo,
        "verdict",
        json!({"verdict": v.id, "record": v.record_id, "kind": v.verdict, "state": v.state,
               "dream": b.dream, "by": b.reviewer.trim()}),
    )
    .await;
    Ok(Json(json!({"verdict": verdict_json(&v)})))
}

/// The daemon's rules for a review of a `promote` or a `conflict`, by another agent or a
/// person. Community has one shared token and no identities, so these hold the names it is
/// given, not who sent them: the reviewer is not, in any case, the verdict's author or the agent
/// of the run that proposed it; and a promote's acceptance needs the decision still live (not
/// superseded). A deployment that must know who decided enforces it in its `MergeGuard`, which
/// is asked next, as for any review.
async fn by_name_review(
    st: &AppState,
    v: &VerdictRow,
    rec: &Record,
    reviewer: &str,
    agree: bool,
) -> Result<()> {
    let same = |name: &str| name.to_lowercase() == reviewer.to_lowercase();
    let run_agent = match v.dream_id {
        Some(run) => Some(st.pg.get_dream(run).await?.agent),
        None => None,
    };
    if same(&v.author_agent) || run_agent.as_deref().is_some_and(same) {
        return Err(AvError::IllegalState(format!(
            "{reviewer:?} is the name of the agent that proposed it; another agent or a person \
             reviews a {}: review it under another name, with `av --agent NAME verdict \
             accept` or AV_AGENT=NAME (with neither, `av verdict accept` reviews as `user`)",
            v.verdict
        )));
    }
    if agree && v.verdict == "promote" && rec.superseded_by.is_some() {
        return Err(AvError::IllegalState(format!(
            "{} is superseded: its promote cannot become a convention; reject or retract it",
            rec_handle(rec.id)
        )));
    }
    Ok(())
}

#[derive(Deserialize)]
struct ByBody {
    by: String,
}

async fn retract_one(
    st: &AppState,
    repo: &str,
    v: &VerdictRow,
    by: &str,
    dream: Option<Uuid>,
) -> Result<VerdictRow> {
    let row = st
        .pg
        .insert_verdict(NewVerdict {
            record: v.record_id,
            verdict: "retracted",
            reason: "",
            canonical: None,
            retracts: Some(v.id),
            evidence: json!([]),
            anchor_hashes: json!([]),
            author_agent: by,
            dream,
            convention: None,
            against: None,
        })
        .await?;
    feed(
        st,
        repo,
        "verdict",
        json!({"verdict": v.id, "record": v.record_id, "kind": v.verdict,
               "state": "retracted", "by": by}),
    )
    .await;
    Ok(row)
}

/// Takes a verdict back, whatever its state: a `retracted` row that names it. The record is
/// then shown as if the verdict had never been written.
async fn retract(
    State(st): State<AppState>,
    Path(id): Path<String>,
    Json(b): Json<ByBody>,
) -> ApiResult<Value> {
    let v = st.pg.find_verdict(&id).await?;
    if v.state == "retraction" || v.state == "retracted" {
        return Err(AvError::Conflict(format!("{} is already retracted", vd_handle(v.id))).into());
    }
    let rec = st.store.get_record(v.record_id).await?;
    let repo = repo_of(&st, &rec).await?;
    retract_one(&st, &repo, &v, b.by.trim(), None).await?;
    Ok(Json(
        json!({"verdict": verdict_json(&st.pg.get_verdict(v.id).await?)}),
    ))
}

async fn counts(st: &AppState, d: &Dream) -> Result<Value> {
    let mut by: HashMap<String, usize> = HashMap::new();
    for v in st
        .pg
        .list_verdicts(&d.repo, None, Some(d.id), None, 1000)
        .await?
    {
        if v.verdict != "retracted" {
            *by.entry(v.verdict).or_default() += 1;
        }
    }
    Ok(json!(by))
}

#[derive(Deserialize, Default)]
struct FinishBody {
    #[serde(default)]
    failed: bool,
}

async fn finish(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
    body: Option<Json<FinishBody>>,
) -> ApiResult<Value> {
    let failed = body.is_some_and(|b| b.failed);
    let d = st.pg.get_dream(id).await?;
    let status = if failed { "failed" } else { "done" };
    let d = st.pg.end_dream(id, status, counts(&st, &d).await?).await?;
    feed(
        &st,
        &d.repo,
        "dream:finished",
        json!({"dream": d.id, "status": d.status, "counts": d.counts}),
    )
    .await;
    Ok(Json(json!({"dream": d})))
}

/// Retracts every verdict a review run wrote and marks the run `undone`: packs are then what
/// they were before it.
async fn undo(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
    Json(b): Json<ByBody>,
) -> ApiResult<Value> {
    let d = st.pg.get_dream(id).await?;
    if d.mode != "review" {
        return Err(AvError::IllegalState(
            "only a review run is undone; to take back what a verify run agreed to, retract \
             the verdicts"
                .into(),
        )
        .into());
    }
    let mut retracted = 0;
    for v in st
        .pg
        .list_verdicts(&d.repo, None, Some(d.id), None, 1000)
        .await?
    {
        if v.state == "retraction" || v.state == "retracted" {
            continue;
        }
        // a verdict retracted by hand since the list was read is already taken back
        match retract_one(&st, &d.repo, &v, b.by.trim(), Some(d.id)).await {
            Ok(_) => retracted += 1,
            Err(AvError::Conflict(_)) => {}
            Err(e) => return Err(e.into()),
        }
    }
    let d = st.pg.end_dream(id, "undone", d.counts.clone()).await?;
    feed(
        &st,
        &d.repo,
        "dream:finished",
        json!({"dream": d.id, "status": d.status, "retracted": retracted}),
    )
    .await;
    Ok(Json(json!({"dream": d, "retracted": retracted})))
}

async fn show(State(st): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Value> {
    let d = st.pg.get_dream(id).await?;
    let verdicts: Vec<Value> = st
        .pg
        .list_verdicts(&d.repo, None, Some(d.id), None, 1000)
        .await?
        .iter()
        .map(verdict_json)
        .collect();
    Ok(Json(json!({"dream": d, "verdicts": verdicts})))
}

/// One verdict (`id`: the full id or `vd:…`) with the record it judges and its reviews (oldest
/// first, each with its run and evidence), so a person can read all of it before agreeing.
async fn show_verdict(State(st): State<AppState>, Path(id): Path<String>) -> ApiResult<Value> {
    let v = st.pg.find_verdict(&id).await?;
    let rec = st.store.get_record(v.record_id).await?;
    let repo = repo_of(&st, &rec).await?;
    let reviews = st.pg.verdict_reviews(v.id).await?;
    Ok(Json(json!({
        "verdict": verdict_json(&v),
        "repo": repo,
        "record": {"id": rec_handle(rec.id), "kind": rec.kind, "summary": rec.summary,
                   "body": rec.body, "anchors": rec.anchors, "author": rec.author_agent},
        "reviews": reviews,
    })))
}

#[derive(Deserialize)]
struct ListQuery {
    /// `proposed`, `effective`, `rejected`, `retracted` or `lapsed`.
    state: Option<String>,
    record: Option<String>,
    limit: Option<i64>,
}

async fn list(
    State(st): State<AppState>,
    Path(repo): Path<String>,
    Query(q): Query<ListQuery>,
) -> ApiResult<Value> {
    st.vcs.trunk(&repo).await?;
    let record = match q.record.as_deref() {
        Some(named) => Some(
            st.store
                .find_record(named.trim().strip_prefix("rec:").unwrap_or(named.trim()))
                .await?
                .id,
        ),
        None => None,
    };
    let verdicts: Vec<Value> = st
        .pg
        .list_verdicts(
            &repo,
            q.state.as_deref(),
            None,
            record,
            q.limit.unwrap_or(100).clamp(1, 1000),
        )
        .await?
        .iter()
        .filter(|v| v.state != "retraction")
        .map(verdict_json)
        .collect();
    Ok(Json(json!({"verdicts": verdicts})))
}
