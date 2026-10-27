//! Candidate gathering: turns a task into scored, tiered `Item`s.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::sync::Arc;

use chrono::Utc;
use futures::future::join_all;
use scholion_core::*;
use scholion_index::{AnchorResolution, CodeIndex, IndexedSymbol};

use crate::budget::{Item, Section};
use crate::lexical;
use crate::score::{self, FAR_HOPS};

pub(crate) struct Deps<'a> {
    pub vcs: &'a dyn Vcs,
    pub store: &'a dyn IntentStore,
    pub index: &'a dyn CodeIndex,
    pub semantic: Option<Arc<dyn Semantic>>,
}

pub(crate) struct Gathered {
    pub items: Vec<Item>,
    pub degraded: Vec<String>,
}

struct Structural {
    gathered: Gathered,
    hops_by_sym: HashMap<SymbolId, u8>,
    seed_set: HashSet<SymbolId>,
    seed_paths: BTreeSet<String>,
    neighbor_paths: BTreeSet<String>,
    contents: HashMap<String, String>,
    /// `sym:` id → test-file factor applied on top of `code_score`.
    code_factor: HashMap<String, f32>,
    located: Vec<IndexedSymbol>,
    has_head: bool,
}

const MAX_NEIGHBORS: usize = 40;
/// With no seeds (a fresh task whose goal names no identifier), this many of the best text hits
/// stand in for seeds, so what they reference comes in too.
const HIT_SEEDS: usize = 2;
const FOCUS_LIMIT: usize = 10;
/// Symbols one identifier named in the goal may seed (a method name shared by many classes).
const NAMED_PER_IDENT: usize = 4;
const MAX_CODE_LINES: usize = 120;
const MAX_SKELETON_LINES: usize = 30;
const MAX_TREE_LINES: usize = 80;
const MAX_CHANGED_LISTED: usize = 20;
const MAX_UNREADABLE_NOTED: usize = 3;
/// Above every `score::record_score` result: pinned review findings lead the History section.
const PINNED_REVIEW_SCORE: f32 = 1000.0;

pub(crate) fn short_id(prefix: &str, uuid: &uuid::Uuid) -> String {
    format!("{prefix}:{}", &uuid.simple().to_string()[..8])
}

fn clip(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_owned()
    } else {
        format!("{}…", s.chars().take(max).collect::<String>().trim_end())
    }
}

fn item(id: String, section: Section, score: f32, texts: [String; 3]) -> Item {
    Item {
        id,
        section,
        score,
        tiers: texts,
        reason: "",
        source: None,
        required: false,
        group: None,
    }
}

pub(crate) fn lang_of(path: &str) -> &'static str {
    match path.rsplit_once('.').map(|(_, e)| e) {
        Some("py") => "python",
        Some("ts") | Some("tsx") => "typescript",
        Some("rs") => "rust",
        _ => "",
    }
}

/// `lines` start..=end (1-based), at most `cap` lines, with a truncation note.
pub(crate) fn excerpt(content: &str, start: u32, end: u32, cap: usize) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let from = (start.max(1) as usize - 1).min(lines.len());
    let to = (end as usize).min(lines.len()).max(from);
    let body = &lines[from..to];
    let mut text = body
        .iter()
        .take(cap)
        .copied()
        .collect::<Vec<_>>()
        .join("\n");
    if body.len() > cap {
        text.push_str(&format!(
            "\n… ({} more lines; expand the id for the rest)",
            body.len() - cap
        ));
    }
    text
}

pub(crate) fn code_block(path: &str, code: &str) -> String {
    format!("```{}\n{}\n```", lang_of(path), code)
}

pub(crate) fn symbol_handle(s: &IndexedSymbol) -> String {
    format!(
        "- sym:{} {} ({}:{}-{})",
        s.id.0, s.symbol.qname, s.symbol.path, s.symbol.start_line, s.symbol.end_line
    )
}

/// `stale`: an anchored symbol or file is gone. `changed`: the anchored code changed after the record
/// was written, so the claim may no longer hold (Plan 2b). `stale` wins when both apply.
pub(crate) fn record_texts(rec: &Record, stale: bool, changed: bool) -> (String, [String; 3]) {
    let id = short_id("rec", &rec.id.0);
    let flag = if stale {
        " (anchor_stale)"
    } else if changed {
        " (changed since recorded)"
    } else {
        ""
    };
    let kind = rec.kind.as_str();
    let handle = format!("- {id} {kind}: {}{flag}", clip(&rec.summary, 70));
    let mut summary = format!("- {id} {kind}: {}{flag}", rec.summary);
    if !rec.anchors.is_empty() {
        let anchors: Vec<String> = rec
            .anchors
            .iter()
            .map(|a| match &a.symbol {
                Some(s) => format!("{}:{s}", a.path),
                None => a.path.clone(),
            })
            .collect();
        summary.push_str(&format!("\n  anchors: {}", anchors.join(", ")));
    }
    let mut full = summary.clone();
    if !rec.body.trim().is_empty() {
        for line in rec.body.trim().lines() {
            full.push_str(&format!("\n  {line}"));
        }
    }
    (id, [handle, summary, full])
}

/// Symbols of `changed` paths whose lines changed between base and head (every symbol of a new or
/// unreadable-at-base file), in the task's own index scope.
pub(crate) async fn changed_symbols(
    d: &Deps<'_>,
    task: &Task,
    changed: &[String],
) -> Result<Vec<IndexedSymbol>> {
    let has_both = !task.base_change.0.is_empty() && !task.head_change.0.is_empty();
    let mut out = vec![];
    for path in changed {
        let syms = d.index.symbols_in(&task.repo, &task.slug, path).await?;
        if syms.is_empty() {
            continue;
        }
        let ranges = if has_both {
            changed_ranges_of(d.vcs, &task.repo, task, path).await
        } else {
            None
        };
        match ranges {
            None => out.extend(syms),
            Some(r) => out.extend(syms.into_iter().filter(|s| {
                r.iter()
                    .any(|(a, b)| s.symbol.start_line <= *b && *a <= s.symbol.end_line)
            })),
        }
    }
    Ok(out)
}

