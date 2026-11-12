//! Shared harness for the daemon's HTTP tests: a TestDb, a fixture git repo imported as `demo`,
//! real `JjVcs`, and helpers to drive the router.
#![allow(dead_code)] // each test file uses a different subset

pub use std::path::{Path, PathBuf};
use std::process::Command;
pub use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

pub use axum::Router;
use axum::body::Body;
pub use axum::http::{Request, StatusCode};
use margincode_core::{
    Advance, ChangeId, LogEntry, MergeOutcome, RebaseOutcome, Result as VcsResult, WorkspaceInfo,
};
pub use margincode_daemon::{AppState, router_with};
pub use margincode_index::{CodeIndex, PgIndex};
pub use margincode_store::PgStore;
pub use margincode_testutil::TestDb;
pub use margincode_vcs::JjVcs;
pub use serde_json::{Value, json};
pub use tempfile::TempDir;
use tower::ServiceExt;

pub const HOST_WS: &str = "/host/ws";

pub fn git(dir: &Path, args: &[&str]) {
    let out = Command::new("git")
        .current_dir(dir)
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

/// `git` in `dir`, returning trimmed stdout.
pub fn git_out(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).trim().to_owned()
}

pub fn fixture_git_repo() -> TempDir {
    let d = TempDir::new().unwrap();
    git(d.path(), &["init", "-q", "-b", "main"]);
    std::fs::create_dir_all(d.path().join("app")).unwrap();
    std::fs::write(
        d.path().join("app/util.py"),
        "def format_name(n):\n    return n.title()\n",
    )
    .unwrap();
    std::fs::write(
        d.path().join("app/greeter.py"),
        "from util import format_name\n\nclass Greeter:\n    def hello(self, name):\n        return format_name(name)\n",
    )
    .unwrap();
    std::fs::write(d.path().join(".gitignore"), "node_modules/\n").unwrap();
    git(d.path(), &["add", "-A"]);
    git(d.path(), &["commit", "-q", "-m", "init"]);
    d
}

pub struct Env {
    pub _db: TestDb,
    pub store: Arc<PgStore>,
    pub _src: TempDir,
    pub _store: TempDir,
    pub ws: TempDir,
    pub app: Router,
    pub src_path: PathBuf,
    /// The state behind `app`, for what has no route (the backup loop's `backup_tick`).
    pub state: AppState,
}

pub async fn env() -> Env {
    env_with(|s| s).await
}

/// `tweak` customizes the state (semantic backends) before the router is built.
pub async fn env_with(tweak: impl FnOnce(AppState) -> AppState) -> Env {
    env_with_vcs(|v| v, tweak).await
}

/// Like `env_with`, and `wrap` may put a test double around the real `JjVcs`.
pub async fn env_with_vcs(
    wrap: impl FnOnce(Arc<dyn margincode_core::Vcs>) -> Arc<dyn margincode_core::Vcs>,
    tweak: impl FnOnce(AppState) -> AppState,
) -> Env {
    env_full(wrap, |i| i, tweak, Router::new()).await
}

/// Like `env_with`, and `wrap` may put a test double around the real `PgIndex`.
pub async fn env_with_index(
    wrap: impl FnOnce(Arc<dyn CodeIndex>) -> Arc<dyn CodeIndex>,
    tweak: impl FnOnce(AppState) -> AppState,
) -> Env {
    env_full(|v| v, wrap, tweak, Router::new()).await
}

/// Like `env_with`, with `extra` routes (`router_with`).
pub async fn env_with_routes(
    tweak: impl FnOnce(AppState) -> AppState,
    extra: Router<AppState>,
) -> Env {
    env_full(|v| v, |i| i, tweak, extra).await
}

