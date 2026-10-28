mod common;

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use common::*;
use scholion_core::*;
use scholion_index::CodeIndex;
use scholion_pack::{CodeDetail, PackRequest, Packer, estimate_tokens};

/// Scripted `Semantic`. `calls` counts `search` invocations.
struct FakeSemantic {
    hits: Vec<SemanticHit>,
    fail: bool,
    delay: Duration,
    calls: AtomicUsize,
}

#[async_trait]
impl Semantic for FakeSemantic {
    async fn search(&self, _repo: &str, _query: &str, _limit: usize) -> Result<Vec<SemanticHit>> {
        let _ = self.calls.fetch_add(1, Ordering::SeqCst);
        if !self.delay.is_zero() {
            tokio::time::sleep(self.delay).await;
        }
        if self.fail {
            Err(AvError::Semantic("down".into()))
        } else {
            Ok(self.hits.clone())
        }
    }
}

fn record_hit(rec: &Record, score: f32) -> SemanticHit {
    SemanticHit {
        kind: SemanticKind::Record,
        score,
        repo: "demo".into(),
        task_id: Some(rec.task_id),
        record_id: Some(rec.id),
        symbol_id: None,
        convention_id: None,
        path: None,
        change_id: None,
        superseded: false,
    }
}

fn packer_with(fx: &Fx, sem: Arc<FakeSemantic>) -> Packer {
    Packer::with_semantic(
        fx.vcs.clone(),
        fx.store.clone(),
        fx.index.clone(),
        Some(sem),
    )
}

const FAM: AgentFamily = AgentFamily::Other;

fn pos(haystack: &str, needle: &str) -> usize {
    haystack
        .find(needle)
        .unwrap_or_else(|| panic!("{needle:?} not in pack:\n{haystack}"))
}

#[tokio::test]
async fn empty_task_pack_has_task_and_map_and_no_panic() {
    let fx = fixture().await;
    // never got a workspace: no base/head change at all
    let bare = fx.store.create_task(new_task("Bare task")).await.unwrap();
    let pack = fx.packer.build(req(&bare, 2000, "s")).await.unwrap();
    assert!(
        pack.markdown.contains("## Task") && pack.markdown.contains("## Map"),
        "{}",
        pack.markdown
    );
    assert!(pack.markdown.contains("bare-task"));
    // has a workspace but no edits yet (base == head)
    let idle = fx.store.create_task(new_task("Idle task")).await.unwrap();
    fx.store
        .set_workspace(
            idle.id,
            std::path::Path::new("/ws/demo/idle"),
            &ChangeId("base".into()),
        )
        .await
        .unwrap();
    let pack = fx.packer.build(req(&idle, 2000, "s2")).await.unwrap();
    assert!(pack.markdown.contains("## Task") && pack.markdown.contains("## Map"));
    assert!(
        pack.markdown.contains("app/"),
        "cold-start map shows the repo layout:\n{}",
        pack.markdown
    );
}

#[tokio::test]
async fn attempt_on_seed_symbol_outranks_unrelated_note() {
    let fx = fixture().await;
    let t = fx.task.id;
    fx.store
        .add_record(record(
            t,
            RecordKind::Note,
            "unrelated gardening note",
            "",
            vec![],
        ))
        .await
        .unwrap();
    fx.store
        .add_record(record(
            t,
            RecordKind::Attempt,
            "tried memoizing hello; broke tests",
            "",
            vec![anchor("app/greeter.py", Some("Greeter.hello"))],
        ))
        .await
        .unwrap();
    let pack = fx.packer.build(req(&fx.task, 3000, "s")).await.unwrap();
    assert!(
        pos(&pack.markdown, "tried memoizing") < pos(&pack.markdown, "unrelated gardening"),
        "{}",
        pack.markdown
    );
}

#[tokio::test]
async fn superseded_record_absent_from_pack() {
    let fx = fixture().await;
    let t = fx.task.id;
    let old = fx
        .store
        .add_record(record(
            t,
            RecordKind::Decision,
            "use md5 for hashing",
            "",
            vec![anchor("app/greeter.py", None)],
        ))
        .await
        .unwrap();
    let mut newer = record(
        t,
        RecordKind::Decision,
        "use argon2 for hashing",
        "",
        vec![anchor("app/greeter.py", None)],
    );
    newer.supersedes = Some(old.id);
    fx.store.add_record(newer).await.unwrap();
    let pack = fx.packer.build(req(&fx.task, 3000, "s")).await.unwrap();
    assert!(
        pack.markdown.contains("argon2") && !pack.markdown.contains("md5"),
        "{}",
        pack.markdown
    );
}

#[tokio::test]
async fn second_pack_in_same_session_references_instead_of_resending() {
    let fx = fixture().await;
    let first = fx
        .packer
        .build(req(&fx.task, 3000, "sess-1"))
        .await
        .unwrap();
    let second = fx
        .packer
        .build(req(&fx.task, 3000, "sess-1"))
        .await
        .unwrap();
    let other = fx
        .packer
        .build(req(&fx.task, 3000, "sess-2"))
        .await
        .unwrap();
    assert!(!first.markdown.contains("sent earlier"));
    assert!(
        second.markdown.contains("sent earlier"),
        "{}",
        second.markdown
    );
    assert!(
        second.tokens < first.tokens,
        "{} !< {}",
        second.tokens,
        first.tokens
    );
    assert!(
        !other.markdown.contains("sent earlier"),
        "a new session gets everything again"
    );
}

#[tokio::test]
async fn live_work_warning_lists_other_open_task_touching_same_path() {
    let fx = fixture().await;
    let peer = fx
        .store
        .create_task(new_task("Refactor greeter"))
        .await
        .unwrap();
    fx.store
        .set_head(
            peer.id,
            &ChangeId("head".into()),
            &["app/greeter.py".into()],
        )
        .await
        .unwrap();
    let pack = fx.packer.build(req(&fx.task, 3000, "s")).await.unwrap();
    let live = &pack.markdown[pos(&pack.markdown, "## Live work")..];
    assert!(
        live.contains("refactor-greeter") && live.contains("app/greeter.py"),
        "{live}"
    );
    assert!(
        !live.contains("improve-greeting"),
        "own task is not a warning"
    );
}

