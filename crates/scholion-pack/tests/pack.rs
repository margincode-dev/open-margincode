use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use crate::common::*;
use async_trait::async_trait;
use scholion_core::*;
use scholion_index::{CodeIndex, ScopeView};
use scholion_pack::{CodeDetail, Pack, PackRequest, Packer, estimate_tokens};

/// Scripted `Semantic`. `calls` counts `search` invocations. Like the real one, each kind of hit
/// answers at most `limit`, best first.
struct FakeSemantic {
    hits: Vec<SemanticHit>,
    fail: bool,
    delay: Duration,
    calls: AtomicUsize,
}

#[async_trait]
impl Semantic for FakeSemantic {
    async fn search(&self, _repo: &str, _query: &str, limit: usize) -> Result<Vec<SemanticHit>> {
        let _ = self.calls.fetch_add(1, Ordering::SeqCst);
        if !self.delay.is_zero() {
            tokio::time::sleep(self.delay).await;
        }
        if self.fail {
            return Err(AvError::Semantic("down".into()));
        }
        let mut hits = self.hits.clone();
        hits.sort_by(|a, b| b.score.total_cmp(&a.score));
        let mut per_kind: std::collections::HashMap<String, usize> = Default::default();
        hits.retain(|h| {
            let n = per_kind.entry(format!("{:?}", h.kind)).or_default();
            *n += 1;
            *n <= limit
        });
        Ok(hits)
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
        .symbols_in(&ScopeView::new("demo", &fx.task.slug), "app/greeter.py")
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
                .search_symbols(&ScopeView::new("demo", "main"), q, 5)
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
        .search_symbols(&ScopeView::new("demo", &fx.task.slug), "zebra", 5)
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

/// Thirty records on `app/greeter.py` that have nothing to do with the task's goal ("Make hello
/// louder"), each newer and of a stronger kind than `lesson`, which is about it.
async fn crowded_greeter(fx: &Fx, lesson_summary: &str) -> Record {
    let other = fx
        .store
        .create_task(new_task("Earlier work"))
        .await
        .unwrap();
    let lesson = fx
        .store
        .add_record(record(
            other.id,
            RecordKind::Decision,
            lesson_summary,
            "",
            vec![anchor("app/greeter.py", Some("Greeter.hello"))],
        ))
        .await
        .unwrap();
    for i in 0..30 {
        fx.store
            .add_record(record(
                other.id,
                RecordKind::Attempt,
                &format!(
                    "Wrapping `Greeter.hello` in a retry loop (variant {i}) hid the real error \
                     and the suite hung on its second pass."
                ),
                "",
                vec![anchor("app/greeter.py", Some("Greeter.hello"))],
            ))
            .await
            .unwrap();
    }
    lesson
}

fn short(rec: &Record) -> String {
    format!("rec:{}", &rec.id.0.simple().to_string()[..8])
}

#[tokio::test]
async fn records_at_puts_what_matches_the_readers_goal_first() {
    let fx = fixture().await;
    let lesson = crowded_greeter(
        &fx,
        "A louder hello ends in one exclamation mark; capitals read as shouting to screen readers.",
    )
    .await;
    let at = fx
        .packer
        .records_at(fx.task.id, "app/greeter.py", None)
        .await
        .unwrap();
    assert!(
        at.records.len() < 31,
        "the answer is cut at its token cap: {}",
        at.records.len()
    );
    assert_eq!(
        at.records.first(),
        Some(&short(&lesson)),
        "no semantic layer: the words of the goal pick the record\n{}",
        at.markdown
    );
}

#[tokio::test]
async fn records_at_uses_similarity_when_the_words_do_not_match() {
    let fx = fixture().await;
    // not one word of the goal
    let lesson = crowded_greeter(
        &fx,
        "Greetings gain volume by punctuation only; capitals read as shouting to screen readers.",
    )
    .await;
    let plain = fx
        .packer
        .records_at(fx.task.id, "app/greeter.py", None)
        .await
        .unwrap();
    assert!(
        !plain.records.contains(&short(&lesson)),
        "without similarity it is behind 30 newer attempts:\n{}",
        plain.markdown
    );
    let sem = Arc::new(FakeSemantic {
        hits: vec![record_hit(&lesson, 0.8)],
        fail: false,
        delay: Duration::ZERO,
        calls: AtomicUsize::new(0),
    });
    let at = packer_with(&fx, sem)
        .records_at(fx.task.id, "app/greeter.py", None)
        .await
        .unwrap();
    assert_eq!(at.records.first(), Some(&short(&lesson)), "{}", at.markdown);
}

#[tokio::test]
async fn records_at_does_not_wait_for_a_slow_semantic_layer() {
    let fx = fixture().await;
    let lesson = crowded_greeter(
        &fx,
        "Greetings gain volume by punctuation only; capitals read as shouting to screen readers.",
    )
    .await;
    let sem = Arc::new(FakeSemantic {
        hits: vec![record_hit(&lesson, 0.8)],
        fail: false,
        delay: Duration::from_secs(5),
        calls: AtomicUsize::new(0),
    });
    let started = std::time::Instant::now();
    let at = packer_with(&fx, sem)
        .records_at(fx.task.id, "app/greeter.py", None)
        .await
        .unwrap();
    assert!(
        started.elapsed() < Duration::from_secs(2),
        "it answered late"
    );
    assert!(!at.records.is_empty(), "and still answers: {}", at.markdown);
}

/// `score::RECORD_HITS`: record hits a pack of up to 4,000 tokens asks for. From 4,250 tokens the
/// cap is twice the code cap (`score::record_hit_limit`): 48 at 6,000, 64 at 8,000.
const RECORD_HITS: usize = 32;

/// The lines of one `## ` section of a pack.
fn section<'a>(md: &'a str, heading: &str) -> &'a str {
    let start = pos(md, heading) + heading.len();
    let rest = &md[start..];
    &rest[..rest.find("\n## ").unwrap_or(rest.len())]
}

