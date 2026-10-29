//! Scoring (Plan 1 weights; Plan 2 adds a semantic term and re-balances).

use std::time::Duration;

use scholion_core::RecordKind;

pub const W_STRUCT: f32 = 0.35;
pub const W_SEM: f32 = 0.30;
pub const W_RECENCY: f32 = 0.15;
pub const W_KIND: f32 = 0.20;
/// Added to records anchored directly on a seed symbol. Outside the weights, which sum to 1.
pub const SEED_BONUS: f32 = 1.0;
/// Per-collection cap for `Semantic::search` and for new semantic or lexical code items at the
/// smallest budgets; `candidate_limit` raises it with the budget.
pub const SEMANTIC_LIMIT: usize = 8;
/// Upper bound of `candidate_limit`.
pub const MAX_CANDIDATES: usize = 40;
/// Reciprocal rank fusion (handoff A1): a code hit counts as similarity
/// `RRF_SIM * (RRF_K + 1) / (RRF_K + 1 + rank)` from each search that found it (lexical BM25,
/// semantic), so the top hit of one search is `RRF_SIM` and a symbol both find adds up both.
pub const RRF_SIM: f32 = 0.75;
pub const RRF_K: f32 = 60.0;

/// The similarity a code hit at 0-based `rank` of one search counts as.
pub fn rrf_sim(rank: usize) -> f32 {
    RRF_SIM * (RRF_K + 1.0) / (RRF_K + 1.0 + rank as f32)
}

/// How many semantic and lexical code candidates a pack of `budget` tokens considers: about one
/// per 250 tokens, so a larger budget is filled rather than left empty (Plan 6, E1).
pub fn candidate_limit(budget: u32) -> usize {
    (budget as usize / 250).clamp(SEMANTIC_LIMIT, MAX_CANDIDATES)
}
/// Semantic search is abandoned this long after it starts.
pub const SEMANTIC_TIMEOUT: Duration = Duration::from_millis(150);
/// Hop distance assigned to anything not connected to the seeds.
pub const FAR_HOPS: u8 = 3;
/// Score of a record that belongs to the task itself but carries no anchors.
pub const RECENCY_HALF_LIFE_DAYS: f64 = 30.0;

pub fn kind_priority(kind: RecordKind) -> f32 {
    match kind {
        RecordKind::Attempt => 1.0,
        RecordKind::Review => 0.9,
        RecordKind::Validation => 0.3,
        RecordKind::Decision => 0.8,
        RecordKind::Intent => 0.7,
        RecordKind::Handoff => 0.6,
        RecordKind::Note => 0.4,
    }
}

/// 1.0 for brand-new, halving every 30 days.
pub fn recency(age_days: f64) -> f32 {
    2f64.powf(-age_days.max(0.0) / RECENCY_HALF_LIFE_DAYS) as f32
}

pub fn structural(hops: u8) -> f32 {
    1.0 / (1.0 + hops as f32)
}

pub(crate) fn clamp_sim(semantic: f32) -> f32 {
    semantic.clamp(0.0, 1.0)
}

pub fn record_score(
    kind: RecordKind,
    hops: u8,
    age_days: f64,
    on_seed: bool,
    semantic: f32,
) -> f32 {
    W_STRUCT * structural(hops)
        + W_SEM * clamp_sim(semantic)
        + W_RECENCY * recency(age_days)
        + W_KIND * kind_priority(kind)
        + if on_seed { SEED_BONUS } else { 0.0 }
}

/// Code has no age. Recency contributes its full weight; the kind term stays at the midpoint.
/// Callers multiply by `TEST_CODE_FACTOR` for test paths.
pub fn code_score(hops: u8, semantic: f32) -> f32 {
    W_STRUCT * structural(hops) + W_SEM * clamp_sim(semantic) + W_RECENCY + W_KIND * 0.5
}

/// `0.5 + W_SEM * clamp(semantic)`. Exactly `0.5` when `semantic` is 0.
pub fn convention_score(semantic: f32) -> f32 {
    0.5 + W_SEM * clamp_sim(semantic)
}

/// Code in test files is rarely what an agent needs first; it is ranked down, never dropped.
pub const TEST_CODE_FACTOR: f32 = 0.6;
/// Records whose anchored code changed since they were written rank lower (Plan 2b).
pub const CHANGED_FACTOR: f32 = 0.7;
/// Another task's intent restates that task's goal, which is often worded like ours, so
/// semantic search ranks it high; it is rarely what an agent needs, so it ranks lower.
pub const FOREIGN_INTENT_FACTOR: f32 = 0.4;

pub fn is_test_path(path: &str) -> bool {
    let mut segments = path.split('/');
    let file = segments.next_back().unwrap_or("");
    segments.any(|s| matches!(s, "tests" | "test" | "__tests__"))
        || file.starts_with("test_")
        || [
            "_test.py",
            "_test.rs",
            ".test.ts",
            ".test.tsx",
            ".spec.ts",
            ".spec.tsx",
        ]
        .iter()
        .any(|suffix| file.ends_with(suffix))
}