#[tokio::test]
async fn stale_anchor_record_still_returned_with_flag() {
    let fx = fixture().await;
    let other = fx
        .store
        .create_task(new_task("Earlier work"))
        .await
        .unwrap();
    fx.store
        .add_record(record(
            other.id,
            RecordKind::Decision,
            "hello must stay synchronous",
            "",
            vec![anchor("app/greeter.py", Some("Greeter.renamed_away"))],
        ))
        .await
        .unwrap();
    let pack = fx.packer.build(req(&fx.task, 3000, "s")).await.unwrap();
    let line = pack
        .markdown
        .lines()
        .find(|l| l.contains("hello must stay synchronous"))
        .expect("record returned via its path");
    assert!(line.contains("anchor_stale"), "{line}");
}

#[tokio::test]
async fn anchor_on_existing_non_code_file_is_not_stale_but_missing_file_is() {
    let fx = fixture().await;
    let t = fx.task.id;
    // README.md exists at head but is never indexed (not a code file); docs/gone.md does not exist at all
    fx.store
        .add_record(record(
            t,
            RecordKind::Note,
            "readme explains setup",
            "",
            vec![anchor("README.md", None)],
        ))
        .await
        .unwrap();
    fx.store
        .add_record(record(
            t,
            RecordKind::Note,
            "see the old design doc",
            "",
            vec![anchor("docs/gone.md", None)],
        ))
        .await
        .unwrap();
    fx.store
        .add_record(record(
            t,
            RecordKind::Note,
            "readme symbol claim",
            "",
            vec![anchor("README.md", Some("Intro"))],
        ))
        .await
        .unwrap();
    let pack = fx.packer.build(req(&fx.task, 4000, "s")).await.unwrap();
    let line = |needle: &str| {
        pack.markdown
            .lines()
            .find(|l| l.contains(needle))
            .unwrap_or_else(|| panic!("{needle} missing:\n{}", pack.markdown))
            .to_owned()
    };
    assert!(
        !line("readme explains setup").contains("anchor_stale"),
        "existing non-code file is fine"
    );
    assert!(
        line("see the old design doc").contains("anchor_stale"),
        "a file that is gone is stale"
    );
    assert!(
        line("readme symbol claim").contains("anchor_stale"),
        "a symbol in a non-indexed file cannot be resolved"
    );
}

#[tokio::test]
async fn convention_only_when_scope_matches() {
    let fx = fixture().await;
    fx.store
        .add_convention("demo", "app/**/*.py", "use type hints everywhere")
        .await
        .unwrap();
    fx.store
        .add_convention("demo", "docs/**", "keep docs short")
        .await
        .unwrap();
    let pack = fx.packer.build(req(&fx.task, 3000, "s")).await.unwrap();
    assert!(
        pack.markdown.contains("use type hints"),
        "{}",
        pack.markdown
    );
    assert!(!pack.markdown.contains("keep docs short"));
}

#[tokio::test]
async fn structural_neighbor_and_focus_symbols_appear_in_code() {
    let fx = fixture().await;
    let pack = fx.packer.build(req(&fx.task, 4000, "s")).await.unwrap();
    let code = &pack.markdown[pos(&pack.markdown, "## Code")..];
    assert!(
        code.contains("Greeter.hello") && code.contains("format_name"),
        "seed + 1-hop callee:\n{code}"
    );
    assert!(!code.contains("zebra"), "unrelated symbol stays out");
    let mut r = req(&fx.task, 4000, "s2");
    r.focus = Some("zebra".into());
    let focused = fx.packer.build(r).await.unwrap();
    assert!(focused.markdown.contains("zebra"), "{}", focused.markdown);
}

#[tokio::test]
async fn container_type_does_not_repeat_member_bodies_already_listed() {
    let fx = fixture().await;
    let pack = fx.packer.build(req(&fx.task, 4000, "s")).await.unwrap();
    assert_eq!(
        pack.markdown.matches("return format_name(name)").count(),
        1,
        "{}",
        pack.markdown
    );
    assert!(pack.markdown.contains("Greeter (app/greeter.py:3-8)"));
}

#[tokio::test]
async fn only_symbols_whose_lines_changed_are_seeds() {
    let fx = fixture().await;
    let pack = fx.packer.build(req(&fx.task, 4000, "s")).await.unwrap();
    let code = &pack.markdown[pos(&pack.markdown, "## Code")..];
    assert!(
        code.contains("Greeter.hello"),
        "the edited method is a seed:\n{code}"
    );
    assert!(
        !code.contains("Greeter.goodbye"),
        "an untouched sibling in the same file is not:\n{code}"
    );
    // it still shows up in the file skeleton, which is where siblings belong
    assert!(
        pack.markdown.contains("def goodbye(self, name):"),
        "{}",
        pack.markdown
    );
}

#[tokio::test]
async fn every_symbol_of_a_brand_new_file_is_a_seed() {
    let fx = fixture().await;
    let mut head: Vec<(&str, &str)> = HEAD.to_vec();
    head.push((
        "app/fresh.py",
        "def one():\n    return 1\n\ndef two():\n    return 2\n",
    ));
    fx.vcs.set("head2", &head);
    fx.index
        .index_paths(
            "demo",
            &fx.task.slug,
            vec![(
                "app/fresh.py".to_string(),
                b"def one():\n    return 1\n\ndef two():\n    return 2\n".to_vec(),
            )],
        )
        .await
        .unwrap();
    fx.store
        .set_head(
            fx.task.id,
            &ChangeId("head2".into()),
            &["app/greeter.py".into(), "app/fresh.py".into()],
        )
        .await
        .unwrap();
    let task = fx.store.get_task(fx.task.id).await.unwrap();
    let pack = fx.packer.build(req(&task, 4000, "s")).await.unwrap();
    let code = &pack.markdown[pos(&pack.markdown, "## Code")..];
    assert!(code.contains(" one ") && code.contains(" two "), "{code}");
}

#[tokio::test]
async fn test_code_ranks_below_production_code_of_equal_distance() {
    let fx = fixture().await;
    let (t, p) = (
        "def test_early():\n    assert True\n",
        "def late_production():\n    return 1\n",
    );
    let mut head: Vec<(&str, &str)> = HEAD.to_vec();
    head.extend([("tests/test_early.py", t), ("app/late.py", p)]);
    fx.vcs.set("head3", &head);
    // the test file is indexed FIRST, so it has the lower symbol id: only a real penalty can reorder them
    fx.index
        .index_paths(
            "demo",
            &fx.task.slug,
            vec![("tests/test_early.py".to_string(), t.as_bytes().to_vec())],
        )
        .await
        .unwrap();
    fx.index
        .index_paths(
            "demo",
            &fx.task.slug,
            vec![("app/late.py".to_string(), p.as_bytes().to_vec())],
        )
        .await
        .unwrap();
    fx.store
        .set_head(
            fx.task.id,
            &ChangeId("head3".into()),
            &[
                "app/greeter.py".into(),
                "tests/test_early.py".into(),
                "app/late.py".into(),
            ],
        )
        .await
        .unwrap();
    let task = fx.store.get_task(fx.task.id).await.unwrap();
    let pack = fx.packer.build(req(&task, 4000, "s")).await.unwrap();
    let code = &pack.markdown[pos(&pack.markdown, "## Code")..];
    assert!(
        pos(code, "late_production") < pos(code, "test_early"),
        "{code}"
    );
}

