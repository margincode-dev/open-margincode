//! Where a record comes from, as the store knows it without a model: what became of the task that
//! wrote it. A decision from work that was reverted or abandoned is served, but ranked lower and
//! marked, so the next agent knows the claim's origin failed. Pure: `scholion-store` answers the
//! facts for a set of records in one query (`IntentStore::record_origins`) and `scholion-pack`
//! applies the factor and prints the label.
//!
//! Outcomes judge code, not notes. A merge shows that code passed review, not that a note about
//! it is true, so no origin raises a record: the best factor is 1.0.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{RecordKind, TaskStatus};

/// What became of the task a record belongs to, at the time of asking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Origin {
    /// Its task is merged.
    Merged,
    /// Its task is still under way: open, handed off, in review or conflicted.
    Open,
    /// Its task was abandoned.
    Abandoned,
    /// Its task's merge was reverted after the record was written, and it has not merged again.
    Reverted,
}

impl Origin {
    pub fn as_str(self) -> &'static str {
        match self {
            Origin::Merged => "merged",
            Origin::Open => "open",
            Origin::Abandoned => "abandoned",
            Origin::Reverted => "reverted",
        }
    }

    /// What a record's line says of its origin. Nothing for merged work, the usual case, which
    /// then costs no tokens.
    pub fn label(self) -> Option<&'static str> {
        match self {
            Origin::Merged => None,
            other => Some(other.as_str()),
        }
    }
}

/// What the store knows about a record's task, and the verdict in effect on the record, if one
/// is (`IntentStore::record_origins`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OriginFacts {
    pub status: TaskStatus,
    /// When the task's merge was last reverted, if it ever was.
    pub reverted_at: Option<DateTime<Utc>>,
    /// The newest verdict on the record that a second look confirmed and nothing retracted.
    pub verdict: Option<EffectiveVerdict>,
}

/// What a later look at a record (a dream, `av dream`) found, once a second run or a person
/// agreed. A verdict judges a record and never edits it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerdictKind {
    /// Checked against the code as it is now; still true.
    Confirmed,
    /// The code or a later outcome contradicts it. Shown, marked and ranked low: a disputed
    /// decision is a dead end worth a warning.
    Disputed,
    /// Says what another live record says; that one is shown.
    Duplicate,
    /// Was true; the code it describes is gone or replaced.
    Outdated,
}

impl VerdictKind {
    pub fn as_str(self) -> &'static str {
        match self {
            VerdictKind::Confirmed => "confirmed",
            VerdictKind::Disputed => "disputed",
            VerdictKind::Duplicate => "duplicate",
            VerdictKind::Outdated => "outdated",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "confirmed" => VerdictKind::Confirmed,
            "disputed" => VerdictKind::Disputed,
            "duplicate" => VerdictKind::Duplicate,
            "outdated" => VerdictKind::Outdated,
            _ => return None,
        })
    }

    /// Whether packs and the pre-read hook leave the record out (search still finds it).
    pub fn hides(self) -> bool {
        matches!(self, VerdictKind::Duplicate | VerdictKind::Outdated)
    }
}

/// A verdict in effect on a record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EffectiveVerdict {
    pub id: uuid::Uuid,
    pub kind: VerdictKind,
    /// Why, in at most 200 characters (always set for `disputed`).
    pub reason: String,
    /// For `confirmed`: per anchor of the record, the hash of the anchored code the verdict
    /// saw. "Changed since recorded" is then reckoned from these, not from the record's own.
    #[serde(skip)]
    pub anchor_hashes: Vec<Option<String>>,
}

impl EffectiveVerdict {
    /// `vd:<8 hex>`, as a record's line names it.
    pub fn handle(&self) -> String {
        format!("vd:{}", &self.id.simple().to_string()[..8])
    }
}

/// A record's origin and the multiplier on its score in packs.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Standing {
    pub origin: Origin,
    pub factor: f32,
}

/// Factor of a claim (not a dead end) whose task's merge was reverted.
pub const REVERTED_FACTOR: f32 = 0.4;
/// Factor of a claim whose task was abandoned.
pub const ABANDONED_FACTOR: f32 = 0.6;
/// Factor of a record under a `disputed` verdict, on top of its origin's.
pub const DISPUTED_FACTOR: f32 = 0.3;

