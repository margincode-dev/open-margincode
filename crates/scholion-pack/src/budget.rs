//! Token-budget allocation: split a budget across sections, give every item a handle, then upgrade the
//! best-scored items to richer tiers.

use scholion_core::{AgentFamily, Tier};

use crate::tokens::estimate_tokens;

/// Share of the requested budget the pack may actually use (10% safety margin).
const SAFETY_PERCENT: u64 = 90;
/// At most this many omitted handles are listed.
const MAX_OMITTED_LINES: usize = 20;
/// Rough cost of one omitted-handle line, used to size the reserve for the omitted list.
const OMITTED_LINE_TOKENS: u32 = 12;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Section {
    Task,
    /// A merge into main that did not apply: conflicting paths, symbols and the records of the
    /// task that changed them on main.
    Conflicts,
    Map,
    History,
    Code,
    ConventionsLive,
}

impl Section {
    /// Fill order; unused budget flows from each section to the next.
    pub const ALL: [Section; 6] = [
        Section::Task,
        Section::Conflicts,
        Section::Map,
        Section::History,
        Section::Code,
        Section::ConventionsLive,
    ];

    /// The group key a section's items fill under when they have no `Item::group`.
    pub fn key(self) -> &'static str {
        match self {
            Section::Task => "task",
            Section::Conflicts => "conflicts",
            Section::Map => "map",
            Section::History => "history",
            Section::Code => "code",
            Section::ConventionsLive => "conventions_live",
        }
    }

    /// task/intent 10 · conflicts 5 · map 10 · history 20 · code 45 · conventions+live work 10.
    /// An unused share (conflicts, almost always) flows on to the next section.
    pub fn share_percent(self) -> u64 {
        match self {
            Section::Task | Section::Map | Section::ConventionsLive => 10,
            Section::Conflicts => 5,
            Section::History => 20,
            Section::Code => 45,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub id: String,
    pub section: Section,
    pub score: f32,
    /// Rendered text for `Tier::Handle`, `Tier::Summary`, `Tier::Full` (indexed by tier).
    pub tiers: [String; 3],
    /// Why this item is a candidate (`seed`, `hop1`, `anchored`, `semantic`, …), for receipts.
    pub reason: &'static str,
    /// Source path and 1-based inclusive line range, for code items.
    pub source: Option<(String, u32, u32)>,
    /// Hard constraint: its handle is reserved before section shares are applied, so ranking
    /// cannot drop it for being long (Plan 2b). It still upgrades like any other item.
    pub required: bool,
    /// The share this item fills under in `allocate_with_shares`; `None` = its section's key.
    pub group: Option<String>,
}

impl Item {
    pub fn because(mut self, reason: &'static str) -> Self {
        self.reason = reason;
        self
    }

    pub fn required_if(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn at(mut self, path: &str, start: u32, end: u32) -> Self {
        self.source = Some((path.to_owned(), start, end));
        self
    }

    pub fn text(&self, tier: Tier) -> &str {
        &self.tiers[tier as usize]
    }

    pub fn in_group(mut self, group: &str) -> Self {
        self.group = Some(group.to_owned());
        self
    }

    fn group_key(&self) -> &str {
        self.group.as_deref().unwrap_or(self.section.key())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Allocation {
    /// Grouped by section (fill order; share order for `allocate_with_shares`), best score first
    /// within one.
    pub chosen: Vec<(Item, Tier)>,
    /// Handle lines of items that did not fit (at most 20), best score first.
    pub omitted: Vec<String>,
    /// Number of items that did not fit, listed or not.
    pub omitted_total: usize,
    /// Estimated tokens of the chosen tier texts plus the listed omitted lines.
    pub used: u32,
}

struct Priced {
    item: Item,
    costs: [u32; 3],
}

struct Fill {
    chosen: Vec<(usize, Tier)>,
    omitted: Vec<usize>,
    used: u32,
}

fn fill(priced: &[Priced], budget: u32, shares: &[(String, u8)]) -> Fill {
    let mut out = Fill {
        chosen: vec![],
        omitted: vec![],
        used: 0,
    };
    // Hard inclusion: reserve handles for required items first, best score first.
    let mut req: Vec<usize> = (0..priced.len())
        .filter(|&i| priced[i].item.required)
        .collect();
    req.sort_by(|&a, &b| {
        priced[b]
            .item
            .score
            .total_cmp(&priced[a].item.score)
            .then_with(|| priced[a].item.id.cmp(&priced[b].item.id))
    });
    let mut reserved: std::collections::HashSet<usize> = std::collections::HashSet::new();
    let mut reserved_cost: u32 = 0;
    for i in req {
        let c = priced[i].costs[Tier::Handle as usize];
        if reserved_cost + c <= budget {
            reserved_cost += c;
            reserved.insert(i);
        }
    }
    let budget = budget - reserved_cost;
    out.used += reserved_cost;

    // an item whose group has no share can never be chosen
    out.omitted.extend((0..priced.len()).filter(|&i| {
        let key = priced[i].item.group_key();
        !shares.iter().any(|(k, _)| k == key)
    }));

    let mut carry: u32 = 0;
    for (key, share) in shares {
        let mut idxs: Vec<usize> = (0..priced.len())
            .filter(|&i| priced[i].item.group_key() == key)
            .collect();
        idxs.sort_by(|&a, &b| {
            priced[b]
                .item
                .score
                .total_cmp(&priced[a].item.score)
                .then_with(|| priced[a].item.id.cmp(&priced[b].item.id))
        });
        let mut remaining = (budget as u64 * u64::from(*share) / 100) as u32 + carry;

        // Phase A: every item that fits gets a handle.
        let mut kept: Vec<(usize, Tier)> = vec![];
        for &i in &idxs {
            let c = priced[i].costs[Tier::Handle as usize];
            if reserved.contains(&i) {
                kept.push((i, Tier::Handle)); // already paid for
            } else if priced[i].item.required {
                out.omitted.push(i); // required, but not even its handle fit the whole budget
            } else if c <= remaining {
                remaining -= c;
                kept.push((i, Tier::Handle));
            } else {
                out.omitted.push(i);
            }
        }
        // Phase B: upgrade in score order to the richest tier that still fits.
        for (i, tier) in &mut kept {
            for candidate in [Tier::Full, Tier::Summary] {
                let (cur, cand) = (
                    priced[*i].costs[*tier as usize],
                    priced[*i].costs[candidate as usize],
                );
                if cand > cur && cand - cur <= remaining {
                    remaining -= cand - cur;
                    *tier = candidate;
                    break;
                }
            }
        }
        out.used += kept
            .iter()
            .map(|(i, t)| {
                // reserved handles were counted above; count only their upgrade here
                let base = if reserved.contains(i) {
                    priced[*i].costs[Tier::Handle as usize]
                } else {
                    0
                };
                priced[*i].costs[*t as usize] - base
            })
            .sum::<u32>();
        out.chosen.extend(kept);
        carry = remaining;
    }
    // What the last share leaves would be wasted: spend it on chosen items in any section, best
    // first, signatures before bodies (a named symbol is worth more than a longer one).
    let mut order: Vec<usize> = (0..out.chosen.len()).collect();
    order.sort_by(|&a, &b| {
        let (pa, pb) = (&priced[out.chosen[a].0].item, &priced[out.chosen[b].0].item);
        pb.score
            .total_cmp(&pa.score)
            .then_with(|| pa.id.cmp(&pb.id))
    });
    for (from, to) in [(Tier::Handle, Tier::Summary), (Tier::Summary, Tier::Full)] {
        for &k in &order {
            let (i, tier) = out.chosen[k];
            if tier != from {
                continue;
            }
            let (cur, cand) = (priced[i].costs[from as usize], priced[i].costs[to as usize]);
            if cand > cur && cand - cur <= carry {
                carry -= cand - cur;
                out.used += cand - cur;
                out.chosen[k].1 = to;
            }
        }
    }
    out
}

/// Allocates `budget` tokens (of which 90% is usable) across `items`. `used <= budget` always holds.
pub fn allocate(items: Vec<Item>, budget: u32, family: AgentFamily) -> Allocation {
    let shares: Vec<(String, u8)> = Section::ALL
        .iter()
        .map(|s| (s.key().to_owned(), s.share_percent() as u8))
        .collect();
    allocate_with_shares(items, budget, family, &shares)
}

/// `allocate` with caller-defined groups: `shares` are (group key, percent) in fill order, and
/// unused budget flows from each group to the next. An item fills under `Item::group`, or under its
/// section's key when that is `None`; items of a group without a share are omitted.
pub fn allocate_with_shares(
    items: Vec<Item>,
    budget: u32,
    family: AgentFamily,
    shares: &[(String, u8)],
) -> Allocation {
    let effective = (budget as u64 * SAFETY_PERCENT / 100) as u32;
    let priced: Vec<Priced> = items
        .into_iter()
        .map(|item| {
            let costs = [0, 1, 2].map(|t| estimate_tokens(&item.tiers[t], family));
            Priced { item, costs }
        })
        .collect();

    let mut f = fill(&priced, effective, shares);
    let mut listed: Vec<String> = vec![];
    let mut listed_cost = 0;
    if !f.omitted.is_empty() {
        // Reserve room for the omitted list, then refill with what is left.
        let reserve = (effective / 4).min(MAX_OMITTED_LINES as u32 * OMITTED_LINE_TOKENS);
        f = fill(&priced, effective - reserve, shares);
        let room = effective - f.used;
        for &i in f.omitted.iter().take(MAX_OMITTED_LINES) {
            let line = &priced[i].item.tiers[Tier::Handle as usize];
            let c = estimate_tokens(line, family);
            if listed_cost + c > room {
                break;
            }
            listed_cost += c;
            listed.push(line.clone());
        }
    }

    Allocation {
        chosen: f
            .chosen
            .iter()
            .map(|&(i, t)| (priced[i].item.clone(), t))
            .collect(),
        omitted_total: f.omitted.len(),
        omitted: listed,
        used: f.used + listed_cost,
    }
}
