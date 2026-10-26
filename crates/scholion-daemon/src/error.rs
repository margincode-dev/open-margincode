use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use scholion_core::AvError;
use serde_json::json;

pub(crate) struct ApiError(pub AvError);

impl From<AvError> for ApiError {
    fn from(e: AvError) -> Self {
        ApiError(e)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match &self.0 {
            AvError::SummaryTooLong(_) | AvError::Invalid(_) => StatusCode::BAD_REQUEST,
            AvError::NotFound(_) => StatusCode::NOT_FOUND,
            AvError::IllegalState(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AvError::Vcs(_) | AvError::Store(_) | AvError::Index(_) | AvError::Semantic(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        if status.is_server_error() {
            tracing::error!(error = %self.0, "request failed");
        }
        (status, Json(json!({ "error": self.0.to_string() }))).into_response()
    }
}

pub(crate) type ApiResult<T> = Result<Json<T>, ApiError>;
