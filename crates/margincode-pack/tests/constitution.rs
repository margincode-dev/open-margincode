use crate::common::*;
use margincode_core::*;
use margincode_pack::BundleRequest;

async fn add(fx: &Fx, level: ConstitutionLevel<'_>, title: &str, text: &str, check: Option<&str>) {
    fx.store
        .amend_constitution(
            level,
            &ArticleChange::Add {
                title: title.into(),
                text: text.into(),
                check: check.map(str::to_owned),
            },
            "user",
            "we agreed",
        )
        .await
        .unwrap();
}

const OWN: ConstitutionLevel<'static> = ConstitutionLevel::Repo("demo");

#[tokio::test]
async fn golden_pack_with_a_shared_article() {
    let fx = fixture().await;
    add(
        &fx,
        OWN,
        "Never log a provider key",
        "No key in a log line, ever.",
        None,
    )
    .await;
    add(
        &fx,
        OWN,
        "Lint passes",
        "Run the linter before review.",
        Some("make lint"),
    )
    .await;
    add(
        &fx,
        ConstitutionLevel::Shared,
        "Write in English",
        "Code, records and docs.",
        None,
    )
    .await;
    fx.store
        .add_convention("demo", "app/**/*.py", "use type hints everywhere")
        .await
        .unwrap();
    let pack = fx
        .packer
        .build(req(&fx.task, 2500, "golden"))
        .await
        .unwrap();
    let md = &pack.markdown;
    assert!(
        md.starts_with("## Task\n") && md.contains("\n\n## Constitution (v2, organisation v1; it outranks conventions and records)\n- org:1 Write in English\n"),
        "{md}"
    );
    assert!(
        md.find("## Constitution").unwrap() < md.find("## Map").unwrap(),
        "{md}"
    );
    let conventions = &md[md.find("## Conventions").unwrap()..];
    assert!(
        !conventions.contains("art:") && !conventions.contains("Never log"),
        "{md}"
    );
    let got = crate::pack::normalize(md);
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/constitution.md");
    if std::env::var("UPDATE_GOLDEN").is_ok() {
        std::fs::write(path, &got).unwrap();
    }
    let want = std::fs::read_to_string(path)
        .expect("golden file missing: run with UPDATE_GOLDEN=1 and review it");
    assert_eq!(got, want);
}

#[tokio::test]
async fn a_task_that_changed_nothing_still_gets_the_constitution() {
    let fx = fixture().await;
    add(
        &fx,
        OWN,
        "Never log a provider key",
        "No key in a log line.",
        Some("make lint"),
    )
    .await;
    let fresh = fx.store.create_task(new_task("Plan only")).await.unwrap();
    let pack = fx.packer.build(req(&fresh, 2500, "fresh")).await.unwrap();
    assert!(
        pack.markdown.contains(
            "## Constitution (v1; it outranks conventions and records)\n- art:1 Never log a provider key [check]"
        ),
        "{}",
        pack.markdown
    );
}

#[tokio::test]
async fn no_articles_no_section() {
    let fx = fixture().await;
    let pack = fx.packer.build(req(&fx.task, 2500, "none")).await.unwrap();
    assert!(!pack.markdown.contains("Constitution"), "{}", pack.markdown);
    add(&fx, OWN, "Rule", "text", None).await;
    fx.store
        .amend_constitution(OWN, &ArticleChange::Repeal { number: 1 }, "user", "gone")
        .await
        .unwrap();
    let pack = fx
        .packer
        .build(req(&fx.task, 2500, "repealed"))
        .await
        .unwrap();
    assert!(!pack.markdown.contains("Constitution"), "{}", pack.markdown);
}

