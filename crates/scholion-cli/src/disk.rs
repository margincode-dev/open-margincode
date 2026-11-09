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
    if let Some(line) = free_space(v) {
        let _ = writeln!(o, "{line}");
    }
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
        if let Some(budget) = repo["build_budget_bytes"].as_u64().filter(|b| *b > 0) {
            let used = repo["build_bytes"].as_u64().unwrap_or(0)
                + repo["build_cache_bytes"].as_u64().unwrap_or(0);
            let over = if repo["over_budget"].as_bool() == Some(true) {
                "  OVER BUDGET"
            } else {
                ""
            };
            let _ = writeln!(
                o,
                "  build        {} of {} (AV_BUILD_CACHE_MAX_GB){over}",
                human(used),
                human(budget)
            );
        }
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
            for line in containers_lines(&f["containers"]) {
                let _ = writeln!(o, "    {line}");
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

/// `GET /v1/disk?fs=1` (`av disk --fs`) for people: the free-space line and, when a budget is
/// set, the build budget.
pub fn render_disk_fs(v: &Value) -> String {
    let mut o =
        free_space(v).unwrap_or_else(|| "disk: this daemon does not report free space".into());
    o.push('\n');
    if let Some(budget) = v["build_budget_bytes"].as_u64().filter(|b| *b > 0) {
        let _ = writeln!(
            o,
            "build budget {} per repo (AV_BUILD_CACHE_MAX_GB)",
            human(budget)
        );
    }
    o
}

/// A footprint's `containers` section (what carried the task's label when the session stopped,
/// and the containers it started without one), one line each for containers, images, volumes and
/// unlabelled containers; none when it has none.
fn containers_lines(c: &Value) -> Vec<String> {
    let list = |key: &str, show: &dyn Fn(&Value) -> String| -> Option<String> {
        let items: Vec<String> = c[key].as_array()?.iter().map(show).collect();
        (!items.is_empty()).then(|| format!("{key:11} {}", items.join(", ")))
    };
    let container = |v: &Value| match v["size"].as_str() {
        Some(size) if !size.is_empty() => format!("{} ({}, {size})", s(&v["name"]), s(&v["state"])),
        _ => format!("{} ({})", s(&v["name"]), s(&v["state"])),
    };
    let image = |v: &Value| {
        let name = v["tags"][0]
            .as_str()
            .map(str::to_owned)
            .unwrap_or_else(|| s(&v["id"]).chars().take(19).collect());
        match v["bytes"].as_u64() {
            Some(b) => format!("{name} ({})", human(b)),
            None => name,
        }
    };
    let volume = |v: &Value| s(&v["name"]).to_owned();
    [
        list("containers", &container),
        list("images", &image),
        list("volumes", &volume),
        list("unlabelled", &container),
    ]
    .into_iter()
    .flatten()
    .collect()
}

/// The filesystem under the workspaces and the free-space floor; `None` from a daemon that does
/// not report them.
fn free_space(v: &Value) -> Option<String> {
    let free = v.get("fs_free_bytes")?;
    let floor = v["min_free_bytes"].as_u64().unwrap_or(0);
    let mut line = match free.as_u64() {
        Some(free) => format!(
            "disk: {} free of {}",
            human(free),
            bytes(&v["fs_total_bytes"])
        ),
        None => "disk: free space not measured on this platform".to_owned(),
    };
    if floor == 0 {
        line.push_str(", no free-space floor (AV_DISK_MIN_FREE_GB=0)");
    } else {
        let _ = write!(line, ", floor {} (AV_DISK_MIN_FREE_GB)", human(floor));
    }
    if v["below_floor"].as_bool() == Some(true) {
        line.push_str(
            "\nBELOW THE FLOOR: no new task starts until there is room; tasks already started \
             keep working (submit, merge or abandon them, or free space)",
        );
    }
    Some(line)
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
    let budget = &v["build_budget"];
    for r in budget["removed"].as_array().into_iter().flatten() {
        let whose = match r["slug"].as_str() {
            Some(slug) => format!("{}/{slug} target/", s(&r["repo"])),
            None => format!("{} deps cache", s(&r["repo"])),
        };
        let why = match s(&r["what"]) {
            "finished" => format!("{} task", s(&r["status"])),
            "idle" => format!(
                "idle {}",
                age(r["idle_secs"].as_u64().unwrap_or(0).min(999 * 86_400))
            ),
            _ => "still over".to_owned(),
        };
        let _ = writeln!(
            o,
            "{verb} {whose}  {} (over the build budget: {why})",
            bytes(&r["bytes"])
        );
    }
    for r in budget["over"].as_array().into_iter().flatten() {
        let _ = writeln!(
            o,
            "{} is still over its build budget: {} of {} (only builds touched recently are left)",
            s(&r["repo"]),
            bytes(&r["bytes"]),
            bytes(&r["budget_bytes"])
        );
    }
    let budget_removed = budget["removed"].as_array().is_some_and(|r| !r.is_empty());
    for e in v["errors"].as_array().into_iter().flatten() {
        let _ = writeln!(
            o,
            "failed {}/{}: {}",
            s(&e["repo"]),
            s(&e["slug"]),
            s(&e["error"])
        );
    }
    if reclaimed.is_empty() && v["build_cache_bytes"].is_null() && !budget_removed {
        let _ = writeln!(o, "nothing to reclaim");
    }
    let freed = bytes(&v["freed_bytes"]);
    let _ = writeln!(o, "{} {freed}", if dry { "would free" } else { "freed" });
    let kept = v["kept"].as_array().map_or(0, Vec::len);
    if kept > 0 {
        let _ = writeln!(o, "kept {kept} (younger than retention)");
    }
    if v["containers"].is_object() {
        o.push_str(&render_containers_gc(&v["containers"]));
    }
    o
}

/// The `containers` part of `av gc --containers` (`containers::gc`) for people.
fn render_containers_gc(c: &Value) -> String {
    let mut o = String::new();
    let dry = c["dry_run"].as_bool() == Some(true);
    let verb = if dry { "would remove" } else { "removed" };
    let what = |e: &Value| -> String {
        let name = s(&e["name"]);
        match e["kind"].as_str() {
            Some("container") => match e["size"].as_str() {
                Some(size) if !size.is_empty() => {
                    format!("{name} ({}, {size})", s(&e["state"]))
                }
                _ => format!("{name} ({})", s(&e["state"])),
            },
            Some("image") => match e["bytes"].as_u64() {
                Some(b) => format!("{name} ({})", human(b)),
                None => name.to_owned(),
            },
            _ => name.to_owned(),
        }
    };
    let task = |e: &Value| format!("{}/{}", s(&e["repo"]), s(&e["slug"]));
    let removed = c["removed"].as_array().cloned().unwrap_or_default();
    for e in &removed {
        let _ = writeln!(o, "{verb} {} {}  {}", s(&e["kind"]), task(e), what(e));
    }
    for e in c["running"].as_array().into_iter().flatten() {
        let _ = writeln!(
            o,
            "running {}  {} (av never stops a container)",
            task(e),
            s(&e["name"])
        );
    }
    for e in c["errors"].as_array().into_iter().flatten() {
        let _ = writeln!(
            o,
            "failed {} {}  {}: {}",
            s(&e["kind"]),
            task(e),
            s(&e["name"]),
            s(&e["error"])
        );
    }
    if removed.is_empty() && c["running"].as_array().is_none_or(Vec::is_empty) {
        let _ = writeln!(o, "no containers of finished tasks");
    }
    let (open, unknown) = (
        c["left"]["open"].as_u64().unwrap_or(0),
        c["left"]["unknown"].as_u64().unwrap_or(0),
    );
    if open + unknown > 0 {
        let _ = writeln!(
            o,
            "left alone: {open} of open tasks, {unknown} of tasks the daemon does not know"
        );
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
    fn a_gc_with_containers_says_what_went_and_what_runs() {
        let gc = json!({"reclaimed": [], "freed_bytes": 0, "kept": [], "dry_run": true,
            "containers": {"dry_run": true,
                "removed": [{"kind": "container", "repo": "demo", "slug": "add-thing",
                             "name": "b-build", "state": "exited"},
                            {"kind": "image", "repo": "demo", "slug": "add-thing",
                             "name": "sha256:b", "bytes": 2048},
                            {"kind": "volume", "repo": "demo", "slug": "add-thing",
                             "name": "b-vol"}],
                "running": [{"repo": "demo", "slug": "add-thing", "name": "b-db"}],
                "errors": [{"kind": "volume", "repo": "demo", "slug": "x", "name": "v",
                            "error": "docker volume rm v failed"}],
                "left": {"open": 0, "unknown": 2}}});
        let out = render_gc(&gc);
        assert!(
            out.contains("would remove container demo/add-thing  b-build (exited)\n"),
            "{out}"
        );
        assert!(
            out.contains("would remove image demo/add-thing  sha256:b (2 KB)\n"),
            "{out}"
        );
        assert!(
            out.contains("would remove volume demo/add-thing  b-vol\n"),
            "{out}"
        );
        assert!(
            out.contains("running demo/add-thing  b-db (av never stops a container)\n"),
            "{out}"
        );
        assert!(
            out.contains("failed volume demo/x  v: docker volume rm v failed\n"),
            "{out}"
        );
        assert!(
            out.contains("left alone: 0 of open tasks, 2 of tasks the daemon does not know\n"),
            "{out}"
        );
        let out = render_gc(
            &json!({"reclaimed": [], "freed_bytes": 0, "kept": [], "dry_run": false,
            "containers": {"dry_run": false, "removed": [], "running": [], "errors": [],
                           "left": {"open": 0, "unknown": 0}}}),
        );
        assert!(out.contains("no containers of finished tasks\n"), "{out}");
        assert!(!out.contains("left alone"), "{out}");
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
        // a daemon that does not report the filesystem gets no disk line
        assert!(!out.contains("disk:"), "{out}");
    }

    #[test]
    fn a_footprints_containers_are_listed_under_its_session() {
        let out = render_disk(&json!({"ws_root": "/w", "retain_secs": 3600, "repos": [],
            "totals": {},
            "outside": [{"session": "cc-1", "agent": "claude", "task": null, "added": [],
                         "total_bytes": 0, "at": "2026-11-07T10:00:00Z",
                         "containers": {
                             "containers": [{"id": "abc", "name": "av-build", "state": "running",
                                             "size": "12MB (virtual 1.2GB)"},
                                            {"id": "def", "name": "av-db", "state": "exited"}],
                             "images": [{"id": "sha256:aaaaaaaaaaaaaaaaaaaaaaaa", "bytes": 1234567,
                                         "tags": ["demo/build:latest"]},
                                        {"id": "sha256:bbbbbbbbbbbbbbbbbbbbbbbb"}],
                             "volumes": [{"name": "av-pgdata"}],
                             "unlabelled": [{"id": "fff", "name": "stray-redis",
                                             "state": "running"}]}}]}));
        assert!(
            out.contains(
                "    containers  av-build (running, 12MB (virtual 1.2GB)), av-db (exited)
"
            ),
            "{out}"
        );
        assert!(
            out.contains(
                "    images      demo/build:latest (1 MB), sha256:bbbbbbbbbbbb
"
            ),
            "{out}"
        );
        assert!(
            out.contains(
                "    volumes     av-pgdata
"
            ),
            "{out}"
        );
        assert!(
            out.contains("    unlabelled  stray-redis (running)\n"),
            "{out}"
        );
        // a report without the section (an older av) prints none of these
        let out = render_disk(&json!({"ws_root": "/w", "repos": [], "totals": {},
            "outside": [{"session": "cc-2", "added": [], "total_bytes": 0, "at": "x"}]}));
        assert!(!out.contains("containers"), "{out}");
    }

    const GIB: u64 = 1 << 30;

    #[test]
    fn a_disk_with_a_build_budget_shows_the_build_output_against_it() {
        let mut v = json!({"ws_root": "/w", "retain_secs": 3600, "outside": [], "totals": {},
            "build_budget_bytes": 20 * GIB,
            "repos": [{"name": "demo", "workspaces": [],
                       "build_cache": {"enabled": true, "config": null},
                       "build_bytes": 11 * GIB, "build_cache_bytes": GIB + GIB / 5,
                       "build_budget_bytes": 20 * GIB, "over_budget": false}]});
        let out = render_disk(&v);
        assert!(
            out.contains("build        12.2 GB of 20.0 GB (AV_BUILD_CACHE_MAX_GB)\n"),
            "{out}"
        );
        v["repos"][0]["over_budget"] = json!(true);
        assert!(render_disk(&v).contains("OVER BUDGET"));
        v["repos"][0]["build_budget_bytes"] = json!(0);
        assert!(!render_disk(&v).contains("AV_BUILD_CACHE_MAX_GB"));
    }

    #[test]
    fn a_gc_says_what_the_build_budget_removed() {
        let out = render_gc(&json!({"reclaimed": [], "freed_bytes": 3 * GIB, "kept": [],
            "dry_run": false,
            "build_budget": {"budget_bytes": 4 * GIB, "freed_bytes": 3 * GIB, "removed": [
                {"repo": "demo", "what": "finished", "slug": "wave", "status": "merged",
                 "bytes": GIB},
                {"repo": "demo", "what": "idle", "slug": "shout", "status": "open",
                 "bytes": GIB, "idle_secs": 7200},
                {"repo": "demo", "what": "deps_cache", "slug": null, "status": null,
                 "bytes": GIB}],
              "over": [{"repo": "demo", "bytes": 5 * GIB, "budget_bytes": 4 * GIB}]}}));
        for want in [
            "removed demo/wave target/  1.0 GB (over the build budget: merged task)",
            "removed demo/shout target/  1.0 GB (over the build budget: idle 2h)",
            "removed demo deps cache  1.0 GB (over the build budget: still over)",
            "demo is still over its build budget: 5.0 GB of 4.0 GB",
        ] {
            assert!(out.contains(want), "missing {want:?} in\n{out}");
        }
        assert!(!out.contains("nothing to reclaim"), "{out}");
    }

    #[test]
    fn a_disk_shows_free_space_and_the_floor() {
        let mut v = json!({"ws_root": "/w", "retain_secs": 3600, "repos": [], "outside": [],
                           "totals": {}, "fs_free_bytes": 50 * GIB, "fs_total_bytes": 200 * GIB,
                           "min_free_bytes": 20 * GIB, "below_floor": false});
        let out = render_disk(&v);
        assert!(
            out.contains("disk: 50.0 GB free of 200.0 GB, floor 20.0 GB (AV_DISK_MIN_FREE_GB)"),
            "{out}"
        );
        assert!(!out.contains("BELOW"), "{out}");

        v["fs_free_bytes"] = json!(5 * GIB);
        v["below_floor"] = json!(true);
        let out = render_disk(&v);
        assert!(out.contains("disk: 5.0 GB free"), "{out}");
        assert!(out.contains("BELOW THE FLOOR: no new task starts"), "{out}");

        v["min_free_bytes"] = json!(0);
        v["below_floor"] = json!(false);
        let out = render_disk(&v);
        assert!(
            out.contains("no free-space floor (AV_DISK_MIN_FREE_GB=0)"),
            "{out}"
        );

        v["fs_free_bytes"] = Value::Null;
        v["fs_total_bytes"] = Value::Null;
        let out = render_disk(&v);
        assert!(out.contains("free space not measured"), "{out}");
    }

    #[test]
    fn disk_fs_prints_only_the_free_space_and_the_budget() {
        let mut v = json!({"fs_free_bytes": 50 * GIB, "fs_total_bytes": 200 * GIB,
                           "min_free_bytes": 20 * GIB, "below_floor": false,
                           "build_budget_bytes": 0});
        assert_eq!(
            render_disk_fs(&v),
            "disk: 50.0 GB free of 200.0 GB, floor 20.0 GB (AV_DISK_MIN_FREE_GB)\n"
        );
        v["build_budget_bytes"] = json!(30 * GIB);
        v["below_floor"] = json!(true);
        let out = render_disk_fs(&v);
        assert!(
            out.contains("\nBELOW THE FLOOR: no new task starts"),
            "{out}"
        );
        assert!(
            out.ends_with("\nbuild budget 30.0 GB per repo (AV_BUILD_CACHE_MAX_GB)\n"),
            "{out}"
        );
        assert_eq!(
            render_disk_fs(&json!({})),
            "disk: this daemon does not report free space\n"
        );
    }
}