/// The `rec:` ids of a section's record lines, in order.
fn record_ids(section: &str) -> Vec<String> {
    section
        .lines()
        .filter_map(|l| l.strip_prefix("- "))
        .filter(|l| l.starts_with("rec:"))
        .map(|l| l[..12].to_owned())
        .collect()
}

/// The task's own intent as the best semantic hit, then `n` notes of another task that nothing
/// but a semantic hit brings into a pack, best first.
async fn own_intent_then_cabbage_notes(fx: &Fx, n: usize) -> Vec<SemanticHit> {
    let own = fx
        .store
        .task_records(fx.task.id)
        .await
        .unwrap()
        .into_iter()
        .find(|r| r.kind == RecordKind::Intent)
        .unwrap();
    let other = fx
        .store
        .create_task(new_task("Earlier work"))
        .await
        .unwrap();
    let mut hits = vec![record_hit(&own, 0.99)];
    for i in 0..n {
        let rec = fx
            .store
            .add_record(record(
                other.id,
                RecordKind::Note,
                &format!("Cabbage note {i:02} about the cellar"),
                "",
                vec![],
            ))
            .await
            .unwrap();
        hits.push(record_hit(&rec, 0.9 - i as f32 * 0.01));
    }
    hits
}

#[tokio::test]
async fn a_pack_asks_for_more_record_hits_than_code_hits_and_its_own_intent_takes_none() {
    let fx = fixture().await;
    // semantic search puts the task's own intent first: it restates the goal
    let hits = own_intent_then_cabbage_notes(&fx, RECORD_HITS).await;
    let sem = Arc::new(FakeSemantic {
        hits,
        fail: false,
        delay: Duration::ZERO,
        calls: AtomicUsize::new(0),
    });
    // 6000 tokens: 24 code hits, and room to show every record
    let pack = packer_with(&fx, sem)
        .build(req(&fx.task, 6000, "record-hits"))
        .await
        .unwrap();
    for i in 0..RECORD_HITS {
        assert!(
            pack.markdown.contains(&format!("Cabbage note {i:02}")),
            "record hit {i} is not a candidate:\n{}",
            pack.markdown
        );
    }
}

#[tokio::test]
async fn the_tasks_own_intent_does_not_count_toward_the_record_hit_cap() {
    let fx = fixture().await;
    // one more note than the cap, behind the task's own intent
    let hits = own_intent_then_cabbage_notes(&fx, RECORD_HITS + 1).await;
    let sem = Arc::new(FakeSemantic {
        hits,
        fail: false,
        delay: Duration::ZERO,
        calls: AtomicUsize::new(0),
    });
    // 4000 tokens: 16 code hits, so the record cap is RECORD_HITS itself, and every candidate
    // is at least named
    let pack = packer_with(&fx, sem)
        .build(req(&fx.task, 4000, "record-hits-own-intent"))
        .await
        .unwrap();
    let md = &pack.markdown;
    let last = RECORD_HITS - 1;
    assert!(
        md.contains(&format!("Cabbage note {last:02}")),
        "the task's own intent took the last record hit's place:\n{md}"
    );
    assert!(
        !md.contains(&format!("Cabbage note {RECORD_HITS:02}")),
        "the cap is RECORD_HITS other records, not one more:\n{md}"
    );
}

#[tokio::test]
async fn the_record_whose_words_match_the_goal_leads_the_records_anchored_on_the_seed() {
    let fx = fixture().await;
    // thirty attempts on the changed symbol, newer and of a stronger kind than the lesson
    let lesson = crowded_greeter(
        &fx,
        "A louder hello ends in one exclamation mark; capitals read as shouting to screen readers.",
    )
    .await;
    let pack = fx
        .packer
        .build(req(&fx.task, 800, "lead-words"))
        .await
        .unwrap();
    let md = &pack.markdown;
    let history = section(md, "## Decisions & dead ends");
    assert_eq!(
        record_ids(history).first(),
        Some(&short(&lesson)),
        "no semantic layer: the goal's words pick it\n{md}"
    );
    assert!(md.contains(&lesson.summary), "and it is not clipped:\n{md}");
}

