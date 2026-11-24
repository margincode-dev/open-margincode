use margincode_core::*;
use margincode_store::PgStore;
use margincode_testutil::TestDb;

async fn store() -> (TestDb, PgStore) {
    let db = TestDb::new().await;
    let s = PgStore::connect(&db.url).await.unwrap();
    (db, s)
}

const DEMO: ConstitutionLevel<'static> = ConstitutionLevel::Repo("demo");

fn add(title: &str, text: &str, check: Option<&str>) -> ArticleChange {
    ArticleChange::Add {
        title: title.into(),
        text: text.into(),
        check: check.map(str::to_owned),
    }
}

async fn amend(s: &PgStore, level: ConstitutionLevel<'_>, change: ArticleChange) -> Amendment {
    s.amend_constitution(level, &change, "alice", "because")
        .await
        .unwrap()
}

#[tokio::test]
async fn an_amend_keeps_the_number_bumps_the_version_and_retires_the_old_row() {
    let (_db, s) = store().await;
    let first = amend(
        &s,
        DEMO,
        add("Never log a key", "No provider key in a log line.", None),
    )
    .await;
    assert_eq!(
        (first.version, first.article, first.action),
        (1, 1, AmendAction::Add)
    );
    assert_eq!(first.handle, "art:1");
    let old_row = first.convention_id.unwrap();
    let second = amend(
        &s,
        DEMO,
        ArticleChange::Amend {
            number: 1,
            title: None,
            text: Some("No provider key in any log line or error.".into()),
            check: Some("grep -r sk- logs && exit 1 || exit 0".into()),
        },
    )
    .await;
    assert_eq!(
        (second.version, second.article, second.action),
        (2, 1, AmendAction::Amend)
    );
    let c = s.constitution(DEMO).await.unwrap();
    assert_eq!((c.version, c.shared_version), (2, 0));
    assert_eq!(c.articles.len(), 1);
    let a = &c.articles[0];
    assert_eq!(a.title, "Never log a key");
    assert_eq!(a.text, "No provider key in any log line or error.");
    assert!(a.check.is_some());
    assert_ne!(a.convention_id, old_row);
    assert_eq!(a.last.version, 2);
    // the old row is out of force: approval's checks see the new one only
    let rows = s
        .conventions_for("demo", &["src/x.rs".into()])
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].id, a.convention_id);
    assert_eq!(rows[0].handle(), "art:1");
}

#[tokio::test]
async fn a_number_is_never_given_again_after_a_repeal() {
    let (_db, s) = store().await;
    amend(&s, DEMO, add("One", "first", None)).await;
    amend(&s, DEMO, add("Two", "second", None)).await;
    let gone = amend(&s, DEMO, ArticleChange::Repeal { number: 2 }).await;
    assert_eq!(
        (gone.action, gone.convention_id),
        (AmendAction::Repeal, None)
    );
    let third = amend(&s, DEMO, add("Three", "third", None)).await;
    assert_eq!(third.article, 3);
    let c = s.constitution(DEMO).await.unwrap();
    let numbers: Vec<i32> = c.articles.iter().map(|a| a.number).collect();
    assert_eq!(numbers, [1, 3]);
    assert_eq!(c.version, 4);
    let history = s.amendments(DEMO).await.unwrap();
    let actions: Vec<_> = history.iter().map(|h| (h.version, h.action)).collect();
    assert_eq!(
        actions,
        [
            (1, AmendAction::Add),
            (2, AmendAction::Add),
            (3, AmendAction::Repeal),
            (4, AmendAction::Add)
        ]
    );
    assert!(matches!(
        s.amend_constitution(DEMO, &ArticleChange::Repeal { number: 2 }, "a", "again")
            .await,
        Err(AvError::NotFound(_))
    ));
}

