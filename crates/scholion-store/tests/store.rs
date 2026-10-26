use std::path::Path;

use scholion_core::*;
use scholion_store::PgStore;
use scholion_testutil::TestDb;

async fn store() -> (TestDb, PgStore) {
    let db = TestDb::new().await;
    let store = PgStore::connect(&db.url).await.unwrap();
    (db, store)
}

fn new_task(title: &str) -> NewTask {
    NewTask {
        repo: "demo".into(),
        title: title.into(),
        goal: "goal".into(),
        owner_agent: "claude".into(),
    }
}

fn rec(task: TaskId, kind: RecordKind, summary: &str, anchors: Vec<Anchor>) -> NewRecord {
    NewRecord {
        task_id: task,
        kind,
        summary: summary.into(),
        body: format!("body of {summary}"),
        change_id: None,
        supersedes: None,
        author_agent: "claude".into(),
        anchors,
    }
}

fn anchor(path: &str, symbol: Option<&str>) -> Anchor {
    Anchor {
        path: path.into(),
        symbol: symbol.map(str::to_owned),
    }
}

#[tokio::test]
async fn slug_is_unique_for_same_title() {
    let (_db, s) = store().await;
    let a = s.create_task(new_task("Fix login")).await.unwrap();
    let b = s.create_task(new_task("Fix login")).await.unwrap();
    let c = s.create_task(new_task("  Fix   LOGIN!! ")).await.unwrap();
    assert_eq!(a.slug, "fix-login");
    assert_eq!(b.slug, "fix-login-2");
    assert_eq!(c.slug, "fix-login-3");
    assert_eq!(a.status, TaskStatus::Open);
}

#[tokio::test]
async fn concurrent_creates_with_same_title_get_distinct_slugs() {
    let (_db, s) = store().await;
    let tasks = futures_join(&s, 6).await;
    let mut slugs: Vec<_> = tasks.iter().map(|t| t.slug.clone()).collect();
    slugs.sort();
    slugs.dedup();
    assert_eq!(slugs.len(), 6);
}

async fn futures_join(s: &PgStore, n: usize) -> Vec<Task> {
    let mut handles = vec![];
    for _ in 0..n {
        let s = s.clone();
        handles.push(tokio::spawn(async move {
            s.create_task(new_task("Same title")).await.unwrap()
        }));
    }
    let mut out = vec![];
    for h in handles {
        out.push(h.await.unwrap());
    }
    out
}

#[tokio::test]
async fn set_workspace_and_head_roundtrip() {
    let (_db, s) = store().await;
    let t = s.create_task(new_task("T")).await.unwrap();
    s.set_workspace(t.id, Path::new("/ws/demo/t"), &ChangeId("base1".into()))
        .await
        .unwrap();
    let got = s.get_task(t.id).await.unwrap();
    assert_eq!(got.workspace_path, Path::new("/ws/demo/t"));
    assert_eq!(
        (got.base_change.0.as_str(), got.head_change.0.as_str()),
        ("base1", "base1")
    );
    s.set_head(
        t.id,
        &ChangeId("head2".into()),
        &["a.py".into(), "b.py".into()],
    )
    .await
    .unwrap();
    assert_eq!(s.get_task(t.id).await.unwrap().head_change.0, "head2");
    assert!(matches!(
        s.get_task(TaskId(uuid::Uuid::new_v4())).await,
        Err(AvError::NotFound(_))
    ));
}

#[tokio::test]
async fn records_roundtrip_with_anchors() {
    let (_db, s) = store().await;
    let t = s.create_task(new_task("T")).await.unwrap();
    let r = s
        .add_record(rec(
            t.id,
            RecordKind::Decision,
            "use argon2",
            vec![anchor("auth.py", Some("hash_pw")), anchor("cfg.py", None)],
        ))
        .await
        .unwrap();
    let got = s.get_record(r.id).await.unwrap();
    assert_eq!(got.summary, "use argon2");
    assert_eq!(got.kind, RecordKind::Decision);
    assert_eq!(got.anchors.len(), 2);
    assert!(got.anchors.contains(&anchor("auth.py", Some("hash_pw"))));
    assert_eq!(s.task_records(t.id).await.unwrap().len(), 1);
}

