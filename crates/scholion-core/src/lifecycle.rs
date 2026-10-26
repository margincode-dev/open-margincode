//! Task lifecycle: the only legal status transitions (Plan 3).

use serde::{Deserialize, Serialize};

use crate::error::{AvError, Result};
use crate::types::TaskStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Event {
    Handoff,
    Claim,
    Submit,
    RequestChanges,
    Approve,
    MergeOk,
    MergeConflict,
    Resolve,
    Undo,
    /// A merged task's change was reverted on main; the task goes back to its owner.
    Revert,
    Abandon,
}

impl Event {
    /// The event feed's name for this transition.
    pub fn feed_kind(self) -> &'static str {
        match self {
            Event::Handoff => "handoff",
            Event::Claim => "claim",
            Event::Submit => "submit",
            Event::RequestChanges => "request_changes",
            Event::Approve => "approve",
            Event::MergeOk => "merged",
            Event::MergeConflict => "conflicted",
            Event::Resolve => "resolve",
            Event::Undo => "undo",
            Event::Revert => "revert",
            Event::Abandon => "abandon",
        }
    }
}

/// The status after `event`, or `IllegalState` when the transition is not in the table.
pub fn next(status: TaskStatus, event: Event) -> Result<TaskStatus> {
    use Event as E;
    use TaskStatus as S;
    match (status, event) {
        (S::Open, E::Handoff) => Ok(S::HandedOff),
        (S::HandedOff, E::Claim) => Ok(S::Open),
        (S::Open, E::Submit) => Ok(S::InReview),
        (S::InReview, E::RequestChanges) => Ok(S::Open),
        (S::Merged, E::Revert) => Ok(S::Open),
        // an approval is recorded separately; the task stays in review until it merges
        (S::InReview, E::Approve) => Ok(S::InReview),
        (S::InReview, E::MergeOk) => Ok(S::Merged),
        (S::InReview, E::MergeConflict) => Ok(S::Conflicted),
        (S::Conflicted, E::Resolve) => Ok(S::Open),
        (S::Merged, E::Undo) => Ok(S::InReview),
        (S::Open | S::HandedOff | S::InReview | S::Conflicted, E::Abandon) => Ok(S::Abandoned),
        (from, ev) => Err(AvError::IllegalState(format!(
            "a task that is {} cannot take {ev:?}",
            from.as_str()
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use TaskStatus::*;

    const ALL_STATUSES: [TaskStatus; 6] =
        [Open, HandedOff, InReview, Conflicted, Merged, Abandoned];
    const ALL_EVENTS: [Event; 10] = [
        Event::Handoff,
        Event::Claim,
        Event::Submit,
        Event::RequestChanges,
        Event::Approve,
        Event::MergeOk,
        Event::MergeConflict,
        Event::Resolve,
        Event::Undo,
        Event::Abandon,
    ];

    #[test]
    fn legal_transitions_follow_the_table() {
        let table = [
            (Open, Event::Handoff, HandedOff),
            (HandedOff, Event::Claim, Open),
            (Open, Event::Submit, InReview),
            (InReview, Event::RequestChanges, Open),
            (InReview, Event::Approve, InReview),
            (InReview, Event::MergeOk, Merged),
            (InReview, Event::MergeConflict, Conflicted),
            (Conflicted, Event::Resolve, Open),
            (Merged, Event::Undo, InReview),
            (Open, Event::Abandon, Abandoned),
            (HandedOff, Event::Abandon, Abandoned),
            (InReview, Event::Abandon, Abandoned),
            (Conflicted, Event::Abandon, Abandoned),
        ];
        for (from, ev, to) in table {
            assert_eq!(next(from, ev).unwrap(), to, "{from:?} --{ev:?}-->");
        }
        // and nothing else is legal
        let legal = ALL_STATUSES
            .iter()
            .flat_map(|s| ALL_EVENTS.iter().map(move |e| (*s, *e)))
            .filter(|(s, e)| next(*s, *e).is_ok())
            .count();
        assert_eq!(legal, table.len());
    }

    #[test]
    fn abandoned_is_terminal_and_merged_allows_only_undo() {
        for ev in ALL_EVENTS {
            assert!(next(Abandoned, ev).is_err(), "{ev:?}");
            assert_eq!(next(Merged, ev).is_ok(), ev == Event::Undo, "{ev:?}");
        }
    }

    #[test]
    fn illegal_pair_error_names_both_sides() {
        match next(Open, Event::MergeOk) {
            Err(AvError::IllegalState(msg)) => {
                assert!(msg.contains("open") && msg.contains("MergeOk"), "{msg}")
            }
            other => panic!("{other:?}"),
        }
    }
}