/// The standing of a record of kind `kind`, written at `written`, whose task is as `facts` say.
///
/// - merged: 1.0.
/// - reverted (a revert after the record was written, and the task is not merged now): 0.4. A
///   record written after the revert belongs to the work that follows it and is `open`.
/// - abandoned: 0.6.
/// - open: 1.0. Work under way is marked and not ranked lower: a factor on open decisions
///   (0.8 was tried) took the lead from records that match the reader's goal, which is measured
///   behaviour, and nothing measured speaks for it.
///
/// An `attempt` keeps 1.0 whatever became of its task: a dead end is still one when the work
/// around it failed, and is often why it failed.
pub fn standing(facts: &OriginFacts, kind: RecordKind, written: DateTime<Utc>) -> Standing {
    let origin = match facts.status {
        TaskStatus::Merged => Origin::Merged,
        TaskStatus::Abandoned => Origin::Abandoned,
        _ if facts.reverted_at.is_some_and(|at| at > written) => Origin::Reverted,
        _ => Origin::Open,
    };
    let mut factor = match (origin, kind) {
        (Origin::Merged | Origin::Open, _) | (_, RecordKind::Attempt) => 1.0,
        (Origin::Reverted, _) => REVERTED_FACTOR,
        (Origin::Abandoned, _) => ABANDONED_FACTOR,
    };
    // a disputed record of any kind: an attempt someone showed to be wrong is not a dead end
    if facts
        .verdict
        .as_ref()
        .is_some_and(|v| v.kind == VerdictKind::Disputed)
    {
        factor *= DISPUTED_FACTOR;
    }
    Standing { origin, factor }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    const KINDS: [RecordKind; 7] = [
        RecordKind::Intent,
        RecordKind::Decision,
        RecordKind::Attempt,
        RecordKind::Note,
        RecordKind::Handoff,
        RecordKind::Review,
        RecordKind::Validation,
    ];
    const STATUSES: [TaskStatus; 6] = [
        TaskStatus::Open,
        TaskStatus::HandedOff,
        TaskStatus::InReview,
        TaskStatus::Conflicted,
        TaskStatus::Merged,
        TaskStatus::Abandoned,
    ];

    fn at(status: TaskStatus, reverted_after: Option<i64>, kind: RecordKind) -> Standing {
        let written = Utc::now();
        let facts = OriginFacts {
            status,
            reverted_at: reverted_after.map(|s| written + Duration::seconds(s)),
            verdict: None,
        };
        standing(&facts, kind, written)
    }

    #[test]
    fn merged_work_is_the_baseline_and_nothing_ranks_above_it() {
        for kind in KINDS {
            let s = at(TaskStatus::Merged, None, kind);
            assert_eq!((s.origin, s.factor), (Origin::Merged, 1.0), "{kind:?}");
            // a revert that a later merge followed does not count
            let s = at(TaskStatus::Merged, Some(60), kind);
            assert_eq!((s.origin, s.factor), (Origin::Merged, 1.0), "{kind:?}");
        }
        for status in STATUSES {
            for kind in KINDS {
                for rev in [None, Some(-60), Some(60)] {
                    assert!(at(status, rev, kind).factor <= 1.0, "{status:?} {kind:?}");
                }
            }
        }
    }

    #[test]
    fn a_record_written_before_its_tasks_revert_is_reverted() {
        let s = at(TaskStatus::Open, Some(60), RecordKind::Decision);
        assert_eq!((s.origin, s.factor), (Origin::Reverted, REVERTED_FACTOR));
        let s = at(TaskStatus::InReview, Some(60), RecordKind::Note);
        assert_eq!((s.origin, s.factor), (Origin::Reverted, REVERTED_FACTOR));
    }

    #[test]
    fn a_record_written_after_the_revert_is_open() {
        let s = at(TaskStatus::Open, Some(-60), RecordKind::Decision);
        assert_eq!((s.origin, s.factor), (Origin::Open, 1.0));
    }

    #[test]
    fn abandoned_work_ranks_its_claims_lower() {
        for kind in [RecordKind::Decision, RecordKind::Note, RecordKind::Intent] {
            let s = at(TaskStatus::Abandoned, None, kind);
            assert_eq!((s.origin, s.factor), (Origin::Abandoned, ABANDONED_FACTOR));
        }
        // abandoned wins over an earlier revert: it is the task's last word
        assert_eq!(
            at(TaskStatus::Abandoned, Some(60), RecordKind::Decision).origin,
            Origin::Abandoned
        );
    }

    #[test]
    fn a_dead_end_keeps_its_rank_whatever_became_of_its_task() {
        for (status, rev) in [
            (TaskStatus::Abandoned, None),
            (TaskStatus::Open, Some(60)),
            (TaskStatus::Open, None),
            (TaskStatus::HandedOff, None),
        ] {
            assert_eq!(
                at(status, rev, RecordKind::Attempt).factor,
                1.0,
                "{status:?}"
            );
        }
    }

    #[test]
    fn work_under_way_is_open_and_keeps_its_rank() {
        for status in [
            TaskStatus::Open,
            TaskStatus::HandedOff,
            TaskStatus::InReview,
            TaskStatus::Conflicted,
        ] {
            let s = at(status, None, RecordKind::Decision);
            assert_eq!((s.origin, s.factor), (Origin::Open, 1.0));
        }
    }

    #[test]
    fn a_disputed_record_ranks_low_whatever_its_origin_and_kind() {
        let written = Utc::now();
        let verdict = |kind| EffectiveVerdict {
            id: uuid::Uuid::nil(),
            kind,
            reason: String::new(),
            anchor_hashes: vec![],
        };
        let with = |status, kind, record| {
            let facts = OriginFacts {
                status,
                reverted_at: None,
                verdict: Some(verdict(kind)),
            };
            standing(&facts, record, written).factor
        };
        assert_eq!(
            with(
                TaskStatus::Merged,
                VerdictKind::Disputed,
                RecordKind::Decision
            ),
            DISPUTED_FACTOR
        );
        assert_eq!(
            with(
                TaskStatus::Merged,
                VerdictKind::Disputed,
                RecordKind::Attempt
            ),
            DISPUTED_FACTOR
        );
        assert_eq!(
            with(
                TaskStatus::Abandoned,
                VerdictKind::Disputed,
                RecordKind::Note
            ),
            ABANDONED_FACTOR * DISPUTED_FACTOR
        );
        // the other verdicts do not rank: they hide, or confirm
        for kind in [
            VerdictKind::Confirmed,
            VerdictKind::Duplicate,
            VerdictKind::Outdated,
        ] {
            assert_eq!(with(TaskStatus::Merged, kind, RecordKind::Decision), 1.0);
        }
        assert!(VerdictKind::Duplicate.hides() && VerdictKind::Outdated.hides());
        assert!(!VerdictKind::Disputed.hides() && !VerdictKind::Confirmed.hides());
    }

    #[test]
    fn only_merged_work_goes_unlabelled() {
        assert_eq!(Origin::Merged.label(), None);
        assert_eq!(Origin::Open.label(), Some("open"));
        assert_eq!(Origin::Abandoned.label(), Some("abandoned"));
        assert_eq!(Origin::Reverted.label(), Some("reverted"));
    }
}
