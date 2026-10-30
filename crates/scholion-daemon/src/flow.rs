//! Task lifecycle operations (Plan 3). Kept in the daemon rather than a separate crate: every
//! operation first runs the daemon's checkpoint + re-index.

use std::collections::{BTreeSet, HashMap};

use axum::Json;
use axum::extract::{Path, State};
use axum::http::HeaderMap;
use scholion_core::lifecycle::{Event, next};
use scholion_core::{
    AgentFamily, Anchor, AvError, ChangeId, Convention, HandoffState, MergeOutcome, MergeState,
    NewRecord, RecordId, RecordKind, Result, Task, TaskId, TaskStatus, Verdict,
};
use scholion_pack::BundleRequest;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::AppState;
use crate::api::{BUNDLE_BUDGET, RecordBody, check_record, snapshot_and_index, write_record};
use crate::error::ApiResult;
use crate::indexing::{
    rebuild_task_overlay, reconcile_task_overlay, reindex_changed, replace_task_overlay,
};

/// Serializes merge and undo across the daemon, so a status check and the trunk move it guards
/// cannot interleave with another merge of the same task.
static MERGE_LOCK: Mutex<()> = Mutex::const_new(());

/// Request header carrying the claim generation a write is made under (see `fence`).
pub(crate) const GENERATION_HEADER: &str = "x-av-generation";

/// Refuses a write made under an older claim of the task; a write under the current claim renews
/// its lease. A request without the header is not fenced. Call with `AppState::lock_task` held, so
/// no claim lands between this check and the write.
pub(crate) async fn fence(st: &AppState, task: &Task, headers: &HeaderMap) -> Result<()> {
    let Some(value) = headers.get(GENERATION_HEADER) else {
        return Ok(());
    };
    let generation: i64 = value
        .to_str()
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .ok_or_else(|| AvError::Invalid(format!("{GENERATION_HEADER} must be a number")))?;
    if generation != task.generation
        || !st
            .store
            .renew_lease(task.id, generation, st.lease_secs)
            .await?
    {
        return Err(AvError::IllegalState(format!(
            "claim lost: {} claimed task {} (generation {}, this write was made under {generation}); \
             stop writing to its workspace",
            task.owner_agent, task.slug, task.generation
        )));
    }
    Ok(())
}

/// Whether the task explains itself (besides its intent, at least one decision, attempt, note or
/// handoff record with a summary), and its changed symbols that no record of the task anchors to
/// (by symbol or by file), sorted.
pub(crate) async fn coverage(st: &AppState, task: &Task) -> Result<(bool, Vec<String>)> {
    let records = st.store.task_records(task.id).await?;
    let explained = records.iter().any(|r| {
        matches!(
            r.kind,
            RecordKind::Decision | RecordKind::Attempt | RecordKind::Note | RecordKind::Handoff
        ) && !r.summary.trim().is_empty()
    });
    let (anchored_syms, anchored_paths): (BTreeSet<String>, BTreeSet<String>) = (
        records
            .iter()
            .flat_map(|r| r.anchors.iter().filter_map(|a| a.symbol.clone()))
            .collect(),
        records
            .iter()
            .flat_map(|r| r.anchors.iter().filter(|a| a.symbol.is_none()))
            .map(|a| a.path.clone())
            .collect(),
    );
    // Test code never needs a record. Only Rust keeps tests inside source files, so only a `.rs`
    // file that the path and name don't already mark as a test is read (once) at the task's head.
    let mut rust_src: HashMap<String, Option<String>> = HashMap::new();
    let mut changed = vec![];
    for s in st.packer.changed_symbols(task.id).await? {
        let sym = &s.symbol;
        let maybe_inline = sym.path.ends_with(".rs")
            && !scholion_pack::is_test_code(&sym.path, &sym.qname, 0, None);
        if maybe_inline && !rust_src.contains_key(&sym.path) {
            let text = st
                .vcs
                .read_file(&task.repo, &task.head_change, &sym.path)
                .await
                .ok()
                .map(|b| String::from_utf8_lossy(&b).into_owned());
            rust_src.insert(sym.path.clone(), text);
        }
        let src = rust_src.get(&sym.path).and_then(|t| t.as_deref());
        if !scholion_pack::is_test_code(&sym.path, &sym.qname, sym.start_line, src) {
            changed.push(s);
        }
    }
    let unrecorded: Vec<String> = changed
        .into_iter()
        .filter(|s| {
            !anchored_syms.contains(&s.symbol.qname) && !anchored_paths.contains(&s.symbol.path)
        })
        .map(|s| s.symbol.qname)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    Ok((explained, unrecorded))
}

/// The submit and handoff gate: the task must explain itself (`coverage`). Returns the changed
/// symbols that no record of the task anchors to; the refusal lists them too.
async fn gate(st: &AppState, task: &Task, action: &str) -> Result<Vec<String>> {
    let (explained, unrecorded) = coverage(st, task).await?;
    if !explained {
        let how = if action == "submitting" {
            "av record ..., or in one call av submit --record \"decision: ... @ path:Symbol\""
        } else {
            "av record ..."
        };
        return Err(AvError::IllegalState(format!(
            "record a decision, attempt or note before {action} ({how}); \
             changed symbols with no record: {}",
            if unrecorded.is_empty() {
                "none".to_owned()
            } else {
                unrecorded.join(", ")
            }
        )));
    }
    Ok(unrecorded)
}

