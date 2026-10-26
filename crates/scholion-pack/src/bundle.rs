//! Handoff bundle (Plan 3 Task 3): what the next agent needs to continue a task, within a token
//! budget. Same allocator as context packs, with its own sections and shares. The review (PR)
//! bundle (Task 4) adds test status, risks and a conventions checklist.

use std::collections::HashMap;

use scholion_core::{
    AgentFamily, Convention, HandoffState, Record, RecordKind, Result, SymbolId, Task, TaskId,
    Tier, Vcs,
};
use scholion_index::IndexedSymbol;
use serde::{Deserialize, Serialize};

use crate::assemble::{record_texts, symbol_handle};
use crate::budget::{Item, Section, allocate_with_shares};
use crate::render::{fit, push_footer, push_section};
use crate::tokens::estimate_tokens;

/// (group, share %, heading) in fill and render order.
type Layout = [(&'static str, u8, &'static str)];
const HANDOFF: &Layout = &[
    ("goal", 10, "Goal"),
    ("state", 25, "State"),
    ("records", 30, "Records"),
    ("changes", 30, "Changes"),
    ("questions", 5, "Open questions"),
];
const REVIEW: &Layout = &[
    ("goal", 10, "Goal"),
    ("tests", 5, "Tests"),
    ("risks", 10, "Risks"),
    ("checklist", 10, "Checklist"),
    ("changes", 30, "Changes"),
    ("records", 20, "Records"),
    ("state", 10, "State"),
    ("questions", 5, "Open questions"),
];
/// A changed symbol referenced by at least this many symbols is a risk area.
const RISKY_IN_DEGREE: u32 = 5;
/// Longest diff shown for one changed symbol.
const MAX_HUNK_LINES: usize = 60;

#[derive(Debug, Clone)]
pub struct BundleRequest {
    pub task_id: TaskId,
    pub budget_tokens: u32,
    pub family: AgentFamily,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bundle {
    pub markdown: String,
    /// Estimated tokens of `markdown`; never above the requested budget.
    pub tokens: u32,
    /// Handles of items that did not fit (at most 20).
    pub omitted: Vec<String>,
}

fn item(id: String, group: &str, score: f32, tiers: [String; 3]) -> Item {
    Item {
        id,
        section: Section::Task, // unused: bundles fill by group
        score,
        tiers,
        reason: "",
        source: None,
        required: false,
        group: Some(group.to_owned()),
    }
}

/// `label: text` as handle (first line, clipped), summary (first three lines) and full text.
fn labelled(label: &str, text: &str) -> [String; 3] {
    let lines: Vec<&str> = text.trim().lines().collect();
    let first = lines.first().copied().unwrap_or("");
    let handle = if first.chars().count() > 80 || lines.len() > 1 {
        format!(
            "{label}: {}…",
            first.chars().take(80).collect::<String>().trim_end()
        )
    } else {
        format!("{label}: {first}")
    };
    let block = |n: usize| {
        let mut s = format!(
            "{label}: {}",
            lines
                .iter()
                .take(n)
                .copied()
                .collect::<Vec<_>>()
                .join("\n  ")
        );
        if lines.len() > n {
            s.push_str("\n  …");
        }
        s
    };
    [handle, block(3), block(usize::MAX)]
}

/// The newest handoff record that nothing supersedes, as the state it carries.
pub(crate) fn latest_handoff(records: &[Record]) -> Option<HandoffState> {
    let rec = records
        .iter()
        .rfind(|r| r.kind == RecordKind::Handoff && r.superseded_by.is_none())?;
    Some(
        serde_json::from_str(&rec.body).unwrap_or_else(|_| HandoffState {
            remaining: if rec.body.trim().is_empty() {
                rec.summary.clone()
            } else {
                rec.body.clone()
            },
            ..HandoffState::default()
        }),
    )
}

fn goal_item(task: &Task) -> Item {
    let head = format!(
        "# {} · {} · {}",
        task.title,
        task.slug,
        task.status.as_str()
    );
    let first = task.goal.lines().next().unwrap_or("").trim();
    let meta = format!(
        "Owner: {}\nWorkspace: {}",
        task.owner_agent,
        task.workspace_path.display()
    );
    let mut it = item(
        "goal".into(),
        "goal",
        10.0,
        [
            head.clone(),
            format!("{head}\nGoal: {first}\n{meta}"),
            format!("{head}\nGoal: {}\n{meta}", task.goal.trim()),
        ],
    );
    it.required = true;
    it
}

fn state_items(state: &HandoffState) -> Vec<Item> {
    let mut out = vec![];
    // remaining and broken are what the next agent acts on first; they are never ranked out
    for (key, label, text, score, required) in [
        ("remaining", "Remaining", &state.remaining, 3.0, true),
        ("broken", "Currently broken", &state.broken, 2.5, true),
        ("done", "Done", &state.done, 2.0, false),
    ] {
        if text.trim().is_empty() {
            continue;
        }
        let mut it = item(
            format!("state:{key}"),
            "state",
            score,
            labelled(label, text),
        );
        it.required = required;
        out.push(it);
    }
    out
}

/// Records other than the intent (shown as the goal) and handoffs (shown as the state), without
/// superseded ones. Decisions and dead ends rank first, newer before older.
fn record_items(records: &[Record]) -> Vec<Item> {
    let shown: Vec<&Record> = records
        .iter()
        .filter(|r| !matches!(r.kind, RecordKind::Intent | RecordKind::Handoff))
        .filter(|r| r.superseded_by.is_none())
        .collect();
    let n = shown.len().max(1) as f32;
    shown
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let kind_score = match r.kind {
                RecordKind::Decision | RecordKind::Attempt => 2.0,
                RecordKind::Review => 1.5,
                _ => 1.0,
            };
            let (id, tiers) = record_texts(r, false, false);
            item(id, "records", kind_score + i as f32 / n * 0.5, tiers)
        })
        .collect()
}

