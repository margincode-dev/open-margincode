//! Routing: which agent should take which waiting task (Plan 8). Pure and deterministic, no LLM.
//!
//! An agent *can do* an item when every tag the item needs is in its profile's `best` or `can`.
//! Candidates are ranked by the item's `prefer` pin, then by score (2 per need in `best`, 1 per
//! need in `can`), then by cost (cheapest first), then by name. For `reserve_secs` after an item
//! is posted only its top-ranked agent may take it; afterwards any agent that can do it may.

use std::collections::HashMap;

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// What an agent is good at and what it costs; set by the user (`av agent set`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    /// Tags this agent is best at.
    #[serde(default)]
    pub best: Vec<String>,
    /// Tags this agent can do, but is not the best at.
    #[serde(default)]
    pub can: Vec<String>,
    /// 1 is the cheapest.
    pub cost_rank: i32,
    /// Other names the agent introduces itself with (MCP `clientInfo.name`).
    #[serde(default)]
    pub aliases: Vec<String>,
    /// The most open tasks it may hold at once (an agent CLI with subagents is a team that works
    /// in parallel). `None`: no limit.
    #[serde(default)]
    pub max_open: Option<i32>,
}

/// A task waiting in the backlog for an agent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BacklogItem {
    pub id: Uuid,
    pub repo: String,
    pub title: String,
    pub goal: String,
    #[serde(default)]
    pub needs: Vec<String>,
    /// An agent pinned to the item for its reserve window.
    pub prefer: Option<String>,
    pub priority: i32,
    pub reserve_secs: i64,
    pub created_at: DateTime<Utc>,
    /// Backlog items that must be merged before `next` offers this one.
    #[serde(default)]
    pub after: Vec<Uuid>,
}

/// What the user posts to the backlog (`av post`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewBacklogItem {
    pub repo: String,
    pub title: String,
    pub goal: String,
    #[serde(default)]
    pub needs: Vec<String>,
    pub prefer: Option<String>,
    #[serde(default)]
    pub priority: i32,
    pub reserve_secs: i64,
    pub posted_by: String,
    /// Items this one starts after, each a backlog item id or the slug of its title (same repo).
    #[serde(default)]
    pub after: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BacklogState {
    Queued,
    Taken,
    Dropped,
}

impl BacklogState {
    pub fn as_str(self) -> &'static str {
        match self {
            BacklogState::Queued => "queued",
            BacklogState::Taken => "taken",
            BacklogState::Dropped => "dropped",
        }
    }
}

impl std::str::FromStr for BacklogState {
    type Err = crate::AvError;
    fn from_str(s: &str) -> crate::Result<Self> {
        match s {
            "queued" => Ok(BacklogState::Queued),
            "taken" => Ok(BacklogState::Taken),
            "dropped" => Ok(BacklogState::Dropped),
            other => Err(crate::AvError::Invalid(format!(
                "unknown backlog state {other:?} (queued, taken or dropped)"
            ))),
        }
    }
}

/// A backlog row: the item plus what became of it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BacklogEntry {
    #[serde(flatten)]
    pub item: BacklogItem,
    pub state: BacklogState,
    /// The task made for the item once an agent took it.
    pub task_id: Option<crate::TaskId>,
    pub taken_by: Option<String>,
    pub taken_at: Option<DateTime<Utc>>,
    pub posted_by: String,
    /// Titles of the items in `after` that are not merged yet; empty: nothing to wait for.
    #[serde(default)]
    pub waiting_on: Vec<String>,
}

/// A handed-off task that names what its next agent needs (`av handoff --needs`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandedOff {
    pub task: crate::Task,
    pub needs: Vec<String>,
    pub prefer: Option<String>,
    /// When the tags were set: the start of the reserve window.
    pub since: DateTime<Utc>,
}

/// An open task whose owner went quiet: its lease expired, or it never had one and is older than
/// a lease. `since` is when it became free to take: the start of its reserve window.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stale {
    pub task: crate::Task,
    pub since: DateTime<Utc>,
}