/// What the submit gate would say now, without submitting (handoff B7): for a hook that asks the
/// agent to record its decisions and dead ends before a session ends. Reads the last checkpoint.
pub(crate) async fn unrecorded(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<serde_json::Value> {
    let _guard = st.lock_task(TaskId(id)).await;
    let task = st.store.get_task(TaskId(id)).await?;
    reconcile_task_overlay(&st, &task).await?;
    let (explained, unrecorded) = coverage(&st, &task).await?;
    Ok(Json(serde_json::json!({
        "status": task.status.as_str(),
        "explained": explained,
        "unrecorded_symbols": unrecorded,
    })))
}

#[derive(Deserialize)]
pub(crate) struct SubmitBody {
    agent: String,
    /// Records to write before the gate (`POST .../records` bodies, without `agent`: they take
    /// this one). All are checked before any is written.
    #[serde(default)]
    records: Vec<Value>,
}

/// What an agent does once its task is in review. Fixed copy: agents read this verbatim.
const NEXT_IN_REVIEW: &str =
    "in review: a reviewer runs av review to approve or request changes; nothing else to do";

/// Turns the inline records of a submit body into `RecordBody`s authored by `agent`. Errors name
/// the offending index.
fn inline_records(agent: &str, raw: Vec<Value>) -> Result<Vec<RecordBody>> {
    raw.into_iter()
        .enumerate()
        .map(|(i, mut v)| {
            let obj = v
                .as_object_mut()
                .ok_or_else(|| AvError::Invalid(format!("records[{i}] must be an object")))?;
            obj.insert("agent".into(), Value::String(agent.to_owned()));
            serde_json::from_value(v).map_err(|e| AvError::Invalid(format!("records[{i}]: {e}")))
        })
        .collect()
}

#[derive(Serialize)]
pub(crate) struct SubmitReport {
    status: String,
    head: String,
    changed: Vec<String>,
    /// Changed symbols that no record of this task anchors to (by symbol or by file). Advisory.
    unrecorded_symbols: Vec<String>,
    /// Ids of the inline records this call wrote, in order.
    recorded: Vec<RecordId>,
    /// What happens next, for the agent to read.
    next: &'static str,
}

/// Snapshot, then refuse unless there is something to review and the task explains it: besides
/// its intent, at least one decision, attempt, note or handoff record.
pub(crate) async fn submit(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Json(body): Json<SubmitBody>,
) -> ApiResult<SubmitReport> {
    // held until the transition, so no checkpoint moves the head between the gate and the status
    let _guard = st.lock_task(scholion_core::TaskId(id)).await;
    let mut task = st.store.get_task(TaskId(id)).await?;
    fence(&st, &task, &headers).await?;
    if task.status == TaskStatus::Conflicted {
        // Bring main in first; the owner must clear any conflict markers before review.
        let conflicts = sync_workspace(&st, &task).await?;
        if !conflicts.is_empty() {
            return Err(AvError::IllegalState(format!(
                "rebased onto main; resolve the conflict markers in {} and submit again",
                conflicts.join(", ")
            ))
            .into());
        }
        st.store.transition(task.id, Event::Resolve).await?;
        task = st.store.get_task(task.id).await?;
    }
    next(task.status, Event::Submit)?; // fail fast, before touching the workspace
    // Check every inline record before anything is written, so a bad one fails the whole submit.
    let inline = inline_records(&body.agent, body.records)?;
    for r in &inline {
        check_record(&st, r).await?;
    }
    let (head, changed, _) = snapshot_and_index(&st, &task).await?;
    // the gate reads source at the task's head, which the snapshot just moved
    let task = st.store.get_task(task.id).await?;
    if head == task.base_change || changed.is_empty() {
        return Err(AvError::IllegalState(
            "nothing to submit: no changes since the task started".into(),
        )
        .into());
    }
    refuse_conflict_markers(&st, &task.repo, &head, &changed).await?;
    // written before the gate, so they count toward it
    let mut recorded = Vec::with_capacity(inline.len());
    for r in inline {
        recorded.push(write_record(&st, &task, r).await?.id);
    }
    let unrecorded = gate(&st, &task, "submitting").await?;
    let status = st.store.transition(task.id, Event::Submit).await?;
    carry_approval(&st, &st.store.get_task(task.id).await?).await?;
    Ok(Json(SubmitReport {
        status: status.as_str().to_owned(),
        head: head.0,
        changed,
        unrecorded_symbols: unrecorded,
        recorded,
        next: NEXT_IN_REVIEW,
    }))
}

/// Rebases the task onto main, replaces the overlay with `changed(T, H')`, and stores the new
/// base, head and conflicts.
async fn sync_workspace(st: &AppState, task: &Task) -> Result<Vec<String>> {
    let rebased = st.vcs.rebase_workspace(&task.repo, &task.slug).await?;
    st.store
        .set_workspace(task.id, &task.workspace_path, &rebased.base)
        .await?;
    let task = st.store.get_task(task.id).await?;
    // After rebase B = T; O is exactly the task's remaining diff against trunk.
    let head = st.vcs.snapshot(&task.repo, &task.slug).await?;
    let changed = st
        .vcs
        .changed_paths(&task.repo, &rebased.base, &head)
        .await?;
    replace_task_overlay(st, &task.repo, &task.slug, &head).await?;
    st.store.set_head(task.id, &head, &changed).await?;
    let mut state = st.store.merge_state(task.id).await?;
    state.conflicts = rebased.conflicts.clone();
    st.store.set_merge_state(task.id, &state).await?;
    Ok(rebased.conflicts)
}

#[derive(Deserialize)]
pub(crate) struct AgentBody {
    agent: String,
}

/// Rebases an open or conflicted task onto the current main. Conflicts stay in the workspace as
/// markers and are listed.
pub(crate) async fn sync(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Json(_body): Json<AgentBody>,
) -> ApiResult<Value> {
    let _guard = st.lock_task(TaskId(id)).await;
    let task = st.store.get_task(TaskId(id)).await?;
    fence(&st, &task, &headers).await?;
    if !matches!(task.status, TaskStatus::Open | TaskStatus::Conflicted) {
        return Err(AvError::IllegalState(format!(
            "a task that is {} cannot sync; only open or conflicted tasks can",
            task.status.as_str()
        ))
        .into());
    }
    let conflicts = sync_workspace(&st, &task).await?;
    let task = st.store.get_task(task.id).await?;
    Ok(Json(json!({
        "status": task.status.as_str(),
        "base": task.base_change,
        "head": task.head_change,
        "conflicts": conflicts,
    })))
}

#[derive(Deserialize)]
pub(crate) struct ValidateBody {
    agent: String,
    command: String,
    exit_code: i32,
    #[serde(default)]
    output_tail: String,
    /// Results of the task's convention checks (`GET /v1/tasks/{id}/checks`), run alongside.
    #[serde(default)]
    checks: Vec<CheckResult>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct CheckResult {
    convention_id: i64,
    exit_code: i32,
    #[serde(default)]
    output_tail: String,
}

/// Records that `command` ran in the workspace with `exit_code`, bound to the head it ran at.
/// Approval requires a passing validation at the head being approved.
pub(crate) async fn validate(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Json(b): Json<ValidateBody>,
) -> ApiResult<Value> {
    let _guard = st.lock_task(TaskId(id)).await;
    let task = st.store.get_task(TaskId(id)).await?;
    fence(&st, &task, &headers).await?;
    let (head, _, _) = snapshot_and_index(&st, &task).await?;
    let summary: String = format!("`{}` exit {}", b.command, b.exit_code)
        .chars()
        .take(200)
        .collect();
    let body = json!({
        "command": b.command,
        "exit_code": b.exit_code,
        "head_change": head.0,
        "output_tail": b.output_tail,
        "checks": b.checks,
    });
    let record = st
        .store
        .add_record(NewRecord {
            task_id: task.id,
            kind: RecordKind::Validation,
            summary,
            body: body.to_string(),
            change_id: Some(head.clone()),
            supersedes: None,
            author_agent: b.agent,
            anchors: vec![],
        })
        .await?;
    let task = st.store.get_task(task.id).await?;
    if task.status == TaskStatus::InReview {
        carry_approval(&st, &task).await?;
    }
    Ok(Json(json!({
        "record": record.id,
        "head": head,
        "passed": b.exit_code == 0,
    })))
}

/// Line starts that only conflict markers have: jj's (`<<<<<<< Conflict 1 of 1`, `%%%%%%% Changes
/// from base to side #1`, `+++++++ Contents of side #2`, `>>>>>>> Conflict 1 of 1 ends`) and
/// git's. A bare `=======` is left alone: it underlines Markdown and reStructuredText headings.
const MARKERS: [&str; 4] = ["<<<<<<< ", ">>>>>>> ", "%%%%%%% ", "+++++++ "];

/// Refuses a submit while a changed text file still holds conflict markers. jj counts a file as
/// resolved once it is rewritten, and no test reads a stylesheet; in the stress runs two agents
/// submitted leftover markers that only a reviewer caught.
async fn refuse_conflict_markers(
    st: &AppState,
    repo: &str,
    head: &ChangeId,
    changed: &[String],
) -> Result<()> {
    let mut found = vec![];
    for (path, bytes) in st.vcs.read_files(repo, head, changed).await? {
        let Ok(bytes) = bytes else { continue };
        let Ok(text) = String::from_utf8(bytes) else {
            continue;
        };
        for (i, line) in text.lines().enumerate() {
            if MARKERS.iter().any(|m| line.starts_with(m)) {
                found.push(format!("{path}:{}", i + 1));
                break;
            }
        }
    }
    if found.is_empty() {
        return Ok(());
    }
    Err(AvError::IllegalState(format!(
        "conflict markers are still in {}: resolve every conflict marker, then checkpoint and submit again",
        found.join(", ")
    )))
}

/// Reviewer name of approvals that av carried across a rebase.
const CARRIER: &str = "av";

/// Carries the task's last human approval to its current head when the head only moved by a
/// rebase that kept the approved change. Antique's idea, in small: reviewers judge the change,
/// not every commit id it is rebased to. It carries only when
/// - the task's newest verdict is an approval at an earlier head H0 (carried ones chain back to
///   the human approval at H0), with no request for changes after it;
/// - a passing validation exists at the current head;
/// - the changed files are the same set, and per file the edits are identical
///   (`patch_signature`, line numbers aside), or the file was changed by main too
///   (`changed(base0, H0) ∩ changed(base0, base)`) and matches the repo's `carry_globs`.
///
/// Anything missing fails closed. A carry is an `approve` verdict by `av` plus a `review` record.
/// Idempotent; takes no lock (callers hold the task's).
pub(crate) async fn carry_approval(st: &AppState, task: &Task) -> Result<Option<ChangeId>> {
    let head = &task.head_change;
    if head.0.is_empty() || st.store.latest_verdict(task.id, head).await?.is_some() {
        return Ok(None);
    }
    let Some(last) = st.store.latest_verdict_any(task.id).await? else {
        return Ok(None);
    };
    if last.verdict != Verdict::Approve {
        return Ok(None);
    }
    let approved = match &last.carried_from {
        None => last,
        Some(h0) => match st.store.human_verdict_at(task.id, h0).await? {
            Some(v) if v.verdict == Verdict::Approve => v,
            _ => return Ok(None),
        },
    };
    let Some(base0) = approved.base.clone() else {
        return Ok(None);
    };
    let h0 = approved.head.clone();
    if !matches!(validation_at(st, task.id, head).await?, Some((_, true)))
        || check_problem(st, task, head).await?.is_some()
    {
        return Ok(None);
    }
    let repo = &task.repo;
    let base = &task.base_change;
    let mut before = st.vcs.changed_paths(repo, &base0, &h0).await?;
    let mut after = st.vcs.changed_paths(repo, base, head).await?;
    before.sort();
    after.sort();
    if before != after {
        return Ok(None);
    }
    let main_moved: BTreeSet<String> = st
        .vcs
        .changed_paths(repo, &base0, base)
        .await?
        .into_iter()
        .collect();
    let globs = {
        let mut b = globset::GlobSetBuilder::new();
        for g in st.store.repo_settings(repo).await?.carry_globs {
            if let Ok(g) = globset::Glob::new(&g) {
                b.add(g);
            }
        }
        b.build().map_err(|e| AvError::Invalid(e.to_string()))?
    };
    let (old_base, old_head, new_base, new_head) = futures::try_join!(
        st.vcs.read_files(repo, &base0, &after),
        st.vcs.read_files(repo, &h0, &after),
        st.vcs.read_files(repo, base, &after),
        st.vcs.read_files(repo, head, &after),
    )?;
    let mut resolved = vec![];
    for (i, path) in after.iter().enumerate() {
        let file = |files: &Vec<(String, Result<Vec<u8>>)>| -> Result<Option<Vec<u8>>> {
            match &files[i].1 {
                Ok(bytes) => Ok(Some(bytes.clone())),
                Err(AvError::NotFound(_)) => Ok(None),
                Err(e) => Err(AvError::Vcs(e.to_string())),
            }
        };
        let (ob, oh, nb, nh) = (
            file(&old_base)?,
            file(&old_head)?,
            file(&new_base)?,
            file(&new_head)?,
        );
        if same_edit(&ob, &oh, &nb, &nh) {
            continue;
        }
        if main_moved.contains(path) && globs.is_match(path) {
            resolved.push(path.clone());
            continue;
        }
        return Ok(None);
    }
    st.store
        .add_verdict(
            task.id,
            CARRIER,
            Verdict::Approve,
            head,
            Some(base),
            Some(&h0),
        )
        .await?;
    let how = if resolved.is_empty() {
        "the rebase kept every approved edit".to_owned()
    } else {
        format!("resolution differs only in {}", resolved.join(", "))
    };
    let summary: String = format!(
        "approval by {} at {} carried to {}: {how}",
        approved.reviewer,
        short(&h0.0),
        short(&head.0)
    )
    .chars()
    .take(200)
    .collect();
    st.store
        .transition_with_record(
            task.id,
            Event::Approve,
            NewRecord {
                task_id: task.id,
                kind: RecordKind::Review,
                summary,
                body: String::new(),
                change_id: Some(head.clone()),
                supersedes: None,
                author_agent: CARRIER.into(),
                anchors: vec![],
            },
        )
        .await?;
    Ok(Some(h0))
}

/// Whether `old_base → old_head` and `new_base → new_head` make the same edit to one file:
/// the same creation/deletion, and the same text changes wherever they sit. Non-UTF-8 content
/// must match byte for byte.
fn same_edit(
    old_base: &Option<Vec<u8>>,
    old_head: &Option<Vec<u8>>,
    new_base: &Option<Vec<u8>>,
    new_head: &Option<Vec<u8>>,
) -> bool {
    if old_base.is_some() != new_base.is_some() || old_head.is_some() != new_head.is_some() {
        return false;
    }
    let text = |b: &Option<Vec<u8>>| match b {
        None => Some(String::new()),
        Some(bytes) => String::from_utf8(bytes.clone()).ok(),
    };
    match (
        text(old_base),
        text(old_head),
        text(new_base),
        text(new_head),
    ) {
        (Some(ob), Some(oh), Some(nb), Some(nh)) => {
            scholion_pack::diff::patch_signature(&ob, &oh)
                == scholion_pack::diff::patch_signature(&nb, &nh)
        }
        _ => old_base == new_base && old_head == new_head,
    }
}

/// Required conventions with a check command whose glob matches a path the task changed.
pub(crate) async fn applicable_checks(st: &AppState, task: &Task) -> Result<Vec<Convention>> {
    if task.head_change.0.is_empty() {
        return Ok(vec![]);
    }
    let changed = st
        .vcs
        .changed_paths(&task.repo, &task.base_change, &task.head_change)
        .await?;
    Ok(st
        .store
        .conventions_for(&task.repo, &changed)
        .await?
        .into_iter()
        .filter(|c| c.required && c.check_command.is_some())
        .collect())
}

/// Why the task's convention checks do not allow approval at `head` (`None`: they do). The
/// newest result per convention across the validations at `head` counts; a check with no
/// result counts as not run.
async fn check_problem(st: &AppState, task: &Task, head: &ChangeId) -> Result<Option<String>> {
    let wanted = applicable_checks(st, task).await?;
    if wanted.is_empty() {
        return Ok(None);
    }
    let mut records: Vec<_> = st
        .store
        .task_records(task.id)
        .await?
        .into_iter()
        .filter(|r| r.kind == RecordKind::Validation && r.change_id.as_ref() == Some(head))
        .collect();
    records.sort_by_key(|r| r.created_at);
    let mut latest = std::collections::HashMap::new();
    for r in &records {
        let body: Value = serde_json::from_str(&r.body).unwrap_or_default();
        for c in body["checks"].as_array().into_iter().flatten() {
            if let (Some(id), Some(code)) = (c["convention_id"].as_i64(), c["exit_code"].as_i64()) {
                latest.insert(id, code);
            }
        }
    }
    let mut problems = vec![];
    for c in wanted {
        let cmd = c.check_command.unwrap_or_default();
        match latest.get(&c.id.0) {
            Some(0) => {}
            Some(code) => problems.push(format!(
                "conv:{} check `{cmd}` failed (exit {code}): {}",
                c.id.0, c.text
            )),
            None => problems.push(format!(
                "conv:{} check `{cmd}` not run at head {} (`av validate` runs it): {}",
                c.id.0,
                short(&head.0),
                c.text
            )),
        }
    }
    Ok((!problems.is_empty()).then(|| problems.join("; ")))
}

/// The latest validation at `head`, as (command, passed).
async fn validation_at(
    st: &AppState,
    task: TaskId,
    head: &ChangeId,
) -> Result<Option<(String, bool)>> {
    let records = st.store.task_records(task).await?;
    Ok(records
        .iter()
        .filter(|r| r.kind == RecordKind::Validation && r.change_id.as_ref() == Some(head))
        .max_by_key(|r| r.created_at)
        .map(|r| {
            let body: Value = serde_json::from_str(&r.body).unwrap_or_default();
            (
                body["command"].as_str().unwrap_or_default().to_owned(),
                body["exit_code"].as_i64() == Some(0),
            )
        }))
}

#[derive(Deserialize)]
pub(crate) struct Finding {
    summary: String,
    #[serde(default)]
    body: String,
    #[serde(default)]
    anchors: Vec<Anchor>,
}

#[derive(Deserialize)]
pub(crate) struct ReviewBody {
    reviewer: String,
    verdict: Verdict,
    #[serde(default)]
    findings: Vec<Finding>,
}

/// Appends a verdict bound to the task's current head. `approve` needs a passing validation at that
/// head; `request_changes` sends the task back to its owner with the findings as `review` records.
pub(crate) async fn review(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
    Json(b): Json<ReviewBody>,
) -> ApiResult<Value> {
    let task = st.store.get_task(TaskId(id)).await?;
    if task.status != TaskStatus::InReview {
        return Err(AvError::IllegalState(format!(
            "only a task in review can be reviewed; this one is {}",
            task.status.as_str()
        ))
        .into());
    }
    if b.reviewer.trim().is_empty() || b.reviewer == task.owner_agent {
        return Err(AvError::IllegalState(format!(
            "the reviewer must be another agent than the owner ({})",
            task.owner_agent
        ))
        .into());
    }
    let head = task.head_change.clone();
    if b.verdict == Verdict::Approve {
        match validation_at(&st, task.id, &head).await? {
            Some((_, true)) => {}
            Some((command, false)) => {
                return Err(AvError::IllegalState(format!(
                    "cannot approve: `{command}` failed at head {}",
                    short(&head.0)
                ))
                .into());
            }
            None => {
                return Err(AvError::IllegalState(format!(
                    "cannot approve: tests not run at head {} (the owner runs `av validate -- <cmd>`)",
                    short(&head.0)
                ))
                .into());
            }
        }
        if let Some(problem) = check_problem(&st, &task, &head).await? {
            return Err(AvError::IllegalState(format!("cannot approve: {problem}")).into());
        }
        let action = crate::Guarded::Approve {
            task: &task,
            reviewer: &b.reviewer,
        };
        if let Err(reason) = st.guard.check(action).await {
            return Err(AvError::IllegalState(format!("cannot approve: {reason}")).into());
        }
    }
    st.store
        .add_verdict(
            task.id,
            &b.reviewer,
            b.verdict,
            &head,
            Some(&task.base_change),
            None,
        )
        .await?;
    for f in b.findings {
        st.store
            .add_record(NewRecord {
                task_id: task.id,
                kind: RecordKind::Review,
                summary: f.summary,
                body: f.body,
                change_id: Some(head.clone()),
                supersedes: None,
                author_agent: b.reviewer.clone(),
                anchors: f.anchors,
            })
            .await?;
    }
    let status = if b.verdict == Verdict::RequestChanges {
        st.store.transition(task.id, Event::RequestChanges).await?
    } else {
        st.store.transition(task.id, Event::Approve).await?
    };
    Ok(Json(json!({
        "status": status.as_str(),
        "verdict": b.verdict.as_str(),
        "head": head,
    })))
}

/// Written into every merge commit's message; finds a merge on trunk after a crash.
fn merge_marker(task: &Task) -> String {
    format!("av task {}", task.id)
}

/// Written into a revert commit's message. It must not contain `merge_marker`.
fn revert_marker(task: &Task) -> String {
    format!("av revert {}", task.id)
}

/// A merge commit of `task` on trunk's recent history, as (pre-merge parent, merge commit). Found
/// when an earlier `merge` moved main but died before recording it. An undone merge is no longer
/// on trunk, so it is never found.
async fn merged_already(st: &AppState, task: &Task) -> Result<Option<(ChangeId, ChangeId)>> {
    let (marker, reverted) = (merge_marker(task), revert_marker(task));
    // newest first: a revert of this task above its old merge means that merge no longer counts
    for c in st.vcs.trunk_log(&task.repo, RECOVERY_DEPTH).await? {
        if c.description.contains(&reverted) {
            return Ok(None);
        }
        if c.description.contains(&marker) {
            return Ok(c.parents.first().map(|p| (p.clone(), c.change)));
        }
    }
    Ok(None)
}

/// How far back on trunk `merge` looks for its own earlier, unrecorded merge.
const RECOVERY_DEPTH: usize = 50;

/// Merges an approved task into main. A conflict leaves main untouched, marks the task
/// `conflicted` and names the paths; the owner then runs `av sync`, fixes the markers and submits.
/// If an earlier attempt moved main but died before recording it, this one records that merge
/// instead of merging again.
pub(crate) async fn merge(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
    Json(b): Json<AgentBody>,
) -> ApiResult<Value> {
    let _guard = MERGE_LOCK.lock().await;
    // merge snapshots the workspace: no checkpoint of the owner may run alongside
    let _task_guard = st.lock_task(TaskId(id)).await;
    let task = st.store.get_task(TaskId(id)).await?;
    next(task.status, Event::MergeOk)?;
    let head = task.head_change.clone();
    carry_approval(&st, &task).await?;
    match st.store.latest_verdict(task.id, &head).await? {
        Some((_, Verdict::Approve)) => {}
        Some((reviewer, Verdict::RequestChanges)) => {
            return Err(AvError::IllegalState(format!(
                "{reviewer} requested changes at head {}",
                short(&head.0)
            ))
            .into());
        }
        None => {
            return Err(AvError::IllegalState(format!(
                "no approval at head {}; another agent must `av review --verdict approve` first",
                short(&head.0)
            ))
            .into());
        }
    }
    if let Some((pre_merge, main)) = merged_already(&st, &task).await? {
        return Ok(Json(
            record_merge(&st, &task, pre_merge, main, b.agent, true).await?,
        ));
    }
    let action = crate::Guarded::Merge {
        task: &task,
        agent: &b.agent,
    };
    if let Err(reason) = st.guard.check(action).await {
        return Err(AvError::IllegalState(format!("cannot merge: {reason}")).into());
    }
    let message = format!(
        "{}\n\n{}\n\n{} ({})",
        task.title,
        task.goal,
        merge_marker(&task),
        task.slug
    );
    match st
        .vcs
        .merge_task(&task.repo, &task.slug, &head, &message)
        .await?
    {
        MergeOutcome::Merged { pre_merge, main } => Ok(Json(
            record_merge(&st, &task, pre_merge, main, b.agent, false).await?,
        )),
        MergeOutcome::Conflicted { paths } => {
            let mut state = st.store.merge_state(task.id).await?;
            state.conflicts = paths.clone();
            st.store.set_merge_state(task.id, &state).await?;
            let status = st.store.transition(task.id, Event::MergeConflict).await?;
            Ok(Json(json!({
                "status": status.as_str(),
                "main": Value::Null,
                "changed": [],
                "conflicts": paths,
            })))
        }
    }
}

/// Stores the merge, marks the task merged and re-indexes `main` for the merged paths.
async fn record_merge(
    st: &AppState,
    task: &Task,
    pre_merge: ChangeId,
    main: ChangeId,
    agent: String,
    recovered: bool,
) -> Result<Value> {
    st.store
        .set_merge_state(
            task.id,
            &MergeState {
                pre_merge: Some(pre_merge.clone()),
                merge_commit: Some(main.clone()),
                merged_by: Some(agent),
                conflicts: vec![],
            },
        )
        .await?;
    let status = st.store.transition(task.id, Event::MergeOk).await?;
    crate::backup::spawn_backup(st, &task.repo);
    drop_task_scope(st, task).await;
    let changed = st.vcs.changed_paths(&task.repo, &pre_merge, &main).await?;
    let warnings = reindex_changed(st, &task.repo, "main", &main, &changed).await?;
    Ok(json!({
        "status": status.as_str(),
        "main": main,
        "changed": changed,
        "conflicts": [],
        "warnings": warnings,
        "recovered": recovered,
    }))
}

/// Moves main back to where it was before this task's merge (only while main is still that merge,
/// and only by the agent that merged), returns the task to review and records a note.
#[derive(Deserialize)]
pub(crate) struct RevertBody {
    agent: String,
    #[serde(default)]
    reason: String,
}

/// Reverts a merged task on main with a new commit that inverts its merge, whatever merged
/// since (Scion's revert pipeline, in small; `undo` only works while the merge is newest). The
/// task goes back to `open` for its owner to fix, keeping its workspace. A revert that conflicts
/// with later work writes nothing.
pub(crate) async fn revert(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
    Json(b): Json<RevertBody>,
) -> ApiResult<Value> {
    let _guard = MERGE_LOCK.lock().await;
    let task = st.store.get_task(TaskId(id)).await?;
    next(task.status, Event::Revert)?;
    let state = st.store.merge_state(task.id).await?;
    let Some(merged) = state.merge_commit.clone() else {
        return Err(AvError::IllegalState("this task has no merge to revert".into()).into());
    };
    let message = format!(
        "Revert {}\n\n{}{}\n{} ({})",
        task.title,
        if b.reason.is_empty() { "" } else { &b.reason },
        if b.reason.is_empty() { "" } else { "\n" },
        revert_marker(&task),
        task.slug
    );
    let (pre, main) = match st.vcs.revert_merge(&task.repo, &merged, &message).await? {
        MergeOutcome::Merged { pre_merge, main } => (pre_merge, main),
        MergeOutcome::Conflicted { paths } => {
            return Err(AvError::IllegalState(format!(
                "reverting {} conflicts with later work in {}; main is unchanged",
                short(&merged.0),
                paths.join(", ")
            ))
            .into());
        }
    };
    st.store
        .set_merge_state(task.id, &MergeState::default())
        .await?;
    let summary: String = format!(
        "merge {} reverted by {} in {}{}",
        short(&merged.0),
        b.agent,
        short(&main.0),
        if b.reason.is_empty() {
            String::new()
        } else {
            format!(": {}", b.reason)
        }
    )
    .chars()
    .take(200)
    .collect();
    let (status, _) = st
        .store
        .transition_with_record(
            task.id,
            Event::Revert,
            NewRecord {
                task_id: task.id,
                kind: RecordKind::Note,
                summary,
                body: String::new(),
                change_id: Some(main.clone()),
                supersedes: None,
                author_agent: b.agent,
                anchors: vec![],
            },
        )
        .await?;
    crate::backup::spawn_backup(&st, &task.repo);
    // Restore main's index first so the task overlay diffs against the post-revert tip.
    let changed = st.vcs.changed_paths(&task.repo, &pre, &main).await?;
    reindex_changed(&st, &task.repo, "main", &main, &changed).await?;
    if let Err(e) = rebuild_task_overlay(&st, &task).await {
        tracing::warn!(task = %task.slug, error = %e, "rebuilding the reverted task overlay failed");
    }
    Ok(Json(
        json!({ "status": status.as_str(), "main": main, "reverted": merged }),
    ))
}

pub(crate) async fn undo(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
    Json(b): Json<AgentBody>,
) -> ApiResult<Value> {
    let _guard = MERGE_LOCK.lock().await;
    let task = st.store.get_task(TaskId(id)).await?;
    next(task.status, Event::Undo)?;
    let state = st.store.merge_state(task.id).await?;
    let (Some(pre), Some(merged)) = (state.pre_merge.clone(), state.merge_commit.clone()) else {
        return Err(AvError::IllegalState("this task has no merge to undo".into()).into());
    };
    if state.merged_by.as_deref() != Some(b.agent.as_str()) {
        return Err(AvError::IllegalState(format!(
            "only the agent that merged ({}) can undo it",
            state.merged_by.unwrap_or_default()
        ))
        .into());
    }
    st.vcs.undo_merge(&task.repo, &pre, &merged).await?;
    st.store
        .set_merge_state(task.id, &MergeState::default())
        .await?;
    let status = st.store.transition(task.id, Event::Undo).await?;
    crate::backup::spawn_backup(&st, &task.repo);
    // Restore main's index first so the task overlay diffs against the undone tip.
    let changed = st.vcs.changed_paths(&task.repo, &pre, &merged).await?;
    reindex_changed(&st, &task.repo, "main", &pre, &changed).await?;
    if let Err(e) = rebuild_task_overlay(&st, &task).await {
        tracing::warn!(task = %task.slug, error = %e, "rebuilding the undone task overlay failed");
    }
    st.store
        .add_record(NewRecord {
            task_id: task.id,
            kind: RecordKind::Note,
            summary: format!("undo of merge {}", short(&merged.0)),
            body: String::new(),
            change_id: Some(pre.clone()),
            supersedes: None,
            author_agent: b.agent,
            anchors: vec![],
        })
        .await?;
    Ok(Json(json!({ "status": status.as_str(), "main": pre })))
}

/// Ends a task for good (any status but merged). Only its owner may; the records stay.
pub(crate) async fn abandon(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Json(b): Json<AgentBody>,
) -> ApiResult<Value> {
    let _guard = st.lock_task(TaskId(id)).await;
    let task = st.store.get_task(TaskId(id)).await?;
    fence(&st, &task, &headers).await?;
    next(task.status, Event::Abandon)?;
    if b.agent != task.owner_agent {
        return Err(AvError::IllegalState(format!(
            "only the owner ({}) can abandon task {}",
            task.owner_agent, task.slug
        ))
        .into());
    }
    let status = st.store.transition(task.id, Event::Abandon).await?;
    drop_task_scope(&st, &task).await;
    Ok(Json(json!({ "status": status.as_str() })))
}

/// A merged or abandoned task's index scope is no longer needed. A failure only leaves rows
/// behind, so it is logged, never returned.
async fn drop_task_scope(st: &AppState, task: &Task) {
    if let Err(e) = st.index.drop_scope(&task.repo, &task.slug).await {
        tracing::warn!(task = %task.slug, error = %e, "dropping the task's index scope failed");
    }
}

#[derive(Deserialize)]
pub(crate) struct HandoffBody {
    agent: String,
    #[serde(flatten)]
    state: HandoffState,
    budget: Option<u32>,
    family: Option<AgentFamily>,
}

/// Snapshots, checks the record gate on the records that already exist, then stores the state as a
/// `handoff` record and moves the task to `handed_off` in one transaction. Returns the bundle the
/// next agent starts from.
pub(crate) async fn handoff(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Json(b): Json<HandoffBody>,
) -> ApiResult<Value> {
    let _guard = st.lock_task(TaskId(id)).await;
    let task = st.store.get_task(TaskId(id)).await?;
    fence(&st, &task, &headers).await?;
    next(task.status, Event::Handoff)?;
    if b.agent != task.owner_agent {
        return Err(AvError::IllegalState(format!(
            "only the owner ({}) can hand off task {}; another agent can claim it after a \
             handoff or once its lease expires",
            task.owner_agent, task.slug
        ))
        .into());
    }
    if !st.to_local_path(&task.workspace_path).is_dir() {
        return Err(AvError::IllegalState(format!(
            "the workspace of task {} is missing ({}); restore it before handing off. The \
             task stays {} and its records are unchanged",
            task.slug,
            task.workspace_path.display(),
            task.status.as_str()
        ))
        .into());
    }
    let (head, _, _) = snapshot_and_index(&st, &task).await?;
    let task = st.store.get_task(task.id).await?;
    // before the handoff record exists, so it cannot satisfy its own gate
    gate(&st, &task, "handing off").await?;
    let summary = [&b.state.remaining, &b.state.done]
        .into_iter()
        .map(|s| s.trim())
        .find(|s| !s.is_empty())
        .unwrap_or("handoff")
        .chars()
        .take(200)
        .collect();
    let (status, record) = st
        .store
        .transition_with_record(
            task.id,
            Event::Handoff,
            NewRecord {
                task_id: task.id,
                kind: RecordKind::Handoff,
                summary,
                body: serde_json::to_string(&b.state).map_err(|e| AvError::Store(e.to_string()))?,
                change_id: Some(head.clone()),
                supersedes: None,
                author_agent: b.agent,
                anchors: vec![],
            },
        )
        .await?;
    // snapshot_and_index already reconciled; re-check in case main moved during the handoff write.
    let task = st.store.get_task(task.id).await?;
    reconcile_task_overlay(&st, &task).await?;
    let bundle = st
        .packer
        .handoff_bundle(BundleRequest {
            task_id: task.id,
            budget_tokens: b.budget.unwrap_or(BUNDLE_BUDGET),
            family: b.family.unwrap_or_default(),
        })
        .await?;
    Ok(Json(json!({
        "status": status.as_str(),
        "head": head,
        "record": record.id,
        "bundle": bundle,
    })))
}

#[derive(Deserialize)]
pub(crate) struct ClaimBody {
    agent: String,
    budget: Option<u32>,
    family: Option<AgentFamily>,
}

/// Takes over a handed-off task, or an open one whose lease expired: the claimant becomes the owner
/// under a new generation (older writers are fenced off) and gets a fresh session, so its first
/// context pack is sent in full, plus the handoff bundle.
pub(crate) async fn claim(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
    Json(b): Json<ClaimBody>,
) -> ApiResult<Value> {
    if b.agent.trim().is_empty() {
        return Err(AvError::Invalid("agent must not be empty".into()).into());
    }
    // no fenced write of the previous owner can run between its check and this claim
    let _guard = st.lock_task(TaskId(id)).await;
    let task = st
        .store
        .claim_task(TaskId(id), &b.agent, st.lease_secs)
        .await?;
    reconcile_task_overlay(&st, &task).await?;
    let bundle = st
        .packer
        .handoff_bundle(BundleRequest {
            task_id: task.id,
            budget_tokens: b.budget.unwrap_or(BUNDLE_BUDGET),
            family: b.family.unwrap_or_default(),
        })
        .await?;
    Ok(Json(json!({
        "generation": task.generation,
        "session": Uuid::new_v4().to_string(),
        "task": task,
        "bundle": bundle,
    })))
}

fn short(hex: &str) -> &str {
    &hex[..hex.len().min(12)]
}
