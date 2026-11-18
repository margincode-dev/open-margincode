//! Many agents at once against one daemon: the races a single-agent test never hits.
//! Run with `--nocapture` to see timings.

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use crate::common::*;
use async_trait::async_trait;
use futures::future::join_all;
use margincode_core::{ChangeId, IntentStore, Vcs, WorkspaceInfo};

const AGENTS: usize = 6;

async fn post(e: &Env, uri: &str, body: Value) -> (StatusCode, Value) {
    call(&e.app, "POST", uri, Some(body)).await
}

async fn start_as(e: &Env, title: &str, agent: &str) -> Value {
    let (status, body) = post(
        e,
        "/v1/tasks",
        json!({"repo": "demo", "title": title, "goal": "Extend the greeter",
               "agent": agent, "session": format!("sess-{agent}")}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body
}

async fn agent_run(e: &Env, n: usize) -> Value {
    let agent = format!("agent-{n}");
    let t = start_as(e, &format!("swarm {n}"), &agent).await;
    let ws = ws_dir(e, &t);
    std::fs::write(
        ws.join(format!("app/feature_{n}.py")),
        format!("def feature_{n}(x):\n    return x * {n}\n"),
    )
    .unwrap();
    std::fs::write(
        ws.join("app/util.py"),
        format!(
            "def format_name(n):\n    return n.title()\n\n\ndef helper_{n}():\n    return {n}\n"
        ),
    )
    .unwrap();
    let tid = id(&t);
    let (status, body) = post(
        e,
        &format!("/v1/tasks/{tid}/records"),
        json!({"kind": "decision", "summary": format!("feature {n} lives in its own module"),
               "agent": agent, "anchors": [{"path": format!("app/feature_{n}.py")}]}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = post(e, &format!("/v1/tasks/{tid}/checkpoint"), json!({})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{tid}/context?budget=1500&session=sess-{agent}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = post(
        e,
        &format!("/v1/tasks/{tid}/submit"),
        json!({"agent": agent}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["status"], "in_review", "{body}");
    t
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn swarm_of_agents_runs_start_to_submit_concurrently() {
    let e = env().await;
    let started = Instant::now();
    let tasks = join_all((0..AGENTS).map(|n| agent_run(&e, n))).await;
    eprintln!(
        "swarm: {AGENTS} agents start→submit in {:?}",
        started.elapsed()
    );
    for (n, t) in tasks.iter().enumerate() {
        let (_, got) = call(&e.app, "GET", &format!("/v1/tasks/{}", id(t)), None).await;
        assert_eq!(got["task"]["status"], "in_review", "{got}");
        let changed: Vec<&str> = got["changed"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(Value::as_str)
            .collect();
        assert!(
            changed.contains(&format!("app/feature_{n}.py").as_str()),
            "{got}"
        );
        // no other agent's file leaked into this workspace
        assert_eq!(changed.len(), 2, "{got}");
    }
    let (_, feed) = call(
        &e.app,
        "GET",
        "/v1/events?repo=demo&since=0&limit=500",
        None,
    )
    .await;
    let events = feed["events"].as_array().unwrap();
    let ids: Vec<i64> = events.iter().map(|ev| ev["id"].as_i64().unwrap()).collect();
    assert!(ids.windows(2).all(|w| w[0] < w[1]), "{ids:?}");
    for kind in ["started", "submit"] {
        let n = events.iter().filter(|ev| ev["kind"] == kind).count();
        assert_eq!(n, AGENTS, "{kind}: {feed}");
    }

    // A latecomer touching the shared file is warned about every submitted task.
    let late = start_as(&e, "latecomer", "agent-late").await;
    std::fs::write(
        ws_dir(&e, &late).join("app/util.py"),
        "def format_name(n):\n    return n.upper()\n",
    )
    .unwrap();
    let (status, _) = post(
        &e,
        &format!("/v1/tasks/{}/checkpoint", id(&late)),
        json!({}),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let (_, pack) = call(
        &e.app,
        "GET",
        &format!(
            "/v1/tasks/{}/context?budget=4000&session=sess-late",
            id(&late)
        ),
        None,
    )
    .await;
    let md = pack["markdown"].as_str().unwrap();
    for t in &tasks {
        let slug = t["task"]["slug"].as_str().unwrap();
        assert!(
            md.contains(&format!("live:{slug}")),
            "{slug} missing:\n{md}"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn live_work_lists_a_task_in_review_with_its_status() {
    let e = env().await;
    let a = start_as(&e, "first", "agent-a").await;
    edit_greeter(&e, &a);
    post(
        &e,
        &format!("/v1/tasks/{}/records", id(&a)),
        json!({"kind": "note", "summary": "greeter shouts", "agent": "agent-a"}),
    )
    .await;
    let (status, body) = submit(&e, &a).await;
    assert_eq!(status, StatusCode::OK, "{body}");

    let b = start_as(&e, "second", "agent-b").await;
    edit_greeter(&e, &b);
    post(&e, &format!("/v1/tasks/{}/checkpoint", id(&b)), json!({})).await;
    let (_, pack) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{}/context?budget=4000&session=sess-b", id(&b)),
        None,
    )
    .await;
    let md = pack["markdown"].as_str().unwrap();
    assert!(md.contains("live:first"), "{md}");
    assert!(md.contains("in review"), "{md}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_checkpoints_of_one_task_keep_the_newest_head() {
    let e = env().await;
    let t = start_task(&e, "busy").await;
    let ws = ws_dir(&e, &t);
    let tid = id(&t);
    const WRITERS: usize = 4;
    const ROUNDS: usize = 4;
    const N: usize = WRITERS * ROUNDS;
    join_all((0..WRITERS).map(|w| {
        let (ws, tid, e) = (ws.clone(), tid.clone(), &e);
        async move {
            for r in 0..ROUNDS {
                let n = w * ROUNDS + r;
                std::fs::write(
                    ws.join(format!("app/f{n}.py")),
                    format!("def f{n}():\n    return {n}\n"),
                )
                .unwrap();
                let (status, body) =
                    post(e, &format!("/v1/tasks/{tid}/checkpoint"), json!({})).await;
                assert_eq!(status, StatusCode::OK, "{body}");
            }
        }
    }))
    .await;
    // Every write happened before the last snapshot, so the stored head must hold all of them.
    let (_, got) = call(&e.app, "GET", &format!("/v1/tasks/{tid}"), None).await;
    let changed = got["changed"].as_array().unwrap();
    assert_eq!(changed.len(), N, "stale head stored: {got}");
    // and the index agrees with the head: every new function is a symbol of this task
    let (_, pack) = call(
        &e.app,
        "GET",
        &format!("/v1/tasks/{tid}/context?budget=8000"),
        None,
    )
    .await;
    let md = pack["markdown"].as_str().unwrap();
    for n in 0..N {
        assert!(md.contains(&format!("f{n}")), "f{n} not indexed:\n{md}");
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_expands_never_exceed_the_cap() {
    const CAP: u64 = 200;
    let e = env_with(|s| s.with_expand_cap(Some(CAP))).await;
    let t = start_task(&e, "Improve greeting").await;
    let (_, r) = post(
        &e,
        &format!("/v1/tasks/{}/records", id(&t)),
        json!({"kind": "note", "summary": "a note worth expanding", "agent": "claude",
               "body": "Some longer body text that costs a few tokens to expand each time."}),
    )
    .await;
    let rec = format!("rec:{}", &r["id"].as_str().unwrap().replace('-', "")[..8]);
    let url = format!("/v1/tasks/{}/items/{rec}?session=greedy", id(&t));
    let results = join_all((0..24).map(|_| call(&e.app, "GET", &url, None))).await;
    let ok: Vec<&Value> = results
        .iter()
        .filter(|(s, _)| *s == StatusCode::OK)
        .map(|(_, b)| b)
        .collect();
    let per = ok[0]["tokens"].as_u64().unwrap();
    let spent = e.store.expand_tokens("greedy").await.unwrap();
    assert!(spent <= CAP, "spent {spent} > cap {CAP} ({} ok)", ok.len());
    assert_eq!(
        ok.len() as u64,
        CAP / per,
        "spent {spent}, {per} per expand"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn context_without_a_session_is_never_deduplicated() {
    let e = env().await;
    let a = start_task(&e, "one").await;
    let b = start_task(&e, "two").await;
    let ctx = |t: &Value| format!("/v1/tasks/{}/context?budget=4000", id(t));
    let (_, first_a) = call(&e.app, "GET", &ctx(&a), None).await;
    let (_, again_a) = call(&e.app, "GET", &ctx(&a), None).await;
    let (_, first_b) = call(&e.app, "GET", &ctx(&b), None).await;
    let (_, fresh_b) = call(
        &e.app,
        "GET",
        &format!("{}&session=brand-new", ctx(&b)),
        None,
    )
    .await;
    assert_eq!(first_a["markdown"], again_a["markdown"]);
    // another agent without a session is not told it was "sent earlier" what task one received
    assert_eq!(first_b["markdown"], fresh_b["markdown"]);
}

/// Delays the next `changed_paths` call once armed: the checkpoint that makes it stalls between
/// its snapshot and storing the new head.
struct StallNextDiff {
    inner: Arc<dyn Vcs>,
    armed: Arc<AtomicBool>,
}

#[async_trait]
impl Vcs for StallNextDiff {
    async fn init_repo(&self, repo: &str, src: &Path) -> margincode_core::Result<ChangeId> {
        self.inner.init_repo(repo, src).await
    }
    async fn create_workspace(
        &self,
        repo: &str,
        slug: &str,
    ) -> margincode_core::Result<WorkspaceInfo> {
        self.inner.create_workspace(repo, slug).await
    }
    async fn snapshot(&self, repo: &str, slug: &str) -> margincode_core::Result<ChangeId> {
        self.inner.snapshot(repo, slug).await
    }
    async fn changed_paths(
        &self,
        repo: &str,
        base: &ChangeId,
        head: &ChangeId,
    ) -> margincode_core::Result<Vec<String>> {
        if self.armed.swap(false, Ordering::SeqCst) {
            tokio::time::sleep(Duration::from_millis(600)).await;
        }
        self.inner.changed_paths(repo, base, head).await
    }
    async fn list_files(
        &self,
        repo: &str,
        change: &ChangeId,
    ) -> margincode_core::Result<Vec<String>> {
        self.inner.list_files(repo, change).await
    }
    async fn read_file(
        &self,
        repo: &str,
        change: &ChangeId,
        path: &str,
    ) -> margincode_core::Result<Vec<u8>> {
        self.inner.read_file(repo, change, path).await
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn a_slow_checkpoint_never_overwrites_a_newer_head() {
    let armed = Arc::new(AtomicBool::new(false));
    let flag = armed.clone();
    let e = env_with_vcs(
        move |inner| Arc::new(StallNextDiff { inner, armed: flag }),
        |s| s,
    )
    .await;
    let t = start_task(&e, "racy").await;
    let (ws, tid) = (ws_dir(&e, &t), id(&t));
    std::fs::write(ws.join("app/a.py"), "def a():\n    return 1\n").unwrap();
    let cp = format!("/v1/tasks/{tid}/checkpoint");
    armed.store(true, Ordering::SeqCst);
    let slow = post(&e, &cp, json!({}));
    let fast = async {
        // the slow checkpoint has snapshotted {a} and is stalled; now add b and checkpoint again
        tokio::time::sleep(Duration::from_millis(150)).await;
        std::fs::write(ws.join("app/b.py"), "def b():\n    return 2\n").unwrap();
        post(&e, &cp, json!({})).await
    };
    let ((s1, b1), (s2, b2)) = tokio::join!(slow, fast);
    assert_eq!((s1, s2), (StatusCode::OK, StatusCode::OK), "{b1} {b2}");
    let (_, got) = call(&e.app, "GET", &format!("/v1/tasks/{tid}"), None).await;
    let mut changed: Vec<&str> = got["changed"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(Value::as_str)
        .collect();
    changed.sort();
    assert_eq!(
        changed,
        ["app/a.py", "app/b.py"],
        "stale head stored: {got}"
    );
}

/// Timing only (`--ignored --nocapture`): compare the printed numbers before and after a change.
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
#[ignore]
async fn bench_swarm_on_a_300_file_repo() {
    let e = env().await;
    let src = TempDir::new().unwrap();
    git(src.path(), &["init", "-q", "-b", "main"]);
    for d in 0..10 {
        let dir = src.path().join(format!("pkg{d}"));
        std::fs::create_dir_all(&dir).unwrap();
        for f in 0..30 {
            let body: String = (0..8)
                .map(|k| format!("def fn_{d}_{f}_{k}(x):\n    return helper_{d}(x) + {k}\n\n"))
                .collect();
            std::fs::write(dir.join(format!("mod{f}.py")), body).unwrap();
        }
    }
    git(src.path(), &["add", "-A"]);
    git(src.path(), &["commit", "-q", "-m", "init"]);

    let t0 = Instant::now();
    let (status, body) = post(
        &e,
        "/v1/repos",
        json!({"name": "big", "git_source": src.path()}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    eprintln!("bench: import 300 files: {:?}", t0.elapsed());

    let t1 = Instant::now();
    let tasks = join_all((0..AGENTS).map(|n| {
        let e = &e;
        async move {
            let (status, body) = post(
                e,
                "/v1/tasks",
                json!({"repo": "big", "title": format!("bench {n}"), "goal": "touch pkg0",
                       "agent": format!("a{n}"), "session": format!("bench-{n}")}),
            )
            .await;
            assert_eq!(status, StatusCode::OK, "{body}");
            body
        }
    }))
    .await;
    eprintln!("bench: {AGENTS} concurrent start_task: {:?}", t1.elapsed());

    let t2 = Instant::now();
    join_all(tasks.iter().enumerate().map(|(n, t)| {
        let e = &e;
        async move {
            let ws =
                e.ws.path()
                    .join("big")
                    .join(t["task"]["slug"].as_str().unwrap());
            for f in 0..3 {
                std::fs::write(
                    ws.join(format!("pkg0/mod{}.py", n * 3 + f)),
                    format!("def changed_{n}_{f}(x):\n    return x\n"),
                )
                .unwrap();
                let tid = id(t);
                let (status, body) =
                    post(e, &format!("/v1/tasks/{tid}/checkpoint"), json!({})).await;
                assert_eq!(status, StatusCode::OK, "{body}");
                let (status, body) = call(
                    &e.app,
                    "GET",
                    &format!("/v1/tasks/{tid}/context?budget=3000&session=bench-{n}"),
                    None,
                )
                .await;
                assert_eq!(status, StatusCode::OK, "{body}");
            }
        }
    }))
    .await;
    eprintln!(
        "bench: {AGENTS} agents x 3 (edit, checkpoint, context): {:?}",
        t2.elapsed()
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn racing_claims_have_one_winner_and_fence_the_old_owner() {
    let e = env().await;
    let t = start_task(&e, "Pass it on").await;
    edit_greeter(&e, &t);
    record_decision(&e, &t).await;
    assert_eq!(handoff(&e, &t, "claude").await.0, StatusCode::OK);
    let tid = id(&t);
    let claims = join_all((0..AGENTS).map(|n| {
        let e = &e;
        let t = &t;
        async move { claim(e, t, &format!("agent-{n}")).await }
    }));
    let records_uri = format!("/v1/tasks/{tid}/records");
    let stale = join_all((0..AGENTS).map(|_| {
        call_as(
            &e.app,
            "POST",
            &records_uri,
            Some(json!({"kind": "note", "summary": "late note", "agent": "claude"})),
            Some(1),
        )
    }));
    let (claims, stale) = tokio::join!(claims, stale);
    let winners: Vec<&Value> = claims
        .iter()
        .filter(|(s, _)| *s == StatusCode::OK)
        .map(|(_, b)| b)
        .collect();
    assert_eq!(winners.len(), 1, "{claims:?}");
    assert_eq!(winners[0]["generation"], 2);
    for (s, b) in claims.iter().filter(|(s, _)| *s != StatusCode::OK) {
        assert_eq!(*s, StatusCode::UNPROCESSABLE_ENTITY, "{b}");
    }
    // every old-generation write either landed before the claim or was refused after it
    let task = margincode_core::TaskId(tid.parse().unwrap());
    let claimed_at = e.store.get_task(task).await.unwrap().lease_expires.unwrap()
        - chrono::Duration::seconds(1800);
    let late = e
        .store
        .task_records(task)
        .await
        .unwrap()
        .into_iter()
        .filter(|r| r.summary == "late note" && r.created_at > claimed_at)
        .count();
    assert_eq!(late, 0, "a stale write landed after the claim");
    for (s, b) in &stale {
        assert!(
            *s == StatusCode::OK
                || b["error"]
                    .as_str()
                    .unwrap_or_default()
                    .contains("claim lost"),
            "{s}: {b}"
        );
    }
}

/// A `CodeIndex` whose `ensure_overlay` is slow and counts how many run at once (the start queue serializes them).
struct SlowOverlay {
    inner: Arc<dyn CodeIndex>,
    running: std::sync::atomic::AtomicUsize,
    most: std::sync::atomic::AtomicUsize,
    delay: Duration,
}

#[async_trait]
impl CodeIndex for SlowOverlay {
    async fn index_paths_at(
        &self,
        repo: &str,
        scope: &str,
        change: Option<&ChangeId>,
        embed: bool,
        files: Vec<(String, Vec<u8>)>,
    ) -> margincode_core::Result<margincode_index::IndexStats> {
        self.inner
            .index_paths_at(repo, scope, change, embed, files)
            .await
    }
    async fn scope_change(
        &self,
        repo: &str,
        scope: &str,
    ) -> margincode_core::Result<Option<ChangeId>> {
        self.inner.scope_change(repo, scope).await
    }
    async fn set_scope_change(
        &self,
        repo: &str,
        scope: &str,
        change: &ChangeId,
    ) -> margincode_core::Result<()> {
        self.inner.set_scope_change(repo, scope, change).await
    }
    async fn ensure_overlay(
        &self,
        repo: &str,
        scope: &str,
        main_at: &ChangeId,
    ) -> margincode_core::Result<()> {
        let now = self.running.fetch_add(1, Ordering::SeqCst) + 1;
        self.most.fetch_max(now, Ordering::SeqCst);
        tokio::time::sleep(self.delay).await;
        let r = self.inner.ensure_overlay(repo, scope, main_at).await;
        self.running.fetch_sub(1, Ordering::SeqCst);
        r
    }
    async fn overlay_main_at(
        &self,
        repo: &str,
        scope: &str,
    ) -> margincode_core::Result<Option<ChangeId>> {
        self.inner.overlay_main_at(repo, scope).await
    }
    async fn set_overlay_main_at(
        &self,
        repo: &str,
        scope: &str,
        main_at: &ChangeId,
    ) -> margincode_core::Result<()> {
        self.inner.set_overlay_main_at(repo, scope, main_at).await
    }
    async fn overlay_paths(&self, repo: &str, scope: &str) -> margincode_core::Result<Vec<String>> {
        self.inner.overlay_paths(repo, scope).await
    }
    async fn upsert_overlay_paths(
        &self,
        repo: &str,
        scope: &str,
        files: Vec<(String, Vec<u8>)>,
    ) -> margincode_core::Result<margincode_index::IndexStats> {
        self.inner.upsert_overlay_paths(repo, scope, files).await
    }
    async fn tombstone_overlay_paths(
        &self,
        repo: &str,
        scope: &str,
        paths: &[String],
    ) -> margincode_core::Result<()> {
        self.inner.tombstone_overlay_paths(repo, scope, paths).await
    }
    async fn clear_overlay_paths(
        &self,
        repo: &str,
        scope: &str,
        paths: &[String],
    ) -> margincode_core::Result<()> {
        self.inner.clear_overlay_paths(repo, scope, paths).await
    }
    async fn clear_overlay(&self, repo: &str, scope: &str) -> margincode_core::Result<()> {
        self.inner.clear_overlay(repo, scope).await
    }
    async fn list_code_chunks(
        &self,
        repo: Option<&str>,
    ) -> margincode_core::Result<Vec<margincode_index::CodeChunk>> {
        self.inner.list_code_chunks(repo).await
    }
    async fn chunk_exists(&self, id: margincode_core::SymbolId) -> margincode_core::Result<bool> {
        self.inner.chunk_exists(id).await
    }
    async fn remove_paths(
        &self,
        repo: &str,
        scope: &str,
        paths: &[String],
    ) -> margincode_core::Result<()> {
        self.inner.remove_paths(repo, scope, paths).await
    }
    async fn symbols_in(
        &self,
        view: &margincode_index::ScopeView,
        path: &str,
    ) -> margincode_core::Result<Vec<margincode_index::IndexedSymbol>> {
        self.inner.symbols_in(view, path).await
    }
    async fn get_symbols(
        &self,
        ids: &[margincode_core::SymbolId],
    ) -> margincode_core::Result<Vec<margincode_index::IndexedSymbol>> {
        self.inner.get_symbols(ids).await
    }
    async fn search_symbols(
        &self,
        view: &margincode_index::ScopeView,
        query: &str,
        limit: usize,
    ) -> margincode_core::Result<Vec<margincode_index::IndexedSymbol>> {
        self.inner.search_symbols(view, query, limit).await
    }
    async fn resolve(
        &self,
        view: &margincode_index::ScopeView,
        anchor: &margincode_core::Anchor,
    ) -> margincode_core::Result<margincode_index::AnchorResolution> {
        self.inner.resolve(view, anchor).await
    }
    async fn search_code_lexical(
        &self,
        view: &margincode_index::ScopeView,
        terms: &[String],
        limit: usize,
    ) -> margincode_core::Result<Vec<(margincode_core::SymbolId, f32)>> {
        self.inner.search_code_lexical(view, terms, limit).await
    }
    async fn neighbors(
        &self,
        view: &margincode_index::ScopeView,
        seeds: &[margincode_core::SymbolId],
        hops: u8,
    ) -> margincode_core::Result<Vec<(margincode_core::SymbolId, u8)>> {
        self.inner.neighbors(view, seeds, hops).await
    }
    async fn in_degree(
        &self,
        view: &margincode_index::ScopeView,
        ids: &[margincode_core::SymbolId],
    ) -> margincode_core::Result<std::collections::HashMap<margincode_core::SymbolId, u32>> {
        self.inner.in_degree(view, ids).await
    }
    async fn symbol_scope(
        &self,
        id: margincode_core::SymbolId,
    ) -> margincode_core::Result<Option<(String, String)>> {
        self.inner.symbol_scope(id).await
    }
    async fn drop_scope(&self, repo: &str, scope: &str) -> margincode_core::Result<()> {
        self.inner.drop_scope(repo, scope).await
    }
    async fn prune_parse_cache(&self) -> margincode_core::Result<u64> {
        self.inner.prune_parse_cache().await
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn a_burst_of_starts_queues_for_overlay_opens_and_all_succeed() {
    let slow: std::sync::Mutex<Option<Arc<SlowOverlay>>> = Default::default();
    let e = env_with_index(
        |inner| {
            let s = Arc::new(SlowOverlay {
                inner,
                running: Default::default(),
                most: Default::default(),
                delay: Duration::from_millis(50),
            });
            *slow.lock().unwrap() = Some(s.clone());
            s
        },
        |s| s.with_start_limit(2),
    )
    .await;
    let slow = slow.lock().unwrap().clone().unwrap();
    let names: Vec<(String, String)> = (0..32)
        .map(|i| (format!("burst {i}"), format!("agent-{i}")))
        .collect();
    let started = join_all(names.iter().map(|(t, a)| start_as(&e, t, a))).await;
    assert_eq!(started.len(), 32);
    let most = slow.most.load(Ordering::SeqCst);
    assert!(
        most <= 2,
        "{most} overlay opens ran at once; the limit is 2"
    );
    assert!(most >= 1);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn a_start_whose_client_leaves_while_queued_leaves_no_task_behind() {
    let e = env_with_index(
        |inner| {
            Arc::new(SlowOverlay {
                inner,
                running: Default::default(),
                most: Default::default(),
                delay: Duration::from_secs(2),
            })
        },
        |s| s.with_start_limit(1),
    )
    .await;
    let start = |title: &'static str| {
        let app = e.app.clone();
        tokio::spawn(async move {
            call(
                &app,
                "POST",
                "/v1/tasks",
                Some(json!({"repo": "demo", "title": title, "goal": "g",
                            "agent": title, "session": title})),
            )
            .await
        })
    };
    let first = start("first");
    tokio::time::sleep(Duration::from_millis(300)).await;
    let second = start("second"); // waits for the first one's turn
    tokio::time::sleep(Duration::from_millis(300)).await;
    second.abort(); // its client gives up while it is queued
    let (status, body) = first.await.unwrap();
    assert_eq!(status, StatusCode::OK, "{body}");
    let (_, tasks) = call(&e.app, "GET", "/v1/tasks?repo=demo", None).await;
    let titles: Vec<&str> = tasks["tasks"]
        .as_array()
        .unwrap_or_else(|| panic!("{tasks}"))
        .iter()
        .map(|t| t["title"].as_str().unwrap())
        .collect();
    assert_eq!(titles, ["first"], "{tasks}");
}

async fn set_profile(e: &Env, name: &str, best: &[&str], can: &[&str], cost: i32) {
    let (status, body) = call(
        &e.app,
        "PUT",
        &format!("/v1/agents/{name}"),
        Some(json!({"best": best, "can": can, "cost_rank": cost})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
}

/// The example set from the design, plus a fifth agent that cannot do code or images.
async fn five_profiles(e: &Env) {
    set_profile(e, "grok", &["image"], &["ui"], 2).await;
    set_profile(e, "claude", &["code", "tests"], &["docs", "ui"], 4).await;
    set_profile(e, "muse", &[], &["code", "docs"], 1).await;
    set_profile(e, "codex", &["math"], &["code"], 3).await;
    set_profile(e, "gemini", &[], &["docs", "ui"], 3).await;
}

async fn post_items(e: &Env, tag: &str, n: usize, reserve: i64) {
    for i in 0..n {
        let needs: Vec<&str> = if tag.is_empty() { vec![] } else { vec![tag] };
        let (status, body) = post(
            e,
            "/v1/backlog",
            json!({"repo": "demo", "title": format!("{} item {i}", if tag.is_empty() { "untagged" } else { tag }),
                   "goal": "Extend the greeter", "needs": needs, "reserve_secs": reserve}),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
    }
}

async fn pull_all(e: &Env, agent: &str) -> Vec<String> {
    let mut got = Vec::new();
    loop {
        let (status, body) = post(
            e,
            "/v1/backlog/next",
            json!({"agent": agent, "session": format!("sess-{agent}")}),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{agent}: {body}");
        if body["task"].is_null() {
            return got;
        }
        assert_eq!(body["task"]["owner_agent"], agent, "{body}");
        got.push(body["task"]["title"].as_str().unwrap().to_owned());
    }
}

fn count(titles: &[String], prefix: &str) -> usize {
    titles.iter().filter(|t| t.starts_with(prefix)).count()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn five_agents_pull_a_mixed_backlog_and_each_item_goes_to_its_best_agent() {
    let e = env().await;
    five_profiles(&e).await;
    // 5 image + 5 code + 4 math + 3 docs + 3 untagged = 20, all inside their reserve window
    post_items(&e, "image", 5, 600).await;
    post_items(&e, "code", 5, 600).await;
    post_items(&e, "math", 4, 600).await;
    post_items(&e, "docs", 3, 600).await;
    post_items(&e, "", 3, 600).await;

    let agents = ["grok", "claude", "muse", "codex", "gemini"];
    let started = Instant::now();
    let got: Vec<Vec<String>> = join_all(agents.iter().map(|a| pull_all(&e, a))).await;
    eprintln!("20 items pulled by 5 agents in {:?}", started.elapsed());

    let by = |name: &str| &got[agents.iter().position(|a| *a == name).unwrap()];
    assert_eq!(by("grok").len(), 5);
    assert_eq!(count(by("grok"), "image"), 5);
    assert_eq!(by("claude").len(), 5);
    assert_eq!(count(by("claude"), "code"), 5);
    assert_eq!(by("codex").len(), 4);
    assert_eq!(count(by("codex"), "math"), 4);
    // muse is the cheapest: docs and untagged work
    assert_eq!(by("muse").len(), 6);
    assert_eq!(count(by("muse"), "docs") + count(by("muse"), "untagged"), 6);
    // gemini is ranked first on nothing, so inside the reserve windows it gets nothing
    assert!(by("gemini").is_empty(), "{:?}", by("gemini"));

    let (_, list) = call(&e.app, "GET", "/v1/backlog?state=taken", None).await;
    let items = list["items"].as_array().unwrap();
    assert_eq!(items.len(), 20);
    let mut tasks: Vec<&str> = items
        .iter()
        .map(|i| i["task_id"].as_str().unwrap())
        .collect();
    tasks.sort();
    tasks.dedup();
    assert_eq!(tasks.len(), 20, "two items share a task");
    let (_, queued) = call(&e.app, "GET", "/v1/backlog?state=queued", None).await;
    assert!(queued["items"].as_array().unwrap().is_empty());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn when_the_best_agent_stops_pulling_the_others_finish_the_work() {
    let e = env().await;
    five_profiles(&e).await;
    // claude's quota is out: it never pulls. The reserve window has passed (reserve 0), so any
    // agent that can do code takes the code items; grok and gemini cannot, and image items stay.
    post_items(&e, "code", 6, 0).await;
    post_items(&e, "image", 2, 0).await;

    let agents = ["muse", "codex", "gemini"]; // not grok, not claude
    let got: Vec<Vec<String>> = join_all(agents.iter().map(|a| pull_all(&e, a))).await;
    let total: usize = got.iter().map(Vec::len).sum();
    assert_eq!(total, 6, "{got:?}");
    assert!(
        got[0].len() + got[1].len() == 6,
        "muse and codex share the code items"
    );
    assert!(got[2].is_empty(), "gemini cannot do code");
    let (_, queued) = call(&e.app, "GET", "/v1/backlog?state=queued", None).await;
    let left: Vec<&str> = queued["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|i| i["title"].as_str().unwrap())
        .collect();
    assert_eq!(left.len(), 2, "{left:?}");
    assert!(left.iter().all(|t| t.starts_with("image")));

    assert_eq!(pull_all(&e, "grok").await.len(), 2);
}

/// What one looping agent saw: the tasks it worked on, the ones it reviewed (with their owners),
/// and how many times it called `next`.
#[derive(Debug, Default)]
struct LoopRun {
    worked: Vec<String>,
    reviewed: Vec<(String, String)>,
    calls: usize,
}

/// An agent that does only what `next` returns, as the agent instructions say: review, merge,
/// take a handoff, or work a new task. It stops after `idle` empty waits in a row. When
/// `hand_off` is set, it hands off the first task it starts, with no tags, instead of finishing.
async fn loop_agent(e: &Env, agent: &str, idle: usize, hand_off: bool) -> LoopRun {
    let mut run = LoopRun::default();
    let mut empty = 0;
    let mut handed = !hand_off;
    while empty < idle {
        run.calls += 1;
        let (status, body) = post(
            e,
            "/v1/backlog/next",
            json!({"agent": agent, "session": format!("sess-{agent}"), "wait_secs": 1}),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{agent}: {body}");
        if body["task"].is_null() {
            empty += 1;
            continue;
        }
        empty = 0;
        let tid = body["task"]["id"].as_str().unwrap().to_owned();
        let task_op = |op: &str| format!("/v1/tasks/{tid}/{op}");
        if body["from"] == "review" {
            let owner = body["task"]["owner_agent"].as_str().unwrap().to_owned();
            assert_ne!(owner, agent, "{agent} was offered its own review: {body}");
            if body["approved"] == false {
                let (s, b) = post(
                    e,
                    &task_op("validate"),
                    json!({"agent": agent, "command": "pytest", "exit_code": 0}),
                )
                .await;
                assert_eq!(s, StatusCode::OK, "{b}");
                let (s, b) = post(
                    e,
                    &task_op("review"),
                    json!({"reviewer": agent, "verdict": "approve"}),
                )
                .await;
                assert_eq!(s, StatusCode::OK, "{b}");
            }
            let (s, b) = post(e, &task_op("merge"), json!({"agent": agent})).await;
            assert_eq!(s, StatusCode::OK, "{agent} merging {tid}: {b}");
            run.reviewed.push((tid, owner));
            continue;
        }
        // new work, a handoff or an abandoned task: each task writes its own file, so merges
        // never conflict
        assert_eq!(body["task"]["owner_agent"], agent, "{body}");
        let slug = body["task"]["slug"].as_str().unwrap().to_owned();
        let module = slug.replace('-', "_");
        std::fs::write(
            ws_dir(e, &body).join(format!("app/{module}.py")),
            format!("def {module}():\n    return {:?}\n", agent),
        )
        .unwrap();
        let (s, b) = post(
            e,
            &task_op("records"),
            json!({"kind": "note", "summary": format!("{agent} wrote app/{module}.py"),
                   "agent": agent, "anchors": [{"path": format!("app/{module}.py")}]}),
        )
        .await;
        assert_eq!(s, StatusCode::OK, "{b}");
        if !handed {
            handed = true;
            let (s, b) = post(
                e,
                &task_op("handoff"),
                json!({"agent": agent, "done": "a first draft", "remaining": "finish it",
                       "broken": "", "questions": ""}),
            )
            .await;
            assert_eq!(s, StatusCode::OK, "{b}");
            continue;
        }
        let (s, b) = post(e, &task_op("submit"), json!({"agent": agent})).await;
        assert_eq!(s, StatusCode::OK, "{agent} submitting {tid}: {b}");
        run.worked.push(tid);
    }
    run
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn swarm_loop_runs_backlog_to_merged() {
    let e = env().await;
    set_profile(&e, "claude", &["code", "tests"], &["docs", "ui"], 4).await;
    set_profile(&e, "grok", &["image"], &["ui", "docs"], 2).await;
    post_items(&e, "code", 2, 600).await;
    post_items(&e, "image", 1, 600).await;
    post_items(&e, "", 1, 600).await;

    let started = Instant::now();
    let (claude, grok) = futures::join!(
        loop_agent(&e, "claude", 4, true),
        loop_agent(&e, "grok", 4, false)
    );
    eprintln!(
        "loop: {:?}\nclaude {claude:?}\ngrok {grok:?}",
        started.elapsed()
    );

    let (_, list) = call(&e.app, "GET", "/v1/backlog?state=taken", None).await;
    let items = list["items"].as_array().unwrap();
    assert_eq!(items.len(), 4, "{list}");
    for item in items {
        let tid = item["task_id"].as_str().unwrap();
        let (_, t) = call(&e.app, "GET", &format!("/v1/tasks/{tid}"), None).await;
        assert_eq!(t["task"]["status"], "merged", "{t}");
    }
    // claude handed its first code task off untagged: grok finished it
    assert_eq!(claude.worked.len(), 1, "{claude:?}");
    assert_eq!(grok.worked.len(), 3, "{grok:?}");
    let mut reviewed: Vec<&str> = claude
        .reviewed
        .iter()
        .chain(&grok.reviewed)
        .map(|(t, _)| t.as_str())
        .collect();
    reviewed.sort();
    let mut worked: Vec<&str> = claude
        .worked
        .iter()
        .chain(&grok.worked)
        .map(String::as_str)
        .collect();
    worked.sort();
    assert_eq!(reviewed, worked);
    assert!(claude.reviewed.iter().all(|(_, o)| o == "grok"));
    assert!(grok.reviewed.iter().all(|(_, o)| o == "claude"));
    assert!(
        claude.calls <= 25 && grok.calls <= 25,
        "{claude:?} {grok:?}"
    );
}

/// A decision on `main`'s greeter and a promote of it, proposed by a review run (`dreamer`).
/// Answers the verdict's full id and its `vd:` handle.
async fn proposed_promote(e: &Env) -> (uuid::Uuid, String) {
    let t = start_task(e, "Earlier work").await;
    let (status, rec) = post(
        e,
        &format!("/v1/tasks/{}/records", id(&t)),
        json!({"kind": "decision", "summary": "hello never shouts", "agent": "claude",
               "anchors": [{"path": "app/greeter.py"}]}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{rec}");
    let handle = format!("rec:{}", &rec["id"].as_str().unwrap().replace('-', "")[..8]);
    let (status, d) = post(e, "/v1/repos/demo/dreams", json!({"agent": "dreamer"})).await;
    assert_eq!(status, StatusCode::OK, "{d}");
    let (status, r) = post(
        e,
        &format!("/v1/dreams/{}/verdicts", d["dream"]["id"].as_str().unwrap()),
        json!({"verdicts": [{"record": handle, "verdict": "promote",
               "text": "greetings never shout", "scope_glob": "app/**",
               "evidence": [{"record": handle, "quote": "hello never shouts"}]}]}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{r}");
    let v = &r["results"][0]["verdict"];
    assert!(v.is_object(), "{r}");
    (
        v["id"].as_str().unwrap().parse().unwrap(),
        v["handle"].as_str().unwrap().to_owned(),
    )
}

/// Holds the verdict's row lock until `release` is called, so requests that need it queue up.
async fn hold_verdict(e: &Env, verdict: uuid::Uuid) -> sqlx::Transaction<'static, sqlx::Postgres> {
    let mut tx = e.store.pool().begin().await.unwrap();
    sqlx::query("SELECT 1 FROM record_verdicts WHERE id = $1 FOR UPDATE")
        .bind(verdict)
        .execute(&mut *tx)
        .await
        .unwrap();
    tx
}

/// Waits (with a deadline) until `n` statements wait for a lock.
/// Only statements of this test's database whose text names `table` count: on a server shared
/// by many tests (`AV_TEST_DATABASE_URL`) other tests' lock waits are not this one's.
async fn lock_waiters(e: &Env, table: &str, n: i64) {
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        let waiting: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM pg_stat_activity
              WHERE datname = current_database() AND wait_event_type = 'Lock'
                AND query LIKE '%' || $1 || '%'",
        )
        .bind(table)
        .fetch_one(e.store.pool())
        .await
        .unwrap();
        if waiting >= n {
            return;
        }
        assert!(
            Instant::now() < deadline,
            "{waiting} of {n} requests wait for the lock"
        );
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
}

fn spawn_post(e: &Env, uri: String, body: Value) -> tokio::task::JoinHandle<(StatusCode, Value)> {
    let app = e.app.clone();
    tokio::spawn(async move { call(&app, "POST", &uri, Some(body)).await })
}

async fn promoted_rows(e: &Env, verdict: uuid::Uuid) -> (i64, i64) {
    let reviews: i64 =
        sqlx::query_scalar("SELECT count(*) FROM verdict_reviews WHERE verdict_id = $1")
            .bind(verdict)
            .fetch_one(e.store.pool())
            .await
            .unwrap();
    let live: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM conventions WHERE source = $1 AND removed_at IS NULL",
    )
    .bind(format!("vd:{}", verdict.simple()))
    .fetch_one(e.store.pool())
    .await
    .unwrap();
    (reviews, live)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn two_people_accepting_one_promote_at_once_leave_one_review_and_one_convention() {
    let e = env().await;
    let (verdict, vd) = proposed_promote(&e).await;
    let hold = hold_verdict(&e, verdict).await;
    let uri = format!("/v1/verdicts/{vd}/review");
    let first = spawn_post(&e, uri.clone(), json!({"reviewer": "vinh", "agree": true}));
    let second = spawn_post(&e, uri, json!({"reviewer": "mai", "agree": true}));
    lock_waiters(&e, "record_verdicts", 2).await;
    hold.commit().await.unwrap();
    let mut answers = vec![first.await.unwrap(), second.await.unwrap()];
    answers.sort_by_key(|(s, _)| s.as_u16());
    assert_eq!(answers[0].0, StatusCode::OK, "{answers:?}");
    assert_eq!(answers[1].0, StatusCode::CONFLICT, "{answers:?}");
    assert_eq!(promoted_rows(&e, verdict).await, (1, 1));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn a_retraction_that_races_an_accept_leaves_no_live_convention() {
    let e = env().await;
    let (verdict, vd) = proposed_promote(&e).await;
    let hold = hold_verdict(&e, verdict).await;
    let retract = spawn_post(
        &e,
        format!("/v1/verdicts/{vd}/retract"),
        json!({"by": "vinh"}),
    );
    lock_waiters(&e, "record_verdicts", 1).await;
    let accept = spawn_post(
        &e,
        format!("/v1/verdicts/{vd}/review"),
        json!({"reviewer": "mai", "agree": true}),
    );
    lock_waiters(&e, "record_verdicts", 2).await;
    hold.commit().await.unwrap();
    let (rs, rb) = retract.await.unwrap();
    assert_eq!(rs, StatusCode::OK, "{rb}");
    let (as_, ab) = accept.await.unwrap();
    assert_eq!(
        as_,
        StatusCode::CONFLICT,
        "the accept sees the retraction: {ab}"
    );
    assert_eq!(promoted_rows(&e, verdict).await, (0, 0));
}

/// A decision on `main`'s greeter and a review run handed it; answers the record's id, its
/// `rec:` handle and the run.
async fn decision_in_a_run(e: &Env) -> (uuid::Uuid, String, Value) {
    let t = start_task(e, "Earlier work").await;
    let (status, rec) = post(
        e,
        &format!("/v1/tasks/{}/records", id(&t)),
        json!({"kind": "decision", "summary": "hello never shouts", "agent": "claude",
               "anchors": [{"path": "app/greeter.py"}]}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{rec}");
    let rid: uuid::Uuid = rec["id"].as_str().unwrap().parse().unwrap();
    let handle = format!("rec:{}", &rid.simple().to_string()[..8]);
    let (status, d) = post(e, "/v1/repos/demo/dreams", json!({"agent": "dreamer"})).await;
    assert_eq!(status, StatusCode::OK, "{d}");
    (rid, handle, d)
}

fn promote_body(handle: &str) -> Value {
    json!({"verdicts": [{"record": handle, "verdict": "promote",
           "text": "greetings never shout", "scope_glob": "app/**",
           "evidence": [{"record": handle, "quote": "hello never shouts"}]}]})
}

/// Holds the record's row lock, which stalls a verdict's insert (its foreign key) and, once
/// proposals lock the record, the proposal itself.
async fn hold_record(e: &Env, record: uuid::Uuid) -> sqlx::Transaction<'static, sqlx::Postgres> {
    let mut tx = e.store.pool().begin().await.unwrap();
    sqlx::query("SELECT 1 FROM records WHERE id = $1 FOR UPDATE")
        .bind(record)
        .execute(&mut *tx)
        .await
        .unwrap();
    tx
}

async fn promotes_of(e: &Env, record: uuid::Uuid) -> i64 {
    sqlx::query_scalar(
        "SELECT count(*) FROM record_verdicts WHERE record_id = $1 AND verdict = 'promote'",
    )
    .bind(record)
    .fetch_one(e.store.pool())
    .await
    .unwrap()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn two_identical_promotes_at_once_leave_one_verdict() {
    let e = env().await;
    let (rid, handle, d) = decision_in_a_run(&e).await;
    let uri = format!("/v1/dreams/{}/verdicts", d["dream"]["id"].as_str().unwrap());
    let hold = hold_record(&e, rid).await;
    let one = spawn_post(&e, uri.clone(), promote_body(&handle));
    let two = spawn_post(&e, uri, promote_body(&handle));
    lock_waiters(&e, "record", 2).await;
    hold.commit().await.unwrap();
    let mut accepted = vec![];
    for answer in [one.await.unwrap(), two.await.unwrap()] {
        assert_eq!(answer.0, StatusCode::OK, "{answer:?}");
        accepted.push(answer.1["accepted"].as_u64().unwrap());
    }
    accepted.sort();
    assert_eq!(accepted, [0, 1]);
    assert_eq!(promotes_of(&e, rid).await, 1);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn two_runs_promoting_one_decision_at_once_leave_one_verdict() {
    let e = env().await;
    let (rid, handle, d) = decision_in_a_run(&e).await;
    // a second running review run handed the same record: runs on overlapping areas are refused
    // at start, so it is written as the daemon would, on an area of its own
    let other = uuid::Uuid::new_v4();
    sqlx::query(
        "INSERT INTO dreams (id, repo, mode, area, agent, worklist, lease_expires)
         VALUES ($1, 'demo', 'review', 'elsewhere', 'other', $2, now() + interval '1 hour')",
    )
    .bind(other)
    .bind(json!({"commit": d["dream"]["worklist"]["commit"], "records": [rid]}))
    .execute(e.store.pool())
    .await
    .unwrap();
    let hold = hold_record(&e, rid).await;
    let one = spawn_post(
        &e,
        format!("/v1/dreams/{}/verdicts", d["dream"]["id"].as_str().unwrap()),
        promote_body(&handle),
    );
    let two = spawn_post(
        &e,
        format!("/v1/dreams/{other}/verdicts"),
        promote_body(&handle),
    );
    lock_waiters(&e, "record", 2).await;
    hold.commit().await.unwrap();
    let mut whys = vec![];
    for answer in [one.await.unwrap(), two.await.unwrap()] {
        assert_eq!(answer.0, StatusCode::OK, "{answer:?}");
        whys.push(
            answer.1["results"][0]["why"]
                .as_str()
                .unwrap_or("")
                .to_owned(),
        );
    }
    whys.sort();
    assert_eq!(whys[0], "", "{whys:?}");
    assert!(whys[1].contains("already has a promote"), "{whys:?}");
    assert_eq!(promotes_of(&e, rid).await, 1);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn an_undo_racing_a_retraction_of_one_of_its_verdicts_still_finishes() {
    let e = env().await;
    let (verdict, vd) = proposed_promote(&e).await;
    let dream: uuid::Uuid =
        sqlx::query_scalar("SELECT dream_id FROM record_verdicts WHERE id = $1")
            .bind(verdict)
            .fetch_one(e.store.pool())
            .await
            .unwrap();
    let hold = hold_verdict(&e, verdict).await;
    let retract = spawn_post(
        &e,
        format!("/v1/verdicts/{vd}/retract"),
        json!({"by": "vinh"}),
    );
    lock_waiters(&e, "record_verdicts", 1).await;
    let undo = spawn_post(
        &e,
        format!("/v1/dreams/{dream}/undo"),
        json!({"by": "vinh"}),
    );
    lock_waiters(&e, "record_verdicts", 2).await;
    hold.commit().await.unwrap();
    let (status, body) = retract.await.unwrap();
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, body) = undo.await.unwrap();
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["dream"]["status"], "undone", "{body}");
}