/// 1-based inclusive line ranges of `path` at head that differ from base. `None` = no usable base (a new
/// file, or unreadable): treat the whole file as changed.
async fn changed_ranges_of(
    vcs: &dyn Vcs,
    repo: &str,
    task: &Task,
    path: &str,
) -> Option<Vec<(u32, u32)>> {
    let (base, head) = futures::join!(
        vcs.read_file(repo, &task.base_change, path),
        vcs.read_file(repo, &task.head_change, path)
    );
    let (base, head) = (
        String::from_utf8_lossy(&base.ok()?).into_owned(),
        String::from_utf8_lossy(&head.ok()?).into_owned(),
    );
    Some(line_ranges(&base, &head))
}

fn line_ranges(base: &str, head: &str) -> Vec<(u32, u32)> {
    use similar::DiffOp;
    let diff = similar::TextDiff::from_lines(base, head);
    let mut out = vec![];
    for op in diff.ops() {
        match *op {
            DiffOp::Equal { .. } => {}
            DiffOp::Insert {
                new_index, new_len, ..
            }
            | DiffOp::Replace {
                new_index, new_len, ..
            } => {
                out.push((new_index as u32 + 1, (new_index + new_len) as u32));
            }
            // lines removed between head lines `new_index` and `new_index + 1`: both neighbours are touched
            DiffOp::Delete { new_index, .. } => {
                out.push(((new_index as u32).max(1), new_index as u32 + 1))
            }
        }
    }
    out
}

/// The auto-recorded intent (summary = first line of the goal, clipped to the summary limit; body
/// empty or the goal itself) adds nothing next to the `Goal:` line of the task header.
fn repeats_goal(rec: &Record, task: &Task) -> bool {
    let goal = task.goal.trim();
    let first = goal.lines().next().unwrap_or("").trim();
    let summary = rec.summary.trim();
    let body = rec.body.trim();
    !summary.is_empty() && first.starts_with(summary) && (body.is_empty() || body == goal)
}

struct Located {
    sym: IndexedSymbol,
    hops: u8,
}

fn semantic_query(focus: Option<&str>, goal: &str) -> Option<String> {
    if let Some(q) = focus.map(str::trim).filter(|s| !s.is_empty()) {
        return Some(q.to_owned());
    }
    let goal = goal.trim();
    if goal.is_empty() {
        None
    } else {
        Some(goal.to_owned())
    }
}

pub(crate) async fn gather(
    d: &Deps<'_>,
    task: &Task,
    focus: Option<&str>,
    budget: u32,
) -> Result<Gathered> {
    let query = semantic_query(focus, &task.goal);
    let limit = score::candidate_limit(budget);
    let sem = d.semantic.clone();
    let repo = task.repo.clone();
    let query_for_search = query.clone();
    // Started here, not after the index reads, so a slow index does not add another 150 ms.
    let search = async move {
        match (sem, query_for_search) {
            (Some(sem), Some(q)) => Some(sem.search(&repo, &q, limit).await),
            _ => None,
        }
    };
    let terms: Vec<String> = query
        .as_deref()
        .map(lexical::goal_terms)
        .unwrap_or_default()
        .into_iter()
        .take(lexical::MAX_TERMS)
        .collect();
    let (search_out, lexical_out, structural) = tokio::join!(
        tokio::time::timeout(score::SEMANTIC_TIMEOUT, search),
        d.index
            .search_code_lexical(&task.repo, &task.slug, &terms, limit),
        gather_structural(d, task, focus),
    );
    let mut parts = structural?;
    apply_lexical(d, task, lexical_out, limit, &mut parts).await?;
    apply_semantic(d, task, query.as_deref(), search_out, limit, &mut parts).await?;
    if parts.seed_set.is_empty() {
        expand_best_hits(d, task, limit, &mut parts).await?;
    }
    Ok(parts.gathered)
}

/// A cold start has no seeds, so nothing comes in through the reference graph, and an agent
/// goes looking for the types and functions the code it will change uses. The best production
/// functions and methods among the code hits stand in for seeds: their direct neighbours join as
/// two hops away (one hop from a guess), which ranks them above weak text matches and below the
/// hits themselves. They are context for a guess, so at most a signature each, and at most half
/// the candidate limit: their bodies would crowd out the hits' own (E1).
async fn expand_best_hits(
    d: &Deps<'_>,
    task: &Task,
    limit: usize,
    parts: &mut Structural,
) -> Result<()> {
    let mut hits: Vec<(&Item, SymbolId)> = parts
        .gathered
        .items
        .iter()
        .filter(|i| i.section == Section::Code && parts.code_factor.get(&i.id) == Some(&1.0))
        .filter_map(|i| Some((i, SymbolId(i.id.strip_prefix("sym:")?.parse().ok()?))))
        .collect();
    hits.sort_by(|(a, _), (b, _)| b.score.total_cmp(&a.score).then_with(|| a.id.cmp(&b.id)));
    // A type's neighbours are everything that uses it, so only functions and methods stand in.
    let best: Vec<SymbolId> = hits.iter().take(limit).map(|(_, id)| *id).collect();
    let kinds: HashMap<SymbolId, scholion_index::SymbolKind> = d
        .index
        .get_symbols(&best)
        .await?
        .into_iter()
        .map(|s| (s.id, s.symbol.kind))
        .collect();
    let seeds: Vec<SymbolId> = best
        .into_iter()
        .filter(|id| {
            kinds
                .get(id)
                .is_some_and(|k| *k != scholion_index::SymbolKind::Type)
        })
        .take(HIT_SEEDS)
        .collect();
    if seeds.is_empty() {
        return Ok(());
    }
    let mut near = d.index.neighbors(&task.repo, &task.slug, &seeds, 1).await?;
    near.retain(|(id, _)| {
        let key = format!("sym:{}", id.0);
        !parts.gathered.items.iter().any(|i| i.id == key)
    });
    near.sort_by_key(|(id, _)| id.0);
    near.truncate(limit / 2);
    let ids: Vec<SymbolId> = near.iter().map(|(id, _)| *id).collect();
    let syms = d.index.get_symbols(&ids).await?;
    let mut pool = parts.located.clone();
    pool.extend(syms.iter().cloned());
    for sym in &syms {
        let hide = listed_type_hides_body(sym, &pool);
        let src = source_at_head(
            d,
            task,
            parts.has_head,
            &mut parts.contents,
            &sym.symbol.path,
        )
        .await;
        let mut it = code_item(sym, 2, 0.0, src.as_deref(), hide).because("hit-neighbor");
        it.tiers[Tier::Full as usize] = it.tiers[Tier::Summary as usize].clone();
        let factor = if score::is_test_path(&sym.symbol.path) {
            score::TEST_CODE_FACTOR
        } else {
            1.0
        };
        parts.code_factor.insert(it.id.clone(), factor);
        parts.gathered.items.push(it);
    }
    Ok(())
}