/// Lowercases and trims a tag; blank tags are an error.
pub fn clean_tag(tag: &str) -> crate::Result<String> {
    let t = tag.trim().to_lowercase();
    if t.is_empty() {
        return Err(crate::AvError::Invalid("a tag must not be blank".into()));
    }
    Ok(t)
}

/// Default reserve window of a backlog item.
pub const DEFAULT_RESERVE_SECS: i64 = 600;

impl Profile {
    /// Every need is in `best` or `can`.
    pub fn can_do(&self, needs: &[String]) -> bool {
        needs
            .iter()
            .all(|n| self.best.contains(n) || self.can.contains(n))
    }

    /// 2 per need in `best`, 1 per need in `can`.
    pub fn score(&self, needs: &[String]) -> u32 {
        needs
            .iter()
            .map(|n| {
                if self.best.contains(n) {
                    2
                } else {
                    u32::from(self.can.contains(n))
                }
            })
            .sum()
    }
}

/// The profile `name` refers to, directly or through an alias.
pub fn resolve<'a>(profiles: &'a [Profile], name: &str) -> Option<&'a Profile> {
    profiles.iter().find(|p| p.name == name).or_else(|| {
        profiles
            .iter()
            .find(|p| p.aliases.iter().any(|a| a == name))
    })
}

/// Agent names that may take `item`, best first: the pinned agent, then every profile that can do
/// it by score, cost and name.
pub fn rank(item: &BacklogItem, profiles: &[Profile]) -> Vec<String> {
    let mut able: Vec<&Profile> = profiles.iter().filter(|p| p.can_do(&item.needs)).collect();
    able.sort_by(|a, b| {
        b.score(&item.needs)
            .cmp(&a.score(&item.needs))
            .then(a.cost_rank.cmp(&b.cost_rank))
            .then(a.name.cmp(&b.name))
    });
    let mut out: Vec<String> = Vec::new();
    if let Some(p) = &item.prefer {
        out.push(p.clone());
    }
    for p in able {
        if !out.contains(&p.name) {
            out.push(p.name.clone());
        }
    }
    out
}

/// Whether `agent` is within the reserve window's exclusive holder, or the window is over.
fn reserved_for(item: &BacklogItem, ranked: &[String], now: DateTime<Utc>) -> Option<String> {
    let open = item.created_at + Duration::seconds(item.reserve_secs.max(0));
    if now >= open {
        return None;
    }
    ranked.first().cloned()
}

/// May `agent` (a profile name; unknown names have no profile) take `item` at `now`?
pub fn eligible(agent: &str, item: &BacklogItem, profiles: &[Profile], now: DateTime<Utc>) -> bool {
    let ranked = rank(item, profiles);
    match reserved_for(item, &ranked, now) {
        // Inside the reserve window only the top-ranked agent may take it. With no candidate at
        // all (no profiles) nobody holds it, so anyone may.
        Some(holder) => holder == agent,
        None if ranked.is_empty() => item.needs.is_empty(),
        None => {
            item.needs.is_empty() && resolve(profiles, agent).is_none()
                || ranked.iter().any(|r| r == agent)
        }
    }
}

/// The items `agent` may take now, best first: those it ranks first on, then higher priority,
/// then older, then by id. The first one is the item to hand out.
pub fn order_for<'a>(
    agent: &str,
    items: &'a [BacklogItem],
    profiles: &[Profile],
    now: DateTime<Utc>,
) -> Vec<&'a BacklogItem> {
    order_for_excluding(agent, items, profiles, now, &HashMap::new())
}

