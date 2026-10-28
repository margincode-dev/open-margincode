use std::sync::Arc;

use scholion_core::{
    AgentFamily, AvError, IntentStore, NewReceipt, PackMode, ReceiptItem, Result, Semantic, TaskId,
    Tier, Vcs,
};
use scholion_index::CodeIndex;
use serde::{Deserialize, Serialize};

use crate::assemble::{Deps, code_block, excerpt, gather, record_texts};
use crate::budget::{Section, allocate};
use crate::bundle::{Bundle, BundleInputs, BundleRequest, ReviewInputs};
use crate::ledger::{collapse_sent, delivered};
use crate::render::{fit, render, skeleton_text};
use crate::tokens::estimate_tokens;

/// Longest code excerpt returned by `expand`.
const MAX_EXPAND_LINES: usize = 400;

#[derive(Debug, Clone)]
pub struct PackRequest {
    pub task_id: TaskId,
    pub budget_tokens: u32,
    pub focus: Option<String>,
    /// The ledger to dedup against and extend. `None`: every item is sent as if for the first
    /// time and nothing is remembered (no shared default ledger between unrelated agents).
    pub session: Option<String>,
    pub family: AgentFamily,
    pub code: CodeDetail,
    /// Include the Map section (file skeletons and the repo layout). An agent that lists files
    /// itself, or finds them from the code items' paths, can do without it.
    pub map: bool,
    /// At most this many code items, the best scored. Every item a pack names is a file an agent
    /// may open, so a long list of weak candidates costs reads, not just pack tokens.
    pub max_code: Option<usize>,
}

/// How much of each code item a pack may show.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodeDetail {
    /// Up to the source excerpt.
    #[default]
    Full,
    /// Up to path, lines and signature. For agents that must read a file before editing it
    /// (Claude Code), a body in the pack is only sent a second time when they do.
    Signatures,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pack {
    pub markdown: String,
    /// Estimated tokens of `markdown`; never above the requested budget.
    pub tokens: u32,
    /// Handles of items that did not fit (at most 20).
    pub omitted: Vec<String>,
    /// Non-fatal problems (unreadable files, unavailable data).
    pub degraded: Vec<String>,
    /// `rcp:<8 hex>`: the stored manifest of this pack. `None` when it could not be written.
    #[serde(default)]
    pub receipt: Option<String>,
    /// Tokens this session has pulled through `expand` so far (Plan 2b: expand is not free).
    #[serde(default)]
    pub session_expand_tokens: u64,
}

/// One charged expansion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expanded {
    pub full: String,
    pub tokens: u32,
    pub session_expand_tokens: u64,
}

/// Bumped by any change to ranking or rendering; recorded in every receipt.
pub const PACKER_VERSION: u32 = 3;

pub struct Packer {
    vcs: Arc<dyn Vcs>,
    store: Arc<dyn IntentStore>,
    index: Arc<dyn CodeIndex>,
    semantic: Option<Arc<dyn Semantic>>,
}

impl Packer {
    pub fn new(vcs: Arc<dyn Vcs>, store: Arc<dyn IntentStore>, index: Arc<dyn CodeIndex>) -> Self {
        Self::with_semantic(vcs, store, index, None)
    }

    pub fn with_semantic(
        vcs: Arc<dyn Vcs>,
        store: Arc<dyn IntentStore>,
        index: Arc<dyn CodeIndex>,
        semantic: Option<Arc<dyn Semantic>>,
    ) -> Self {
        Self {
            vcs,
            store,
            index,
            semantic,
        }
    }

