use std::path::{Path, PathBuf};

use margincode_bench::conflict::{
    self, Agent, Category, CheckArgs, FakeMode, RunResult, Set, may_start, refusals,
    rule_quote_refusal, score, spent,
};
use tempfile::TempDir;

use crate::common::{real_av, serial, start_daemon};

fn set_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../bench/conflict-check")
}

fn the_set() -> Set {
    Set::load(&set_dir()).unwrap()
}

#[test]
fn the_committed_set_holds_section_8s_numbers() {
    let set = the_set();
    set.check().unwrap();
    assert_eq!(set.ids_of(Category::Planted).len(), 8);
    assert_eq!(set.controls().len(), 16);
}

#[test]
fn a_set_that_breaks_a_number_or_a_daemon_rule_is_refused() {
    let set_text = std::fs::read_to_string(set_dir().join("set.toml")).unwrap();
    let key_text = std::fs::read_to_string(set_dir().join("key.toml")).unwrap();
    let refused = |set: &str, key: &str, says: &str| {
        let e = Set::parse(&set_dir(), set, key)
            .unwrap()
            .check()
            .unwrap_err();
        assert!(e.contains(says), "{says}: {e}");
    };
    // a control made planted: 9 planted, 15 controls
    refused(
        &set_text,
        &key_text.replacen(
            "[r05]\ncategory = \"untouched\"",
            "[r05]\ncategory = \"planted\"\nrule = \"art-logs\"",
            1,
        ),
        "9 Planted, not 8",
    );
    // the older record is no longer seeded before the rules
    refused(
        &set_text.replacen("before_rules = true\n", "", 1),
        &key_text,
        "seeded before the rules",
    );
    // an attempt counted as an untouched control
    refused(
        &set_text,
        &key_text.replacen(
            "[r06]\ncategory = \"attempt\"",
            "[r06]\ncategory = \"untouched\"",
            1,
        ),
        "an attempt control is an attempt",
    );
    // a convention conflict on a record its glob does not cover
    refused(
        &set_text,
        &key_text.replacen("rule = \"conv-twice\"", "rule = \"conv-errors\"", 1),
        "conv-errors covers none of its anchors",
    );
}

/// What a model that reads the key would end with: the planted records `conflict` against
/// their rules with `--conflicts` and `confirmed` without; every control `confirmed` in both.
fn keyed(set: &Set) -> (RunResult, RunResult) {
    let mut with = RunResult::default();
    let mut without = RunResult::default();
    for (id, k) in &set.key {
        let confirmed = ("confirmed".to_owned(), None);
        without.accepted.insert(id.clone(), confirmed.clone());
        if k.category == Category::Planted {
            with.accepted
                .insert(id.clone(), ("conflict".to_owned(), k.rule.clone()));
            with.proposed_conflict.insert(id.clone());
        } else {
            with.accepted.insert(id.clone(), confirmed);
        }
    }
    (with, without)
}

fn passes(s: &conflict::Score) -> Vec<bool> {
    s.conditions.iter().map(|c| c.pass).collect()
}

#[test]
fn the_keyed_answer_passes_and_each_break_fails_its_condition_only() {
    let set = the_set();
    let planted: Vec<String> = set
        .ids_of(Category::Planted)
        .iter()
        .map(|s| s.to_string())
        .collect();
    let silent: Vec<String> = set
        .ids_of(Category::Silent)
        .iter()
        .map(|s| s.to_string())
        .collect();
    let (with, without) = keyed(&set);
    let s = score(&set, &with, &without);
    assert!(s.pass, "{s:#?}");
    assert_eq!(
        s.conditions.iter().map(|c| c.count).collect::<Vec<_>>(),
        [8, 0, 16, 0]
    );

    // (a): 6 of 8 still passes; 5 against the right rule does not, nor one left as confirmed
    let mut w = with.clone();
    for id in &planted[..3] {
        assert_ne!(set.key[id].rule.as_deref(), Some("art-utc"));
        w.accepted
            .insert(id.clone(), ("conflict".into(), Some("art-utc".into())));
    }
    let s = score(&set, &w, &without);
    assert_eq!(s.conditions[0].count, 5);
    assert_eq!(passes(&s), [false, true, true, true]);
    let mut w = with.clone();
    for id in &planted[..3] {
        w.accepted.insert(id.clone(), ("confirmed".into(), None));
    }
    assert_eq!(
        passes(&score(&set, &w, &without)),
        [false, true, true, true]
    );
    let mut w = with.clone();
    for id in &planted[..2] {
        w.accepted.remove(id);
    }
    assert_eq!(passes(&score(&set, &w, &without)), [true; 4]);

    // (b): one control sent as a conflict passes, two do not, refused or not; (c) stays at 14
    let mut w = with.clone();
    w.proposed_conflict.insert(silent[0].clone());
    w.accepted
        .insert(silent[0].clone(), ("conflict".into(), None));
    assert_eq!(passes(&score(&set, &w, &without)), [true; 4]);
    w.proposed_conflict.insert(silent[1].clone());
    w.accepted.remove(&silent[1]);
    let s = score(&set, &w, &without);
    assert_eq!(passes(&s), [true, false, true, true]);
    assert_eq!((s.conditions[1].count, s.conditions[2].count), (2, 14));

    // (c): no verdict is a kind of its own; three controls that differ fail it
    let controls = set.controls();
    let mut w = with.clone();
    for id in &controls[..3] {
        w.accepted.remove(*id);
    }
    assert_eq!(
        passes(&score(&set, &w, &without)),
        [true, true, false, true]
    );
    let mut wo = without.clone();
    for id in &controls[..3] {
        wo.accepted.remove(*id);
    }
    assert_eq!(passes(&score(&set, &w, &wo)), [true; 4]);

    // (d): two refused rule quotes pass, three do not
    let mut w = with.clone();
    w.rule_quote_refused.extend(planted[..2].iter().cloned());
    assert_eq!(passes(&score(&set, &w, &without)), [true; 4]);
    w.rule_quote_refused.insert(planted[2].clone());
    assert_eq!(
        passes(&score(&set, &w, &without)),
        [true, true, true, false]
    );
}

