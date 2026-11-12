//! E4 at scale (Plan 13, Phase 1): `run --noise N` adds N irrelevant but plausible notes to a
//! relay task's pool, so the `notes` arm's `NOTES.md` and the av arms' records grow by the same
//! N notes, word for word.
//!
//! The notes come from a seeded generator over the base tree's own source files and symbols
//! (`scholion_index::extract`), not from an LLM and not from history (the tallyho repo is one
//! commit). Each is anchored to a real file, and to a real symbol in it unless it is about the
//! whole module; test files are left out. Relevant files are **not** left out: in a real repo old
//! notes pile up on the hot files too, and av's anchoring has to pick the lesson out from among
//! them (the pre-read hook ranks a file's records by kind, then newest first, within 400 tokens).
//!
//! Every (anchor, template) pair is shuffled once by the seed and the first N are taken, so a
//! level's notes are the first N of any higher level's (N=50 ⊂ N=200 ⊂ N=500), and every task on
//! the same base gets the same notes. Past anchors × templates the order repeats with a round
//! number, so notes stay distinct even on a tiny repo.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::Result;
use crate::retrieval::{indexable, is_test_path};
use crate::spec::{RelayNote, TaskSpec, git_in, resolve_rev};
use crate::stats::Rng;

/// Where a noise note is anchored: a file, and a symbol in it unless the note is about the module.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Anchor {
    pub path: String,
    pub symbol: Option<String>,
}

/// One kind of note, with `{what}` for the symbol (`` `Cart.add` ``) or the module.
struct Template {
    kind: &'static str,
    summary: &'static str,
    body: &'static str,
}

/// Generic engineering lessons that sound like an earlier agent's notes but help none of the
/// relay tasks: nothing about money, rounding, currencies, taxes, coupons, slugs, ids, events,
/// refunds, regions, pagination or error message wording.
const TEMPLATES: [Template; 18] = [
    Template {
        kind: "attempt",
        summary: "Tried calling {what} twice to force a refresh; the second call returned the same stale object and nothing updated.",
        body: "A sleep between the calls did not help either. It needs a real invalidation path, not a retry.",
    },
    Template {
        kind: "attempt",
        summary: "Wrapping {what} in a bare try/except with a retry loop hid the real error and made the suite hang on the second pass.",
        body: "Narrow the except or let it raise; retries belong at a call site that can back off.",
    },
    Template {
        kind: "attempt",
        summary: "Caching what {what} returns in a module-level dict broke tests that expect a fresh value per case.",
        body: "Keep any cache on the instance, or clear it in a fixture. A process-wide cache leaks between tests.",
    },
    Template {
        kind: "attempt",
        summary: "Replacing the loop in {what} with a comprehension made it shorter but much harder to step through in a debugger, so it was reverted.",
        body: "No measurable speed-up either; readability won.",
    },
    Template {
        kind: "attempt",
        summary: "Adding type: ignore comments around {what} to quiet the checker hid a real Optional bug two callers up.",
        body: "Fix the annotation instead; the ignore comments were removed again.",
    },
    Template {
        kind: "attempt",
        summary: "Moving the imports inside {what} to speed up start-up saved under a millisecond and confused the linter.",
        body: "Start-up time is dominated by the interpreter itself; imports stay at the top of the module.",
    },
    Template {
        kind: "decision",
        summary: "Log lines from {what} go through the logging module, never print(), so operators can filter them by logger name.",
        body: "Agreed in the ops review; print() in library code is a review reject.",
    },
    Template {
        kind: "decision",
        summary: "New optional parameters of {what} are keyword-only (after *), so call sites stay readable when options are added.",
        body: "A positional bool added in the middle of a signature bit us once.",
    },
    Template {
        kind: "decision",
        summary: "{what} keeps its docstring as the one place its behaviour is described; the README only links to it.",
        body: "The README is for operators, not API detail.",
    },
    Template {
        kind: "decision",
        summary: "Type hints on {what} use the built-in generics (list[str], dict[str, int]), not the typing aliases.",
        body: "The project targets a Python that has them; mixing both styles made reviews noisy.",
    },
    Template {
        kind: "decision",
        summary: "Constants that {what} depends on live at the top of its module in UPPER_CASE, not as magic numbers inline.",
        body: "Makes them greppable and lets tests patch them in one place.",
    },
    Template {
        kind: "decision",
        summary: "Tests for {what} build their inputs with small factory helpers in conftest, not shared module-level objects.",
        body: "Shared objects let one test's mutation leak into the next.",
    },
    Template {
        kind: "note",
        summary: "Remember {what} is not thread-safe; each worker needs its own instance or an external lock.",
        body: "Found after a flaky run under pytest-xdist.",
    },
    Template {
        kind: "note",
        summary: "{what} is on the hot path of the nightly batch job; profile before adding work to it.",
        body: "A previous change doubled the batch time without anyone noticing for a week.",
    },
    Template {
        kind: "note",
        summary: "The repr of objects around {what} shows up in logs, so keep it short and free of large nested data.",
        body: "A log line of several kilobytes once filled the disk of a small box.",
    },
    Template {
        kind: "note",
        summary: "{what} reads files as UTF-8 explicitly; the platform default encoding differs on the Windows build agents.",
        body: "Always pass encoding= when opening text files here.",
    },
    Template {
        kind: "note",
        summary: "Deprecation warnings from {what} are turned into errors in CI (filterwarnings = error), so fix them when they appear.",
        body: "Silencing them locally only moves the failure to CI.",
    },
    Template {
        kind: "note",
        summary: "Benchmarks for {what} are in a separate, manual job; the unit tests must not depend on timing.",
        body: "Timing assertions were flaky on shared runners and were removed.",
    },
];