#[tokio::test]
async fn records_anchored_to_matches_by_path_or_symbol_only_in_repo() {
    let (_db, s) = store().await;
    let t = s.create_task(new_task("T")).await.unwrap();
    let mut other = new_task("Other repo");
    other.repo = "elsewhere".into();
    let o = s.create_task(other).await.unwrap();
    s.add_record(rec(
        t.id,
        RecordKind::Attempt,
        "by path",
        vec![anchor("a.py", None)],
    ))
    .await
    .unwrap();
    s.add_record(rec(
        t.id,
        RecordKind::Note,
        "by symbol",
        vec![anchor("b.py", Some("Greeter.hello"))],
    ))
    .await
    .unwrap();
    s.add_record(rec(
        t.id,
        RecordKind::Note,
        "unrelated",
        vec![anchor("z.py", Some("zzz"))],
    ))
    .await
    .unwrap();
    s.add_record(rec(
        o.id,
        RecordKind::Note,
        "other repo",
        vec![anchor("a.py", None)],
    ))
    .await
    .unwrap();

    let by_path = s
        .records_anchored_to("demo", &["a.py".into()], &[])
        .await
        .unwrap();
    assert_eq!(
        by_path
            .iter()
            .map(|r| r.summary.as_str())
            .collect::<Vec<_>>(),
        vec!["by path"]
    );
    let by_sym = s
        .records_anchored_to("demo", &[], &["Greeter.hello".into()])
        .await
        .unwrap();
    assert_eq!(
        by_sym
            .iter()
            .map(|r| r.summary.as_str())
            .collect::<Vec<_>>(),
        vec!["by symbol"]
    );
    assert!(
        s.records_anchored_to("demo", &[], &[])
            .await
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn superseded_record_hidden_from_anchor_query_but_still_in_task_records() {
    let (_db, s) = store().await;
    let t = s.create_task(new_task("T")).await.unwrap();
    let old = s
        .add_record(rec(
            t.id,
            RecordKind::Decision,
            "use md5",
            vec![anchor("a.py", None)],
        ))
        .await
        .unwrap();
    let mut newer = rec(
        t.id,
        RecordKind::Decision,
        "use argon2",
        vec![anchor("a.py", None)],
    );
    newer.supersedes = Some(old.id);
    s.add_record(newer).await.unwrap();

    let anchored = s
        .records_anchored_to("demo", &["a.py".into()], &[])
        .await
        .unwrap();
    assert_eq!(
        anchored
            .iter()
            .map(|r| r.summary.as_str())
            .collect::<Vec<_>>(),
        vec!["use argon2"]
    );
    assert_eq!(s.task_records(t.id).await.unwrap().len(), 2);
    let old_now = s.get_record(old.id).await.unwrap();
    let newer_id = anchored[0].id;
    assert_eq!(old_now.superseded_by, Some(newer_id));
    assert_eq!(anchored[0].superseded_by, None);
}

#[tokio::test]
async fn summary_201_chars_rejected_by_store() {
    let (_db, s) = store().await;
    let t = s.create_task(new_task("T")).await.unwrap();
    let err = s
        .add_record(rec(t.id, RecordKind::Note, &"x".repeat(201), vec![]))
        .await
        .unwrap_err();
    assert!(matches!(err, AvError::SummaryTooLong(201)));
    assert!(
        s.add_record(rec(t.id, RecordKind::Note, &"x".repeat(200), vec![]))
            .await
            .is_ok()
    );
}

#[tokio::test]
async fn live_tasks_touching_excludes_self_merged_and_abandoned() {
    let (db, s) = store().await;
    let me = s.create_task(new_task("Me")).await.unwrap();
    let peer = s.create_task(new_task("Peer")).await.unwrap();
    let merged = s.create_task(new_task("Merged")).await.unwrap();
    let reviewed = s.create_task(new_task("Reviewed")).await.unwrap();
    for t in [&me, &peer, &merged, &reviewed] {
        s.set_head(t.id, &ChangeId("h".into()), &["a.py".into()])
            .await
            .unwrap();
    }
    let pool = sqlx::PgPool::connect(&db.url).await.unwrap();
    for (t, status) in [(&merged, "merged"), (&reviewed, "in_review")] {
        sqlx::query("UPDATE tasks SET status = $2 WHERE id = $1")
            .bind(t.id.0)
            .bind(status)
            .execute(&pool)
            .await
            .unwrap();
    }

    let got = s
        .live_tasks_touching("demo", &["a.py".into()], me.id)
        .await
        .unwrap();
    // a task in review is about to land on main: exactly the one to warn about
    assert_eq!(
        got.iter().map(|t| t.slug.as_str()).collect::<Vec<_>>(),
        vec!["peer", "reviewed"]
    );
    assert!(
        s.live_tasks_touching("demo", &["nope.py".into()], me.id)
            .await
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn set_head_replaces_touched_paths() {
    let (_db, s) = store().await;
    let me = s.create_task(new_task("Me")).await.unwrap();
    let peer = s.create_task(new_task("Peer")).await.unwrap();
    s.set_head(peer.id, &ChangeId("h".into()), &["a.py".into()])
        .await
        .unwrap();
    s.set_head(peer.id, &ChangeId("h2".into()), &["b.py".into()])
        .await
        .unwrap();
    assert!(
        s.live_tasks_touching("demo", &["a.py".into()], me.id)
            .await
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        s.live_tasks_touching("demo", &["b.py".into()], me.id)
            .await
            .unwrap()
            .len(),
        1
    );
}

#[tokio::test]
async fn conventions_for_matches_glob() {
    let (_db, s) = store().await;
    s.add_convention("demo", "src/**/*.py", "use type hints")
        .await
        .unwrap();
    s.add_convention("demo", "docs/**", "keep docs short")
        .await
        .unwrap();
    s.add_convention("other", "src/**/*.py", "other repo rule")
        .await
        .unwrap();
    let got = s
        .conventions_for("demo", &["src/a/b.py".into()])
        .await
        .unwrap();
    assert_eq!(
        got.iter().map(|c| c.text.as_str()).collect::<Vec<_>>(),
        vec!["use type hints"]
    );
    assert!(
        s.conventions_for("demo", &["lib/x.py".into()])
            .await
            .unwrap()
            .is_empty()
    );
    assert!(s.conventions_for("demo", &[]).await.unwrap().is_empty());
}

#[tokio::test]
async fn ledger_put_then_get_keeps_highest_tier() {
    let (_db, s) = store().await;
    s.ledger_put(
        "s1",
        &[
            ("rec:1".into(), Tier::Summary),
            ("sym:2".into(), Tier::Handle),
        ],
    )
    .await
    .unwrap();
    s.ledger_put(
        "s1",
        &[("rec:1".into(), Tier::Handle), ("sym:2".into(), Tier::Full)],
    )
    .await
    .unwrap();
    let got = s.ledger_get("s1").await.unwrap();
    assert_eq!(got["rec:1"], Tier::Summary);
    assert_eq!(got["sym:2"], Tier::Full);
    assert!(s.ledger_get("other-session").await.unwrap().is_empty());
}

#[tokio::test]
async fn find_record_resolves_unique_prefix_and_rejects_bad_ones() {
    let (_db, s) = store().await;
    let t = s.create_task(new_task("T")).await.unwrap();
    let r = s
        .add_record(rec(t.id, RecordKind::Note, "hello", vec![]))
        .await
        .unwrap();
    let full = r.id.to_string();
    assert_eq!(s.find_record(&full).await.unwrap().id, r.id);
    assert_eq!(s.find_record(&full[..8]).await.unwrap().id, r.id);
    assert!(
        matches!(s.find_record(&full[..3]).await, Err(AvError::Invalid(_))),
        "too short"
    );
    assert!(
        matches!(s.find_record("zzzzzzzz").await, Err(AvError::Invalid(_))),
        "not hex"
    );
    assert!(
        matches!(s.find_record("%%%%%%%%").await, Err(AvError::Invalid(_))),
        "wildcards rejected"
    );
    assert!(matches!(
        s.find_record("deadbeef").await,
        Err(AvError::NotFound(_))
    ));
}

#[tokio::test]
async fn touched_paths_returns_sorted_current_set() {
    let (_db, s) = store().await;
    let t = s.create_task(new_task("T")).await.unwrap();
    assert!(s.touched_paths(t.id).await.unwrap().is_empty());
    s.set_head(t.id, &ChangeId("h".into()), &["b.py".into(), "a.py".into()])
        .await
        .unwrap();
    assert_eq!(
        s.touched_paths(t.id).await.unwrap(),
        vec!["a.py".to_string(), "b.py".to_string()]
    );
    s.set_head(t.id, &ChangeId("h2".into()), &["c.py".into()])
        .await
        .unwrap();
    assert_eq!(
        s.touched_paths(t.id).await.unwrap(),
        vec!["c.py".to_string()]
    );
}

#[tokio::test]
async fn set_status_changes_status_and_hides_task_from_live_work() {
    let (_db, s) = store().await;
    let me = s.create_task(new_task("Me")).await.unwrap();
    let peer = s.create_task(new_task("Peer")).await.unwrap();
    s.set_head(peer.id, &ChangeId("h".into()), &["a.py".into()])
        .await
        .unwrap();
    assert_eq!(
        s.live_tasks_touching("demo", &["a.py".into()], me.id)
            .await
            .unwrap()
            .len(),
        1
    );
    s.set_status(peer.id, TaskStatus::Abandoned).await.unwrap();
    assert_eq!(
        s.get_task(peer.id).await.unwrap().status,
        TaskStatus::Abandoned
    );
    assert!(
        s.live_tasks_touching("demo", &["a.py".into()], me.id)
            .await
            .unwrap()
            .is_empty()
    );
    assert!(matches!(
        s.set_status(TaskId(uuid::Uuid::new_v4()), TaskStatus::Merged)
            .await,
        Err(AvError::NotFound(_))
    ));
}

#[tokio::test]
async fn receipt_roundtrip_by_prefix() {
    let (_db, s) = store().await;
    let t = s.create_task(new_task("Receipts")).await.unwrap();
    let id = s
        .add_receipt(NewReceipt {
            task_id: t.id,
            session: "s".into(),
            head_change: ChangeId("abc".into()),
            packer_version: 1,
            budget: 500,
            tokens: 120,
            items: vec![ReceiptItem {
                id: "sym:1".into(),
                tier: Tier::Full,
                score: 1.5,
                reason: "seed".into(),
                source: Some(("a.py".into(), 1, 3)),
                tokens: 40,
            }],
            omitted: vec!["- sym:2 far".into()],
        })
        .await
        .unwrap();
    assert!(id.starts_with("rcp:") && id.len() == 12, "{id}");
    let got = s.get_receipt(&id[4..10]).await.unwrap();
    assert_eq!(got["task_id"], t.id.0.to_string());
    assert_eq!(got["head_change"], "abc");
    assert_eq!(got["items"][0]["reason"], "seed");
    assert_eq!(got["items"][0]["source"], serde_json::json!(["a.py", 1, 3]));
    assert!(matches!(
        s.get_receipt("ffffff").await,
        Err(AvError::NotFound(_))
    ));
}

#[tokio::test]
async fn add_record_with_stores_hashes_and_receipt_or_writes_nothing() {
    let (db, s) = store().await;
    let t = s.create_task(new_task("Me")).await.unwrap();
    let anchors = vec![Anchor {
        path: "a.py".into(),
        symbol: Some("f".into()),
    }];
    let hashes = [Some("h1".to_owned())];
    // an unknown receipt fails the whole write: no record without its citation
    let err = s
        .add_record_with(
            rec(t.id, RecordKind::Note, "n", anchors.clone()),
            &hashes,
            Some("rcp:deadbeef"),
        )
        .await;
    assert!(matches!(err, Err(AvError::NotFound(_))), "{err:?}");
    assert!(s.task_records(t.id).await.unwrap().is_empty());
    // so does a hash list that does not match the anchors
    let err = s
        .add_record_with(
            rec(t.id, RecordKind::Note, "n", anchors.clone()),
            &[None, None],
            None,
        )
        .await;
    assert!(matches!(err, Err(AvError::Invalid(_))), "{err:?}");
    assert!(s.task_records(t.id).await.unwrap().is_empty());

    let rcp = s
        .add_receipt(NewReceipt {
            task_id: t.id,
            session: "s".into(),
            head_change: ChangeId("abc".into()),
            packer_version: 1,
            budget: 500,
            tokens: 0,
            items: vec![],
            omitted: vec![],
        })
        .await
        .unwrap();
    let r = s
        .add_record_with(
            rec(t.id, RecordKind::Note, "n", anchors),
            &hashes,
            Some(&rcp),
        )
        .await
        .unwrap();
    let pool = sqlx::PgPool::connect(&db.url).await.unwrap();
    let (hash, cited): (Option<String>, String) = sqlx::query_as(
        "SELECT a.content_hash, 'rcp:' || left(replace(r.receipt_id::text, '-', ''), 8)
         FROM records r JOIN anchors a ON a.record_id = r.id WHERE r.id = $1",
    )
    .bind(r.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        (hash.as_deref(), cited.as_str()),
        (Some("h1"), rcp.as_str())
    );
}

async fn handed_off_task(s: &PgStore) -> Task {
    let t = s.create_task(new_task("Pass it on")).await.unwrap();
    s.transition(t.id, lifecycle::Event::Handoff).await.unwrap();
    s.get_task(t.id).await.unwrap()
}

#[tokio::test]
async fn claim_moves_a_handed_off_task_to_the_claimant_and_bumps_the_generation() {
    let (_db, s) = store().await;
    let t = handed_off_task(&s).await;
    assert_eq!(t.generation, 1);
    let c = s.claim_task(t.id, "codex", 600).await.unwrap();
    assert_eq!(c.status, TaskStatus::Open);
    assert_eq!(c.owner_agent, "codex");
    assert_eq!(c.generation, 2);
    let lease = c.lease_expires.expect("a claim sets a lease");
    let secs = (lease - chrono::Utc::now()).num_seconds();
    assert!((590..=600).contains(&secs), "lease in {secs}s");
}

#[tokio::test]
async fn claim_by_the_owner_is_refused() {
    let (_db, s) = store().await;
    let t = handed_off_task(&s).await;
    let err = s.claim_task(t.id, "claude", 600).await.unwrap_err();
    assert!(matches!(err, AvError::IllegalState(_)), "{err:?}");
    assert!(err.to_string().contains("already yours"), "{err}");
    let after = s.get_task(t.id).await.unwrap();
    assert_eq!((after.status, after.generation), (TaskStatus::HandedOff, 1));
}

#[tokio::test]
async fn an_open_task_can_be_claimed_only_once_its_lease_expired() {
    let (_db, s) = store().await;
    let t = s.create_task(new_task("Busy")).await.unwrap();
    assert!(s.renew_lease(t.id, 1, 600).await.unwrap());
    let err = s.claim_task(t.id, "codex", 600).await.unwrap_err();
    assert!(err.to_string().contains("lease"), "{err}");
    // lease of 0 seconds: expired as soon as it is written
    assert!(s.renew_lease(t.id, 1, 0).await.unwrap());
    let c = s.claim_task(t.id, "codex", 600).await.unwrap();
    assert_eq!((c.owner_agent.as_str(), c.generation), ("codex", 2));
    // a task in review is not up for grabs, lease or not
    let r = s.create_task(new_task("Reviewing")).await.unwrap();
    s.transition(r.id, lifecycle::Event::Submit).await.unwrap();
    let err = s.claim_task(r.id, "codex", 600).await.unwrap_err();
    assert!(err.to_string().contains("in_review"), "{err}");
}

#[tokio::test]
async fn renew_lease_only_for_the_current_generation() {
    let (_db, s) = store().await;
    let t = handed_off_task(&s).await;
    s.claim_task(t.id, "codex", 600).await.unwrap();
    assert!(
        !s.renew_lease(t.id, 1, 600).await.unwrap(),
        "stale generation"
    );
    assert!(s.renew_lease(t.id, 2, 600).await.unwrap());
}

#[tokio::test]
async fn concurrent_claims_have_one_winner() {
    let (_db, s) = store().await;
    let t = handed_off_task(&s).await;
    let agents: Vec<String> = (0..8).map(|n| format!("agent-{n}")).collect();
    let results =
        futures::future::join_all(agents.iter().map(|a| s.claim_task(t.id, a, 600))).await;
    let won: Vec<&Task> = results.iter().filter_map(|r| r.as_ref().ok()).collect();
    assert_eq!(won.len(), 1, "{results:?}");
    let after = s.get_task(t.id).await.unwrap();
    assert_eq!(after.owner_agent, won[0].owner_agent);
    assert_eq!(after.generation, 2);
}

#[tokio::test]
async fn transition_with_record_writes_both_or_neither() {
    let (_db, s) = store().await;
    let t = s.create_task(new_task("Hand over")).await.unwrap();
    let r = rec(t.id, RecordKind::Handoff, "finish the parser", vec![]);
    let (status, record) = s
        .transition_with_record(t.id, lifecycle::Event::Handoff, r.clone())
        .await
        .unwrap();
    assert_eq!(status, TaskStatus::HandedOff);
    assert_eq!(record.kind, RecordKind::Handoff);
    // illegal now (already handed off): no second record
    let err = s
        .transition_with_record(t.id, lifecycle::Event::Handoff, r)
        .await
        .unwrap_err();
    assert!(matches!(err, AvError::IllegalState(_)), "{err:?}");
    let handoffs = s
        .task_records(t.id)
        .await
        .unwrap()
        .into_iter()
        .filter(|r| r.kind == RecordKind::Handoff)
        .count();
    assert_eq!(handoffs, 1);
    // an invalid record fails the whole step: the status does not move either
    let u = s.create_task(new_task("Too long")).await.unwrap();
    let long = rec(u.id, RecordKind::Handoff, &"x".repeat(201), vec![]);
    assert!(
        s.transition_with_record(u.id, lifecycle::Event::Handoff, long)
            .await
            .is_err()
    );
    assert_eq!(s.get_task(u.id).await.unwrap().status, TaskStatus::Open);
}

#[tokio::test]
async fn backup_state_tracks_remote_failures_and_due_repos() {
    let (_db, s) = store().await;
    assert_eq!(
        s.backup_state("demo").await.unwrap(),
        BackupState::default()
    );
    let st = s
        .set_backup_remote("demo", Some("/srv/b.git"))
        .await
        .unwrap();
    assert_eq!(st.remote.as_deref(), Some("/srv/b.git"));
    assert_eq!(
        s.due_backups().await.unwrap(),
        vec!["demo".to_owned()],
        "a new remote is due"
    );
    let st = s
        .backup_failed("demo", "unreachable", std::time::Duration::from_secs(300))
        .await
        .unwrap();
    assert_eq!(
        (st.failures, st.last_error.as_deref()),
        (1, Some("unreachable"))
    );
    assert!(
        s.due_backups().await.unwrap().is_empty(),
        "waits for the backoff"
    );
    let st = s
        .backup_failed("demo", "still down", std::time::Duration::ZERO)
        .await
        .unwrap();
    assert_eq!(st.failures, 2);
    assert_eq!(s.due_backups().await.unwrap(), vec!["demo".to_owned()]);
    let st = s
        .backup_succeeded("demo", &ChangeId("abc".into()))
        .await
        .unwrap();
    assert_eq!(st.failures, 0);
    assert_eq!(st.last_error, None);
    assert_eq!(st.next_retry, None);
    assert_eq!(st.last_pushed, Some(ChangeId("abc".into())));
    assert!(s.due_backups().await.unwrap().is_empty());
    s.mark_backups_due().await.unwrap();
    assert_eq!(s.due_backups().await.unwrap(), vec!["demo".to_owned()]);
    let st = s.set_backup_remote("demo", None).await.unwrap();
    assert_eq!(st.remote, None);
    assert!(s.due_backups().await.unwrap().is_empty());
}

#[tokio::test]
async fn prune_removes_idle_sessions_only() {
    let (_db, s) = store().await;
    for session in ["s-old", "s-new"] {
        s.ledger_put(session, &[("rec:1".into(), Tier::Full)])
            .await
            .unwrap();
        s.charge_expand(session, 10, None).await.unwrap();
    }
    for table in ["ledger", "ledger_sessions"] {
        sqlx::query(&format!(
            "UPDATE {table} SET last_seen = now() - interval '40 days' WHERE session = 's-old'"
        ))
        .execute(s.pool())
        .await
        .unwrap();
    }
    let month = std::time::Duration::from_secs(30 * 86_400);
    assert_eq!(s.prune_sessions(month).await.unwrap(), 1);
    assert!(s.ledger_get("s-old").await.unwrap().is_empty());
    assert_eq!(s.expand_tokens("s-old").await.unwrap(), 0);
    assert_eq!(s.ledger_get("s-new").await.unwrap().len(), 1);
    assert_eq!(s.expand_tokens("s-new").await.unwrap(), 10);
    assert_eq!(s.prune_sessions(month).await.unwrap(), 0);
}
