//! `conflict` verdicts: a record against a rule's `conventions` row, read beside the newest
//! effective verdict and in force only while the rule's row is.

use margincode_core::standing::VerdictKind;
use margincode_core::*;
use margincode_store::{NewDream, NewVerdict, PgStore, VerdictRow};
use margincode_testutil::TestDb;
use serde_json::json;
use uuid::Uuid;

async fn store() -> (TestDb, PgStore) {
    let db = TestDb::new().await;
    let s = PgStore::connect(&db.url).await.unwrap();
    (db, s)
}

const DEMO: ConstitutionLevel<'static> = ConstitutionLevel::Repo("demo");

async fn record(s: &PgStore, summary: &str) -> Record {
    let t = s
        .create_task(NewTask {
            repo: "demo".into(),
            title: format!("work on {summary}"),
            goal: "goal".into(),
            owner_agent: "claude".into(),
        })
        .await
        .unwrap();
    s.add_record(NewRecord {
        task_id: t.id,
        kind: RecordKind::Decision,
        summary: summary.into(),
        body: String::new(),
        change_id: None,
        supersedes: None,
        author_agent: "claude".into(),
        anchors: vec![Anchor {
            path: "app/api/handler.py".into(),
            symbol: None,
        }],
    })
    .await
    .unwrap()
}

async fn article(s: &PgStore, level: ConstitutionLevel<'_>, title: &str) -> ConventionId {
    let change = ArticleChange::Add {
        title: title.into(),
        text: format!("{title}, in any handler."),
        check: None,
    };
    s.amend_constitution(level, &change, "alice", "because")
        .await
        .unwrap()
        .convention_id
        .unwrap()
}

async fn run(s: &PgStore) -> Uuid {
    s.create_dream(NewDream {
        repo: "demo",
        mode: "review",
        area: "",
        agent: "dreamer",
        instructions_sha: None,
        worklist: json!({}),
        lease_secs: 3600,
    })
    .await
    .unwrap()
    .id
}

async fn end(s: &PgStore, dream: Uuid) {
    s.end_dream(dream, "done", json!({})).await.unwrap();
}

fn verdict<'a>(rec: &Record, kind: &'a str, dream: Option<Uuid>) -> NewVerdict<'a> {
    NewVerdict {
        record: rec.id,
        verdict: kind,
        reason: "",
        canonical: None,
        retracts: None,
        evidence: json!([]),
        anchor_hashes: json!([]),
        author_agent: "dreamer",
        dream,
        convention: None,
        against: None,
    }
}

async fn conflict(s: &PgStore, rec: &Record, rule: ConventionId, reason: &str) -> VerdictRow {
    let dream = run(s).await;
    let v = s
        .insert_verdict(NewVerdict {
            reason,
            against: Some(rule),
            ..verdict(rec, "conflict", Some(dream))
        })
        .await
        .unwrap();
    end(s, dream).await;
    v
}

async fn agree(s: &PgStore, v: &VerdictRow, yes: bool) -> VerdictRow {
    s.add_verdict_review(v.id, "owner", None, yes, &json!([]))
        .await
        .unwrap()
}

async fn facts(s: &PgStore, rec: &Record) -> margincode_core::standing::OriginFacts {
    s.record_origins(&[rec.id])
        .await
        .unwrap()
        .remove(&rec.id)
        .unwrap()
}

