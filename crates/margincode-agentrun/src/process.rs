use std::fs::File;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Waited {
    pub exit: Option<i32>,
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
            kill_group(child.id());
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

unsafe extern "C" {
    fn kill(pid: i32, sig: i32) -> i32;
}

/// SIGKILLs process group `pgid` with the system call: a `kill` program would get the caller's
/// environment and be visible to the processes it kills. A group already gone is no error.
fn kill_group(pgid: u32) {
    const SIGKILL: i32 = 9;
    if let Ok(pgid) = i32::try_from(pgid) {
        // SAFETY: kill(2) takes plain integers and touches no memory of ours
        unsafe { kill(-pgid, SIGKILL) };
    }
}

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
