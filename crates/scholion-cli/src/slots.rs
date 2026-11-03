//! Host-wide slots: at most N of something at once on this machine, across every `av` process
//! that shares `$AV_HOME`. A slot is a file `<dir>/slot-<i>.lock` held with an OS file lock;
//! the OS drops a lock when its process dies, so a killed process never leaks one. `av watch
//! --max-agents` counts running agents in `$AV_HOME/agents`, `av validate --max-builds` running
//! validations in `$AV_HOME/builds`.

use std::fs::{File, OpenOptions, TryLockError};
use std::path::Path;
use std::time::Duration;

use crate::client::{CliError, Result};

/// Takes a free one of `n` slots in `dir`: the open, locked slot file, held until dropped.
/// `None` when all `n` are held.
pub fn take(dir: &Path, n: u32) -> std::io::Result<Option<File>> {
    std::fs::create_dir_all(dir)?;
    for i in 0..n {
        let f = OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(dir.join(format!("slot-{i}.lock")))?;
        match f.try_lock() {
            Ok(()) => return Ok(Some(f)),
            Err(TryLockError::WouldBlock) => {}
            Err(TryLockError::Error(e)) => return Err(e),
        }
    }
    Ok(None)
}

/// Waits until one of `n` slots in `dir` is free and takes it, saying once on stderr that it
/// waits (`what`: "build" for "all 2 build slots on this host are in use; waiting").
pub fn wait_for(dir: &Path, n: u32, what: &str, poll: Duration) -> Result<File> {
    let mut said = false;
    loop {
        match take(dir, n) {
            Ok(Some(slot)) => return Ok(slot),
            Ok(None) => {
                if !said {
                    eprintln!("all {n} {what} slots on this host are in use; waiting");
                    said = true;
                }
                std::thread::sleep(poll);
            }
            Err(e) => {
                return Err(CliError::Local(format!(
                    "{what} slots in {}: {e}",
                    dir.display()
                )));
            }
        }
    }
}
