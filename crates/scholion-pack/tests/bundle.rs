mod common;

use common::*;
use scholion_core::*;
use scholion_pack::{BundleRequest, estimate_tokens};

const FAM: AgentFamily = AgentFamily::Other;

fn pos(haystack: &str, needle: &str) -> usize {
    haystack
        .find(needle)
        .unwrap_or_else(|| panic!("{needle:?} not in bundle:\n{haystack}"))
}

fn state() -> HandoffState {
    HandoffState {
        done: "hello now ends with an exclamation mark".into(),
        remaining: "make goodbye louder too".into(),
        broken: "test_goodbye fails".into(),
        questions: "should goodbye shout in all caps?".into(),
    }
}

/// The fixture task with a decision, a dead end, a note, a superseded note and a handoff record.
async fn handed_off() -> Fx {
    let fx = fixture().await;
    let add = |kind, summary: &str, body: &str, anchors| {
        fx.store
            .add_record(record(fx.task.id, kind, summary, body, anchors))
    };
    add(RecordKind::Note, "greeting style lives in util", "", vec![])
        .await
        .unwrap();
    add(
        RecordKind::Decision,
        "append the mark in hello, not in format_name",
        "format_name is shared with goodbye",
        vec![anchor("app/greeter.py", Some("Greeter.hello"))],
    )
    .await
    .unwrap();
    add(
        RecordKind::Attempt,
        "tried upper() in format_name; broke other callers",
        "",
        vec![anchor("app/util.py", Some("format_name"))],
    )
    .await
    .unwrap();
    let old = add(RecordKind::Note, "old guess about goodbye", "", vec![])
        .await
        .unwrap();
    let mut newer = record(
        fx.task.id,
        RecordKind::Note,
        "goodbye is out of scope for now",
        "",
        vec![],
    );
    newer.supersedes = Some(old.id);
    fx.store.add_record(newer).await.unwrap();
    let st = state();
    fx.store
        .add_record(record(
            fx.task.id,
            RecordKind::Handoff,
            &st.remaining,
            &serde_json::to_string(&st).unwrap(),
            vec![],
        ))
        .await
        .unwrap();
    fx
}

fn breq(task: &Task, budget: u32) -> BundleRequest {
    BundleRequest {
        task_id: task.id,
        budget_tokens: budget,
        family: FAM,
    }
}

#[tokio::test]
async fn bundle_is_within_budget_for_budgets_150_1000_8000() {
    let fx = handed_off().await;
    for budget in [150, 1000, 8000] {
        let b = fx
            .packer
            .handoff_bundle(breq(&fx.task, budget))
            .await
            .unwrap();
        assert!(
            b.tokens <= budget,
            "{} > {budget}:\n{}",
            b.tokens,
            b.markdown
        );
        assert_eq!(b.tokens, estimate_tokens(&b.markdown, FAM));
        assert!(b.markdown.contains("## Goal"), "{budget}:\n{}", b.markdown);
    }
}

#[tokio::test]
async fn bundle_sections_come_in_order_with_decisions_and_dead_ends_first() {
    let fx = handed_off().await;
    let b = fx
        .packer
        .handoff_bundle(breq(&fx.task, 8000))
        .await
        .unwrap();
    let md = &b.markdown;
    let order = [
        "## Goal",
        "## State",
        "## Records",
        "## Changes",
        "## Open questions",
    ];
    for w in order.windows(2) {
        assert!(
            pos(md, w[0]) < pos(md, w[1]),
            "{} before {}:\n{md}",
            w[0],
            w[1]
        );
    }
    assert!(pos(md, "test_goodbye fails") < pos(md, "## Records"));
    assert!(pos(md, "all caps?") > pos(md, "## Open questions"));
    let records = &md[pos(md, "## Records")..pos(md, "## Changes")];
    assert!(
        pos(records, "decision:") < pos(records, "greeting style"),
        "{records}"
    );
    assert!(
        pos(records, "attempt:") < pos(records, "greeting style"),
        "{records}"
    );
    assert!(
        !md.contains("old guess"),
        "superseded records are left out:\n{md}"
    );
    assert!(
        !records.contains("handoff:") && !records.contains("intent:"),
        "the handoff state and the goal have their own sections:\n{records}"
    );
    // with room to spare, the changed symbol comes with its hunk
    let changes = &md[pos(md, "## Changes")..pos(md, "## Open questions")];
    assert!(changes.contains("Greeter.hello"), "{changes}");
    assert!(
        changes.contains("+        return format_name(name) + \"!\""),
        "{changes}"
    );
}

#[tokio::test]
async fn small_budget_keeps_handles_and_lists_what_was_left_out() {
    let fx = handed_off().await;
    let b = fx.packer.handoff_bundle(breq(&fx.task, 150)).await.unwrap();
    assert!(!b.markdown.contains("```diff"), "{}", b.markdown);
    assert!(
        b.markdown.contains("make goodbye louder"),
        "remaining is never dropped:\n{}",
        b.markdown
    );
}

