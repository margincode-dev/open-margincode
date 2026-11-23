//! What a merged task's context packs were worth: the code they sent against the files the task
//! changed and the files its agent opened. Pure: the daemon hands in the task's receipts, the
//! merge's changed paths and the task's reads (`task_reads`), and stores the answer.
//!
//! The files that count are the benchmark's gold files (`margincode-bench retrieval`): modified,
//! in a language the index parses, and not a test. A file the task added could not have been in a
//! pack; a test file is not what a pack is for.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use serde::Serialize;

use crate::{ReceiptItem, Tier};

/// Languages the index parses; only their files can be carried as code.
pub fn indexable(path: &str) -> bool {
    matches!(
        path.rsplit_once('.').map(|(_, e)| e),
        Some("py" | "ts" | "tsx" | "rs")
    )
}

pub fn is_test_path(path: &str) -> bool {
    let p = path.to_ascii_lowercase();
    let dirs: Vec<&str> = p.split('/').collect();
    let file = dirs.last().copied().unwrap_or("");
    dirs[..dirs.len().saturating_sub(1)].iter().any(|d| {
        matches!(
            *d,
            "test" | "tests" | "__tests__" | "spec" | "specs" | "testing"
        )
    }) || file.starts_with("test_")
        || file == "conftest.py"
        || [
            "_test.py",
            "_tests.py",
            "_test.rs",
            ".test.ts",
            ".test.tsx",
            ".spec.ts",
            ".spec.tsx",
        ]
        .iter()
        .any(|s| file.ends_with(s))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathChange {
    Added,
    Modified,
    Deleted,
}

/// Whether a changed path is one the measures count (the benchmark's gold rule).
pub fn counted(change: PathChange, path: &str) -> bool {
    change == PathChange::Modified && indexable(path) && !is_test_path(path)
}

/// One stored pack: its session (empty: none) and its items, as the receipt has them.
#[derive(Debug, Clone)]
pub struct SentPack {
    pub session: String,
    pub items: Vec<ReceiptItem>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Measures {
    /// Tokens of code items sent, back-references left out.
    pub code_tokens: u64,
    /// Of those, the tokens on files the task changed.
    pub code_tokens_changed: u64,
    /// Of those, the tokens on files the task changed or its agent opened.
    pub code_tokens_used: u64,
    /// The changed files that count, sorted.
    pub changed: Vec<String>,
    /// Changed files no pack carried as code nor named.
    pub missed: Vec<String>,
    /// Changed files a pack named (a handle, a map line) but never carried as code.
    pub named_only: Vec<String>,
    /// Opened files that count and that no pack carried as code.
    pub opened_not_sent: Vec<String>,
}

/// `packs` oldest first. An item is a back-reference when an earlier pack of the same session
/// sent it at this tier or higher: the session ledger then sent "sent earlier" in its place.
pub fn measure(
    packs: &[SentPack],
    changed: &[(String, PathChange)],
    opened: &[String],
) -> Measures {
    let added: BTreeSet<&str> = changed
        .iter()
        .filter(|(_, c)| *c == PathChange::Added)
        .map(|(p, _)| p.as_str())
        .collect();
    let changed: BTreeSet<&str> = changed
        .iter()
        .filter(|(p, c)| counted(*c, p))
        .map(|(p, _)| p.as_str())
        .collect();
    let opened: BTreeSet<&str> = opened
        .iter()
        .map(String::as_str)
        .filter(|p| indexable(p) && !is_test_path(p) && !added.contains(p))
        .collect();
    let mut m = Measures::default();
    let mut carried: BTreeSet<&str> = BTreeSet::new();
    let mut named: BTreeSet<&str> = BTreeSet::new();
    let mut seen: HashMap<&str, HashMap<&str, Tier>> = HashMap::new();
    for pack in packs {
        let ledger = seen.entry(pack.session.as_str()).or_default();
        let mut sent: BTreeMap<&str, Tier> = BTreeMap::new();
        for item in &pack.items {
            if let Some(path) = item.id.strip_prefix("file:") {
                named.insert(path);
                continue;
            }
            let Some((path, _, _)) = item.source.as_ref().filter(|_| item.id.starts_with("sym:"))
            else {
                continue;
            };
            if item.tier >= Tier::Summary {
                carried.insert(path);
            } else {
                named.insert(path);
            }
            let back_ref = !pack.session.is_empty()
                && ledger
                    .get(item.id.as_str())
                    .is_some_and(|t| item.tier <= *t);
            if !back_ref {
                let tokens = u64::from(item.tokens);
                m.code_tokens += tokens;
                if changed.contains(path.as_str()) {
                    m.code_tokens_changed += tokens;
                }
                if changed.contains(path.as_str()) || opened.contains(path.as_str()) {
                    m.code_tokens_used += tokens;
                }
            }
            sent.insert(&item.id, item.tier);
        }
        for (id, tier) in sent {
            let t = ledger.entry(id).or_insert(tier);
            *t = (*t).max(tier);
        }
    }
    m.changed = changed.iter().map(|p| p.to_string()).collect();
    for p in &changed {
        if carried.contains(p) {
            continue;
        }
        if named.contains(p) {
            m.named_only.push(p.to_string());
        } else {
            m.missed.push(p.to_string());
        }
    }
    m.opened_not_sent = opened
        .iter()
        .filter(|p| !carried.contains(*p))
        .map(|p| p.to_string())
        .collect();
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    fn code(id: &str, path: &str, tier: Tier, tokens: u32) -> ReceiptItem {
        ReceiptItem {
            id: id.into(),
            tier,
            score: 1.0,
            reason: "seed".into(),
            source: Some((path.into(), 1, 9)),
            tokens,
        }
    }

    fn other(id: &str, tier: Tier, tokens: u32) -> ReceiptItem {
        ReceiptItem {
            id: id.into(),
            tier,
            score: 1.0,
            reason: "map".into(),
            source: None,
            tokens,
        }
    }

    fn pack(session: &str, items: Vec<ReceiptItem>) -> SentPack {
        SentPack {
            session: session.into(),
            items,
        }
    }

    fn modified(paths: &[&str]) -> Vec<(String, PathChange)> {
        paths
            .iter()
            .map(|p| (p.to_string(), PathChange::Modified))
            .collect()
    }

    #[test]
    fn gold_files_are_modified_indexed_non_test_files() {
        for p in [
            "tests/x.py",
            "src/app_test.py",
            "web/foo.test.ts",
            "a/__tests__/b.tsx",
            "test_x.py",
            "pkg/spec/a.rs",
            "Src/Testing/a.py",
            "conftest.py",
            "a/b_tests.py",
        ] {
            assert!(is_test_path(p), "{p}");
        }
        for p in [
            "src/latest.py",
            "src/testing_utils.py",
            "src/contest.rs",
            "app/greeter.py",
        ] {
            assert!(!is_test_path(p), "{p}");
        }
        assert!(counted(PathChange::Modified, "app/util.py"));
        assert!(!counted(PathChange::Added, "app/new.py"));
        assert!(!counted(PathChange::Deleted, "app/old.py"));
        assert!(!counted(PathChange::Modified, "tests/test_util.py"));
        assert!(!counted(PathChange::Modified, "docs/reference.md"));
        assert!(!counted(PathChange::Modified, "web/app.js"));
    }

    #[test]
    fn shares_count_code_tokens_on_changed_and_opened_files() {
        let packs = [pack(
            "s",
            vec![
                code("sym:1", "app/util.py", Tier::Full, 60),
                code("sym:2", "app/greeter.py", Tier::Summary, 30),
                code("sym:3", "app/types.py", Tier::Summary, 10),
                other("rec:ab", Tier::Summary, 500),
                other("file:app/types.py", Tier::Summary, 40),
            ],
        )];
        let m = measure(
            &packs,
            &modified(&["app/util.py"]),
            &["app/greeter.py".into()],
        );
        assert_eq!(m.code_tokens, 100);
        assert_eq!(m.code_tokens_changed, 60);
        assert_eq!(m.code_tokens_used, 90);
        assert_eq!(m.changed, vec!["app/util.py"]);
        assert!(m.missed.is_empty() && m.named_only.is_empty(), "{m:?}");
        assert!(m.opened_not_sent.is_empty(), "{m:?}");
    }

    #[test]
    fn a_back_reference_in_the_same_session_counts_no_tokens() {
        let first = vec![
            code("sym:1", "app/util.py", Tier::Summary, 40),
            code("sym:2", "app/greeter.py", Tier::Handle, 5),
        ];
        let second = vec![
            code("sym:1", "app/util.py", Tier::Summary, 8),
            code("sym:2", "app/greeter.py", Tier::Summary, 20),
        ];
        let m = measure(
            &[pack("s", first.clone()), pack("s", second.clone())],
            &modified(&["app/util.py", "app/greeter.py"]),
            &[],
        );
        assert_eq!(
            m.code_tokens,
            40 + 5 + 20,
            "an upgrade to a higher tier is new text"
        );
        let m = measure(
            &[pack("", first.clone()), pack("", second.clone())],
            &modified(&["app/util.py"]),
            &[],
        );
        assert_eq!(m.code_tokens, 40 + 5 + 8 + 20, "no session, no ledger");
        let m = measure(
            &[pack("a", first), pack("b", second)],
            &modified(&["app/util.py"]),
            &[],
        );
        assert_eq!(m.code_tokens, 73, "another session's ledger is its own");
    }

    #[test]
    fn changed_files_are_carried_named_only_or_missed() {
        let packs = [
            pack(
                "s",
                vec![
                    code("sym:1", "app/a.py", Tier::Summary, 10),
                    code("sym:2", "app/b.py", Tier::Handle, 2),
                    other("file:app/c.py", Tier::Handle, 2),
                ],
            ),
            pack("s", vec![code("sym:9", "app/b2.py", Tier::Full, 10)]),
        ];
        let mut changed = modified(&[
            "app/a.py",
            "app/b.py",
            "app/b2.py",
            "app/c.py",
            "app/d.py",
            "tests/test_a.py",
            "README.md",
        ]);
        changed.push(("app/new.py".into(), PathChange::Added));
        changed.push(("app/gone.py".into(), PathChange::Deleted));
        let opened: Vec<String> = [
            "app/b.py",
            "app/a.py",
            "app/new.py",
            "app/e.py",
            "tests/t.py",
        ]
        .map(String::from)
        .to_vec();
        let m = measure(&packs, &changed, &opened);
        assert_eq!(
            m.changed,
            vec!["app/a.py", "app/b.py", "app/b2.py", "app/c.py", "app/d.py"]
        );
        assert_eq!(m.named_only, vec!["app/b.py", "app/c.py"]);
        assert_eq!(m.missed, vec!["app/d.py"]);
        assert_eq!(
            m.changed.len(),
            2 + m.named_only.len() + m.missed.len(),
            "changed = carried + named only + missed"
        );
        assert_eq!(m.opened_not_sent, vec!["app/b.py", "app/e.py"]);
    }

    #[test]
    fn a_task_without_packs_misses_every_changed_file() {
        let m = measure(&[], &modified(&["app/a.py"]), &[]);
        assert_eq!(m.code_tokens, 0);
        assert_eq!(m.missed, vec!["app/a.py"]);
    }
}
