use scholion_core::*;
use scholion_store::PgStore;
use scholion_testutil::TestDb;
use sqlx::Row;

async fn store() -> (TestDb, PgStore) {
    let db = TestDb::new().await;
    let store = PgStore::connect(&db.url).await.unwrap();
    (db, store)
}

fn task(repo: &str, title: &str) -> NewTask {
    NewTask {
        repo: repo.into(),
        title: title.into(),
        goal: "goal".into(),
        owner_agent: "claude".into(),
    }
}

fn record(task: TaskId, summary: &str, body: &str, supersedes: Option<RecordId>) -> NewRecord {
    NewRecord {
        task_id: task,
        kind: RecordKind::Note,
        summary: summary.into(),
        body: body.into(),
        change_id: None,
        supersedes,
        author_agent: "claude".into(),
        anchors: vec![],
    }
}

async fn install_boom(s: &PgStore) {
    sqlx::query(
        "CREATE FUNCTION av_test_outbox_boom() RETURNS trigger LANGUAGE plpgsql AS $$
         BEGIN RAISE EXCEPTION 'boom'; END; $$",
    )
    .execute(s.pool())
    .await
    .unwrap();
    sqlx::query(
        "CREATE TRIGGER av_test_outbox_boom BEFORE INSERT ON outbox
         FOR EACH ROW EXECUTE FUNCTION av_test_outbox_boom()",
    )
    .execute(s.pool())
    .await
    .unwrap();
}

async fn drop_boom(s: &PgStore) {
    sqlx::query("DROP TRIGGER av_test_outbox_boom ON outbox")
        .execute(s.pool())
        .await
        .unwrap();
    sqlx::query("DROP FUNCTION av_test_outbox_boom()")
        .execute(s.pool())
        .await
        .unwrap();
}

async fn count(s: &PgStore, table: &str) -> i64 {
    let sql = format!("SELECT count(*) FROM {table}");
    sqlx::query_scalar(&sql).fetch_one(s.pool()).await.unwrap()
}

#[tokio::test]
async fn record_and_outbox_commit_or_roll_back_together() {
    let (_db, s) = store().await;
    let t = s.create_task(task("demo", "Outbox record")).await.unwrap();
    install_boom(&s).await;

    let err = s
        .add_record(record(t.id, "sum", "body", None))
        .await
        .unwrap_err();
    assert!(matches!(err, AvError::Store(_)), "{err}");
    assert_eq!(count(&s, "records").await, 0);
    assert_eq!(count(&s, "outbox").await, 0);

    drop_boom(&s).await;
    s.add_record(record(t.id, "sum", "body", None))
        .await
        .unwrap();
    assert_eq!(count(&s, "records").await, 1);
    assert_eq!(count(&s, "outbox").await, 1);
    let row = sqlx::query("SELECT kind, op, text, superseded FROM outbox")
        .fetch_one(s.pool())
        .await
        .unwrap();
    let kind: String = row.get("kind");
    let op: String = row.get("op");
    let text: String = row.get("text");
    let superseded: bool = row.get("superseded");
    assert_eq!(kind, "record");
    assert_eq!(op, "upsert");
    assert_eq!(text, "sum\nbody");
    assert!(!superseded);
}

#[tokio::test]
async fn supersede_enqueues_upsert_and_supersede() {
    let (_db, s) = store().await;
    let t = s.create_task(task("demo", "Supersede")).await.unwrap();
    let old = s
        .add_record(record(t.id, "old", "old body", None))
        .await
        .unwrap();
    sqlx::query("DELETE FROM outbox")
        .execute(s.pool())
        .await
        .unwrap();

    let new = s
        .add_record(record(t.id, "new", "new body", Some(old.id)))
        .await
        .unwrap();

    let rows = sqlx::query("SELECT op, entity_id, text, superseded FROM outbox ORDER BY id")
        .fetch_all(s.pool())
        .await
        .unwrap();
    assert_eq!(rows.len(), 2);
    let upsert = rows
        .iter()
        .find(|r| r.get::<String, _>("op") == "upsert")
        .unwrap();
    let supersede = rows
        .iter()
        .find(|r| r.get::<String, _>("op") == "supersede")
        .unwrap();
    assert_eq!(upsert.get::<String, _>("entity_id"), new.id.to_string());
    assert_eq!(upsert.get::<String, _>("text"), "new\nnew body");
    assert!(!upsert.get::<bool, _>("superseded"));
    assert_eq!(supersede.get::<String, _>("entity_id"), old.id.to_string());
    assert!(supersede.get::<Option<String>, _>("text").is_none());
    assert!(supersede.get::<bool, _>("superseded"));
}

