//! Plan 8 Task 3: agent profiles, the backlog and the handoff tags.

use margincode_core::routing::*;
use margincode_core::*;
use margincode_store::PgStore;
use margincode_testutil::TestDb;

async fn store() -> (TestDb, PgStore) {
    let db = TestDb::new().await;
    let store = PgStore::connect(&db.url).await.unwrap();
    (db, store)
}

fn profile(name: &str, best: &[&str], can: &[&str], cost: i32) -> Profile {
    Profile {
        name: name.into(),
        best: best.iter().map(|s| s.to_string()).collect(),
        can: can.iter().map(|s| s.to_string()).collect(),
        cost_rank: cost,
        aliases: vec![],
        max_open: None,
        managed_by: None,
    }
}

fn new_item(repo: &str, title: &str, needs: &[&str]) -> NewBacklogItem {
    NewBacklogItem {
        repo: repo.into(),
        title: title.into(),
        goal: format!("goal of {title}"),
        needs: needs.iter().map(|s| s.to_string()).collect(),
        prefer: None,
        priority: 0,
        reserve_secs: DEFAULT_RESERVE_SECS,
        posted_by: "me".into(),
        after: vec![],
        target: None,
        propose: false,
        why: None,
    }
}

fn new_task() -> NewTask {
    NewTask {
        repo: "demo".into(),
        title: "Pass it on".into(),
        goal: "goal".into(),
        owner_agent: "claude".into(),
    }
}

#[tokio::test]
async fn profiles_are_upserted_and_listed_cheapest_first() {
    let (_db, s) = store().await;
    s.put_profile(&profile("claude", &["code"], &["docs"], 4))
        .await
        .unwrap();
    s.put_profile(&profile("muse", &[], &["code"], 1))
        .await
        .unwrap();
    s.put_profile(&profile("grok", &["image"], &[], 2))
        .await
        .unwrap();
    // a second put replaces the first
    let mut c = profile("claude", &["code", "tests"], &[], 3);
    c.aliases = vec!["claude-code".into()];
    c.managed_by = Some("ee".into());
    s.put_profile(&c).await.unwrap();

    let all = s.list_profiles().await.unwrap();
    let names: Vec<&str> = all.iter().map(|p| p.name.as_str()).collect();
    assert_eq!(names, ["muse", "grok", "claude"]);
    assert_eq!(all[2], c);
}

#[tokio::test]
async fn blank_tags_and_names_are_refused() {
    let (_db, s) = store().await;
    let err = s
        .put_profile(&profile("grok", &["image", " "], &[], 2))
        .await
        .unwrap_err();
    assert!(matches!(err, AvError::Invalid(_)), "{err:?}");
    let err = s.put_profile(&profile("", &[], &[], 2)).await.unwrap_err();
    assert!(matches!(err, AvError::Invalid(_)), "{err:?}");
    let err = s
        .post_backlog(new_item("demo", "t", &[""]))
        .await
        .unwrap_err();
    assert!(matches!(err, AvError::Invalid(_)), "{err:?}");
    assert!(s.list_profiles().await.unwrap().is_empty());
}

#[tokio::test]
async fn tags_are_stored_lowercase_and_trimmed() {
    let (_db, s) = store().await;
    s.put_profile(&profile("grok", &[" Image "], &["UI"], 2))
        .await
        .unwrap();
    let e = s
        .post_backlog(new_item("demo", "t", &["IMAGE"]))
        .await
        .unwrap();
    let p = &s.list_profiles().await.unwrap()[0];
    assert_eq!(
        (p.best.clone(), p.can.clone()),
        (vec!["image".into()], vec!["ui".into()])
    );
    assert_eq!(e.item.needs, ["image"]);
}

#[tokio::test]
async fn the_backlog_lists_oldest_first_and_filters_by_repo_and_state() {
    let (_db, s) = store().await;
    let a = s
        .post_backlog(new_item("demo", "first", &[]))
        .await
        .unwrap();
    let b = s
        .post_backlog(new_item("other", "second", &["image"]))
        .await
        .unwrap();
    let c = s
        .post_backlog(new_item("demo", "third", &[]))
        .await
        .unwrap();
    assert_eq!(a.state, BacklogState::Queued);
    assert_eq!(b.item.needs, ["image"]);
    assert_eq!(b.posted_by, "me");

    let ids = |v: Vec<BacklogEntry>| v.into_iter().map(|e| e.item.id).collect::<Vec<_>>();
    assert_eq!(
        ids(s.list_backlog(None, None).await.unwrap()),
        [a.item.id, b.item.id, c.item.id]
    );
    assert_eq!(
        ids(s.list_backlog(Some("demo"), None).await.unwrap()),
        [a.item.id, c.item.id]
    );
    assert!(s.take_backlog(a.item.id, "muse").await.unwrap());
    assert_eq!(
        ids(s
            .list_backlog(None, Some(BacklogState::Queued))
            .await
            .unwrap()),
        [b.item.id, c.item.id]
    );
}

