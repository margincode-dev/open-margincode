//! Outcome rows and task reads (`pack_outcomes`, `task_reads`).

use margincode_core::outcome::{Measures, SentPack};
use margincode_core::*;
use margincode_store::{NewOutcome, PgStore, TaskPacks};
use margincode_testutil::TestDb;

async fn store() -> (TestDb, PgStore) {
    let db = TestDb::new().await;
    let store = PgStore::connect(&db.url).await.unwrap();
    (db, store)
}

async fn task(s: &PgStore, title: &str) -> Task {
    s.create_task(NewTask {
        repo: "demo".into(),
        title: title.into(),
        goal: "goal".into(),
        owner_agent: "claude".into(),
    })
    .await
    .unwrap()
}

async fn set_merged(s: &PgStore, t: &Task, status: &str, merge_commit: &str) {
    sqlx::query("UPDATE tasks SET status = $2, merge_commit = $3 WHERE id = $1")
        .bind(t.id.0)
        .bind(status)
        .bind(merge_commit)
        .execute(s.pool())
        .await
        .unwrap();
}

fn measures(code_tokens: u64, missed: &[&str]) -> Measures {
    Measures {
        code_tokens,
        code_tokens_changed: code_tokens / 2,
        code_tokens_used: code_tokens / 2,
        changed: vec!["app/a.py".into(), "app/b.py".into()],
        missed: missed.iter().map(|p| p.to_string()).collect(),
        named_only: vec![],
        opened_not_sent: vec!["app/c.py".into()],
    }
}

async fn put(s: &PgStore, t: &Task, merge_commit: &str, m: &Measures) {
    s.put_outcome(NewOutcome {
        task: t,
        merge_commit: Some(merge_commit),
        changed_source: "vcs",
        packs: &TaskPacks {
            packs: vec![],
            first: Some((11, 800)),
        },
        measures: m,
        files_opened: 3,
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn a_read_counts_once_per_call_and_an_expand_marks_the_path() {
    let (_db, s) = store().await;
    let t = task(&s, "a").await;
    s.note_read(t.id, "app/a.py", false).await.unwrap();
    s.note_read(t.id, "app/a.py", true).await.unwrap();
    s.note_read(t.id, "app/a.py", false).await.unwrap();
    s.note_read(t.id, "app/b.py", false).await.unwrap();
    let rows: Vec<(String, i32, bool)> =
        sqlx::query_as("SELECT path, reads, expanded FROM task_reads ORDER BY path")
            .fetch_all(s.pool())
            .await
            .unwrap();
    assert_eq!(
        rows,
        vec![("app/a.py".into(), 3, true), ("app/b.py".into(), 1, false)]
    );
    assert_eq!(s.read_paths(t.id).await.unwrap(), ["app/a.py", "app/b.py"]);
}

#[tokio::test]
async fn an_outcome_is_replaced_by_a_new_merge_and_marked_reverted_when_its_merge_is_gone() {
    let (_db, s) = store().await;
    let t = task(&s, "a").await;
    set_merged(&s, &t, "merged", "m1").await;
    put(&s, &t, "m1", &measures(100, &["app/b.py"])).await;
    let rows = s.outcomes("demo").await.unwrap();
    assert_eq!(rows.len(), 1);
    let r = &rows[0];
    assert!(!r.reverted);
    assert_eq!(
        (r.packer_version, r.first_budget, r.packs),
        (Some(11), Some(800), 0)
    );
    assert_eq!((r.code_tokens, r.code_tokens_changed), (100, 50));
    assert_eq!((r.files_changed, r.files_missed), (2, 1));
    assert_eq!(r.missed_paths, ["app/b.py"]);
    assert_eq!((r.files_opened, r.opened_not_sent), (3, 1));

    set_merged(&s, &t, "open", "m1").await;
    assert!(s.outcomes("demo").await.unwrap()[0].reverted, "reverted");
    set_merged(&s, &t, "in_review", "").await;
    assert!(s.outcomes("demo").await.unwrap()[0].reverted, "undone");

    set_merged(&s, &t, "merged", "m2").await;
    assert!(
        s.outcomes("demo").await.unwrap()[0].reverted,
        "merged again, not yet measured"
    );
    put(&s, &t, "m2", &measures(40, &[])).await;
    let rows = s.outcomes("demo").await.unwrap();
    assert_eq!(rows.len(), 1);
    assert!(!rows[0].reverted);
    assert_eq!((rows[0].code_tokens, rows[0].files_missed), (40, 0));
    assert!(s.outcomes("other").await.unwrap().is_empty());
}

#[tokio::test]
async fn task_packs_are_the_receipts_oldest_first_and_backfill_finds_merged_tasks_without_one() {
    let (_db, s) = store().await;
    let t = task(&s, "a").await;
    let item = |id: &str, tokens| ReceiptItem {
        id: id.into(),
        tier: Tier::Summary,
        score: 1.0,
        reason: "seed".into(),
        source: Some(("app/a.py".into(), 1, 2)),
        tokens,
    };
    for (budget, id) in [(800, "sym:1"), (2000, "sym:2")] {
        s.add_receipt(NewReceipt {
            task_id: t.id,
            session: "s-1".into(),
            head_change: ChangeId(String::new()),
            packer_version: 11,
            budget,
            tokens: 5,
            items: vec![item(id, 5)],
            omitted: vec![],
        })
        .await
        .unwrap();
    }
    let packs = s.task_packs(t.id).await.unwrap();
    assert_eq!(packs.first, Some((11, 800)));
    let ids: Vec<&str> = packs
        .packs
        .iter()
        .map(|p: &SentPack| p.items[0].id.as_str())
        .collect();
    assert_eq!(ids, ["sym:1", "sym:2"]);
    assert_eq!(packs.packs[0].session, "s-1");
    assert_eq!(packs.packs[0].items[0].source, item("sym:1", 5).source);

    let _open = task(&s, "open").await;
    assert!(
        s.merged_without_outcome("demo", 10)
            .await
            .unwrap()
            .is_empty()
    );
    set_merged(&s, &t, "merged", "m1").await;
    assert_eq!(s.merged_without_outcome("demo", 10).await.unwrap(), [t.id]);
    put(&s, &t, "m1", &measures(1, &[])).await;
    assert!(
        s.merged_without_outcome("demo", 10)
            .await
            .unwrap()
            .is_empty()
    );
}