#[tokio::test]
async fn the_goal_is_not_stated_twice() {
    let fx = fixture().await;
    let pack = fx.packer.build(req(&fx.task, 3000, "s")).await.unwrap();
    assert_eq!(
        pack.markdown.matches("Make hello louder").count(),
        1,
        "goal line only; the identical intent record is redundant:\n{}",
        pack.markdown
    );
    // a genuinely different intent record is kept
    fx.store
        .add_record(record(
            fx.task.id,
            RecordKind::Intent,
            "scope: greeter only, not util",
            "",
            vec![],
        ))
        .await
        .unwrap();
    let pack = fx.packer.build(req(&fx.task, 3000, "s2")).await.unwrap();
    assert!(
        pack.markdown.contains("scope: greeter only"),
        "{}",
        pack.markdown
    );
}

#[tokio::test]
async fn a_long_goal_whose_intent_summary_was_clipped_is_not_stated_twice() {
    let fx = fixture().await;
    let goal = format!(
        "Quiet the zebra herd {}",
        "and keep every stripe intact ".repeat(10)
    );
    let t = cold_task(&fx, "Zebra", goal.trim()).await;
    // what start_task records: the first line clipped to 200 characters, the goal as the body
    let summary: String = goal.trim().chars().take(200).collect();
    fx.store
        .add_record(record(
            t.id,
            RecordKind::Intent,
            &summary,
            goal.trim(),
            vec![],
        ))
        .await
        .unwrap();
    for budget in [800, 4000] {
        let pack = fx
            .packer
            .build(req(&t, budget, &format!("long{budget}")))
            .await
            .unwrap();
        assert!(
            pack.markdown.matches("Quiet the zebra herd").count() <= 1,
            "budget {budget}: the auto-recorded intent repeats the goal:\n{}",
            pack.markdown
        );
    }
}

#[tokio::test]
async fn pack_tokens_never_exceed_budget() {
    let fx = fixture().await;
    for (i, budget) in [0u32, 25, 60, 120, 300, 800, 3000, 20000]
        .into_iter()
        .enumerate()
    {
        let pack = fx
            .packer
            .build(req(&fx.task, budget, &format!("b{i}")))
            .await
            .unwrap();
        let actual = estimate_tokens(&pack.markdown, FAM);
        assert!(
            actual <= budget,
            "budget {budget}: pack is {actual} tokens\n{}",
            pack.markdown
        );
        assert_eq!(pack.tokens, actual);
    }
}

#[tokio::test]
async fn tiny_budget_still_lists_what_was_omitted() {
    let fx = fixture().await;
    let pack = fx.packer.build(req(&fx.task, 150, "s")).await.unwrap();
    assert!(estimate_tokens(&pack.markdown, FAM) <= 150);
    assert!(!pack.omitted.is_empty(), "{}", pack.markdown);
}

#[tokio::test]
async fn expand_returns_full_record_and_code() {
    let fx = fixture().await;
    fx.store
        .add_record(record(
            fx.task.id,
            RecordKind::Decision,
            "keep greeting pure",
            "Greeting must not touch IO: tests rely on it.",
            vec![anchor("app/greeter.py", Some("Greeter.hello"))],
        ))
        .await
        .unwrap();
    let pack = fx.packer.build(req(&fx.task, 600, "s")).await.unwrap();
    let rec_id = find_id(&pack.markdown, "rec:").expect("a record id in the pack");
    let sym_id = find_id(&pack.markdown, "sym:").expect("a symbol id in the pack");
    let rec_text = fx.packer.expand(fx.task.id, &rec_id).await.unwrap();
    assert!(
        rec_text.contains("keep greeting pure") || rec_text.contains("Make hello louder"),
        "{rec_text}"
    );
    let code = fx.packer.expand(fx.task.id, &sym_id).await.unwrap();
    assert!(code.contains("def "), "{code}");
    assert!(matches!(
        fx.packer.expand(fx.task.id, "nope:1").await,
        Err(AvError::Invalid(_))
    ));
}

#[tokio::test]
async fn expand_accepts_convention_and_task_ids() {
    let fx = fixture().await;
    let conv = fx
        .store
        .add_convention("demo", "app/**/*.py", "use type hints everywhere")
        .await
        .unwrap();
    let text = fx
        .packer
        .expand(fx.task.id, &format!("conv:{}", conv.0))
        .await
        .unwrap();
    assert!(text.contains("use type hints everywhere"), "{text}");
    assert!(text.contains("app/**/*.py"), "{text}");

    let other = fx
        .store
        .create_task(new_task("Add farewell"))
        .await
        .unwrap();
    let text = fx
        .packer
        .expand(fx.task.id, &format!("task:{}", other.slug))
        .await
        .unwrap();
    assert!(
        text.contains("Add farewell") && text.contains("open"),
        "{text}"
    );
    assert!(matches!(
        fx.packer.expand(fx.task.id, "task:no-such-task").await,
        Err(AvError::NotFound(_))
    ));
    assert!(matches!(
        fx.packer.expand(fx.task.id, "conv:999999").await,
        Err(AvError::NotFound(_))
    ));
}