#[tokio::test]
async fn concurrent_takes_of_one_item_have_one_winner() {
    let (_db, s) = store().await;
    let item = s.post_backlog(new_item("demo", "t", &[])).await.unwrap();
    let agents: Vec<String> = (0..8).map(|n| format!("agent-{n}")).collect();
    let results =
        futures::future::join_all(agents.iter().map(|a| s.take_backlog(item.item.id, a))).await;
    let won: Vec<usize> = results
        .iter()
        .enumerate()
        .filter(|(_, r)| *r.as_ref().unwrap())
        .map(|(i, _)| i)
        .collect();
    assert_eq!(won.len(), 1, "{results:?}");
    let e = &s.list_backlog(None, None).await.unwrap()[0];
    assert_eq!(e.state, BacklogState::Taken);
    assert_eq!(e.taken_by.as_deref(), Some(agents[won[0]].as_str()));
    assert!(e.taken_at.is_some());
}

#[tokio::test]
async fn an_untaken_item_is_queued_again_and_a_linked_one_names_its_task() {
    let (_db, s) = store().await;
    let item = s.post_backlog(new_item("demo", "t", &[])).await.unwrap();
    assert!(s.take_backlog(item.item.id, "muse").await.unwrap());
    assert!(!s.take_backlog(item.item.id, "codex").await.unwrap());
    s.untake_backlog(item.item.id).await.unwrap();
    let e = &s.list_backlog(None, None).await.unwrap()[0];
    assert_eq!(
        (e.state, e.taken_by.as_deref()),
        (BacklogState::Queued, None)
    );

    assert!(s.take_backlog(item.item.id, "codex").await.unwrap());
    let task = s.create_task(new_task()).await.unwrap();
    s.link_backlog_task(item.item.id, task.id).await.unwrap();
    let e = &s.list_backlog(None, None).await.unwrap()[0];
    assert_eq!(e.task_id, Some(task.id));
    // a taken item cannot be untaken once it has a task
    assert!(s.untake_backlog(item.item.id).await.is_err());
}

#[tokio::test]
async fn only_a_queued_item_can_be_dropped() {
    let (_db, s) = store().await;
    let a = s.post_backlog(new_item("demo", "a", &[])).await.unwrap();
    let b = s.post_backlog(new_item("demo", "b", &[])).await.unwrap();
    s.drop_backlog(a.item.id).await.unwrap();
    assert!(!s.take_backlog(a.item.id, "muse").await.unwrap());
    assert!(matches!(
        s.drop_backlog(a.item.id).await.unwrap_err(),
        AvError::IllegalState(_)
    ));
    assert!(s.take_backlog(b.item.id, "muse").await.unwrap());
    assert!(matches!(
        s.drop_backlog(b.item.id).await.unwrap_err(),
        AvError::IllegalState(_)
    ));
    assert!(matches!(
        s.drop_backlog(uuid::Uuid::new_v4()).await.unwrap_err(),
        AvError::NotFound(_)
    ));
}

