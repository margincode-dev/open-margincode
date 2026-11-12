//! Cold start from git history (`history.rs`): an import with `mine_history`, and
//! `POST /v1/repos/{name}/history`.

use crate::common::*;
use margincode_core::IntentStore as _;

/// A commit in `dir` at `secs` past 2026-01-01 (committer and author time), so newest-first is
/// the order written here.
fn commit_at(dir: &Path, secs: u64, args: &[&str]) {
    let date = format!("{} +0000", 1_767_225_600 + secs);
    let out = std::process::Command::new("git")
        .current_dir(dir)
        .env("GIT_AUTHOR_DATE", &date)
        .env("GIT_COMMITTER_DATE", &date)
        .args([
            "-c",
            "user.name=t",
            "-c",
            "user.email=t@t",
            "-c",
            "commit.gpgsign=false",
        ])
        .args(args)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

fn write(dir: &Path, path: &str, text: &str) {
    let p = dir.join(path);
    std::fs::create_dir_all(p.parent().unwrap()).unwrap();
    std::fs::write(p, text).unwrap();
}

const TOML: &str = "Parse config files as TOML";
const CACHE: &str = "Cache parsed configs per process";
const DEFAULTS: &str = "Load defaults before the user file so user values win";

/// History, oldest first: a root commit, TOML (with a trailer), a typo fix, `wip`, a branch
/// commit (cache) merged with a merge commit after a main commit (defaults), and a 30-file sweep.
/// Worth mining: DEFAULTS (newest), CACHE, TOML.
fn history_repo() -> TempDir {
    let d = TempDir::new().unwrap();
    let p = d.path();
    git(p, &["init", "-q", "-b", "main"]);
    write(p, "app/config.py", "def load(path):\n    return {}\n");
    write(p, "app/cache.py", "CACHE = {}\n");
    write(p, "README.md", "# hist\n");
    git(p, &["add", "-A"]);
    commit_at(p, 0, &["commit", "-q", "-m", "init"]);
    write(
        p,
        "app/config.py",
        "import tomllib\n\ndef load(path):\n    return tomllib.load(open(path, 'rb'))\n",
    );
    git(p, &["add", "-A"]);
    commit_at(
        p,
        10,
        &[
            "commit",
            "-q",
            "-m",
            &format!(
                "{TOML}\n\nUsers already write TOML for their other tools, so one format.\n\nCo-Authored-By: Bot <bot@example.com>"
            ),
        ],
    );
    write(p, "README.md", "# hist, a demo\n");
    git(p, &["add", "-A"]);
    commit_at(p, 20, &["commit", "-q", "-m", "fix typo in README"]);
    write(p, "app/cache.py", "CACHE = {}  # wip\n");
    git(p, &["add", "-A"]);
    commit_at(p, 30, &["commit", "-q", "-m", "wip"]);
    git(p, &["checkout", "-q", "-b", "feature"]);
    write(
        p,
        "app/cache.py",
        "CACHE = {}\n\ndef cached(path, load):\n    return CACHE.setdefault(path, load(path))\n",
    );
    git(p, &["add", "-A"]);
    commit_at(
        p,
        40,
        &[
            "commit",
            "-q",
            "-m",
            &format!("{CACHE}\n\nParsing on every call cost 40 ms a request."),
        ],
    );
    git(p, &["checkout", "-q", "main"]);
    write(
        p,
        "app/config.py",
        "import tomllib\n\nDEFAULTS = {}\n\ndef load(path):\n    return {**DEFAULTS, **tomllib.load(open(path, 'rb'))}\n",
    );
    git(p, &["add", "-A"]);
    commit_at(p, 50, &["commit", "-q", "-m", DEFAULTS]);
    commit_at(
        p,
        60,
        &[
            "merge",
            "-q",
            "--no-ff",
            "feature",
            "-m",
            "Merge branch 'feature'",
        ],
    );
    for i in 0..30 {
        write(p, &format!("fixtures/case_{i}.toml"), "a = 1\n");
    }
    git(p, &["add", "-A"]);
    commit_at(
        p,
        70,
        &[
            "commit",
            "-q",
            "-m",
            "Add thirty parser fixtures for the config tests",
        ],
    );
    d
}

async fn import(e: &Env, name: &str, src: &Path, mine: usize) -> Value {
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/repos",
        Some(json!({"name": name, "git_source": src, "mine_history": mine})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body
}

async fn mined(e: &Env, repo: &str) -> Vec<margincode_core::Record> {
    let t = e.store.task_by_slug(repo, "git-history").await.unwrap();
    assert_eq!(t.owner_agent, "git");
    assert_eq!(t.status, margincode_core::TaskStatus::Merged, "never open");
    e.store.task_records(t.id).await.unwrap()
}

fn summaries(recs: &[margincode_core::Record]) -> Vec<&str> {
    let mut s: Vec<&str> = recs.iter().map(|r| r.summary.as_str()).collect();
    s.sort();
    s
}

#[tokio::test(flavor = "multi_thread")]
async fn import_mines_commits_worth_keeping_into_anchored_notes() {
    let e = env().await;
    let src = history_repo();
    let job = import(&e, "hist", src.path(), 50).await;
    assert_eq!(job["mined_records"], 3, "{job}");
    let recs = mined(&e, "hist").await;
    assert_eq!(summaries(&recs), vec![CACHE, DEFAULTS, TOML]);
    for r in &recs {
        assert_eq!(r.kind, margincode_core::RecordKind::Note);
        assert_eq!(r.author_agent, "git");
        let sha = r.change_id.as_ref().unwrap().0.clone();
        assert_eq!(
            git_out(src.path(), &["log", "-1", "--format=%s", &sha]),
            r.summary,
            "change_id is the git commit"
        );
        assert!(
            r.body
                .contains(&format!("Mined from git commit {}", &sha[..12]))
        );
    }
    let toml = recs.iter().find(|r| r.summary == TOML).unwrap();
    assert!(
        toml.body.starts_with("Users already write TOML"),
        "{}",
        toml.body
    );
    assert!(
        !toml.body.contains("Co-Authored-By"),
        "trailers dropped: {}",
        toml.body
    );
    let paths = |r: &margincode_core::Record| -> Vec<String> {
        r.anchors.iter().map(|a| a.path.clone()).collect()
    };
    assert_eq!(paths(toml), vec!["app/config.py"]);
    let cache = recs.iter().find(|r| r.summary == CACHE).unwrap();
    assert_eq!(
        paths(cache),
        vec!["app/cache.py"],
        "a merged branch's commit is mined too"
    );
    // the root, the typo fix, `wip`, the merge and the 30-file sweep are not
    for skipped in [
        "init",
        "fix typo",
        "wip",
        "Merge branch",
        "thirty parser fixtures",
    ] {
        assert!(
            recs.iter().all(|r| !r.summary.contains(skipped)),
            "{skipped}"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn import_without_mine_history_mines_nothing() {
    let e = env().await;
    let src = history_repo();
    let job = import(&e, "hist", src.path(), 0).await;
    assert!(job.get("mined_records").is_none(), "{job}");
    assert!(e.store.task_by_slug("hist", "git-history").await.is_err());
}

#[tokio::test(flavor = "multi_thread")]
async fn the_cap_holds_and_mining_again_only_adds_what_is_new() {
    let e = env().await;
    let src = history_repo();
    let job = import(&e, "hist", src.path(), 1).await;
    assert_eq!(job["mined_records"], 1);
    assert_eq!(
        summaries(&mined(&e, "hist").await),
        vec![DEFAULTS],
        "newest first"
    );

    let (status, r) = call(
        &e.app,
        "POST",
        "/v1/repos/hist/history",
        Some(json!({"limit": 50})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{r}");
    assert_eq!(r["records"], 2, "{r}");
    assert_eq!(r["already"], 1, "{r}");
    assert_eq!(r["task"], "hist/git-history");
    assert_eq!(
        summaries(&mined(&e, "hist").await),
        vec![CACHE, DEFAULTS, TOML]
    );

    // again: nothing new
    let (_, r) = call(
        &e.app,
        "POST",
        "/v1/repos/hist/history",
        Some(json!({"limit": 50})),
    )
    .await;
    assert_eq!(r["records"], 0, "{r}");
    assert_eq!(r["already"], 3, "{r}");
    // a second add of the same repo is refused and mines nothing either
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/repos",
        Some(json!({"name": "hist", "git_source": src.path(), "mine_history": 50})),
    )
    .await;
    assert_ne!(status, StatusCode::OK, "{body}");
    assert_eq!(mined(&e, "hist").await.len(), 3, "no duplicates");
    // the history task has no workspace: reclaiming and the disk listing pass it by
    let (status, gc) = call(&e.app, "POST", "/v1/gc", Some(json!({"retain_secs": 0}))).await;
    assert_eq!(status, StatusCode::OK, "{gc}");
    let (status, disk) = call(&e.app, "GET", "/v1/disk", None).await;
    assert_eq!(status, StatusCode::OK, "{disk}");
    assert!(
        !gc.to_string().contains("git-history") && !disk.to_string().contains("git-history"),
        "{gc}
{disk}"
    );
    // an unknown repo
    let (status, _) = call(
        &e.app,
        "POST",
        "/v1/repos/nope/history",
        Some(json!({"limit": 5})),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test(flavor = "multi_thread")]
async fn mined_records_reach_agents_below_their_own_and_why_names_the_commit() {
    let e = env().await;
    let src = history_repo();
    import(&e, "hist", src.path(), 50).await;
    let start = |title: &str, agent: &str| {
        json!({"repo": "hist", "title": title, "goal": "Support YAML configs too",
               "agent": agent, "session": format!("s-{title}")})
    };
    // an agent's note about config.py, from another task
    let (_, other) = call(&e.app, "POST", "/v1/tasks", Some(start("Earlier", "codex"))).await;
    let other_id = other["task"]["id"].as_str().unwrap().to_owned();
    let (status, _) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{other_id}/records"),
        Some(
            json!({"kind": "note", "summary": "load() must never raise on a missing file",
                    "agent": "codex", "anchors": [{"path": "app/config.py"}]}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let (_, t) = call(&e.app, "POST", "/v1/tasks", Some(start("Yaml", "claude"))).await;
    let id = t["task"]["id"].as_str().unwrap().to_owned();
    // what a hook shows an agent opening the file: the agent's note first, then the mined ones,
    // each saying where it comes from
    let (_, at) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{id}/records/at?path=app/config.py"),
        None,
    )
    .await;
    let md = at["markdown"].as_str().unwrap();
    let agent = md.find("must never raise").expect(md);
    let defaults = md.find(DEFAULTS).expect(md);
    let toml = md.find(TOML).expect(md);
    assert!(agent < defaults && agent < toml, "{md}");
    assert!(md.contains("note (git "), "{md}");
    // a pack after editing config.py ranks them the same way
    std::fs::write(
        e.ws.path().join("hist/yaml/app/config.py"),
        "import tomllib\n\nDEFAULTS = {}\n\ndef load(path):\n    return {}\n",
    )
    .unwrap();
    let (status, _) = call(&e.app, "POST", &format!("/v1/tasks/{id}/checkpoint"), None).await;
    assert_eq!(status, StatusCode::OK);
    let (_, pack) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{id}/context?budget=4000&session=fresh"),
        None,
    )
    .await;
    let md = pack["markdown"].as_str().unwrap();
    let agent = md.find("must never raise").expect(md);
    let mined_at = md.find(DEFAULTS).expect(md);
    assert!(agent < mined_at, "{md}");
    // why a line of imported history is that way: the commit's mined record
    let (status, why) = call(
        &e.app,
        "GET",
        "/v1/repos/hist/why?path=app/config.py&line=1",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{why}");
    assert!(why["task"].is_null(), "{why}");
    let recs = why["records"].as_array().unwrap();
    assert_eq!(recs.len(), 1, "{why}");
    assert_eq!(recs[0]["summary"], DEFAULTS);
    assert_eq!(recs[0]["by"], "git");
    assert_eq!(recs[0]["origin"], "git history");
}
