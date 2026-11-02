//! Plan 10 A3: targets. A target is something a team works toward (a release, a milestone);
//! backlog items count toward at most one, and its progress is how many of those have merged.

use scholion_core::routing::*;
use scholion_core::*;
use scholion_store::PgStore;
use scholion_testutil::TestDb;

async fn store() -> (TestDb, PgStore) {
    let db = TestDb::new().await;
    let store = PgStore::connect(&db.url).await.unwrap();
    (db, store)
}

fn new_target(repo: &str, title: &str) -> NewTarget {
    NewTarget {
        repo: repo.into(),
        title: title.into(),
        detail: String::new(),
        due: None,
        created_by: "me".into(),
    }
}

fn new_item(repo: &str, title: &str, target: Option<&str>) -> NewBacklogItem {
    NewBacklogItem {
        repo: repo.into(),
        title: title.into(),
        goal: format!("goal of {title}"),
        needs: vec![],
        prefer: None,
        priority: 0,
        reserve_secs: DEFAULT_RESERVE_SECS,
        posted_by: "me".into(),
        after: vec![],
        target: target.map(str::to_owned),
    }
}

#[tokio::test]
async fn targets_are_added_listed_by_repo_and_closed() {
    let (_db, s) = store().await;
    let mut t = new_target("demo", "  Ship v1  ");
    t.detail = "the first release".into();
    t.due = Some(chrono::NaiveDate::from_ymd_opt(2026, 12, 1).unwrap());
    let v1 = s.add_target(t).await.unwrap();
    assert_eq!(v1.title, "Ship v1");
    assert_eq!(v1.status, TargetStatus::Active);
    assert_eq!(v1.detail, "the first release");
    assert_eq!(v1.due.unwrap().to_string(), "2026-12-01");
    assert_eq!((v1.items, v1.queued, v1.working, v1.merged), (0, 0, 0, 0));
    s.add_target(new_target("other", "Elsewhere"))
        .await
        .unwrap();

    let demo = s.list_targets(Some("demo"), None).await.unwrap();
    assert_eq!(demo.len(), 1);
    assert_eq!(s.list_targets(None, None).await.unwrap().len(), 2);

    let done = s
        .set_target_status(v1.id, TargetStatus::Done)
        .await
        .unwrap();
    assert_eq!(done.status, TargetStatus::Done);
    assert!(
        s.list_targets(Some("demo"), Some(TargetStatus::Active))
            .await
            .unwrap()
            .is_empty()
    );
    // reopening is a status change like any other
    let back = s
        .set_target_status(v1.id, TargetStatus::Active)
        .await
        .unwrap();
    assert_eq!(back.status, TargetStatus::Active);

    let err = s
        .set_target_status(uuid::Uuid::new_v4(), TargetStatus::Done)
        .await
        .unwrap_err();
    assert!(matches!(err, AvError::NotFound(_)), "{err:?}");
}

#[tokio::test]
async fn a_blank_or_long_title_is_invalid() {
    let (_db, s) = store().await;
    let err = s.add_target(new_target("demo", "   ")).await.unwrap_err();
    assert!(matches!(err, AvError::Invalid(_)), "{err:?}");
    let err = s
        .add_target(new_target("demo", &"x".repeat(201)))
        .await
        .unwrap_err();
    assert!(matches!(err, AvError::Invalid(_)), "{err:?}");
    let err = s.add_target(new_target(" ", "Ship")).await.unwrap_err();
    assert!(matches!(err, AvError::Invalid(_)), "{err:?}");
}

#[tokio::test]
async fn a_target_is_named_by_id_or_by_the_slug_of_its_title() {
    let (_db, s) = store().await;
    let v1 = s.add_target(new_target("demo", "Ship v1")).await.unwrap();
    s.add_target(new_target("other", "Ship v1")).await.unwrap();
    assert_eq!(
        s.resolve_target(Some("demo"), "ship-v1").await.unwrap(),
        v1.id
    );
    assert_eq!(
        s.resolve_target(None, &v1.id.to_string()).await.unwrap(),
        v1.id
    );
    // the same slug in two repos needs the repo
    let err = s.resolve_target(None, "ship-v1").await.unwrap_err();
    assert!(matches!(err, AvError::Invalid(_)), "{err:?}");
    let err = s.resolve_target(Some("demo"), "nope").await.unwrap_err();
    assert!(matches!(err, AvError::NotFound(_)), "{err:?}");
    // an id of another repo is not this repo's target
    let err = s
        .resolve_target(Some("other"), &v1.id.to_string())
        .await
        .unwrap_err();
    assert!(matches!(err, AvError::NotFound(_)), "{err:?}");
}

