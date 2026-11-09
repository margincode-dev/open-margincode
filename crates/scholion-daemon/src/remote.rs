//! Plan 14, remote workspaces by checkpoint-by-patch. An agent on another host works in a local
//! checkout of a task (`av remote clone`) and sends each checkpoint as a git-format patch against
//! the head it last synced to. The daemon still owns and snapshots every workspace: it applies the
//! patch to the files of the task's workspace (in process, `scholion-vcs`, no git or other command)
//! and snapshots it as a local checkpoint would.
//!
//! - `POST /v1/tasks/{id}/checkpoint?base=<change>` with the patch as the body (`api::checkpoint`
//!   hands it here). The workspace is snapshotted first, so edits made on the daemon's host since
//!   the last checkpoint are recorded and make the base stale. A base whose files are not the
//!   head's is `409` with `head_change` (and header `X-Av-Head`): the client pulls and resends.
//! - `GET /v1/tasks/{id}/patch?from=<change>`: the task's head as a patch from `from` (default: the
//!   empty tree, a whole checkout), headers `X-Av-Head` and `X-Av-Tree`.

use axum::Router;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use scholion_core::{AvError, ChangeId, Task, TaskId};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::AppState;
use crate::api::{index_head, resolve_at, snapshot_and_index};
use crate::error::ApiError;

/// Largest patch body a remote checkpoint takes (`413` above it).
pub const MAX_PATCH_BYTES: usize = 64 * 1024 * 1024;

pub(crate) fn routes() -> Router<AppState> {
    Router::new().route("/v1/tasks/{id}/patch", get(patch))
}

/// The task's head: its last checkpoint, else its base.
fn head_of(task: &Task) -> ChangeId {
    if task.head_change.0.is_empty() {
        task.base_change.clone()
    } else {
        task.head_change.clone()
    }
}

/// Whether `tree` is a git tree id (a conflicted tree has none).
fn git_tree(tree: &str) -> Option<&str> {
    (tree.len() == 40 && tree.bytes().all(|b| b.is_ascii_hexdigit())).then_some(tree)
}

/// A remote checkpoint of `task` against `base`. The caller holds `lock_task` and ran `fence`.
pub(crate) async fn checkpoint(
    st: &AppState,
    task: Task,
    base: &str,
    body: &[u8],
) -> Result<Response, ApiError> {
    let base = base.trim().to_ascii_lowercase();
    if base.is_empty() || !base.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(AvError::Invalid(format!("bad base {base:?}; expected a commit id")).into());
    }
    let base = ChangeId(base);
    let base_tree = match st.vcs.tree_id(&task.repo, &base).await {
        Ok(t) => t,
        Err(AvError::Vcs(_) | AvError::NotFound(_)) => {
            return Err(AvError::NotFound(format!(
                "base {} is not a commit of repo {}; clone the task again (`av remote clone`)",
                base.0, task.repo
            ))
            .into());
        }
        Err(e) => return Err(e.into()),
    };
    // edits on the daemon's host since the last checkpoint are recorded first, as any snapshot would
    let mut task = task;
    let head = st.vcs.snapshot(&task.repo, &task.slug).await?;
    if head != head_of(&task) {
        index_head(st, &task, head.clone()).await?;
        task = st.store.get_task(task.id).await?;
    }
    let head_tree = st.vcs.tree_id(&task.repo, &head).await?;
    if head != base && head_tree != base_tree {
        let message = format!(
            "stale base: the workspace is at {head}, not {base}; fetch GET /v1/tasks/{id}/patch?from={base}, \
             apply it to your checkout, then send the checkpoint again (`av remote pull` does it)",
            head = head.0,
            base = base.0,
            id = task.id.0,
        );
        let mut resp = (
            StatusCode::CONFLICT,
            axum::Json(json!({
                "error": message, "head_change": head, "base": base,
                "tree": git_tree(&head_tree),
            })),
        )
            .into_response();
        if let Ok(v) = HeaderValue::from_str(&head.0) {
            resp.headers_mut().insert("x-av-head", v);
        }
        return Ok(resp);
    }
    let applied = st.vcs.apply_patch(&task.repo, &task.slug, body).await?;
    let (head, changed, warnings) = snapshot_and_index(st, &task).await?;
    let tree = st.vcs.tree_id(&task.repo, &head).await?;
    let answer = json!({
        "head_change": head, "changed": changed, "warnings": warnings,
        "applied": applied, "tree": git_tree(&tree),
    });
    let answer =
        crate::ext::with_hook(st.checkpoint_hook.as_ref(), &task, &head, &changed, answer).await;
    Ok(axum::Json(answer).into_response())
}

#[derive(Deserialize)]
struct PatchQuery {
    from: Option<String>,
}

/// `GET /v1/tasks/{id}/patch?from=`: the task's head as a git-format patch from `from` (`main`,
/// `task:<slug>` or a commit id; none: the empty tree). Reads the store only, no snapshot.
async fn patch(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
    Query(q): Query<PatchQuery>,
) -> Result<Response, ApiError> {
    let task = st.store.get_task(TaskId(id)).await?;
    let head = head_of(&task);
    let from = match q.from.as_deref().map(str::trim).filter(|f| !f.is_empty()) {
        Some(f) => Some(resolve_at(&st, &task.repo, Some(f)).await?),
        None => None,
    };
    let bytes = st.vcs.diff_patch(&task.repo, from.as_ref(), &head).await?;
    let tree = st.vcs.tree_id(&task.repo, &head).await?;
    let mut resp = (
        StatusCode::OK,
        [(
            header::CONTENT_TYPE,
            HeaderValue::from_static("text/x-diff"),
        )],
        bytes,
    )
        .into_response();
    let h = resp.headers_mut();
    if let Ok(v) = HeaderValue::from_str(&head.0) {
        h.insert("x-av-head", v);
    }
    if let Some(t) = git_tree(&tree).and_then(|t| HeaderValue::from_str(t).ok()) {
        h.insert("x-av-tree", t);
    }
    Ok(resp)
}
