//! Workers pull jobs off the queue and retry the ones that fail.
use std::time::Duration;

use crate::queue::{Job, JobQueue};

pub const MAX_ATTEMPTS: u32 = 8;

/// Delay before the next attempt: doubles each time, from a base of 200 ms.
pub fn backoff_delay(attempts: u32) -> Duration {
    let millis = 200u64 << attempts;
    Duration::from_millis(millis)
}

pub fn handle_job(job: &Job) -> Result<(), String> {
    if job.payload.is_empty() {
        return Err("empty payload".into());
    }
    Ok(())
}

pub fn run_worker(queue: &mut JobQueue) -> usize {
    let mut done = 0;
    while let Some(mut job) = queue.pop() {
        match handle_job(&job) {
            Ok(()) => {
                queue.complete(job.id);
                done += 1;
            }
            Err(_) if job.attempts + 1 < MAX_ATTEMPTS => {
                job.attempts += 1;
                std::thread::sleep(backoff_delay(job.attempts));
                queue.push(job);
            }
            Err(_) => queue.complete(job.id),
        }
    }
    done
}
