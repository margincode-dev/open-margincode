//! Questions for a person: `ask` writes a `question` record and tells the agent to proceed or
//! park, from its level; an answer is a `decision` that supersedes the question.

use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::routing::{get, post};
use axum::{Json, Router};
use margincode_core::question::{
    Asked, MAX_COMMENT_CHARS, MAX_OPEN_QUESTIONS, Question, QuestionState, Undo, answer_record,
    same_agent,
};
use margincode_core::*;
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use crate::AppState;
use crate::api::{MAX_SUPERSEDE_CHAIN, guard_supersede, rec_handle, snapshot_and_index};
use crate::error::ApiResult;
use crate::flow::fence;

pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .route("/v1/tasks/{id}/questions", post(ask))
        .route("/v1/questions", get(open))
        .route("/v1/questions/{id}/answer", post(answer))
}

/// Every field may be missing or `null` (an MCP client sends what the agent gave): `checked`
/// says what is wrong.
#[derive(Deserialize)]
struct AskBody {
    question: Option<String>,
    options: Option<Vec<String>>,
    recommended: Option<String>,
    why: Option<String>,
    depends: Option<String>,
    undo: Option<String>,
    state: Option<QuestionState>,
    anchors: Option<Vec<Anchor>>,
    agent: Option<String>,
}

async fn ask(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Json(b): Json<AskBody>,
) -> ApiResult<Value> {
    let question = Question {
        question: b.question.unwrap_or_default(),
        options: b.options.unwrap_or_default(),
        recommended: b.recommended.unwrap_or_default(),
        why: b.why.unwrap_or_default(),
        depends: b.depends.unwrap_or_default(),
        undo: Undo::named(b.undo.as_deref().unwrap_or_default())?,
        state: b.state.unwrap_or_default(),
    }
    .checked()?;
    let anchors = b.anchors.unwrap_or_default();
    let agent = b.agent.unwrap_or_default().trim().to_owned();
    if agent.is_empty() {
        return Err(AvError::Invalid("agent must name who asks".into()).into());
    }
    let _guard = st.lock_task(TaskId(id)).await;
    let task = st.store.get_task(TaskId(id)).await?;
    fence(&st, &task, &headers).await?;
    if task.status != TaskStatus::Open {
        return Err(AvError::IllegalState(format!(
            "questions are asked on an open task; {} is {}",
            task.slug,
            task.status.as_str()
        ))
        .into());
    }
    let open: Vec<String> = st
        .store
        .task_records(task.id)
        .await?
        .into_iter()
        .filter(|r| r.kind == RecordKind::Question && r.superseded_by.is_none())
        .map(|r| rec_handle(r.id))
        .collect();
    if open.len() >= MAX_OPEN_QUESTIONS {
        return Err(AvError::IllegalState(format!(
            "task {} already has {} open questions ({}): answer or withdraw one first (a person \
             answers with `av answer <id> <option>`; withdraw one with a note that supersedes it)",
            task.slug,
            open.len(),
            open.join(", ")
        ))
        .into());
    }
    // the anchored code is hashed as it is now, as `record` does
    snapshot_and_index(&st, &task).await?;
    let level = st
        .store
        .ask_level(&task.owner_agent)
        .await?
        .unwrap_or(st.ask_level);
    let asked = Asked {
        instruction: level.instruction(question.undo),
        question,
        level,
    };
    let hashes = st.packer.anchor_hashes(task.id, &anchors).await?;
    let record = st
        .store
        .add_record_with(
            NewRecord {
                task_id: task.id,
                kind: RecordKind::Question,
                summary: asked.question.question.clone(),
                body: serde_json::to_string(&asked).map_err(|e| AvError::Store(e.to_string()))?,
                change_id: None,
                supersedes: None,
                author_agent: agent.clone(),
                anchors,
            },
            &hashes,
            None,
        )
        .await?;
    let payload = json!({"record": record.id, "level": asked.level.as_str(),
                         "instruction": asked.instruction.as_str(), "by": agent});
    if let Err(e) = st
        .store
        .add_event(&task.repo, "question:asked", Some(task.id), payload)
        .await
    {
        tracing::warn!(error = %e, record = %record.id, "question:asked event not written");
    }
    Ok(Json(json!({
        "id": record.id,
        "handle": rec_handle(record.id),
        "level": asked.level.as_str(),
        "instruction": asked.instruction.as_str(),
        "told": asked.told(),
        "open": open.len() + 1,
    })))
}

#[derive(Deserialize)]
struct OpenQuery {
    repo: Option<String>,
}

async fn open(State(st): State<AppState>, Query(q): Query<OpenQuery>) -> ApiResult<Value> {
    let repo = q.repo.as_deref().map(str::trim).filter(|r| !r.is_empty());
    let mut out = vec![];
    for r in st.store.open_questions(repo).await? {
        let task = st.store.get_task(r.task_id).await?;
        let asked: Option<Asked> = serde_json::from_str(&r.body).ok();
        out.push(json!({
            "id": r.id,
            "handle": rec_handle(r.id),
            "asked_by": r.author_agent,
            "asked_at": r.created_at,
            "task": {"id": task.id, "repo": task.repo, "slug": task.slug,
                     "status": task.status.as_str()},
            "question": asked,
        }));
    }
    Ok(Json(json!({ "questions": out })))
}

/// The question a person's answer `rec` answers (through any corrections), when it is one.
pub(crate) async fn answered_question(st: &AppState, rec: &Record) -> Result<Option<Record>> {
    let mut at = rec.clone();
    // the answer and its corrections: one step more than the corrections `answer` allows
    for _ in 0..=MAX_SUPERSEDE_CHAIN {
        if at.kind != RecordKind::Decision {
            return Ok(None);
        }
        let Some(earlier) = at.supersedes else {
            return Ok(None);
        };
        at = st.store.get_record(earlier).await?;
        if at.kind == RecordKind::Question {
            return Ok(Some(at));
        }
    }
    Ok(None)
}