/// Unified-diff hunks of `base → head` whose new-side lines overlap `start..=end`.
fn hunks_for(base: &str, head: &str, start: u32, end: u32) -> String {
    let diff = similar::TextDiff::from_lines(base, head);
    let mut out = String::new();
    for hunk in diff.unified_diff().context_radius(1).iter_hunks() {
        let ops = hunk.ops();
        let (Some(first), Some(last)) = (ops.first(), ops.last()) else {
            continue;
        };
        let (from, to) = (
            first.new_range().start as u32 + 1,
            last.new_range().end as u32,
        );
        if from <= end && start <= to.max(from) {
            out.push_str(&hunk.to_string());
        }
    }
    let lines: Vec<&str> = out.lines().collect();
    if lines.len() > MAX_HUNK_LINES {
        format!(
            "{}\n… ({} more lines)",
            lines[..MAX_HUNK_LINES].join("\n"),
            lines.len() - MAX_HUNK_LINES
        )
    } else {
        out.trim_end().to_owned()
    }
}

async fn read_text(
    vcs: &dyn Vcs,
    task: &Task,
    change: &scholion_core::ChangeId,
    path: &str,
) -> String {
    match vcs.read_file(&task.repo, change, path).await {
        Ok(b) => String::from_utf8_lossy(&b).into_owned(),
        Err(_) => String::new(), // added or deleted: diff against nothing
    }
}