/// Test code: a test path, a `test_` function, a Rust `#[test]`-style attribute just above the
/// symbol, or anything inside an inline `#[cfg(test)] mod NAME { ... }` (where Rust keeps its
/// unit tests). `start_line` is 1-based; without the source only the path and name are checked.
pub fn is_test_code(path: &str, name: &str, start_line: u32, src: Option<&str>) -> bool {
    let leaf = name.rsplit('.').next().unwrap_or(name);
    if is_test_path(path) || leaf.starts_with("test_") {
        return true;
    }
    let Some(src) = src.filter(|_| path.ends_with(".rs")) else {
        return false;
    };
    let lines: Vec<&str> = src.lines().map(str::trim).collect();
    if in_cfg_test_module(&lines, start_line) {
        return true;
    }
    lines
        .iter()
        .take(start_line.saturating_sub(1) as usize)
        .rev()
        .take_while(|l| l.starts_with("#[") || l.starts_with("///") || l.is_empty())
        .any(|l| l.starts_with("#[") && l.contains("test") && !l.starts_with("#[cfg(not(test"))
}

/// Whether the 1-based `line` falls inside an inline `#[cfg(test)] mod NAME { ... }`. A
/// `#[cfg(test)]` on a `mod NAME;` declaration, an import or anything else gates no block.
fn in_cfg_test_module(lines: &[&str], line: u32) -> bool {
    let at = line.saturating_sub(1) as usize;
    let mut i = 0;
    while i < lines.len() && i <= at {
        if !lines[i].starts_with("#[cfg(test)]") {
            i += 1;
            continue;
        }
        // the item the attribute gates: the rest of this line, else the first line below that is
        // neither another attribute nor blank
        let rest = lines[i]["#[cfg(test)]".len()..].trim();
        let mut item = i;
        let head = if rest.is_empty() || rest.starts_with("#[") {
            loop {
                item += 1;
                match lines.get(item) {
                    Some(l) if l.is_empty() || l.starts_with("#[") => continue,
                    other => break other.copied().unwrap_or(""),
                }
            }
        } else {
            rest
        };
        if !is_inline_mod(head) {
            i += 1;
            continue;
        }
        // brace-match from the module's `{` to its end (or the file's, if unbalanced)
        let mut depth = 0i32;
        let mut end = lines.len() - 1;
        'scan: for (n, l) in lines.iter().enumerate().skip(item) {
            for c in code_chars(l) {
                match c {
                    '{' => depth += 1,
                    '}' => {
                        depth -= 1;
                        if depth == 0 {
                            end = n;
                            break 'scan;
                        }
                    }
                    _ => {}
                }
            }
        }
        if at <= end {
            return true;
        }
        i = end + 1;
    }
    false
}

/// `mod NAME {` (any visibility), as opposed to `mod NAME;`.
fn is_inline_mod(line: &str) -> bool {
    let mut words = line.split_whitespace();
    let mut w = words.next();
    if w.is_some_and(|w| w == "pub" || w.starts_with("pub(")) {
        w = words.next();
    }
    w == Some("mod") && line.contains('{')
}

