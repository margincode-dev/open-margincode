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
/// Similarity the best lexical hit counts as (the rest in proportion to their scores).
pub const LEXICAL_SIM: f32 = 0.75;

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