#[tokio::test]
async fn convention_outbox_rolls_back_with_the_convention() {
    let (_db, s) = store().await;
    install_boom(&s).await;
    let err = s
        .add_convention("demo", "crates/**", "prefer sqlx")
        .await
        .unwrap_err();
    assert!(matches!(err, AvError::Store(_)), "{err}");
    assert_eq!(count(&s, "conventions").await, 0);
    assert_eq!(count(&s, "outbox").await, 0);

    drop_boom(&s).await;
    let id = s
        .add_convention("demo", "crates/**", "prefer sqlx")
        .await
        .unwrap();
    assert_eq!(count(&s, "conventions").await, 1);
    assert_eq!(count(&s, "outbox").await, 1);
    let row = sqlx::query(
        "SELECT kind, op, entity_id, convention_id, text, superseded, task_id, record_id
         FROM outbox",
    )
    .fetch_one(s.pool())
    .await
    .unwrap();
    assert_eq!(row.get::<String, _>("kind"), "convention");
    assert_eq!(row.get::<String, _>("op"), "upsert");
    assert_eq!(row.get::<String, _>("entity_id"), id.0.to_string());
    assert_eq!(row.get::<i64, _>("convention_id"), id.0);
    assert_eq!(row.get::<String, _>("text"), "prefer sqlx");
    assert!(!row.get::<bool, _>("superseded"));
    assert!(row.get::<Option<uuid::Uuid>, _>("task_id").is_none());
    assert!(row.get::<Option<uuid::Uuid>, _>("record_id").is_none());
}

#[tokio::test]
async fn fts_finds_repo_record_and_hides_superseded_and_other_repo() {
    let (_db, s) = store().await;
    let demo = s.create_task(task("demo", "Fts")).await.unwrap();
    let other = s.create_task(task("other", "Fts other")).await.unwrap();
    let body = "exponential backoff for the embedder";
    let live = s
        .add_record(record(demo.id, "note", body, None))
        .await
        .unwrap();
    let doomed = s
        .add_record(record(demo.id, "copy", body, None))
        .await
        .unwrap();
    s.add_record(record(demo.id, "later", "unrelated", Some(doomed.id)))
        .await
        .unwrap();
    s.add_record(record(other.id, "foreign", body, None))
        .await
        .unwrap();

    let hits = s
        .search_records_fts("demo", "embedder backoff", 10)
        .await
        .unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].id, live.id);

    assert!(
        s.search_records_fts("demo", "", 10)
            .await
            .unwrap()
            .is_empty()
    );
    assert!(
        s.search_records_fts("demo", "the", 10)
            .await
            .unwrap()
            .is_empty()
    );
    assert!(
        s.search_records_fts("demo", "   ", 10)
            .await
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn claim_is_fifo_and_holds_the_row() {
    let (_db, s) = store().await;
    let t = s.create_task(task("demo", "Claim")).await.unwrap();
    for i in 0..3 {
        s.add_record(record(t.id, &format!("n{i}"), "body", None))
            .await
            .unwrap();
    }
    let ids: Vec<i64> = sqlx::query_scalar("SELECT id FROM outbox ORDER BY id")
        .fetch_all(s.pool())
        .await
        .unwrap();
    assert_eq!(ids.len(), 3);

    let first = s.claim_outbox(2).await.unwrap();
    assert_eq!(
        first.iter().map(|r| r.id).collect::<Vec<_>>(),
        ids[..2].to_vec()
    );
    for row in &first {
        assert_eq!(row.attempts, 1);
        let ahead: bool = sqlx::query_scalar(
            "SELECT locked_until > now() + interval '55 seconds'
                 AND locked_until < now() + interval '70 seconds'
             FROM outbox WHERE id = $1",
        )
        .bind(row.id)
        .fetch_one(s.pool())
        .await
        .unwrap();
        assert!(ahead, "locked_until should be about 60s ahead");
    }

    let second = s.claim_outbox(2).await.unwrap();
    assert_eq!(second.len(), 1);
    assert_eq!(second[0].id, ids[2]);

    s.complete_outbox(ids[0]).await.unwrap();
    s.complete_outbox(ids[1]).await.unwrap();
    s.fail_outbox(ids[2], "embed down").await.unwrap();
    assert_eq!(count(&s, "outbox").await, 1);
    let err: String = sqlx::query_scalar("SELECT last_error FROM outbox WHERE id = $1")
        .bind(ids[2])
        .fetch_one(s.pool())
        .await
        .unwrap();
    assert_eq!(err, "embed down");

    s.complete_outbox(ids[2]).await.unwrap();
    assert_eq!(count(&s, "outbox").await, 0);
}
