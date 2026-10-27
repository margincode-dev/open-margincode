use scholion_core::{AvError, NewRecord, RecordKind, TaskId};

fn rec(summary: String) -> NewRecord {
    NewRecord {
        task_id: TaskId(uuid::Uuid::nil()),
        kind: RecordKind::Decision,
        summary,
        body: String::new(),
        change_id: None,
        supersedes: None,
        author_agent: "t".into(),
        anchors: vec![],
    }
}

#[test]
fn accented_and_cjk_summaries_are_measured_in_characters() {
    assert!(rec("é".repeat(200)).validate().is_ok());
    assert!(rec("決".repeat(200)).validate().is_ok());
    assert!(matches!(rec("é".repeat(201)).validate(), Err(AvError::SummaryTooLong(201))));
}

#[test]
fn ascii_limit_unchanged() {
    assert!(rec("x".repeat(200)).validate().is_ok());
    assert!(rec("x".repeat(201)).validate().is_err());
}
