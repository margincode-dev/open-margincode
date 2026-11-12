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
use margincode_store::{Dream, NewDream, NewVerdict, PgStore, VerdictRow};
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use crate::AppState;
use crate::error::ApiResult;

/// Records a dream is handed unless `limit` says otherwise, and the most it may ask for.
const WORKLIST_DEFAULT: usize = 20;
const WORKLIST_MAX: usize = 50;
/// The newest live records the worklist is ranked over.
const WORKLIST_SCAN: i64 = 400;
/// Packs of this many days back count toward "shown".
const SHOWN_DAYS: i32 = 30;
/// A quote must have at least this many characters that are not white space: one word found
/// somewhere in a file shows nothing.
const MIN_QUOTE_CHARS: usize = 12;
/// Anchored code handed to a dream is clipped to this many characters per anchor.
const CODE_CLIP: usize = 6000;
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
        .route("/v1/verdicts/{id}/review", post(review))
        .route("/v1/verdicts/{id}/retract", post(retract))
}

fn rec_handle(id: RecordId) -> String {
    format!("rec:{}", &id.0.simple().to_string()[..8])
}

fn vd_handle(id: Uuid) -> String {
    format!("vd:{}", &id.simple().to_string()[..8])
}

/// A verdict as the API answers it: the row, its `vd:` handle and its record's `rec:` handle.
fn verdict_json(v: &VerdictRow) -> Value {
    let mut out = json!(v);
    out["handle"] = json!(vd_handle(v.id));
    out["record"] = json!(rec_handle(v.record_id));
    out
}

async fn feed(st: &AppState, repo: &str, kind: &str, payload: Value) {
    if let Err(e) = st.store.add_event(repo, kind, None, payload).await {
        tracing::warn!(error = %e, kind, "dream event not written");
    }
}

/// The repo a record belongs to.
async fn repo_of(st: &AppState, rec: &Record) -> Result<String> {
    Ok(st.store.get_task(rec.task_id).await?.repo)
}

// ---- starting a run: the worklist and the bundle ----

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
        let failed = origin == Some(Origin::Reverted)
            || (origin == Some(Origin::Abandoned) && rec.kind == RecordKind::Decision);
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
        ranked.push((rec.clone(), score));
    }
    ranked.sort_by(|a, b| b.1.total_cmp(&a.1).then_with(|| a.0.id.0.cmp(&b.0.id.0)));
    ranked.truncate(limit);
    Ok(ranked)
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
    let (worklist_json, items) = match mode {
        "review" => {
            let ranked = worklist(&st, &repo, &area, &trunk, limit).await?;
            let mut items = Vec::with_capacity(ranked.len());
            for (rec, score) in &ranked {
                let mut item = bundle_item(&st, &repo, &trunk, rec).await?;
                item["score"] = json!(score);
                items.push(item);
            }
            let ids: Vec<RecordId> = ranked.iter().map(|(r, _)| r.id).collect();
            (json!({"commit": trunk.0, "records": ids}), items)
        }
        "verify" => {
            let mut waiting = st
                .pg
                .list_verdicts(&repo, Some("proposed"), None, None, 1000)
                .await?;
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

// ---- evidence: every quote is found where it says it is ----

enum Cited {
    Code,
    Record(RecordId),
    Event,
}

fn quoted(text: &str, quote: &str) -> bool {
    let norm = |s: &str| s.replace("\r\n", "\n");
    norm(text).contains(norm(quote).trim_matches('\n'))
}

/// Checks one evidence entry. `{code: path, commit?, quote}`: the quote is in that file at
/// the commit (the run's own when none is given). `{record: id, quote}`: in that record's
/// summary or body, a record of this repository. `{event: id}`: an event of this repository.
/// The refusal says what was not found; nothing is corrected or matched loosely.
async fn check_evidence(
    st: &AppState,
    repo: &str,
    commit: &ChangeId,
    e: &Value,
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
    if let Some(id) = e["event"].as_i64() {
        return match st.pg.event_in_repo(repo, id).await {
            Ok(true) => Ok(Cited::Event),
            Ok(false) => Err(format!("event {id} is not an event of repo {repo}")),
            Err(e) => Err(e.to_string()),
        };
    }
    Err("an evidence entry is {code, quote}, {record, quote} or {event}".into())
}

async fn check_all(
    st: &AppState,
    repo: &str,
    commit: &ChangeId,
    evidence: &[Value],
) -> std::result::Result<Vec<Cited>, String> {
    let mut cited = Vec::with_capacity(evidence.len());
    for (i, e) in evidence.iter().enumerate() {
        cited.push(
            check_evidence(st, repo, commit, e)
                .await
                .map_err(|why| format!("evidence[{i}]: {why}"))?,
        );
    }
    Ok(cited)
}

// ---- proposing verdicts ----

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
    let kind = item["verdict"]
        .as_str()
        .and_then(VerdictKind::parse)
        .ok_or("verdict must be confirmed, disputed, duplicate or outdated")?;
    let reason = item["reason"].as_str().unwrap_or("").trim();
    if reason.chars().count() > MAX_SUMMARY_CHARS {
        return Err(format!(
            "reason is {} characters; at most {MAX_SUMMARY_CHARS}",
            reason.chars().count()
        ));
    }
    if kind == VerdictKind::Disputed && reason.is_empty() {
        return Err("a disputed verdict needs a reason: it is printed with the record".into());
    }
    let already = st
        .pg
        .list_verdicts(&d.repo, None, Some(d.id), Some(rec.id), 1)
        .await
        .map_err(|e| e.to_string())?;
    if !already.is_empty() {
        return Err(format!(
            "this run already judged {}; one verdict per record and run",
            rec_handle(rec.id)
        ));
    }
    let evidence = item["evidence"].as_array().cloned().unwrap_or_default();
    let cited = check_all(st, &d.repo, &dream_commit(d), &evidence).await?;
    let has_code = cited.iter().any(|c| matches!(c, Cited::Code));
    let has_event = cited.iter().any(|c| matches!(c, Cited::Event));
    let cites = |id: RecordId| {
        cited
            .iter()
            .any(|c| matches!(c, Cited::Record(r) if *r == id))
    };
    let mut canonical = None;
    let mut hashes = json!([]);
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
        })
        .await
        .map_err(|e| e.to_string())
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

// ---- the second look ----

#[derive(Deserialize)]
struct ReviewBody {
    reviewer: String,
    agree: bool,
    /// The verify run that makes this review; without it the reviewer is a person.
    dream: Option<Uuid>,
    #[serde(default)]
    evidence: Vec<Value>,
}

/// Agrees or disagrees with a proposed verdict. From a verify run: the verdict is on that
/// run's list, and the review carries at least one quote of its own, checked like a verdict's.
/// Without `dream`, a person's decision. A verdict takes effect when its newest review agrees.
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
    check_all(&st, &repo, &commit, &b.evidence)
        .await
        .map_err(AvError::Invalid)?;
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

// ---- a run's end, and taking a whole run back ----

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
        retract_one(&st, &d.repo, &v, b.by.trim(), Some(d.id)).await?;
        retracted += 1;
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

#[derive(Deserialize)]
struct ListQuery {
    /// `proposed`, `effective`, `rejected` or `retracted`.
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
