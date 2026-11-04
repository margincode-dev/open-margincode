//! Plan 11: the workspaces of merged and abandoned tasks are reclaimed (`POST /v1/gc`, the sweep,
//! or at once with `AV_WS_RETAIN_SECS=0`), restored by undo and revert, and `GET /v1/disk` shows
//! what is on disk.

use crate::common::*;

use std::time::Duration;

use scholion_daemon::{BoxError, Config, serve_with};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

async fn post(e: &Env, t: &Value, op: &str, body: Value) -> (StatusCode, Value) {
    call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/{op}", id(t)),
        Some(body),
    )
    .await
}

/// A task that edited the greeter, recorded, submitted, validated and was approved.
async fn approved(e: &Env, title: &str) -> Value {
    let t = start_task(e, title).await;
    edit_greeter(e, &t);
    record_decision(e, &t).await;
    let (status, body) = submit(e, &t).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = post(
        e,
        &t,
        "validate",
        json!({"agent": "claude", "command": "pytest", "exit_code": 0}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = post(
        e,
        &t,
        "review",
        json!({"reviewer": "reviewer", "verdict": "approve"}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    t
}

/// `approved`, then merged by `merger`; returns the task and the merge response.
async fn merged(e: &Env, title: &str) -> (Value, Value) {
    let t = approved(e, title).await;
    let (status, body) = post(e, &t, "merge", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["status"], "merged", "{body}");
    (t, body)
}

async fn gc(e: &Env, body: Value) -> Value {
    let (status, out) = call(&e.app, "POST", "/v1/gc", Some(body)).await;
    assert_eq!(status, StatusCode::OK, "{out}");
    out
}

fn slug(t: &Value) -> &str {
    t["task"]["slug"].as_str().unwrap()
}

fn listed<'a>(list: &'a Value, t: &Value) -> Option<&'a Value> {
    list.as_array()
        .unwrap()
        .iter()
        .find(|w| w["slug"] == t["task"]["slug"])
}

#[tokio::test(flavor = "multi_thread")]
async fn gc_removes_a_merged_tasks_workspace() {
    let e = env().await;
    let (t, merge) = merged(&e, "Wave hello").await;
    assert_eq!(merge["workspace"], "kept", "{merge}");
    assert!(ws_dir(&e, &t).exists());

    let out = gc(&e, json!({"retain_secs": 0})).await;
    let gone = listed(&out["reclaimed"], &t).unwrap_or_else(|| panic!("{out}"));
    assert_eq!(gone["repo"], "demo");
    assert_eq!(gone["status"], "merged");
    assert_eq!(gone["path"], format!("{HOST_WS}/demo/{}", slug(&t)));
    assert!(gone["bytes"].as_u64().unwrap() > 0, "{out}");
    assert!(out["freed_bytes"].as_u64().unwrap() >= gone["bytes"].as_u64().unwrap());
    assert_eq!(out["dry_run"], false);
    assert!(!ws_dir(&e, &t).exists());

    let (status, body) = call(&e.app, "GET", &format!("/v1/tasks/{}", id(&t)), None).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["task"]["status"], "merged");

    // the feed says so
    let (_, events) = call(
        &e.app,
        "GET",
        "/v1/events?repo=demo&since=0&limit=500",
        None,
    )
    .await;
    let ev = events["events"]
        .as_array()
        .unwrap()
        .iter()
        .find(|ev| ev["kind"] == "workspace:reclaimed")
        .unwrap_or_else(|| panic!("{events}"));
    assert_eq!(ev["payload"]["slug"], t["task"]["slug"]);

    // nothing left to do
    let again = gc(&e, json!({"retain_secs": 0})).await;
    assert!(listed(&again["reclaimed"], &t).is_none(), "{again}");
}

#[tokio::test(flavor = "multi_thread")]
async fn undo_after_gc_restores_the_workspace_with_the_edit() {
    let e = env().await;
    let (t, _) = merged(&e, "Wave hello").await;
    gc(&e, json!({"retain_secs": 0})).await;
    assert!(!ws_dir(&e, &t).exists());

    let (status, body) = post(&e, &t, "undo", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["status"], "in_review");
    assert_eq!(body["workspace"], "restored", "{body}");
    let greeter = std::fs::read_to_string(ws_dir(&e, &t).join("app/greeter.py")).unwrap();
    assert!(greeter.contains("wave"), "{greeter}");

    // the restored task merges again
    let (status, body) = post(&e, &t, "merge", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["status"], "merged", "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn revert_after_gc_restores_the_workspace() {
    let e = env().await;
    let (t, _) = merged(&e, "Wave hello").await;
    gc(&e, json!({"retain_secs": 0})).await;
    assert!(!ws_dir(&e, &t).exists());

    let (status, body) = post(&e, &t, "revert", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["status"], "open");
    assert_eq!(body["workspace"], "restored", "{body}");
    let greeter = std::fs::read_to_string(ws_dir(&e, &t).join("app/greeter.py")).unwrap();
    assert!(greeter.contains("wave"), "{greeter}");
    // the owner keeps working in it
    let (status, body) = post(&e, &t, "checkpoint", json!({"agent": "claude"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn gc_removes_an_abandoned_tasks_workspace() {
    let e = env().await;
    let t = start_task(&e, "Never mind").await;
    edit_greeter(&e, &t);
    let (status, body) = post(&e, &t, "abandon", json!({"agent": "claude"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["workspace"], "kept", "{body}");

    let out = gc(&e, json!({"retain_secs": 0})).await;
    let gone = listed(&out["reclaimed"], &t).unwrap_or_else(|| panic!("{out}"));
    assert_eq!(gone["status"], "abandoned");
    assert!(!ws_dir(&e, &t).exists());
}

#[tokio::test(flavor = "multi_thread")]
async fn open_and_in_review_tasks_are_never_reclaimed() {
    let e = env().await;
    let open = start_task(&e, "Still working").await;
    edit_greeter(&e, &open);
    let review = start_task(&e, "Waiting for review").await;
    edit_greeter(&e, &review);
    record_decision(&e, &review).await;
    let (status, body) = submit(&e, &review).await;
    assert_eq!(status, StatusCode::OK, "{body}");

    let out = gc(&e, json!({"retain_secs": 0})).await;
    assert_eq!(out["reclaimed"], json!([]), "{out}");
    assert_eq!(out["kept"], json!([]), "{out}");
    assert!(ws_dir(&e, &open).exists());
    assert!(ws_dir(&e, &review).exists());
}

#[tokio::test(flavor = "multi_thread")]
async fn a_workspace_younger_than_the_retention_is_kept() {
    let e = env().await;
    let (t, _) = merged(&e, "Wave hello").await;
    let out = gc(&e, json!({"retain_secs": 3600})).await;
    assert_eq!(out["reclaimed"], json!([]), "{out}");
    let kept = listed(&out["kept"], &t).unwrap_or_else(|| panic!("{out}"));
    assert_eq!(kept["status"], "merged");
    assert!(kept["bytes"].as_u64().unwrap() > 0, "{out}");
    assert!(kept["age_secs"].as_u64().unwrap() < 3600, "{out}");
    assert!(ws_dir(&e, &t).exists());

    // the default retention (3600 s) applies when the request names none
    let out = gc(&e, json!({})).await;
    assert_eq!(out["reclaimed"], json!([]), "{out}");
    assert!(ws_dir(&e, &t).exists());
}

#[tokio::test(flavor = "multi_thread")]
async fn dry_run_lists_but_removes_nothing() {
    let e = env().await;
    let (t, _) = merged(&e, "Wave hello").await;
    let out = gc(&e, json!({"retain_secs": 0, "dry_run": true})).await;
    assert_eq!(out["dry_run"], true);
    let gone = listed(&out["reclaimed"], &t).unwrap_or_else(|| panic!("{out}"));
    assert!(gone["bytes"].as_u64().unwrap() > 0, "{out}");
    assert!(ws_dir(&e, &t).exists());
    // jj still has it, so a real gc afterwards removes it
    let out = gc(&e, json!({"retain_secs": 0})).await;
    assert!(listed(&out["reclaimed"], &t).is_some(), "{out}");
    assert!(!ws_dir(&e, &t).exists());
}

#[tokio::test(flavor = "multi_thread")]
async fn gc_build_cache_removes_each_repos_cache() {
    let e = env().await;
    let cache = e.ws.path().join("demo/.cache/cargo-target/debug");
    std::fs::create_dir_all(&cache).unwrap();
    std::fs::write(cache.join("big"), vec![0u8; 4096]).unwrap();
    let out = gc(&e, json!({"build_cache": true, "dry_run": true})).await;
    assert_eq!(out["build_cache_bytes"], 4096, "{out}");
    assert!(cache.exists());
    let out = gc(&e, json!({"build_cache": true})).await;
    assert_eq!(out["build_cache_bytes"], 4096, "{out}");
    assert!(!e.ws.path().join("demo/.cache").exists());
}

#[tokio::test(flavor = "multi_thread")]
async fn disk_lists_workspaces_with_size_status_and_whether_reclaimable() {
    let e = env_with(|s| s.with_ws_retain(None)).await;
    let open = start_task(&e, "Still working").await;
    let (t, _) = merged(&e, "Wave hello").await;

    let (status, disk) = call(&e.app, "GET", "/v1/disk", None).await;
    assert_eq!(status, StatusCode::OK, "{disk}");
    assert_eq!(disk["ws_root"], HOST_WS);
    let repo = disk["repos"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["name"] == "demo")
        .unwrap_or_else(|| panic!("{disk}"));
    let w = listed(&repo["workspaces"], &t).unwrap_or_else(|| panic!("{disk}"));
    assert_eq!(w["status"], "merged");
    assert!(w["bytes"].as_u64().unwrap() > 0, "{disk}");
    assert!(w["age_secs"].is_u64(), "{disk}");
    assert_eq!(w["reclaimable"], false, "retain never: {disk}");
    let o = listed(&repo["workspaces"], &open).unwrap_or_else(|| panic!("{disk}"));
    assert_eq!(o["status"], "open");
    assert_eq!(o["reclaimable"], false);
    assert!(
        disk["totals"]["workspaces_bytes"].as_u64().unwrap() > 0,
        "{disk}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn disk_marks_a_merged_workspace_reclaimable_past_the_retention() {
    // 1 s, not 0: retain 0 removes the workspace with the merge itself
    let e = env_with(|s| s.with_ws_retain(Some(1))).await;
    let (t, merge) = merged(&e, "Wave hello").await;
    assert_eq!(merge["workspace"], "kept", "{merge}");
    tokio::time::sleep(Duration::from_millis(1100)).await;
    let (status, disk) = call(&e.app, "GET", "/v1/disk", None).await;
    assert_eq!(status, StatusCode::OK, "{disk}");
    let w = listed(&disk["repos"][0]["workspaces"], &t).unwrap_or_else(|| panic!("{disk}"));
    assert_eq!(w["status"], "merged");
    assert!(w["bytes"].as_u64().unwrap() > 0, "{disk}");
    assert!(w["age_secs"].as_u64().unwrap() >= 1, "{disk}");
    assert_eq!(w["reclaimable"], true, "{disk}");
    // the configured retention is gc's default
    let out = gc(&e, json!({})).await;
    assert!(listed(&out["reclaimed"], &t).is_some(), "{out}");
    let (_, disk) = call(&e.app, "GET", "/v1/disk", None).await;
    assert!(
        listed(&disk["repos"][0]["workspaces"], &t).is_none(),
        "{disk}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn retain_zero_removes_the_workspace_with_the_merge() {
    let e = env_with(|s| s.with_ws_retain(Some(0))).await;
    let (t, merge) = merged(&e, "Wave hello").await;
    assert_eq!(merge["build"], "removed", "{merge}");
    assert_eq!(merge["workspace"], "removed", "{merge}");
    assert!(!ws_dir(&e, &t).exists());
    // nothing left for gc
    let out = gc(&e, json!({})).await;
    assert_eq!(out["reclaimed"], json!([]), "{out}");
}

// ---- Plan 12: build output goes with the merge, the workspace waits for the retention ----

/// Puts 8 KiB of "build output" in the task's workspace.
fn build_in(e: &Env, t: &Value) -> PathBuf {
    let built = ws_dir(e, t).join("target/debug/deps");
    std::fs::create_dir_all(&built).unwrap();
    std::fs::write(built.join("wave-0123"), vec![0u8; 8192]).unwrap();
    ws_dir(e, t).join("target")
}

#[tokio::test(flavor = "multi_thread")]
async fn a_merge_removes_the_build_output_and_keeps_the_workspace() {
    let e = env().await; // retention 3600
    let t = approved(&e, "Wave hello").await;
    let target = build_in(&e, &t);
    let (status, merge) = post(&e, &t, "merge", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::OK, "{merge}");
    assert_eq!(merge["workspace"], "kept", "{merge}");
    assert_eq!(merge["build"], "removed", "{merge}");
    assert!(!target.exists());
    assert!(ws_dir(&e, &t).join("app/greeter.py").exists());
    // measured and removed in the background, after the answer
    let ev = feed_event(&e, "build:reclaimed").await;
    assert_eq!(ev["payload"]["bytes"], 8192, "{ev}");
    // the workspace still follows the retention
    let (_, disk) = call(&e.app, "GET", "/v1/disk", None).await;
    let w = listed(&disk["repos"][0]["workspaces"], &t).unwrap_or_else(|| panic!("{disk}"));
    assert_eq!(w["build_bytes"], 0, "{disk}");
    assert_eq!(w["reclaimable"], false, "{disk}");
    // an undo gets a workspace that simply rebuilds
    let (status, body) = post(&e, &t, "undo", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["workspace"], "kept", "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn an_abandon_removes_the_build_output_too_and_says_none_without_any() {
    let e = env().await;
    let t = start_task(&e, "Never mind").await;
    let target = build_in(&e, &t);
    let (status, body) = post(&e, &t, "abandon", json!({"agent": "claude"})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["workspace"], "kept", "{body}");
    assert_eq!(body["build"], "removed", "{body}");
    assert!(!target.exists());
    assert!(ws_dir(&e, &t).exists());

    let t = start_task(&e, "Nothing built").await;
    let (_, body) = post(&e, &t, "abandon", json!({"agent": "claude"})).await;
    assert_eq!(body["build"], "none", "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn retain_never_keeps_the_build_output() {
    let e = env_with(|s| s.with_ws_retain(None)).await;
    let t = approved(&e, "Wave hello").await;
    let target = build_in(&e, &t);
    let (status, merge) = post(&e, &t, "merge", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::OK, "{merge}");
    assert_eq!(merge["workspace"], "kept", "{merge}");
    assert_eq!(merge["build"], "kept", "{merge}");
    assert!(target.exists());
}

// ---- Task 3 (Plan 11), reworked by Plan 12: lean builds, one target dir per workspace ----

const CARGO_CONFIG: &str = "# written by av: lean builds for every task workspace of this repo.
# Delete this file, or set AV_BUILD_CACHE=off before `av up`, to opt out.
# Each workspace builds into its own target/ (removed with the workspace); never share a
# target dir between workspaces of one repo: cargo would run another task's test binary.
[build]
incremental = false
[profile.dev]
debug = \"line-tables-only\"
# Dependencies carry no debug info at all; the repo's own crates keep file:line.
[profile.dev.package.\"*\"]
debug = false
";

/// A git repo with a root `Cargo.toml`.
fn cargo_git_repo() -> TempDir {
    let d = TempDir::new().unwrap();
    git(d.path(), &["init", "-q", "-b", "main"]);
    std::fs::write(
        d.path().join("Cargo.toml"),
        "[package]
name = \"crab\"
version = \"0.1.0\"
edition = \"2021\"
",
    )
    .unwrap();
    std::fs::create_dir_all(d.path().join("src")).unwrap();
    std::fs::write(
        d.path().join("src/lib.rs"),
        "pub fn crab() {}
",
    )
    .unwrap();
    git(d.path(), &["add", "-A"]);
    git(d.path(), &["commit", "-q", "-m", "init"]);
    d
}

async fn import(e: &Env, name: &str, src: &Path) {
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/repos",
        Some(json!({"name": name, "git_source": src})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
}

fn cargo_config(e: &Env, repo: &str) -> PathBuf {
    e.ws.path().join(repo).join(".cargo/config.toml")
}

#[tokio::test(flavor = "multi_thread")]
async fn a_cargo_repo_gets_one_build_cache_for_its_workspaces() {
    let e = env().await;
    let src = cargo_git_repo();
    import(&e, "crab", src.path()).await;
    assert_eq!(
        std::fs::read_to_string(cargo_config(&e, "crab")).unwrap(),
        CARGO_CONFIG
    );
    // a repo without a root Cargo.toml gets none
    assert!(!cargo_config(&e, "demo").exists());
    assert!(!e.ws.path().join("demo/.cargo").exists());

    let (status, disk) = call(&e.app, "GET", "/v1/disk", None).await;
    assert_eq!(status, StatusCode::OK, "{disk}");
    let repo = |name: &str| {
        disk["repos"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["name"] == name)
            .unwrap_or_else(|| panic!("{disk}"))
            .clone()
    };
    assert_eq!(repo("crab")["build_cache"]["enabled"], true, "{disk}");
    assert_eq!(
        repo("crab")["build_cache"]["config"],
        format!("{HOST_WS}/crab/.cargo/config.toml"),
        "{disk}"
    );
    assert_eq!(repo("demo")["build_cache"]["config"], Value::Null, "{disk}");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_users_own_cargo_config_is_never_overwritten() {
    let e = env().await;
    let mine = "[build]
target-dir = \"/elsewhere\"
";
    std::fs::create_dir_all(e.ws.path().join("crab/.cargo")).unwrap();
    std::fs::write(cargo_config(&e, "crab"), mine).unwrap();
    let src = cargo_git_repo();
    import(&e, "crab", src.path()).await;
    assert_eq!(
        std::fs::read_to_string(cargo_config(&e, "crab")).unwrap(),
        mine
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn an_older_av_cargo_config_is_rewritten() {
    let e = env().await;
    std::fs::create_dir_all(e.ws.path().join("crab/.cargo")).unwrap();
    std::fs::write(
        cargo_config(&e, "crab"),
        "# written by av: one cargo target for every task workspace of this repo.
[build]
",
    )
    .unwrap();
    let src = cargo_git_repo();
    import(&e, "crab", src.path()).await;
    assert_eq!(
        std::fs::read_to_string(cargo_config(&e, "crab")).unwrap(),
        CARGO_CONFIG
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn build_cache_off_writes_nothing() {
    let e = env_with(|s| s.with_build_cache(false)).await;
    let src = cargo_git_repo();
    import(&e, "crab", src.path()).await;
    assert!(!cargo_config(&e, "crab").exists());
    let (_, disk) = call(&e.app, "GET", "/v1/disk", None).await;
    let crab = disk["repos"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["name"] == "crab")
        .unwrap_or_else(|| panic!("{disk}"));
    assert_eq!(crab["build_cache"]["enabled"], false, "{disk}");
}

#[tokio::test(flavor = "multi_thread")]
async fn start_up_gives_repos_imported_earlier_their_build_cache() {
    let slot: Arc<std::sync::Mutex<Option<AppState>>> = Arc::default();
    let keep = slot.clone();
    let e = env_with(move |s| {
        *keep.lock().unwrap() = Some(s.clone());
        s
    })
    .await;
    let src = cargo_git_repo();
    import(&e, "crab", src.path()).await;
    // as if imported by a daemon without the build cache
    std::fs::remove_dir_all(e.ws.path().join("crab/.cargo")).unwrap();
    let state = slot.lock().unwrap().clone().unwrap();
    scholion_daemon::sync_repos(&state).await.unwrap();
    assert_eq!(
        std::fs::read_to_string(cargo_config(&e, "crab")).unwrap(),
        CARGO_CONFIG
    );
}

/// Where cargo, run in `dir` with av's config in force (no `CARGO_TARGET_DIR` from this test's
/// own environment), would put its build.
fn cargo_target_dir(dir: &Path) -> PathBuf {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let out = std::process::Command::new(cargo)
        .args([
            "metadata",
            "--format-version",
            "1",
            "--no-deps",
            "--offline",
        ])
        .current_dir(dir)
        // The daemon cannot control an agent's environment; with these set, cargo would use them
        // in every workspace. `av validate`, the hooks and av-wrap.sh point them at the
        // workspace's own target/ (Plan 12 Task 3, `scholion-cli/src/target_dir.rs`).
        .env_remove("CARGO_TARGET_DIR")
        .env_remove("CARGO_BUILD_TARGET_DIR")
        .output()
        .expect("cargo runs");
    assert!(
        out.status.success(),
        "cargo metadata in {}: {}",
        dir.display(),
        String::from_utf8_lossy(&out.stderr)
    );
    let meta: Value = serde_json::from_slice(&out.stdout).unwrap();
    PathBuf::from(meta["target_directory"].as_str().unwrap())
}

#[tokio::test(flavor = "multi_thread")]
async fn two_workspaces_of_one_repo_never_share_a_target_dir() {
    let e = env().await;
    let src = cargo_git_repo();
    import(&e, "crab", src.path()).await;
    let mut dirs = Vec::new();
    for title in ["Pinch harder", "Walk sideways"] {
        let (status, t) = call(
            &e.app,
            "POST",
            "/v1/tasks",
            Some(json!({"repo": "crab", "title": title, "goal": title,
                        "agent": "claude", "session": "s-1"})),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{t}");
        let ws = e.ws.path().join("crab").join(slug(&t));
        let target = cargo_target_dir(&ws);
        // its own target/, inside the workspace, so it goes when the workspace is reclaimed
        assert!(
            target.ends_with(Path::new(slug(&t)).join("target")),
            "{}",
            target.display()
        );
        dirs.push(target);
    }
    assert_ne!(dirs[0], dirs[1]);
    assert!(!e.ws.path().join("crab/.cache").exists());
}

#[tokio::test(flavor = "multi_thread")]
async fn disk_and_gc_count_a_workspaces_build_output_apart() {
    let e = env().await;
    let (t, _) = merged(&e, "Wave hello").await;
    let built = ws_dir(&e, &t).join("target/debug/deps");
    std::fs::create_dir_all(&built).unwrap();
    std::fs::write(built.join("wave-0123"), vec![0u8; 8192]).unwrap();

    let (status, disk) = call(&e.app, "GET", "/v1/disk", None).await;
    assert_eq!(status, StatusCode::OK, "{disk}");
    let w = listed(&disk["repos"][0]["workspaces"], &t).unwrap_or_else(|| panic!("{disk}"));
    assert_eq!(w["build_bytes"], 8192, "{disk}");
    assert!(w["bytes"].as_u64().unwrap() > 8192, "{disk}");
    assert_eq!(disk["totals"]["build_bytes"], 8192, "{disk}");

    let out = gc(&e, json!({"retain_secs": 0})).await;
    let gone = listed(&out["reclaimed"], &t).unwrap_or_else(|| panic!("{out}"));
    assert_eq!(gone["build_bytes"], 8192, "{out}");
    assert!(gone["bytes"].as_u64().unwrap() > 8192, "{out}");
    assert!(!ws_dir(&e, &t).exists());
}

// ---- Task 4: what sessions added outside any workspace ----

#[tokio::test(flavor = "multi_thread")]
async fn a_reported_footprint_shows_under_outside_newest_first() {
    let e = env().await;
    let mb = 1024 * 1024;
    let report = |session: &str, path: &str, bytes: u64| {
        json!({"session": session, "agent": "claude", "task": null,
               "added": [{"path": path, "bytes": bytes}], "partial": false})
    };
    for body in [
        report("cc-1", "/home/u/junk/target", 3 * mb),
        report("cc-2", "/tmp/x/node_modules", 2 * mb),
        // a later stop of the same session replaces its report
        report("cc-1", "/home/u/junk/target", 5 * mb),
    ] {
        let (status, out) = call(&e.app, "POST", "/v1/footprint", Some(body)).await;
        assert_eq!(status, StatusCode::OK, "{out}");
    }
    let (status, disk) = call(&e.app, "GET", "/v1/disk", None).await;
    assert_eq!(status, StatusCode::OK, "{disk}");
    let outside = disk["outside"].as_array().unwrap();
    assert_eq!(outside.len(), 2, "{disk}");
    assert_eq!(outside[0]["session"], "cc-1");
    assert_eq!(outside[0]["total_bytes"], 5 * mb);
    assert_eq!(outside[0]["agent"], "claude");
    assert!(outside[0]["at"].is_string(), "{disk}");
    assert_eq!(outside[1]["session"], "cc-2");
    assert_eq!(outside[1]["added"][0]["path"], "/tmp/x/node_modules");

    let (status, _) = call(
        &e.app,
        "POST",
        "/v1/footprint",
        Some(json!({"session": "", "added": []})),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test(flavor = "multi_thread")]
async fn a_footprint_names_its_tasks_slug() {
    let e = env().await;
    let t = start_task(&e, "Make it loud").await;
    let body = json!({"session": "cc-3", "agent": "claude", "task": t["task"]["id"],
                      "added": [{"path": "/home/u/junk/target", "bytes": 4096}]});
    let (status, out) = call(&e.app, "POST", "/v1/footprint", Some(body)).await;
    assert_eq!(status, StatusCode::OK, "{out}");
    // a task the daemon does not know keeps no slug
    let body = json!({"session": "cc-4", "task": "not-a-task", "added": []});
    let (status, out) = call(&e.app, "POST", "/v1/footprint", Some(body)).await;
    assert_eq!(status, StatusCode::OK, "{out}");
    let (_, disk) = call(&e.app, "GET", "/v1/disk", None).await;
    assert_eq!(disk["outside"][1]["slug"], "make-it-loud", "{disk}");
    assert!(disk["outside"][0]["slug"].is_null(), "{disk}");
}

// ---- the sweep, through the daemon's own start-up (`serve_with`) ----

fn config(db: &TestDb, dir: &Path, retain: Option<u64>) -> Config {
    Config {
        database_url: Some(db.url.clone()),
        pg_dir: dir.join("pg"),
        pg_install_dir: None,
        store_root: dir.join("store"),
        ws_root: dir.join("ws"),
        ws_host_root: dir.join("ws"),
        listen: "127.0.0.1:0".into(),
        embed_api_key: None,
        embed: None,
        qdrant_url: None,
        expand_cap: None,
        pg_max_connections: 4,
        max_concurrent_starts: None,
        lease_secs: 1800,
        build_cache: true,
        ws_retain_secs: retain,
        backup_remotes: Vec::new(),
        token: None,
    }
}

/// One HTTP/1.1 request on a raw socket: (status code, JSON body).
async fn raw(addr: std::net::SocketAddr, method: &str, path: &str, body: Value) -> (u16, Value) {
    let mut s = tokio::net::TcpStream::connect(addr).await.unwrap();
    let body = body.to_string();
    let req = format!(
        "{method} {path} HTTP/1.1\r\nhost: x\r\ncontent-type: application/json\r\n\
         content-length: {}\r\nconnection: close\r\n\r\n{body}",
        body.len()
    );
    s.write_all(req.as_bytes()).await.unwrap();
    let mut buf = String::new();
    s.read_to_string(&mut buf).await.unwrap();
    let code = buf[9..12].parse().unwrap();
    let text = buf.split("\r\n\r\n").nth(1).unwrap_or_default();
    (code, serde_json::from_str(text).unwrap_or(Value::Null))
}

async fn ok(addr: std::net::SocketAddr, path: &str, body: Value) -> Value {
    let (code, out) = raw(addr, "POST", path, body).await;
    assert_eq!(code, 200, "{path}: {out}");
    out
}

#[tokio::test(flavor = "multi_thread")]
async fn the_sweep_removes_a_merged_workspace_without_a_gc_call() {
    let db = TestDb::new().await;
    let dir = TempDir::new().unwrap();
    let src = fixture_git_repo();
    // 1 s, not 0: retain 0 would remove it inline with the merge and prove nothing of the sweep
    let cfg = config(&db, dir.path(), Some(1));
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let ws = dir.path().join("ws");
    let (stop, stopped) = tokio::sync::oneshot::channel::<()>();
    let server = serve_with(
        cfg,
        listener,
        |state: AppState| async move {
            Ok::<_, BoxError>((
                state.with_reclaim_tick(Duration::from_millis(200)),
                Router::new(),
            ))
        },
        async move {
            let _ = stopped.await;
        },
    );
    let client = async move {
        for _ in 0..200 {
            if tokio::net::TcpStream::connect(addr).await.is_ok() {
                break;
            }
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
        ok(
            addr,
            "/v1/repos",
            json!({"name": "demo", "git_source": src.path()}),
        )
        .await;
        let t = ok(
            addr,
            "/v1/tasks",
            json!({"repo": "demo", "title": "Wave hello", "goal": "Make hello louder",
                   "agent": "claude", "session": "s-1"}),
        )
        .await;
        let tid = t["task"]["id"].as_str().unwrap().to_owned();
        let wdir = ws.join("demo").join(t["task"]["slug"].as_str().unwrap());
        std::fs::write(wdir.join("app/greeter.py"), "def wave():\n    return 1\n").unwrap();
        ok(
            addr,
            &format!("/v1/tasks/{tid}/records"),
            json!({"kind": "decision", "summary": "wave", "agent": "claude"}),
        )
        .await;
        ok(
            addr,
            &format!("/v1/tasks/{tid}/submit"),
            json!({"agent": "claude"}),
        )
        .await;
        ok(
            addr,
            &format!("/v1/tasks/{tid}/validate"),
            json!({"agent": "claude", "command": "pytest", "exit_code": 0}),
        )
        .await;
        ok(
            addr,
            &format!("/v1/tasks/{tid}/review"),
            json!({"reviewer": "reviewer", "verdict": "approve"}),
        )
        .await;
        let merge = ok(
            addr,
            &format!("/v1/tasks/{tid}/merge"),
            json!({"agent": "merger"}),
        )
        .await;
        assert_eq!(merge["workspace"], "kept", "{merge}");
        assert!(wdir.exists());
        let mut gone = false;
        for _ in 0..40 {
            if !wdir.exists() {
                gone = true;
                break;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        stop.send(()).unwrap();
        assert!(gone, "the sweep removed {wdir:?} within 4 s");
    };
    let (served, ()) = tokio::time::timeout(Duration::from_secs(60), async {
        tokio::join!(server, client)
    })
    .await
    .expect("daemon stops on the shutdown future");
    served.unwrap();
}

// ---- Plan 12 Task 5: dependencies are built once per repo ----

const SERDE_LIB: &str = "a266d54d01e4376a";
const SERDE_SCRIPT: &str = "1f0e2d3c4b5a6978";
const SERDE_RUN: &str = "77a0bc3e1d2f4a55";
const OWN_LIB: &str = "0b1c2d3e4f5a6b7c";
const OWN_TEST: &str = "9a8b7c6d5e4f3a2b";
const OWN_SCRIPT: &str = "5e6f7a8b9c0d1e2f";
const OWN_RUN: &str = "3c4d5e6f7a8b9c0d";

/// A cargo-like `target/debug` in the task's workspace, laid out as cargo 1.97 writes it: a
/// registry dependency (`serde`: its lib, its build script and that script's run) and the repo's
/// own crate (`demo-core`: lib, test binary, build script and its run), each with its dep-info.
fn cargo_build_in(e: &Env, t: &Value) -> PathBuf {
    let ws = ws_dir(e, t);
    let d = ws.join("target/debug");
    let td = d.display().to_string();
    let reg = "/usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.229";
    let own = ws.join("crates/core").display().to_string();
    // as rustc does: a new file renamed over the old one, never a write into a seeded link
    let w = |rel: &str, bytes: &[u8]| {
        let p = d.join(rel);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        let tmp = p.with_extension("tmp-build");
        std::fs::write(&tmp, bytes).unwrap();
        std::fs::rename(&tmp, &p).unwrap();
    };
    let fp = |unit: &str, kinds: &[&str]| {
        for k in kinds {
            w(&format!(".fingerprint/{unit}/{k}"), b"0123456789abcdef");
            w(&format!(".fingerprint/{unit}/{k}.json"), b"{}");
        }
        w(&format!(".fingerprint/{unit}/invoked.timestamp"), b"");
    };
    // serde, from the registry
    w(&format!("deps/libserde-{SERDE_LIB}.rlib"), &[1; 4096]);
    w(&format!("deps/libserde-{SERDE_LIB}.rmeta"), &[2; 1024]);
    let out = format!("{td}/build/serde-{SERDE_RUN}/out");
    let lib_d = format!(
        "{td}/deps/libserde-{SERDE_LIB}.rlib {td}/deps/libserde-{SERDE_LIB}.rmeta: \
         {reg}/src/lib.rs {out}/private.rs\n\n{reg}/src/lib.rs:\n{out}/private.rs:\n\n\
         # env-dep:OUT_DIR={out}\n"
    );
    w(&format!("deps/serde-{SERDE_LIB}.d"), lib_d.as_bytes());
    fp(
        &format!("serde-{SERDE_LIB}"),
        &["lib-serde", "dep-lib-serde"],
    );
    let script_d = format!(
        "{td}/build/serde-{SERDE_SCRIPT}/build_script_build-{SERDE_SCRIPT}: {reg}/build.rs\n\n\
         {reg}/build.rs:\n"
    );
    w(
        &format!("build/serde-{SERDE_SCRIPT}/build_script_build-{SERDE_SCRIPT}.d"),
        script_d.as_bytes(),
    );
    w(
        &format!("build/serde-{SERDE_SCRIPT}/build-script-build"),
        &[3; 512],
    );
    fp(
        &format!("serde-{SERDE_SCRIPT}"),
        &["build-script-build-script-build"],
    );
    w(
        &format!("build/serde-{SERDE_RUN}/output"),
        b"cargo:rustc-cfg=std\n",
    );
    w(
        &format!("build/serde-{SERDE_RUN}/root-output"),
        out.as_bytes(),
    );
    w(
        &format!("build/serde-{SERDE_RUN}/out/private.rs"),
        &[4; 256],
    );
    fp(
        &format!("serde-{SERDE_RUN}"),
        &["run-build-script-build-script-build"],
    );
    // the repo's own crate
    w(&format!("deps/libdemo_core-{OWN_LIB}.rlib"), &[5; 2048]);
    let own_d =
        format!("{td}/deps/libdemo_core-{OWN_LIB}.rlib: {own}/src/lib.rs\n\n{own}/src/lib.rs:\n");
    w(&format!("deps/demo_core-{OWN_LIB}.d"), own_d.as_bytes());
    fp(&format!("demo-core-{OWN_LIB}"), &["lib-demo_core"]);
    w(&format!("deps/demo_core-{OWN_TEST}"), &[6; 8192]);
    let test_d = format!("{td}/deps/demo_core-{OWN_TEST}: {own}/src/lib.rs\n\n{own}/src/lib.rs:\n");
    w(&format!("deps/demo_core-{OWN_TEST}.d"), test_d.as_bytes());
    fp(&format!("demo-core-{OWN_TEST}"), &["test-lib-demo_core"]);
    let own_script_d = format!(
        "{td}/build/demo-core-{OWN_SCRIPT}/build_script_build-{OWN_SCRIPT}: {own}/build.rs\n\n\
         {own}/build.rs:\n"
    );
    w(
        &format!("build/demo-core-{OWN_SCRIPT}/build_script_build-{OWN_SCRIPT}.d"),
        own_script_d.as_bytes(),
    );
    w(
        &format!("build/demo-core-{OWN_SCRIPT}/build-script-build"),
        &[7; 512],
    );
    fp(
        &format!("demo-core-{OWN_SCRIPT}"),
        &["build-script-build-script-build"],
    );
    w(
        &format!("build/demo-core-{OWN_RUN}/output"),
        b"cargo:rerun-if-changed=build.rs\n",
    );
    fp(
        &format!("demo-core-{OWN_RUN}"),
        &["run-build-script-build-script-build"],
    );
    ws.join("target")
}

fn deps_cache(e: &Env) -> PathBuf {
    e.ws.path().join("demo/.cache/deps-target/debug")
}

/// What a dependency leaves in the cache.
fn shared_entries() -> Vec<String> {
    vec![
        format!("deps/libserde-{SERDE_LIB}.rlib"),
        format!("deps/libserde-{SERDE_LIB}.rmeta"),
        format!("deps/serde-{SERDE_LIB}.d"),
        format!(".fingerprint/serde-{SERDE_LIB}/lib-serde"),
        format!(".fingerprint/serde-{SERDE_SCRIPT}/build-script-build-script-build"),
        format!(".fingerprint/serde-{SERDE_RUN}/run-build-script-build-script-build"),
        format!("build/serde-{SERDE_SCRIPT}/build-script-build"),
        format!("build/serde-{SERDE_RUN}/root-output"),
        format!("build/serde-{SERDE_RUN}/out/private.rs"),
    ]
}

/// What the repo's own crates leave, which never enters the cache: their fingerprints hash
/// their path relative to the workspace root, so another workspace would take them as fresh.
fn own_entries() -> Vec<String> {
    vec![
        format!("deps/libdemo_core-{OWN_LIB}.rlib"),
        format!("deps/demo_core-{OWN_LIB}.d"),
        format!("deps/demo_core-{OWN_TEST}"),
        format!("deps/demo_core-{OWN_TEST}.d"),
        format!(".fingerprint/demo-core-{OWN_LIB}"),
        format!(".fingerprint/demo-core-{OWN_TEST}"),
        format!(".fingerprint/demo-core-{OWN_SCRIPT}"),
        format!(".fingerprint/demo-core-{OWN_RUN}"),
        format!("build/demo-core-{OWN_SCRIPT}"),
        format!("build/demo-core-{OWN_RUN}"),
    ]
}

/// The first `kind` event of repo `demo`, waiting up to 20 s for it: moving build output out and
/// seeding it in run in the background (on a slow mount they take a minute), after the answer.
async fn feed_event(e: &Env, kind: &str) -> Value {
    let deadline = std::time::Instant::now() + Duration::from_secs(20);
    loop {
        let (_, events) = call(
            &e.app,
            "GET",
            "/v1/events?repo=demo&since=0&limit=500",
            None,
        )
        .await;
        let found = events["events"]
            .as_array()
            .unwrap()
            .iter()
            .find(|ev| ev["kind"] == kind)
            .cloned();
        if let Some(ev) = found {
            return ev;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "no {kind} in {events}"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn a_merge_keeps_the_dependencies_in_the_repos_deps_cache_and_never_the_repos_own_crates() {
    let e = env().await;
    let t = approved(&e, "Wave hello").await;
    let target = cargo_build_in(&e, &t);
    let (status, merge) = post(&e, &t, "merge", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::OK, "{merge}");
    assert_eq!(merge["build"], "removed", "{merge}");
    assert!(!target.exists());
    // kept and removed in the background, after the answer
    let ev = feed_event(&e, "build:reclaimed").await;
    let cache = deps_cache(&e);
    for entry in shared_entries() {
        assert!(
            cache.join(&entry).exists(),
            "{entry} should be in the deps cache"
        );
    }
    for entry in own_entries() {
        assert!(
            !cache.join(&entry).exists(),
            "{entry} must stay out of the deps cache"
        );
    }
    // nothing is left on its way out
    let aside = e.ws.path().join("demo/.cache/reclaim");
    assert_eq!(std::fs::read_dir(&aside).map(|d| d.count()).unwrap_or(0), 0);
    assert!(
        ev["payload"]["cached_bytes"].as_u64().unwrap() >= 4096 + 1024 + 512 + 256,
        "{ev}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn a_new_task_starts_with_the_dependencies_linked_and_their_mutable_files_copied() {
    let e = env().await;
    let t = approved(&e, "Wave hello").await;
    cargo_build_in(&e, &t);
    let (status, merge) = post(&e, &t, "merge", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::OK, "{merge}");
    feed_event(&e, "build:reclaimed").await;
    let t2 = start_task(&e, "Wave goodbye").await;
    // built aside and moved in whole, after start_task answered
    let ev = feed_event(&e, "build:seeded").await;
    assert_eq!(ev["payload"]["slug"], slug(&t2), "{ev}");
    assert!(ev["payload"]["linked"].as_u64().unwrap() >= 2, "{ev}");
    let seeded = ws_dir(&e, &t2).join("target/debug");
    for entry in shared_entries() {
        assert!(seeded.join(&entry).exists(), "{entry} should be seeded");
    }
    for entry in own_entries() {
        assert!(!seeded.join(&entry).exists(), "{entry} must not be seeded");
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let cache = deps_cache(&e);
        let same = |rel: &str| {
            std::fs::metadata(seeded.join(rel)).unwrap().ino()
                == std::fs::metadata(cache.join(rel)).unwrap().ino()
        };
        // rustc replaces these (a new file renamed over), so one copy on disk serves every task
        assert!(same(&format!("deps/libserde-{SERDE_LIB}.rlib")));
        assert!(same(&format!("deps/libserde-{SERDE_LIB}.rmeta")));
        // rustc rewrites dep-info and cargo its fingerprints in place, and a build script may
        // write into its out dir: each task gets its own copy, or one task's rebuild would
        // change the cache under every other
        assert!(!same(&format!("deps/serde-{SERDE_LIB}.d")));
        assert!(!same(&format!(".fingerprint/serde-{SERDE_LIB}/lib-serde")));
        assert!(!same(&format!("build/serde-{SERDE_RUN}/out/private.rs")));
        // GET /v1/disk counts the linked bytes once, under the repo's build cache
        let (_, disk) = call(&e.app, "GET", "/v1/disk", None).await;
        let w = listed(&disk["repos"][0]["workspaces"], &t2).unwrap_or_else(|| panic!("{disk}"));
        let apparent = walk_bytes(&ws_dir(&e, &t2).join("target"));
        assert_eq!(
            w["build_bytes"].as_u64().unwrap() + 4096 + 1024,
            apparent,
            "{disk}"
        );
        assert!(
            disk["repos"][0]["build_cache_bytes"].as_u64().unwrap() >= 4096 + 1024,
            "{disk}"
        );
    }
}

#[cfg(unix)]
fn walk_bytes(p: &Path) -> u64 {
    let meta = std::fs::symlink_metadata(p).unwrap();
    if !meta.is_dir() {
        return meta.len();
    }
    std::fs::read_dir(p)
        .unwrap()
        .flatten()
        .map(|e| walk_bytes(&e.path()))
        .sum()
}

#[tokio::test(flavor = "multi_thread")]
async fn an_entry_already_in_the_deps_cache_is_never_replaced() {
    let e = env().await;
    let cache = deps_cache(&e);
    let rlib = cache.join(format!("deps/libserde-{SERDE_LIB}.rlib"));
    let fingerprint = cache.join(format!(".fingerprint/serde-{SERDE_LIB}/lib-serde"));
    for p in [&rlib, &fingerprint] {
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, b"first").unwrap();
    }
    let t = approved(&e, "Wave hello").await;
    cargo_build_in(&e, &t);
    let (status, merge) = post(&e, &t, "merge", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::OK, "{merge}");
    feed_event(&e, "build:reclaimed").await;
    assert_eq!(std::fs::read(&rlib).unwrap(), b"first");
    assert_eq!(std::fs::read(&fingerprint).unwrap(), b"first");
    // and what was missing is still added
    assert!(
        cache
            .join(format!("deps/libserde-{SERDE_LIB}.rmeta"))
            .exists()
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn retain_zero_keeps_the_dependencies_too() {
    let e = env_with(|s| s.with_ws_retain(Some(0))).await;
    let t = approved(&e, "Wave hello").await;
    cargo_build_in(&e, &t);
    let (status, merge) = post(&e, &t, "merge", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::OK, "{merge}");
    assert_eq!(merge["workspace"], "removed", "{merge}");
    feed_event(&e, "build:reclaimed").await;
    assert!(
        deps_cache(&e)
            .join(format!("deps/libserde-{SERDE_LIB}.rlib"))
            .exists()
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn build_cache_off_neither_keeps_nor_links_dependencies() {
    let e = env_with(|s| s.with_build_cache(false)).await;
    let t = approved(&e, "Wave hello").await;
    cargo_build_in(&e, &t);
    let (status, merge) = post(&e, &t, "merge", json!({"agent": "merger"})).await;
    assert_eq!(status, StatusCode::OK, "{merge}");
    assert!(!e.ws.path().join("demo/.cache").exists());
    let rlib = deps_cache(&e).join(format!("deps/libserde-{SERDE_LIB}.rlib"));
    std::fs::create_dir_all(rlib.parent().unwrap()).unwrap();
    std::fs::write(&rlib, b"by hand").unwrap();
    let t2 = start_task(&e, "Wave goodbye").await;
    assert!(!ws_dir(&e, &t2).join("target").exists());
}

#[tokio::test(flavor = "multi_thread")]
async fn start_up_removes_build_output_a_stopped_daemon_left_on_its_way_in_or_out() {
    let slot: Arc<std::sync::Mutex<Option<AppState>>> = Arc::default();
    let keep = slot.clone();
    let e = env_with(move |s| {
        *keep.lock().unwrap() = Some(s.clone());
        s
    })
    .await;
    let t = start_task(&e, "Wave hello").await;
    let _ = t;
    let cache = e.ws.path().join("demo/.cache");
    for left in [
        "reclaim/wave-hello-1-0/debug/deps",
        "seed/wave-hello-2-1/debug/deps",
    ] {
        std::fs::create_dir_all(cache.join(left)).unwrap();
        std::fs::write(cache.join(left).join("libx-a266d54d01e4376a.rlib"), [0; 64]).unwrap();
    }
    let kept = cache.join("deps-target/debug/deps/libserde-a266d54d01e4376a.rlib");
    std::fs::create_dir_all(kept.parent().unwrap()).unwrap();
    std::fs::write(&kept, b"cached").unwrap();
    let state = slot.lock().unwrap().clone().unwrap();
    scholion_daemon::sync_repos(&state).await.unwrap();
    assert!(!cache.join("reclaim").exists());
    assert!(!cache.join("seed").exists());
    assert_eq!(std::fs::read(&kept).unwrap(), b"cached");
}
