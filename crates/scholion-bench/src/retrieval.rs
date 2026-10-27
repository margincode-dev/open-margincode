//! Offline pack retrieval (Plan 6, E1): no agent, no money. For a task with a known fix, the
//! non-test source files and the innermost symbols the fix changed are the gold set. Each method
//! turns the goal into a text of at most `budget` cl100k tokens: Scholion's context pack (from a
//! daemon), and three baselines that need no Scholion: BM25 over files, BM25 over symbols, and an
//! Aider-style repo map (PageRank over the symbol reference graph). Every text is scored the same
//! way, by matching its lines against the gold code, so no method is scored on its own terms.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::Result;
use crate::calibrate::cl100k_tokens;
use crate::runner::{Daemon, scrubbed_copy};
use crate::spec::load_spec_with_root;
use crate::stats::Rng;

/// The methods, in report order.
pub const METHODS: [&str; 4] = ["pack", "bm25_files", "bm25_symbols", "repo_map"];

/// Lines shorter than this (after whitespace is collapsed) are too generic to count as evidence.
const MIN_LINE: usize = 12;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievalTask {
    pub name: String,
    pub repo_path: PathBuf,
    pub base_rev: String,
    pub fix_rev: String,
    pub goal: String,
}

/// Plan 4 `.toml` specs, and `.jsonl` retrieval sets (one task per line, `repo_path` may start
/// with `~/`). With `repos`, each task's repo is looked up by its directory name under it.
pub fn load_tasks(paths: &[PathBuf], repos: Option<&Path>) -> Result<Vec<RetrievalTask>> {
    let mut out = vec![];
    for p in paths {
        if p.is_dir() {
            for f in crate::spec::spec_files(p)? {
                out.extend(load_tasks(&[f], repos)?);
            }
            continue;
        }
        let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
        if ext == "jsonl" {
            let text = std::fs::read_to_string(p).map_err(|e| format!("{}: {e}", p.display()))?;
            for (i, line) in text.lines().enumerate() {
                if line.trim().is_empty() {
                    continue;
                }
                let mut t: RetrievalTask = serde_json::from_str(line)
                    .map_err(|e| format!("{}:{}: {e}", p.display(), i + 1))?;
                t.repo_path = resolve_repo(&t.repo_path, repos);
                out.push(t);
            }
        } else {
            let s = load_spec_with_root(p, repos)?;
            let Some(fix_rev) = s.fix_rev else {
                eprintln!("{}: skipped (no fix_rev, so no gold set)", s.name);
                continue;
            };
            out.push(RetrievalTask {
                name: s.name,
                repo_path: s.repo_path,
                base_rev: s.base_rev,
                fix_rev,
                goal: s.goal,
            });
        }
    }
    Ok(out)
}

fn resolve_repo(p: &Path, repos: Option<&Path>) -> PathBuf {
    if let (Some(root), Some(name)) = (repos, p.file_name()) {
        return root.join(name);
    }
    match p.to_str().and_then(|s| s.strip_prefix("~/")) {
        Some(rest) => PathBuf::from(std::env::var("HOME").unwrap_or_default()).join(rest),
        None => p.to_path_buf(),
    }
}

// ---------------------------------------------------------------- text helpers

