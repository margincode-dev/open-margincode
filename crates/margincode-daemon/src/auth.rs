//! `AV_TOKEN`: when set, every route but `/healthz` and `/ui` needs `Authorization: Bearer <token>`;
//! unset means open to this machine, so a `Host` or `Origin` from another site is refused (`foreign`).

use std::net::IpAddr;

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
        return match foreign(&req, &st.allowed_hosts) {
            None => next.run(req).await,
            Some(error) => (StatusCode::FORBIDDEN, Json(json!({ "error": error }))).into_response(),
        };
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

/// The host of a `Host` value or an origin's authority: lower case, without the port, the brackets
/// of an IPv6 address or a trailing dot.
pub(crate) fn host_name(authority: &str) -> String {
    let a = authority.trim().to_ascii_lowercase();
    let host = match a.strip_prefix('[') {
        Some(rest) => rest.split(']').next().unwrap_or_default(),
        None => match a.rsplit_once(':') {
            Some((h, port)) if port.bytes().all(|b| b.is_ascii_digit()) => h,
            _ => &a,
        },
    };
    host.trim_end_matches('.').to_owned()
}

fn is_loopback(host: &str) -> bool {
    host == "localhost"
        || host.ends_with(".localhost")
        || host.parse::<IpAddr>().is_ok_and(|ip| ip.is_loopback())
}

/// Why a token-less daemon refuses `req`, if it does. A DNS-rebinding page sends its own site's
/// dotted DNS name as `Host`, so an IP, a single-label name (a compose service) or Docker's host
/// names pass; an `Origin` must be this machine, an allowed host, or the request's own host.
fn foreign(req: &Request, allowed: &[String]) -> Option<String> {
    let header = |name| req.headers().get(name).and_then(|v| v.to_str().ok());
    let sent_host = header(header::HOST).or_else(|| req.uri().authority().map(|a| a.as_str()));
    let ours = |h: &str| is_loopback(h) || allowed.iter().any(|a| a == h);
    if let Some(host) = sent_host.map(host_name)
        && !(ours(&host)
            || host.parse::<IpAddr>().is_ok()
            || !host.contains('.')
            || matches!(
                host.as_str(),
                "host.docker.internal" | "gateway.docker.internal"
            ))
    {
        return Some(format!(
            "Host {host:?} is refused: without AV_TOKEN the daemon answers only this machine. Set \
             AV_TOKEN (and the same AV_TOKEN for clients), or name the host in AV_ALLOWED_HOSTS"
        ));
    }
    let origin = header(header::ORIGIN)?;
    let authority = origin
        .split_once("://")
        .map(|(_, rest)| rest.trim_end_matches('/'));
    let same_site =
        authority.is_some_and(|a| sent_host.is_some_and(|h| a.eq_ignore_ascii_case(h.trim())));
    if authority.is_some_and(|a| ours(&host_name(a))) || same_site {
        return None;
    }
    Some(format!(
        "a request from {origin:?} is refused: without AV_TOKEN the daemon answers only pages of \
         this machine. Set AV_TOKEN, or name the origin's host in AV_ALLOWED_HOSTS"
    ))
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

    #[test]
    fn host_names_drop_port_brackets_case_and_trailing_dot() {
        for (given, name) in [
            ("127.0.0.1:7420", "127.0.0.1"),
            ("LocalHost.:80", "localhost"),
            ("[::1]:7420", "::1"),
            ("[FD00::1]", "fd00::1"),
            ("daemon", "daemon"),
            (" Host.Docker.Internal:7420 ", "host.docker.internal"),
        ] {
            assert_eq!(host_name(given), name, "{given}");
        }
    }
}
