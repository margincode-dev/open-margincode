//! E-G1 capability profile (`margincode-bench routing-profile`). Pure: no daemon, no network.
//!
//! The results file is JSONL (one cell per line; blank lines and `#` comments are ignored) or a
//! JSON array of the same objects. Fields:
//! - `tag` — `image`, `ui`, `code`, `tests`, `docs`, `math`, or any other tag
//! - `agent` — the profile name (`av agent set` uses it)
//! - `trials` — sessions in the cell
//! - `accepted` — how many of those the hidden tests accepted
//! - `tokens_per_accepted` — that vendor's tokens per accepted task; null when nothing was
//!   accepted (the number is then ignored)
//!
//! For a tag, an agent is `best` when its accepted rate is within 10 points of the highest and
//! no other such agent is cheaper in tokens per accepted task. It is `can` when its accepted
//! rate is at least half the highest (and it is not `best`). Otherwise it is left out.
//! A tie for `best`, or a cell with fewer than 10 trials, is `inconclusive`. An agent with any
//! inconclusive cell gets no `av agent set` line, so the command cannot replace the user's profile.
//! Cost rank is not in the file: the printed lines do not pass `--cost`.

use std::collections::BTreeMap;

use serde::Deserialize;

use crate::Result;
use crate::stats::wilson_interval;

const MIN_TRIALS: u32 = 10;
/// 10 percentage points, with rates stored as parts per million.
const NEAR_PPM: i64 = 100_000;
const TAG_ORDER: [&str; 6] = ["image", "ui", "code", "tests", "docs", "math"];

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Cell {
    pub tag: String,
    pub agent: String,
    pub trials: u32,
    pub accepted: u32,
    #[serde(default)]
    pub tokens_per_accepted: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Call {
    Best,
    Can,
    Out,
    Inconclusive,
}

impl Call {
    pub fn as_str(self) -> &'static str {
        match self {
            Call::Best => "best",
            Call::Can => "can",
            Call::Out => "out",
            Call::Inconclusive => "inconclusive",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellDecision {
    pub tag: String,
    pub agent: String,
    pub call: Call,
}

/// `accepted / trials` in parts per million, rounded to the nearest integer.
fn rate_ppm(accepted: u32, trials: u32) -> i64 {
    let t = i64::from(trials);
    (i64::from(accepted) * 1_000_000 + t / 2) / t
}

fn tag_rank(tag: &str) -> (usize, &str) {
    match TAG_ORDER.iter().position(|t| *t == tag) {
        Some(i) => (i, ""),
        None => (TAG_ORDER.len(), tag),
    }
}

/// A finite token cost is cheaper than a missing one. Two missing costs are not ordered.
fn cheaper(a: Option<u64>, b: Option<u64>) -> bool {
    match (a, b) {
        (Some(x), Some(y)) => x < y,
        (Some(_), None) => true,
        _ => false,
    }
}

fn cost_of(cell: &Cell) -> Option<u64> {
    if cell.accepted == 0 {
        None
    } else {
        cell.tokens_per_accepted
    }
}

pub fn parse_results(text: &str) -> Result<Vec<Cell>> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err("results file is empty".into());
    }
    let cells: Vec<Cell> = if trimmed.starts_with('[') {
        serde_json::from_str(trimmed).map_err(|e| format!("results: {e}"))?
    } else {
        let mut cells = vec![];
        for (i, line) in text.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let cell: Cell =
                serde_json::from_str(line).map_err(|e| format!("results line {}: {e}", i + 1))?;
            cells.push(cell);
        }
        cells
    };
    if cells.is_empty() {
        return Err("results file has no cells".into());
    }
    let mut seen = BTreeMap::<(&str, &str), ()>::new();
    for c in &cells {
        if c.tag.is_empty() || c.agent.is_empty() {
            return Err("tag and agent must be non-empty".into());
        }
        if c.accepted > c.trials {
            return Err(format!(
                "{} {}: accepted {} is greater than trials {}",
                c.tag, c.agent, c.accepted, c.trials
            ));
        }
        if seen
            .insert((c.tag.as_str(), c.agent.as_str()), ())
            .is_some()
        {
            return Err(format!("{} {} is listed twice", c.tag, c.agent));
        }
    }
    Ok(cells)
}

