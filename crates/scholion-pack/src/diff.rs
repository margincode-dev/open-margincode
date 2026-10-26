//! Text diffs between two versions of a file: unified diffs for readers, and a line-number-free
//! patch signature that stays equal when the same change is rebased onto moved lines.

use similar::{ChangeTag, TextDiff};

/// Context lines around each hunk of a unified diff.
const CONTEXT: usize = 3;

/// A unified diff of `old` → `new` with `a/<path>` and `b/<path>` headers; empty when equal.
pub fn unified_diff(path: &str, old: &str, new: &str) -> String {
    if old == new {
        return String::new();
    }
    TextDiff::from_lines(old, new)
        .unified_diff()
        .context_radius(CONTEXT)
        .header(&format!("a/{path}"), &format!("b/{path}"))
        .to_string()
}

/// The change from `base` to `head` as (removed, added) text per hunk, in order, without line
/// numbers or context. Two patches with equal signatures make the same edits, wherever the
/// edited lines sit in the file.
pub fn patch_signature(base: &str, head: &str) -> Vec<(String, String)> {
    let diff = TextDiff::from_lines(base, head);
    diff.grouped_ops(0)
        .iter()
        .map(|group| {
            let (mut removed, mut added) = (String::new(), String::new());
            for op in group {
                for change in diff.iter_changes(op) {
                    match change.tag() {
                        ChangeTag::Delete => removed.push_str(change.value()),
                        ChangeTag::Insert => added.push_str(change.value()),
                        ChangeTag::Equal => {}
                    }
                }
            }
            (removed, added)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unified_diff_has_headers_and_changed_lines() {
        let d = unified_diff("a.py", "x = 1\ny = 2\n", "x = 1\ny = 3\n");
        assert!(d.starts_with("--- a/a.py\n+++ b/a.py\n"), "{d}");
        assert!(d.contains("-y = 2\n+y = 3\n"), "{d}");
        assert_eq!(unified_diff("a.py", "same\n", "same\n"), "");
    }

    #[test]
    fn signature_ignores_where_the_edit_sits() {
        let edit = patch_signature("a\nb\nc\n", "a\nB\nc\n");
        let moved = patch_signature("new top\na\nb\nc\n", "new top\na\nB\nc\n");
        assert_eq!(edit, moved);
        assert_eq!(edit, vec![("b\n".into(), "B\n".into())]);
    }

    #[test]
    fn signature_tells_different_edits_and_hunk_counts_apart() {
        let one = patch_signature("a\nb\nc\nd\ne\nf\ng\n", "a\nB\nc\nd\ne\nf\ng\n");
        let two = patch_signature("a\nb\nc\nd\ne\nf\ng\n", "a\nB\nc\nd\ne\nF\ng\n");
        assert_ne!(one, two);
        assert_eq!(two.len(), 2);
        assert_ne!(one, patch_signature("a\nb\nc\n", "a\nX\nc\n"));
    }
}