#[tokio::test]
async fn the_record_most_similar_to_the_goal_leads_the_records_anchored_on_the_seed() {
    let fx = fixture().await;
    crowded_greeter(&fx, "Greeter.hello keeps its signature.").await;
    let other = fx.store.create_task(new_task("Side quest")).await.unwrap();
    // not one word of the goal, on a neighbour of the changed symbol, and the weakest kind
    let lesson = fx
        .store
        .add_record(record(
            other.id,
            RecordKind::Note,
            "Greetings gain volume by punctuation only; capitals read as shouting to screen readers.",
            "",
            vec![anchor("app/util.py", Some("format_name"))],
        ))
        .await
        .unwrap();
    let plain = fx
        .packer
        .build(req(&fx.task, 800, "lead-plain"))
        .await
        .unwrap();
    assert_ne!(
        record_ids(section(&plain.markdown, "## Decisions & dead ends")).first(),
        Some(&short(&lesson)),
        "without similarity the anchored attempts come first:\n{}",
        plain.markdown
    );
    let sem = Arc::new(FakeSemantic {
        hits: vec![record_hit(&lesson, 0.8)],
        fail: false,
        delay: Duration::ZERO,
        calls: AtomicUsize::new(0),
    });
    let pack = packer_with(&fx, sem)
        .build(req(&fx.task, 800, "lead-similar"))
        .await
        .unwrap();
    let md = &pack.markdown;
    assert_eq!(
        record_ids(section(md, "## Decisions & dead ends")).first(),
        Some(&short(&lesson)),
        "{md}"
    );
    assert!(md.contains(&lesson.summary), "and it is not clipped:\n{md}");
}

#[tokio::test]
async fn a_record_mined_from_git_history_does_not_lead_even_when_it_matches_best() {
    let fx = fixture().await;
    let other = fx
        .store
        .create_task(new_task("Earlier work"))
        .await
        .unwrap();
    let attempt = fx
        .store
        .add_record(record(
            other.id,
            RecordKind::Attempt,
            "Wrapping format_name in a retry loop hid the real error.",
            "",
            vec![anchor("app/util.py", Some("format_name"))],
        ))
        .await
        .unwrap();
    let mut mined = record(
        other.id,
        RecordKind::Note,
        "Make hello louder with one exclamation mark",
        "",
        vec![anchor("app/util.py", Some("format_name"))],
    );
    mined.author_agent = history::MINED_AUTHOR.into();
    mined.change_id = Some(ChangeId("1a2b3c4d5e6f7a8b".into()));
    let mined = fx.store.add_record(mined).await.unwrap();
    let sem = Arc::new(FakeSemantic {
        hits: vec![record_hit(&mined, 0.95)],
        fail: false,
        delay: Duration::ZERO,
        calls: AtomicUsize::new(0),
    });
    let pack = packer_with(&fx, sem)
        .build(req(&fx.task, 2000, "mined"))
        .await
        .unwrap();
    let ids = record_ids(section(&pack.markdown, "## Decisions & dead ends"));
    assert_eq!(
        ids,
        vec![short(&attempt), short(&mined)],
        "{}",
        pack.markdown
    );
}

// ---- what the review of the goal leads found (bench/results/2026-11-first-pack/README.md,
// "Found in review" and the follow-up under it) ----

/// Why each chosen item of a pack is in it, by item id: the `reason` in its receipt.
async fn reasons(fx: &Fx, pack: &Pack) -> std::collections::HashMap<String, String> {
    let id = pack.receipt.clone().expect("pack carries a receipt id");
    let r = fx.store.get_receipt(&id[4..]).await.unwrap();
    let mut out = std::collections::HashMap::new();
    for it in r["items"].as_array().unwrap() {
        let item = it["id"].as_str().unwrap().to_owned();
        let reason = it["reason"].as_str().unwrap().to_owned();
        out.insert(item, reason);
    }
    out
}

#[tokio::test]
async fn a_goal_lead_whose_anchor_changed_ranks_below_an_unchanged_record_on_a_named_symbol() {
    // Deliberate, not an oversight. A goal lead keeps its `record_factor`, so one whose anchored
    // code changed since it was recorded is damped (`CHANGED_FACTOR`) and may not lead: ranking
    // a record down when its code has moved is the packer's only defence against a note that
    // went out of date. The decision and its reasons are recorded in
    // `bench/results/2026-11-first-pack/README.md`, "Found in review" and the follow-up under
    // it. Change this test only together with that decision.
    let fx = fixture().await;
    // the goal names `format_name`, which makes it a seed
    let t = cold_task(
        &fx,
        "Rename",
        "Rename format_name to pretty_name everywhere",
    )
    .await;
    let other = fx
        .store
        .create_task(new_task("Earlier work"))
        .await
        .unwrap();
    let on_named = vec![anchor("app/util.py", Some("format_name"))];
    // two words of the goal ("rename", "name"): the one record that matches it
    let lesson = fx
        .store
        .add_record(record(
            other.id,
            RecordKind::Decision,
            "Callers import the old name long after a rename; keep an alias for a release.",
            "",
            on_named.clone(),
        ))
        .await
        .unwrap();
    // not one word of the goal: all it has is its anchor on the symbol the goal names
    let rival = fx
        .store
        .add_record(record(
            other.id,
            RecordKind::Attempt,
            "Wrapping it in a retry loop hid the real error and the suite hung.",
            "",
            on_named.clone(),
        ))
        .await
        .unwrap();
    let now = fx.packer.anchor_hashes(t.id, &on_named).await.unwrap();
    assert!(now[0].is_some(), "the anchored symbol exists at head");
    fx.store.set_anchor_hashes(lesson.id, &now).await.unwrap();
    fx.store.set_anchor_hashes(rival.id, &now).await.unwrap();

    // neither record's code has changed: the goal lead leads
    let pack = fx.packer.build(req(&t, 2000, "same")).await.unwrap();
    let ids = record_ids(section(&pack.markdown, "## Decisions & dead ends"));
    assert_eq!(ids, [short(&lesson), short(&rival)], "{}", pack.markdown);

    // the lesson's code is no longer what it was when the lesson was recorded
    fx.store
        .set_anchor_hashes(lesson.id, &[Some("0".repeat(64))])
        .await
        .unwrap();
    let pack = fx.packer.build(req(&t, 2000, "moved")).await.unwrap();
    let md = &pack.markdown;
    let ids = record_ids(section(md, "## Decisions & dead ends"));
    assert_eq!(ids, [short(&rival), short(&lesson)], "{md}");
    let lesson_id = short(&lesson);
    let line = md
        .lines()
        .find(|l| l.contains(&lesson_id))
        .unwrap_or_else(|| panic!("{md}"));
    assert!(line.contains("(changed since recorded)"), "{line}");
    // it is still the goal lead, only damped: the order above is not a promotion that was lost
    let why = reasons(&fx, &pack).await;
    assert_eq!(why[&short(&lesson)], "goal-lead", "{why:?}");
    assert_eq!(why[&short(&rival)], "anchored", "{why:?}");
}