/// Every source file of `rev` the index would parse (tests left out), as a module-level anchor
/// plus one anchor per function, method and type in it; sorted.
pub fn anchors_at(repo: &Path, rev: &str) -> Result<Vec<Anchor>> {
    let rev = resolve_rev(repo, rev)?;
    let names = git_in(
        repo,
        &["ls-tree", "-r", "--name-only", "--end-of-options", &rev],
    )?;
    let mut out = BTreeSet::new();
    for path in String::from_utf8_lossy(&names).lines() {
        if !indexable(path) || is_test_path(path) {
            continue;
        }
        let bytes = git_in(
            repo,
            &["show", "--end-of-options", &format!("{rev}:{path}")],
        )?;
        out.insert(Anchor {
            path: path.to_owned(),
            symbol: None,
        });
        if bytes.len() > scholion_index::MAX_FILE_BYTES {
            continue;
        }
        for s in scholion_index::extract(path, &bytes).unwrap_or_default() {
            if s.kind != scholion_index::SymbolKind::Constant {
                out.insert(Anchor {
                    path: path.to_owned(),
                    symbol: Some(s.qname),
                });
            }
        }
    }
    Ok(out.into_iter().collect())
}

fn module_name(path: &str) -> &str {
    let file = path.rsplit('/').next().unwrap_or(path);
    file.rsplit_once('.').map_or(file, |(stem, _)| stem)
}

/// At most `max` characters.
fn clip(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_owned();
    }
    let mut t: String = s.chars().take(max - 3).collect();
    t.push_str("...");
    t
}

/// `n` noise notes over `anchors`, deterministic in `seed`; the first `k` of any larger `n` are
/// the notes for `k`. Empty without anchors.
pub fn generate(anchors: &[Anchor], n: usize, seed: u64) -> Vec<RelayNote> {
    if anchors.is_empty() || n == 0 {
        return vec![];
    }
    let mut order: Vec<(usize, usize)> = (0..anchors.len())
        .flat_map(|a| (0..TEMPLATES.len()).map(move |t| (a, t)))
        .collect();
    Rng::new(seed).shuffle(&mut order);
    (0..n)
        .map(|i| {
            let (a, t) = order[i % order.len()];
            let round = i / order.len();
            let (anchor, tpl) = (&anchors[a], &TEMPLATES[t]);
            let what = match &anchor.symbol {
                Some(s) => format!("`{s}`"),
                None => format!("the `{}` module", module_name(&anchor.path)),
            };
            let mut summary = tpl.summary.replace("{what}", &what);
            if let Some(rest) = summary.strip_prefix("the ") {
                summary = format!("The {rest}");
            }
            let summary = match round {
                0 => clip(&summary, 200),
                r => {
                    let tag = format!(" (seen again, round {})", r + 1);
                    clip(&summary, 200 - tag.chars().count()) + &tag
                }
            };
            RelayNote {
                pair: format!("noise-{:04}", i + 1),
                kind: tpl.kind.to_owned(),
                path: anchor.path.clone(),
                symbol: anchor.symbol.clone(),
                summary,
                body: tpl.body.to_owned(),
                noise: true,
                no_commit: false,
            }
        })
        .collect()
}

/// `base` with `noise` spread through it at seeded positions; each keeps its own order (so the
/// lessons' commits stay in pool order), and without noise `base` is returned as it is.
pub fn interleave(base: Vec<RelayNote>, noise: Vec<RelayNote>, seed: u64) -> Vec<RelayNote> {
    if noise.is_empty() {
        return base;
    }
    let mut from_base: Vec<bool> = std::iter::repeat_n(true, base.len())
        .chain(std::iter::repeat_n(false, noise.len()))
        .collect();
    // another stream than `generate`'s, so positions do not track the notes' own shuffle
    Rng::new(seed ^ 0x6E6F_6973_6500).shuffle(&mut from_base);
    let (mut b, mut n) = (base.into_iter(), noise.into_iter());
    from_base
        .into_iter()
        .filter_map(|take_base| if take_base { b.next() } else { n.next() })
        .collect()
}

/// `run --noise N`: every relay task gets N noise notes interleaved into its pool, generated once
/// per (repo, base) so tasks on one base share them. Refuses a task without `relay_pool`.
pub fn add_noise(specs: &mut [TaskSpec], n: usize, seed: u64) -> Result<()> {
    if n == 0 {
        return Ok(());
    }
    if let Some(s) = specs.iter().find(|s| s.relay_pool.is_none()) {
        return Err(format!(
            "--noise needs relay tasks: task {} has no relay_pool",
            s.name
        ));
    }
    let mut by_base: BTreeMap<(PathBuf, String), Vec<RelayNote>> = BTreeMap::new();
    for s in specs.iter_mut() {
        let key = (s.repo_path.clone(), resolve_rev(&s.repo_path, &s.base_rev)?);
        let noise = match by_base.get(&key) {
            Some(v) => v.clone(),
            None => {
                let v = generate(&anchors_at(&key.0, &key.1)?, n, seed);
                if v.is_empty() {
                    return Err(format!(
                        "task {}: the base tree has no source files to anchor noise to",
                        s.name
                    ));
                }
                by_base.insert(key, v.clone());
                v
            }
        };
        s.relay_notes = interleave(std::mem::take(&mut s.relay_notes), noise, seed);
    }
    Ok(())
}
