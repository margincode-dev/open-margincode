//! Lexical signals from the goal text (Plan 6): search words for full-text code search, and the
//! identifiers a goal names outright (`human_readable_name`, `Packer::expand`, `readUrl()`),
//! which seed the pack like a focus does. Both work without the semantic layer.

use std::collections::BTreeSet;

const STOP: &[&str] = &[
    "the", "and", "for", "with", "that", "this", "from", "into", "when", "should", "must", "not",
    "are", "was", "but", "its", "can", "has", "have", "will", "all", "any", "each", "use", "uses",
    "used", "add", "make", "fix", "new", "now", "also", "only", "than", "then", "there", "their",
    "them", "they", "what", "which", "who", "how", "out", "one", "two", "via", "per", "def",
    "self", "return", "none", "true", "false", "let", "mut", "pub", "const", "function", "import",
    "class", "str", "int", "bool", "string", "default", "instead", "still", "without", "keep",
    "while", "does", "doesn", "don", "get", "set", "test", "tests",
];

/// At most this many search words, rarest first (the store weighs them by IDF).
pub const MAX_TERMS: usize = 16;
/// At most this many identifiers taken from one goal.
pub const MAX_NAMED: usize = 8;

/// Identifier-aware words: each alphanumeric run lowercased, plus its camelCase parts.
pub fn words(text: &str) -> Vec<String> {
    let mut out = vec![];
    for raw in text.split(|c: char| !c.is_alphanumeric()) {
        if raw.len() < 2 {
            continue;
        }
        let whole = raw.to_lowercase();
        let chars: Vec<char> = raw.chars().collect();
        let mut parts = vec![];
        let mut cur = String::new();
        for (i, &c) in chars.iter().enumerate() {
            let boundary = i > 0
                && c.is_uppercase()
                && (chars[i - 1].is_lowercase()
                    || chars.get(i + 1).is_some_and(|n| n.is_lowercase()));
            if boundary && !cur.is_empty() {
                parts.push(std::mem::take(&mut cur).to_lowercase());
            }
            cur.push(c);
        }
        if !cur.is_empty() {
            parts.push(cur.to_lowercase());
        }
        out.push(whole.clone());
        if parts.len() > 1 {
            out.extend(parts.into_iter().filter(|p| p.len() >= 2 && *p != whole));
        }
    }
    out
}

/// The goal's search words: three letters or more, no stop words or bare numbers, each once.
pub fn goal_terms(goal: &str) -> Vec<String> {
    let mut seen = BTreeSet::new();
    words(goal)
        .into_iter()
        .filter(|w| w.len() >= 3 && !STOP.contains(&w.as_str()) && !w.chars().all(char::is_numeric))
        .filter(|w| seen.insert(w.clone()))
        .collect()
}

fn ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Identifiers the goal names: anything in backticks, and words that only code would contain:
/// snake_case, camelCase, dotted or `::` paths, or a name followed by `(`. Paths use `.` (the
/// index's qname separator); file names (`core.py`) are left to full-text search.
pub fn named_identifiers(goal: &str) -> Vec<String> {
    let mut found: Vec<(String, bool)> = vec![];
    for (i, span) in goal.split('`').enumerate() {
        let quoted = i % 2 == 1;
        let chars: Vec<char> = span.chars().collect();
        let mut j = 0;
        while j < chars.len() {
            if !(chars[j].is_alphabetic() || chars[j] == '_') {
                j += 1;
                continue;
            }
            let start = j;
            loop {
                while j < chars.len() && ident_char(chars[j]) {
                    j += 1;
                }
                let sep = if chars.get(j) == Some(&'.') {
                    1
                } else if chars.get(j) == Some(&':') && chars.get(j + 1) == Some(&':') {
                    2
                } else {
                    0
                };
                if sep > 0
                    && chars
                        .get(j + sep)
                        .is_some_and(|c| c.is_alphabetic() || *c == '_')
                {
                    j += sep;
                    continue;
                }
                break;
            }
            let token: String = chars[start..j]
                .iter()
                .collect::<String>()
                .replace("::", ".");
            let call = chars.get(j) == Some(&'(');
            found.push((token, quoted || call));
        }
    }
    let mut seen = BTreeSet::new();
    found
        .into_iter()
        .filter(|(t, strong)| {
            let last = t.rsplit('.').next().unwrap_or(t);
            let file = matches!(
                last,
                "py" | "rs" | "ts" | "tsx" | "js" | "md" | "toml" | "json" | "yaml" | "yml"
            );
            let camel = t
                .chars()
                .zip(t.chars().skip(1))
                .any(|(a, b)| a.is_lowercase() && b.is_uppercase());
            t.len() >= 4 && !file && (*strong || t.contains('_') || t.contains('.') || camel)
        })
        .map(|(t, _)| t)
        .filter(|t| seen.insert(t.to_lowercase()))
        .take(MAX_NAMED)
        .collect()
}

/// Whether a symbol is what a named identifier refers to: the whole qname, or its last segment
/// (or segments, for a dotted name), compared without case.
pub fn names_symbol(ident: &str, qname: &str) -> bool {
    let (i, q) = (ident.to_lowercase(), qname.to_lowercase());
    q == i || q.ends_with(&format!(".{i}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words_split_identifiers() {
        let w = words("parseHTTPRequest snake_case");
        for want in [
            "parsehttprequest",
            "parse",
            "http",
            "request",
            "snake",
            "case",
        ] {
            assert!(w.contains(&want.to_owned()), "{want} missing from {w:?}");
        }
    }

    #[test]
    fn goal_terms_drop_stop_words_and_repeats() {
        assert_eq!(
            goal_terms("Make the greeter add an exclamation mark, the greeter! 42"),
            vec!["greeter", "exclamation", "mark"]
        );
    }

    #[test]
    fn named_identifiers_are_what_only_code_would_say() {
        assert_eq!(
            named_identifiers(
                "Add `Parameter.spec` and deprecate human_readable_name; Packer::expand and \
                 readUrl() break, see core.py. Plain words like Parameter or e.g. stay out."
            ),
            vec![
                "Parameter.spec",
                "human_readable_name",
                "Packer.expand",
                "readUrl"
            ]
        );
        assert_eq!(
            named_identifiers("`spec`, `run`"),
            vec!["spec"],
            "short names are noise"
        );
        assert!(named_identifiers("Fix the bug when the path is empty").is_empty());
    }

    #[test]
    fn names_symbol_matches_whole_or_trailing_segments() {
        assert!(names_symbol(
            "human_readable_name",
            "Parameter.human_readable_name"
        ));
        assert!(names_symbol("Packer.expand", "Packer.expand"));
        assert!(names_symbol("parameter.SPEC", "Parameter.spec"));
        assert!(!names_symbol("name", "Parameter.human_readable_name"));
        assert!(!names_symbol("expand", "expander"));
    }
}