#[test]
fn refusals_are_read_round_by_round_and_only_a_rule_quote_counts_for_d() {
    let log = "dream 1a2b3c4d review cc-x: 24 to look at in /h -> running sh -c x\n\
               rec:0a1b2c3d conflict -> vd:aa proposed\n\
               rec:1b2c3d4e refused: evidence[1]: the quote is not in art:2 word for word\n\
               rec:2c3d4e5f refused: evidence[0]: the quote is not in rec:2c3d4e5f word for word\n\
               dream 1a2b3c4d: 2 refused; asking the agent to correct them\n\
               rec:1b2c3d4e refused: evidence[1]: the quote is not in conv:17 word for word\n\
               dream 1a2b3c4d done: 3 proposed, 1 refused\n";
    let rounds = refusals(log);
    assert_eq!(rounds.len(), 2);
    assert_eq!(rounds[0].len(), 2);
    assert_eq!(rounds[1][0].0, "rec:1b2c3d4e");
    assert!(rule_quote_refusal(&rounds[0][0].1));
    assert!(!rule_quote_refusal(&rounds[0][1].1), "a record quote");
    assert!(rule_quote_refusal(&rounds[1][0].1));
    assert!(!rule_quote_refusal(
        "the quote is not in storage/db.py at commit abc word for word"
    ));
    assert!(!rule_quote_refusal(
        "art:9 is not a rule in force in repo x: name one from the item's `rules`"
    ));
}

#[test]
fn spend_counts_each_call_and_a_call_with_no_cost_at_its_cap() {
    let out = TempDir::new().unwrap();
    let run = out.path().join("conflicts/home/dreams/d1");
    std::fs::create_dir_all(&run).unwrap();
    std::fs::write(
        run.join("claude-result-1.json"),
        r#"{"total_cost_usd": 0.12}"#,
    )
    .unwrap();
    std::fs::write(run.join("claude-result-2.json"), "").unwrap();
    std::fs::write(run.join("bundle.json"), r#"{"total_cost_usd": 9}"#).unwrap();
    let got = spent(out.path());
    assert!(
        (got - (0.12 + conflict::CALL_CAP_USD)).abs() < 1e-9,
        "{got}"
    );
    assert!(may_start(0.0) && may_start(1.1));
    assert!(
        !may_start(1.11),
        "a run of two calls at $0.45 would pass $2"
    );
}

/// The whole path with the scripted agent: the real `av` and daemon, two stores, two runs.
#[test]
fn a_dry_check_passes_with_the_keyed_agent_and_each_break_fails_its_condition() {
    let _g = serial();
    let Some(av) = real_av() else {
        println!("skip: no av binary beside the test binary (cargo build -p margincode-cli)");
        return;
    };
    let d = start_daemon(None);
    let tmp = TempDir::new().unwrap();
    let import = tmp.path().join("src");
    std::fs::create_dir_all(&import).unwrap();
    for (mode, want) in [
        (FakeMode::Keyed, [true; 4]),
        (FakeMode::BreakA, [false, true, true, true]),
        (FakeMode::BreakB, [true, false, true, true]),
        (FakeMode::BreakC, [true, true, false, true]),
        (FakeMode::BreakD, [true, true, true, false]),
    ] {
        let out = tmp.path().join(mode.name());
        if mode == FakeMode::Keyed {
            // a check stopped before result.json: started again on the same --out
            let old = out.join("ccstopped/conflicts/home/dreams/d0");
            std::fs::create_dir_all(&old).unwrap();
            std::fs::write(old.join("bundle.json"), "{}").unwrap();
            std::fs::write(
                old.join("claude-result-1.json"),
                r#"{"total_cost_usd": 0.12}"#,
            )
            .unwrap();
        }
        let args = CheckArgs {
            set_dir: set_dir(),
            out: out.clone(),
            av_bin: av.to_string_lossy().into(),
            av_url: d.url.clone(),
            av_token: None,
            import_dir: import.clone(),
            import_mount: import.to_string_lossy().into(),
            agent: Agent::Fake {
                exe: PathBuf::from(env!("CARGO_BIN_EXE_margincode-bench")),
                mode,
            },
        };
        let o = conflict::check(&args).unwrap_or_else(|e| panic!("{}: {e}", mode.name()));
        assert_eq!(passes(&o.score), want, "{}: {}", mode.name(), o.report);
        assert_eq!(o.score.pass, mode == FakeMode::Keyed);
        let stopped = if mode == FakeMode::Keyed { 0.12 } else { 0.0 };
        assert!((o.spent_usd - stopped).abs() < 1e-9, "{}", o.spent_usd);
        assert!(out.join("result.json").is_file() && out.join("report.txt").is_file());
        if mode == FakeMode::Keyed {
            assert_eq!(
                o.score
                    .conditions
                    .iter()
                    .map(|c| c.count)
                    .collect::<Vec<_>>(),
                [8, 0, 16, 0],
                "{}",
                o.report
            );
            let e = conflict::check(&args).unwrap_err();
            assert!(e.contains("finished check"), "{e}");
        }
        if mode == FakeMode::BreakD {
            assert_eq!(o.score.conditions[0].count, 8, "corrected in the fix round");
        }
    }
}