/// One item per changed symbol (handle, signature, its hunks), then one per changed file that has
/// no changed symbol.
async fn change_items(
    vcs: &dyn Vcs,
    task: &Task,
    changed: &[String],
    symbols: &[IndexedSymbol],
) -> Vec<Item> {
    let mut out = vec![];
    let total = (changed.len() + symbols.len()).max(1) as f32;
    let mut rank = 0usize;
    let mut score = || {
        rank += 1;
        2.0 - rank as f32 / total
    };
    for path in changed {
        let syms: Vec<&IndexedSymbol> = symbols.iter().filter(|s| &s.symbol.path == path).collect();
        if syms.is_empty() {
            out.push(item(
                format!("file:{path}"),
                "changes",
                score() - 1.0,
                [
                    format!("- {path} (changed)"),
                    format!("- {path} (changed)"),
                    format!("- {path} (changed)"),
                ],
            ));
            continue;
        }
        let base = read_text(vcs, task, &task.base_change, path).await;
        let head = read_text(vcs, task, &task.head_change, path).await;
        for s in &syms {
            let handle = symbol_handle(s);
            let summary = format!("{handle}\n  {}", s.symbol.signature.trim());
            // a hunk is shown once, with the innermost changed symbol (the method, not its class)
            let encloses = syms.iter().any(|t| {
                t.id != s.id
                    && s.symbol.start_line <= t.symbol.start_line
                    && t.symbol.end_line <= s.symbol.end_line
            });
            let hunks = if encloses {
                String::new()
            } else {
                hunks_for(&base, &head, s.symbol.start_line, s.symbol.end_line)
            };
            let full = if hunks.is_empty() {
                summary.clone()
            } else {
                format!("{handle}\n```diff\n{hunks}\n```")
            };
            out.push(item(
                format!("sym:{}", s.id.0),
                "changes",
                score(),
                [handle, summary, full],
            ));
        }
    }
    out
}

fn questions_item(state: &HandoffState) -> Option<Item> {
    (!state.questions.trim().is_empty()).then(|| {
        item(
            "questions".into(),
            "questions",
            1.0,
            labelled("Question", &state.questions),
        )
    })
}

fn short(hex: &str) -> &str {
    &hex[..hex.len().min(12)]
}

/// The newest `validation` record bound to the task's current head.
fn tests_item(task: &Task, records: &[Record]) -> Item {
    let head = short(&task.head_change.0);
    let text = match records.iter().rfind(|r| {
        r.kind == RecordKind::Validation && r.change_id.as_ref() == Some(&task.head_change)
    }) {
        Some(r) => {
            let body: serde_json::Value = serde_json::from_str(&r.body).unwrap_or_default();
            let command = body["command"].as_str().unwrap_or(&r.summary);
            match body["exit_code"].as_i64() {
                Some(0) => format!("Tests: `{command}` passed at head {head}"),
                code => format!(
                    "Tests: `{command}` FAILED (exit {}) at head {head}",
                    code.map_or("?".to_owned(), |c| c.to_string())
                ),
            }
        }
        None => {
            format!("Tests: not run at this head ({head}); the owner runs `av validate -- <cmd>`")
        }
    };
    let mut it = item(
        "tests".into(),
        "tests",
        10.0,
        [text.clone(), text.clone(), text],
    );
    it.required = true;
    it
}

/// Decisions with no `attempt` on any of their anchors (no alternatives recorded), and changed
/// symbols that many others reference.
fn risk_items(
    records: &[Record],
    symbols: &[IndexedSymbol],
    in_degree: &HashMap<SymbolId, u32>,
) -> Vec<Item> {
    let mut out = vec![];
    for s in symbols {
        let n = in_degree.get(&s.id).copied().unwrap_or(0);
        if n >= RISKY_IN_DEGREE {
            let h = format!(
                "- ⚠ sym:{} {} is referenced by {n} symbols; check its callers",
                s.id.0, s.symbol.qname
            );
            out.push(item(
                format!("risk:sym:{}", s.id.0),
                "risks",
                2.0 + n as f32 / 100.0,
                [h.clone(), h.clone(), h],
            ));
        }
    }
    let attempts: Vec<&Record> = records
        .iter()
        .filter(|r| r.kind == RecordKind::Attempt && r.superseded_by.is_none())
        .collect();
    for d in records.iter().filter(|r| {
        r.kind == RecordKind::Decision && r.superseded_by.is_none() && !r.anchors.is_empty()
    }) {
        let weighed = attempts
            .iter()
            .any(|a| a.anchors.iter().any(|x| d.anchors.contains(x)));
        if !weighed {
            let (id, _) = record_texts(d, false, false);
            let h = format!(
                "- ⚠ {id} decision with no alternatives recorded: {}",
                d.summary
            );
            out.push(item(
                format!("risk:{id}"),
                "risks",
                1.5,
                [h.clone(), h.clone(), h],
            ));
        }
    }
    out
}

