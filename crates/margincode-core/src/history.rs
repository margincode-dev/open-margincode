//! Reading a repository's git history as knowledge: which commits say something worth keeping, and
//! the message as text an agent can use. Shared by the import's cold start (the daemon mines
//! commits into records, `av repo add --mine-history N`) and `margincode-bench mine` (commits as
//! retrieval tasks). Pure: callers walk the history their own way (jj-lib in the daemon, `git log`
//! in the bench) and ask these rules about each commit.

use std::sync::LazyLock;

use regex::Regex;

/// The author of every record mined from git history. Packs rank its records below what agents
/// wrote (`margincode-pack`'s `MINED_FACTOR`) and show their origin; no agent should use the name.
pub const MINED_AUTHOR: &str = "git";

/// Whether a record was mined from git history rather than written by an agent.
pub fn is_mined(author_agent: &str) -> bool {
    author_agent == MINED_AUTHOR
}

/// A cleaned message shorter than this says too little to keep ("fix", "update tests").
pub const MIN_MESSAGE_CHARS: usize = 20;

static TRAILER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^[A-Z][A-Za-z-]+: .*(@|https?://)|^(Co-Authored-By|Co-authored-by|Signed-off-by|Claude-Session|Change-Id|Reviewed-by|Acked-by|Reviewed-on|Tested-by):",
    )
    .unwrap()
});
static URL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"https?://\S+").unwrap());
static HOUSEKEEPING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)^(merge|revert|bump|release|chore|docs?|style|ci|build|deps|format|fmt|lint|typo|wip)\b|^(update|upgrade) (dependency|dependencies|deps)|\b(typo|clippy|rustfmt|prettier|black|ruff|formatter|eslint)\b",
    )
    .unwrap()
});

/// A commit message as text: trailers, sign-offs and URLs dropped, blank runs collapsed, at most
/// `max_chars` bytes (cut at a character boundary).
pub fn clean_message(subject: &str, body: &str, max_chars: usize) -> String {
    let mut lines = vec![subject.trim().to_owned(), String::new()];
    for l in body.lines() {
        if TRAILER.is_match(l.trim()) {
            continue;
        }
        let l = URL.replace_all(l, "").trim_end().to_owned();
        if l.trim().is_empty() && lines.last().is_some_and(|p: &String| p.is_empty()) {
            continue;
        }
        lines.push(l);
    }
    let mut text = lines.join("\n").trim().to_owned();
    if text.len() > max_chars {
        let cut = text
            .char_indices()
            .map(|(i, c)| i + c.len_utf8())
            .take_while(|end| *end <= max_chars)
            .last()
            .unwrap_or(0);
        text.truncate(cut);
    }
    text
}

/// Whether a subject looks like housekeeping (merges, reverts, version bumps, formatting, typos,
/// `wip`) rather than a change with a reason.
pub fn housekeeping(subject: &str) -> bool {
    HOUSEKEEPING.is_match(subject.trim())
}

/// A full commit description split into its subject (first line) and body.
pub fn split_message(description: &str) -> (&str, &str) {
    let d = description.trim_start();
    match d.split_once('\n') {
        Some((subject, body)) => (subject.trim_end(), body),
        None => (d.trim_end(), ""),
    }
}

/// The cleaned message of a commit worth mining, or `None` for a merge (more than one parent), a
/// root commit, housekeeping, or a message too short to say anything.
pub fn worth_mining(parents: usize, subject: &str, body: &str, max_chars: usize) -> Option<String> {
    if parents != 1 || housekeeping(subject) {
        return None;
    }
    let text = clean_message(subject, body, max_chars);
    (text.chars().count() >= MIN_MESSAGE_CHARS).then_some(text)
}

/// `text` cut to at most `max` characters, with `…` when something was cut.
pub fn clip_chars(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        return text.to_owned();
    }
    let mut out: String = text.chars().take(max.saturating_sub(1)).collect();
    out.push('…');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_message_drops_trailers_and_links() {
        let g = clean_message(
            "Fix rounding in allocate",
            "\nCents were lost when splitting.\nSee https://example.com/issue/4\n\n\nCo-Authored-By: X <x@y>\nSigned-off-by: A <a@b>\n",
            1500,
        );
        assert_eq!(
            g,
            "Fix rounding in allocate\n\nCents were lost when splitting.\nSee"
        );
    }

    #[test]
    fn clean_message_cuts_at_a_character_boundary() {
        let g = clean_message(&"é".repeat(30), "", 11);
        assert!(g.len() <= 11 && g.chars().all(|c| c == 'é'), "{g}");
    }

    #[test]
    fn housekeeping_subjects_are_recognized() {
        for s in [
            "Merge pull request #12 from x/y",
            "Bump serde from 1.0 to 1.1",
            "fix typo in README",
            "chore: tidy",
            "Run rustfmt",
            "wip",
            "docs: explain config",
        ] {
            assert!(housekeeping(s), "{s}");
        }
        for s in [
            "Cache parsed configs per process",
            "Fix rounding in allocate",
        ] {
            assert!(!housekeeping(s), "{s}");
        }
    }

    #[test]
    fn worth_mining_skips_merges_roots_housekeeping_and_short_messages() {
        let body = "\nWe parse TOML because the users already write it.";
        assert!(worth_mining(1, "Parse config files as TOML", body, 1500).is_some());
        assert!(worth_mining(2, "Parse config files as TOML", body, 1500).is_none());
        assert!(worth_mining(0, "Parse config files as TOML", body, 1500).is_none());
        assert!(worth_mining(1, "fmt", "", 1500).is_none());
        assert!(worth_mining(1, "Fix it", "", 1500).is_none(), "too short");
    }

    #[test]
    fn split_message_and_clip() {
        assert_eq!(split_message("subject\n\nbody\n"), ("subject", "\nbody\n"));
        assert_eq!(split_message("only\n"), ("only", ""));
        assert_eq!(clip_chars("abcdef", 4), "abc…");
        assert_eq!(clip_chars("abc", 4), "abc");
        assert!(is_mined(MINED_AUTHOR) && !is_mined("claude"));
    }
}
