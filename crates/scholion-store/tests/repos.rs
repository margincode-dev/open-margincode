//! The `repos` table: every imported repo, where it came from and when it was added.

use scholion_core::*;
use scholion_store::PgStore;
use scholion_testutil::TestDb;

#[tokio::test(flavor = "multi_thread")]
async fn repos_are_listed_by_name_and_a_second_register_keeps_the_first_source() {
    let db = TestDb::new().await;
    let store = PgStore::connect(&db.url).await.unwrap();
    assert!(store.list_repos().await.unwrap().is_empty());
    store
        .register_repo("zeta", Some("/src/zeta"))
        .await
        .unwrap();
    store.register_repo("alpha", None).await.unwrap();
    // a start-up backfill (no source known) never overwrites what the import recorded
    store.register_repo("zeta", None).await.unwrap();
    let repos = store.list_repos().await.unwrap();
    let names: Vec<_> = repos.iter().map(|r| r.name.as_str()).collect();
    assert_eq!(names, ["alpha", "zeta"]);
    assert_eq!(repos[0].git_source, None);
    assert_eq!(repos[1].git_source.as_deref(), Some("/src/zeta"));
    // a backfill that does know the source fills in a missing one
    store
        .register_repo("alpha", Some("/src/alpha"))
        .await
        .unwrap();
    assert_eq!(
        store.list_repos().await.unwrap()[0].git_source.as_deref(),
        Some("/src/alpha")
    );
    assert!(store.register_repo(" ", None).await.is_err());
}