/// An attempt anchored on the changed symbol, and three notes of the same earlier task that
/// only a semantic hit brings into a pack (no anchor). None of the four has a word of the goal.
async fn seed_attempt_and_three_notes(fx: &Fx) -> (Record, Vec<Record>) {
    let other = fx
        .store
        .create_task(new_task("Earlier work"))
        .await
        .unwrap();
    let attempt = fx
        .store
        .add_record(record(
            other.id,
            RecordKind::Attempt,
            "Wrapping the greeter in a retry loop hid the real error.",
            "",
            vec![anchor("app/greeter.py", Some("Greeter.hello"))],
        ))
        .await
        .unwrap();
    let mut notes = vec![];
    for what in ["punctuation", "capitals", "whispering"] {
        let rec = fx
            .store
            .add_record(record(
                other.id,
                RecordKind::Note,
                &format!("Greetings and {what}: what screen readers do with them"),
                "",
                vec![],
            ))
            .await
            .unwrap();
        notes.push(rec);
    }
    (attempt, notes)
}

/// A semantic layer that answers with three notes as record hits, at these similarities.
fn notes_as_hits(notes: &[Record], similarity: [f32; 3]) -> Arc<FakeSemantic> {
    Arc::new(FakeSemantic {
        hits: vec![
            record_hit(&notes[0], similarity[0]),
            record_hit(&notes[1], similarity[1]),
            record_hit(&notes[2], similarity[2]),
        ],
        fail: false,
        delay: Duration::ZERO,
        calls: AtomicUsize::new(0),
    })
}

#[tokio::test]
async fn exactly_two_records_lead_and_the_third_best_match_keeps_its_own_rank() {
    let fx = fixture().await;
    let (attempt, notes) = seed_attempt_and_three_notes(&fx).await;
    // three records match the goal, each a little less than the one before
    let sem = notes_as_hits(&notes, [0.9, 0.8, 0.7]);
    let pack = packer_with(&fx, sem)
        .build(req(&fx.task, 4000, "two-leads"))
        .await
        .unwrap();
    let md = &pack.markdown;
    // the two best lead, then what is anchored on the changed symbol, then the third match
    let want = [&notes[0], &notes[1], &attempt, &notes[2]].map(short);
    let ids = record_ids(section(md, "## Decisions & dead ends"));
    assert_eq!(ids, want, "{md}");
    let why = reasons(&fx, &pack).await;
    let leads = why.values().filter(|r| *r == "goal-lead").count();
    assert_eq!(leads, 2, "{why:?}");
    assert_eq!(why[&short(&notes[2])], "semantic", "{why:?}");
}

#[tokio::test]
async fn a_tie_that_the_cut_would_split_promotes_neither_record_and_a_tie_above_it_both() {
    let fx = fixture().await;
    let (attempt, notes) = seed_attempt_and_three_notes(&fx).await;
    // The second and third match equally well. Which of the two leads could only be decided by
    // something that says nothing about the task (a record id, the order they were written
    // in), so neither does (`relevance::leads`).
    let sem = notes_as_hits(&notes, [0.9, 0.8, 0.8]);
    let pack = packer_with(&fx, sem)
        .build(req(&fx.task, 4000, "lead-tie"))
        .await
        .unwrap();
    let md = &pack.markdown;
    let ids = record_ids(section(md, "## Decisions & dead ends"));
    assert_eq!(ids.len(), 4, "{md}");
    assert_eq!(ids[..2], [short(&notes[0]), short(&attempt)], "{md}");
    let why = reasons(&fx, &pack).await;
    assert_eq!(why[&short(&notes[0])], "goal-lead", "{why:?}");
    assert_eq!(why[&short(&notes[1])], "semantic", "{why:?}");
    assert_eq!(why[&short(&notes[2])], "semantic", "{why:?}");

    // The first and second match equally well and the third less: that tie is above the cut,
    // so both lead.
    let sem = notes_as_hits(&notes, [0.9, 0.9, 0.7]);
    let pack = packer_with(&fx, sem)
        .build(req(&fx.task, 4000, "lead-pair"))
        .await
        .unwrap();
    let why = reasons(&fx, &pack).await;
    assert_eq!(why[&short(&notes[0])], "goal-lead", "{why:?}");
    assert_eq!(why[&short(&notes[1])], "goal-lead", "{why:?}");
    assert_eq!(why[&short(&notes[2])], "semantic", "{why:?}");
}