/// `order_for`, where `excluded` names an agent per item (by item id) that may not take it: the
/// agent that handed a task off. It neither takes the item nor holds its reserve window, so the
/// window goes to the best of the others.
pub fn order_for_excluding<'a>(
    agent: &str,
    items: &'a [BacklogItem],
    profiles: &[Profile],
    now: DateTime<Utc>,
    excluded: &HashMap<Uuid, String>,
) -> Vec<&'a BacklogItem> {
    let mut out: Vec<(bool, &BacklogItem)> = Vec::new();
    for i in items {
        let others: Vec<Profile>;
        let profiles = match excluded.get(&i.id) {
            Some(ex) if ex == agent => continue,
            Some(ex) => {
                others = profiles.iter().filter(|p| &p.name != ex).cloned().collect();
                &others[..]
            }
            None => profiles,
        };
        if eligible(agent, i, profiles, now) {
            out.push((rank(i, profiles).first().is_some_and(|r| r == agent), i));
        }
    }
    out.sort_by(|(af, a), (bf, b)| {
        bf.cmp(af)
            .then(b.priority.cmp(&a.priority))
            .then(a.created_at.cmp(&b.created_at))
            .then(a.id.cmp(&b.id))
    });
    out.into_iter().map(|(_, i)| i).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tags(t: &[&str]) -> Vec<String> {
        t.iter().map(|s| s.to_string()).collect()
    }

    fn profile(name: &str, best: &[&str], can: &[&str], cost: i32) -> Profile {
        Profile {
            name: name.into(),
            best: tags(best),
            can: tags(can),
            cost_rank: cost,
            aliases: vec![],
            max_open: None,
        }
    }

    /// The example set from the design.
    fn profiles() -> Vec<Profile> {
        vec![
            profile("grok", &["image"], &["ui"], 2),
            profile("claude", &["code", "tests"], &["docs", "ui"], 4),
            profile("muse", &[], &["code", "docs"], 1),
            profile("codex", &["math"], &["code"], 3),
        ]
    }

    fn t0() -> DateTime<Utc> {
        "2026-10-31T12:00:00Z".parse().unwrap()
    }

    fn item(n: u128, needs: &[&str]) -> BacklogItem {
        BacklogItem {
            id: Uuid::from_u128(n),
            repo: "r".into(),
            title: format!("t{n}"),
            goal: "g".into(),
            needs: tags(needs),
            prefer: None,
            priority: 0,
            reserve_secs: DEFAULT_RESERVE_SECS,
            created_at: t0(),
            after: vec![],
        }
    }

    #[test]
    fn untagged_work_goes_to_the_cheapest_agent() {
        assert_eq!(rank(&item(1, &[]), &profiles())[0], "muse");
    }

    #[test]
    fn tags_pick_the_best_agent_then_the_cheaper_one() {
        let p = profiles();
        assert_eq!(rank(&item(1, &["image"]), &p), ["grok"]);
        assert_eq!(rank(&item(1, &["math"]), &p), ["codex"]);
        // code: claude is best (2), codex and muse can (1); muse is cheaper than codex.
        assert_eq!(rank(&item(1, &["code"]), &p), ["claude", "muse", "codex"]);
    }

    #[test]
    fn an_agent_must_have_every_need() {
        let p = profiles();
        // Only claude has both tags.
        assert_eq!(rank(&item(1, &["code", "ui"]), &p), ["claude"]);
        assert!(rank(&item(1, &["image", "math"]), &p).is_empty());
    }

    #[test]
    fn prefer_ranks_first_even_without_the_tags() {
        let mut i = item(1, &["image"]);
        i.prefer = Some("claude".into());
        assert_eq!(rank(&i, &profiles()), ["claude", "grok"]);
    }

    #[test]
    fn only_the_top_agent_may_take_an_item_inside_its_reserve_window() {
        let p = profiles();
        let i = item(1, &["code"]);
        let now = t0() + Duration::seconds(10);
        assert!(eligible("claude", &i, &p, now));
        assert!(!eligible("muse", &i, &p, now));
        assert!(!eligible("grok", &i, &p, now));
    }

    #[test]
    fn after_the_reserve_window_any_agent_that_can_do_it_may() {
        let p = profiles();
        let i = item(1, &["code"]);
        let later = t0() + Duration::seconds(DEFAULT_RESERVE_SECS);
        assert!(eligible("claude", &i, &p, later));
        assert!(eligible("muse", &i, &p, later));
        assert!(eligible("codex", &i, &p, later));
        // grok has no `code`: never.
        assert!(!eligible("grok", &i, &p, later));
    }

    #[test]
    fn an_agent_without_a_profile_takes_only_untagged_work_after_the_window() {
        let p = profiles();
        let later = t0() + Duration::seconds(DEFAULT_RESERVE_SECS + 1);
        assert!(eligible("stranger", &item(1, &[]), &p, later));
        assert!(!eligible("stranger", &item(2, &["code"]), &p, later));
        let early = t0() + Duration::seconds(1);
        assert!(!eligible("stranger", &item(1, &[]), &p, early));
    }

    #[test]
    fn with_no_profiles_anyone_takes_untagged_work_and_nobody_tagged_work() {
        let now = t0() + Duration::seconds(1);
        assert!(eligible("anyone", &item(1, &[]), &[], now));
        assert!(!eligible("anyone", &item(2, &["code"]), &[], now));
    }

    #[test]
    fn a_zero_reserve_opens_the_item_at_once() {
        let mut i = item(1, &["code"]);
        i.reserve_secs = 0;
        assert!(eligible("muse", &i, &profiles(), t0()));
    }

    #[test]
    fn order_prefers_items_the_agent_ranks_first_then_priority_then_age() {
        let p = profiles();
        let later = t0() + Duration::seconds(DEFAULT_RESERVE_SECS + 1);
        let mut urgent = item(1, &["docs"]); // claude can, muse can: muse ranks first
        urgent.priority = 9;
        let mut code = item(2, &["code"]); // claude ranks first
        code.created_at = t0() + Duration::seconds(5);
        let old_code = item(3, &["code"]);
        let items = vec![urgent.clone(), code.clone(), old_code.clone()];
        let ids: Vec<Uuid> = order_for("claude", &items, &p, later)
            .iter()
            .map(|i| i.id)
            .collect();
        // claude ranks first on both code items (oldest first), then the urgent docs item.
        assert_eq!(ids, [old_code.id, code.id, urgent.id]);
    }

    #[test]
    fn order_is_deterministic_for_equal_items() {
        let p = profiles();
        let a = item(7, &[]);
        let b = item(3, &[]);
        let now = t0() + Duration::seconds(1);
        let one: Vec<Uuid> = order_for("muse", &[a.clone(), b.clone()], &p, now)
            .iter()
            .map(|i| i.id)
            .collect();
        let two: Vec<Uuid> = order_for("muse", &[b, a], &p, now)
            .iter()
            .map(|i| i.id)
            .collect();
        assert_eq!(one, two);
        assert_eq!(one[0], Uuid::from_u128(3));
    }

    #[test]
    fn aliases_resolve_to_the_profile() {
        let mut p = profiles();
        p[3].aliases = tags(&["codex-mcp-client"]);
        assert_eq!(resolve(&p, "codex-mcp-client").unwrap().name, "codex");
        assert_eq!(resolve(&p, "grok").unwrap().name, "grok");
        assert!(resolve(&p, "nobody").is_none());
    }

    #[test]
    fn an_excluded_agent_neither_takes_an_item_nor_holds_its_window() {
        // claude hands off code work: claude would rank first, but it may not take its own task
        let it = item(1, &["code"]);
        let ex: HashMap<Uuid, String> = [(it.id, "claude".to_string())].into();
        let items = [it];
        let now = t0(); // inside the reserve window
        assert!(order_for_excluding("claude", &items, &profiles(), now, &ex).is_empty());
        // of the others, muse (can code, cost 1) beats codex (can code, cost 3): muse holds it
        assert_eq!(
            order_for_excluding("muse", &items, &profiles(), now, &ex).len(),
            1
        );
        assert!(order_for_excluding("codex", &items, &profiles(), now, &ex).is_empty());
        // without the exclusion claude holds the window and muse waits
        assert!(order_for("muse", &items, &profiles(), now).is_empty());
    }
}