/// The characters of a line that are code: string literals, char literals and `//` comments are
/// dropped, so a brace inside them does not count.
fn code_chars(line: &str) -> impl Iterator<Item = char> + '_ {
    let mut chars = line.chars().peekable();
    let mut in_str = false;
    std::iter::from_fn(move || {
        while let Some(c) = chars.next() {
            if in_str {
                match c {
                    '\\' => {
                        chars.next();
                    }
                    '"' => in_str = false,
                    _ => {}
                }
                continue;
            }
            match c {
                '"' => in_str = true,
                '/' if chars.peek() == Some(&'/') => return None,
                '\'' => {
                    // a char literal ('{', '\n') is skipped; a lifetime ('a) has no closing quote
                    let mut look = chars.clone();
                    let a = look.next();
                    let closes = if a == Some('\\') {
                        look.next();
                        look.next() == Some('\'')
                    } else {
                        look.next() == Some('\'')
                    };
                    if closes {
                        chars = look;
                    }
                }
                _ => return Some(c),
            }
        }
        None
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn candidate_limit_grows_with_the_budget() {
        assert_eq!(candidate_limit(500), SEMANTIC_LIMIT);
        assert_eq!(candidate_limit(2000), 8);
        assert_eq!(candidate_limit(4000), 16);
        assert_eq!(candidate_limit(8000), 32);
        assert_eq!(candidate_limit(100_000), MAX_CANDIDATES);
    }

    #[test]
    fn recognises_test_paths() {
        for p in [
            "tests/a.py",
            "backend/tests/test_x.py",
            "pkg/test_util.py",
            "src/a_test.rs",
            "web/app.test.ts",
            "web/__tests__/a.ts",
        ] {
            assert!(is_test_path(p), "{p}");
        }
        for p in [
            "app/greeter.py",
            "src/latest.rs",
            "contest/a.py",
            "app/testing_tools.py",
            "docs/tests.md.py",
        ] {
            assert!(!is_test_path(p), "{p}");
        }
    }

    #[test]
    fn test_code_is_found_by_its_source_not_only_its_path() {
        let rs = "pub fn add_parents() {}\n\n#[test]\nfn absolute_parent() {}\n\n#[tokio::test(flavor = \"multi_thread\")]\nasync fn spawns() {}\n\n#[cfg(test)]\nmod tests {\n    fn helper() {}\n}\n";
        assert!(!is_test_code("src/dir.rs", "add_parents", 1, Some(rs)));
        assert!(is_test_code("src/dir.rs", "absolute_parent", 4, Some(rs)));
        assert!(is_test_code("src/dir.rs", "spawns", 7, Some(rs)));
        assert!(
            is_test_code("src/dir.rs", "helper", 11, Some(rs)),
            "after #[cfg(test)]"
        );
        assert!(is_test_code("app/x.py", "test_split", 1, None));
        assert!(is_test_code("tests/a.rs", "anything", 1, None));
        assert!(!is_test_code("app/x.py", "latest_split", 1, None));
        assert!(!is_test_code("src/dir.rs", "add_parents", 1, None));
    }

    #[test]
    fn only_an_inline_cfg_test_module_marks_what_is_inside_it() {
        // `#[cfg(test)] mod tests;` lives in another file: the fn below is real code
        let decl = "#[cfg(test)]\nmod tests;\n\npub fn real() {}\n";
        assert!(!is_test_code("src/a.rs", "real", 4, Some(decl)));
        // `#[cfg(test)] use x;` gates one import only
        let import = "#[cfg(test)]\nuse std::fmt;\n\npub fn real() {}\n";
        assert!(!is_test_code("src/a.rs", "real", 4, Some(import)));
        // an inline module (attributes and blank lines between are fine) holds its own symbols,
        // and code after its closing brace is real again
        let inline = "pub fn before() {}\n\n#[cfg(test)]\n#[allow(dead_code)]\n\nmod tests {\n    fn t() {\n        let _ = \"}\";\n    }\n}\n\npub fn after() {}\n";
        assert!(!is_test_code("src/a.rs", "before", 1, Some(inline)));
        assert!(is_test_code("src/a.rs", "t", 7, Some(inline)));
        assert!(!is_test_code("src/a.rs", "after", 12, Some(inline)));
        // one-line form
        let one = "#[cfg(test)] mod tests {\n    fn t() {}\n}\nfn after() {}\n";
        assert!(is_test_code("src/a.rs", "t", 2, Some(one)));
        assert!(!is_test_code("src/a.rs", "after", 4, Some(one)));
    }

    #[test]
    fn rrf_sim_falls_slowly_with_rank() {
        assert_eq!(rrf_sim(0), RRF_SIM);
        assert!(rrf_sim(1) < rrf_sim(0) && rrf_sim(15) > RRF_SIM * 0.75);
        assert!(
            rrf_sim(0) + rrf_sim(20) > rrf_sim(0) * 1.5,
            "found twice beats found once"
        );
    }

    #[test]
    fn weights_sum_to_one() {
        let sum = W_STRUCT + W_SEM + W_RECENCY + W_KIND;
        assert!((sum - 1.0).abs() < 1e-5, "{sum}");
    }

    #[test]
    fn semantic_similarity_is_clamped() {
        assert_eq!(
            record_score(RecordKind::Note, 1, 0.0, false, 2.0),
            record_score(RecordKind::Note, 1, 0.0, false, 1.0)
        );
        assert_eq!(
            record_score(RecordKind::Note, 1, 0.0, false, -1.0),
            record_score(RecordKind::Note, 1, 0.0, false, 0.0)
        );
        assert_eq!(code_score(1, 2.0), code_score(1, 1.0));
        assert_eq!(code_score(1, -1.0), code_score(1, 0.0));
        assert_eq!(convention_score(2.0), convention_score(1.0));
        assert_eq!(convention_score(-1.0), convention_score(0.0));
    }

    #[test]
    fn attempt_on_seed_beats_unanchored_note() {
        assert!(
            record_score(RecordKind::Attempt, 0, 0.0, true, 0.0)
                > record_score(RecordKind::Note, 1, 0.0, false, 0.0)
        );
    }

    #[test]
    fn attempt_on_seed_beats_perfect_semantic_note() {
        assert!(
            record_score(RecordKind::Attempt, 0, 0.0, true, 0.0)
                > record_score(RecordKind::Note, FAR_HOPS, 0.0, false, 1.0)
        );
    }

    #[test]
    fn closer_and_newer_score_higher() {
        assert!(structural(0) > structural(1) && structural(1) > structural(2));
        assert!(recency(0.0) > recency(30.0) && (recency(30.0) - 0.5).abs() < 1e-6);
        assert_eq!(recency(-5.0), 1.0);
        assert!(
            record_score(RecordKind::Note, 1, 1.0, false, 0.0)
                > record_score(RecordKind::Note, 1, 90.0, false, 0.0)
        );
    }

    #[test]
    fn code_score_falls_with_distance() {
        assert!(code_score(0, 0.0) > code_score(1, 0.0) && code_score(1, 0.0) > code_score(2, 0.0));
    }
}
