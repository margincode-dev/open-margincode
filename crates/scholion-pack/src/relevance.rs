//! How well records match a goal: by its words (always) and by embedding similarity (when the
//! semantic layer gave hits), fused by rank.
//!
//! An anchor says a record is about the same code as the task. It does not say the record is
//! about the same thing, and where one symbol or file has many records only that can pick among
//! them.

use std::collections::{HashMap, HashSet};

use crate::lexical;
use crate::score;

const BM25_K1: f32 = 1.2;
const BM25_B: f32 = 0.75;

/// `regions` and `region`, `ids` and `id`: the plural of a goal's word is the same word.
fn fold(word: &str) -> &str {
    match word.strip_suffix('s') {
        Some(stem) if stem.len() >= 2 && !stem.ends_with('s') => stem,
        _ => word,
    }
}

/// BM25 of each text against the goal's words, with document frequencies taken from `texts`
/// themselves. A word every one of them has (the name of the symbol they are all anchored on)
/// counts for nothing, since it cannot tell them apart. 0.0 where no other word of the goal
/// occurs.
pub(crate) fn word_scores(goal: &str, texts: &[String]) -> Vec<f32> {
    let terms: HashSet<String> = lexical::goal_terms(goal)
        .iter()
        .map(|t| fold(t).to_owned())
        .collect();
    if terms.is_empty() || texts.is_empty() {
        return vec![0.0; texts.len()];
    }
    let docs: Vec<Vec<String>> = texts
        .iter()
        .map(|t| {
            lexical::words(t)
                .iter()
                .map(|w| fold(w).to_owned())
                .collect()
        })
        .collect();
    let n = docs.len() as f32;
    let mean_len = (docs.iter().map(Vec::len).sum::<usize>() as f32 / n).max(1.0);
    // the usual BM25 weight, less what a word in every text would get
    let idf = |d: f32| (1.0 + (n - d + 0.5) / (d + 0.5)).ln() - (1.0 + 0.5 / (n + 0.5)).ln();
    let mut df: HashMap<&str, f32> = HashMap::new();
    for doc in &docs {
        let seen: HashSet<&str> = doc.iter().map(String::as_str).collect();
        for t in seen {
            if terms.contains(t) {
                *df.entry(t).or_default() += 1.0;
            }
        }
    }
    docs.iter()
        .map(|doc| {
            let mut tf: HashMap<&str, f32> = HashMap::new();
            for w in doc {
                if terms.contains(w.as_str()) {
                    *tf.entry(w).or_default() += 1.0;
                }
            }
            let norm = BM25_K1 * (1.0 - BM25_B + BM25_B * doc.len() as f32 / mean_len);
            // sorted, so the sum does not depend on the map's order
            let mut parts: Vec<(&str, f32)> = tf.into_iter().collect();
            parts.sort_by(|a, b| a.0.cmp(b.0));
            parts
                .into_iter()
                .map(|(t, f)| {
                    let d = df.get(t).copied().unwrap_or(0.0);
                    idf(d).max(0.0) * f * (BM25_K1 + 1.0) / (f + norm)
                })
                .sum()
        })
        .collect()
}

