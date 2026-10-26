//! Session ledger: remember what each agent session has already been sent.

use std::collections::HashMap;

use scholion_core::Tier;

use crate::budget::Item;

pub(crate) fn sent_line(id: &str) -> String {
    format!("- ↩ {id} (sent earlier)")
}

/// For items already sent at tier `p`, every tier up to `p` becomes a one-line back-reference, so the
/// allocator accounts for the real (small) cost and never "upgrades" into text the agent already has.
pub(crate) fn collapse_sent(items: &mut [Item], sent: &HashMap<String, Tier>) {
    for item in items {
        if let Some(prior) = sent.get(&item.id) {
            let line = sent_line(&item.id);
            for t in 0..=(*prior as usize) {
                item.tiers[t] = line.clone();
            }
        }
    }
}

pub(crate) fn delivered(chosen: &[(Item, Tier)]) -> Vec<(String, Tier)> {
    chosen.iter().map(|(i, t)| (i.id.clone(), *t)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::budget::Section;

    fn item(id: &str) -> Item {
        Item {
            id: id.into(),
            section: Section::Code,
            score: 1.0,
            tiers: ["h".into(), "summary text".into(), "full text".into()],
            reason: "",
            source: None,
            required: false,
            group: None,
        }
    }

    #[test]
    fn collapses_tiers_up_to_the_prior_one_only() {
        let mut items = vec![item("sym:1"), item("sym:2")];
        let sent = HashMap::from([("sym:1".to_string(), Tier::Summary)]);
        collapse_sent(&mut items, &sent);
        assert_eq!(items[0].tiers[0], sent_line("sym:1"));
        assert_eq!(items[0].tiers[1], sent_line("sym:1"));
        assert_eq!(items[0].tiers[2], "full text");
        assert_eq!(items[1].tiers[1], "summary text");
    }
}