/// Full-text code hits (Plan 6). The best hit counts as similarity `score::LEXICAL_SIM`, the
/// others in proportion; a hit on an item already in the pack raises its score instead.
async fn apply_lexical(
    d: &Deps<'_>,
    task: &Task,
    hits: Result<Vec<(SymbolId, f32)>>,
    limit: usize,
    parts: &mut Structural,
) -> Result<()> {
    let hits = match hits {
        Ok(h) => h,
        Err(e) => {
            parts.gathered.degraded.push(format!("lexical search: {e}"));
            return Ok(());
        }
    };
    let Some(top) = hits.first().map(|(_, s)| *s).filter(|s| *s > 0.0) else {
        return Ok(());
    };
    let sim = |s: f32| score::LEXICAL_SIM * s / top;
    let mut fresh: Vec<(SymbolId, f32)> = vec![];
    for (id, s) in &hits {
        let key = format!("sym:{}", id.0);
        if let Some(item) = parts.gathered.items.iter_mut().find(|i| i.id == key) {
            let factor = parts.code_factor.get(&key).copied().unwrap_or(1.0);
            item.score += score::W_SEM * sim(*s) * factor;
        } else if fresh.len() < limit {
            fresh.push((*id, *s));
        }
    }
    if fresh.is_empty() {
        return Ok(());
    }
    let ids: Vec<SymbolId> = fresh.iter().map(|(id, _)| *id).collect();
    let syms = d.index.get_symbols(&ids).await?;
    let by_id: HashMap<SymbolId, IndexedSymbol> = syms.into_iter().map(|s| (s.id, s)).collect();
    let mut pool = parts.located.clone();
    pool.extend(fresh.iter().filter_map(|(id, _)| by_id.get(id).cloned()));
    for (id, s) in &fresh {
        let Some(sym) = by_id.get(id) else {
            continue;
        };
        let hide = listed_type_hides_body(sym, &pool);
        let src = source_at_head(
            d,
            task,
            parts.has_head,
            &mut parts.contents,
            &sym.symbol.path,
        )
        .await;
        let factor = if score::is_test_path(&sym.symbol.path) {
            score::TEST_CODE_FACTOR
        } else {
            1.0
        };
        parts.code_factor.insert(format!("sym:{}", id.0), factor);
        parts.gathered.items.push(
            code_item(sym, score::FAR_HOPS, sim(*s), src.as_deref(), hide).because("lexical"),
        );
    }
    Ok(())
}