#[derive(Deserialize)]
struct AnswerBody {
    #[serde(default)]
    option: String,
    /// Replace the answer a question already has.
    #[serde(default)]
    correct: bool,
    #[serde(default)]
    comment: String,
    #[serde(default)]
    by: String,
}

async fn answer(
    State(st): State<AppState>,
    Path(named): Path<String>,
    Json(b): Json<AnswerBody>,
) -> ApiResult<Value> {
    let by = b.by.trim().to_owned();
    if by.is_empty() {
        return Err(AvError::Invalid("by must name who answers".into()).into());
    }
    let comment = b.comment.trim();
    if comment.chars().count() > MAX_COMMENT_CHARS {
        return Err(AvError::Invalid(format!(
            "the comment is {} characters; the limit is {MAX_COMMENT_CHARS}",
            comment.chars().count()
        ))
        .into());
    }
    let named = named.trim();
    let found = st
        .store
        .find_record(named.strip_prefix("rec:").unwrap_or(named))
        .await?;
    let shown = rec_handle(found.id);
    if found.kind != RecordKind::Question {
        return Err(AvError::IllegalState(format!(
            "{shown} is a {}, not a question",
            found.kind.as_str()
        ))
        .into());
    }
    let _guard = st.lock_task(found.task_id).await;
    let q = st.store.get_record(found.id).await?;
    let task = st.store.get_task(q.task_id).await?;
    if matches!(task.status, TaskStatus::Merged | TaskStatus::Abandoned) {
        return Err(AvError::IllegalState(format!(
            "task {} is {}: it is finished, so {shown} is no longer answered there; record the \
             decision on a new task",
            task.slug,
            task.status.as_str()
        ))
        .into());
    }
    let mut earlier = None;
    if let Some(next) = q.superseded_by {
        let mut live = st.store.get_record(next).await?;
        if live.kind != RecordKind::Decision {
            return Err(AvError::Conflict(format!(
                "{shown} was withdrawn: {} ({})",
                live.summary,
                rec_handle(live.id)
            ))
            .into());
        }
        let mut corrections = 0;
        while corrections < MAX_SUPERSEDE_CHAIN {
            let Some(n) = live.superseded_by else { break };
            live = st.store.get_record(n).await?;
            corrections += 1;
        }
        if !b.correct {
            return Err(AvError::Conflict(format!(
                "{shown} is already answered: {} ({}, by {}); to change the answer, answer again \
                 with `av answer {shown} <option> --correct`",
                live.summary,
                rec_handle(live.id),
                live.author_agent
            ))
            .into());
        }
        if corrections >= MAX_SUPERSEDE_CHAIN {
            return Err(AvError::IllegalState(format!(
                "{shown}'s answer was corrected {MAX_SUPERSEDE_CHAIN} times, the most it may be; \
                 record the decision on a new task"
            ))
            .into());
        }
        earlier = Some(live);
    }
    let asked: Asked = serde_json::from_str(&q.body)
        .map_err(|e| AvError::Store(format!("{shown}'s body is not a question: {e}")))?;
    let profiles = st.store.list_profiles().await?;
    if same_agent(&profiles, &by, &q.author_agent) {
        return Err(AvError::IllegalState(format!(
            "{by} asked {shown}: an agent does not answer its own question. A person answers it, \
             or the agent withdraws it with a note that supersedes it"
        ))
        .into());
    }
    if same_agent(&profiles, &by, &task.owner_agent) {
        return Err(AvError::IllegalState(format!(
            "{by} owns task {}: the owner does not answer a question on its own task. A person \
             answers {shown}, or the owner withdraws it with a note that supersedes it",
            task.slug
        ))
        .into());
    }
    let numbered: Vec<String> = asked
        .question
        .options
        .iter()
        .enumerate()
        .map(|(i, o)| format!("{} {o}", i + 1))
        .collect();
    let option = asked
        .question
        .option(&b.option)
        .ok_or_else(|| {
            AvError::Invalid(format!(
                "{:?} is not an option of {shown}; its options: {}",
                b.option.trim(),
                numbered.join(", ")
            ))
        })?
        .to_owned();
    let replaced = earlier.as_ref().unwrap_or(&q);
    guard_supersede(&st, &task, replaced, &by, true).await?;
    let (summary, body) = answer_record(&asked.question, &option, comment);
    let decision = st
        .store
        .add_record_with(
            NewRecord {
                task_id: q.task_id,
                kind: RecordKind::Decision,
                summary,
                body,
                change_id: None,
                supersedes: Some(replaced.id),
                author_agent: by.clone(),
                anchors: q.anchors.clone(),
            },
            &q.anchor_hashes,
            None,
        )
        .await?;
    let recommended = option == asked.question.recommended;
    for (kind, payload) in [
        (
            "record:superseded",
            json!({"old": replaced.id, "new": decision.id, "by": by}),
        ),
        (
            "question:answered",
            json!({"question": q.id, "decision": decision.id, "option": option, "by": by,
                   "recommended": recommended, "corrects": earlier.as_ref().map(|r| r.id)}),
        ),
    ] {
        if let Err(e) = st
            .store
            .add_event(&task.repo, kind, Some(task.id), payload)
            .await
        {
            tracing::warn!(error = %e, record = %decision.id, "{kind} event not written");
        }
    }
    Ok(Json(json!({
        "question": q.id,
        "decision": decision,
        "option": option,
        "recommended": recommended,
    })))
}
