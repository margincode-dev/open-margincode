//! `av context stats` and `av context backfill`: how good merged tasks' packs were
//! (`GET /v1/repos/{name}/context/outcomes`), medians per packer version.

use std::fmt::Write;

use serde_json::Value;

fn s(v: &Value) -> &str {
    v.as_str().unwrap_or("")
}

fn n(v: &Value) -> u64 {
    v.as_u64().unwrap_or(0)
}

fn plural(n: u64) -> &'static str {
    if n == 1 { "" } else { "s" }
}

fn share(v: &Value) -> String {
    v.as_f64()
        .map_or_else(|| "-".into(), |x| format!("{:.0}%", x * 100.0))
}

fn median(v: &Value) -> String {
    match v.as_f64() {
        Some(x) if x.fract() == 0.0 => format!("{x:.0}"),
        Some(x) => format!("{x:.1}"),
        None => "-".into(),
    }
}

pub fn render(v: &Value) -> String {
    let (measured, reverted, no_reads) = (n(&v["measured"]), n(&v["reverted"]), n(&v["no_reads"]));
    let mut o = format!(
        "{}: {measured} merged task{} measured",
        s(&v["repo"]),
        plural(measured)
    );
    if reverted > 0 {
        let _ = write!(o, ", {reverted} reverted (left out)");
    }
    if no_reads > 0 {
        let _ = write!(o, "; no read reached the daemon for {no_reads}");
    }
    o.push('\n');
    let versions = v["by_packer"].as_array().cloned().unwrap_or_default();
    if !versions.is_empty() {
        o.push_str(
            "packer  tasks  changed share  used share  code tokens  changed  missed  named only  opened, not sent\n",
        );
        for p in &versions {
            let version = p["packer_version"]
                .as_i64()
                .map_or_else(|| "none".into(), |x| x.to_string());
            let _ = writeln!(
                o,
                "{version:>6}  {:>5}  {:>13}  {:>10}  {:>11}  {:>7}  {:>6}  {:>10}  {:>16}",
                n(&p["tasks"]),
                share(&p["changed_share"]),
                share(&p["used_share"]),
                median(&p["code_tokens"]),
                median(&p["files_changed"]),
                median(&p["files_missed"]),
                median(&p["files_named_only"]),
                median(&p["opened_not_sent"]),
            );
        }
        o.push_str(
            "Medians per task. A share is over the tasks whose packs sent code; files are the modified, non-test source files.\n",
        );
    }
    for r in v["rows"].as_array().into_iter().flatten() {
        let code = r["code_tokens"].as_f64().unwrap_or(0.0);
        let of = |k: &str| {
            if code > 0.0 {
                format!("{:.0}%", r[k].as_f64().unwrap_or(0.0) / code * 100.0)
            } else {
                "-".into()
            }
        };
        let mut line = format!(
            "{}  v{}  {} pack{}, {} code tokens, {} changed, {} used; {} files changed, {} missed, {} named only, {} opened not sent",
            s(&r["slug"]),
            r["packer_version"]
                .as_i64()
                .map_or_else(|| "-".into(), |x| x.to_string()),
            n(&r["packs"]),
            plural(n(&r["packs"])),
            n(&r["code_tokens"]),
            of("code_tokens_changed"),
            of("code_tokens_used"),
            n(&r["files_changed"]),
            n(&r["files_missed"]),
            n(&r["files_named_only"]),
            n(&r["opened_not_sent"]),
        );
        if r["reverted"] == true {
            line.push_str(" (reverted)");
        }
        let missed: Vec<&str> = r["missed_paths"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .collect();
        if !missed.is_empty() {
            let _ = write!(line, " · missed: {}", missed.join(", "));
        }
        let _ = writeln!(o, "{line}");
    }
    o
}

pub fn render_backfill(v: &Value) -> String {
    let computed = n(&v["computed"]);
    let mut o = format!("measured {computed} merged task{}", plural(computed));
    let failed = v["failed"].as_array().cloned().unwrap_or_default();
    if !failed.is_empty() {
        let _ = write!(o, "; {} failed", failed.len());
    }
    if v["more"] == true {
        o.push_str("; run again for more");
    }
    o.push('\n');
    for f in &failed {
        let _ = writeln!(o, "  {}: {}", s(&f["task"]), s(&f["error"]));
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn stats_print_a_line_per_packer_version_and_one_per_task() {
        let v = json!({
            "repo": "demo", "measured": 3, "reverted": 1, "no_reads": 2,
            "by_packer": [
                {"packer_version": 11, "tasks": 3, "no_code": 0, "no_reads": 2,
                 "changed_share": 0.42, "used_share": 0.6, "code_tokens": 310.0,
                 "files_changed": 2.0, "files_missed": 1.0, "files_named_only": 0.0,
                 "opened_not_sent": 0.5},
            ],
            "rows": [
                {"slug": "louder-hello", "packer_version": 11, "packs": 2, "code_tokens": 200,
                 "code_tokens_changed": 80, "code_tokens_used": 120, "files_changed": 2,
                 "files_missed": 1, "files_named_only": 0, "opened_not_sent": 1,
                 "missed_paths": ["app/util.py"], "reverted": false},
                {"slug": "quiet", "packer_version": null, "packs": 0, "code_tokens": 0,
                 "code_tokens_changed": 0, "code_tokens_used": 0, "files_changed": 1,
                 "files_missed": 1, "files_named_only": 0, "opened_not_sent": 0,
                 "missed_paths": [], "reverted": true},
            ],
        });
        assert_eq!(
            render(&v),
            "demo: 3 merged tasks measured, 1 reverted (left out); no read reached the daemon for 2\n\
             packer  tasks  changed share  used share  code tokens  changed  missed  named only  opened, not sent\n\
             \x20   11      3            42%         60%          310        2       1           0               0.5\n\
             Medians per task. A share is over the tasks whose packs sent code; files are the modified, non-test source files.\n\
             louder-hello  v11  2 packs, 200 code tokens, 40% changed, 60% used; 2 files changed, 1 missed, 0 named only, 1 opened not sent · missed: app/util.py\n\
             quiet  v-  0 packs, 0 code tokens, - changed, - used; 1 files changed, 1 missed, 0 named only, 0 opened not sent (reverted)\n"
        );
        assert_eq!(
            render(
                &json!({"repo": "demo", "measured": 0, "reverted": 0, "no_reads": 0, "by_packer": []})
            ),
            "demo: 0 merged tasks measured\n"
        );
    }

    #[test]
    fn backfill_says_how_many_it_measured_and_what_failed() {
        assert_eq!(
            render_backfill(&json!({"computed": 1, "failed": [], "more": false})),
            "measured 1 merged task\n"
        );
        assert_eq!(
            render_backfill(&json!({"computed": 0, "more": true,
                "failed": [{"task": "t-1", "error": "repo gone"}]})),
            "measured 0 merged tasks; 1 failed; run again for more\n  t-1: repo gone\n"
        );
    }
}
