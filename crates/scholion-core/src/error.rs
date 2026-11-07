use thiserror::Error;

pub type Result<T> = std::result::Result<T, AvError>;

#[derive(Debug, Error)]
pub enum AvError {
    #[error("summary is {0} chars; the limit is 200")]
    SummaryTooLong(usize),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("invalid input: {0}")]
    Invalid(String),
    /// A well-formed request that the task's current state does not allow (HTTP 422).
    #[error("not allowed now: {0}")]
    IllegalState(String),
    /// A well-formed request that would overwrite what another writer owns (HTTP 409).
    #[error("conflict: {0}")]
    Conflict(String),
    /// The disk under the workspace root is below the daemon's free-space floor
    /// (`AV_DISK_MIN_FREE_GB`), so no new task workspace is made (HTTP 507).
    #[error("not enough disk space: {0}")]
    InsufficientStorage(String),
    #[error("vcs error: {0}")]
    Vcs(String),
    #[error("store error: {0}")]
    Store(String),
    #[error("index error: {0}")]
    Index(String),
    #[error("semantic error: {0}")]
    Semantic(String),
}
