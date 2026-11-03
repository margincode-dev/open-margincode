//! `av disk` and `av gc`: what the daemon's workspaces, caches and store take on disk, what
//! sessions left outside any workspace, and reclaiming the workspaces of finished tasks.

use std::fmt::Write;

use serde_json::Value;

use crate::footprint::human;

/// Paths of one session's footprint shown; the rest are counted.
const TOP_PATHS: usize = 5;

fn s(v: &Value) -> &str {
    v.as_str().unwrap_or("")
}

fn bytes(v: &Value) -> String {
    human(v.as_u64().unwrap_or(0))
}

/// `45s`, `12m`, `3h`, `5d`.
pub fn age(secs: u64) -> String {
    match secs {
        s if s < 60 => format!("{s}s"),
        s if s < 3600 => format!("{}m", s / 60),
        s if s < 86_400 => format!("{}h", s / 3600),
        s => format!("{}d", s / 86_400),
    }
}

/// `GET /v1/disk` for people.
pub fn render_disk(v: &Value) -> String {
    let mut o = String::new();
    let retain = match v["retain_secs"].as_u64() {
        Some(r) => format!("finished tasks' workspaces removed after {}", age(r)),
        None => "finished tasks' workspaces are kept (AV_WS_RETAIN_SECS=never)".into(),
    };
    let _ = writeln!(o, "workspaces in {} ({retain})", s(&v["ws_root"]));
    let repos = v["repos"].as_array().cloned().unwrap_or_default();
    if repos.is_empty() {
        let _ = writeln!(o, "no repos");
    }
    for repo in &repos {
        let _ = writeln!(o, "\n{}", s(&repo["name"]));
        let workspaces = repo["workspaces"].as_array().cloned().unwrap_or_default();
        if workspaces.is_empty() {
            let _ = writeln!(o, "  no workspaces");
        } else {
            let width = workspaces
                .iter()
                .map(|w| s(&w["slug"]).len())
                .max()
                .unwrap_or(0)
                .max(4);
            let _ = writeln!(
                o,
                "  {:width$}  {:10} {:>8} {:>8}  {:>5}",
                "SLUG", "STATUS", "SIZE", "BUILD", "AGE"
            );
            for w in &workspaces {
                let status = w["status"].as_str().unwrap_or("-");
                let age = w["age_secs"]
                    .as_u64()
                    .map(age)
                    .unwrap_or_else(|| "-".into());
                let mark = if w["reclaimable"].as_bool() == Some(true) {
                    "  reclaimable"
                } else {
                    ""
                };
                let build = match w["build_bytes"].as_u64() {
                    Some(0) | None => "-".to_owned(),
                    Some(b) => human(b),
                };
                let line = format!(
                    "  {:width$}  {status:10} {:>8} {build:>8}  {age:>5}{mark}",
                    s(&w["slug"]),
                    bytes(&w["bytes"])
                );
                let _ = writeln!(o, "{}", line.trim_end());
            }
        }
        let _ = writeln!(o, "  scratch      {}", bytes(&repo["scratch_bytes"]));
        let cache = &repo["build_cache"];
        let config = match (cache["enabled"].as_bool(), cache["config"].as_str()) {
            (Some(false), _) => "off (AV_BUILD_CACHE=off)".to_owned(),
            (_, Some(path)) => format!("cargo config {path}"),
            _ => "no av cargo config here".to_owned(),
        };
        let _ = writeln!(
            o,
            "  build cache  {} ({config})",
            bytes(&repo["build_cache_bytes"])
        );
        let _ = writeln!(o, "  store        {}", bytes(&repo["store_bytes"]));
    }
    let outside = v["outside"].as_array().cloned().unwrap_or_default();
    if !outside.is_empty() {
        let _ = writeln!(
            o,
            "\noutside any workspace (added by sessions; av never removes these)"
        );
        for f in &outside {
            let task = f["slug"]
                .as_str()
                .or_else(|| f["task"].as_str())
                .unwrap_or("-");
            let at = s(&f["at"]).replace('T', " ");
            let at: String = at.chars().take(16).collect();
            let partial = if f["partial"].as_bool() == Some(true) {
                " (partial scan)"
            } else {
                ""
            };
            let _ = writeln!(
                o,
                "  {}  {task}  {at}  {}{partial}",
                f["agent"].as_str().unwrap_or("-"),
                bytes(&f["total_bytes"])
            );
            let added = f["added"].as_array().cloned().unwrap_or_default();
            for a in added.iter().take(TOP_PATHS) {
                let _ = writeln!(o, "    {:>8}  {}", bytes(&a["bytes"]), s(&a["path"]));
            }
            if added.len() > TOP_PATHS {
                let _ = writeln!(o, "    and {} more", added.len() - TOP_PATHS);
            }
        }
    }
    if let Some(pg) = v["pg_dir"].as_str() {
        let _ = writeln!(o, "\npostgres     {pg}");
    }
    let t = &v["totals"];
    let _ = writeln!(
        o,
        "\ntotal: workspaces {} (build output {}), scratch {}, build cache {}, store {}",
        bytes(&t["workspaces_bytes"]),
        bytes(&t["build_bytes"]),
        bytes(&t["scratch_bytes"]),
        bytes(&t["build_cache_bytes"]),
        bytes(&t["store_bytes"])
    );
    o
}