async fn gather_structural(d: &Deps<'_>, task: &Task, focus: Option<&str>) -> Result<Structural> {
    let (repo, scope) = (task.repo.as_str(), task.slug.as_str());
    let mut degraded = vec![];
    let mut items = vec![];
    let mut code_factor: HashMap<String, f32> = HashMap::new();
    let has_base = !task.base_change.0.is_empty();
    let has_head = !task.head_change.0.is_empty();

    let changed: Vec<String> = if has_base && has_head {
        match d
            .vcs
            .changed_paths(repo, &task.base_change, &task.head_change)
            .await
        {
            Ok(p) => p,
            Err(e) => {
                degraded.push(format!("changes unavailable: {e}"));
                vec![]
            }
        }
    } else {
        vec![]
    };

    // ---- seeds: symbols whose lines changed (all symbols of a new file), symbols the goal names
    // outright (Plan 6), plus `focus` matches ----
    let mut seeds = changed_symbols(d, task, &changed).await?;
    for ident in lexical::named_identifiers(&task.goal) {
        let named = d
            .index
            .search_symbols(repo, scope, &ident, FOCUS_LIMIT)
            .await?
            .into_iter()
            .filter(|s| lexical::names_symbol(&ident, &s.symbol.qname))
            .take(NAMED_PER_IDENT);
        for s in named {
            if !seeds.iter().any(|x| x.id == s.id) {
                seeds.push(s);
            }
        }
    }
    if let Some(q) = focus {
        for s in d.index.search_symbols(repo, scope, q, FOCUS_LIMIT).await? {
            if !seeds.iter().any(|x| x.id == s.id) {
                seeds.push(s);
            }
        }
    }
    let seed_ids: Vec<SymbolId> = seeds.iter().map(|s| s.id).collect();
    let seed_set: HashSet<SymbolId> = seed_ids.iter().copied().collect();

    let mut neighbor_hops = d.index.neighbors(repo, scope, &seed_ids, 2).await?;
    neighbor_hops.sort_by_key(|(id, h)| (*h, id.0));
    neighbor_hops.truncate(MAX_NEIGHBORS);
    let neighbor_ids: Vec<SymbolId> = neighbor_hops.iter().map(|(id, _)| *id).collect();
    let neighbor_syms = d.index.get_symbols(&neighbor_ids).await?;

    let mut located: Vec<Located> = seeds
        .iter()
        .cloned()
        .map(|sym| Located { sym, hops: 0 })
        .collect();
    for sym in neighbor_syms {
        let hops = neighbor_hops
            .iter()
            .find(|(id, _)| *id == sym.id)
            .map(|(_, h)| *h)
            .unwrap_or(2);
        located.push(Located { sym, hops });
    }
    let hops_by_sym: HashMap<SymbolId, u8> = located.iter().map(|l| (l.sym.id, l.hops)).collect();
    let seed_paths: BTreeSet<String> = changed
        .iter()
        .cloned()
        .chain(seeds.iter().map(|s| s.symbol.path.clone()))
        .collect();
    let neighbor_paths: BTreeSet<String> = located
        .iter()
        .filter(|l| l.hops > 0)
        .map(|l| l.sym.symbol.path.clone())
        .collect();
    let our_paths: Vec<String> = seed_paths.iter().cloned().collect();

    // ---- file contents at head (for code excerpts) ----
    let wanted: BTreeSet<String> = located.iter().map(|l| l.sym.symbol.path.clone()).collect();
    let mut contents: HashMap<String, String> = HashMap::new();
    if has_head {
        let reads = join_all(wanted.iter().map(|p| async move {
            (p.clone(), d.vcs.read_file(repo, &task.head_change, p).await)
        }))
        .await;
        let mut unreadable = 0;
        for (path, res) in reads {
            match res {
                Ok(bytes) => {
                    contents.insert(path, String::from_utf8_lossy(&bytes).into_owned());
                }
                Err(_) if unreadable < MAX_UNREADABLE_NOTED => {
                    unreadable += 1;
                    degraded.push(format!("unreadable at head: {path}"));
                }
                Err(_) => {}
            }
        }
    }

    // ---- Task section ----
    let changed_listed: Vec<&str> = changed
        .iter()
        .take(MAX_CHANGED_LISTED)
        .map(String::as_str)
        .collect();
    let handle = format!(
        "# {} · {} · {}",
        task.title,
        task.slug,
        task.status.as_str()
    );
    let summary = format!("{handle}\nGoal: {}", task.goal);
    let full = format!(
        "{summary}\nWorkspace: {}\nChanged ({}): {}",
        task.workspace_path.display(),
        changed.len(),
        if changed_listed.is_empty() {
            "nothing yet".to_owned()
        } else {
            changed_listed.join(", ")
        }
    );
    items.push(
        item(
            format!("task:{}", task.slug),
            Section::Task,
            1.0,
            [handle, summary, full],
        )
        .because("task"),
    );

    // ---- Code section ----
    for l in &located {
        let s = &l.sym;
        let handle = symbol_handle(s);
        let summary = format!("{handle}\n  {}", s.symbol.signature);
        // A type whose members are listed too shows only its signature: their bodies are the same text.
        let member_prefix = format!("{}.", s.symbol.qname);
        let has_listed_members = s.symbol.kind == scholion_index::SymbolKind::Type
            && located.iter().any(|o| {
                o.sym.symbol.path == s.symbol.path && o.sym.symbol.qname.starts_with(&member_prefix)
            });
        let full = match contents.get(&s.symbol.path).filter(|_| !has_listed_members) {
            Some(src) => format!(
                "{handle}\n{}",
                code_block(
                    &s.symbol.path,
                    &excerpt(src, s.symbol.start_line, s.symbol.end_line, MAX_CODE_LINES)
                )
            ),
            None => summary.clone(),
        };
        let factor = if score::is_test_path(&s.symbol.path) {
            score::TEST_CODE_FACTOR
        } else {
            1.0
        };
        let id = format!("sym:{}", s.id.0);
        code_factor.insert(id.clone(), factor);
        let reason = match l.hops {
            0 => "seed",
            1 => "hop1",
            2 => "hop2",
            _ => "far",
        };
        items.push(
            item(
                id,
                Section::Code,
                score::code_score(l.hops, 0.0) * factor,
                [handle, summary, full],
            )
            .because(reason)
            .at(&s.symbol.path, s.symbol.start_line, s.symbol.end_line),
        );
    }

    // ---- History / Task records ----
    let qnames: Vec<String> = located
        .iter()
        .filter(|l| l.hops <= 1)
        .map(|l| l.sym.symbol.qname.clone())
        .collect();
    let own = d.store.task_records(task.id).await?;
    let anchored = d
        .store
        .records_anchored_to(repo, &our_paths, &qnames)
        .await?;
    let mut records: BTreeMap<String, Record> = BTreeMap::new();
    for r in own.into_iter().chain(anchored) {
        if r.superseded_by.is_none() {
            records.entry(r.id.to_string()).or_insert(r);
        }
    }
    let anchors = AnchorCtx {
        index: d.index,
        vcs: d.vcs,
        repo,
        scope,
        head: has_head.then_some(&task.head_change),
        seed_set: &seed_set,
        hops_by_sym: &hops_by_sym,
        seed_paths: &seed_paths,
        neighbor_paths: &neighbor_paths,
    };
    // A merge into main that did not apply: its paths, and what the other side recorded about them.
    let conflicts: BTreeSet<String> = if task.status == TaskStatus::Conflicted {
        d.store
            .merge_state(task.id)
            .await?
            .conflicts
            .into_iter()
            .collect()
    } else {
        BTreeSet::new()
    };
    // The owner works through review findings first: pinned at full tier until resubmitted.
    let owner_at_work = matches!(task.status, TaskStatus::Open | TaskStatus::Conflicted);
    let now = Utc::now();
    for rec in records.values() {
        if rec.kind == RecordKind::Intent && rec.task_id == task.id && repeats_goal(rec, task) {
            continue;
        }
        let (hops, stale, on_seed, changed) = anchors.standing(rec).await?;
        let (id, texts) = record_texts(rec, stale, changed);
        let factor = record_factor(rec, task, changed);
        if rec.kind == RecordKind::Intent && rec.task_id == task.id {
            items.push(item(id, Section::Task, 0.9, texts).because("task"));
        } else if rec.kind == RecordKind::Review && rec.task_id == task.id && owner_at_work {
            let full = texts[Tier::Full as usize].clone();
            items.push(
                item(
                    id,
                    Section::History,
                    PINNED_REVIEW_SCORE,
                    [full.clone(), full.clone(), full],
                )
                .because("pinned-review")
                .required_if(true),
            );
        } else if rec.task_id != task.id && rec.anchors.iter().any(|a| conflicts.contains(&a.path))
        {
            items.push(item(id, Section::Conflicts, 1.5, texts).because("conflict-record"));
        } else {
            let age_days = (now - rec.created_at).num_seconds() as f64 / 86_400.0;
            let reason = if rec.task_id == task.id {
                "own-record"
            } else {
                "anchored"
            };
            items.push(
                item(
                    id,
                    Section::History,
                    score::record_score(rec.kind, hops, age_days, on_seed, 0.0) * factor,
                    texts,
                )
                .because(reason),
            );
        }
    }

    // ---- Conflicts ----
    for path in &conflicts {
        let handle = format!("- ⚠ {path} conflicts with main");
        let full = format!(
            "{handle}: main changed it after this task started. Run `av sync`, resolve the \
             conflict markers, then `av submit`."
        );
        items.push(
            item(
                format!("conflict:{path}"),
                Section::Conflicts,
                3.0,
                [handle, full.clone(), full],
            )
            .because("conflict")
            .required_if(true),
        );
    }
    for s in seeds.iter().filter(|s| conflicts.contains(&s.symbol.path)) {
        let h = format!("{} (conflicting)", symbol_handle(s));
        items.push(
            item(
                format!("conflict-sym:{}", s.id.0),
                Section::Conflicts,
                2.0,
                [h.clone(), h.clone(), h],
            )
            .because("conflict"),
        );
    }
    if !conflicts.is_empty() {
        let paths: Vec<String> = conflicts.iter().cloned().collect();
        for rec in d.store.records_anchored_to(repo, &paths, &[]).await? {
            if rec.task_id == task.id || records.contains_key(&rec.id.to_string()) {
                continue;
            }
            let (id, texts) = record_texts(&rec, false, false);
            items.push(item(id, Section::Conflicts, 1.5, texts).because("conflict-record"));
        }
    }

    // ---- Conventions ----
    let mut conventions = d.store.conventions_for(repo, &our_paths).await?;
    if our_paths.is_empty() {
        // Nothing changed yet, so nothing scopes the rules: hard constraints that apply anywhere in
        // the repo are shown up front, while the agent is still planning (Plan 2b).
        let change = if has_head {
            &task.head_change
        } else {
            &task.base_change
        };
        if !change.0.is_empty()
            && let Ok(files) = d.vcs.list_files(repo, change).await
        {
            conventions.extend(
                d.store
                    .conventions_for(repo, &files)
                    .await?
                    .into_iter()
                    .filter(|c| c.required),
            );
        }
    }
    for c in conventions {
        let handle = format!("- conv:{} {}", c.id.0, clip(&c.text, 70));
        let full = format!("- conv:{} ({}) {}", c.id.0, c.scope_glob, c.text);
        items.push(
            item(
                format!("conv:{}", c.id.0),
                Section::ConventionsLive,
                score::convention_score(0.0),
                [handle, full.clone(), full],
            )
            .because("convention")
            .required_if(c.required),
        );
    }

    // ---- Live work ----
    if !our_paths.is_empty() {
        let ours: BTreeSet<&String> = our_paths.iter().collect();
        for other in d
            .store
            .live_tasks_touching(repo, &our_paths, task.id)
            .await?
        {
            let touched = d.store.touched_paths(other.id).await?;
            let overlap: Vec<&str> = touched
                .iter()
                .filter(|p| ours.contains(p))
                .take(5)
                .map(String::as_str)
                .collect();
            if overlap.is_empty() {
                continue;
            }
            // a task past `open` is closer to main, so say where it is
            let status = match other.status {
                TaskStatus::Open => String::new(),
                s => format!(" ({})", s.as_str().replace('_', " ")),
            };
            let handle = format!(
                "- ⚠ live:{}{status} also touches {}",
                other.slug,
                overlap.join(", ")
            );
            let full = format!(
                "{handle} (owner {}; \"{}\")",
                other.owner_agent,
                clip(&other.title, 60)
            );
            items.push(
                item(
                    format!("live:{}", other.slug),
                    Section::ConventionsLive,
                    0.9,
                    [handle.clone(), handle, full],
                )
                .because("live"),
            );
        }
    }

    // ---- Map: skeletons of seed / neighbor files, plus the repo layout ----
    let map_paths: BTreeSet<&String> = seed_paths.iter().chain(neighbor_paths.iter()).collect();
    let skeletons = join_all(
        map_paths
            .iter()
            .map(|p| async move { (p.to_string(), d.index.symbols_in(repo, scope, p).await) }),
    )
    .await;
    let mut skeleton_paths: BTreeSet<String> = BTreeSet::new();
    for (path, res) in skeletons {
        let syms = res?;
        if syms.is_empty() {
            continue;
        }
        skeleton_paths.insert(path.clone());
        let best = syms
            .iter()
            .map(|s| hops_by_sym.get(&s.id).copied().unwrap_or(FAR_HOPS))
            .min()
            .unwrap_or(FAR_HOPS);
        let mut summary = format!("- {path}");
        for s in syms.iter().take(MAX_SKELETON_LINES) {
            summary.push_str(&format!(
                "\n  L{}: {}",
                s.symbol.start_line, s.symbol.signature
            ));
        }
        items.push(
            item(
                format!("file:{path}"),
                Section::Map,
                score::code_score(best, 0.0),
                [format!("- {path}"), summary.clone(), summary],
            )
            .because("map"),
        );
    }
    let tree_change = if has_head {
        Some(&task.head_change)
    } else if has_base {
        Some(&task.base_change)
    } else {
        None
    };
    if let Some(change) = tree_change {
        match d.vcs.list_files(repo, change).await {
            Ok(files) => items.extend(repo_tree_item(&files, &skeleton_paths)),
            Err(e) => degraded.push(format!("repo layout unavailable: {e}")),
        }
    }

    Ok(Structural {
        gathered: Gathered { items, degraded },
        hops_by_sym,
        seed_set,
        seed_paths,
        neighbor_paths,
        contents,
        code_factor,
        located: located.into_iter().map(|l| l.sym).collect(),
        has_head,
    })
}