#[tokio::test]
async fn a_superseded_record_hit_takes_no_place_under_the_record_hit_cap() {
    let fx = fixture().await;
    // One more note than the cap, and no hit for the task's own intent, so every hit counts
    // and the search (which answers the cap and one more) returns them all.
    let mut hits = own_intent_then_cabbage_notes(&fx, RECORD_HITS + 1).await;
    let hits = hits.split_off(1);
    // The best hit was superseded after it was embedded: its vector does not say so yet, the
    // store does.
    let old = hits[0].record_id.unwrap();
    let mut newer = record(
        hits[0].task_id.unwrap(),
        RecordKind::Note,
        "Turnips keep longer than anything else down there",
        "",
        vec![],
    );
    newer.supersedes = Some(old);
    fx.store.add_record(newer).await.unwrap();
    let sem = Arc::new(FakeSemantic {
        hits,
        fail: false,
        delay: Duration::ZERO,
        calls: AtomicUsize::new(0),
    });
    // 4000 tokens: the record cap is RECORD_HITS, and every candidate is at least named
    let pack = packer_with(&fx, sem)
        .build(req(&fx.task, 4000, "record-hits-superseded"))
        .await
        .unwrap();
    let md = &pack.markdown;
    assert!(
        !md.contains("Cabbage note 00"),
        "a superseded record is not shown:\n{md}"
    );
    assert!(
        md.contains(&format!("Cabbage note {RECORD_HITS:02}")),
        "the superseded hit took the place of the last record hit:\n{md}"
    );
}

// ---- records are claims that can be corrected ----

/// What a pack prints under its records heading when it shows another task's record.
const CORRECT_HINT: &str = "Claims by earlier agents: check against the code; correct wrong ones \
                            with `av record --supersedes rec:…`";
/// What `records_at` (the pre-read hook) ends with, once per session.
const RECORDS_AT_HINT: &str =
    "Check them against the code; correct wrong ones with `av record --supersedes <id>`.";

/// A record of another task, anchored to `path`.
async fn earlier_record(fx: &Fx, kind: RecordKind, summary: &str, path: &str) -> Record {
    let other = fx
        .store
        .create_task(new_task("Earlier work"))
        .await
        .unwrap();
    fx.store
        .add_record(record(
            other.id,
            kind,
            summary,
            "",
            vec![anchor(path, None)],
        ))
        .await
        .unwrap()
}

#[tokio::test]
async fn a_pack_that_shows_another_tasks_record_says_a_wrong_one_can_be_corrected() {
    let fx = fixture().await;
    earlier_record(
        &fx,
        RecordKind::Decision,
        "hello never ends in a mark",
        "app/greeter.py",
    )
    .await;
    let pack = fx
        .packer
        .build(req(&fx.task, 2500, "claims"))
        .await
        .unwrap();
    let md = &pack.markdown;
    let history: Vec<&str> = section(md, "## Decisions & dead ends").lines().collect();
    assert_eq!(history[1], CORRECT_HINT, "right under the heading:\n{md}");
    assert!(history[2].starts_with("- rec:"), "{md}");
    assert_eq!(md.matches(CORRECT_HINT).count(), 1, "{md}");
    assert!(
        estimate_tokens(CORRECT_HINT, FAM) <= 25,
        "the line costs {} tokens",
        estimate_tokens(CORRECT_HINT, FAM)
    );
    assert_eq!(pack.tokens, estimate_tokens(md, FAM), "{md}");

    // a later pack of the session only refers back to the record: the line is not repeated
    let again = fx
        .packer
        .build(req(&fx.task, 2500, "claims"))
        .await
        .unwrap();
    assert!(
        again.markdown.contains("(sent earlier)"),
        "{}",
        again.markdown
    );
    assert!(!again.markdown.contains(CORRECT_HINT), "{}", again.markdown);
}

#[tokio::test]
async fn a_pack_without_a_correctable_record_of_another_task_does_not_say_it() {
    let fx = fixture().await;
    // the task's own record
    fx.store
        .add_record(record(
            fx.task.id,
            RecordKind::Attempt,
            "tried memoizing hello; broke tests",
            "",
            vec![anchor("app/greeter.py", Some("Greeter.hello"))],
        ))
        .await
        .unwrap();
    // another task's review finding: shown, but no one supersedes a review
    earlier_record(
        &fx,
        RecordKind::Review,
        "hello must not shout",
        "app/greeter.py",
    )
    .await;
    let pack = fx.packer.build(req(&fx.task, 2500, "own")).await.unwrap();
    let md = &pack.markdown;
    assert!(md.contains("tried memoizing hello"), "{md}");
    assert!(md.contains("hello must not shout"), "{md}");
    assert!(!md.contains("--supersedes"), "{md}");
}

