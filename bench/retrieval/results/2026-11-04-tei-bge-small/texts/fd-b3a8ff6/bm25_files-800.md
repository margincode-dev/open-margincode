### src/walk.rs
```rust
use std::borrow::Cow;
use std::ffi::OsStr;
use std::io::{self, Write};
use std::mem;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{Result, anyhow};
use crossbeam_channel::{Receiver, RecvTimeoutError, SendError, Sender, bounded};
use etcetera::BaseStrategy;
use ignore::overrides::{Override, OverrideBuilder};
use ignore::{WalkBuilder, WalkParallel, WalkState};
use regex::bytes::Regex;

use crate::config::Config;
use crate::dir_entry::DirEntry;
use crate::error::print_error;
use crate::exec;
use crate::exit_codes::{ExitCode, merge_exitcodes};
use crate::filesystem;
use crate::output;

/// The receiver thread can either be buffering results or directly streaming to the console.
#[derive(PartialEq)]
enum ReceiverMode {
    /// Receiver is still buffering in order to sort the results, if the search finishes fast
    /// enough.
    Buffering,

    /// Receiver is directly printing results to the output.
    Streaming,
}

/// The Worker threads can result in a valid entry having PathBuf or an error.
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum WorkerResult {
    // Errors should be rare, so it's probably better to allow large_enum_variant than
    // to box the Entry variant
    Entry(DirEntry),
    Error(ignore::Error),
}

/// A batch of WorkerResults to send over a channel.
#[derive(Clone)]
struct Batch {
    items: Arc<Mutex<Option<Vec<WorkerResult>>>>,
}

impl Batch {
    fn new() -> Self {
        Self {
            items: Arc::new(Mutex::new(Some(vec![]))),
        }
    }

    fn lock(&self) -> MutexGuard<'_, Option<Vec<WorkerResult>>> {
        self.items.lock().unwrap()
    }
}

impl IntoIterator for Batch {
    type Item = WorkerResult;
    type IntoIter = std::vec::IntoIter<WorkerResult>;

    fn into_iter(self) -> Self::IntoIter {
        self.lock().take().unwrap().into_iter()
    }
}

/// Wrapper that sends batches of items at once over a channel.
struct BatchSender {
    batch: Batch,
    tx: Sender<Batch>,
    limit: usize,
}

impl BatchSender {
    fn new(tx: Sender<Batch>, limit: usize) -> Self {
        Self {
            batch: Batch::new(),
            tx,
            limit,
        }
    }

    /// Check if we need to flush a batch.
    fn needs_flush(&self, batch: Option<&Vec<WorkerResult>>) -> bool {
        match batch {
            // Limit the batch size to provide some backpressure
            Some(vec) => vec.len() >= self.limit,
            // Batch was already taken by the receiver, so make a new one
            None => true,
        }
    }

    /// Add an item to a batch.
    fn send(&mut self, item: WorkerResult) -> Result<(), SendError<()>> {
        let mut batch = self.batch.lock();

        if self.needs_flush(batch.as_ref()) {
            drop(batch);
            self.batch = Batch::new();
            batch = self.batch.lock();
        }

        let items = batch.as_mut().unwrap();
        items.push(item);

        if items.len() == 1 {
            // New batch, send it over the channel
            self.tx
                .send(self.batch.clone())
                .map_err(|_| SendError(()))?;
        }

        Ok(())
    }
}

/// Maximum size of the output buffer before flushing results to the console
