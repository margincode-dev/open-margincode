use std::fmt;
use std::time::Duration;

use serde_json::Value;

#[derive(Debug)]
pub enum CliError {
    Unreachable { url: String, detail: String },
    Api { status: u16, message: String },
    NoTask,
    Local(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::Unreachable { url, detail } => write!(
                f,
                "cannot reach the Scholion daemon at {url} ({detail}). Start one with `av up` (or `docker compose up -d`), or set AV_URL to point elsewhere."
            ),
            CliError::Api { status, message } => write!(f, "{message} (HTTP {status})"),
            CliError::NoTask => write!(
                f,
                "no task here: run `av task start --repo R --title T --goal G`, or pass --task <id> / set AV_TASK"
            ),
            CliError::Local(m) => write!(f, "{m}"),
        }
    }
}

pub type Result<T> = std::result::Result<T, CliError>;

pub struct Client {
    base: String,
    agent: ureq::Agent,
    /// `Authorization: Bearer …` on every request when set.
    token: Option<String>,
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

    fn finish(
        &self,
        _path: &str,
        res: std::result::Result<ureq::Response, ureq::Error>,
    ) -> Result<Value> {
        match res {
            Ok(resp) => resp
                .into_json::<Value>()
                .map_err(|e| CliError::Local(format!("bad response from daemon: {e}"))),
            Err(ureq::Error::Status(status, resp)) => {
                let message = resp
                    .into_json::<Value>()
                    .ok()
                    .and_then(|v| v["error"].as_str().map(str::to_owned))
                    .unwrap_or_else(|| "request failed".to_owned());
                Err(CliError::Api { status, message })
            }
            Err(ureq::Error::Transport(t)) => Err(CliError::Unreachable {
                url: self.base.clone(),
                detail: t.to_string(),
            }),
        }
    }
}