/// Whitespace collapsed to single spaces, ends trimmed.
pub fn norm(line: &str) -> String {
    line.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn distinctive(line: &str) -> bool {
    line.len() >= MIN_LINE && line.chars().any(char::is_alphanumeric)
}

/// Languages the Scholion index parses; gold and baselines stay within them.
pub fn indexable(path: &str) -> bool {
    matches!(
        path.rsplit_once('.').map(|(_, e)| e),
        Some("py" | "ts" | "tsx" | "rs")
    )
}

/// Test code: a test directory, or a test file by naming convention.
pub fn is_test_path(path: &str) -> bool {
    let p = path.to_ascii_lowercase();
    let dirs: Vec<&str> = p.split('/').collect();
    let file = dirs.last().copied().unwrap_or("");
    dirs[..dirs.len().saturating_sub(1)].iter().any(|d| {
        matches!(
            *d,
            "test" | "tests" | "__tests__" | "spec" | "specs" | "testing"
        )
    }) || file.starts_with("test_")
        || file == "conftest.py"
        || [
            "_test.py",
            "_tests.py",
            "_test.rs",
            ".test.ts",
            ".test.tsx",
            ".spec.ts",
            ".spec.tsx",
        ]
        .iter()
        .any(|s| file.ends_with(s))
}

const STOP: &[&str] = &[
    "the", "and", "for", "with", "that", "this", "from", "into", "when", "should", "must", "not",
    "are", "was", "but", "its", "can", "has", "have", "will", "all", "any", "each", "use", "uses",
    "used", "add", "make", "fix", "new", "now", "also", "only", "than", "then", "there", "their",
    "them", "they", "what", "which", "who", "how", "out", "one", "two", "via", "per", "def",
    "self", "return", "none", "true", "false", "let", "mut", "pub", "const", "function", "import",
    "class", "str", "int", "bool", "string", "default", "instead", "still", "without", "keep",
    "while", "does", "doesn", "don", "get", "set", "test", "tests",
];

/// Identifier-aware words: each alphanumeric run lowercased, plus its camelCase parts.
pub fn words(text: &str) -> Vec<String> {
    let mut out = vec![];
    for raw in text.split(|c: char| !c.is_alphanumeric()) {
        if raw.len() < 2 {
            continue;
        }
        let whole = raw.to_lowercase();
        let mut parts = vec![];
        let mut cur = String::new();
        let chars: Vec<char> = raw.chars().collect();
        for (i, &c) in chars.iter().enumerate() {
            let boundary = i > 0
                && c.is_uppercase()
                && (chars[i - 1].is_lowercase()
                    || chars.get(i + 1).is_some_and(|n| n.is_lowercase()));
            if boundary && !cur.is_empty() {
                parts.push(std::mem::take(&mut cur).to_lowercase());
            }
            cur.push(c);
        }
        if !cur.is_empty() {
            parts.push(cur.to_lowercase());
        }
        out.push(whole.clone());
        if parts.len() > 1 {
            out.extend(parts.into_iter().filter(|p| p.len() >= 2 && *p != whole));
        }
    }
    out
}

/// The goal's search terms: words of three letters or more, minus stop words, deduplicated.
pub fn goal_terms(goal: &str) -> Vec<String> {
    let mut seen = BTreeSet::new();
    words(goal)
        .into_iter()
        .filter(|w| w.len() >= 3 && !STOP.contains(&w.as_str()) && !w.chars().all(char::is_numeric))
        .filter(|w| seen.insert(w.clone()))
        .collect()
}

// ---------------------------------------------------------------- the repository

/// The source files of a checked-out tree that the index would parse.
pub struct Repo {
    pub files: BTreeMap<String, String>,
    /// Normalized distinctive line → how many files contain it.
    line_files: HashMap<String, usize>,
}

impl Repo {
    pub fn load(dir: &Path) -> Result<Repo> {
        let out = git(dir, &["ls-files", "-z"])?;
        let mut files = BTreeMap::new();
        for p in out.split('\0').filter(|p| !p.is_empty() && indexable(p)) {
            let Ok(bytes) = std::fs::read(dir.join(p)) else {
                continue;
            };
            if bytes.len() > scholion_index::MAX_FILE_BYTES {
                continue;
            }
            if let Ok(text) = String::from_utf8(bytes) {
                files.insert(p.to_owned(), text);
            }
        }
        let mut line_files: HashMap<String, usize> = HashMap::new();
        for text in files.values() {
            let set: HashSet<String> = text.lines().map(norm).filter(|l| distinctive(l)).collect();
            for l in set {
                *line_files.entry(l).or_default() += 1;
            }
        }
        Ok(Repo { files, line_files })
    }

    fn unique(&self, line: &str) -> bool {
        self.line_files.get(line) == Some(&1)
    }

    fn symbols(&self) -> Vec<scholion_index::Symbol> {
        self.files
            .iter()
            .flat_map(|(p, t)| scholion_index::extract(p, t.as_bytes()).unwrap_or_default())
            .collect()
    }
}

fn git(dir: &Path, args: &[&str]) -> Result<String> {
    let out = Command::new("git")
        .current_dir(dir)
        .args(args)
        .output()
        .map_err(|e| format!("git {args:?}: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "git {args:?} in {}: {}",
            dir.display(),
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

// ---------------------------------------------------------------- gold

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GoldSymbol {
    pub path: String,
    pub qname: String,
    /// The definition's first source line, normalized.
    pub signature: String,
    /// Its other distinctive lines, normalized.
    pub body: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Gold {
    /// Non-test source files the fix modified (files it only added cannot be retrieved).
    pub files: Vec<String>,
    pub symbols: Vec<GoldSymbol>,
    /// Whether the goal text names a gold file stem or gold symbol; such tasks are easier for
    /// every method and are reported separately.
    pub leak: bool,
}

/// Old-side line ranges (1-based, inclusive) of a `-U0` diff's hunks. A pure insertion after line
/// `a` touches lines `a` and `a + 1`.
pub fn hunk_ranges(diff: &str) -> Vec<(u32, u32)> {
    let mut out = vec![];
    for line in diff.lines().filter(|l| l.starts_with("@@ -")) {
        let old = line[4..].split_whitespace().next().unwrap_or("");
        let (start, len) = match old.split_once(',') {
            Some((s, n)) => (s.parse().unwrap_or(0), n.parse().unwrap_or(1)),
            None => (old.parse().unwrap_or(0), 1u32),
        };
        if len == 0 {
            out.push((start.max(1), start + 1));
        } else {
            out.push((start, start + len - 1));
        }
    }
    out
}

/// Of the symbols overlapping a range, those that contain no other overlapping symbol.
fn innermost(
    syms: &[scholion_index::Symbol],
    (lo, hi): (u32, u32),
) -> Vec<&scholion_index::Symbol> {
    let hit: Vec<_> = syms
        .iter()
        .filter(|s| s.start_line <= hi && s.end_line >= lo)
        .collect();
    hit.iter()
        .filter(|s| {
            !hit.iter().any(|o| {
                !std::ptr::eq(**s, *o) && s.start_line <= o.start_line && s.end_line >= o.end_line
            })
        })
        .copied()
        .collect()
}

pub fn gold(repo: &Path, base: &str, fix: &str, goal: &str) -> Result<Gold> {
    let status = git(repo, &["diff", "--name-status", "--no-renames", base, fix])?;
    let mut g = Gold::default();
    for line in status.lines() {
        let mut parts = line.split('\t');
        let (Some(st), Some(path)) = (parts.next(), parts.next()) else {
            continue;
        };
        if !st.starts_with('M') || !indexable(path) || is_test_path(path) {
            continue;
        }
        g.files.push(path.to_owned());
        let text = git(repo, &["show", &format!("{base}:{path}")])?;
        let lines: Vec<&str> = text.lines().collect();
        let syms = scholion_index::extract(path, text.as_bytes()).unwrap_or_default();
        let diff = git(repo, &["diff", "-U0", base, fix, "--", path])?;
        let mut picked: Vec<&scholion_index::Symbol> = vec![];
        for r in hunk_ranges(&diff) {
            for s in innermost(&syms, r) {
                if !picked.iter().any(|p| std::ptr::eq(*p, s)) {
                    picked.push(s);
                }
            }
        }
        for s in picked {
            let at = |n: u32| {
                lines
                    .get(n.saturating_sub(1) as usize)
                    .copied()
                    .unwrap_or("")
            };
            g.symbols.push(GoldSymbol {
                path: path.to_owned(),
                qname: s.qname.clone(),
                signature: norm(at(s.start_line)),
                body: (s.start_line + 1..=s.end_line)
                    .map(|n| norm(at(n)))
                    .filter(|l| distinctive(l))
                    .collect(),
            });
        }
    }
    let goal_words: HashSet<String> = words(goal).into_iter().collect();
    g.leak = g.files.iter().any(|f| {
        let stem = f.rsplit('/').next().unwrap_or(f);
        let stem = stem.split('.').next().unwrap_or(stem).to_lowercase();
        stem.len() >= 4 && goal_words.contains(&stem)
    }) || g.symbols.iter().any(|s| {
        let short = s
            .qname
            .rsplit('.')
            .next()
            .unwrap_or(&s.qname)
            .to_lowercase();
        short.len() >= 4 && goal_words.contains(&short)
    });
    Ok(g)
}

// ---------------------------------------------------------------- scoring

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Scores {
    pub tokens: u64,
    /// Share of gold files whose path appears.
    pub files_named: f64,
    /// Share of gold files with code shown: a gold symbol's signature, or three lines found in
    /// no other file of the repo.
    pub files_code: f64,
    /// Share of gold symbols whose signature line appears.
    pub symbols_sig: f64,
    /// Share of gold symbols with at least half of their distinctive lines shown.
    pub symbols_body: f64,
    /// Share of the text's distinctive lines that are gold code (a precision).
    pub gold_share: f64,
    /// Where the first gold path or signature appears, as a fraction of the text (lower is
    /// better); `None` when nothing gold appears.
    pub first_gold: Option<f64>,
}

pub fn score(text: &str, gold: &Gold, repo: &Repo) -> Scores {
    let text_lines: Vec<String> = text.lines().map(norm).collect();
    let line_set: HashSet<&str> = text_lines.iter().map(String::as_str).collect();
    let shown = |l: &str| line_set.contains(l);
    let sig_shown = |s: &GoldSymbol| {
        !s.signature.is_empty()
            && text_lines.iter().any(|l| l.contains(&s.signature))
            && (repo.unique(&s.signature) || text.contains(&s.path))
    };
    let ratio = |n: usize, d: usize| if d == 0 { 0.0 } else { n as f64 / d as f64 };

    let files_named = gold
        .files
        .iter()
        .filter(|f| text.contains(f.as_str()))
        .count();
    let files_code = gold
        .files
        .iter()
        .filter(|f| {
            gold.symbols.iter().any(|s| &s.path == *f && sig_shown(s))
                || repo.files.get(f.as_str()).is_some_and(|t| {
                    t.lines()
                        .map(norm)
                        .filter(|l| distinctive(l) && repo.unique(l) && shown(l))
                        .collect::<HashSet<_>>()
                        .len()
                        >= 3
                })
        })
        .count();
    let symbols_sig = gold.symbols.iter().filter(|s| sig_shown(s)).count();
    let symbols_body = gold
        .symbols
        .iter()
        .filter(|s| {
            if s.body.len() < 2 {
                sig_shown(s)
            } else {
                s.body.iter().filter(|l| shown(l)).count() * 2 >= s.body.len()
            }
        })
        .count();

    let gold_lines: HashSet<&str> = gold
        .symbols
        .iter()
        .flat_map(|s| s.body.iter().chain(std::iter::once(&s.signature)))
        .map(String::as_str)
        .collect();
    let gold_file_unique: HashSet<String> = gold
        .files
        .iter()
        .filter_map(|f| repo.files.get(f.as_str()))
        .flat_map(|t| t.lines().map(norm))
        .filter(|l| distinctive(l) && repo.unique(l))
        .collect();
    let dist: Vec<&String> = text_lines.iter().filter(|l| distinctive(l)).collect();
    let gold_n = dist
        .iter()
        .filter(|l| gold_lines.contains(l.as_str()) || gold_file_unique.contains(l.as_str()))
        .count();

    let first = gold
        .files
        .iter()
        .filter_map(|f| text.find(f.as_str()))
        .chain(
            gold.symbols
                .iter()
                .filter(|s| !s.signature.is_empty())
                .filter_map(|s| text.find(&s.signature)),
        )
        .min();

    Scores {
        tokens: cl100k_tokens(text),
        files_named: ratio(files_named, gold.files.len()),
        files_code: ratio(files_code, gold.files.len()),
        symbols_sig: ratio(symbols_sig, gold.symbols.len()),
        symbols_body: ratio(symbols_body, gold.symbols.len()),
        gold_share: ratio(gold_n, dist.len()),
        first_gold: first.map(|i| i as f64 / text.len().max(1) as f64),
    }
}

// ---------------------------------------------------------------- baselines

/// Concatenates items in order while the total stays within `budget` cl100k tokens. The first
/// item that does not fit is cut to the lines that do (if any), and nothing follows it.
pub fn fit(items: &[String], budget: u32) -> String {
    let budget = u64::from(budget);
    let mut out = String::new();
    for item in items {
        if cl100k_tokens(&format!("{out}{item}")) <= budget {
            out.push_str(item);
            continue;
        }
        let mut lo = 0usize;
        let lines: Vec<&str> = item.split_inclusive('\n').collect();
        let mut hi = lines.len();
        while lo < hi {
            let mid = (lo + hi).div_ceil(2);
            if cl100k_tokens(&format!("{out}{}", lines[..mid].concat())) <= budget {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }
        if lo >= 3 {
            out.push_str(&lines[..lo].concat());
        }
        break;
    }
    out
}

/// Okapi BM25 (k1 = 1.2, b = 0.75) of each document against the query terms, best first; zero
/// scores are dropped. Ties keep document order.
pub fn bm25(docs: &[Vec<String>], query: &[String]) -> Vec<(usize, f64)> {
    let n = docs.len() as f64;
    if docs.is_empty() || query.is_empty() {
        return vec![];
    }
    let avg = docs.iter().map(Vec::len).sum::<usize>() as f64 / n;
    let tfs: Vec<HashMap<&str, usize>> = docs
        .iter()
        .map(|d| {
            let mut m = HashMap::new();
            for w in d {
                *m.entry(w.as_str()).or_default() += 1;
            }
            m
        })
        .collect();
    let mut scored: Vec<(usize, f64)> = tfs
        .iter()
        .enumerate()
        .map(|(i, tf)| {
            let len = docs[i].len() as f64;
            let s: f64 = query
                .iter()
                .map(|q| {
                    let df = tfs.iter().filter(|t| t.contains_key(q.as_str())).count() as f64;
                    let f = *tf.get(q.as_str()).unwrap_or(&0) as f64;
                    if f == 0.0 {
                        return 0.0;
                    }
                    let idf = ((n - df + 0.5) / (df + 0.5) + 1.0).ln();
                    idf * f * 2.2 / (f + 1.2 * (0.25 + 0.75 * len / avg.max(1.0)))
                })
                .sum();
            (i, s)
        })
        .filter(|(_, s)| *s > 0.0)
        .collect();
    scored.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
    scored
}

fn fence(path: &str) -> &'static str {
    match path.rsplit_once('.').map(|(_, e)| e) {
        Some("py") => "python",
        Some("rs") => "rust",
        Some("ts" | "tsx") => "typescript",
        _ => "",
    }
}

/// Whole files, best BM25 match first.
pub fn bm25_files(repo: &Repo, goal: &str, budget: u32) -> String {
    let paths: Vec<&String> = repo.files.keys().collect();
    let docs: Vec<Vec<String>> = paths
        .iter()
        .map(|p| words(&format!("{p}\n{}", repo.files[*p])))
        .collect();
    let items: Vec<String> = bm25(&docs, &goal_terms(goal))
        .into_iter()
        .map(|(i, _)| {
            let p = paths[i];
            format!(
                "### {p}\n```{}\n{}\n```\n",
                fence(p),
                repo.files[p].trim_end()
            )
        })
        .collect();
    fit(&items, budget)
}

/// Symbol bodies, best BM25 match first (what full-text code search would hand an agent).
pub fn bm25_symbols(repo: &Repo, goal: &str, budget: u32) -> String {
    let syms = repo.symbols();
    let bodies: Vec<String> = syms
        .iter()
        .map(|s| {
            let lines: Vec<&str> = repo.files[&s.path].lines().collect();
            let lo = s.start_line.saturating_sub(1) as usize;
            let hi = (s.end_line as usize).min(lines.len());
            lines.get(lo..hi).unwrap_or(&[]).join("\n")
        })
        .collect();
    let docs: Vec<Vec<String>> = syms
        .iter()
        .zip(&bodies)
        .map(|(s, b)| words(&format!("{} {}\n{b}", s.path, s.qname)))
        .collect();
    let mut shown: Vec<(u32, u32, &str)> = vec![];
    let mut items = vec![];
    for (i, _) in bm25(&docs, &goal_terms(goal)) {
        let s = &syms[i];
        // a method inside a class already shown adds nothing
        if shown
            .iter()
            .any(|(a, b, p)| *p == s.path && *a <= s.start_line && *b >= s.end_line)
        {
            continue;
        }
        shown.push((s.start_line, s.end_line, &s.path));
        items.push(format!(
            "- {} ({}:{}-{})\n```{}\n{}\n```\n",
            s.qname,
            s.path,
            s.start_line,
            s.end_line,
            fence(&s.path),
            bodies[i]
        ));
    }
    fit(&items, budget)
}

/// An Aider-style repo map: files ranked by personalized PageRank over "file A uses an identifier
/// that file B defines" edges (identifiers the goal names weigh 10x, private ones 0.1x; files whose
/// path or symbols match the goal are the personalization), then each file's symbols as
/// `L<n>: <signature>` lines, best files and goal-matching symbols first.
pub fn repo_map(repo: &Repo, goal: &str, budget: u32) -> String {
    let syms = repo.symbols();
    let terms: HashSet<String> = goal_terms(goal).into_iter().collect();
    let paths: Vec<&String> = repo.files.keys().collect();
    let idx: HashMap<&str, usize> = paths
        .iter()
        .enumerate()
        .map(|(i, p)| (p.as_str(), i))
        .collect();
    let short = |q: &str| q.rsplit('.').next().unwrap_or(q).to_owned();
    let mut defs: HashMap<String, BTreeSet<usize>> = HashMap::new();
    for s in &syms {
        defs.entry(short(&s.qname))
            .or_default()
            .insert(idx[s.path.as_str()]);
    }
    let matches = |name: &str| words(name).iter().filter(|w| terms.contains(*w)).count();

    // weighted edges referencer -> definer
    let n = paths.len();
    let mut edges: Vec<HashMap<usize, f64>> = vec![HashMap::new(); n];
    for s in &syms {
        let from = idx[s.path.as_str()];
        for r in &s.refs {
            let Some(targets) = defs.get(r) else { continue };
            let mut w = 1.0 / (targets.len() as f64).sqrt();
            if matches(r) > 0 {
                w *= 10.0;
            }
            if r.starts_with('_') {
                w *= 0.1;
            }
            for &to in targets {
                if to != from {
                    *edges[from].entry(to).or_default() += w;
                }
            }
        }
    }
    let mut pers: Vec<f64> = paths.iter().map(|p| matches(p) as f64).collect();
    for s in &syms {
        pers[idx[s.path.as_str()]] += matches(&s.qname) as f64;
    }
    let total: f64 = pers.iter().sum();
    if total == 0.0 {
        pers.iter_mut().for_each(|p| *p = 1.0 / n.max(1) as f64);
    } else {
        pers.iter_mut().for_each(|p| *p /= total);
    }
    let mut rank = pers.clone();
    for _ in 0..30 {
        let mut next: Vec<f64> = pers.iter().map(|p| 0.15 * p).collect();
        for (from, out) in edges.iter().enumerate() {
            let sum: f64 = out.values().sum();
            if sum == 0.0 {
                // dangling: hand its rank back through the personalization
                for (i, p) in pers.iter().enumerate() {
                    next[i] += 0.85 * rank[from] * p;
                }
                continue;
            }
            for (&to, &w) in out {
                next[to] += 0.85 * rank[from] * w / sum;
            }
        }
        rank = next;
    }

    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|a, b| rank[*b].total_cmp(&rank[*a]).then(a.cmp(b)));
    let mut by_file: HashMap<usize, Vec<&scholion_index::Symbol>> = HashMap::new();
    for s in &syms {
        by_file.entry(idx[s.path.as_str()]).or_default().push(s);
    }
    let items: Vec<String> = order
        .into_iter()
        .filter_map(|f| {
            let mut ss = by_file.remove(&f)?;
            ss.sort_by_key(|s| (std::cmp::Reverse(matches(&s.qname)), s.start_line));
            let lines: Vec<&str> = repo.files[paths[f]].lines().collect();
            let mut item = format!("{}:\n", paths[f]);
            for s in ss {
                let first = lines
                    .get(s.start_line.saturating_sub(1) as usize)
                    .map(|l| l.trim())
                    .unwrap_or(&s.signature);
                item.push_str(&format!("  L{}: {first}\n", s.start_line));
            }
            Some(item)
        })
        .collect();
    fit(&items, budget)
}

// ---------------------------------------------------------------- running

pub struct RetrievalArgs {
    pub budgets: Vec<u32>,
    pub out: PathBuf,
    /// Host directory the daemon imports from, and how the daemon sees it.
    pub import_dir: PathBuf,
    pub import_mount: String,
    /// Without a semantic layer the daemon's packs use full-text search only.
    pub allow_no_semantic: bool,
    pub index_wait_secs: u64,
    /// Skip the pack method (baselines only, no daemon).
    pub no_pack: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Row {
    pub task: String,
    pub method: String,
    pub budget: u32,
    pub leak: bool,
    pub gold_files: usize,
    pub gold_symbols: usize,
    /// The pack's `degraded` notes (e.g. `semantic` when semantic search failed or timed out).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub degraded: Vec<String>,
    #[serde(flatten)]
    pub scores: Scores,
}

fn wait_semantic(daemon: &Daemon, repo: &str, args: &RetrievalArgs) -> Result<bool> {
    let start = Instant::now();
    loop {
        let s = daemon.get(&format!("/v1/repos/{repo}/outbox"))?;
        if s["semantic"] != true {
            if args.allow_no_semantic {
                return Ok(false);
            }
            return Err(
                "the daemon has no semantic layer; packs would undersell Scholion \
                        (pass --allow-no-semantic to measure full-text-only packs)"
                    .into(),
            );
        }
        if s["pending"].as_i64() == Some(0) {
            return Ok(true);
        }
        if start.elapsed() >= Duration::from_secs(args.index_wait_secs) {
            return Err(format!(
                "{repo}: embeddings still queued after {} s",
                args.index_wait_secs
            ));
        }
        std::thread::sleep(Duration::from_secs(2));
    }
}

/// Imports the base tree as its own repo, starts a task with the goal, and returns the first pack
/// at each budget, each for a fresh session (nothing deduplicated against earlier packs).
fn packs(
    t: &RetrievalTask,
    run_id: &str,
    src: &Path,
    daemon: &Daemon,
    args: &RetrievalArgs,
) -> Result<(Vec<(String, Vec<String>)>, bool)> {
    let name = src
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or("bad src dir")?;
    daemon.post(
        "/v1/repos",
        json!({"name": name, "git_source": format!("{}/{name}", args.import_mount.trim_end_matches('/'))}),
        None,
    )?;
    let semantic = wait_semantic(daemon, name, args)?;
    let started = daemon.post(
        "/v1/tasks",
        json!({"repo": name, "title": t.name, "goal": t.goal, "agent": "retrieval-eval",
               "session": format!("ret-{run_id}-start"), "pack_mode": "full"}),
        None,
    )?;
    let id = started["task"]["id"]
        .as_str()
        .ok_or("no task id")?
        .to_owned();
    let mut out = vec![];
    for b in &args.budgets {
        let session = uuid::Uuid::new_v4();
        let p = daemon.get(&format!(
            "/v1/tasks/{id}/context?budget={b}&family=other&session=ret-{session}"
        ))?;
        let degraded = p["degraded"]
            .as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(str::to_owned))
                    .collect()
            })
            .unwrap_or_default();
        out.push((
            p["markdown"].as_str().unwrap_or_default().to_owned(),
            degraded,
        ));
    }
    Ok((out, semantic))
}

pub fn run(
    tasks: &[RetrievalTask],
    daemon: Option<&Daemon>,
    args: &RetrievalArgs,
) -> Result<Vec<Row>> {
    std::fs::create_dir_all(args.out.join("texts")).map_err(|e| e.to_string())?;
    let run_id = format!(
        "{:x}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
            & 0xffffff
    );
    let mut rows = vec![];
    let mut golds = vec![];
    let mut semantic_any = None;
    for t in tasks {
        let g = match gold(&t.repo_path, &t.base_rev, &t.fix_rev, &t.goal) {
            Ok(g) if !g.files.is_empty() => g,
            Ok(_) => {
                eprintln!(
                    "{}: skipped (the fix modifies no non-test source file)",
                    t.name
                );
                continue;
            }
            Err(e) => {
                eprintln!("{}: skipped ({e})", t.name);
                continue;
            }
        };
        let src = args.import_dir.join(format!("r{run_id}-{}", t.name));
        scrubbed_copy(&t.repo_path, &t.base_rev, &src)?;
        let repo = Repo::load(&src)?;
        let mut texts: Vec<(&str, u32, String, Vec<String>)> = vec![];
        if !args.no_pack {
            let daemon = daemon.ok_or("the pack method needs a daemon (--url)")?;
            let (ps, semantic) = packs(t, &run_id, &src, daemon, args)?;
            semantic_any = Some(semantic_any.unwrap_or(false) || semantic);
            for (b, (p, degraded)) in args.budgets.iter().zip(ps) {
                texts.push(("pack", *b, p, degraded));
            }
        }
        for &b in &args.budgets {
            texts.push(("bm25_files", b, bm25_files(&repo, &t.goal, b), vec![]));
            texts.push(("bm25_symbols", b, bm25_symbols(&repo, &t.goal, b), vec![]));
            texts.push(("repo_map", b, repo_map(&repo, &t.goal, b), vec![]));
        }
        let dir = args.out.join("texts").join(&t.name);
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        for (m, b, text, degraded) in texts {
            let _ = std::fs::write(dir.join(format!("{m}-{b}.md")), &text);
            rows.push(Row {
                task: t.name.clone(),
                method: m.to_owned(),
                budget: b,
                leak: g.leak,
                gold_files: g.files.len(),
                gold_symbols: g.symbols.len(),
                degraded,
                scores: score(&text, &g, &repo),
            });
        }
        golds.push(json!({"task": t.name, "gold": g}));
        eprintln!(
            "{}: {} gold files, {} gold symbols",
            t.name,
            g.files.len(),
            g.symbols.len()
        );
        let _ = std::fs::remove_dir_all(&src);
    }
    let jsonl = |v: Vec<String>| v.join("\n") + "\n";
    std::fs::write(
        args.out.join("rows.jsonl"),
        jsonl(
            rows.iter()
                .map(|r| serde_json::to_string(r).unwrap())
                .collect(),
        ),
    )
    .map_err(|e| e.to_string())?;
    std::fs::write(
        args.out.join("gold.jsonl"),
        jsonl(golds.iter().map(|g| g.to_string()).collect()),
    )
    .map_err(|e| e.to_string())?;
    let meta = json!({"budgets": args.budgets, "tasks": tasks.len(), "scored_tasks": golds.len(),
                      "pack": !args.no_pack, "semantic": semantic_any});
    std::fs::write(args.out.join("meta.json"), meta.to_string()).map_err(|e| e.to_string())?;
    std::fs::write(args.out.join("report.md"), report(&rows, &meta)).map_err(|e| e.to_string())?;
    Ok(rows)
}

// ---------------------------------------------------------------- report

const METRICS: [&str; 5] = [
    "files_code",
    "symbols_sig",
    "symbols_body",
    "files_named",
    "gold_share",
];

fn metric(r: &Row, m: &str) -> f64 {
    match m {
        "files_named" => r.scores.files_named,
        "files_code" => r.scores.files_code,
        "symbols_sig" => r.scores.symbols_sig,
        "symbols_body" => r.scores.symbols_body,
        "gold_share" => r.scores.gold_share,
        _ => 0.0,
    }
}

fn mean(v: &[f64]) -> f64 {
    if v.is_empty() {
        0.0
    } else {
        v.iter().sum::<f64>() / v.len() as f64
    }
}

/// 95 % bootstrap interval of the mean (2000 resamples, fixed seed).
pub fn bootstrap_mean_ci(v: &[f64], seed: u64) -> Option<(f64, f64)> {
    if v.len() < 2 {
        return None;
    }
    let mut rng = Rng::new(seed);
    let mut means: Vec<f64> = (0..2000)
        .map(|_| {
            mean(
                &(0..v.len())
                    .map(|_| v[rng.below(v.len())])
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    means.sort_by(f64::total_cmp);
    Some((means[50], means[1949]))
}

pub fn report(rows: &[Row], meta: &serde_json::Value) -> String {
    let mut s = String::from("# Offline pack retrieval (Plan 6, E1)\n\n");
    let tasks: BTreeSet<&str> = rows.iter().map(|r| r.task.as_str()).collect();
    let leaky = rows
        .iter()
        .filter(|r| r.leak)
        .map(|r| r.task.as_str())
        .collect::<BTreeSet<_>>();
    s.push_str(&format!(
        "{} tasks ({} whose goal names a gold file or symbol). Semantic layer: {}. Methods get \
         the same cl100k budget; every text is scored by the same line matching against the \
         fix's gold files and symbols.\n\n",
        tasks.len(),
        leaky.len(),
        match meta["semantic"].as_bool() {
            Some(true) => "on",
            Some(false) => "off (full-text packs)",
            None => "n/a",
        }
    ));
    let degraded = rows
        .iter()
        .filter(|r| r.method == "pack" && !r.degraded.is_empty())
        .count();
    let packs = rows.iter().filter(|r| r.method == "pack").count();
    if packs > 0 {
        s.push_str(&format!(
            "Packs marked degraded (semantic search failed or timed out): {degraded} of {packs}.\n\n"
        ));
    }
    s.push_str("Metrics (higher is better, means over tasks):\n\n\
        - `files_code`: gold files with code shown (a gold signature, or three lines unique to the file)\n\
        - `symbols_sig`: gold symbols whose signature line is shown\n\
        - `symbols_body`: gold symbols with at least half their lines shown\n\
        - `files_named`: gold file paths mentioned\n\
        - `gold_share`: share of the text's lines that are gold code (precision)\n\n");
    let budgets: BTreeSet<u32> = rows.iter().map(|r| r.budget).collect();
    let methods: Vec<&str> = METHODS
        .iter()
        .copied()
        .filter(|m| rows.iter().any(|r| r.method == *m))
        .collect();
    for subset in ["all", "no-leak"] {
        let keep = |r: &&Row| subset == "all" || !r.leak;
        let n = rows
            .iter()
            .filter(keep)
            .map(|r| r.task.as_str())
            .collect::<BTreeSet<_>>()
            .len();
        if n == 0 {
            continue;
        }
        s.push_str(&format!(
            "## {} tasks (n = {n})\n\n",
            if subset == "all" {
                "All"
            } else {
                "Goal names no gold file or symbol:"
            }
        ));
        for b in &budgets {
            s.push_str(&format!(
                "### Budget {b}\n\n| method | tokens | {} |\n|---|---|{}\n",
                METRICS.join(" | "),
                "---|".repeat(METRICS.len())
            ));
            for m in &methods {
                let rs: Vec<&Row> = rows
                    .iter()
                    .filter(keep)
                    .filter(|r| r.method == *m && r.budget == *b)
                    .collect();
                let toks = mean(
                    &rs.iter()
                        .map(|r| r.scores.tokens as f64)
                        .collect::<Vec<_>>(),
                );
                let cells: Vec<String> = METRICS
                    .iter()
                    .map(|k| {
                        format!(
                            "{:.2}",
                            mean(&rs.iter().map(|r| metric(r, k)).collect::<Vec<_>>())
                        )
                    })
                    .collect();
                s.push_str(&format!("| {m} | {toks:.0} | {} |\n", cells.join(" | ")));
            }
            s.push('\n');
        }
    }
    if methods.contains(&"pack") {
        s.push_str("## Pack against each baseline, paired by task\n\nMean difference (pack minus baseline) with a 95 % bootstrap interval, and wins/ties/losses over tasks.\n\n| budget | baseline | metric | diff | 95 % CI | W/T/L |\n|---|---|---|---|---|---|\n");
        for b in &budgets {
            for base in methods.iter().filter(|m| **m != "pack") {
                for k in ["files_code", "symbols_sig"] {
                    let mut diffs = vec![];
                    let (mut w, mut t, mut l) = (0, 0, 0);
                    for task in &tasks {
                        let get = |m: &str| {
                            rows.iter()
                                .find(|r| r.task == *task && r.method == m && r.budget == *b)
                                .map(|r| metric(r, k))
                        };
                        if let (Some(p), Some(q)) = (get("pack"), get(base)) {
                            diffs.push(p - q);
                            match (p - q).partial_cmp(&0.0) {
                                Some(std::cmp::Ordering::Greater) => w += 1,
                                Some(std::cmp::Ordering::Less) => l += 1,
                                _ => t += 1,
                            }
                        }
                    }
                    let ci = bootstrap_mean_ci(&diffs, 7)
                        .map(|(a, c)| format!("[{a:+.2}, {c:+.2}]"))
                        .unwrap_or_else(|| "n/a".into());
                    s.push_str(&format!(
                        "| {b} | {base} | {k} | {:+.2} | {ci} | {w}/{t}/{l} |\n",
                        mean(&diffs)
                    ));
                }
            }
        }
        s.push('\n');
    }
    if let Some(mid) = budgets.iter().nth(budgets.len() / 2) {
        s.push_str(&format!("## Per task at budget {mid}: `files_code` / `symbols_sig`\n\n| task | gold files | gold symbols | leak | {} |\n|---|---|---|---|{}\n", methods.join(" | "), "---|".repeat(methods.len())));
        for task in &tasks {
            let first = rows.iter().find(|r| r.task == *task).unwrap();
            let cells: Vec<String> = methods
                .iter()
                .map(|m| {
                    rows.iter()
                        .find(|r| r.task == *task && r.method == *m && r.budget == *mid)
                        .map(|r| {
                            format!("{:.2} / {:.2}", r.scores.files_code, r.scores.symbols_sig)
                        })
                        .unwrap_or_else(|| "–".into())
                })
                .collect();
            s.push_str(&format!(
                "| {task} | {} | {} | {} | {} |\n",
                first.gold_files,
                first.gold_symbols,
                if first.leak { "yes" } else { "" },
                cells.join(" | ")
            ));
        }
    }
    s
}