#[tokio::test]
async fn the_line_about_correcting_records_is_counted_inside_the_budget() {
    let fx = fixture().await;
    for i in 0..6 {
        earlier_record(
            &fx,
            RecordKind::Decision,
            &format!("hello never ends in a mark, reason {i}"),
            "app/greeter.py",
        )
        .await;
    }
    let mut said = 0;
    for budget in [90, 120, 150, 200, 260, 330, 400, 600, 800] {
        let pack = fx
            .packer
            .build(req(&fx.task, budget, &format!("budget-{budget}")))
            .await
            .unwrap();
        let md = &pack.markdown;
        assert!(pack.tokens <= budget, "{} > {budget}:\n{md}", pack.tokens);
        assert_eq!(pack.tokens, estimate_tokens(md, FAM), "{md}");
        if md.contains(CORRECT_HINT) {
            said += 1;
            assert!(md.contains("\n- rec:"), "the line without a record:\n{md}");
        } else {
            assert!(
                !section_or_empty(md, "## Decisions & dead ends").contains("- rec:"),
                "records of another task without the line:\n{md}"
            );
        }
    }
    assert!(said >= 3, "only {said} of the packs had room for records");
}

fn section_or_empty<'a>(md: &'a str, heading: &str) -> &'a str {
    if md.contains(heading) {
        section(md, heading)
    } else {
        ""
    }
}

#[tokio::test]
async fn records_at_says_once_per_session_that_a_wrong_record_can_be_corrected() {
    let fx = fixture().await;
    earlier_record(
        &fx,
        RecordKind::Decision,
        "hello never ends in a mark",
        "app/greeter.py",
    )
    .await;
    earlier_record(
        &fx,
        RecordKind::Note,
        "format_name is shared with goodbye",
        "app/util.py",
    )
    .await;
    earlier_record(
        &fx,
        RecordKind::Review,
        "the readme must name the flag",
        "README.md",
    )
    .await;
    let at = |path: &'static str, session: Option<&'static str>| {
        let packer = &fx.packer;
        let task = fx.task.id;
        async move { packer.records_at(task, path, session).await.unwrap() }
    };

    let first = at("app/greeter.py", Some("jit")).await;
    assert!(
        first
            .markdown
            .ends_with(&format!("Full text: `av expand <id>`.\n{RECORDS_AT_HINT}")),
        "{}",
        first.markdown
    );
    assert!(
        estimate_tokens(&first.markdown, AgentFamily::default())
            <= scholion_pack::RECORDS_AT_TOKENS
    );
    // another file in the same session: its records, and not the line again
    let second = at("app/util.py", Some("jit")).await;
    assert!(
        second.markdown.contains("shared with goodbye"),
        "{}",
        second.markdown
    );
    assert!(
        !second.markdown.contains("--supersedes"),
        "{}",
        second.markdown
    );
    assert!(second.markdown.ends_with("Full text: `av expand <id>`."));
    // a new session is told once more
    let fresh = at("app/util.py", Some("jit-2")).await;
    assert!(
        fresh.markdown.ends_with(RECORDS_AT_HINT),
        "{}",
        fresh.markdown
    );
    // without a session nothing is remembered, so every answer says it
    for _ in 0..2 {
        let unnamed = at("app/greeter.py", None).await;
        assert!(
            unnamed.markdown.ends_with(RECORDS_AT_HINT),
            "{}",
            unnamed.markdown
        );
    }
    // a review finding is no one's to supersede: not said, and still owed to the session
    let review = at("README.md", Some("jit-3")).await;
    assert!(
        review.markdown.contains("must name the flag"),
        "{}",
        review.markdown
    );
    assert!(
        !review.markdown.contains("--supersedes"),
        "{}",
        review.markdown
    );
    let later = at("app/greeter.py", Some("jit-3")).await;
    assert!(
        later.markdown.ends_with(RECORDS_AT_HINT),
        "{}",
        later.markdown
    );
}

/// A pack without its footer (`tokens: N/B` and what was omitted).
fn above_footer(md: &str) -> &str {
    md.rsplit_once("\ntokens: ").map_or(md, |(body, _)| body)
}

