//! `av attention`: what needs a person or an agent now (`GET /v1/attention`), worst first, one
//! line per item.

use std::fmt::Write;

use chrono::{DateTime, Utc};
use serde_json::Value;

use crate::disk::age;

fn s(v: &Value) -> &str {
    v.as_str().unwrap_or("")
}

fn time(v: &Value) -> Option<DateTime<Utc>> {
    v.as_str()?.parse().ok()
}

/// `3 main_red            2h  demo/fix-login  pytest exited 1 on main · since @1a2b3c4d`.
/// How long is reckoned on the daemon's clock (`generated_at`), so a skewed local clock does
/// not change it.
pub fn render(v: &Value) -> String {
    let items = v["items"].as_array().cloned().unwrap_or_default();
    if items.is_empty() {
        return "nothing needs attention\n".into();
    }
    let now = time(&v["generated_at"]);
    let mut o = String::new();
    for i in &items {
        let how_long = match (now, time(&i["since"])) {
            (Some(now), Some(since)) => age((now - since).num_seconds().max(0) as u64),
            _ => "?".into(),
        };
        let place = match (i["repo"].as_str(), i["slug"].as_str()) {
            (Some(r), Some(slug)) => format!("{r}/{slug}"),
            (Some(r), None) => r.to_owned(),
            (None, Some(slug)) => slug.to_owned(),
            (None, None) => "-".into(),
        };
        let mut line = format!(
            "{} {:<19} {:>4}  {place}  {}",
            i["severity"],
            s(&i["kind"]),
            how_long,
            s(&i["title"])
        );
        if !s(&i["detail"]).is_empty() {
            let _ = write!(line, " · {}", s(&i["detail"]));
        }
        let _ = writeln!(o, "{line}");
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn one_line_per_item_with_how_long_on_the_daemons_clock() {
        let v = json!({
            "generated_at": "2026-11-05T12:00:00Z",
            "items": [
                {"kind": "main_red", "severity": 3, "repo": "demo", "slug": "fix-login",
                 "since": "2026-11-05T10:00:00Z", "title": "pytest exited 1 on main",
                 "detail": "since @1a2b3c4d · task fix-login"},
                {"kind": "unroutable_backlog", "severity": 1, "repo": "demo",
                 "since": "2026-11-05T11:59:30Z", "title": "Train the model", "detail": "needs gpu"},
            ],
        });
        assert_eq!(
            render(&v),
            "3 main_red              2h  demo/fix-login  pytest exited 1 on main · since @1a2b3c4d · task fix-login\n\
             1 unroutable_backlog   30s  demo  Train the model · needs gpu\n"
        );
        assert_eq!(
            render(&json!({"items": [], "generated_at": "2026-11-05T12:00:00Z"})),
            "nothing needs attention\n"
        );
    }
}
