//! `AV_TOKEN` (Plan 3 Task 9): when set, every route but `/healthz` and `/ui` needs
//! `Authorization: Bearer <token>`. Unset means open, as before.

use axum::Json;
use axum::extract::{Request, State};
use axum::http::{StatusCode, header};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;

use crate::AppState;

/// Compares SHA-256 digests in constant time, so neither the content nor the length of the
/// expected token leaks through timing.
fn token_matches(given: &str, expected: &str) -> bool {
    let (a, b) = (
        Sha256::digest(given.as_bytes()),
        Sha256::digest(expected.as_bytes()),
    );
    a.ct_eq(&b).into()
}

pub(crate) async fn require_token(
    State(st): State<AppState>,
    req: Request,
    next: Next,
) -> Response {
    let Some(expected) = st.token.as_deref() else {
        return next.run(req).await;
    };
    let given = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));
    match given {
        Some(t) if token_matches(t.trim(), expected) => next.run(req).await,
        _ => (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "error": "missing or wrong token: send `Authorization: Bearer <AV_TOKEN>` (the av CLI sends $AV_TOKEN)"
            })),
        )
            .into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_exact_token_matches() {
        assert!(token_matches("sekrit", "sekrit"));
        assert!(!token_matches("sekri", "sekrit"));
        assert!(!token_matches("sekrit2", "sekrit"));
        assert!(!token_matches("", "sekrit"));
    }
}
