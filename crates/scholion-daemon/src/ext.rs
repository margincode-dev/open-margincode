//! Extension points for code built on top of the daemon: extra routes (`router_with`), typed
//! services kept in `AppState` (`with_extension`), the event feed in process
//! (`subscribe_events`), and a guard the lifecycle asks before an approval or a merge
//! (`MergeGuard`). The daemon alone uses none of them: no extensions, and a guard that allows
//! everything the lifecycle rules allow.

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use scholion_core::{AvEvent, IntentStore, Task};
use tokio::sync::broadcast;

/// Services of an extension, one per type.
#[derive(Clone, Default)]
pub(crate) struct Extensions(Arc<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>);

impl Extensions {
    pub(crate) fn insert<T: Send + Sync + 'static>(&mut self, value: Arc<T>) {
        Arc::make_mut(&mut self.0).insert(TypeId::of::<T>(), value);
    }

    pub(crate) fn get<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        self.0.get(&TypeId::of::<T>())?.clone().downcast().ok()
    }
}

/// What a `MergeGuard` is asked about. The lifecycle rules (tests at head, a reviewer other than
/// the owner, an approval at head) were already checked when it is asked.
#[derive(Debug, Clone, Copy)]
pub enum Guarded<'a> {
    /// `reviewer` approves `task` at its head.
    Approve { task: &'a Task, reviewer: &'a str },
    /// `agent` merges the approved `task` into main.
    Merge { task: &'a Task, agent: &'a str },
}

/// Asked before an approval and before a merge. `Err(reason)` refuses the request with HTTP 422
/// and the reason; the task stays as it was.
#[async_trait]
pub trait MergeGuard: Send + Sync {
    async fn check(&self, action: Guarded<'_>) -> Result<(), String>;
}

/// The daemon's own guard: the lifecycle rules are all there is.
pub struct AllowAll;

#[async_trait]
impl MergeGuard for AllowAll {
    async fn check(&self, _: Guarded<'_>) -> Result<(), String> {
        Ok(())
    }
}

/// How many events a slow subscriber may fall behind before it misses some
/// (`RecvError::Lagged`).
const FEED_CAPACITY: usize = 1024;
/// How often the feed reads new events. Same as the `/v1/events` long poll.
const FEED_POLL: std::time::Duration = std::time::Duration::from_millis(200);
const FEED_BATCH: usize = 500;

/// The event feed in process: one reader of the events table for all subscribers, started by
/// the first subscriber and stopped when the last one is gone.
#[derive(Clone, Default)]
pub(crate) struct EventFeed(Arc<Mutex<Option<broadcast::Sender<AvEvent>>>>);

impl EventFeed {
    /// Every event committed after this call, of every repo, oldest first.
    pub(crate) async fn subscribe(
        &self,
        store: Arc<dyn IntentStore>,
    ) -> scholion_core::Result<broadcast::Receiver<AvEvent>> {
        // read before taking the lock: a feed started now begins after this id
        let since = store.last_event_id(None).await?;
        let mut slot = self.0.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(tx) = slot.as_ref() {
            return Ok(tx.subscribe());
        }
        let (tx, rx) = broadcast::channel(FEED_CAPACITY);
        *slot = Some(tx.clone());
        tokio::spawn(feed(self.0.clone(), tx, store, since));
        Ok(rx)
    }
}

async fn feed(
    slot: Arc<Mutex<Option<broadcast::Sender<AvEvent>>>>,
    tx: broadcast::Sender<AvEvent>,
    store: Arc<dyn IntentStore>,
    mut since: i64,
) {
    loop {
        {
            // checked under the lock `subscribe` takes, so no subscriber joins a stopping feed
            let mut slot = slot.lock().unwrap_or_else(|e| e.into_inner());
            if tx.receiver_count() == 0 {
                *slot = None;
                return;
            }
        }
        match store.events_since(None, since, FEED_BATCH).await {
            Ok(events) => {
                let full = events.len() == FEED_BATCH;
                for ev in events {
                    since = ev.id;
                    let _ = tx.send(ev);
                }
                if full {
                    continue; // more are waiting
                }
            }
            Err(e) => tracing::warn!(error = %e, "event feed read failed"),
        }
        tokio::time::sleep(FEED_POLL).await;
    }
}