fn hit_id(hit: &SemanticHit) -> String {
    match hit.kind {
        SemanticKind::Record => hit
            .record_id
            .map(|id| short_id("rec", &id.0))
            .unwrap_or_default(),
        SemanticKind::CodeChunk => hit
            .symbol_id
            .map(|id| format!("sym:{}", id.0))
            .unwrap_or_default(),
        SemanticKind::Convention => hit
            .convention_id
            .map(|id| format!("conv:{}", id.0))
            .unwrap_or_default(),
    }
}

async fn apply_semantic(
    d: &Deps<'_>,
    task: &Task,
    query: Option<&str>,
    search_out: std::result::Result<Option<Result<Vec<SemanticHit>>>, tokio::time::error::Elapsed>,
    limit: usize,
    parts: &mut Structural,
) -> Result<()> {
    match search_out {
        Ok(None) => Ok(()),
        Ok(Some(Ok(hits))) => merge_hits(d, task, hits, limit, parts).await,
        Ok(Some(Err(_))) | Err(_) => {
            parts.gathered.degraded.push("semantic".to_owned());
            if let Some(q) = query
                && let Ok(recs) = d
                    .store
                    .search_records_fts(&task.repo, q, score::SEMANTIC_LIMIT)
                    .await
            {
                for rec in recs {
                    include_record(d, task, parts, &rec, 0.0, "fts").await?;
                }
            }
            Ok(())
        }
    }
}

