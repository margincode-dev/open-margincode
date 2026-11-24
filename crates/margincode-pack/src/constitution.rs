//! The constitution as one pack item: its id carries both versions, so the ledger resends it
//! after an amendment at either level.

use margincode_core::{Article, Constitution};

use crate::budget::{Item, Section};

const PREFIX: &str = "const:v";
const OUTRANKS: &str = "it outranks conventions and records";

/// `const:v<repo>`, or `const:v<repo>.<shared>` while a shared article is in force.
pub(crate) fn item_id(c: &Constitution) -> String {
    if c.articles.iter().any(|a| a.shared) {
        format!("{PREFIX}{}.{}", c.version, c.shared_version)
    } else {
        format!("{PREFIX}{}", c.version)
    }
}

fn versions(id: &str) -> (&str, Option<&str>) {
    let v = id.strip_prefix(PREFIX).unwrap_or(id);
    match v.split_once('.') {
        Some((own, shared)) => (own, Some(shared)),
        None => (v, None),
    }
}

/// The section heading for the item `id`.
pub(crate) fn heading(id: &str) -> String {
    match versions(id) {
        (own, Some(shared)) => format!("Constitution (v{own}, organisation v{shared}; {OUTRANKS})"),
        (own, None) => format!("Constitution (v{own}; {OUTRANKS})"),
    }
}

/// The footer line of a pack whose budget did not hold even the titles.
pub(crate) fn not_shown(id: &str, tokens: u32) -> String {
    let v = id.strip_prefix(PREFIX).unwrap_or(id);
    format!("constitution v{v} not shown ({tokens} tokens): av expand const")
}

fn title_line(a: &Article) -> String {
    let check = if a.check.is_some() { " [check]" } else { "" };
    format!("- {} {}{check}", a.handle, a.title)
}

/// Required, score 0.9: below the task line and above every convention. `None` with no article
/// in force.
pub(crate) fn item(c: &Constitution) -> Option<Item> {
    if c.articles.is_empty() {
        return None;
    }
    let titles: Vec<String> = c.articles.iter().map(title_line).collect();
    let texts: Vec<String> = c
        .articles
        .iter()
        .map(|a| format!("{}\n  {}", title_line(a), a.text))
        .collect();
    let full = texts.join("\n");
    Some(
        Item {
            id: item_id(c),
            section: Section::Constitution,
            score: 0.9,
            tiers: [titles.join("\n"), full.clone(), full],
            reason: "",
            source: None,
            required: true,
            group: None,
            order: None,
        }
        .because("constitution"),
    )
}

/// One article in full, for `expand`: its text, its check and its last amendment.
pub(crate) fn article_full(a: &Article) -> String {
    let check = match &a.check {
        Some(c) => format!("\n  check: `{c}`"),
        None => String::new(),
    };
    let l = &a.last;
    format!(
        "- {} {}\n  {}{check}\n  last amended: v{} {} by {} on {}: {}",
        a.handle,
        a.title,
        a.text,
        l.version,
        l.action.as_str(),
        l.by,
        l.at.format("%Y-%m-%d"),
        l.reason
    )
}

/// The whole constitution in full, for `expand const`.
pub(crate) fn full(c: &Constitution) -> String {
    let mut out = format!("## {}", heading(&item_id(c)));
    if c.articles.is_empty() {
        out.push_str("\n(no articles in force)");
    }
    for a in &c.articles {
        out.push('\n');
        out.push_str(&article_full(a));
    }
    out
}