#[tokio::test]
async fn limits_are_checked_before_anything_is_written() {
    let (_db, s) = store().await;
    let long = |n: usize| "x".repeat(n);
    for (change, reason) in [
        (add(&long(61), "text", None), "why"),
        (add("title", &long(401), None), "why"),
        (add(" ", "text", None), "why"),
        (add("title", "", None), "why"),
        (add("title", "text", None), ""),
        (add("title", "text", None), &long(201)),
    ] {
        let got = s.amend_constitution(DEMO, &change, "alice", reason).await;
        assert!(matches!(got, Err(AvError::IllegalState(_))), "{got:?}");
    }
    let got = s
        .amend_constitution(DEMO, &add(&long(60), &long(400), None), "alice", &long(200))
        .await;
    assert!(got.is_ok(), "{got:?}");
    let c = s.constitution(DEMO).await.unwrap();
    assert_eq!((c.version, c.articles.len()), (1, 1));
    let unchanged = ArticleChange::Amend {
        number: 1,
        title: None,
        text: None,
        check: None,
    };
    assert!(matches!(
        s.amend_constitution(DEMO, &unchanged, "alice", "nothing")
            .await,
        Err(AvError::IllegalState(_))
    ));
    assert!(matches!(
        s.amend_constitution(DEMO, &add("t", "x", None), " ", "why")
            .await,
        Err(AvError::Invalid(_))
    ));
    assert_eq!(s.constitution(DEMO).await.unwrap().version, 1);
}

#[tokio::test]
async fn ten_articles_is_the_cap_shared_ones_included() {
    let (_db, s) = store().await;
    for i in 0..3 {
        amend(
            &s,
            ConstitutionLevel::Shared,
            add(&format!("org {i}"), "shared", None),
        )
        .await;
    }
    for i in 0..7 {
        amend(&s, DEMO, add(&format!("own {i}"), "own", None)).await;
    }
    let refused = s
        .amend_constitution(DEMO, &add("one more", "x", None), "alice", "why")
        .await;
    match refused {
        Err(AvError::IllegalState(m)) => assert_eq!(
            m,
            "demo has 10 articles in force (3 from the organisation, 7 its own); the limit is 10: \
             repeal one first"
        ),
        other => panic!("{other:?}"),
    }
    // another repository still has room for its own seven
    amend(&s, ConstitutionLevel::Repo("other"), add("own", "x", None)).await;
    let refused = s
        .amend_constitution(
            ConstitutionLevel::Shared,
            &add("org 4", "x", None),
            "ee",
            "why",
        )
        .await;
    match refused {
        Err(AvError::IllegalState(m)) => assert_eq!(
            m,
            "an organisation article would put more than 10 articles on demo (7 of its own): \
             with 3 from the organisation, each would pass the limit; repeal one there first"
        ),
        other => panic!("{other:?}"),
    }
    amend(&s, DEMO, ArticleChange::Repeal { number: 7 }).await;
    amend(&s, ConstitutionLevel::Shared, add("org 4", "x", None)).await;
    let c = s.constitution(DEMO).await.unwrap();
    assert_eq!(c.articles.len(), 10);
    assert_eq!((c.version, c.shared_version), (8, 4));
}

#[tokio::test]
async fn the_organisation_level_alone_is_capped_at_ten() {
    let (_db, s) = store().await;
    for i in 0..10 {
        amend(
            &s,
            ConstitutionLevel::Shared,
            add(&format!("org {i}"), "x", None),
        )
        .await;
    }
    match s
        .amend_constitution(
            ConstitutionLevel::Shared,
            &add("org 10", "x", None),
            "ee",
            "why",
        )
        .await
    {
        Err(AvError::IllegalState(m)) => assert_eq!(
            m,
            "the organisation has 10 articles in force; the limit is 10: repeal one first"
        ),
        other => panic!("{other:?}"),
    }
}