async fn merge_hits(
    d: &Deps<'_>,
    task: &Task,
    mut hits: Vec<SemanticHit>,
    limit: usize,
    parts: &mut Structural,
) -> Result<()> {
    hits.retain(|h| h.repo == task.repo);
    let mut hits = to_task_scope(d, task, hits).await?;
    hits.sort_by(|a, b| {
        b.score
            .total_cmp(&a.score)
            .then_with(|| hit_id(a).cmp(&hit_id(b)))
    });
    let mut seen = HashSet::new();
    hits.retain(|h| {
        let id = hit_id(h);
        !id.is_empty() && seen.insert(id)
    });

    let mut candidates: Vec<(SymbolId, f32)> = Vec::new();
    for hit in &hits {
        if hit.kind != SemanticKind::CodeChunk || hit.superseded {
            continue;
        }
        // Another task's unmerged chunk is not context. `main` has a null task id.
        if matches!(hit.task_id, Some(id) if id != task.id) {
            continue;
        }
        let Some(sym_id) = hit.symbol_id else {
            continue;
        };
        let id = format!("sym:{}", sym_id.0);
        if let Some(item) = parts.gathered.items.iter_mut().find(|i| i.id == id) {
            let factor = parts.code_factor.get(&id).copied().unwrap_or(1.0);
            item.score += score::W_SEM * score::clamp_sim(hit.score) * factor;
            continue;
        }
        candidates.push((sym_id, hit.score));
    }
    if !candidates.is_empty() {
        let ids: Vec<SymbolId> = candidates.iter().map(|(id, _)| *id).collect();
        let syms = d.index.get_symbols(&ids).await?;
        let by_id: HashMap<SymbolId, IndexedSymbol> = syms.into_iter().map(|s| (s.id, s)).collect();
        let mut added = Vec::new();
        for (id, _) in &candidates {
            if added.len() >= limit {
                break;
            }
            if let Some(sym) = by_id.get(id) {
                added.push(sym.clone());
            }
        }
        let mut pool = parts.located.clone();
        pool.extend(added.iter().cloned());
        for sym in &added {
            let hide = listed_type_hides_body(sym, &pool);
            let sim = candidates
                .iter()
                .find(|(id, _)| *id == sym.id)
                .map(|(_, s)| *s)
                .unwrap_or(0.0);
            let src = source_at_head(
                d,
                task,
                parts.has_head,
                &mut parts.contents,
                &sym.symbol.path,
            )
            .await;
            parts
                .gathered
                .items
                .push(code_item(sym, score::FAR_HOPS, sim, src.as_deref(), hide));
        }
    }

    for hit in &hits {
        if hit.superseded {
            continue;
        }
        match hit.kind {
            SemanticKind::Record => {
                let Some(rid) = hit.record_id else {
                    continue;
                };
                let rec = match d.store.get_record(rid).await {
                    Ok(r) => r,
                    Err(AvError::NotFound(_)) => continue,
                    Err(e) => return Err(e),
                };
                include_record(d, task, parts, &rec, hit.score, "semantic").await?;
            }
            SemanticKind::Convention => {
                let Some(cid) = hit.convention_id else {
                    continue;
                };
                let id = format!("conv:{}", cid.0);
                // Missing means the glob did not match the task paths (Plan 1 scope rule).
                if let Some(item) = parts.gathered.items.iter_mut().find(|i| i.id == id) {
                    item.score = score::convention_score(hit.score);
                }
            }
            SemanticKind::CodeChunk => {}
        }
    }
    Ok(())
}

/// Code hits from `main` carry `main`'s symbol ids, but a task's pack works in the task's own scope.
/// Map each such hit to the task-scope symbol with the same path and qname, so one function never
/// appears twice under two ids; drop the hit when the task no longer has that symbol.
async fn to_task_scope(
    d: &Deps<'_>,
    task: &Task,
    hits: Vec<SemanticHit>,
) -> Result<Vec<SemanticHit>> {
    let is_main_code = |h: &SemanticHit| {
        h.kind == SemanticKind::CodeChunk && h.task_id.is_none() && h.symbol_id.is_some()
    };
    if task.slug == "main" || !hits.iter().any(is_main_code) {
        return Ok(hits);
    }
    let ids: Vec<SymbolId> = hits
        .iter()
        .filter(|h| is_main_code(h))
        .filter_map(|h| h.symbol_id)
        .collect();
    let main_syms: HashMap<SymbolId, IndexedSymbol> = d
        .index
        .get_symbols(&ids)
        .await?
        .into_iter()
        .map(|s| (s.id, s))
        .collect();
    let mut by_path: HashMap<String, HashMap<String, SymbolId>> = HashMap::new();
    let mut out = Vec::with_capacity(hits.len());
    for mut hit in hits {
        if !is_main_code(&hit) {
            out.push(hit);
            continue;
        }
        let Some(main) = hit.symbol_id.and_then(|id| main_syms.get(&id)) else {
            continue;
        };
        let path = &main.symbol.path;
        if !by_path.contains_key(path) {
            let syms = d.index.symbols_in(&task.repo, &task.slug, path).await?;
            by_path.insert(
                path.clone(),
                syms.into_iter().map(|s| (s.symbol.qname, s.id)).collect(),
            );
        }
        if let Some(id) = by_path[path].get(&main.symbol.qname) {
            hit.symbol_id = Some(*id);
            out.push(hit);
        }
    }
    Ok(out)
}