#[tokio::test]
async fn an_agreed_conflict_is_read_beside_the_standing_verdict_and_never_in_its_place() {
    let (_db, s) = store().await;
    let rule = article(&s, DEMO, "Never log a request body").await;

    let rec = record(&s, "log the request body on error").await;
    let disputed = s
        .insert_verdict(NewVerdict {
            reason: "the handler logs nothing",
            ..verdict(&rec, "disputed", None)
        })
        .await
        .unwrap();
    agree(&s, &disputed, true).await;
    let c = agree(
        &s,
        &conflict(&s, &rec, rule, "says to log request bodies").await,
        true,
    )
    .await;
    assert_eq!(c.against.as_deref(), Some("art:1"));
    let f = facts(&s, &rec).await;
    assert_eq!(
        f.verdict.as_ref().map(|v| v.kind),
        Some(VerdictKind::Disputed),
        "a newer agreed conflict does not take the disputed verdict's place"
    );
    assert_eq!(f.overruled.len(), 1, "{:?}", f.overruled);
    assert_eq!(f.overruled[0].rule, "art:1");
    assert_eq!(f.overruled[0].reason, "says to log request bodies");
    assert_eq!(f.overruled[0].id, c.id);

    let confirmed = s
        .insert_verdict(verdict(&rec, "confirmed", None))
        .await
        .unwrap();
    agree(&s, &confirmed, true).await;
    let f = facts(&s, &rec).await;
    assert_eq!(
        f.verdict.as_ref().map(|v| v.kind),
        Some(VerdictKind::Confirmed)
    );
    assert_eq!(
        f.overruled.len(),
        1,
        "the code agreeing with the record does not clear an overruling"
    );

    let shown = record(&s, "bodies go to the audit log").await;
    let hidden = record(&s, "bodies go to the audit log, again").await;
    let dup = s
        .insert_verdict(NewVerdict {
            canonical: Some(shown.id),
            ..verdict(&hidden, "duplicate", None)
        })
        .await
        .unwrap();
    agree(&s, &dup, true).await;
    agree(
        &s,
        &conflict(&s, &hidden, rule, "says to log bodies").await,
        true,
    )
    .await;
    let f = facts(&s, &hidden).await;
    assert_eq!(
        f.verdict.as_ref().map(|v| v.kind),
        Some(VerdictKind::Duplicate),
        "a duplicate stays hidden under a newer conflict"
    );
}

#[tokio::test]
async fn overrulings_come_organisation_first_then_articles_then_conventions() {
    let (_db, s) = store().await;
    let conv = s
        .add_convention("demo", "app/api/**", "handlers log no bodies")
        .await
        .unwrap();
    let own = article(&s, DEMO, "Never log a request body").await;
    let org = article(&s, ConstitutionLevel::Shared, "Log no personal data").await;
    let rec = record(&s, "log the request body on error").await;
    for rule in [conv, own, org] {
        agree(&s, &conflict(&s, &rec, rule, "logs bodies").await, true).await;
    }
    let rules: Vec<String> = facts(&s, &rec)
        .await
        .overruled
        .into_iter()
        .map(|o| o.rule)
        .collect();
    assert_eq!(
        rules,
        ["org:1", "art:1", format!("conv:{}", conv.0).as_str()]
    );
}

#[tokio::test]
async fn only_an_agreed_conflict_on_a_rule_in_force_overrules() {
    let (_db, s) = store().await;
    let conv = s
        .add_convention("demo", "app/**", "handlers log no bodies")
        .await
        .unwrap();
    let rule = article(&s, DEMO, "Never log a request body").await;

    let proposed = record(&s, "proposed").await;
    let v = conflict(&s, &proposed, rule, "logs bodies").await;
    assert_eq!(v.state, "proposed");
    assert!(facts(&s, &proposed).await.overruled.is_empty());
    let waiting: Vec<Uuid> = s
        .proposed_for_verify("demo", 100)
        .await
        .unwrap()
        .iter()
        .map(|v| v.id)
        .collect();
    assert!(
        !waiting.contains(&v.id),
        "no verify run is handed a conflict"
    );

    let rejected = record(&s, "rejected").await;
    agree(
        &s,
        &conflict(&s, &rejected, rule, "logs bodies").await,
        false,
    )
    .await;
    assert!(facts(&s, &rejected).await.overruled.is_empty());

    let retracted = record(&s, "retracted").await;
    let v = agree(
        &s,
        &conflict(&s, &retracted, rule, "logs bodies").await,
        true,
    )
    .await;
    s.insert_verdict(NewVerdict {
        retracts: Some(v.id),
        ..verdict(&retracted, "retracted", None)
    })
    .await
    .unwrap();
    assert!(facts(&s, &retracted).await.overruled.is_empty());

    let removed = record(&s, "removed").await;
    let v = agree(&s, &conflict(&s, &removed, conv, "logs bodies").await, true).await;
    assert_eq!(facts(&s, &removed).await.overruled.len(), 1);
    s.remove_convention("demo", conv).await.unwrap();
    assert!(facts(&s, &removed).await.overruled.is_empty());
    assert_eq!(s.get_verdict(v.id).await.unwrap().state, "lapsed");

    let amended = record(&s, "amended").await;
    let v = agree(&s, &conflict(&s, &amended, rule, "logs bodies").await, true).await;
    assert_eq!(facts(&s, &amended).await.overruled.len(), 1);
    s.amend_constitution(
        DEMO,
        &ArticleChange::Amend {
            number: 1,
            title: None,
            text: Some("No request body in a log line.".into()),
            check: None,
        },
        "alice",
        "plainer",
    )
    .await
    .unwrap();
    assert!(facts(&s, &amended).await.overruled.is_empty());
    let v = s.get_verdict(v.id).await.unwrap();
    assert_eq!(v.state, "lapsed", "an amendment retires the row it names");
    assert_eq!(v.against.as_deref(), Some("art:1"));
    let lapsed: Vec<Uuid> = s
        .list_verdicts("demo", Some("lapsed"), None, None, 100)
        .await
        .unwrap()
        .iter()
        .map(|v| v.id)
        .collect();
    assert!(lapsed.contains(&v.id), "{lapsed:?}");
}

