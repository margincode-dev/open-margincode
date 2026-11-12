use std::fmt;
use std::time::Duration;

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

/// This build's version, compared with the daemon's `X-Av-Version`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Characters of a plain-text error body shown in the message (a proxy's HTML page can be long).
const MAX_PLAIN_ERROR: usize = 300;

pub type Result<T> = std::result::Result<T, CliError>;

pub struct Client {
    base: String,
    agent: ureq::Agent,
    /// `Authorization: Bearer …` on every request when set.
    token: Option<String>,
    /// The version warning is printed once per process.
    warned: std::cell::Cell<bool>,
}

impl Client {
    pub fn new(base: &str, token: Option<String>) -> Self {
        let agent = ureq::AgentBuilder::new()
            .timeout_connect(Duration::from_secs(5))
            .timeout(Duration::from_secs(300))
            .build();
        Client {
            base: base.trim_end_matches('/').to_owned(),
            agent,
            token: token.filter(|t| !t.trim().is_empty()),
            warned: std::cell::Cell::new(false),
        }
    }

    fn authorized(&self, req: ureq::Request) -> ureq::Request {
        match &self.token {
            Some(t) => req.set("Authorization", &format!("Bearer {}", t.trim())),
            None => req,
        }
    }

    pub fn post(&self, path: &str, body: &Value) -> Result<Value> {
        self.post_as(path, body, None)
    }

    /// `post` made under claim `generation` (`X-Av-Generation`); the daemon refuses it once the
    /// task has been claimed again.
    pub fn post_as(&self, path: &str, body: &Value, generation: Option<i64>) -> Result<Value> {
        let mut req = self.authorized(self.agent.post(&format!("{}{path}", self.base)));
        if let Some(g) = generation {
            req = req.set("x-av-generation", &g.to_string());
        }
        self.finish(path, req.send_json(body))
    }

    pub fn delete(&self, path: &str) -> Result<Value> {
        let req = self.authorized(self.agent.delete(&format!("{}{path}", self.base)));
        self.finish(path, req.call())
    }

    pub fn put(&self, path: &str, body: &Value) -> Result<Value> {
        let req = self.authorized(self.agent.put(&format!("{}{path}", self.base)));
        self.finish(path, req.send_json(body))
    }

    pub fn get(&self, path: &str, query: &[(&str, &str)]) -> Result<Value> {
        let mut req = self.authorized(self.agent.get(&format!("{}{path}", self.base)));
        for (k, v) in query {
            req = req.query(k, v);
        }
        self.finish(path, req.call())
    }

    /// `get` of a plain-text body (e.g. `/healthz`), with a short timeout.
    pub fn get_text(&self, path: &str) -> Result<String> {
        let req = self
            .authorized(self.agent.get(&format!("{}{path}", self.base)))
            .timeout(Duration::from_secs(2));
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
        let mut req = self.authorized(self.agent.post(&format!("{}{path}", self.base)));
        for (k, v) in query {
            req = req.query(k, v);
        }
        if let Some(g) = generation {
            req = req.set("x-av-generation", &g.to_string());
        }
        let req = req.set("content-type", "text/x-diff");
        self.finish(path, req.send_bytes(body))
    }

    /// `get` of a raw body with the response headers named in `want`, in that order.
    pub fn get_bytes_with(
        &self,
        path: &str,
        query: &[(&str, &str)],
        want: &[&str],
    ) -> Result<(Vec<u8>, Vec<Option<String>>)> {
        let mut req = self.authorized(self.agent.get(&format!("{}{path}", self.base)));
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

    /// `get` of a raw (non-JSON) body.
    pub fn get_bytes(&self, path: &str, query: &[(&str, &str)]) -> Result<Vec<u8>> {
        let mut req = self.authorized(self.agent.get(&format!("{}{path}", self.base)));
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

    /// Warn once on stderr when the daemon (`X-Av-Version`) is another build than this av.
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
                // Not every error is the daemon's JSON: axum answers a path or body it cannot
                // parse (`/v1/backlog/not-a-uuid/accept`) in plain text, which says what is wrong.
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
            Err(ureq::Error::Transport(t)) => Err(CliError::Unreachable {
                url: self.base.clone(),
                detail: t.to_string(),
            }),
        }
    }
}