#[tokio::test]
async fn bundle_without_a_handoff_record_still_renders() {
    let fx = fixture().await;
    let b = fx
        .packer
        .handoff_bundle(breq(&fx.task, 1000))
        .await
        .unwrap();
    assert!(b.markdown.contains("## State\n(none)"), "{}", b.markdown);
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
async fn bundle_matches_golden_for_fixture_task() {
    let fx = handed_off().await;
    let b = fx
        .packer
        .handoff_bundle(breq(&fx.task, 2000))
        .await
        .unwrap();
    let got = normalize(&b.markdown);
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/handoff.md");
    if std::env::var("UPDATE_GOLDEN").is_ok() {
        std::fs::write(path, &got).unwrap();
    }
    let want = std::fs::read_to_string(path)
        .expect("golden file missing: run with UPDATE_GOLDEN=1 and review it");
    assert_eq!(got, want);
}

/// Plan 9: an agent that reads `git status` in a workspace sees nothing (av snapshots edits into
/// commits), so both bundles say so under the workspace path.
#[tokio::test]
async fn bundles_say_a_workspace_shows_no_git_changes() {
    let fx = handed_off().await;
    let note = "(edits there are snapshotted into commits, so git status shows nothing; \
                the changes are listed below)";
    let h = fx
        .packer
        .handoff_bundle(breq(&fx.task, 2000))
        .await
        .unwrap();
    let p = fx.packer.pr_bundle(breq(&fx.task, 2000)).await.unwrap();
    for md in [&h.markdown, &p.markdown] {
        let ws = pos(md, "Workspace: ");
        assert!(md[ws..].lines().nth(1) == Some(note), "{md}");
    }
}

#[tokio::test]
async fn pr_bundle_lists_conventions_checklist_and_risks() {
    let fx = handed_off().await;
    fx.store
        .add_convention("demo", "app/**/*.py", "use type hints everywhere")
        .await
        .unwrap();
    // a decision on Greeter.goodbye with nothing tried there
    fx.store
        .add_record(record(
            fx.task.id,
            RecordKind::Decision,
            "goodbye keeps its old wording",
            "",
            vec![anchor("app/greeter.py", Some("Greeter.goodbye"))],
        ))
        .await
        .unwrap();
    let b = fx.packer.pr_bundle(breq(&fx.task, 8000)).await.unwrap();
    let md = &b.markdown;
    for w in [
        "## Goal",
        "## Tests",
        "## Risks",
        "## Checklist",
        "## Changes",
    ]
    .windows(2)
    {
        assert!(
            pos(md, w[0]) < pos(md, w[1]),
            "{} before {}:\n{md}",
            w[0],
            w[1]
        );
    }
    let checklist = &md[pos(md, "## Checklist")..pos(md, "## Changes")];
    assert!(
        checklist.contains("- [ ] conv:") && checklist.contains("type hints"),
        "{checklist}"
    );
    let risks = &md[pos(md, "## Risks")..pos(md, "## Checklist")];
    assert!(
        risks.contains("no alternatives recorded: goodbye keeps its old wording"),
        "{risks}"
    );
    // the attempt in the fixture is a dead end, shown under Records, never as a risk
    assert!(
        !risks.contains("tried upper()"),
        "attempts are not risks:\n{risks}"
    );
}

#[tokio::test]
async fn pr_bundle_reports_test_status_at_the_current_head() {
    let fx = handed_off().await;
    let b = fx.packer.pr_bundle(breq(&fx.task, 2000)).await.unwrap();
    assert!(
        b.markdown.contains("Tests: not run at this head"),
        "{}",
        b.markdown
    );
    // whoever reads the bundle (a reviewer, too) may run the tests and attest them
    assert!(
        b.markdown
            .contains("); run them in the workspace, then av validate -- <cmd>"),
        "{}",
        b.markdown
    );
    let mut older = record(
        fx.task.id,
        RecordKind::Validation,
        "`pytest` exit 0",
        r#"{"command":"pytest","exit_code":0,"head_change":"base"}"#,
        vec![],
    );
    older.change_id = Some(ChangeId("base".into()));
    fx.store.add_record(older).await.unwrap();
    let b = fx.packer.pr_bundle(breq(&fx.task, 2000)).await.unwrap();
    assert!(
        b.markdown.contains("Tests: not run at this head"),
        "a validation at an older head does not count:\n{}",
        b.markdown
    );
    let mut failed = record(
        fx.task.id,
        RecordKind::Validation,
        "`pytest` exit 1",
        r#"{"command":"pytest","exit_code":1,"head_change":"head"}"#,
        vec![],
    );
    failed.change_id = Some(ChangeId("head".into()));
    fx.store.add_record(failed).await.unwrap();
    let b = fx.packer.pr_bundle(breq(&fx.task, 2000)).await.unwrap();
    assert!(
        b.markdown.contains("Tests: `pytest` FAILED (exit 1)"),
        "{}",
        b.markdown
    );
    assert!(b.tokens <= 2000);
}
