//! Session ledger: remember what each agent session has already been sent.

use std::collections::HashMap;

use scholion_core::Tier;

use crate::budget::Item;

pub(crate) fn sent_line(id: &str) -> String {
    format!("- ↩ {id} (sent earlier)")
}

/// For items already sent at tier `p`, every tier up to `p` becomes a one-line back-reference, so the
/// allocator accounts for the real (small) cost and never "upgrades" into text the agent already has.
/// A higher tier whose text is the same as tier `p`'s (code with no body to add) collapses too.
pub(crate) fn collapse_sent(items: &mut [Item], sent: &HashMap<String, Tier>) {
    for item in items {
        if let Some(prior) = sent.get(&item.id) {
            let line = sent_line(&item.id);
            let prior_text = item.tiers[*prior as usize].clone();
            for t in 0..item.tiers.len() {
                if t <= *prior as usize || item.tiers[t] == prior_text {
                    item.tiers[t] = line.clone();
                }
            }
        }
    }
}

/// What each chosen item was sent as: the lowest tier with the text it got. A tier capped to a
/// lower one's text (a signatures pack) does not count as sent, so a later pack still sends it.
pub(crate) fn delivered(chosen: &[(Item, Tier)]) -> Vec<(String, Tier)> {
    chosen
        .iter()
        .map(|(i, t)| {
            let mut t = *t as usize;
            while t > 0 && i.tiers[t] == i.tiers[t - 1] {
                t -= 1;
            }
            (i.id.clone(), TIERS[t])
        })
        .collect()
}

const TIERS: [Tier; 3] = [Tier::Handle, Tier::Summary, Tier::Full];

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
            order: None,
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

    #[test]
    fn a_tier_capped_to_the_one_below_is_delivered_as_that_one() {
        let mut capped = item("sym:1");
        capped.tiers[2] = capped.tiers[1].clone();
        let out = delivered(&[(capped, Tier::Full), (item("sym:2"), Tier::Full)]);
        assert_eq!(
            out,
            vec![
                ("sym:1".into(), Tier::Summary),
                ("sym:2".into(), Tier::Full)
            ]
        );
    }

    #[test]
    fn a_higher_tier_with_the_sent_text_collapses_too() {
        let mut same = item("sym:1");
        same.tiers[2] = same.tiers[1].clone();
        let mut items = vec![same];
        collapse_sent(
            &mut items,
            &HashMap::from([("sym:1".to_string(), Tier::Summary)]),
        );
        assert_eq!(items[0].tiers[2], sent_line("sym:1"));
    }
}