#[tokio::test]
async fn a_proposed_item_waits_until_it_is_accepted() {
    let (_db, s) = store().await;
    let p = s
        .post_backlog(NewBacklogItem {
            propose: true,
            why: Some("  not sure the cache is worth it  ".into()),
            ..new_item("demo", "Cache packs", &[])
        })
        .await
        .unwrap();
    assert_eq!(p.state, BacklogState::Proposed);
    assert_eq!(p.why.as_deref(), Some("not sure the cache is worth it"));
    assert_eq!(p.accepted_by, None);
    // not queued, so not offered; listed as proposed
    assert!(
        s.list_backlog(Some("demo"), Some(BacklogState::Queued))
            .await
            .unwrap()
            .is_empty()
    );
    let proposed = s
        .list_backlog(Some("demo"), Some(BacklogState::Proposed))
        .await
        .unwrap();
    assert_eq!(proposed.len(), 1);
    // a proposed item cannot be taken
    assert!(!s.take_backlog(p.item.id, "claude").await.unwrap());
    assert_eq!(
        s.get_backlog(p.item.id).await.unwrap().state,
        BacklogState::Proposed
    );

    let a = s.accept_backlog(p.item.id, "vinh").await.unwrap();
    assert_eq!(a.state, BacklogState::Queued);
    assert_eq!(a.accepted_by.as_deref(), Some("vinh"));
    assert!(a.accepted_at.is_some());
    assert_eq!(a.why.as_deref(), Some("not sure the cache is worth it"));

    // only a proposed item is accepted
    assert!(matches!(
        s.accept_backlog(p.item.id, "vinh").await.unwrap_err(),
        AvError::IllegalState(_)
    ));
    assert!(matches!(
        s.accept_backlog(uuid::Uuid::new_v4(), "vinh")
            .await
            .unwrap_err(),
        AvError::NotFound(_)
    ));
    assert!(matches!(
        s.get_backlog(uuid::Uuid::new_v4()).await.unwrap_err(),
        AvError::NotFound(_)
    ));
}

#[tokio::test]
async fn a_proposed_item_can_be_dropped_and_a_long_why_is_refused() {
    let (_db, s) = store().await;
    let p = s
        .post_backlog(NewBacklogItem {
            propose: true,
            ..new_item("demo", "Maybe", &[])
        })
        .await
        .unwrap();
    assert_eq!(p.why, None);
    s.drop_backlog(p.item.id).await.unwrap();
    assert_eq!(
        s.get_backlog(p.item.id).await.unwrap().state,
        BacklogState::Dropped
    );
    assert!(matches!(
        s.accept_backlog(p.item.id, "vinh").await.unwrap_err(),
        AvError::IllegalState(_)
    ));

    let long = "x".repeat(WHY_MAX + 1);
    assert!(matches!(
        s.post_backlog(NewBacklogItem {
            propose: true,
            why: Some(long),
            ..new_item("demo", "Too long", &[])
        })
        .await
        .unwrap_err(),
        AvError::Invalid(_)
    ));
}

#[tokio::test]
async fn agent_posts_is_a_repo_setting() {
    let (_db, s) = store().await;
    assert_eq!(
        s.repo_settings("demo").await.unwrap().agent_posts,
        AgentPosts::Queue
    );
    let mut set = s.repo_settings("demo").await.unwrap();
    set.agent_posts = AgentPosts::Propose;
    s.set_repo_settings("demo", &set).await.unwrap();
    let got = s.repo_settings("demo").await.unwrap();
    assert_eq!(got.agent_posts, AgentPosts::Propose);
    assert_eq!(got.carry_globs, set.carry_globs);
    assert_eq!(
        s.repo_settings("other").await.unwrap().agent_posts,
        AgentPosts::Queue
    );
}

#[tokio::test]
async fn a_handoff_can_carry_tags_until_it_is_claimed() {
    let (_db, s) = store().await;
    let t = s.create_task(new_task()).await.unwrap();
    // only a handed-off task can carry them
    let err = s
        .set_handoff_needs(t.id, &["image".to_string()], None)
        .await
        .unwrap_err();
    assert!(matches!(err, AvError::IllegalState(_)), "{err:?}");

    s.transition(t.id, lifecycle::Event::Handoff).await.unwrap();
    s.set_handoff_needs(t.id, &["Image".to_string()], Some("grok"))
        .await
        .unwrap();
    let h = s.handed_off_tasks(Some("demo")).await.unwrap();
    assert_eq!(h.len(), 1);
    assert_eq!(h[0].task.id, t.id);
    assert_eq!(h[0].needs, ["image"]);
    assert_eq!(h[0].prefer.as_deref(), Some("grok"));
    assert!(s.handed_off_tasks(Some("other")).await.unwrap().is_empty());

    s.claim_task(t.id, "grok", 600).await.unwrap();
    assert!(s.handed_off_tasks(None).await.unwrap().is_empty());
}

