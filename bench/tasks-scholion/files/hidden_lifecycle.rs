use scholion_core::lifecycle::{Event, next};
use scholion_core::TaskStatus;

#[test]
fn an_undone_merge_goes_back_to_review() {
    assert_eq!(next(TaskStatus::Merged, Event::Undo).unwrap(), TaskStatus::InReview);
}

#[test]
fn merged_still_refuses_other_events() {
    for ev in [Event::Submit, Event::Approve, Event::MergeOk, Event::Abandon, Event::Claim] {
        assert!(next(TaskStatus::Merged, ev).is_err(), "{ev:?}");
    }
    assert_eq!(next(TaskStatus::Merged, Event::Revert).unwrap(), TaskStatus::Open);
}
