//! Launching an agent CLI: the builder and what comes back.

use std::ffi::OsString;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::{Duration, Instant};

use crate::events::{ParsedSession, Usage, parse_stream};
use crate::process::run_until;

/// One environment change, applied in the order it was asked for.
enum EnvOp {
    Set(OsString, OsString),
    Remove(OsString),
    Clear,
}

/// An agent CLI to run: program, arguments, working directory, environment, a deadline, and
/// where its stdout (the event stream) and stderr go. Knows nothing about which CLI it is; the
/// caller decides the arguments and the variables. `max_budget_usd` is the one flag it adds
/// itself (`--max-budget-usd`, appended last), because every caller wants it the same way.
pub struct AgentCommand {
    program: OsString,
    args: Vec<OsString>,
    cwd: Option<PathBuf>,
    env: Vec<EnvOp>,
    path_prepend: Vec<PathBuf>,
    max_budget_usd: Option<f64>,
    timeout: Duration,
    events_path: PathBuf,
    stderr_path: Option<PathBuf>,
    stop: Option<Arc<AtomicBool>>,
}

/// What a finished (or killed) run left behind.
#[derive(Debug, Clone)]
pub struct RunResult {
    /// The exit code; `None` when it was killed (or ended by a signal).
    pub exit: Option<i32>,
    /// It ran past the deadline and its process group was killed.
    pub killed: bool,
    /// The stop flag was raised and its process group was killed, before the deadline.
    pub stopped: bool,
    /// Wall time from spawn to exit or kill.
    pub wall: Duration,
    /// The file the event stream was captured to.
    pub events_path: PathBuf,
    /// The session total, by `ParsedSession::total_usage`: the `result` event's usage when it has
    /// any, else the sum over the assistant messages (a killed stream).
    pub usage: Usage,
    /// The stream, parsed: `truncated()` when no `result` event arrived.
    pub session: ParsedSession,
}

impl AgentCommand {
    pub fn new(program: impl Into<OsString>) -> Self {
        Self {
            program: program.into(),
            args: vec![],
            cwd: None,
            env: vec![],
            path_prepend: vec![],
            max_budget_usd: None,
            timeout: Duration::from_secs(3600),
            events_path: PathBuf::from("events.jsonl"),
            stderr_path: None,
            stop: None,
        }
    }

    pub fn arg(mut self, arg: impl Into<OsString>) -> Self {
        self.args.push(arg.into());
        self
    }

    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }

    pub fn cwd(mut self, dir: impl AsRef<Path>) -> Self {
        self.cwd = Some(dir.as_ref().to_path_buf());
        self
    }

    pub fn env(mut self, key: impl Into<OsString>, value: impl Into<OsString>) -> Self {
        self.env.push(EnvOp::Set(key.into(), value.into()));
        self
    }

    /// Hides `key` from the child, even when this process has it set.
    pub fn env_remove(mut self, key: impl Into<OsString>) -> Self {
        self.env.push(EnvOp::Remove(key.into()));
        self
    }

    /// Drops every variable the child would inherit from this process; changes asked for after
    /// it still apply. A runner uses it so no credential of the host reaches the agent. `PATH` is
    /// gone too, so name the program by its path or set `PATH` again.
    pub fn env_clear(mut self) -> Self {
        self.env.push(EnvOp::Clear);
        self
    }

    /// Puts `dirs` in front of the child's `PATH`: the inherited one, or the one set with `env`
    /// (none after `env_clear`).
    pub fn path_prepend(mut self, dirs: Vec<PathBuf>) -> Self {
        self.path_prepend = dirs;
        self
    }

    /// `Some(cap)` appends `--max-budget-usd cap` (two decimals) after every other argument.
    pub fn max_budget_usd(mut self, cap: Option<f64>) -> Self {
        self.max_budget_usd = cap;
        self
    }

    /// After this long the child's whole process group is killed.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Where stdout, the event stream, is written (created or truncated).
    pub fn events_path(mut self, path: impl AsRef<Path>) -> Self {
        self.events_path = path.as_ref().to_path_buf();
        self
    }

    /// Kills the child's process group as soon as `flag` is true (`RunResult::stopped`). Another
    /// thread raises it, such as a watchdog reading the events file as it grows.
    pub fn stop_flag(mut self, flag: Arc<AtomicBool>) -> Self {
        self.stop = Some(flag);
        self
    }

    /// Where stderr is written; without it stderr is discarded.
    pub fn stderr_path(mut self, path: impl AsRef<Path>) -> Self {
        self.stderr_path = Some(path.as_ref().to_path_buf());
        self
    }

    /// Runs the command to its end or its deadline, then reads the event stream it wrote.
    pub fn run(self) -> Result<RunResult, String> {
        let create = |p: &Path| File::create(p).map_err(|e| format!("{}: {e}", p.display()));
        let mut cmd = Command::new(&self.program);
        cmd.args(&self.args).stdin(Stdio::null());
        cmd.stdout(create(&self.events_path)?);
        match &self.stderr_path {
            Some(p) => cmd.stderr(create(p)?),
            None => cmd.stderr(Stdio::null()),
        };
        if let Some(dir) = &self.cwd {
            cmd.current_dir(dir);
        }
        // the PATH the child gets, which path_prepend builds on
        let mut path = std::env::var_os("PATH");
        for op in &self.env {
            match op {
                EnvOp::Set(k, v) => {
                    if k == "PATH" {
                        path = Some(v.clone());
                    }
                    cmd.env(k, v)
                }
                EnvOp::Remove(k) => {
                    if k == "PATH" {
                        path = None;
                    }
                    cmd.env_remove(k)
                }
                EnvOp::Clear => {
                    path = None;
                    cmd.env_clear()
                }
            };
        }
        if !self.path_prepend.is_empty() {
            // no PATH means nothing after the prepended dirs (an empty entry would be the cwd)
            let inherited: Vec<PathBuf> = path
                .map(|p| std::env::split_paths(&p).collect())
                .unwrap_or_default();
            let dirs = self.path_prepend.iter().cloned().chain(inherited);
            cmd.env(
                "PATH",
                std::env::join_paths(dirs).map_err(|e| format!("path_prepend: {e}"))?,
            );
        }
        if let Some(cap) = self.max_budget_usd {
            cmd.args(["--max-budget-usd", &format!("{cap:.2}")]);
        }
        let started = Instant::now();
        let never = AtomicBool::new(false);
        let stop = self.stop.as_deref().unwrap_or(&never);
        let waited = run_until(cmd, self.timeout, stop)?;
        let wall = started.elapsed();
        let text = std::fs::read(&self.events_path)
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .map_err(|e| format!("{}: {e}", self.events_path.display()))?;
        let session = parse_stream(&text);
        let usage = session.total_usage();
        Ok(RunResult {
            exit: waited.exit,
            killed: waited.killed,
            stopped: waited.stopped,
            wall,
            events_path: self.events_path,
            usage,
            session,
        })
    }
}