/// `POST /v1/gc` for people.
pub fn render_gc(v: &Value) -> String {
    let mut o = String::new();
    let dry = v["dry_run"].as_bool() == Some(true);
    let verb = if dry { "would remove" } else { "removed" };
    let reclaimed = v["reclaimed"].as_array().cloned().unwrap_or_default();
    for g in &reclaimed {
        let build = match g["build_bytes"].as_u64() {
            Some(b) if b > 0 => format!(" ({} of it build output)", human(b)),
            _ => String::new(),
        };
        let _ = writeln!(
            o,
            "{verb} {}/{} ({})  {}{build}",
            s(&g["repo"]),
            s(&g["slug"]),
            s(&g["status"]),
            bytes(&g["bytes"])
        );
    }
    if let Some(cache) = v["build_cache_bytes"].as_u64() {
        let _ = writeln!(o, "{verb} build cache  {}", human(cache));
    }
    for e in v["errors"].as_array().into_iter().flatten() {
        let _ = writeln!(
            o,
            "failed {}/{}: {}",
            s(&e["repo"]),
            s(&e["slug"]),
            s(&e["error"])
        );
    }
    if reclaimed.is_empty() && v["build_cache_bytes"].is_null() {
        let _ = writeln!(o, "nothing to reclaim");
    }
    let freed = bytes(&v["freed_bytes"]);
    let _ = writeln!(o, "{} {freed}", if dry { "would free" } else { "freed" });
    let kept = v["kept"].as_array().map_or(0, Vec::len);
    if kept > 0 {
        let _ = writeln!(o, "kept {kept} (younger than retention)");
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn ages_read_short() {
        assert_eq!(age(45), "45s");
        assert_eq!(age(600), "10m");
        assert_eq!(age(7200), "2h");
        assert_eq!(age(3 * 86_400), "3d");
    }

    #[test]
    fn an_empty_gc_says_nothing_to_reclaim() {
        let out = render_gc(&json!({"reclaimed": [], "freed_bytes": 0, "kept": [],
                                    "dry_run": false}));
        assert!(out.contains("nothing to reclaim"), "{out}");
        assert!(out.contains("freed 0 B"), "{out}");
        assert!(!out.contains("kept"), "{out}");
    }

    #[test]
    fn a_disk_without_retention_or_build_cache_says_so() {
        let out = render_disk(
            &json!({"ws_root": "/w", "retain_secs": null, "pg_dir": null,
            "repos": [{"name": "demo", "workspaces": [],
                       "build_cache": {"enabled": false, "config": null}}],
            "outside": [], "totals": {}}),
        );
        assert!(out.contains("AV_WS_RETAIN_SECS=never"), "{out}");
        assert!(out.contains("AV_BUILD_CACHE=off"), "{out}");
        assert!(!out.contains("postgres"), "{out}");
    }
}