async fn env_full(
    wrap: impl FnOnce(Arc<dyn margincode_core::Vcs>) -> Arc<dyn margincode_core::Vcs>,
    wrap_index: impl FnOnce(Arc<dyn CodeIndex>) -> Arc<dyn CodeIndex>,
    tweak: impl FnOnce(AppState) -> AppState,
    extra: Router<AppState>,
) -> Env {
    let db = TestDb::new().await;
    let src = fixture_git_repo();
    let store_dir = TempDir::new().unwrap();
    let ws = TempDir::new().unwrap();
    let store = Arc::new(PgStore::connect(&db.url).await.unwrap());
    let index = wrap_index(Arc::new(PgIndex::new(store.pool().clone())));
    let vcs = wrap(Arc::new(JjVcs::new(
        store_dir.path().to_path_buf(),
        ws.path().to_path_buf(),
    )));
    let state = AppState::new(
        vcs,
        store.clone(),
        index,
        ws.path().to_path_buf(),
        PathBuf::from(HOST_WS),
    );
    let state = tweak(state);
    let token = state.token().map(str::to_owned);
    let app = router_with(state.clone(), extra);
    let e = Env {
        _db: db,
        store,
        src_path: src.path().to_path_buf(),
        _src: src,
        _store: store_dir,
        ws,
        app,
        state,
    };
    let bearer = token.map(|t| format!("Bearer {t}"));
    let headers: Vec<(&str, &str)> = bearer
        .as_deref()
        .map(|b| vec![("authorization", b)])
        .unwrap_or_default();
    let (status, body) = call_with(
        &e.app,
        "POST",
        "/v1/repos",
        Some(json!({"name": "demo", "git_source": e.src_path})),
        &headers,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    e
}

pub async fn call(
    app: &Router,
    method: &str,
    uri: &str,
    body: Option<Value>,
) -> (StatusCode, Value) {
    call_as(app, method, uri, body, None).await
}

/// `call` with an `X-Av-Generation` header (the claim the caller writes under).
pub async fn call_as(
    app: &Router,
    method: &str,
    uri: &str,
    body: Option<Value>,
    generation: Option<i64>,
) -> (StatusCode, Value) {
    let g = generation.map(|g| g.to_string());
    let headers: Vec<(&str, &str)> = g
        .as_deref()
        .map(|g| vec![("x-av-generation", g)])
        .unwrap_or_default();
    call_with(app, method, uri, body, &headers).await
}

/// `call` with extra request headers.
pub async fn call_with(
    app: &Router,
    method: &str,
    uri: &str,
    body: Option<Value>,
    headers: &[(&str, &str)],
) -> (StatusCode, Value) {
    let mut req = Request::builder().method(method).uri(uri);
    for (k, v) in headers {
        req = req.header(*k, *v);
    }
    let body = match body {
        Some(v) => {
            req = req.header("content-type", "application/json");
            Body::from(v.to_string())
        }
        None => Body::empty(),
    };
    let resp = app.clone().oneshot(req.body(body).unwrap()).await.unwrap();
    let status = resp.status();
    let bytes = axum::body::to_bytes(resp.into_body(), 10 * 1024 * 1024)
        .await
        .unwrap();
    let value = serde_json::from_slice(&bytes)
        .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&bytes).into_owned()));
    (status, value)
}

