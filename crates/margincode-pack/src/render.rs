//! Markdown rendering of an allocation.

use std::collections::HashSet;

use margincode_core::{AgentFamily, Tier};

use crate::budget::{Item, Section};
use crate::ledger::sent_line;
use crate::tokens::estimate_tokens;

const FOOTER_RULE: &str = "---";

/// The first line of the records section when it shows a record of another task. A stored
/// record is what an earlier agent believed, and an agent handed one as a record followed it
/// even when it was wrong, where the same text in a notes file was simply edited
/// (`bench/results/2026-11-wrong-lesson`). So the pack says what a record is and how a wrong one
/// is corrected. Not printed for a task's own records alone, nor when every other task's record
/// in the section is only a back-reference to an earlier pack of the session.
pub(crate) const CORRECT_HINT: &str = "Claims by earlier agents: check against the code; correct \
     wrong ones with `av record --supersedes rec:…`";

pub(crate) fn push_section(out: &mut String, heading: &str, items: &[&(Item, Tier)], always: bool) {
    push_section_noted(out, heading, None, items, always);
}

/// `push_section` with a line of `note` between the heading and the items.
fn push_section_noted(
    out: &mut String,
    heading: &str,
    note: Option<&str>,
    items: &[&(Item, Tier)],
    always: bool,
) {
    if items.is_empty() && !always {
        return;
    }
    out.push_str(&format!("## {heading}\n"));
    if let Some(note) = note {
        out.push_str(note);
        out.push('\n');
    }
    if items.is_empty() {
        out.push_str("(none)\n");
    }
    for (item, tier) in items {
        out.push_str(item.text(*tier));
        out.push('\n');
    }
    out.push('\n');
}

/// Whether `item` is another task's record the reader may supersede (`foreign`) that a pack
/// shows as more than a back-reference to an earlier pack of the session: at `tier`, or with
/// `None` at any tier it has left. It is what the line under the records heading, `CORRECT_HINT`,
/// is printed for, and so what a pack keeps room for it for (`Packer::build`).
pub(crate) fn claim_shown(item: &Item, foreign: &HashSet<String>, tier: Option<Tier>) -> bool {
    if !foreign.contains(&item.id) {
        return false;
    }
    let back = sent_line(&item.id);
    match tier {
        Some(tier) => item.text(tier) != back,
        None => item.tiers.iter().any(|text| *text != back),
    }
}

/// `foreign`: ids of the History items that are another task's records the reader may supersede
/// (`Gathered::foreign`).
pub(crate) fn render(
    chosen: &[(Item, Tier)],
    foreign: &HashSet<String>,
    omitted: &[String],
    omitted_total: usize,
    tokens: u32,
    budget: u32,
    show_empty_map: bool,
) -> String {
    let pick = |pred: &dyn Fn(&Item) -> bool| -> Vec<&(Item, Tier)> {
        chosen.iter().filter(|(i, _)| pred(i)).collect()
    };
    let mut out = String::new();
    push_section(
        &mut out,
        "Task",
        &pick(&|i| i.section == Section::Task),
        true,
    );
    push_section(
        &mut out,
        "Conflicts",
        &pick(&|i| i.section == Section::Conflicts),
        false,
    );
    push_section(
        &mut out,
        "Map",
        &pick(&|i| i.section == Section::Map),
        show_empty_map,
    );
    let history = pick(&|i| i.section == Section::History);
    let claims = history
        .iter()
        .any(|(i, tier)| claim_shown(i, foreign, Some(*tier)));
    push_section_noted(
        &mut out,
        "Decisions & dead ends",
        claims.then_some(CORRECT_HINT),
        &history,
        false,
    );
    push_section(
        &mut out,
        "Code",
        &pick(&|i| i.section == Section::Code),
        false,
    );
    push_section(
        &mut out,
        "Conventions",
        &pick(&|i| i.section == Section::ConventionsLive && i.id.starts_with("conv:")),
        false,
    );
    push_section(
        &mut out,
        "Live work",
        &pick(&|i| i.section == Section::ConventionsLive && i.id.starts_with("live:")),
        false,
    );
    push_footer(&mut out, omitted, omitted_total, tokens, budget);
    out
}