#[tokio::test]
async fn shared_articles_come_first_and_reach_every_repository() {
    let (_db, s) = store().await;
    amend(&s, DEMO, add("Own rule", "own", Some("true"))).await;
    amend(
        &s,
        ConstitutionLevel::Shared,
        add("Shared rule", "shared", None),
    )
    .await;
    let c = s.constitution(DEMO).await.unwrap();
    let handles: Vec<&str> = c.articles.iter().map(|a| a.handle.as_str()).collect();
    assert_eq!(handles, ["org:1", "art:1"]);
    assert!(c.articles[0].shared && !c.articles[1].shared);
    assert_eq!((c.version, c.shared_version), (1, 1));
    let shared = s.constitution(ConstitutionLevel::Shared).await.unwrap();
    assert_eq!(shared.articles.len(), 1);
    assert_eq!(shared.version, 1);
    let other = s
        .constitution(ConstitutionLevel::Repo("other"))
        .await
        .unwrap();
    assert_eq!(other.articles.len(), 1);
    assert_eq!((other.version, other.shared_version), (0, 1));
    let history = s.amendments(DEMO).await.unwrap();
    let handles: Vec<&str> = history.iter().map(|a| a.handle.as_str()).collect();
    assert_eq!(handles, ["art:1", "org:1"]);
    let rows = s.conventions_for("other", &["a.py".into()]).await.unwrap();
    assert_eq!(rows.len(), 1);
    assert!(rows[0].is_shared());
    assert_eq!(rows[0].handle(), "org:1");
}

#[tokio::test]
async fn articles_are_not_conventions_to_list_or_remove() {
    let (_db, s) = store().await;
    let conv = s
        .add_convention("demo", "app/**", "type hints")
        .await
        .unwrap();
    let art = amend(&s, DEMO, add("A rule", "text", None)).await;
    amend(&s, ConstitutionLevel::Shared, add("Shared", "text", None)).await;
    let listed = s.conventions("demo").await.unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, conv);
    assert_eq!(listed[0].article, None);
    let applies = s
        .conventions_for("demo", &["app/x.py".into()])
        .await
        .unwrap();
    let handles: Vec<String> = applies.iter().map(Convention::handle).collect();
    assert_eq!(
        handles,
        ["org:1", "art:1", format!("conv:{}", conv.0).as_str()]
    );
    assert!(s.conventions_for("demo", &[]).await.unwrap().is_empty());
    let row = art.convention_id.unwrap();
    assert!(matches!(
        s.remove_convention("demo", row).await,
        Err(AvError::IllegalState(_))
    ));
    assert!(matches!(
        s.set_convention_check(row, Some("false")).await,
        Err(AvError::NotFound(_))
    ));
    assert_eq!(s.constitution(DEMO).await.unwrap().articles.len(), 2);
}

#[tokio::test]
async fn the_schema_holds_an_article_shape_and_no_other_row_without_a_repo() {
    let (_db, s) = store().await;
    let bare =
        sqlx::query("INSERT INTO conventions (repo, scope_glob, text) VALUES ('', '**', 'x')")
            .execute(s.pool())
            .await;
    assert!(bare.is_err());
    let scoped = sqlx::query(
        "INSERT INTO conventions (repo, scope_glob, text, required, article, title)
         VALUES ('demo', 'src/**', 'x', true, 1, 't')",
    )
    .execute(s.pool())
    .await;
    assert!(scoped.is_err());
    let shared = sqlx::query(
        "INSERT INTO conventions (repo, scope_glob, text, required, article, title)
         VALUES ('', '**', 'x', true, 1, 't')",
    )
    .execute(s.pool())
    .await;
    assert!(shared.is_ok());
    let twice = sqlx::query(
        "INSERT INTO conventions (repo, scope_glob, text, required, article, title)
         VALUES ('', '**', 'y', true, 1, 'u')",
    )
    .execute(s.pool())
    .await;
    assert!(twice.is_err());
}

#[tokio::test]
async fn a_convention_with_a_check_needs_required_and_a_refused_one_leaves_no_row() {
    let (_db, s) = store().await;
    let c = |required, check: Option<&str>| NewConvention {
        repo: "demo".into(),
        scope_glob: "app/**".into(),
        text: "lint".into(),
        required,
        check: check.map(str::to_owned),
    };
    assert!(matches!(
        s.add_convention_with(&c(false, Some("ruff"))).await,
        Err(AvError::Invalid(_))
    ));
    assert!(s.conventions("demo").await.unwrap().is_empty());
    let id = s
        .add_convention_with(&c(true, Some(" ruff ")))
        .await
        .unwrap();
    let listed = s.conventions("demo").await.unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, id);
    assert!(listed[0].required);
    assert_eq!(listed[0].check_command.as_deref(), Some("ruff"));
}