pub async fn start_task(e: &Env, title: &str) -> Value {
    let (status, body) = call(
        &e.app,
        "POST",
        "/v1/tasks",
        Some(json!({"repo": "demo", "title": title, "goal": "Make hello louder", "agent": "claude", "session": "s-1"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    body
}

pub fn ws_dir(e: &Env, task: &Value) -> PathBuf {
    e.ws.path()
        .join("demo")
        .join(task["task"]["slug"].as_str().unwrap())
}

pub fn id(task: &Value) -> String {
    task["task"]["id"].as_str().unwrap().to_owned()
}
pub async fn submit(e: &Env, t: &Value) -> (StatusCode, Value) {
    call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/submit", id(t)),
        Some(json!({"agent": "claude"})),
    )
    .await
}

pub fn edit_greeter(e: &Env, t: &Value) {
    std::fs::write(
        ws_dir(e, t).join("app/greeter.py"),
        "from util import format_name\n\nclass Greeter:\n    def hello(self, name):\n        return format_name(name) + \"!\"\n\n    def wave(self, name):\n        return \"*waves at \" + name\n",
    )
    .unwrap();
}

/// Passes every `Vcs` call to `inner`, the defaulted methods too (so no default body bypasses
/// it), and counts them: `read_files` (the indexing path), `calls` (all of them) and `seen` (the
/// methods, in call order).
pub struct CountingVcs {
    pub inner: Arc<dyn margincode_core::Vcs>,
    pub read_files: Arc<AtomicUsize>,
    pub calls: Arc<AtomicUsize>,
    pub seen: Arc<Mutex<Vec<&'static str>>>,
}

impl CountingVcs {
    /// Fresh counters around `inner`.
    pub fn new(inner: Arc<dyn margincode_core::Vcs>) -> Self {
        Self {
            inner,
            read_files: Arc::default(),
            calls: Arc::default(),
            seen: Arc::default(),
        }
    }

    fn note(&self, method: &'static str) {
        self.calls.fetch_add(1, Ordering::SeqCst);
        self.seen.lock().unwrap().push(method);
    }
}

#[async_trait::async_trait]
impl margincode_core::Vcs for CountingVcs {
    async fn init_repo(&self, repo: &str, src: &Path) -> VcsResult<ChangeId> {
        self.note("init_repo");
        self.inner.init_repo(repo, src).await
    }
    async fn create_workspace(&self, repo: &str, slug: &str) -> VcsResult<WorkspaceInfo> {
        self.note("create_workspace");
        self.inner.create_workspace(repo, slug).await
    }
    async fn snapshot(&self, repo: &str, slug: &str) -> VcsResult<ChangeId> {
        self.note("snapshot");
        self.inner.snapshot(repo, slug).await
    }
    async fn changed_paths(
        &self,
        repo: &str,
        base: &ChangeId,
        head: &ChangeId,
    ) -> VcsResult<Vec<String>> {
        self.note("changed_paths");
        self.inner.changed_paths(repo, base, head).await
    }
    async fn list_files(&self, repo: &str, change: &ChangeId) -> VcsResult<Vec<String>> {
        self.note("list_files");
        self.inner.list_files(repo, change).await
    }
    async fn read_file(&self, repo: &str, change: &ChangeId, path: &str) -> VcsResult<Vec<u8>> {
        self.note("read_file");
        self.inner.read_file(repo, change, path).await
    }
    async fn read_files(
        &self,
        repo: &str,
        change: &ChangeId,
        paths: &[String],
    ) -> VcsResult<Vec<(String, VcsResult<Vec<u8>>)>> {
        self.note("read_files");
        self.read_files.fetch_add(1, Ordering::SeqCst);
        self.inner.read_files(repo, change, paths).await
    }
    async fn list_repos(&self) -> VcsResult<Vec<String>> {
        self.note("list_repos");
        self.inner.list_repos().await
    }
    async fn trunk(&self, repo: &str) -> VcsResult<ChangeId> {
        self.note("trunk");
        self.inner.trunk(repo).await
    }
    async fn merge_task(
        &self,
        repo: &str,
        slug: &str,
        expected_head: &ChangeId,
        message: &str,
    ) -> VcsResult<MergeOutcome> {
        self.note("merge_task");
        self.inner
            .merge_task(repo, slug, expected_head, message)
            .await
    }
    async fn push_backup(
        &self,
        repo: &str,
        remote_url: &str,
        branch: Option<&str>,
    ) -> VcsResult<ChangeId> {
        self.note("push_backup");
        self.inner.push_backup(repo, remote_url, branch).await
    }
    async fn advance_trunk(
        &self,
        repo: &str,
        remote_url: &str,
        branch: &str,
    ) -> VcsResult<Advance> {
        self.note("advance_trunk");
        self.inner.advance_trunk(repo, remote_url, branch).await
    }
    async fn trunk_log(&self, repo: &str, limit: usize) -> VcsResult<Vec<LogEntry>> {
        self.note("trunk_log");
        self.inner.trunk_log(repo, limit).await
    }
    async fn history(&self, repo: &str, limit: usize) -> VcsResult<Vec<LogEntry>> {
        self.note("history");
        self.inner.history(repo, limit).await
    }
    async fn path_log(
        &self,
        repo: &str,
        path: &str,
        limit: usize,
        scan: usize,
    ) -> VcsResult<Vec<LogEntry>> {
        self.note("path_log");
        self.inner.path_log(repo, path, limit, scan).await
    }
    async fn rebase_workspace(&self, repo: &str, slug: &str) -> VcsResult<RebaseOutcome> {
        self.note("rebase_workspace");
        self.inner.rebase_workspace(repo, slug).await
    }
    async fn revert_merge(
        &self,
        repo: &str,
        merge_commit: &ChangeId,
        message: &str,
    ) -> VcsResult<MergeOutcome> {
        self.note("revert_merge");
        self.inner.revert_merge(repo, merge_commit, message).await
    }
    async fn undo_merge(
        &self,
        repo: &str,
        pre_merge: &ChangeId,
        merge_commit: &ChangeId,
    ) -> VcsResult<()> {
        self.note("undo_merge");
        self.inner.undo_merge(repo, pre_merge, merge_commit).await
    }
    async fn remove_workspace(&self, repo: &str, slug: &str) -> VcsResult<bool> {
        self.note("remove_workspace");
        self.inner.remove_workspace(repo, slug).await
    }
    async fn restore_workspace(
        &self,
        repo: &str,
        slug: &str,
        head: &ChangeId,
    ) -> VcsResult<WorkspaceInfo> {
        self.note("restore_workspace");
        self.inner.restore_workspace(repo, slug, head).await
    }
    async fn apply_patch(&self, repo: &str, slug: &str, patch: &[u8]) -> VcsResult<Vec<String>> {
        self.note("apply_patch");
        self.inner.apply_patch(repo, slug, patch).await
    }
    async fn diff_patch(
        &self,
        repo: &str,
        from: Option<&ChangeId>,
        to: &ChangeId,
    ) -> VcsResult<Vec<u8>> {
        self.note("diff_patch");
        self.inner.diff_patch(repo, from, to).await
    }
    async fn tree_id(&self, repo: &str, change: &ChangeId) -> VcsResult<String> {
        self.note("tree_id");
        self.inner.tree_id(repo, change).await
    }
}

pub async fn record_decision(e: &Env, t: &Value) {
    let (status, body) = call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/records", id(t)),
        Some(
            json!({"kind": "decision", "summary": "hello shouts", "agent": "claude",
                    "anchors": [{"path": "app/greeter.py", "symbol": "Greeter.hello"}]}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
}

pub async fn handoff(e: &Env, t: &Value, agent: &str) -> (StatusCode, Value) {
    call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/handoff", id(t)),
        Some(
            json!({"agent": agent, "done": "hello shouts", "remaining": "make goodbye louder",
                    "broken": "", "questions": "all caps?"}),
        ),
    )
    .await
}

pub async fn claim(e: &Env, t: &Value, agent: &str) -> (StatusCode, Value) {
    call(
        &e.app,
        "POST",
        &format!("/v1/tasks/{}/claim", id(t)),
        Some(json!({"agent": agent})),
    )
    .await
}