async fn include_record(
    d: &Deps<'_>,
    task: &Task,
    parts: &mut Structural,
    rec: &Record,
    semantic: f32,
    reason: &'static str,
) -> Result<()> {
    if rec.superseded_by.is_some() {
        return Ok(());
    }
    if rec.kind == RecordKind::Intent && rec.task_id == task.id && repeats_goal(rec, task) {
        return Ok(());
    }
    let id = short_id("rec", &rec.id.0);
    if let Some(item) = parts.gathered.items.iter_mut().find(|i| i.id == id) {
        item.score += score::W_SEM * score::clamp_sim(semantic) * record_factor(rec, task, false);
        return Ok(());
    }
    let (hops, stale, on_seed, changed) = if rec.anchors.is_empty() {
        (score::FAR_HOPS, false, false, false)
    } else {
        AnchorCtx {
            index: d.index,
            vcs: d.vcs,
            repo: &task.repo,
            scope: &task.slug,
            head: parts.has_head.then_some(&task.head_change),
            seed_set: &parts.seed_set,
            hops_by_sym: &parts.hops_by_sym,
            seed_paths: &parts.seed_paths,
            neighbor_paths: &parts.neighbor_paths,
        }
        .standing(rec)
        .await?
    };
    let (id, texts) = record_texts(rec, stale, changed);
    let factor = record_factor(rec, task, changed);
    if rec.kind == RecordKind::Intent && rec.task_id == task.id {
        parts.gathered.items.push(
            item(
                id,
                Section::Task,
                0.9 + score::W_SEM * score::clamp_sim(semantic),
                texts,
            )
            .because(reason),
        );
    } else {
        let age_days = (Utc::now() - rec.created_at).num_seconds() as f64 / 86_400.0;
        parts.gathered.items.push(
            item(
                id,
                Section::History,
                score::record_score(rec.kind, hops, age_days, on_seed, semantic) * factor,
                texts,
            )
            .because(reason),
        );
    }
    Ok(())
}

fn listed_type_hides_body(s: &IndexedSymbol, others: &[IndexedSymbol]) -> bool {
    if s.symbol.kind != scholion_index::SymbolKind::Type {
        return false;
    }
    let member_prefix = format!("{}.", s.symbol.qname);
    others
        .iter()
        .any(|o| o.symbol.path == s.symbol.path && o.symbol.qname.starts_with(&member_prefix))
}

fn code_item(
    s: &IndexedSymbol,
    hops: u8,
    semantic: f32,
    src: Option<&str>,
    hide_body: bool,
) -> Item {
    let handle = symbol_handle(s);
    let summary = format!("{handle}\n  {}", s.symbol.signature);
    let full = match src.filter(|_| !hide_body) {
        Some(src) => format!(
            "{handle}\n{}",
            code_block(
                &s.symbol.path,
                &excerpt(src, s.symbol.start_line, s.symbol.end_line, MAX_CODE_LINES)
            )
        ),
        None => summary.clone(),
    };
    let factor = if score::is_test_path(&s.symbol.path) {
        score::TEST_CODE_FACTOR
    } else {
        1.0
    };
    item(
        format!("sym:{}", s.id.0),
        Section::Code,
        score::code_score(hops, semantic) * factor,
        [handle, summary, full],
    )
    .because("semantic")
    .at(&s.symbol.path, s.symbol.start_line, s.symbol.end_line)
}

/// Excerpt bytes come only from this task's head. A missing path keeps the signature.
async fn source_at_head(
    d: &Deps<'_>,
    task: &Task,
    has_head: bool,
    contents: &mut HashMap<String, String>,
    path: &str,
) -> Option<String> {
    if let Some(existing) = contents.get(path) {
        return Some(existing.clone());
    }
    if !has_head {
        return None;
    }
    match d.vcs.read_file(&task.repo, &task.head_change, path).await {
        Ok(bytes) => {
            let text = String::from_utf8_lossy(&bytes).into_owned();
            contents.insert(path.to_owned(), text.clone());
            Some(text)
        }
        Err(_) => None,
    }
}

struct AnchorCtx<'a> {
    index: &'a dyn CodeIndex,
    vcs: &'a dyn Vcs,
    repo: &'a str,
    scope: &'a str,
    head: Option<&'a ChangeId>,
    seed_set: &'a HashSet<SymbolId>,
    hops_by_sym: &'a HashMap<SymbolId, u8>,
    seed_paths: &'a BTreeSet<String>,
    neighbor_paths: &'a BTreeSet<String>,
}

impl AnchorCtx<'_> {
    /// Does `path` exist at the task's head? The index only knows code files, so for anything it does not
    /// know, ask the VCS before calling an anchor stale.
    async fn exists_at_head(&self, path: &str) -> bool {
        match self.head {
            Some(head) => self.vcs.read_file(self.repo, head, path).await.is_ok(),
            None => false,
        }
    }

    /// (hops from the seeds, any anchor stale, anchored directly on a seed symbol, any anchored code
    /// changed since the record was written)
    async fn standing(&self, rec: &Record) -> Result<(u8, bool, bool, bool)> {
        if rec.anchors.is_empty() {
            return Ok((1, false, false, false));
        }
        let (mut best, mut stale, mut on_seed, mut changed) = (FAR_HOPS, false, false, false);
        for (i, a) in rec.anchors.iter().enumerate() {
            if let (Some(Some(then)), Some(head)) = (rec.anchor_hashes.get(i), self.head)
                && let Some(now) =
                    anchor_hash(self.vcs, self.index, self.repo, self.scope, head, a).await
            {
                changed |= now != *then;
            }
            let path_only = |path: &str| {
                if self.seed_paths.contains(path) {
                    1
                } else if self.neighbor_paths.contains(path) {
                    2
                } else {
                    FAR_HOPS
                }
            };
            match self.index.resolve(self.repo, self.scope, a).await? {
                AnchorResolution::Symbol(id) => {
                    on_seed |= self.seed_set.contains(&id);
                    best = best.min(self.hops_by_sym.get(&id).copied().unwrap_or(FAR_HOPS));
                }
                AnchorResolution::PathOnly(path) => {
                    stale |= a.symbol.is_some();
                    best = best.min(path_only(&path));
                }
                AnchorResolution::Stale if self.exists_at_head(&a.path).await => {
                    // a real file the index does not cover (README, Dockerfile, docs...)
                    stale |= a.symbol.is_some();
                    best = best.min(path_only(&a.path));
                }
                AnchorResolution::Stale => stale = true,
            }
        }
        Ok((best, stale, on_seed, changed))
    }
}