#[tokio::test]
async fn handed_off_tasks_include_untagged_handoffs() {
    let (_db, s) = store().await;
    let tagged = s.create_task(new_task()).await.unwrap();
    let plain = s.create_task(new_task()).await.unwrap();
    for t in [&tagged, &plain] {
        s.transition(t.id, lifecycle::Event::Handoff).await.unwrap();
    }
    s.set_handoff_needs(tagged.id, &["image".to_string()], None)
        .await
        .unwrap();
    let h = s.handed_off_tasks(None).await.unwrap();
    let ids: Vec<_> = h.iter().map(|h| h.task.id).collect();
    assert_eq!(h.len(), 2, "{ids:?}");
    assert!(ids.contains(&tagged.id) && ids.contains(&plain.id));
    let p = h.iter().find(|h| h.task.id == plain.id).unwrap();
    assert!(p.needs.is_empty() && p.prefer.is_none());
    // a handoff whose time was never stamped (made before Plan 9) counts as old
    assert!(p.since < chrono::Utc::now() - chrono::Duration::days(365));
}

#[tokio::test]
async fn tasks_with_slug_finds_it_in_every_repo() {
    let (_db, s) = store().await;
    for repo in ["b", "a"] {
        let mut t = new_task();
        t.repo = repo.into();
        t.title = "Fix login".into();
        s.create_task(t).await.unwrap();
    }
    let found = s.tasks_with_slug("fix-login").await.unwrap();
    let repos: Vec<_> = found.iter().map(|t| t.repo.as_str()).collect();
    assert_eq!(repos, ["a", "b"]);
    assert!(s.tasks_with_slug("nope").await.unwrap().is_empty());
}

#[tokio::test]
async fn abandoned_tasks_are_open_tasks_past_their_lease() {
    let (_db, s) = store().await;
    let expired = s.create_task(new_task()).await.unwrap();
    let live = s.create_task(new_task()).await.unwrap();
    let fresh = s.create_task(new_task()).await.unwrap(); // never leased, just made
    let reviewed = s.create_task(new_task()).await.unwrap();
    // a claim with a zero lease expires at once; a long one does not
    s.transition(expired.id, lifecycle::Event::Handoff)
        .await
        .unwrap();
    s.claim_task(expired.id, "grok", 0).await.unwrap();
    s.transition(live.id, lifecycle::Event::Handoff)
        .await
        .unwrap();
    s.claim_task(live.id, "grok", 3600).await.unwrap();
    s.transition(reviewed.id, lifecycle::Event::Submit)
        .await
        .unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;

    let a = s.abandoned_tasks(None, 3600).await.unwrap();
    let ids: Vec<_> = a.iter().map(|x| x.task.id).collect();
    assert_eq!(ids, [expired.id], "only the open task past its lease");
    assert!(a[0].since <= chrono::Utc::now());
    // a task that never had a lease is abandoned once it is older than the lease length
    let a = s.abandoned_tasks(None, 0).await.unwrap();
    let ids: Vec<_> = a.iter().map(|x| x.task.id).collect();
    assert!(
        ids.contains(&fresh.id) && ids.contains(&expired.id),
        "{ids:?}"
    );
    assert!(!ids.contains(&live.id) && !ids.contains(&reviewed.id));
    assert!(
        s.abandoned_tasks(Some("other"), 0)
            .await
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn a_profile_keeps_its_open_task_limit() {
    let (_db, s) = store().await;
    let mut c = profile("claude", &["code"], &[], 4);
    c.max_open = Some(2);
    s.put_profile(&c).await.unwrap();
    s.put_profile(&profile("grok", &["image"], &[], 2))
        .await
        .unwrap();
    let all = s.list_profiles().await.unwrap();
    let get = |n: &str| all.iter().find(|p| p.name == n).unwrap().max_open;
    assert_eq!(get("claude"), Some(2));
    assert_eq!(get("grok"), None, "no limit");
    // a limit below 1 would mean "never give it work": refused
    for bad in [0, -1] {
        c.max_open = Some(bad);
        assert!(matches!(s.put_profile(&c).await, Err(AvError::Invalid(_))));
    }
}

#[tokio::test]
async fn open_tasks_are_counted_per_owner_across_repos() {
    let (_db, s) = store().await;
    for repo in ["a", "b"] {
        let mut t = new_task();
        t.repo = repo.into();
        s.create_task(t).await.unwrap();
    }
    let submitted = s.create_task(new_task()).await.unwrap();
    s.transition(submitted.id, lifecycle::Event::Submit)
        .await
        .unwrap();
    let mut other = new_task();
    other.owner_agent = "grok".into();
    s.create_task(other).await.unwrap();
    // in review waits on other agents, so it does not count
    assert_eq!(s.count_open_tasks("claude").await.unwrap(), 2);
    assert_eq!(s.count_open_tasks("grok").await.unwrap(), 1);
    assert_eq!(s.count_open_tasks("nobody").await.unwrap(), 0);
}