/// `tokens: N/B` and the list of omitted handles.
pub(crate) fn push_footer(
    out: &mut String,
    omitted: &[String],
    omitted_total: usize,
    tokens: u32,
    budget: u32,
) {
    out.push_str(&format!("{FOOTER_RULE}\ntokens: {tokens}/{budget}\n"));
    if omitted_total > 0 {
        out.push_str(&format!(
            "omitted {omitted_total} lower-ranked items (expand an id for detail):\n"
        ));
        for line in omitted {
            out.push_str(line);
            out.push('\n');
        }
        if omitted_total > omitted.len() {
            out.push_str(&format!("- … and {} more\n", omitted_total - omitted.len()));
        }
    }
}

/// Renders `chosen` with `render(chosen, omitted_total, tokens)` until the stated token count is
/// the measured one, dropping the lowest-ranked items (hard constraints last) while the text is
/// over `budget`. Returns the text and its tokens; `("", 0)` if nothing fits.
pub(crate) fn fit(
    chosen: &mut Vec<(Item, Tier)>,
    omitted_total: &mut usize,
    budget: u32,
    family: AgentFamily,
    render: impl Fn(&[(Item, Tier)], usize, u32) -> String,
) -> (String, u32) {
    loop {
        let (md, tokens) = stable(|n| render(chosen, *omitted_total, n), family);
        if tokens <= budget {
            return (md, tokens);
        }
        // drop the lowest-ranked item that is not a hard constraint
        let victim = chosen
            .iter()
            .rposition(|(i, _)| !i.required)
            .or(chosen.len().checked_sub(1));
        match victim {
            Some(v) => {
                chosen.remove(v);
                *omitted_total += 1;
            }
            None => return (String::new(), 0),
        }
    }
}

/// Renders until the `tokens: N/B` footer states the measured token count of the text it is in.
fn stable(render: impl Fn(u32) -> String, family: AgentFamily) -> (String, u32) {
    let mut n = 0;
    let mut md = String::new();
    let mut est = 0;
    for _ in 0..3 {
        md = render(n);
        est = estimate_tokens(&md, family);
        if est == n {
            break;
        }
        n = est;
    }
    (md, est)
}

/// Worst-case fixed text of a pack (every heading, footer with large numbers, and with `claims`
/// the line under the records heading: `CORRECT_HINT`, for a pack that may show another task's
/// record); its token cost is subtracted from the budget before items are allocated.
pub(crate) fn skeleton_text(claims: bool) -> String {
    let chosen: Vec<(Item, Tier)> = Section::ALL
        .iter()
        .map(|s| {
            (
                Item {
                    id: "conv:0".into(),
                    section: *s,
                    score: 0.0,
                    tiers: [String::new(), String::new(), String::new()],
                    reason: "",
                    source: None,
                    required: false,
                    group: None,
                    order: None,
                },
                Tier::Handle,
            )
        })
        .collect();
    let foreign: HashSet<String> = claims.then(|| "conv:0".to_owned()).into_iter().collect();
    let mut text = render(&chosen, &foreign, &[], 99_999, 99_999, 99_999, true);
    text.push_str("## Live work\n");
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ledger::collapse_sent;
    use std::collections::HashMap;

    fn record(id: &str) -> Item {
        Item {
            id: id.into(),
            section: Section::History,
            score: 1.0,
            tiers: ["handle".into(), "summary".into(), "full".into()],
            reason: "",
            source: None,
            required: false,
            group: None,
            order: None,
        }
    }

    #[test]
    fn a_record_that_can_only_be_referred_back_to_is_not_a_claim_shown() {
        let foreign: HashSet<String> = ["rec:1".to_owned(), "rec:2".to_owned()].into();
        let mut items = vec![record("rec:1"), record("rec:2"), record("rec:3")];
        collapse_sent(
            &mut items,
            &HashMap::from([
                ("rec:1".to_owned(), Tier::Full),
                ("rec:2".to_owned(), Tier::Summary),
            ]),
        );
        // sent in full: every tier refers back, so no pack of this session shows it again
        assert!(!claim_shown(&items[0], &foreign, None));
        assert!(!claim_shown(&items[0], &foreign, Some(Tier::Full)));
        // sent as a summary: only its full text is still to show
        assert!(claim_shown(&items[1], &foreign, None));
        assert!(!claim_shown(&items[1], &foreign, Some(Tier::Summary)));
        assert!(claim_shown(&items[1], &foreign, Some(Tier::Full)));
        // not another task's supersedable record
        assert!(!claim_shown(&items[2], &foreign, None));
        assert!(!claim_shown(&items[2], &foreign, Some(Tier::Full)));
    }
}
