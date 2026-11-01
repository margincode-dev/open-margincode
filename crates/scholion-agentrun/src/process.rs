//! Child processes with a deadline.

use std::fs::File;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// How a child ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Waited {
    /// The exit code; `None` when it was killed (or ended by a signal).
    pub exit: Option<i32>,
    /// It ran past the deadline and was killed.
    pub killed: bool,
}

/// Runs `cmd` in its own process group; after `timeout` the whole group is SIGKILLed.
pub fn run_with_timeout(mut cmd: Command, timeout: Duration) -> Result<Waited, String> {
    cmd.process_group(0);
    let mut child = cmd.spawn().map_err(|e| format!("spawn: {e}"))?;
    let start = Instant::now();
    loop {
        if let Some(status) = child.try_wait().map_err(|e| e.to_string())? {
            return Ok(Waited {
                exit: status.code(),
                killed: false,
            });
        }
        if start.elapsed() >= timeout {
            let _ = Command::new("kill")
                .args(["-KILL", "--", &format!("-{}", child.id())])
                .status();
            let _ = child.kill();
            let _ = child.wait();
            return Ok(Waited {
                exit: None,
                killed: true,
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