/// The smallest budget at which the second pack of a session (whose first pack had room for
/// everything) still holds all that it holds with room to spare, and that pack. Every budget
/// probed is a new session with the same first pack.
async fn smallest_whole_second_pack(fx: &Fx, tag: &str) -> (u32, String) {
    const ROOMY: u32 = 4000;
    let second = |session: String, budget: u32| async move {
        let first = fx
            .packer
            .build(req(&fx.task, ROOMY, &session))
            .await
            .unwrap();
        assert!(first.omitted.is_empty(), "{}", first.markdown);
        fx.packer
            .build(req(&fx.task, budget, &session))
            .await
            .unwrap()
    };
    let all = second(format!("{tag}-all"), ROOMY).await;
    assert!(all.omitted.is_empty(), "{}", all.markdown);
    let whole = above_footer(&all.markdown).to_owned();
    let holds_all = |budget: u32| {
        let whole = &whole;
        async move {
            let pack = second(format!("{tag}-{budget}"), budget).await;
            assert!(pack.tokens <= budget, "{}", pack.markdown);
            above_footer(&pack.markdown) == whole
        }
    };
    let (mut low, mut high) = (all.tokens / 2, 1000);
    assert!(
        !holds_all(low).await,
        "{low} holds a pack of {}",
        all.tokens
    );
    assert!(
        holds_all(high).await,
        "{high} does not hold:\n{}",
        all.markdown
    );
    // the smallest budget that holds it all is in (low, high]
    while high - low > 1 {
        let mid = low + (high - low) / 2;
        if holds_all(mid).await {
            high = mid;
        } else {
            low = mid;
        }
    }
    (high, all.markdown)
}

#[tokio::test]
async fn a_pack_that_only_refers_back_to_a_record_keeps_no_room_for_the_line() {
    let fx = fixture().await;
    // the task's own record keeps the records section in both measurements
    fx.store
        .add_record(record(
            fx.task.id,
            RecordKind::Attempt,
            "tried memoizing hello; broke tests",
            "",
            vec![anchor("app/greeter.py", Some("Greeter.hello"))],
        ))
        .await
        .unwrap();
    let (before, md) = smallest_whole_second_pack(&fx, "before").await;
    assert!(!md.contains(CORRECT_HINT), "{md}");

    // Another task's record, which the session's first pack shows (with the line) and its second
    // pack only refers back to (without it).
    let claim = earlier_record(
        &fx,
        RecordKind::Decision,
        "hello never ends in a mark",
        "app/greeter.py",
    )
    .await;
    let first = fx
        .packer
        .build(req(&fx.task, 4000, "after-first"))
        .await
        .unwrap();
    assert!(first.markdown.contains(CORRECT_HINT), "{}", first.markdown);
    let (after, md) = smallest_whole_second_pack(&fx, "after").await;
    let back = format!(
        "- ↩ rec:{} (sent earlier)\n",
        &claim.id.0.simple().to_string()[..8]
    );
    assert!(md.contains(&back), "{md}");
    assert!(!md.contains(CORRECT_HINT), "{md}");
    // The tight pack that holds everything grows by the back-reference at most, not by the line
    // too: a line that is not printed takes no room from the items (it would be about 25 tokens,
    // enough to push one out).
    let line = estimate_tokens(CORRECT_HINT, FAM);
    let grew = i64::from(after) - i64::from(before);
    let allowed = i64::from(estimate_tokens(&back, FAM)) + 4;
    assert!(
        allowed < i64::from(line),
        "the test cannot tell the line ({line} tokens) from the back-reference"
    );
    assert!(
        grew <= allowed,
        "everything fits from {before} tokens without the record and from {after} with its \
         back-reference; the line is {line} tokens"
    );
}

#[tokio::test]
async fn records_at_keeps_no_room_for_a_line_no_record_there_can_use() {
    let fx = fixture().await;
    earlier_record(
        &fx,
        RecordKind::Decision,
        "hello never ends in a mark",
        "app/greeter.py",
    )
    .await;
    // review findings only, more than one answer holds, each smaller than the line
    let reviewer = fx.store.create_task(new_task("A review")).await.unwrap();
    for i in 0..60 {
        fx.store
            .add_record(record(
                reviewer.id,
                RecordKind::Review,
                &format!("f{i}"),
                "",
                vec![anchor("a.py", None)],
            ))
            .await
            .unwrap();
    }
    let at = |path: &'static str, session: Option<&'static str>| {
        let packer = &fx.packer;
        let task = fx.task.id;
        async move { packer.records_at(task, path, session).await.unwrap() }
    };
    // a session that has had the line: its answers keep nothing back for it
    let told = at("app/greeter.py", Some("told")).await;
    assert!(
        told.markdown.ends_with(RECORDS_AT_HINT),
        "{}",
        told.markdown
    );
    let without = at("a.py", Some("told")).await;
    // a session that is still owed the line, at a file where no record can be superseded
    for session in [None, Some("fresh")] {
        let owed = at("a.py", session).await;
        assert!(!owed.markdown.contains("--supersedes"), "{}", owed.markdown);
        assert!(owed.records.len() < 60, "the cap did not bind");
        assert_eq!(
            owed.records.len(),
            without.records.len(),
            "room kept for a line that is not printed:\n{}",
            owed.markdown
        );
        assert!(
            estimate_tokens(&owed.markdown, AgentFamily::default())
                <= scholion_pack::RECORDS_AT_TOKENS
        );
    }
    // and the line is still owed where a record can be corrected
    let later = at("app/greeter.py", Some("fresh")).await;
    assert!(
        later.markdown.ends_with(RECORDS_AT_HINT),
        "{}",
        later.markdown
    );
}

// ---- standing: what became of the task a record comes from ----