fn normalize(md: &str) -> String {
    let mut out = String::new();
    let mut rest = md;
    while let Some(i) = rest.find("rec:") {
        out.push_str(&rest[..i]);
        out.push_str("rec:XXXXXXXX");
        rest = &rest[i + 4 + 8..];
    }
    out.push_str(rest);
    out.lines()
        .map(|l| {
            if l.contains("tokens:") {
                l.split("tokens:").next().unwrap().to_owned() + "tokens: N"
            } else {
                l.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

#[tokio::test]
async fn golden_pack_for_fixture_task() {
    let fx = fixture().await;
    fx.store
        .add_record(record(
            fx.task.id,
            RecordKind::Attempt,
            "tried memoizing hello; broke tests",
            "cache keyed on name",
            vec![anchor("app/greeter.py", Some("Greeter.hello"))],
        ))
        .await
        .unwrap();
    fx.store
        .add_convention("demo", "app/**/*.py", "use type hints everywhere")
        .await
        .unwrap();
    let peer = fx
        .store
        .create_task(new_task("Refactor greeter"))
        .await
        .unwrap();
    fx.store
        .set_head(
            peer.id,
            &ChangeId("head".into()),
            &["app/greeter.py".into()],
        )
        .await
        .unwrap();
    let pack = fx
        .packer
        .build(req(&fx.task, 2500, "golden"))
        .await
        .unwrap();
    let got = normalize(&pack.markdown);
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/basic.md");
    if std::env::var("UPDATE_GOLDEN").is_ok() {
        std::fs::write(path, &got).unwrap();
    }
    let want = std::fs::read_to_string(path)
        .expect("golden file missing: run with UPDATE_GOLDEN=1 and review it");
    assert_eq!(got, want);
}

#[tokio::test]
async fn semantic_hit_on_unanchored_other_task_record_is_included() {
    let fx = fixture().await;
    let other = fx.store.create_task(new_task("Side quest")).await.unwrap();
    let rec = fx
        .store
        .add_record(record(
            other.id,
            RecordKind::Note,
            "unanchored other task flower",
            "",
            vec![],
        ))
        .await
        .unwrap();
    let plain = fx
        .packer
        .build(req(&fx.task, 4000, "no-sem"))
        .await
        .unwrap();
    assert!(
        !plain.markdown.contains("unanchored other task flower"),
        "Packer::new does not include an unanchored other-task record:\n{}",
        plain.markdown
    );
    let sem = Arc::new(FakeSemantic {
        hits: vec![record_hit(&rec, 0.8)],
        fail: false,
        delay: Duration::ZERO,
        calls: AtomicUsize::new(0),
    });
    let pack = packer_with(&fx, sem)
        .build(req(&fx.task, 4000, "with-sem"))
        .await
        .unwrap();
    assert!(
        pack.markdown.contains("unanchored other task flower"),
        "{}",
        pack.markdown
    );
}

#[tokio::test]
async fn other_tasks_intents_rank_below_anchored_decisions() {
    let fx = fixture().await;
    let other = fx.store.create_task(new_task("Side quest")).await.unwrap();
    let intent = fx
        .store
        .add_record(record(
            other.id,
            RecordKind::Intent,
            "make hello louder in the side quest",
            "",
            vec![],
        ))
        .await
        .unwrap();
    fx.store
        .add_record(record(
            other.id,
            RecordKind::Decision,
            "format_name strips whitespace first",
            "",
            vec![anchor("app/util.py", Some("format_name"))],
        ))
        .await
        .unwrap();
    // Another task's goal is often worded like ours, so semantic search ranks it high.
    let sem = Arc::new(FakeSemantic {
        hits: vec![record_hit(&intent, 0.95)],
        fail: false,
        delay: Duration::ZERO,
        calls: AtomicUsize::new(0),
    });
    let pack = packer_with(&fx, sem)
        .build(req(&fx.task, 4000, "foreign-intent"))
        .await
        .unwrap();
    let md = &pack.markdown;
    let decision = md.find("format_name strips whitespace").expect(md);
    let intent = md.find("make hello louder in the side quest").expect(md);
    assert!(decision < intent, "{md}");
}

#[tokio::test]
async fn semantic_failure_reports_semantic_and_uses_fts() {
    let fx = fixture().await;
    let other = fx.store.create_task(new_task("Side quest")).await.unwrap();
    // Goal is "Make hello louder"; plainto_tsquery ANDs those lexemes.
    fx.store
        .add_record(record(
            other.id,
            RecordKind::Note,
            "unanchored make hello louder note",
            "",
            vec![],
        ))
        .await
        .unwrap();
    let sem = Arc::new(FakeSemantic {
        hits: vec![],
        fail: true,
        delay: Duration::ZERO,
        calls: AtomicUsize::new(0),
    });
    let pack = packer_with(&fx, sem)
        .build(req(&fx.task, 4000, "fts"))
        .await
        .unwrap();
    assert_eq!(pack.degraded, vec!["semantic".to_string()]);
    assert!(
        pack.markdown.contains("unanchored make hello louder note"),
        "{}",
        pack.markdown
    );
}

#[tokio::test]
async fn blank_focus_and_blank_goal_does_not_search() {
    let fx = fixture().await;
    let mut spec = new_task("Blank goal");
    spec.goal = "  ".into();
    let bare = fx.store.create_task(spec).await.unwrap();
    let sem = Arc::new(FakeSemantic {
        hits: vec![],
        fail: false,
        delay: Duration::ZERO,
        calls: AtomicUsize::new(0),
    });
    let mut r = req(&bare, 2000, "blank");
    r.focus = None;
    let pack = packer_with(&fx, sem.clone()).build(r).await.unwrap();
    assert_eq!(sem.calls.load(Ordering::SeqCst), 0);
    assert!(
        !pack.degraded.iter().any(|d| d == "semantic"),
        "{:?}",
        pack.degraded
    );
}

#[tokio::test]
async fn superseded_hit_is_not_shown() {
    let fx = fixture().await;
    let other = fx.store.create_task(new_task("Side quest")).await.unwrap();
    let old = fx
        .store
        .add_record(record(
            other.id,
            RecordKind::Note,
            "md5 is fine for passwords",
            "",
            vec![],
        ))
        .await
        .unwrap();
    let mut newer = record(
        other.id,
        RecordKind::Decision,
        "use argon2 instead of the old hash",
        "",
        vec![],
    );
    newer.supersedes = Some(old.id);
    fx.store.add_record(newer).await.unwrap();
    let sem = Arc::new(FakeSemantic {
        hits: vec![record_hit(&old, 0.95)],
        fail: false,
        delay: Duration::ZERO,
        calls: AtomicUsize::new(0),
    });
    let pack = packer_with(&fx, sem)
        .build(req(&fx.task, 4000, "sup"))
        .await
        .unwrap();
    assert!(
        !pack.markdown.contains("md5 is fine for passwords"),
        "{}",
        pack.markdown
    );
}

#[tokio::test]
async fn two_builds_with_the_same_hits_are_byte_identical() {
    let fx = fixture().await;
    let other = fx.store.create_task(new_task("Side quest")).await.unwrap();
    let rec = fx
        .store
        .add_record(record(
            other.id,
            RecordKind::Note,
            "stable semantic daisy",
            "",
            vec![],
        ))
        .await
        .unwrap();
    let sem = Arc::new(FakeSemantic {
        hits: vec![record_hit(&rec, 0.7)],
        fail: false,
        delay: Duration::ZERO,
        calls: AtomicUsize::new(0),
    });
    let packer = packer_with(&fx, sem);
    let a = packer.build(req(&fx.task, 4000, "det-1")).await.unwrap();
    let b = packer.build(req(&fx.task, 4000, "det-2")).await.unwrap();
    assert_eq!(a.markdown, b.markdown);
}

#[tokio::test]
async fn slow_semantic_degrades_quickly() {
    let fx = fixture().await;
    let sem = Arc::new(FakeSemantic {
        hits: vec![],
        fail: false,
        delay: Duration::from_secs(2),
        calls: AtomicUsize::new(0),
    });
    let packer = packer_with(&fx, sem);
    let start = Instant::now();
    let pack = packer.build(req(&fx.task, 3000, "slow")).await.unwrap();
    let took = start.elapsed();
    assert!(
        took < Duration::from_secs(1),
        "slow semantic should time out, took {took:?}"
    );
    assert!(
        pack.degraded.iter().any(|d| d == "semantic"),
        "{:?}",
        pack.degraded
    );
}

#[tokio::test]
#[ignore = "timing: run with `cargo test -p scholion-pack --release -- --ignored`"]
async fn warm_build_under_300ms() {
    let fx = fixture().await;
    fx.packer
        .build(req(&fx.task, 4000, "warmup"))
        .await
        .unwrap();
    let start = std::time::Instant::now();
    fx.packer.build(req(&fx.task, 4000, "timed")).await.unwrap();
    let took = start.elapsed();
    assert!(took.as_millis() < 300, "warm build took {took:?}");
    let _ = fx
        .index
        .symbols_in("demo", &fx.task.slug, "app/greeter.py")
        .await
        .unwrap();
}

fn code_hit(sym: SymbolId, path: &str, score: f32) -> SemanticHit {
    SemanticHit {
        kind: SemanticKind::CodeChunk,
        score,
        repo: "demo".into(),
        task_id: None,
        record_id: None,
        symbol_id: Some(sym),
        convention_id: None,
        path: Some(path.into()),
        change_id: None,
        superseded: false,
    }
}

#[tokio::test]
async fn main_scope_code_hits_resolve_to_the_task_scope_symbol() {
    let fx = fixture().await;
    let py: Vec<(String, Vec<u8>)> = BASE
        .iter()
        .filter(|(p, _)| p.ends_with(".py"))
        .map(|(p, c)| (p.to_string(), c.as_bytes().to_vec()))
        .collect();
    fx.index.index_paths("demo", "main", py).await.unwrap();
    let main_sym = |q: &'static str| {
        let index = fx.index.clone();
        async move {
            index
                .search_symbols("demo", "main", q, 5)
                .await
                .unwrap()
                .into_iter()
                .find(|s| s.symbol.qname == q)
                .unwrap()
        }
    };
    let (hello, zebra) = (main_sym("Greeter.hello").await, main_sym("zebra").await);
    let task_zebra = fx
        .index
        .search_symbols("demo", &fx.task.slug, "zebra", 5)
        .await
        .unwrap()
        .into_iter()
        .find(|s| s.symbol.qname == "zebra")
        .unwrap();
    let sem = Arc::new(FakeSemantic {
        hits: vec![
            code_hit(hello.id, "app/greeter.py", 0.9),
            code_hit(zebra.id, "app/unrelated.py", 0.8),
        ],
        fail: false,
        delay: Duration::ZERO,
        calls: AtomicUsize::new(0),
    });
    let pack = packer_with(&fx, sem)
        .build(req(&fx.task, 4000, "main-hits"))
        .await
        .unwrap();
    let md = &pack.markdown;
    for main_id in [hello.id, zebra.id] {
        assert!(
            !md.contains(&format!("sym:{} ", main_id.0)),
            "main-scope id sym:{} leaked into the task pack:\n{md}",
            main_id.0
        );
    }
    assert!(
        md.contains(&format!("sym:{} ", task_zebra.id.0)),
        "zebra should arrive under its task-scope id:\n{md}"
    );
}

#[tokio::test]
async fn pack_returns_receipt_and_manifest_lists_every_chosen_item() {
    let fx = fixture().await;
    let pack = fx.packer.build(req(&fx.task, 4000, "rcp")).await.unwrap();
    let id = pack.receipt.clone().expect("pack carries a receipt id");
    let r = fx.store.get_receipt(&id[4..]).await.unwrap();
    assert_eq!(r["head_change"], fx.task.head_change.0);
    assert_eq!(r["packer_version"], scholion_pack::PACKER_VERSION);
    let items = r["items"].as_array().unwrap();
    assert!(!items.is_empty());
    let mut sum = 0;
    for it in items {
        let iid = it["id"].as_str().unwrap();
        assert!(
            pack.markdown.contains(iid)
                // the task header and map entries render without their ids
                || ["task:", "map:", "file:"].iter().any(|p| iid.starts_with(p)),
            "receipt item {iid} not in the pack:\n{}",
            pack.markdown
        );
        assert!(!it["reason"].as_str().unwrap().is_empty(), "{it}");
        sum += it["tokens"].as_u64().unwrap();
    }
    assert!(sum as u32 <= pack.tokens, "{sum} > {}", pack.tokens);
    // the changed function is a seed and says so
    assert!(
        items
            .iter()
            .any(|it| it["reason"] == "seed" && it["source"][0] == "app/greeter.py"),
        "{items:?}"
    );
}

#[tokio::test]
async fn receipt_write_failure_still_returns_pack_with_degraded_receipt() {
    let fx = fixture().await;
    sqlx::query("DROP TABLE context_receipts")
        .execute(fx.store.pool())
        .await
        .unwrap();
    let pack = fx.packer.build(req(&fx.task, 4000, "nr")).await.unwrap();
    assert!(pack.receipt.is_none());
    assert!(
        pack.degraded.contains(&"receipt".to_string()),
        "{:?}",
        pack.degraded
    );
}

async fn hello_note(fx: &Fx) -> Record {
    fx.store
        .add_record(record(
            fx.task.id,
            RecordKind::Decision,
            "hello must stay pure",
            "",
            vec![Anchor {
                path: "app/greeter.py".into(),
                symbol: Some("Greeter.hello".into()),
            }],
        ))
        .await
        .unwrap()
}

#[tokio::test]
async fn record_on_edited_symbol_is_flagged_changed_since_recorded() {
    let fx = fixture().await;
    let rec = hello_note(&fx).await;
    // written when the symbol looked different
    fx.store
        .set_anchor_hashes(rec.id, &[Some("0".repeat(64))])
        .await
        .unwrap();
    let pack = fx.packer.build(req(&fx.task, 4000, "chg")).await.unwrap();
    let line = pack
        .markdown
        .lines()
        .find(|l| l.contains("hello must stay pure"))
        .unwrap_or_else(|| panic!("{}", pack.markdown));
    assert!(line.contains("(changed since recorded)"), "{line}");
}

#[tokio::test]
async fn record_on_untouched_symbol_is_not_flagged() {
    let fx = fixture().await;
    let rec = hello_note(&fx).await;
    let now = fx
        .packer
        .anchor_hashes(fx.task.id, &rec.anchors)
        .await
        .unwrap();
    assert!(now[0].is_some(), "the anchored symbol exists at head");
    fx.store.set_anchor_hashes(rec.id, &now).await.unwrap();
    let pack = fx.packer.build(req(&fx.task, 4000, "same")).await.unwrap();
    assert!(
        !pack.markdown.contains("changed since recorded"),
        "{}",
        pack.markdown
    );
}

#[tokio::test]
async fn required_convention_survives_a_150_token_budget() {
    let fx = fixture().await;
    let id = fx
        .store
        .add_convention("demo", "app/**/*.py", "never log user names")
        .await
        .unwrap();
    fx.store.set_convention_required(id, true).await.unwrap();
    let pack = fx.packer.build(req(&fx.task, 150, "tiny")).await.unwrap();
    assert!(pack.tokens <= 150, "{}", pack.tokens);
    assert!(
        pack.markdown.contains(&format!("conv:{}", id.0)),
        "required convention dropped:\n{}",
        pack.markdown
    );
}

// ---- Plan 2b Task 4: charged expand ----

#[tokio::test]
async fn expand_is_charged_to_the_session_and_reported_in_packs() {
    let fx = fixture().await;
    let pack = fx.packer.build(req(&fx.task, 600, "ex")).await.unwrap();
    assert_eq!(pack.session_expand_tokens, 0);
    let sym = find_id(&pack.markdown, "sym:").unwrap();
    let a = fx
        .packer
        .expand_charged(fx.task.id, &sym, "ex", None)
        .await
        .unwrap();
    assert!(a.tokens > 0 && a.session_expand_tokens == u64::from(a.tokens));
    // repeat expands are charged again (Review Focus 4)
    let b = fx
        .packer
        .expand_charged(fx.task.id, &sym, "ex", None)
        .await
        .unwrap();
    assert_eq!(b.session_expand_tokens, u64::from(a.tokens) * 2);
    let pack = fx.packer.build(req(&fx.task, 600, "ex")).await.unwrap();
    assert_eq!(pack.session_expand_tokens, u64::from(a.tokens) * 2);
    // other sessions are not charged
    let other = fx.packer.build(req(&fx.task, 600, "other")).await.unwrap();
    assert_eq!(other.session_expand_tokens, 0);
}

#[tokio::test]
async fn expand_over_cap_is_refused_with_reason() {
    let fx = fixture().await;
    let pack = fx.packer.build(req(&fx.task, 600, "cap")).await.unwrap();
    let sym = find_id(&pack.markdown, "sym:").unwrap();
    let first = fx
        .packer
        .expand_charged(fx.task.id, &sym, "cap", Some(10_000))
        .await
        .unwrap();
    let cap = first.session_expand_tokens; // exactly used up
    match fx
        .packer
        .expand_charged(fx.task.id, &sym, "cap", Some(cap))
        .await
    {
        Err(AvError::Invalid(msg)) => assert!(msg.contains("expand cap"), "{msg}"),
        other => panic!("expected a cap refusal, got {other:?}"),
    }
}

async fn review_finding(fx: &Fx) -> Record {
    let mut r = record(
        fx.task.id,
        RecordKind::Review,
        "hello must not shout at empty names",
        "PINNED-BODY: return \"\" when name is empty, then add the mark",
        vec![anchor("app/greeter.py", Some("Greeter.hello"))],
    );
    r.author_agent = "reviewer".into();
    fx.store.add_record(r).await.unwrap()
}

#[tokio::test]
async fn review_findings_are_pinned_first_at_full_tier_while_the_owner_works() {
    let fx = fixture().await;
    for (kind, summary) in [
        (
            RecordKind::Attempt,
            "tried a decorator for the mark; too clever",
        ),
        (RecordKind::Decision, "append the mark inside hello"),
    ] {
        fx.store
            .add_record(record(
                fx.task.id,
                kind,
                summary,
                &"detail ".repeat(40),
                vec![anchor("app/greeter.py", Some("Greeter.hello"))],
            ))
            .await
            .unwrap();
    }
    review_finding(&fx).await;
    let pack = fx.packer.build(req(&fx.task, 400, "pin")).await.unwrap();
    let md = &pack.markdown;
    assert!(pack.tokens <= 400, "{}", pack.tokens);
    let history = &md[pos(md, "## Decisions & dead ends")..];
    let first = history.lines().nth(1).unwrap();
    assert!(first.contains("review: hello must not shout"), "{md}");
    assert!(md.contains("PINNED-BODY"), "pinned at full tier:\n{md}");
}

#[tokio::test]
async fn review_findings_are_not_pinned_once_the_task_is_back_in_review() {
    let fx = fixture().await;
    review_finding(&fx).await;
    fx.store
        .set_status(fx.task.id, TaskStatus::InReview)
        .await
        .unwrap();
    let pack = fx.packer.build(req(&fx.task, 4000, "rev")).await.unwrap();
    let receipt = fx
        .store
        .get_receipt(pack.receipt.as_deref().unwrap())
        .await
        .unwrap();
    let reasons: Vec<&str> = receipt["items"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|i| i["reason"].as_str())
        .collect();
    assert!(!reasons.contains(&"pinned-review"), "{reasons:?}");
}

#[tokio::test]
async fn conflicted_task_pack_names_the_conflicts_and_the_other_tasks_records() {
    let fx = fixture().await;
    let peer = fx
        .store
        .create_task(new_task("Rename hello"))
        .await
        .unwrap();
    fx.store
        .add_record(record(
            peer.id,
            RecordKind::Decision,
            "hello is now greet; callers updated",
            "",
            vec![anchor("app/greeter.py", None)],
        ))
        .await
        .unwrap();
    fx.store
        .set_status(peer.id, TaskStatus::Merged)
        .await
        .unwrap();
    fx.store
        .set_merge_state(
            fx.task.id,
            &MergeState {
                conflicts: vec!["app/greeter.py".into()],
                ..MergeState::default()
            },
        )
        .await
        .unwrap();
    fx.store
        .set_status(fx.task.id, TaskStatus::Conflicted)
        .await
        .unwrap();
    let pack = fx.packer.build(req(&fx.task, 2000, "c")).await.unwrap();
    let md = &pack.markdown;
    let conflicts = &md[pos(md, "## Conflicts")..pos(md, "## Map")];
    assert!(pos(md, "## Task") < pos(md, "## Conflicts"), "{md}");
    assert!(conflicts.contains("app/greeter.py"), "{conflicts}");
    assert!(
        conflicts.contains("Greeter.hello"),
        "the conflicting symbol:\n{conflicts}"
    );
    assert!(
        conflicts.contains("hello is now greet"),
        "the merged task's record:\n{conflicts}"
    );
    assert_eq!(
        md.matches("hello is now greet").count(),
        1,
        "listed once:\n{md}"
    );
}

// ---- Plan 6: lexical goal signals (no semantic layer) ----

/// A task that has changed nothing yet: its first pack is a cold start.
async fn cold_task(fx: &Fx, title: &str, goal: &str) -> Task {
    let t = fx
        .store
        .create_task(NewTask {
            goal: goal.into(),
            ..new_task(title)
        })
        .await
        .unwrap();
    fx.store
        .set_workspace(
            t.id,
            std::path::Path::new("/ws/demo/cold"),
            &ChangeId("base".into()),
        )
        .await
        .unwrap();
    fx.store
        .set_head(t.id, &ChangeId("base".into()), &[])
        .await
        .unwrap();
    fx.index
        .index_paths(
            "demo",
            &t.slug,
            BASE.iter()
                .filter(|(p, _)| p.ends_with(".py"))
                .map(|(p, c)| (p.to_string(), c.as_bytes().to_vec()))
                .collect(),
        )
        .await
        .unwrap();
    fx.store.get_task(t.id).await.unwrap()
}

#[tokio::test]
async fn cold_start_pack_finds_code_by_the_goals_words_without_semantic() {
    let fx = fixture().await;
    let t = cold_task(&fx, "Zebra", "The zebra function should return two").await;
    let pack = fx.packer.build(req(&t, 2000, "cold")).await.unwrap();
    assert!(
        pack.markdown.contains("def zebra():\n    return 1"),
        "full-text search on the goal finds zebra():\n{}",
        pack.markdown
    );
    assert!(pack.degraded.is_empty(), "{:?}", pack.degraded);
}

#[tokio::test]
async fn an_identifier_the_goal_names_seeds_the_pack_with_its_neighbours() {
    let fx = fixture().await;
    let t = cold_task(
        &fx,
        "Rename",
        "Rename format_name to pretty_name everywhere",
    )
    .await;
    let pack = fx.packer.build(req(&t, 2000, "named")).await.unwrap();
    let md = &pack.markdown;
    assert!(
        md.contains("- app/util.py\n  L1: def format_name(n):"),
        "seed file in the Map:\n{md}"
    );
    assert!(
        md.contains("def format_name(n):\n    return n.title()"),
        "seed code:\n{md}"
    );
    assert!(
        md.contains("Greeter.hello"),
        "its caller comes in through the reference graph:\n{md}"
    );
}

#[tokio::test]
async fn a_cold_start_without_seeds_pulls_in_what_the_best_hits_reference() {
    let fx = fixture().await;
    let t = cold_task(&fx, "Louder", "The greeter should say hello louder").await;
    let pack = fx.packer.build(req(&t, 2000, "cold-hops")).await.unwrap();
    let md = &pack.markdown;
    let code = &md[md.find("## Code").expect("a Code section")..];
    assert!(code.contains("Greeter.hello"), "the text hit:\n{md}");
    assert!(
        code.contains("format_name (app/util.py:1-2)\n  def format_name(n):"),
        "what the best hit calls comes in through the reference graph:\n{md}"
    );
    assert!(
        !code.contains("return n.title()"),
        "a neighbour of a guess is context: its signature, not its body:\n{md}"
    );
}

#[tokio::test]
async fn a_signatures_pack_locates_code_without_its_bodies() {
    let fx = fixture().await;
    let t = cold_task(&fx, "Zebra", "The zebra function should return two").await;
    let pack = fx
        .packer
        .build(PackRequest {
            code: CodeDetail::Signatures,
            ..req(&t, 2000, "sigs")
        })
        .await
        .unwrap();
    let md = &pack.markdown;
    assert!(
        md.contains("zebra (app/unrelated.py:1-2)\n  def zebra():"),
        "path, lines and signature:\n{md}"
    );
    assert!(!md.contains("return 1"), "no body:\n{md}");
}

#[tokio::test]
async fn a_pack_without_the_map_leaves_out_the_repo_layout() {
    let fx = fixture().await;
    let with = fx
        .packer
        .build(req(&fx.task, 2000, "map-on"))
        .await
        .unwrap();
    assert!(with.markdown.contains("repo map"), "{}", with.markdown);
    let without = fx
        .packer
        .build(PackRequest {
            map: false,
            ..req(&fx.task, 2000, "map-off")
        })
        .await
        .unwrap();
    assert!(!without.markdown.contains("## Map"), "{}", without.markdown);
    assert!(without.markdown.contains("## Code"), "{}", without.markdown);
}

/// The `- sym:` lines of a pack's Code section, in order.
fn code_lines(md: &str) -> Vec<&str> {
    let code = md.split("## Code").nth(1).unwrap_or("");
    let code = code.split("\n## ").next().unwrap_or("");
    code.lines().filter(|l| l.starts_with("- sym:")).collect()
}

#[tokio::test]
async fn a_pack_capped_at_n_code_items_keeps_the_best_scored_ones() {
    let fx = fixture().await;
    let all = fx
        .packer
        .build(req(&fx.task, 4000, "uncapped"))
        .await
        .unwrap();
    let all = code_lines(&all.markdown);
    assert!(
        all.len() > 1,
        "the fixture needs several code items: {all:?}"
    );
    let capped = fx
        .packer
        .build(PackRequest {
            max_code: Some(1),
            ..req(&fx.task, 4000, "capped")
        })
        .await
        .unwrap();
    assert_eq!(
        code_lines(&capped.markdown),
        all[..1],
        "{}",
        capped.markdown
    );
    assert!(
        !capped
            .omitted
            .iter()
            .any(|h| all[1..].iter().any(|l| l.contains(h.as_str()))),
        "items past the cap are left out, not listed as omitted: {:?}",
        capped.omitted
    );
}

#[tokio::test]
async fn a_body_left_out_of_a_signatures_pack_is_still_sent_by_a_later_full_pack() {
    let fx = fixture().await;
    let t = cold_task(&fx, "Zebra", "The zebra function should return two").await;
    let sigs = fx
        .packer
        .build(PackRequest {
            code: CodeDetail::Signatures,
            ..req(&t, 2000, "same-session")
        })
        .await
        .unwrap();
    assert!(!sigs.markdown.contains("return 1"), "{}", sigs.markdown);
    let full = fx
        .packer
        .build(req(&t, 2000, "same-session"))
        .await
        .unwrap();
    assert!(
        full.markdown.contains("def zebra():\n    return 1"),
        "the session was sent the signature only, so the body is not 'sent earlier':\n{}",
        full.markdown
    );
}

#[tokio::test]
async fn a_type_that_tops_the_text_hits_does_not_pull_in_everything_that_uses_it() {
    let fx = fixture().await;
    let files: &[(&str, &str)] = &[
        (
            "app/settings.py",
            "class Settings:\n    \"\"\"Depth, roots, symlinks and paths for a walk.\"\"\"\n    depth = 1\n    roots = []\n    symlinks = True\n",
        ),
        ("app/cli.py", "def parse_args():\n    return Settings()\n"),
        (
            "app/walk.py",
            "def depth_of(path, roots):\n    return len(path) - len(roots)\n",
        ),
    ];
    fx.vcs.set("zoo", files);
    let t = fx
        .store
        .create_task(NewTask {
            goal: "The depth of a symlink under the walk roots is wrong".into(),
            ..new_task("Depth")
        })
        .await
        .unwrap();
    let zoo = ChangeId("zoo".into());
    fx.store
        .set_workspace(t.id, std::path::Path::new("/ws/demo/depth"), &zoo)
        .await
        .unwrap();
    fx.store.set_head(t.id, &zoo, &[]).await.unwrap();
    fx.index
        .index_paths(
            "demo",
            &t.slug,
            files
                .iter()
                .map(|(p, c)| (p.to_string(), c.as_bytes().to_vec()))
                .collect(),
        )
        .await
        .unwrap();
    let t = fx.store.get_task(t.id).await.unwrap();
    let pack = fx.packer.build(req(&t, 2000, "depth")).await.unwrap();
    let md = &pack.markdown;
    assert!(md.contains("def depth_of(path, roots):"), "{md}");
    assert!(
        !md.contains("parse_args"),
        "a type's neighbours are everything that uses it: no guess to expand from:\n{md}"
    );
}

// ---- a module-qualified goal and a link through a constant (found by the site's replay) ----

const SHOP: &[(&str, &str)] = &[
    (
        "shop/pricing.py",
        "BULK_QTY = 10\n\n\ndef line_total(price, qty):\n    if qty >= BULK_QTY:\n        return price * qty * 0.95\n    return price * qty\n\n\ndef total(lines):\n    return sum(line_total(p, q) for p, q in lines)\n",
    ),
    (
        "shop/cart.py",
        "from pricing import total\n\n\nclass Cart:\n    def checkout_total(self):\n        return total(self.lines)\n",
    ),
    (
        "shop/receipt.py",
        "def render(cart):\n    return str(cart.checkout_total())\n",
    ),
];

/// A task on the shop files, at `head` (defaults to the base files), with an earlier task's
/// decision anchored to `pricing.py:line_total`.
async fn shop_task(fx: &Fx, goal: &str, head: Option<&[(&str, &str)]>) -> Task {
    fx.vcs.set("shop", SHOP);
    let head_files = head.unwrap_or(SHOP);
    fx.vcs.set("shop-head", head_files);
    let earlier = fx
        .store
        .create_task(new_task("Bulk discount"))
        .await
        .unwrap();
    fx.store
        .add_record(record(
            earlier.id,
            RecordKind::Decision,
            "Apply the bulk discount in line_total, not in Cart",
            "",
            vec![anchor("shop/pricing.py", Some("line_total"))],
        ))
        .await
        .unwrap();
    let t = fx
        .store
        .create_task(NewTask {
            goal: goal.into(),
            ..new_task("Shop work")
        })
        .await
        .unwrap();
    fx.store
        .set_workspace(
            t.id,
            std::path::Path::new("/ws/demo/shop"),
            &ChangeId("shop".into()),
        )
        .await
        .unwrap();
    fx.store
        .set_head(t.id, &ChangeId("shop-head".into()), &[])
        .await
        .unwrap();
    fx.index
        .index_paths(
            "demo",
            &t.slug,
            head_files
                .iter()
                .map(|(p, c)| (p.to_string(), c.as_bytes().to_vec()))
                .collect(),
        )
        .await
        .unwrap();
    fx.store.get_task(t.id).await.unwrap()
}

#[tokio::test]
async fn a_goal_naming_module_dot_function_seeds_that_function_and_its_records() {
    let fx = fixture().await;
    let t = shop_task(&fx, "Memoize pricing.line_total for repeated carts", None).await;
    let pack = fx.packer.build(req(&t, 2000, "dotted")).await.unwrap();
    let md = &pack.markdown;
    assert!(
        md.contains("Apply the bulk discount in line_total"),
        "the decision anchored to the named function:\n{md}"
    );
    assert!(md.contains("def line_total(price, qty):\n"), "{md}");
}

#[tokio::test]
async fn a_change_that_uses_another_modules_constant_gets_that_modules_records() {
    let fx = fixture().await;
    let receipt = "from pricing import BULK_QTY\n\n\ndef render(cart):\n    rows = [str(cart.checkout_total())]\n    if any(q >= BULK_QTY for _, q in cart.lines):\n        rows.append('bulk -5%')\n    return '\\n'.join(rows)\n";
    let head: Vec<(&str, &str)> = SHOP
        .iter()
        .map(|&(p, c)| {
            if p == "shop/receipt.py" {
                (p, receipt)
            } else {
                (p, c)
            }
        })
        .collect();
    let t = shop_task(&fx, "Receipts list a bulk discount line", Some(&head)).await;
    let pack = fx.packer.build(req(&t, 2000, "constant")).await.unwrap();
    let md = &pack.markdown;
    assert!(
        md.contains("Apply the bulk discount in line_total"),
        "render uses BULK_QTY, which line_total uses: the decision on line_total is two hops \
         away:\n{md}"
    );
}