/// Reciprocal rank fusion of two orderings of the same items: `words[i]` (BM25, 0.0 = no match)
/// and `similarity[i]` (embedding similarity, `None` = not among the hits). An item neither
/// ordering ranks gets 0.0. Tied items share the mean of their ranks: thirty records that match
/// one common word equally are not thirty best matches.
pub(crate) fn fuse(words: &[f32], similarity: &[Option<f32>]) -> Vec<f32> {
    let mut out = vec![0.0; words.len()];
    let words: Vec<Option<f32>> = words.iter().map(|w| (*w > 0.0).then_some(*w)).collect();
    for ordering in [&words[..], similarity] {
        let mut ranked: Vec<(usize, f32)> = ordering
            .iter()
            .enumerate()
            .filter_map(|(i, s)| s.map(|s| (i, s)))
            .collect();
        ranked.sort_by(|a, b| b.1.total_cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        let mut start = 0;
        while start < ranked.len() {
            let tied = ranked[start..]
                .iter()
                .take_while(|(_, s)| *s == ranked[start].1)
                .count();
            let rank = start as f32 + (tied - 1) as f32 / 2.0;
            for (i, _) in &ranked[start..start + tied] {
                out[*i] += 1.0 / (score::RRF_K + 1.0 + rank);
            }
            start += tied;
        }
    }
    out
}

/// The at most `n` items that match best by `fuse(words, similarity)`, best first, each with its
/// fused score. An item that matches neither way is never one of them, and neither is a tie
/// that the cut would split: of thirty records equally good, none is the best.
pub(crate) fn leads(words: &[f32], similarity: &[Option<f32>], n: usize) -> Vec<(usize, f32)> {
    let fused = fuse(words, similarity);
    let mut ranked: Vec<(usize, f32)> = fused
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, f)| *f > 0.0)
        .collect();
    ranked.sort_by(|a, b| b.1.total_cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    if ranked.len() > n {
        let cut = ranked[n].1;
        ranked.truncate(n);
        ranked.retain(|(_, f)| *f != cut);
    }
    ranked
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leads_are_the_few_best_matches_and_never_a_split_tie() {
        // words and similarity agree on 2, then 0; 1 matches by words only; 3 not at all
        let l = leads(
            &[2.0, 1.0, 3.0, 0.0],
            &[Some(0.8), None, Some(0.9), None],
            2,
        );
        assert_eq!(l.iter().map(|(i, _)| *i).collect::<Vec<_>>(), vec![2, 0]);
        assert!(l[0].1 > l[1].1);
        // fewer matches than places: only those that match
        let l = leads(&[0.0, 1.0, 0.0], &[None, None, None], 2);
        assert_eq!(l.iter().map(|(i, _)| *i).collect::<Vec<_>>(), vec![1]);
        // three equally good for two places: none leads; one better than the tie still does
        assert!(leads(&[1.0, 1.0, 1.0], &[None, None, None], 2).is_empty());
        let l = leads(&[3.0, 1.0, 1.0], &[None, None, None], 2);
        assert_eq!(l.iter().map(|(i, _)| *i).collect::<Vec<_>>(), vec![0]);
        assert!(leads(&[], &[], 2).is_empty());
    }

    fn texts(t: &[&str]) -> Vec<String> {
        t.iter().map(|s| (*s).to_owned()).collect()
    }

    #[test]
    fn a_word_every_text_has_counts_for_nothing() {
        let s = word_scores(
            "Make hello louder",
            &texts(&[
                "Wrapping hello in a retry loop hid the error.",
                "A louder hello ends in one exclamation mark.",
                "Calling hello twice returned the same object.",
            ]),
        );
        assert!(s[1] > 0.0, "{s:?}");
        assert_eq!(
            (s[0], s[2]),
            (0.0, 0.0),
            "only `hello`, which all three have"
        );
    }

    #[test]
    fn plurals_match_and_no_match_is_zero() {
        let s = word_scores(
            "Validate customer regions",
            &texts(&["An invalid region is rejected.", "Unrelated text."]),
        );
        assert!(s[0] > 0.0, "{s:?}");
        assert_eq!(s[1], 0.0);
        assert_eq!(word_scores("", &texts(&["anything"])), vec![0.0]);
    }

    #[test]
    fn fusion_ranks_what_both_orderings_like_first_and_leaves_the_rest_at_zero() {
        let f = fuse(&[0.0, 3.0, 1.0, 0.0], &[None, Some(0.8), None, Some(0.9)]);
        assert_eq!(f[0], 0.0);
        assert!(f[1] > f[3] && f[3] > f[2] && f[2] > 0.0, "{f:?}");
        // equal scores share the mean of their ranks, so a wide tie is worth less than a win
        let t = fuse(&[2.0, 2.0, 2.0, 0.0], &[None, None, None, Some(0.5)]);
        assert_eq!((t[0], t[1]), (t[1], t[2]));
        assert!(t[3] > t[0], "{t:?}");
    }
}
