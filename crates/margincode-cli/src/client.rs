use std::cell::Cell;
use std::fmt;
use std::time::{Duration, Instant};

use serde_json::Value;

#[derive(Debug)]
pub enum CliError {
    Unreachable {
        url: String,
        detail: String,
    },
    /// `body` is the daemon's whole JSON answer (a stale remote checkpoint's `head_change`).
    Api {
        status: u16,
        message: String,
        body: Value,
    },
    NoTask,
    Local(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::Unreachable { url, detail } => write!(
                f,
                "cannot reach the Margincode daemon at {url} ({detail}). Start one with `av up` (or `docker compose up -d`), or set AV_URL to point elsewhere."
            ),
            CliError::Api {
                status, message, ..
            } => write!(f, "{message} (HTTP {status})"),
            CliError::NoTask => write!(
                f,
                "no task here: run `av task start --repo R --title T --goal G`, or pass --task <id> / set AV_TASK"
            ),
            CliError::Local(m) => write!(f, "{m}"),
        }
    }
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Characters of a plain-text error body shown in the message (a proxy's HTML page can be long).
const MAX_PLAIN_ERROR: usize = 300;

pub type Result<T> = std::result::Result<T, CliError>;

const CONNECT: Duration = Duration::from_secs(5);
const OVERALL: Duration = Duration::from_secs(300);

pub struct Client {
    base: String,
    agent: ureq::Agent,
    token: Option<String>,
    warned: Cell<bool>,
    budget: Option<Budget>,
}

/// How long a hook may wait on the daemon, all its requests together. The clock starts at the
/// first request; after a transport error (refused, timed out) nothing more is sent.
struct Budget {
    total: Duration,
    started: Cell<Option<Instant>>,
    failed: Cell<bool>,
    /// How long the request that failed had waited.
    waited: Cell<Duration>,
}

impl Budget {
    fn fail(&self) {
        self.failed.set(true);
        if let Some(t) = self.started.get() {
            self.waited.set(t.elapsed());
        }
    }
}

fn agent(connect: Duration) -> ureq::Agent {
    ureq::AgentBuilder::new()
        .timeout_connect(connect)
        .timeout(OVERALL)
        .build()
}

impl Client {
    pub fn new(base: &str, token: Option<String>) -> Self {
        Client {
            base: base.trim_end_matches('/').to_owned(),
            agent: agent(CONNECT),
            token: token.filter(|t| !t.trim().is_empty()),
            warned: Cell::new(false),
            budget: None,
        }
    }

    /// The same daemon for a hook that gives up after `total` of waiting on it. The connect
    /// timeout is capped too: in ureq 2.12 an agent's `timeout_connect` replaces the request's
    /// deadline while connecting (`stream.rs`, `connect_deadline`).
    pub fn with_budget(&self, total: Duration) -> Client {
        Client {
            base: self.base.clone(),
            agent: agent(CONNECT.min(total)),
            token: self.token.clone(),
            warned: Cell::new(false),
            budget: Some(Budget {
                total,
                started: Cell::new(None),
                failed: Cell::new(false),
                waited: Cell::new(Duration::ZERO),
            }),
        }
    }

    pub fn base(&self) -> &str {
        &self.base
    }

    /// Starts the budget over at the next request (a poll after a deliberate wait), unless a
    /// request already failed.
    pub fn restart_budget(&self) {
        if let Some(b) = &self.budget {
            b.started.set(None);
        }
    }

    /// Sends nothing more, as after a failed request.
    pub fn give_up(&self) {
        if let Some(b) = &self.budget {
            b.failed.set(true);
        }
    }

    /// Whether this hook gave up on the daemon after waiting on it for at least `min` (a
    /// refused connection fails at once, a hung daemon after the budget).
    pub fn gave_up_after(&self, min: Duration) -> bool {
        self.budget
            .as_ref()
            .is_some_and(|b| b.failed.get() && b.waited.get() >= min)
    }

    fn request(&self, method: &str, path: &str, cap: Duration) -> Result<ureq::Request> {
        let mut req = self
            .agent
            .request(method, &format!("{}{path}", self.base))
            .timeout(cap);
        if let Some(b) = &self.budget {
            let unreachable = |detail: String| CliError::Unreachable {
                url: self.base.clone(),
                detail,
            };
            if b.failed.get() {
                return Err(unreachable("an earlier request of this hook failed".into()));
            }
            let started = b.started.get().unwrap_or_else(|| {
                let now = Instant::now();
                b.started.set(Some(now));
                now
            });
            let left = b.total.saturating_sub(started.elapsed());
            if left.is_zero() {
                b.fail();
                return Err(unreachable(format!(
                    "no answer within the hook's {} s",
                    b.total.as_secs()
                )));
            }
            req = req.timeout(cap.min(left));
        }
        Ok(match &self.token {
            Some(t) => req.set("Authorization", &format!("Bearer {}", t.trim())),
            None => req,
        })
    }

    pub fn post(&self, path: &str, body: &Value) -> Result<Value> {
        self.post_as(path, body, None)
    }

    /// The daemon refuses it once the task has been claimed again.
    pub fn post_as(&self, path: &str, body: &Value, generation: Option<i64>) -> Result<Value> {
        let mut req = self.request("POST", path, OVERALL)?;
        if let Some(g) = generation {
            req = req.set("x-av-generation", &g.to_string());
        }
        self.finish(path, req.send_json(body))
    }

    /// Asks the daemon to name the owner in the `.av-task` it hands out (`X-Av-Name-Owner`).
    pub fn post_naming_owner(&self, path: &str, body: &Value) -> Result<Value> {
        let req = self.request("POST", path, OVERALL)?;
        self.finish(path, req.set("x-av-name-owner", "1").send_json(body))
    }

    pub fn delete(&self, path: &str) -> Result<Value> {
        let req = self.request("DELETE", path, OVERALL)?;
        self.finish(path, req.call())
    }

    pub fn delete_with(&self, path: &str, body: &Value) -> Result<Value> {
        let req = self.request("DELETE", path, OVERALL)?;
        self.finish(path, req.send_json(body))
    }

    pub fn put(&self, path: &str, body: &Value) -> Result<Value> {
        let req = self.request("PUT", path, OVERALL)?;
        self.finish(path, req.send_json(body))
    }

    pub fn get(&self, path: &str, query: &[(&str, &str)]) -> Result<Value> {
        let mut req = self.request("GET", path, OVERALL)?;
        for (k, v) in query {
            req = req.query(k, v);
        }
        self.finish(path, req.call())
    }

    /// `get` of a plain-text body (e.g. `/healthz`), with a short timeout.
    pub fn get_text(&self, path: &str) -> Result<String> {
        let req = self.request("GET", path, Duration::from_secs(2))?;
        match req.call() {
            Ok(resp) => resp
                .into_string()
                .map_err(|e| CliError::Local(format!("reading {path}: {e}"))),
            other => self.finish(path, other).map(|_| String::new()),
        }
    }

    /// `post_as` with a raw body (a remote checkpoint's patch) and query parameters.
    pub fn post_bytes(
        &self,
        path: &str,
        query: &[(&str, &str)],
        body: &[u8],
        generation: Option<i64>,
    ) -> Result<Value> {
        let mut req = self.request("POST", path, OVERALL)?;
        for (k, v) in query {
            req = req.query(k, v);
        }
        if let Some(g) = generation {
            req = req.set("x-av-generation", &g.to_string());
        }
        let req = req.set("content-type", "text/x-diff");
        self.finish(path, req.send_bytes(body))
    }

    pub fn get_bytes_with(
        &self,
        path: &str,
        query: &[(&str, &str)],
        want: &[&str],
    ) -> Result<(Vec<u8>, Vec<Option<String>>)> {
        let mut req = self.request("GET", path, OVERALL)?;
        for (k, v) in query {
            req = req.query(k, v);
        }
        match req.call() {
            Ok(resp) => {
                self.check_version(&resp);
                let headers = want
                    .iter()
                    .map(|h| resp.header(h).map(str::to_owned))
                    .collect();
                let mut bytes = Vec::new();
                std::io::Read::read_to_end(&mut resp.into_reader(), &mut bytes)
                    .map_err(|e| CliError::Local(format!("reading {path}: {e}")))?;
                Ok((bytes, headers))
            }
            other => self.finish(path, other).map(|_| (Vec::new(), Vec::new())),
        }
    }

    pub fn get_bytes(&self, path: &str, query: &[(&str, &str)]) -> Result<Vec<u8>> {
        let mut req = self.request("GET", path, OVERALL)?;
        for (k, v) in query {
            req = req.query(k, v);
        }
        match req.call() {
            Ok(resp) => {
                let mut bytes = Vec::new();
                std::io::Read::read_to_end(&mut resp.into_reader(), &mut bytes)
                    .map_err(|e| CliError::Local(format!("reading {path}: {e}")))?;
                Ok(bytes)
            }
            other => self.finish(path, other).map(|_| Vec::new()),
        }
    }

    fn check_version(&self, resp: &ureq::Response) {
        if let Some(v) = resp.header("x-av-version")
            && v != VERSION
            && !self.warned.replace(true)
        {
            eprintln!(
                "av: warning: the daemon at {} is version {v} but this av is {VERSION}; some commands may not work until they match",
                self.base
            );
        }
    }

    fn finish(
        &self,
        path: &str,
        res: std::result::Result<ureq::Response, ureq::Error>,
    ) -> Result<Value> {
        match &res {
            Ok(resp) | Err(ureq::Error::Status(_, resp)) => self.check_version(resp),
            Err(_) => {}
        }
        match res {
            Ok(resp) => resp
                .into_json::<Value>()
                .map_err(|e| CliError::Local(format!("bad response from daemon: {e}"))),
            Err(ureq::Error::Status(status, resp)) => {
                let daemon = resp.header("x-av-version").map(str::to_owned);
                // axum answers a path or body it cannot parse in plain text, which says what is wrong.
                let text = resp.into_string().unwrap_or_default();
                let body = serde_json::from_str::<Value>(&text).unwrap_or(Value::Null);
                let plain = text.trim();
                let plain = (body.is_null() && !plain.is_empty())
                    .then(|| plain.chars().take(MAX_PLAIN_ERROR).collect::<String>());
                let message = body["error"].as_str().map(str::to_owned);
                // A 404 the daemon's own handlers send carries an `error`; a bare one means the
                // route does not exist there, and every route this av calls exists in this av.
                let message = match (message, status) {
                    (Some(m), _) => m,
                    (None, 404) => {
                        let theirs = daemon
                            .map(|v| format!("version {v}"))
                            .unwrap_or_else(|| "an unknown version".to_owned());
                        format!(
                            "the daemon at {} does not serve {path}: it is probably older than this av (daemon {theirs}, av {VERSION}); update the daemon",
                            self.base
                        )
                    }
                    (None, _) => plain.unwrap_or_else(|| "request failed".to_owned()),
                };
                Err(CliError::Api {
                    status,
                    message,
                    body,
                })
            }
            Err(ureq::Error::Transport(t)) => {
                if let Some(b) = &self.budget {
                    b.fail();
                }
                Err(CliError::Unreachable {
                    url: self.base.clone(),
                    detail: t.to_string(),
                })
            }
        }
    }
}
