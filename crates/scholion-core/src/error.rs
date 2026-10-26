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
    #[error("vcs error: {0}")]
    Vcs(String),
    #[error("store error: {0}")]
    Store(String),
    #[error("index error: {0}")]
    Index(String),
    #[error("semantic error: {0}")]
    Semantic(String),
}