struct Rated<'a> {
    agent: &'a str,
    ppm: i64,
    cost: Option<u64>,
}

/// One decision per agent that appears, for every tag that appears. Missing pairs count as
/// no trials (inconclusive).
pub fn decide(cells: &[Cell]) -> Vec<CellDecision> {
    let mut tags: Vec<&str> = cells.iter().map(|c| c.tag.as_str()).collect();
    tags.sort_by(|a, b| tag_rank(a).cmp(&tag_rank(b)).then(a.cmp(b)));
    tags.dedup();
    let mut agents: Vec<&str> = cells.iter().map(|c| c.agent.as_str()).collect();
    agents.sort_unstable();
    agents.dedup();

    let mut out = vec![];
    for tag in tags {
        let mut by_agent: BTreeMap<&str, &Cell> = BTreeMap::new();
        for c in cells.iter().filter(|c| c.tag == tag) {
            by_agent.insert(c.agent.as_str(), c);
        }
        let conclusive: Vec<Rated<'_>> = agents
            .iter()
            .filter_map(|agent| {
                let cell = by_agent.get(agent)?;
                if cell.trials < MIN_TRIALS {
                    return None;
                }
                Some(Rated {
                    agent,
                    ppm: rate_ppm(cell.accepted, cell.trials),
                    cost: cost_of(cell),
                })
            })
            .collect();
        let highest = conclusive.iter().map(|r| r.ppm).max();
        let near: Vec<&Rated<'_>> = match highest {
            Some(high) => conclusive
                .iter()
                .filter(|r| high - r.ppm <= NEAR_PPM)
                .collect(),
            None => vec![],
        };
        let bests: Vec<&str> = near
            .iter()
            .filter(|r| {
                near.iter()
                    .all(|o| o.agent == r.agent || !cheaper(o.cost, r.cost))
            })
            .map(|r| r.agent)
            .collect();
        let tied = bests.len() != 1;
        for agent in &agents {
            let call = if conclusive.iter().all(|r| r.agent != *agent) {
                Call::Inconclusive
            } else if !tied && bests.first() == Some(agent) {
                Call::Best
            } else if tied && bests.contains(agent) {
                Call::Inconclusive
            } else if let Some(high) = highest {
                let ppm = conclusive
                    .iter()
                    .find(|r| r.agent == *agent)
                    .map(|r| r.ppm)
                    .expect("agent is in the conclusive set");
                if ppm * 2 >= high {
                    Call::Can
                } else {
                    Call::Out
                }
            } else {
                Call::Inconclusive
            };
            out.push(CellDecision {
                tag: tag.to_owned(),
                agent: (*agent).to_owned(),
                call,
            });
        }
    }
    out
}

fn why_inconclusive(cells: &[Cell], tag: &str, agent: &str) -> String {
    match cells.iter().find(|c| c.tag == tag && c.agent == agent) {
        Some(c) if c.trials < MIN_TRIALS => format!("{tag}: {} trials", c.trials),
        Some(_) => format!("{tag}: tie"),
        None => format!("{tag}: no trials"),
    }
}

/// `av agent set` lines for agents whose every cell is conclusive, and a `#` note for the rest.
/// Notes are not commands. Order is by agent name; tags follow [`TAG_ORDER`].
pub fn commands(cells: &[Cell], decisions: &[CellDecision]) -> Vec<String> {
    let mut agents: Vec<&str> = decisions.iter().map(|d| d.agent.as_str()).collect();
    agents.sort_unstable();
    agents.dedup();
    let mut lines = vec![];
    for agent in agents {
        let mine: Vec<&CellDecision> = decisions.iter().filter(|d| d.agent == agent).collect();
        if mine.iter().any(|d| d.call == Call::Inconclusive) {
            let why: Vec<String> = mine
                .iter()
                .filter(|d| d.call == Call::Inconclusive)
                .map(|d| why_inconclusive(cells, &d.tag, agent))
                .collect();
            lines.push(format!(
                "# {agent}: inconclusive ({}); profile left unchanged",
                why.join(", ")
            ));
            continue;
        }
        let mut best: Vec<&str> = mine
            .iter()
            .filter(|d| d.call == Call::Best)
            .map(|d| d.tag.as_str())
            .collect();
        let mut can: Vec<&str> = mine
            .iter()
            .filter(|d| d.call == Call::Can)
            .map(|d| d.tag.as_str())
            .collect();
        best.sort_by(|a, b| tag_rank(a).cmp(&tag_rank(b)));
        can.sort_by(|a, b| tag_rank(a).cmp(&tag_rank(b)));
        let mut cmd = format!("av agent set {agent}");
        if !best.is_empty() {
            cmd.push_str(" --best ");
            cmd.push_str(&best.join(","));
        }
        if !can.is_empty() {
            cmd.push_str(" --can ");
            cmd.push_str(&can.join(","));
        }
        lines.push(cmd);
    }
    lines
}