#[tokio::test]
async fn backlog_items_count_toward_a_target_and_progress_follows_their_tasks() {
    let (_db, s) = store().await;
    let v1 = s.add_target(new_target("demo", "Ship v1")).await.unwrap();

    // posted with the target, or linked to it later
    let a = s
        .post_backlog(new_item("demo", "first", Some("ship-v1")))
        .await
        .unwrap();
    assert_eq!(a.target_id, Some(v1.id));
    assert_eq!(a.target.as_deref(), Some("Ship v1"));
    let b = s
        .post_backlog(new_item("demo", "second", None))
        .await
        .unwrap();
    assert_eq!(b.target_id, None);
    let b = s
        .set_backlog_target(b.item.id, Some(&v1.id.to_string()))
        .await
        .unwrap();
    assert_eq!(b.target_id, Some(v1.id));
    let c = s
        .post_backlog(new_item("demo", "third", Some("ship-v1")))
        .await
        .unwrap();

    let t = &s.list_targets(Some("demo"), None).await.unwrap()[0];
    assert_eq!((t.items, t.queued, t.working, t.merged), (3, 3, 0, 0));

    // one taken and worked on, one merged, one dropped (a dropped item no longer counts)
    assert!(s.take_backlog(a.item.id, "claude").await.unwrap());
    let task = s
        .create_task(NewTask {
            repo: "demo".into(),
            title: "first".into(),
            goal: "goal".into(),
            owner_agent: "claude".into(),
        })
        .await
        .unwrap();
    s.link_backlog_task(a.item.id, task.id).await.unwrap();
    assert!(s.take_backlog(b.item.id, "grok").await.unwrap());
    let merged = s
        .create_task(NewTask {
            repo: "demo".into(),
            title: "second".into(),
            goal: "goal".into(),
            owner_agent: "grok".into(),
        })
        .await
        .unwrap();
    s.link_backlog_task(b.item.id, merged.id).await.unwrap();
    sqlx::query("UPDATE tasks SET status = 'merged' WHERE id = $1")
        .bind(merged.id.0)
        .execute(s.pool())
        .await
        .unwrap();
    s.drop_backlog(c.item.id).await.unwrap();

    let t = &s.list_targets(Some("demo"), None).await.unwrap()[0];
    assert_eq!((t.items, t.queued, t.working, t.merged), (2, 0, 1, 1));

    // unlinking takes an item out of the count
    let a = s.set_backlog_target(a.item.id, None).await.unwrap();
    assert_eq!(a.target_id, None);
    let t = &s.list_targets(Some("demo"), None).await.unwrap()[0];
    assert_eq!((t.items, t.working, t.merged), (1, 0, 1));
}

#[tokio::test]
async fn only_an_active_target_of_the_items_repo_takes_items() {
    let (_db, s) = store().await;
    let v1 = s.add_target(new_target("demo", "Ship v1")).await.unwrap();
    s.add_target(new_target("other", "Their goal"))
        .await
        .unwrap();

    // a target of another repo
    let err = s
        .post_backlog(new_item("demo", "x", Some("their-goal")))
        .await
        .unwrap_err();
    assert!(matches!(err, AvError::NotFound(_)), "{err:?}");

    // a closed target
    s.set_target_status(v1.id, TargetStatus::Done)
        .await
        .unwrap();
    let err = s
        .post_backlog(new_item("demo", "y", Some("ship-v1")))
        .await
        .unwrap_err();
    assert!(matches!(err, AvError::IllegalState(_)), "{err:?}");

    // a dropped item cannot be linked
    s.set_target_status(v1.id, TargetStatus::Active)
        .await
        .unwrap();
    let item = s.post_backlog(new_item("demo", "z", None)).await.unwrap();
    s.drop_backlog(item.item.id).await.unwrap();
    let err = s
        .set_backlog_target(item.item.id, Some("ship-v1"))
        .await
        .unwrap_err();
    assert!(matches!(err, AvError::IllegalState(_)), "{err:?}");
    let err = s
        .set_backlog_target(uuid::Uuid::new_v4(), None)
        .await
        .unwrap_err();
    assert!(matches!(err, AvError::NotFound(_)), "{err:?}");
}