/// Another task with one decision anchored on `Greeter.hello`, left in `status` (through a
/// merge and its revert for `reverted`).
async fn decision_of_task(fx: &Fx, title: &str, summary: &str, origin: &str) -> Record {
    use scholion_core::lifecycle::Event;
    let t = fx.store.create_task(new_task(title)).await.unwrap();
    let rec = fx
        .store
        .add_record(record(
            t.id,
            RecordKind::Decision,
            summary,
            "",
            vec![anchor("app/greeter.py", Some("Greeter.hello"))],
        ))
        .await
        .unwrap();
    match origin {
        "open" => {}
        "merged" => fx.store.set_status(t.id, TaskStatus::Merged).await.unwrap(),
        "abandoned" => {
            fx.store.transition(t.id, Event::Abandon).await.unwrap();
        }
        "reverted" => {
            fx.store.set_status(t.id, TaskStatus::Merged).await.unwrap();
            fx.store.transition(t.id, Event::Revert).await.unwrap();
        }
        other => panic!("{other}"),
    }
    rec
}

fn line_of<'a>(md: &'a str, needle: &str) -> &'a str {
    md.lines()
        .find(|l| l.contains(needle))
        .unwrap_or_else(|| panic!("no line with {needle:?}:\n{md}"))
}

#[tokio::test]
async fn a_records_line_says_where_it_comes_from_unless_that_is_merged_work() {
    let fx = fixture().await;
    for origin in ["merged", "open", "abandoned", "reverted"] {
        decision_of_task(&fx, origin, &format!("hello stays {origin}-pure"), origin).await;
    }
    // the reading task's own decision is not labelled: the agent knows where it comes from
    fx.store
        .add_record(record(
            fx.task.id,
            RecordKind::Decision,
            "hello is my own call",
            "",
            vec![anchor("app/greeter.py", Some("Greeter.hello"))],
        ))
        .await
        .unwrap();
    let md = fx
        .packer
        .build(req(&fx.task, 4000, "origin"))
        .await
        .unwrap()
        .markdown;
    assert!(
        line_of(&md, "merged-pure").contains(" decision: hello stays"),
        "{md}"
    );
    for origin in ["open", "abandoned", "reverted"] {
        let line = line_of(&md, &format!("{origin}-pure"));
        assert!(
            line.contains(&format!(" decision, {origin}: hello stays")),
            "{line}"
        );
    }
    assert!(
        line_of(&md, "my own call").contains(" decision: hello is"),
        "{md}"
    );
}

#[tokio::test]
async fn a_decision_of_reverted_work_ranks_below_the_same_of_merged_work() {
    let fx = fixture().await;
    // the reverted one is written last, so recency alone would put it first
    let merged = decision_of_task(&fx, "kept", "hello upper-cases the name", "merged").await;
    let reverted = decision_of_task(&fx, "undone", "hello lower-cases the name", "reverted").await;
    let pack = fx.packer.build(req(&fx.task, 4000, "rank")).await.unwrap();
    let md = &pack.markdown;
    assert!(
        pos(md, "upper-cases") < pos(md, "lower-cases"),
        "merged work first:\n{md}"
    );
    // without the origin the two tie but for age, and the newer one leads
    let _ = (merged, reverted);
    let fx = fixture().await;
    decision_of_task(&fx, "kept", "hello upper-cases the name", "merged").await;
    decision_of_task(&fx, "also kept", "hello lower-cases the name", "merged").await;
    let md = fx
        .packer
        .build(req(&fx.task, 4000, "rank"))
        .await
        .unwrap()
        .markdown;
    assert!(
        pos(&md, "lower-cases") < pos(&md, "upper-cases"),
        "the control: the order above comes from the origin\n{md}"
    );
}

#[tokio::test]
async fn a_dead_end_of_abandoned_work_keeps_its_rank() {
    let fx = fixture().await;
    use scholion_core::lifecycle::Event;
    let gone = fx.store.create_task(new_task("gone")).await.unwrap();
    fx.store
        .add_record(record(
            gone.id,
            RecordKind::Attempt,
            "memoizing hello broke the tests",
            "",
            vec![anchor("app/greeter.py", Some("Greeter.hello"))],
        ))
        .await
        .unwrap();
    fx.store.transition(gone.id, Event::Abandon).await.unwrap();
    decision_of_task(&fx, "kept", "hello upper-cases the name", "merged").await;
    let md = fx
        .packer
        .build(req(&fx.task, 4000, "dead"))
        .await
        .unwrap()
        .markdown;
    assert!(
        line_of(&md, "memoizing").contains(" attempt, abandoned: "),
        "{md}"
    );
    assert!(
        pos(&md, "memoizing") < pos(&md, "upper-cases"),
        "an attempt outranks a decision as before:\n{md}"
    );
}

#[tokio::test]
async fn records_at_and_expand_say_the_origin_too() {
    let fx = fixture().await;
    let rec = decision_of_task(&fx, "undone", "hello lower-cases the name", "reverted").await;
    let at = fx
        .packer
        .records_at(fx.task.id, "app/greeter.py", None)
        .await
        .unwrap();
    assert!(
        line_of(&at.markdown, "lower-cases").contains(" decision, reverted: "),
        "{}",
        at.markdown
    );
    let text = fx.packer.expand(fx.task.id, &short(&rec)).await.unwrap();
    assert!(text.contains(" decision, reverted: "), "{text}");
}