/// SHA-256 (hex) of what `a` points at in `change`: the anchored symbol's source lines, or the whole
/// file for a path anchor. `None` when the file or symbol does not exist there.
pub(crate) async fn anchor_hash(
    vcs: &dyn Vcs,
    index: &dyn CodeIndex,
    repo: &str,
    scope: &str,
    change: &ChangeId,
    a: &Anchor,
) -> Option<String> {
    use sha2::{Digest, Sha256};
    if change.0.is_empty() {
        return None;
    }
    let bytes = vcs.read_file(repo, change, &a.path).await.ok()?;
    let hashed = match &a.symbol {
        None => bytes,
        Some(_) => {
            let AnchorResolution::Symbol(id) = index.resolve(repo, scope, a).await.ok()? else {
                return None;
            };
            let sym = index.get_symbols(&[id]).await.ok()?.into_iter().next()?;
            let (start, end) = (sym.symbol.start_line as usize, sym.symbol.end_line as usize);
            String::from_utf8_lossy(&bytes)
                .lines()
                .skip(start.saturating_sub(1))
                .take(end.saturating_sub(start) + 1)
                .collect::<Vec<_>>()
                .join("\n")
                .into_bytes()
        }
    };
    Some(format!("{:x}", Sha256::digest(&hashed)))
}

/// Multiplier on a History record's score: changed anchors and other tasks' intents rank lower.
fn record_factor(rec: &Record, task: &Task, changed: bool) -> f32 {
    let mut f = if changed { score::CHANGED_FACTOR } else { 1.0 };
    if rec.kind == RecordKind::Intent && rec.task_id != task.id {
        f *= score::FOREIGN_INTENT_FACTOR;
    }
    f
}

/// The repo layout: top-level entries with file counts (summary), and under each top-level
/// directory its direct children (full). Files already shown as skeletons are not repeated.
fn repo_tree_item(files: &[String], skeletons: &BTreeSet<String>) -> Option<Item> {
    if files.is_empty() {
        return None;
    }
    // top-level entry -> (is_dir, file count); and per top dir its direct children
    let mut top: BTreeMap<String, (bool, usize)> = BTreeMap::new();
    let mut children: BTreeMap<String, BTreeMap<String, (bool, usize)>> = BTreeMap::new();
    for f in files {
        let mut parts = f.splitn(2, '/');
        let first = parts.next().unwrap_or_default();
        match parts.next() {
            None => {
                top.entry(first.to_owned()).or_insert((false, 0)).1 += 1;
            }
            Some(rest) => {
                let e = top.entry(format!("{first}/")).or_insert((true, 0));
                e.1 += 1;
                let mut sub = rest.splitn(2, '/');
                let name = sub.next().unwrap_or_default();
                let is_dir = sub.next().is_some();
                if !is_dir && skeletons.contains(f) {
                    continue;
                }
                let key = if is_dir {
                    format!("{name}/")
                } else {
                    name.to_owned()
                };
                children
                    .entry(format!("{first}/"))
                    .or_default()
                    .entry(key)
                    .or_insert((is_dir, 0))
                    .1 += 1;
            }
        }
    }
    let names: Vec<&str> = top.keys().map(String::as_str).take(12).collect();
    let handle = format!("- repo map: {} ({} files)", names.join(" "), files.len());
    let mut summary = format!("- repo map ({} files):", files.len());
    for (name, (is_dir, n)) in &top {
        if *is_dir {
            summary.push_str(&format!("\n  {name} ({n} files)"));
        } else {
            summary.push_str(&format!("\n  {name}"));
        }
    }
    let mut full = format!("- repo map ({} files):", files.len());
    let mut lines = 0;
    'outer: for (name, (is_dir, n)) in &top {
        if *is_dir {
            full.push_str(&format!("\n  {name} ({n} files)"));
        } else {
            full.push_str(&format!("\n  {name}"));
        }
        for (child, (child_dir, m)) in children.get(name).into_iter().flatten() {
            if lines >= MAX_TREE_LINES {
                full.push_str("\n    …");
                break 'outer;
            }
            lines += 1;
            if *child_dir {
                full.push_str(&format!("\n    {child} ({m} files)"));
            } else {
                full.push_str(&format!("\n    {child}"));
            }
        }
    }
    Some(
        item(
            "map:tree".into(),
            Section::Map,
            0.3,
            [handle, summary, full],
        )
        .because("map"),
    )
}

#[cfg(test)]
mod tests {
    use super::line_ranges;

    #[test]
    fn insertions_replacements_and_deletions_map_to_head_lines() {
        assert_eq!(line_ranges("a\nb\nc\n", "a\nb\nc\n"), vec![]);
        assert_eq!(
            line_ranges("a\nb\nc\n", "a\nb\nc\nd\ne\n"),
            vec![(4, 5)],
            "appended lines 4-5"
        );
        assert_eq!(
            line_ranges("a\nb\nc\n", "a\nX\nc\n"),
            vec![(2, 2)],
            "replaced line 2"
        );
        assert_eq!(
            line_ranges("a\nb\nc\n", "a\nc\n"),
            vec![(1, 2)],
            "deleted line touches its neighbours"
        );
        assert_eq!(line_ranges("a\n", "a\nb\n"), vec![(2, 2)]);
    }
}