    fn deps(&self) -> Deps<'_> {
        Deps {
            vcs: self.vcs.as_ref(),
            store: self.store.as_ref(),
            index: self.index.as_ref(),
            semantic: self.semantic.clone(),
        }
    }

    pub async fn build(&self, req: PackRequest) -> Result<Pack> {
        let empty = |degraded| Pack {
            markdown: String::new(),
            tokens: 0,
            omitted: vec![],
            degraded,
            receipt: None,
            session_expand_tokens: 0,
        };
        if req.budget_tokens == 0 {
            return Ok(empty(vec![]));
        }
        let task = self.store.get_task(req.task_id).await?;
        let mut gathered =
            gather(&self.deps(), &task, req.focus.as_deref(), req.budget_tokens).await?;
        if task.pack_mode == PackMode::TaskOnly {
            gathered.items.retain(|i| i.section == Section::Task);
        }
        if !req.map {
            gathered.items.retain(|i| i.section != Section::Map);
        }
        if let Some(max) = req.max_code {
            cap_code(&mut gathered.items, max);
        }
        if req.code == CodeDetail::Signatures {
            for i in gathered
                .items
                .iter_mut()
                .filter(|i| i.section == Section::Code)
            {
                i.tiers[Tier::Full as usize] = i.tiers[Tier::Summary as usize].clone();
            }
        }
        if let Some(session) = &req.session {
            let sent = self.store.ledger_get(session).await?;
            collapse_sent(&mut gathered.items, &sent);
        }

        let skeleton = estimate_tokens(&skeleton_text(), req.family);
        let alloc = allocate(
            gathered.items,
            req.budget_tokens.saturating_sub(skeleton),
            req.family,
        );
        let mut chosen = alloc.chosen;
        let mut omitted_total = alloc.omitted_total;

        // The allocator budgets item text; headings and footer are budgeted via `skeleton`, and the
        // whole is re-measured here. If it is still over (tiny budgets), drop the lowest-ranked items.
        let (markdown, tokens) = fit(
            &mut chosen,
            &mut omitted_total,
            req.budget_tokens,
            req.family,
            |c, total, n| {
                render(
                    c,
                    &alloc.omitted,
                    total,
                    n,
                    req.budget_tokens,
                    req.map && task.pack_mode == PackMode::Full,
                )
            },
        );

        if let Some(session) = &req.session {
            self.store.ledger_put(session, &delivered(&chosen)).await?;
        }
        let mut pack = empty(gathered.degraded);
        let receipt = NewReceipt {
            task_id: task.id,
            session: req.session.clone().unwrap_or_default(),
            head_change: task.head_change.clone(),
            packer_version: PACKER_VERSION,
            budget: req.budget_tokens,
            tokens,
            items: chosen
                .iter()
                .map(|(item, tier)| ReceiptItem {
                    id: item.id.clone(),
                    tier: *tier,
                    score: item.score,
                    reason: item.reason.to_owned(),
                    source: item.source.clone(),
                    tokens: estimate_tokens(item.text(*tier), req.family),
                })
                .collect(),
            omitted: alloc.omitted.clone(),
        };
        // A pack without its receipt is still a good pack: report, do not fail.
        match self.store.add_receipt(receipt).await {
            Ok(id) => pack.receipt = Some(id),
            Err(_) => pack.degraded.push("receipt".to_owned()),
        }
        if let Some(session) = &req.session {
            pack.session_expand_tokens = self.store.expand_tokens(session).await.unwrap_or(0);
        }
        pack.markdown = markdown;
        pack.tokens = tokens;
        pack.omitted = alloc.omitted;
        Ok(pack)
    }

    /// The handoff bundle of a task: goal, the latest handoff state, records (decisions and dead
    /// ends first), changes (hunks while the budget allows) and open questions.
    pub async fn handoff_bundle(&self, req: BundleRequest) -> Result<Bundle> {
        self.bundle(req, false).await
    }

    /// The review (PR) bundle: the handoff bundle's content plus test status at the current head,
    /// risk areas and a checklist of the conventions in scope of the changed paths.
    pub async fn pr_bundle(&self, req: BundleRequest) -> Result<Bundle> {
        self.bundle(req, true).await
    }

    async fn bundle(&self, req: BundleRequest, review: bool) -> Result<Bundle> {
        let task = self.store.get_task(req.task_id).await?;
        let records = self.store.task_records(task.id).await?;
        let changed = if task.base_change.0.is_empty() || task.head_change.0.is_empty() {
            vec![]
        } else {
            self.vcs
                .changed_paths(&task.repo, &task.base_change, &task.head_change)
                .await?
        };
        let symbols = crate::assemble::changed_symbols(&self.deps(), &task, &changed).await?;
        let review = if review {
            let ids: Vec<scholion_core::SymbolId> = symbols.iter().map(|s| s.id).collect();
            Some(ReviewInputs {
                conventions: self.store.conventions_for(&task.repo, &changed).await?,
                in_degree: self.index.in_degree(&ids).await?,
            })
        } else {
            None
        };
        crate::bundle::build(
            self.vcs.as_ref(),
            BundleInputs {
                task: &task,
                records: &records,
                changed: &changed,
                symbols: &symbols,
                review,
            },
            &req,
        )
        .await
    }

    /// Symbols whose lines this task changed (base → head), in the task's own index scope.
    pub async fn changed_symbols(
        &self,
        task_id: TaskId,
    ) -> Result<Vec<scholion_index::IndexedSymbol>> {
        let task = self.store.get_task(task_id).await?;
        if task.base_change.0.is_empty() || task.head_change.0.is_empty() {
            return Ok(vec![]);
        }
        let changed = self
            .vcs
            .changed_paths(&task.repo, &task.base_change, &task.head_change)
            .await?;
        crate::assemble::changed_symbols(&self.deps(), &task, &changed).await
    }

    /// `expand`, charged to `session`. With `cap`, a call that would take the session's total above
    /// it is refused (and nothing is charged).
    pub async fn expand_charged(
        &self,
        task_id: TaskId,
        item_id: &str,
        session: &str,
        cap: Option<u64>,
    ) -> Result<Expanded> {
        let full = self.expand(task_id, item_id).await?;
        let tokens = estimate_tokens(&full, AgentFamily::default());
        let Some(session_expand_tokens) = self
            .store
            .charge_expand(session, u64::from(tokens), cap)
            .await?
        else {
            let so_far = self.store.expand_tokens(session).await?;
            return Err(AvError::Invalid(format!(
                "expand cap reached: this session used {so_far} of {} tokens and {item_id} \
                 needs {tokens}; start a fresh session (av context --fresh) or raise \
                 AV_EXPAND_CAP_TOKENS",
                cap.unwrap_or_default()
            )));
        };
        Ok(Expanded {
            full,
            tokens,
            session_expand_tokens,
        })
    }

    /// Per anchor, the hash of the anchored code at the task's current head (base before the first
    /// checkpoint). Stored with a new record so later packs can tell when that code changed.
    pub async fn anchor_hashes(
        &self,
        task_id: TaskId,
        anchors: &[scholion_core::Anchor],
    ) -> Result<Vec<Option<String>>> {
        let task = self.store.get_task(task_id).await?;
        let change = if task.head_change.0.is_empty() {
            &task.base_change
        } else {
            &task.head_change
        };
        let mut out = Vec::with_capacity(anchors.len());
        for a in anchors {
            out.push(
                crate::assemble::anchor_hash(
                    self.vcs.as_ref(),
                    self.index.as_ref(),
                    &task.repo,
                    &task.slug,
                    change,
                    a,
                )
                .await,
            );
        }
        Ok(out)
    }

    /// Full text of one item id shown in a pack: `rec:<hex>` or `sym:<n>`.
    pub async fn expand(&self, task_id: TaskId, item_id: &str) -> Result<String> {
        let task = self.store.get_task(task_id).await?;
        let elsewhere = || AvError::NotFound(format!("{item_id} is not in repo {}", task.repo));
        if let Some(prefix) = item_id.strip_prefix("rec:") {
            let rec = self.store.find_record(prefix).await?;
            if self.store.get_task(rec.task_id).await?.repo != task.repo {
                return Err(elsewhere());
            }
            return Ok(record_texts(&rec, false, false).1[Tier::Full as usize].clone());
        }
        if let Some(n) = item_id.strip_prefix("sym:") {
            let id: i64 = n
                .parse()
                .map_err(|_| AvError::Invalid(format!("bad symbol id {item_id}")))?;
            match self.index.symbol_scope(scholion_core::SymbolId(id)).await? {
                Some((repo, _)) if repo == task.repo => {}
                _ => return Err(elsewhere()),
            }
            let sym = self
                .index
                .get_symbols(&[scholion_core::SymbolId(id)])
                .await?
                .into_iter()
                .next()
                .ok_or_else(|| AvError::NotFound(item_id.to_owned()))?;
            let s = &sym.symbol;
            let head = format!(
                "- sym:{id} {} ({}:{}-{})",
                s.qname, s.path, s.start_line, s.end_line
            );
            if task.head_change.0.is_empty() {
                return Ok(format!("{head}\n  {}", s.signature));
            }
            let bytes = self
                .vcs
                .read_file(&task.repo, &task.head_change, &s.path)
                .await?;
            let src = String::from_utf8_lossy(&bytes);
            return Ok(format!(
                "{head}\n{}",
                code_block(
                    &s.path,
                    &excerpt(&src, s.start_line, s.end_line, MAX_EXPAND_LINES)
                )
            ));
        }
        if let Some(n) = item_id.strip_prefix("conv:") {
            let id: i64 = n
                .parse()
                .map_err(|_| AvError::Invalid(format!("bad convention id {item_id}")))?;
            let c = self
                .store
                .conventions(&task.repo)
                .await?
                .into_iter()
                .find(|c| c.id.0 == id)
                .ok_or_else(elsewhere)?;
            let required = if c.required { " (required)" } else { "" };
            return Ok(format!(
                "- conv:{id} ({}){required} {}",
                c.scope_glob, c.text
            ));
        }
        if let Some(slug) = item_id.strip_prefix("task:") {
            let t = self.store.task_by_slug(&task.repo, slug).await?;
            let mut out = format!(
                "# {} · {} · {}\nOwner: {}\nGoal: {}",
                t.title,
                t.slug,
                t.status.as_str(),
                t.owner_agent,
                t.goal
            );
            for rec in self.store.task_records(t.id).await? {
                if rec.kind != scholion_core::RecordKind::Intent {
                    out.push('\n');
                    out.push_str(&record_texts(&rec, false, false).1[Tier::Summary as usize]);
                }
            }
            return Ok(out);
        }
        Err(AvError::Invalid(format!(
            "cannot expand {item_id:?}; expected rec:<id>, sym:<n>, conv:<n> or task:<slug>"
        )))
    }
}

/// Keep the `max` best-scored code items (ties: lower id first, for determinism).
fn cap_code(items: &mut Vec<crate::budget::Item>, max: usize) {
    let mut code: Vec<(f32, String)> = items
        .iter()
        .filter(|i| i.section == Section::Code)
        .map(|i| (i.score, i.id.clone()))
        .collect();
    if code.len() <= max {
        return;
    }
    code.sort_by(|a, b| b.0.total_cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
    let keep: std::collections::HashSet<String> = code.into_iter().take(max).map(|c| c.1).collect();
    items.retain(|i| i.section != Section::Code || keep.contains(&i.id));
}