fn pct(accepted: u32, trials: u32) -> String {
    if trials == 0 {
        return "-".into();
    }
    format!("{:.1}%", accepted as f64 / trials as f64 * 100.0)
}

fn wilson_cell(cell: Option<&Cell>) -> String {
    let Some(cell) = cell else {
        return "-".into();
    };
    match wilson_interval(u64::from(cell.accepted), u64::from(cell.trials)) {
        Some((lo, hi)) => format!("[{:.1}%, {:.1}%]", lo * 100.0, hi * 100.0),
        None => "-".into(),
    }
}

/// The table, then the commands. Field rules are the leading `#` lines.
pub fn render(cells: &[Cell]) -> String {
    let decisions = decide(cells);
    let mut rows: Vec<[String; 8]> = vec![];
    for d in &decisions {
        let cell = cells.iter().find(|c| c.tag == d.tag && c.agent == d.agent);
        let (trials, accepted, tokens) = match cell {
            Some(c) => (
                c.trials.to_string(),
                c.accepted.to_string(),
                c.tokens_per_accepted
                    .filter(|_| c.accepted > 0)
                    .map(|n| n.to_string())
                    .unwrap_or_else(|| "-".into()),
            ),
            None => ("0".into(), "0".into(), "-".into()),
        };
        let rate = cell
            .map(|c| pct(c.accepted, c.trials))
            .unwrap_or_else(|| "-".into());
        rows.push([
            d.tag.clone(),
            d.agent.clone(),
            trials,
            accepted,
            rate,
            wilson_cell(cell),
            tokens,
            d.call.as_str().to_owned(),
        ]);
    }
    let header = [
        "tag",
        "agent",
        "trials",
        "accepted",
        "rate",
        "wilson 95%",
        "tokens/accepted",
        "call",
    ];
    let widths: Vec<usize> = (0..header.len())
        .map(|i| {
            header[i]
                .len()
                .max(rows.iter().map(|r| r[i].len()).max().unwrap_or(0))
        })
        .collect();
    let line = |cols: &[String]| -> String {
        cols.iter()
            .enumerate()
            .map(|(i, c)| format!("{c:<width$}", width = widths[i]))
            .collect::<Vec<_>>()
            .join("  ")
            .trim_end()
            .to_owned()
    };
    let mut text = String::from(
        "# routing profile\n\
         # fields: tag, agent, trials, accepted, tokens_per_accepted (null if nothing was accepted)\n\
         # best: accepted rate within 10 points of the highest, and no other such agent is cheaper in tokens per accepted task\n\
         # can: accepted rate at least half the highest, and not best\n\
         # out: below that\n\
         # inconclusive: fewer than 10 trials, or a tie for best. No av agent set line for that agent.\n",
    );
    text.push_str(&line(&header.map(str::to_owned)));
    text.push('\n');
    for row in &rows {
        text.push_str(&line(row));
        text.push('\n');
    }
    text.push('\n');
    for cmd in commands(cells, &decisions) {
        text.push_str(&cmd);
        text.push('\n');
    }
    text.push_str(
        "# Cost is not in the results. Add --cost N yourself (1 is the cheapest); without it, av's default is 5, and the line replaces the whole profile.\n",
    );
    text
}

pub fn report(text: &str) -> Result<String> {
    Ok(render(&parse_results(text)?))
}
