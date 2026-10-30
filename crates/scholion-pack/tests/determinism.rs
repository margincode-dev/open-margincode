//! Plan 6 E2: the same state gives the same pack. The fixture is full of ties (near-identical
//! handlers, records with equal scores created within a second) and the budget cuts some of them,
//! so ids or write order would show if they decided anything. Each pack gets a fresh database,
//! where record ids are random and symbol ids follow the order files were indexed in.

mod common;

use std::path::Path;
use std::sync::Arc;

use common::*;
use scholion_core::*;
use scholion_index::{CodeIndex, PgIndex};
use scholion_pack::Packer;
use scholion_store::PgStore;
use scholion_testutil::TestDb;

const HANDLERS: usize = 12;

fn files() -> Vec<(String, String)> {
    let mut out = vec![(
        "app/greeter.py".to_owned(),
        "def greet(name):\n    return 'hello ' + name\n".to_owned(),
    )];
    for i in 0..HANDLERS {
        out.push((
            format!("app/handler_{i:02}.py"),
            format!(
                "from greeter import greet\n\ndef handle_greeting_{i:02}(name):\n    return greet(name)\n"
            ),
        ));
    }
    out
}

/// Builds the state in a fresh database and returns the pack text with record ids blanked.
/// `reverse` indexes the files in the opposite order; `cold` leaves the task without changes,
/// so it has no seeds and the best text hits stand in.
async fn pack(reverse: bool, cold: bool) -> String {
    let db = TestDb::new().await;
    let store = Arc::new(PgStore::connect(&db.url).await.unwrap());
    let index = Arc::new(PgIndex::new(store.pool().clone()));
    let vcs = Arc::new(FakeVcs::default());
    let mut files = files();
    let refs: Vec<(&str, &str)> = files
        .iter()
        .map(|(p, c)| (p.as_str(), c.as_str()))
        .collect();
    vcs.set("base", &refs);
    if !cold {
        files[0].1 = "def greet(name):\n    return 'HELLO ' + name.upper()\n".to_owned();
    }
    let refs: Vec<(&str, &str)> = files
        .iter()
        .map(|(p, c)| (p.as_str(), c.as_str()))
        .collect();
    vcs.set("head", &refs);
    if reverse {
        files.reverse();
    }

    let touched: Vec<String> = if cold {
        vec![]
    } else {
        vec!["app/greeter.py".into()]
    };
    let make_task = async || {
        let t = store
            .create_task(NewTask {
                repo: "demo".into(),
                title: "Louder greeting".into(),
                goal: "Make every greeting handler louder".into(),
                owner_agent: "claude".into(),
            })
            .await
            .unwrap();
        store
            .set_workspace(t.id, Path::new("/ws/demo/louder"), &ChangeId("base".into()))
            .await
            .unwrap();
        store
            .set_head(t.id, &ChangeId("head".into()), &touched)
            .await
            .unwrap();
        store
            .add_record(record(
                t.id,
                RecordKind::Intent,
                "Make every greeting handler louder",
                "",
                vec![],
            ))
            .await
            .unwrap();
        t
    };
    let task = make_task().await;
    for i in 0..HANDLERS {
        let peer = store
            .create_task(NewTask {
                repo: "demo".into(),
                title: format!("Handler {i:02} cleanup"),
                goal: format!("Tidy greeting handler {i:02}"),
                owner_agent: "claude".into(),
            })
            .await
            .unwrap();
        let path = format!("app/handler_{i:02}.py");
        store
            .set_head(
                peer.id,
                &ChangeId("head".into()),
                std::slice::from_ref(&path),
            )
            .await
            .unwrap();
        for kind in [RecordKind::Decision, RecordKind::Attempt] {
            store
                .add_record(record(
                    peer.id,
                    kind,
                    &format!("greeting handler {i:02}: keep greet() as the only formatter"),
                    "",
                    vec![anchor("app/greeter.py", Some("greet")), anchor(&path, None)],
                ))
                .await
                .unwrap();
        }
    }
    pin_records(&store).await;
    let task = store.get_task(task.id).await.unwrap();

    index
        .index_paths(
            "demo",
            &task.slug,
            files
                .into_iter()
                .map(|(p, c)| (p, c.into_bytes()))
                .collect(),
        )
        .await
        .unwrap();

    let packer = Packer::new(vcs, store, index);
    let pack = packer.build(req(&task, 2000, "s")).await.unwrap();
    blank(&pack.markdown, "rec:")
}