#[tokio::test]
async fn a_waiting_conflict_keeps_its_record_off_the_worklist_until_its_rule_goes() {
    let (_db, s) = store().await;
    let conv = s
        .add_convention("demo", "app/**", "handlers log no bodies")
        .await
        .unwrap();
    let rec = record(&s, "log the request body on error").await;
    let listed = |c: Vec<margincode_store::DreamCandidate>| c.iter().any(|c| c.record.id == rec.id);
    conflict(&s, &rec, conv, "logs bodies").await;
    assert!(!listed(s.dream_candidates("demo", "", 400).await.unwrap()));
    s.remove_convention("demo", conv).await.unwrap();
    assert!(listed(s.dream_candidates("demo", "", 400).await.unwrap()));
}

#[tokio::test]
async fn one_conflict_per_record_and_rule_row_until_it_is_retracted() {
    let (_db, s) = store().await;
    let rule = article(&s, DEMO, "Never log a request body").await;
    let rec = record(&s, "log the request body on error").await;
    let again = async |s: &PgStore| {
        let dream = run(s).await;
        let got = s
            .insert_verdict(NewVerdict {
                reason: "logs bodies",
                against: Some(rule),
                ..verdict(&rec, "conflict", Some(dream))
            })
            .await;
        end(s, dream).await;
        got
    };
    let first = conflict(&s, &rec, rule, "logs bodies").await;
    for (state, agreed) in [("waiting", None), ("rejected", Some(false))] {
        if let Some(yes) = agreed {
            agree(&s, &first, yes).await;
        }
        match again(&s).await {
            Err(AvError::Conflict(why)) => {
                assert!(why.contains(state) && why.contains("art:1"), "{why}")
            }
            other => panic!("{state}: {:?}", other.map(|v| v.id)),
        }
    }
    s.insert_verdict(NewVerdict {
        retracts: Some(first.id),
        ..verdict(&rec, "retracted", None)
    })
    .await
    .unwrap();
    let second = again(&s).await.unwrap();
    agree(&s, &second, true).await;
    match again(&s).await {
        Err(AvError::Conflict(why)) => assert!(why.contains("agreed"), "{why}"),
        other => panic!("{:?}", other.map(|v| v.id)),
    }
    let next = s
        .amend_constitution(
            DEMO,
            &ArticleChange::Amend {
                number: 1,
                title: None,
                text: Some("No request body in a log line.".into()),
                check: None,
            },
            "alice",
            "plainer",
        )
        .await
        .unwrap()
        .convention_id
        .unwrap();
    let dream = run(&s).await;
    s.insert_verdict(NewVerdict {
        reason: "logs bodies",
        against: Some(next),
        ..verdict(&rec, "conflict", Some(dream))
    })
    .await
    .expect("the amended rule is a new row");
}

#[tokio::test]
async fn only_a_conflict_names_a_rule() {
    let (_db, s) = store().await;
    let rule = article(&s, DEMO, "Never log a request body").await;
    let rec = record(&s, "log the request body on error").await;
    let dream = run(&s).await;
    assert!(
        s.insert_verdict(verdict(&rec, "conflict", Some(dream)))
            .await
            .is_err()
    );
    assert!(
        s.insert_verdict(NewVerdict {
            against: Some(rule),
            ..verdict(&rec, "confirmed", None)
        })
        .await
        .is_err()
    );
    let plain = s
        .insert_verdict(verdict(&rec, "confirmed", None))
        .await
        .unwrap();
    assert_eq!(plain.against, None);
    assert!(
        serde_json::to_value(&plain)
            .unwrap()
            .get("against")
            .is_none(),
        "a verdict that names no rule serialises as before"
    );
}
