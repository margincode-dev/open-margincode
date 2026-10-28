//! `GET /ui`: a read-only dashboard for people watching agents work. One static page, built into
//! the binary; it reads `/v1` with the token the viewer types in, so it holds no data itself.

use axum::http::header;
use axum::response::IntoResponse;

const PAGE: &str = include_str!("ui.html");

/// Inline script and style only, and fetches back to this daemon.
const CSP: &str = "default-src 'none'; script-src 'unsafe-inline'; style-src 'unsafe-inline'; \
                   connect-src 'self'; img-src data:; base-uri 'none'; form-action 'none'; \
                   frame-ancestors 'none'";

pub(crate) async fn page() -> impl IntoResponse {
    (
        [
            (header::CONTENT_TYPE, "text/html; charset=utf-8"),
            (header::CONTENT_SECURITY_POLICY, CSP),
            (header::X_CONTENT_TYPE_OPTIONS, "nosniff"),
            (header::REFERRER_POLICY, "no-referrer"),
        ],
        PAGE,
    )
}