/// Record ids and times are part of the state, so a run cannot repeat them; this sets both from
/// each record's content. The id matters beyond its text: its tokens count against the budget.
/// The time: one day and then an hour apart, as far older records differ by less than an f32
/// step in recency, and whole-second ages would let the clock decide.
async fn pin_records(store: &PgStore) {
    let mut tx = store.pool().begin().await.unwrap();
    for sql in [
        "CREATE TEMP TABLE pinned AS
         SELECT r.*, row_number() OVER (ORDER BY summary, kind) AS n FROM records r",
        "CREATE TEMP TABLE moved AS SELECT id AS old,
                (lpad(to_hex(n), 8, '0') || '-0000-4000-8000-000000000000')::uuid AS new
         FROM pinned",
        "UPDATE pinned p SET id = m.new, created_at = now() - interval '1 day' - n * interval '1 hour'
         FROM moved m WHERE p.id = m.old",
        "ALTER TABLE pinned DROP COLUMN n, DROP COLUMN tsv",
        "DO $$ BEGIN EXECUTE (
           SELECT format('INSERT INTO records (%1$s) SELECT %1$s FROM pinned',
                         string_agg(quote_ident(column_name), ', ' ORDER BY ordinal_position))
           FROM information_schema.columns WHERE table_name = 'pinned');
         END $$",
        "UPDATE anchors a SET record_id = m.new FROM moved m WHERE a.record_id = m.old",
        "DELETE FROM records WHERE id IN (SELECT old FROM moved)",
    ] {
        sqlx::query(sql).execute(&mut *tx).await.unwrap();
    }
    tx.commit().await.unwrap();
}

/// Replaces the id after each `prefix` (`rec:1a2b3c4d`, `sym:12`) with `X`.
fn blank(md: &str, prefix: &str) -> String {
    let mut out = String::new();
    let mut rest = md;
    while let Some(i) = rest.find(prefix) {
        out.push_str(&rest[..i + prefix.len()]);
        out.push('X');
        rest = rest[i + prefix.len()..].trim_start_matches(|c: char| c.is_ascii_alphanumeric());
    }
    out.push_str(rest);
    out.lines()
        .map(|l| match l.split_once("tokens:") {
            Some((head, _)) => format!("{head}tokens: N"),
            None => l.to_owned(),
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[tokio::test]
async fn the_same_writes_give_the_same_pack() {
    let (a, b) = (pack(false, false).await, pack(false, false).await);
    assert!(
        a.contains("handle_greeting_"),
        "no handlers in the pack: {a}"
    );
    assert!(a.contains("keep greet()"), "no records in the pack: {a}");
    assert_eq!(a, b);
}

/// Symbol ids differ with the indexing order; which symbols are shown, and in what order, must
/// not.
#[tokio::test]
async fn indexing_order_does_not_change_the_pack() {
    for cold in [false, true] {
        let (a, b) = (pack(false, cold).await, pack(true, cold).await);
        assert!(
            a.contains("handle_greeting_"),
            "no handlers in the pack: {a}"
        );
        assert_eq!(blank(&a, "sym:"), blank(&b, "sym:"), "cold: {cold}");
    }
}

/// Records written in one burst (an agent recording at submit) have equal ages in whole seconds,
/// so equal scores; they list newest first, not in the order of their random ids.
#[tokio::test]
async fn records_written_together_list_newest_first() {
    let fx = fixture().await;
    let peer = fx.store.create_task(new_task("Peer")).await.unwrap();
    let summaries: Vec<String> = (0..6).map(|i| format!("burst note {i}")).collect();
    for s in &summaries {
        fx.store
            .add_record(record(
                peer.id,
                RecordKind::Note,
                s,
                "",
                vec![anchor("app/greeter.py", None)],
            ))
            .await
            .unwrap();
    }
    let pack = fx.packer.build(req(&fx.task, 4000, "s")).await.unwrap();
    let shown: Vec<usize> = summaries
        .iter()
        .map(|s| pack.markdown.find(s.as_str()).expect(s))
        .collect();
    let mut newest_first = shown.clone();
    newest_first.sort_unstable_by(|a, b| b.cmp(a));
    assert_eq!(shown, newest_first, "{}", pack.markdown);
}