#[tokio::test]
async fn the_ledger_sends_it_once_and_again_after_an_amendment_at_either_level() {
    let fx = fixture().await;
    add(
        &fx,
        OWN,
        "Never log a provider key",
        "No key in a log line.",
        None,
    )
    .await;
    let pack = || async {
        fx.packer
            .build(req(&fx.task, 2500, "s"))
            .await
            .unwrap()
            .markdown
    };
    assert!(pack().await.contains("- art:1 Never log a provider key"));
    let again = pack().await;
    assert!(
        again.contains("## Constitution (v1; it outranks conventions and records)\n- ↩ const:v1 (sent earlier)\n"),
        "{again}"
    );
    add(&fx, OWN, "Lint passes", "Run the linter.", None).await;
    let amended = pack().await;
    assert!(
        amended.contains("(v2; it outranks") && amended.contains("- art:2 Lint passes"),
        "{amended}"
    );
    add(&fx, ConstitutionLevel::Shared, "Org rule", "For all.", None).await;
    let shared = pack().await;
    assert!(
        shared.contains("(v2, organisation v1; it outranks") && shared.contains("- org:1 Org rule"),
        "{shared}"
    );
    assert!(pack().await.contains("- ↩ const:v2.1 (sent earlier)"));
}

#[tokio::test]
async fn a_budget_too_small_for_the_titles_says_so_in_the_footer() {
    let fx = fixture().await;
    for n in 0..10 {
        add(
            &fx,
            OWN,
            &format!("{n} {}", "a long title for a rule ".repeat(2)),
            "text",
            None,
        )
        .await;
    }
    let mut seen = None;
    for budget in (40..800).step_by(10) {
        let pack = fx
            .packer
            .build(req(&fx.task, budget, &format!("b{budget}")))
            .await
            .unwrap();
        let md = pack.markdown;
        assert!(estimate_tokens_of(&md) <= budget);
        if md.contains("## Task") && !md.contains("## Constitution") {
            assert!(md.contains("constitution v10 not shown ("), "{md}");
            assert!(md.contains(" tokens): av expand const\n"), "{md}");
            assert!(
                !pack.omitted.iter().any(|o| o.contains("art:")),
                "{:?}",
                pack.omitted
            );
            seen = Some(md);
        }
    }
    assert!(
        seen.is_some(),
        "no budget showed the task without the constitution"
    );
}

fn estimate_tokens_of(md: &str) -> u32 {
    margincode_pack::estimate_tokens(md, AgentFamily::Other)
}

#[tokio::test]
async fn expand_answers_an_article_and_the_whole_constitution_in_full() {
    let fx = fixture().await;
    add(
        &fx,
        OWN,
        "Lint passes",
        "Run the linter.",
        Some("make lint"),
    )
    .await;
    add(&fx, ConstitutionLevel::Shared, "Org rule", "For all.", None).await;
    let art = fx.packer.expand(fx.task.id, "art:1").await.unwrap();
    for want in [
        "art:1 Lint passes",
        "Run the linter.",
        "make lint",
        "we agreed",
    ] {
        assert!(art.contains(want), "{want}: {art}");
    }
    let org = fx.packer.expand(fx.task.id, "org:1").await.unwrap();
    assert!(
        org.contains("org:1 Org rule") && org.contains("For all."),
        "{org}"
    );
    let all = fx.packer.expand(fx.task.id, "const").await.unwrap();
    assert!(
        all.find("org:1").unwrap() < all.find("art:1").unwrap(),
        "{all}"
    );
    assert!(
        all.contains("make lint") && all.contains("we agreed"),
        "{all}"
    );
    assert!(fx.packer.expand(fx.task.id, "art:2").await.is_err());
    let row = fx.store.constitution(OWN).await.unwrap().articles[1].convention_id;
    assert!(
        fx.packer
            .expand(fx.task.id, &format!("conv:{}", row.0))
            .await
            .is_err()
    );
}

#[tokio::test]
async fn the_review_checklist_lists_articles_first() {
    let fx = fixture().await;
    fx.store
        .add_convention("demo", "app/**", "aaa convention first by id")
        .await
        .unwrap();
    add(
        &fx,
        OWN,
        "Lint passes",
        "Run the linter.",
        Some("make lint"),
    )
    .await;
    add(&fx, ConstitutionLevel::Shared, "Org rule", "For all.", None).await;
    let b = fx
        .packer
        .pr_bundle(BundleRequest {
            task_id: fx.task.id,
            budget_tokens: 8000,
            family: AgentFamily::Other,
        })
        .await
        .unwrap();
    let md = &b.markdown;
    let org = md.find("- [ ] org:1 Org rule").expect(md);
    let art = md.find("- [ ] art:1 Lint passes").expect(md);
    let conv = md.find("- [ ] conv:").expect(md);
    assert!(org < art && art < conv, "{md}");
}
