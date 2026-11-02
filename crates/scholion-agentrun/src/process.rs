//! Child processes with a deadline.

use std::fs::File;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

/// How a child ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Waited {
    /// The exit code; `None` when it was killed (or ended by a signal).
    pub exit: Option<i32>,
    /// It ran past the deadline and was killed.
    pub killed: bool,
    /// It was killed because the caller raised the stop flag (`run_until`), before the deadline.
    pub stopped: bool,
}

/// Runs `cmd` in its own process group; after `timeout` the whole group is SIGKILLed.
pub fn run_with_timeout(cmd: Command, timeout: Duration) -> Result<Waited, String> {
    run_until(cmd, timeout, &AtomicBool::new(false))
}

/// [`run_with_timeout`], and the whole group is also SIGKILLed as soon as `stop` is true (looked
/// at every 100 ms). A caller watching the child's output, such as a budget watchdog, raises it
/// from another thread.
pub fn run_until(mut cmd: Command, timeout: Duration, stop: &AtomicBool) -> Result<Waited, String> {
    cmd.process_group(0);
    let mut child = cmd.spawn().map_err(|e| format!("spawn: {e}"))?;
    let start = Instant::now();
    loop {
        if let Some(status) = child.try_wait().map_err(|e| e.to_string())? {
            return Ok(Waited {
                exit: status.code(),
                killed: false,
                stopped: false,
            });
        }
        let stopped = stop.load(Ordering::SeqCst);
        if stopped || start.elapsed() >= timeout {
            let _ = Command::new("kill")
                .args(["-KILL", "--", &format!("-{}", child.id())])
                .status();
            let _ = child.kill();
            let _ = child.wait();
            return Ok(Waited {
                exit: None,
                killed: !stopped,
                stopped,
            });
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

/// `sh -c script` in `cwd` with no stdin and stdout and stderr both written to `log`.
pub fn shell(script: &str, cwd: &Path, log: &Path) -> Result<Command, String> {
    let out = File::create(log).map_err(|e| format!("{}: {e}", log.display()))?;
    let err = out.try_clone().map_err(|e| e.to_string())?;
    let mut cmd = Command::new("sh");
    cmd.arg("-c")
        .arg(script)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .stdout(out)
        .stderr(err);
    Ok(cmd)
}
