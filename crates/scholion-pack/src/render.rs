//! Markdown rendering of an allocation.

use scholion_core::{AgentFamily, Tier};

use crate::budget::{Item, Section};
use crate::tokens::estimate_tokens;

const FOOTER_RULE: &str = "---";

pub(crate) fn push_section(out: &mut String, heading: &str, items: &[&(Item, Tier)], always: bool) {
    if items.is_empty() && !always {
        return;
    }
    out.push_str(&format!("## {heading}\n"));
    if items.is_empty() {
        out.push_str("(none)\n");
    }
    for (item, tier) in items {
        out.push_str(item.text(*tier));
        out.push('\n');
    }
    out.push('\n');
}

pub(crate) fn render(
    chosen: &[(Item, Tier)],
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
    push_section(
        &mut out,
        "Decisions & dead ends",
        &pick(&|i| i.section == Section::History),
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

/// Worst-case fixed text of a pack (every heading, footer with large numbers); its token cost is
/// subtracted from the budget before items are allocated.
pub(crate) fn skeleton_text() -> String {
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
                },
                Tier::Handle,
            )
        })
        .collect();
    let mut text = render(&chosen, &[], 99_999, 99_999, 99_999, true);
    text.push_str("## Live work\n");
    text
}