fn checklist_items(conventions: &[Convention]) -> Vec<Item> {
    conventions
        .iter()
        .map(|c| {
            let h = format!("- [ ] conv:{} ({}) {}", c.id.0, c.scope_glob, c.text);
            let mut it = item(
                format!("check:{}", c.id.0),
                "checklist",
                1.0,
                [h.clone(), h.clone(), h],
            );
            it.required = c.required;
            it
        })
        .collect()
}

fn render_bundle(
    layout: &Layout,
    chosen: &[(Item, Tier)],
    omitted: &[String],
    omitted_total: usize,
    tokens: u32,
    budget: u32,
) -> String {
    let mut out = String::new();
    for (group, _, heading) in layout {
        let items: Vec<&(Item, Tier)> = chosen
            .iter()
            .filter(|(i, _)| i.group.as_deref() == Some(*group))
            .collect();
        push_section(&mut out, heading, &items, true);
    }
    push_footer(&mut out, omitted, omitted_total, tokens, budget);
    out
}

/// What the review bundle adds: conventions in scope of the changed paths, and per changed
/// symbol how many symbols reference it.
pub(crate) struct ReviewInputs {
    pub conventions: Vec<Convention>,
    pub in_degree: HashMap<SymbolId, u32>,
}

pub(crate) struct BundleInputs<'a> {
    pub task: &'a Task,
    pub records: &'a [Record],
    pub changed: &'a [String],
    pub symbols: &'a [IndexedSymbol],
    /// `Some`: the review bundle.
    pub review: Option<ReviewInputs>,
}

pub(crate) async fn build(
    vcs: &dyn Vcs,
    inp: BundleInputs<'_>,
    req: &BundleRequest,
) -> Result<Bundle> {
    if req.budget_tokens == 0 {
        return Ok(Bundle {
            markdown: String::new(),
            tokens: 0,
            omitted: vec![],
        });
    }
    let state = latest_handoff(inp.records).unwrap_or_default();
    let mut items = vec![goal_item(inp.task)];
    items.extend(state_items(&state));
    items.extend(record_items(inp.records));
    items.extend(change_items(vcs, inp.task, inp.changed, inp.symbols).await);
    items.extend(questions_item(&state));
    let layout = match &inp.review {
        Some(r) => {
            items.push(tests_item(inp.task, inp.records));
            items.extend(risk_items(inp.records, inp.symbols, &r.in_degree));
            items.extend(checklist_items(&r.conventions));
            REVIEW
        }
        None => HANDOFF,
    };

    let skeleton = estimate_tokens(
        &render_bundle(layout, &[], &[], 99_999, 99_999, 99_999),
        req.family,
    );
    let shares: Vec<(String, u8)> = layout
        .iter()
        .map(|(k, p, _)| ((*k).to_owned(), *p))
        .collect();
    let alloc = allocate_with_shares(
        items,
        req.budget_tokens.saturating_sub(skeleton),
        req.family,
        &shares,
    );
    let mut chosen = alloc.chosen;
    let mut omitted_total = alloc.omitted_total;
    let (markdown, tokens) = fit(
        &mut chosen,
        &mut omitted_total,
        req.budget_tokens,
        req.family,
        |c, total, n| render_bundle(layout, c, &alloc.omitted, total, n, req.budget_tokens),
    );
    Ok(Bundle {
        markdown,
        tokens,
        omitted: alloc.omitted,
    })
}
