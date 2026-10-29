//! The job queue: ordering, leasing and requeueing of stale jobs.
use std::collections::VecDeque;
use std::time::{Duration, Instant};

pub struct Job {
    pub id: u64,
    pub payload: String,
    pub attempts: u32,
    pub leased_at: Option<Instant>,
}

#[derive(Default)]
pub struct JobQueue {
    ready: VecDeque<Job>,
    leased: Vec<Job>,
}

impl JobQueue {
    pub fn push(&mut self, job: Job) {
        self.ready.push_back(job);
    }

    pub fn pop(&mut self) -> Option<Job> {
        let mut job = self.ready.pop_front()?;
        job.leased_at = Some(Instant::now());
        self.leased.push(Job {
            id: job.id,
            payload: job.payload.clone(),
            attempts: job.attempts,
            leased_at: job.leased_at,
        });
        Some(job)
    }

    pub fn complete(&mut self, id: u64) {
        self.leased.retain(|j| j.id != id);
    }

    /// Puts jobs whose lease ran out back at the front of the queue.
    pub fn requeue_stale(&mut self, lease: Duration) -> usize {
        let now = Instant::now();
        let (stale, live): (Vec<Job>, Vec<Job>) = self
            .leased
            .drain(..)
            .partition(|j| j.leased_at.is_some_and(|t| now - t > lease));
        self.leased = live;
        let n = stale.len();
        for mut job in stale {
            job.leased_at = None;
            self.ready.push_front(job);
        }
        n
    }

    pub fn len(&self) -> usize {
        self.ready.len() + self.leased.len()
    }
}
