//! Embedding chunk length and chunking strategy (2026-11-14). Offline and dense-only: for each
//! retrieval task the goal is embedded raw, as `margincode-pack` sends it, and every symbol at
//! `base_rev` is ranked by cosine similarity to it. The gold symbols' ranks are scored. An arm
//! changes only how a symbol's chunk text is built; the first part of every chunk is
//! `margincode_index::chunk_source`, the text the index embeds today before its cap.
//!
//! Vectors are cached on disk by (embedder tag, text), so arms that share a text embed it once.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::Result;
use crate::retrieval::{RetrievalTask, bootstrap_mean_ci, gold, indexable};

/// Path segments the index never reads (`margincode_index` store's `IGNORED_SEGMENTS`).
const IGNORED_SEGMENTS: [&str; 4] = ["node_modules", ".venv", "target", ".git"];
/// A window arm embeds at most this many windows of one symbol.
const MAX_WINDOWS: usize = 64;
/// The cut-offs of the rank metrics: 8 is the pack's semantic code-hit limit at the agents'
/// default budget (`score::SEMANTIC_LIMIT`), 40 its largest (`MAX_CANDIDATES`).
pub const KS: [usize; 3] = [8, 20, 40];
/// A gold symbol longer than this is "long": the cap in force when this was measured.
pub const LONG: usize = 2000;

// ---------------------------------------------------------------- arms

/// How a symbol's chunk becomes the text(s) that are embedded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm {
    /// The whole chunk cut to at most N chars (`margincode_index::cap_chars`); 0 is no cap.
    Trunc(usize),
    /// A chunk over N chars is split at line boundaries into windows of at most N chars; each
    /// window starts with the chunk's header (path, qualified name, signature) and repeats the
    /// last `overlap` chars of lines of the window before it. The symbol scores its best window.
    Window { size: usize, overlap: usize },
    /// The same windows, but one vector per symbol: the normalized mean of its window vectors.
    WindowMean { size: usize, overlap: usize },
    /// A symbol that contains other symbols (a class, a trait) keeps its own lines and only the
    /// first line of each symbol it contains, then is cut to N chars. Other symbols as `Trunc`.
    Members(usize),
}

impl Arm {
    pub fn parse(s: &str) -> Result<Arm> {
        let parts: Vec<&str> = s.trim().split(':').collect();
        let num = |i: usize| -> Result<usize> {
            let p = parts
                .get(i)
                .ok_or_else(|| format!("arm {s}: missing number"))?;
            if *p == "inf" {
                return Ok(0);
            }
            p.parse()
                .map_err(|_| format!("arm {s}: {p} is not a number"))
        };
        match parts[0] {
            "trunc" if parts.len() == 2 => Ok(Arm::Trunc(num(1)?)),
            "members" if parts.len() == 2 => Ok(Arm::Members(num(1)?)),
            "win" | "winmean" if parts.len() == 3 => {
                let (size, overlap) = (num(1)?, num(2)?);
                if size == 0 || overlap >= size {
                    return Err(format!("arm {s}: needs 0 <= overlap < size"));
                }
                Ok(if parts[0] == "win" {
                    Arm::Window { size, overlap }
                } else {
                    Arm::WindowMean { size, overlap }
                })
            }
            _ => Err(format!(
                "arm {s}: expected trunc:N, members:N, win:N:OVERLAP or winmean:N:OVERLAP"
            )),
        }
    }

    pub fn label(&self) -> String {
        let n = |v: usize| {
            if v == 0 {
                "inf".to_owned()
            } else {
                v.to_string()
            }
        };
        match self {
            Arm::Trunc(c) => format!("trunc:{}", n(*c)),
            Arm::Members(c) => format!("members:{}", n(*c)),
            Arm::Window { size, overlap } => format!("win:{size}:{overlap}"),
            Arm::WindowMean { size, overlap } => format!("winmean:{size}:{overlap}"),
        }
    }

    /// The texts embedded for one symbol.
    pub fn texts(&self, s: &Sym) -> Vec<String> {
        match *self {
            Arm::Trunc(c) => vec![cap(s.full.clone(), c)],
            Arm::Members(c) => vec![cap(s.members.clone().unwrap_or_else(|| s.full.clone()), c)],
            Arm::Window { size, overlap } | Arm::WindowMean { size, overlap } => {
                windows(&s.header, &s.full[s.header.len()..], size, overlap)
            }
        }
    }
}

fn cap(text: String, max: usize) -> String {
    if max == 0 {
        text
    } else {
        margincode_index::cap_chars(text, max)
    }
}

fn chars(s: &str) -> usize {
    s.chars().count()
}

/// See `Arm::Window`. A chunk that fits is one window, the chunk itself.
pub fn windows(header: &str, body: &str, size: usize, overlap: usize) -> Vec<String> {
    if chars(header) + chars(body) <= size {
        return vec![format!("{header}{body}")];
    }
    // A long signature must still leave room for code.
    let room = size.saturating_sub(chars(header)).max(size / 4).max(1);
    let mut pieces: Vec<String> = vec![];
    for line in body.split_inclusive('\n') {
        let cs: Vec<char> = line.chars().collect();
        if cs.len() <= room {
            pieces.push(line.to_owned());
        } else {
            pieces.extend(cs.chunks(room).map(|c| c.iter().collect::<String>()));
        }
    }
    let mut out = vec![];
    let mut i = 0;
    while i < pieces.len() && out.len() < MAX_WINDOWS {
        // Overlap: earlier pieces that fit in `overlap` chars.
        let mut start = i;
        let mut back = 0;
        while start > 0 && !out.is_empty() {
            let l = chars(&pieces[start - 1]);
            if back + l > overlap {
                break;
            }
            back += l;
            start -= 1;
        }
        let mut len = back;
        let mut j = i;
        while j < pieces.len() && (j == i || len + chars(&pieces[j]) <= room) {
            len += chars(&pieces[j]);
            j += 1;
        }
        let text: String = pieces[start..j].concat();
        out.push(margincode_index::cap_chars(format!("{header}{text}"), size));
        i = j;
    }
    out
}

// ---------------------------------------------------------------- symbols

/// One symbol's chunk, as the index would build it (the first symbol of each `(path, qname)`).
#[derive(Debug, Clone)]
pub struct Sym {
    pub path: String,
    pub qname: String,
    pub kind: &'static str,
    /// `chunk_source`: header then source lines, uncapped.
    pub full: String,
    /// `{path}\n{qname}\n{signature}\n`, the start of `full`.
    pub header: String,
    /// For a symbol that contains others: its own lines with each contained symbol reduced to
    /// its first line.
    pub members: Option<String>,
}

pub fn symbols(files: &BTreeMap<String, Vec<u8>>) -> Vec<Sym> {
    let mut out = vec![];
    for (path, bytes) in files {
        let Ok(syms) = margincode_index::extract(path, bytes) else {
            continue;
        };
        let text = std::str::from_utf8(bytes).unwrap_or("");
        let lines: Vec<&str> = text.split_inclusive('\n').collect();
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for s in &syms {
            if !seen.insert(s.qname.as_str()) {
                continue;
            }
            let full = margincode_index::chunk_source(path, s, bytes);
            let header = format!("{path}\n{}\n{}\n", s.qname, s.signature);
            debug_assert!(full.starts_with(&header));
            let inner: Vec<&margincode_index::Symbol> = syms
                .iter()
                .filter(|o| {
                    !std::ptr::eq(*o, s)
                        && o.start_line >= s.start_line
                        && o.end_line <= s.end_line
                        && (o.start_line, o.end_line) != (s.start_line, s.end_line)
                })
                .collect();
            let direct: Vec<&&margincode_index::Symbol> = inner
                .iter()
                .filter(|o| {
                    !inner.iter().any(|p| {
                        !std::ptr::eq(*p, **o)
                            && p.start_line <= o.start_line
                            && p.end_line >= o.end_line
                            && (p.start_line, p.end_line) != (o.start_line, o.end_line)
                    })
                })
                .collect();
            let members = (!direct.is_empty()).then(|| {
                let mut body = String::new();
                for n in s.start_line..=s.end_line {
                    let line = lines.get(n as usize - 1).copied().unwrap_or("");
                    match direct.iter().find(|o| o.start_line <= n && n <= o.end_line) {
                        Some(o) if o.start_line == n => {
                            body.push_str(line);
                            if o.end_line > n {
                                let indent: String =
                                    line.chars().take_while(|c| c.is_whitespace()).collect();
                                body.push_str(&format!("{indent}    …\n"));
                            }
                        }
                        Some(_) => {}
                        None => body.push_str(line),
                    }
                }
                format!("{header}{body}")
            });
            out.push(Sym {
                path: path.clone(),
                qname: s.qname.clone(),
                kind: s.kind.as_str(),
                full,
                header,
                members,
            });
        }
    }
    out
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

/// The files at `rev` the index would parse, read from the object store (no checkout).
pub fn files_at(repo: &Path, rev: &str) -> Result<BTreeMap<String, Vec<u8>>> {
    let commit = git(
        repo,
        &["rev-parse", "--verify", &format!("{rev}^{{commit}}")],
    )?
    .trim()
    .to_owned();
    let list = git(repo, &["ls-tree", "-r", "-z", "--name-only", &commit])?;
    let paths: Vec<String> = list
        .split('\0')
        .filter(|p| {
            !p.is_empty() && indexable(p) && !p.split('/').any(|s| IGNORED_SEGMENTS.contains(&s))
        })
        .map(str::to_owned)
        .collect();
    let mut child = Command::new("git")
        .current_dir(repo)
        .args(["cat-file", "--batch"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("git cat-file: {e}"))?;
    let mut stdin = child.stdin.take().ok_or("git cat-file: no stdin")?;
    let input: String = paths.iter().map(|p| format!("{commit}:{p}\n")).collect();
    let writer = std::thread::spawn(move || stdin.write_all(input.as_bytes()));
    let mut out = BufReader::new(child.stdout.take().ok_or("git cat-file: no stdout")?);
    let mut files = BTreeMap::new();
    for p in &paths {
        let mut head = String::new();
        out.read_line(&mut head).map_err(|e| e.to_string())?;
        let f: Vec<&str> = head.split_whitespace().collect();
        if f.len() != 3 {
            continue; // "<name> missing"
        }
        let size: usize = f[2].parse().map_err(|_| format!("git cat-file: {head}"))?;
        let mut buf = vec![0u8; size + 1];
        out.read_exact(&mut buf).map_err(|e| e.to_string())?;
        buf.pop();
        if f[1] == "blob"
            && size <= margincode_index::MAX_FILE_BYTES
            && std::str::from_utf8(&buf).is_ok()
        {
            files.insert(p.clone(), buf);
        }
    }
    writer
        .join()
        .map_err(|_| "git cat-file writer panicked")?
        .map_err(|e| e.to_string())?;
    child.wait().map_err(|e| e.to_string())?;
    Ok(files)
}

// ---------------------------------------------------------------- embedders

/// An embedding endpoint, called the way `margincode-semantic` calls it.
#[derive(Debug, Clone)]
pub enum Endpoint {
    /// `POST {url}/api/embed` `{"model", "input", "truncate": true}`. With `num_ctx`, also
    /// `"options": {"num_ctx": N, "num_batch": N}`: Ollama embeds at most
    /// min(num_ctx, num_batch) tokens of one input, 2048 with its defaults.
    Ollama {
        url: String,
        model: String,
        num_ctx: Option<u32>,
    },
    /// `POST {url}/embed` `{"inputs", "truncate": true}` (text-embeddings-inference).
    Tei { url: String },
}

impl Endpoint {
    fn agent() -> ureq::Agent {
        ureq::AgentBuilder::new()
            .timeout(Duration::from_secs(900))
            .build()
    }

    /// What the vector cache is keyed by.
    pub fn tag(&self) -> Result<String> {
        Ok(match self {
            Endpoint::Ollama {
                url,
                model,
                num_ctx,
            } => {
                let show: Value = Self::agent()
                    .get(&format!("{url}/api/tags"))
                    .call()
                    .map_err(|e| format!("ollama tags: {e}"))?
                    .into_json()
                    .map_err(|e| e.to_string())?;
                let digest = show["models"]
                    .as_array()
                    .and_then(|ms| {
                        ms.iter().find(|m| {
                            m["name"].as_str() == Some(model)
                                || m["name"].as_str() == Some(&format!("{model}:latest"))
                        })
                    })
                    .and_then(|m| m["digest"].as_str())
                    .ok_or_else(|| format!("ollama has no model {model}"))?
                    .to_owned();
                format!("ollama|{model}|{digest}|num_ctx={num_ctx:?}")
            }
            Endpoint::Tei { url } => {
                let info: Value = Self::agent()
                    .get(&format!("{url}/info"))
                    .call()
                    .map_err(|e| format!("tei info: {e}"))?
                    .into_json()
                    .map_err(|e| e.to_string())?;
                format!(
                    "tei|{}|{}",
                    info["model_id"].as_str().unwrap_or("?"),
                    info["max_input_length"]
                )
            }
        })
    }

    /// Batch size for one request.
    fn batch(&self) -> usize {
        match self {
            Endpoint::Ollama { .. } => 16,
            Endpoint::Tei { .. } => 32,
        }
    }

    /// Vectors, and the tokens the embedder reports it read (Ollama only).
    pub fn embed(&self, texts: &[String]) -> Result<(Vec<Vec<f32>>, Option<u64>)> {
        match self {
            Endpoint::Ollama {
                url,
                model,
                num_ctx,
            } => {
                let mut body =
                    json!({"model": model, "input": texts, "truncate": true, "keep_alive": "30m"});
                if let Some(n) = num_ctx {
                    body["options"] = json!({"num_ctx": n, "num_batch": n});
                }
                let r: Value = Self::agent()
                    .post(&format!("{url}/api/embed"))
                    .send_json(body)
                    .map_err(|e| format!("ollama embed: {e}"))?
                    .into_json()
                    .map_err(|e| e.to_string())?;
                let vs: Vec<Vec<f32>> = serde_json::from_value(r["embeddings"].clone())
                    .map_err(|e| format!("ollama embed: {e}"))?;
                Ok((vs, r["prompt_eval_count"].as_u64()))
            }
            Endpoint::Tei { url } => {
                let r: Value = Self::agent()
                    .post(&format!("{url}/embed"))
                    .send_json(json!({"inputs": texts, "truncate": true}))
                    .map_err(|e| format!("tei embed: {e}"))?
                    .into_json()
                    .map_err(|e| e.to_string())?;
                let vs: Vec<Vec<f32>> =
                    serde_json::from_value(r).map_err(|e| format!("tei embed: {e}"))?;
                Ok((vs, None))
            }
        }
    }

    /// Tokens of one text as the embedder reads it (after its own truncation).
    pub fn tokens(&self, text: &str) -> Result<u64> {
        match self {
            Endpoint::Ollama { .. } => self
                .embed(&[text.to_owned()])?
                .1
                .ok_or_else(|| "ollama sent no prompt_eval_count".to_owned()),
            Endpoint::Tei { url } => {
                let r: Value = Self::agent()
                    .post(&format!("{url}/tokenize"))
                    .send_json(json!({"inputs": text, "add_special_tokens": true}))
                    .map_err(|e| format!("tei tokenize: {e}"))?
                    .into_json()
                    .map_err(|e| e.to_string())?;
                let n = r[0].as_array().map(|a| a.len()).unwrap_or(0) as u64;
                Ok(n)
            }
        }
    }
}

// ---------------------------------------------------------------- vector cache

type Key = [u8; 16];

/// Append-only file of `key (16) | dim (u32 LE) | f32 LE * dim`, normalized vectors.
pub struct Cache {
    file: File,
    at: HashMap<Key, (u64, u32)>,
    tag: String,
}

impl Cache {
    pub fn open(path: &Path, tag: &str) -> Result<Cache> {
        let mut file = File::options()
            .create(true)
            .read(true)
            .append(true)
            .open(path)
            .map_err(|e| format!("{}: {e}", path.display()))?;
        let mut at = HashMap::new();
        let mut bytes = vec![];
        file.read_to_end(&mut bytes).map_err(|e| e.to_string())?;
        let mut i = 0usize;
        while i + 20 <= bytes.len() {
            let key: Key = bytes[i..i + 16].try_into().expect("16 bytes");
            let dim = u32::from_le_bytes(bytes[i + 16..i + 20].try_into().expect("4 bytes"));
            let end = i + 20 + dim as usize * 4;
            if end > bytes.len() {
                break; // a torn last record
            }
            at.insert(key, ((i + 20) as u64, dim));
            i = end;
        }
        Ok(Cache {
            file,
            at,
            tag: tag.to_owned(),
        })
    }

    pub fn key(&self, text: &str) -> Key {
        let mut h = Sha256::new();
        h.update(self.tag.as_bytes());
        h.update([0]);
        h.update(text.as_bytes());
        h.finalize()[..16].try_into().expect("16 bytes")
    }

    pub fn has(&self, k: &Key) -> bool {
        self.at.contains_key(k)
    }

    fn put(&mut self, k: Key, v: &[f32]) -> Result<()> {
        let v = normalized(v);
        let mut rec = Vec::with_capacity(20 + v.len() * 4);
        rec.extend_from_slice(&k);
        rec.extend_from_slice(&(v.len() as u32).to_le_bytes());
        for x in &v {
            rec.extend_from_slice(&x.to_le_bytes());
        }
        let off = self
            .file
            .seek(SeekFrom::End(0))
            .map_err(|e| e.to_string())?;
        self.file.write_all(&rec).map_err(|e| e.to_string())?;
        self.at.insert(k, (off + 20, v.len() as u32));
        Ok(())
    }

    pub fn get(&mut self, k: &Key) -> Result<Vec<f32>> {
        let &(off, dim) = self.at.get(k).ok_or("vector missing from the cache")?;
        let mut buf = vec![0u8; dim as usize * 4];
        self.file
            .seek(SeekFrom::Start(off))
            .map_err(|e| e.to_string())?;
        self.file.read_exact(&mut buf).map_err(|e| e.to_string())?;
        Ok(buf
            .chunks_exact(4)
            .map(|b| f32::from_le_bytes(b.try_into().expect("4 bytes")))
            .collect())
    }

    /// Embeds every text not cached yet, shortest first so a batch holds like lengths.
    pub fn fill(&mut self, ep: &Endpoint, texts: &[String]) -> Result<()> {
        let mut missing: Vec<(Key, &String)> = vec![];
        let mut seen = std::collections::HashSet::new();
        for t in texts {
            let k = self.key(t);
            if !self.has(&k) && seen.insert(k) {
                missing.push((k, t));
            }
        }
        if missing.is_empty() {
            return Ok(());
        }
        missing.sort_by_key(|(_, t)| t.len());
        let started = Instant::now();
        let total = missing.len();
        let mut done = 0;
        let mut i = 0;
        while i < missing.len() {
            // Long inputs go alone or in small batches.
            let mut j = i;
            let mut len = 0;
            while j < missing.len()
                && j - i < ep.batch()
                && (j == i || len + missing[j].1.len() <= 120_000)
            {
                len += missing[j].1.len();
                j += 1;
            }
            let batch: Vec<String> = missing[i..j].iter().map(|(_, t)| (*t).clone()).collect();
            let (vs, _) = ep.embed(&batch)?;
            if vs.len() != batch.len() {
                return Err(format!(
                    "embedder returned {} vectors for {}",
                    vs.len(),
                    batch.len()
                ));
            }
            for ((k, _), v) in missing[i..j].iter().zip(&vs) {
                self.put(*k, v)?;
            }
            done += j - i;
            i = j;
            if done % 2000 < ep.batch() || done == total {
                eprintln!(
                    "  embedded {done}/{total} ({:.0}s)",
                    started.elapsed().as_secs_f64()
                );
            }
        }
        Ok(())
    }
}

fn normalized(v: &[f32]) -> Vec<f32> {
    let n = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if n == 0.0 {
        v.to_vec()
    } else {
        v.iter().map(|x| x / n).collect()
    }
}

fn dot(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

// ---------------------------------------------------------------- scoring

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Metrics {
    /// Share of gold symbols ranked within the first k, for each of `KS`.
    pub recall: Vec<f64>,
    /// 1 / rank of the best-ranked gold symbol.
    pub mrr: f64,
    /// Whether one of the first 8 symbols is in a gold file.
    pub file_hit_8: f64,
}

/// Ranks are 1-based; `scores` ties keep symbol order.
pub fn metrics(
    scores: &[f32],
    syms: &[Sym],
    gold_idx: &[usize],
    gold_files: &[String],
) -> (Metrics, Vec<usize>) {
    let mut order: Vec<usize> = (0..scores.len()).collect();
    order.sort_by(|&a, &b| scores[b].total_cmp(&scores[a]).then(a.cmp(&b)));
    let mut rank = vec![0usize; scores.len()];
    for (r, &i) in order.iter().enumerate() {
        rank[i] = r + 1;
    }
    let ranks: Vec<usize> = gold_idx.iter().map(|&i| rank[i]).collect();
    let n = ranks.len().max(1) as f64;
    let recall = KS
        .iter()
        .map(|&k| ranks.iter().filter(|&&r| r <= k).count() as f64 / n)
        .collect();
    let mrr = ranks.iter().min().map(|&r| 1.0 / r as f64).unwrap_or(0.0);
    let file_hit_8 = order
        .iter()
        .take(8)
        .any(|&i| gold_files.contains(&syms[i].path)) as u8 as f64;
    (
        Metrics {
            recall,
            mrr,
            file_hit_8,
        },
        ranks,
    )
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Row {
    pub task: String,
    pub repo: String,
    pub arm: String,
    pub leak: bool,
    /// A gold symbol's chunk is over `LONG` chars.
    pub long_gold: bool,
    pub gold_max_chars: usize,
    pub symbols: usize,
    pub gold_symbols: usize,
    pub metrics: Metrics,
    pub gold_ranks: Vec<usize>,
}

pub struct Args {
    pub endpoint: Endpoint,
    pub arms: Vec<Arm>,
    pub baseline: Arm,
    pub cache: PathBuf,
    pub out: PathBuf,
    /// Texts per arm whose tokens are measured one by one.
    pub token_sample: usize,
}

/// `margincode-bench chunk-length`: rank every symbol at each task's base by cosine similarity to
/// its raw goal, once per arm.
#[derive(Debug, clap::Args)]
pub struct Cli {
    /// `.jsonl` retrieval sets or Plan 4 spec files/dirs.
    #[arg(required = true)]
    pub tasks: Vec<PathBuf>,
    /// Look each task's repo up by directory name under this directory.
    #[arg(long)]
    pub repos: Option<PathBuf>,
    /// Ollama base URL (e.g. `http://127.0.0.1:11434`).
    #[arg(long)]
    pub ollama: Option<String>,
    #[arg(long, default_value = "embeddinggemma-2")]
    pub model: String,
    /// Send `options: {num_ctx: N, num_batch: N}`; without it Ollama embeds at most 2048 tokens
    /// of an input.
    #[arg(long)]
    pub num_ctx: Option<u32>,
    /// text-embeddings-inference base URL.
    #[arg(long)]
    pub tei: Option<String>,
    /// `trunc:N` (0 or inf: no cap), `members:N`, `win:N:OVERLAP`, `winmean:N:OVERLAP`.
    #[arg(
        long,
        value_delimiter = ',',
        default_value = "trunc:500,trunc:1000,trunc:2000,trunc:4000,trunc:8000,trunc:16000,trunc:inf"
    )]
    pub arms: Vec<String>,
    #[arg(long, default_value = "trunc:2000")]
    pub baseline: String,
    /// Vector cache file (append-only; keyed by embedder and text).
    #[arg(long)]
    pub cache: PathBuf,
    #[arg(long)]
    pub out: PathBuf,
    /// Longest texts per arm whose tokens are measured one by one (0: none).
    #[arg(long, default_value_t = 20)]
    pub token_sample: usize,
}

pub fn main(cli: Cli) -> Result<()> {
    let tasks = crate::retrieval::load_tasks(&cli.tasks, cli.repos.as_deref())?;
    let endpoint = match (cli.ollama, cli.tei) {
        (Some(url), None) => Endpoint::Ollama {
            url: url.trim_end_matches('/').to_owned(),
            model: cli.model,
            num_ctx: cli.num_ctx,
        },
        (None, Some(url)) => Endpoint::Tei {
            url: url.trim_end_matches('/').to_owned(),
        },
        _ => return Err("give exactly one of --ollama and --tei".to_owned()),
    };
    let args = Args {
        endpoint,
        arms: cli
            .arms
            .iter()
            .map(|a| Arm::parse(a))
            .collect::<Result<_>>()?,
        baseline: Arm::parse(&cli.baseline)?,
        cache: cli.cache,
        out: cli.out,
        token_sample: cli.token_sample,
    };
    run(&tasks, &args)?;
    println!("report: {}", args.out.join("report.md").display());
    Ok(())
}

/// Per repo: symbol versions seen (deduped by text) as (kind, chars, gold).
type Lengths = BTreeMap<String, HashMap<Key, (&'static str, usize, bool)>>;

pub fn run(tasks: &[RetrievalTask], args: &Args) -> Result<()> {
    std::fs::create_dir_all(&args.out).map_err(|e| e.to_string())?;
    let tag = args.endpoint.tag()?;
    eprintln!("embedder: {tag}");
    let mut cache = Cache::open(&args.cache, &tag)?;
    let mut arms = args.arms.clone();
    if !arms.contains(&args.baseline) {
        arms.insert(0, args.baseline);
    }
    let mut rows: Vec<Row> = vec![];
    let mut lengths: Lengths = BTreeMap::new();
    // Per arm: unique texts embedded (by key) and their chars, for the index cost.
    let mut cost: BTreeMap<String, HashMap<Key, usize>> = BTreeMap::new();
    let mut samples: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let started = Instant::now();
    for (ti, t) in tasks.iter().enumerate() {
        let g = match gold(&t.repo_path, &t.base_rev, &t.fix_rev, &t.goal) {
            Ok(g) if !g.files.is_empty() && !g.symbols.is_empty() => g,
            Ok(_) => {
                eprintln!("{}: skipped (no gold symbol)", t.name);
                continue;
            }
            Err(e) => {
                eprintln!("{}: skipped ({e})", t.name);
                continue;
            }
        };
        let repo = t
            .repo_path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        let files = files_at(&t.repo_path, &t.base_rev)?;
        let syms = symbols(&files);
        let by_name: HashMap<(&str, &str), usize> = syms
            .iter()
            .enumerate()
            .map(|(i, s)| ((s.path.as_str(), s.qname.as_str()), i))
            .collect();
        let mut gold_idx: Vec<usize> = g
            .symbols
            .iter()
            .filter_map(|gs| by_name.get(&(gs.path.as_str(), gs.qname.as_str())).copied())
            .collect();
        gold_idx.sort();
        gold_idx.dedup();
        if gold_idx.is_empty() {
            eprintln!("{}: skipped (gold symbols not found at base)", t.name);
            continue;
        }
        let lens = lengths.entry(repo.clone()).or_default();
        for (i, s) in syms.iter().enumerate() {
            let k = cache.key(&s.full);
            let e = lens.entry(k).or_insert((s.kind, chars(&s.full), false));
            e.2 |= gold_idx.contains(&i);
        }
        let gold_max_chars = gold_idx
            .iter()
            .map(|&i| chars(&syms[i].full))
            .max()
            .unwrap_or(0);
        eprintln!(
            "[{}/{}] {}: {} symbols, {} gold (longest {} chars), {:.0}s",
            ti + 1,
            tasks.len(),
            t.name,
            syms.len(),
            gold_idx.len(),
            gold_max_chars,
            started.elapsed().as_secs_f64()
        );
        let goal = t.goal.trim().to_owned();
        cache.fill(&args.endpoint, std::slice::from_ref(&goal))?;
        let qv = cache.get(&cache.key(&goal))?;
        for arm in &arms {
            let per_sym: Vec<Vec<String>> = syms.iter().map(|s| arm.texts(s)).collect();
            let all: Vec<String> = per_sym.iter().flatten().cloned().collect();
            cache.fill(&args.endpoint, &all)?;
            let c = cost.entry(arm.label()).or_default();
            let sample = samples.entry(arm.label()).or_default();
            let mut scores = Vec::with_capacity(syms.len());
            for texts in &per_sym {
                let mut vs = vec![];
                for text in texts {
                    let k = cache.key(text);
                    if c.insert(k, chars(text)).is_none() && args.token_sample > 0 {
                        sample.push(text.clone());
                        if sample.len() > 8 * args.token_sample {
                            sample.sort_by_key(|t| std::cmp::Reverse(chars(t)));
                            sample.truncate(args.token_sample);
                        }
                    }
                    vs.push(cache.get(&k)?);
                }
                let s = match arm {
                    Arm::WindowMean { .. } if vs.len() > 1 => {
                        let mut m = vec![0f32; vs[0].len()];
                        for v in &vs {
                            for (a, b) in m.iter_mut().zip(v) {
                                *a += b;
                            }
                        }
                        dot(&qv, &normalized(&m))
                    }
                    _ => vs.iter().map(|v| dot(&qv, v)).fold(f32::MIN, f32::max),
                };
                scores.push(s);
            }
            let (m, ranks) = metrics(&scores, &syms, &gold_idx, &g.files);
            rows.push(Row {
                task: t.name.clone(),
                repo: repo.clone(),
                arm: arm.label(),
                leak: g.leak,
                long_gold: gold_max_chars > LONG,
                gold_max_chars,
                symbols: syms.len(),
                gold_symbols: gold_idx.len(),
                metrics: m,
                gold_ranks: ranks,
            });
        }
    }
    let mut text = String::new();
    for r in &rows {
        text.push_str(&serde_json::to_string(r).map_err(|e| e.to_string())?);
        text.push('\n');
    }
    std::fs::write(args.out.join("rows.jsonl"), text).map_err(|e| e.to_string())?;
    let tokens = measure_tokens(&args.endpoint, &arms, &samples, args.token_sample)?;
    let report = report(&tag, &rows, &arms, &args.baseline, &lengths, &cost, &tokens);
    std::fs::write(args.out.join("report.md"), report).map_err(|e| e.to_string())?;
    Ok(())
}

/// Per arm: (texts measured, median tokens, p90 tokens, max tokens, median chars).
type Tokens = BTreeMap<String, (usize, u64, u64, u64, usize)>;

/// Tokens of the longest texts of each arm, measured one by one. Ollama reports what it read
/// (after its own cut); TEI's tokenizer counts the whole text (TEI then keeps 512).
fn measure_tokens(
    ep: &Endpoint,
    arms: &[Arm],
    samples: &BTreeMap<String, Vec<String>>,
    n: usize,
) -> Result<Tokens> {
    let mut out = BTreeMap::new();
    if n == 0 {
        return Ok(out);
    }
    for arm in arms {
        let Some(texts) = samples.get(&arm.label()) else {
            continue;
        };
        let mut texts: Vec<&String> = texts.iter().collect();
        texts.sort_by_key(|t| std::cmp::Reverse(chars(t)));
        let picked: Vec<&String> = texts.into_iter().take(n).collect();
        if picked.is_empty() {
            continue;
        }
        let mut toks: Vec<u64> = picked.iter().map(|t| ep.tokens(t)).collect::<Result<_>>()?;
        let mut cs: Vec<usize> = picked.iter().map(|t| chars(t)).collect();
        toks.sort();
        cs.sort();
        out.insert(
            arm.label(),
            (
                toks.len(),
                toks[toks.len() / 2],
                toks[(toks.len() * 9 / 10).min(toks.len() - 1)],
                *toks.last().unwrap_or(&0),
                cs[cs.len() / 2],
            ),
        );
    }
    Ok(out)
}

// ---------------------------------------------------------------- report

fn mean(v: &[f64]) -> f64 {
    if v.is_empty() {
        0.0
    } else {
        v.iter().sum::<f64>() / v.len() as f64
    }
}

fn metric(r: &Row, name: &str) -> f64 {
    match name {
        "mrr" => r.metrics.mrr,
        "file@8" => r.metrics.file_hit_8,
        "r@8" => r.metrics.recall[0],
        "r@20" => r.metrics.recall[1],
        "r@40" => r.metrics.recall[2],
        _ => 0.0,
    }
}

const METRICS: [&str; 5] = ["mrr", "r@8", "r@20", "r@40", "file@8"];

/// A named subset of tasks for the report.
type Subset = (&'static str, fn(&Row) -> bool);

#[allow(clippy::too_many_arguments)]
pub fn report(
    tag: &str,
    rows: &[Row],
    arms: &[Arm],
    baseline: &Arm,
    lengths: &Lengths,
    cost: &BTreeMap<String, HashMap<Key, usize>>,
    tokens: &Tokens,
) -> String {
    let mut s = format!("# Embedding chunk length: `{tag}`\n\n");
    let base = baseline.label();
    let tasks: BTreeMap<&str, &Row> = rows
        .iter()
        .filter(|r| r.arm == base)
        .map(|r| (r.task.as_str(), r))
        .collect();
    let mut per_repo: BTreeMap<&str, (usize, usize)> = BTreeMap::new();
    for r in tasks.values() {
        let e = per_repo.entry(r.repo.as_str()).or_default();
        e.0 += 1;
        e.1 += r.long_gold as usize;
    }
    s.push_str(&format!(
        "{} tasks ({} with a gold symbol over {LONG} chars, {} with a leaky goal). Dense-only rank \
         of every symbol at the base revision by cosine similarity to the raw goal.\n\n",
        tasks.len(),
        tasks.values().filter(|r| r.long_gold).count(),
        tasks.values().filter(|r| r.leak).count()
    ));
    s.push_str("| repo | tasks | long gold |\n|---|---:|---:|\n");
    for (repo, (n, l)) in &per_repo {
        s.push_str(&format!("| {repo} | {n} | {l} |\n"));
    }

    s.push_str("\n## Chunk lengths (chars of `chunk_source`, symbol versions deduped by text)\n\n");
    s.push_str("| repo | kind | n | p50 | p90 | p99 | max | >1k | >2k | >4k | >7k | >16k |\n|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n");
    let mut kinds: BTreeMap<(String, String), Vec<usize>> = BTreeMap::new();
    for (repo, m) in lengths {
        for (kind, n, gold) in m.values() {
            kinds
                .entry((repo.clone(), kind.to_string()))
                .or_default()
                .push(*n);
            kinds
                .entry((repo.clone(), "all".into()))
                .or_default()
                .push(*n);
            kinds
                .entry(("all".into(), kind.to_string()))
                .or_default()
                .push(*n);
            kinds
                .entry(("all".into(), "all".into()))
                .or_default()
                .push(*n);
            if *gold {
                kinds
                    .entry(("all".into(), "GOLD".into()))
                    .or_default()
                    .push(*n);
                kinds
                    .entry((repo.clone(), "GOLD".into()))
                    .or_default()
                    .push(*n);
            }
        }
    }
    for ((repo, kind), v) in &mut kinds {
        v.sort();
        let q = |p: f64| v[((v.len() as f64 * p) as usize).min(v.len() - 1)];
        let over = |c: usize| {
            format!(
                "{:.1}%",
                100.0 * v.iter().filter(|&&x| x > c).count() as f64 / v.len() as f64
            )
        };
        s.push_str(&format!(
            "| {repo} | {kind} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            v.len(),
            q(0.5),
            q(0.9),
            q(0.99),
            v.last().unwrap_or(&0),
            over(1000),
            over(2000),
            over(4000),
            over(7000),
            over(16000)
        ));
    }

    if !tokens.is_empty() {
        s.push_str(
            "\n## Tokens of the longest texts of each arm\n\nMeasured one by one. Ollama counts \
             what it read, after its own cut; TEI's tokenizer counts the whole text, of which TEI \
             embeds the first 512 tokens.\n\n",
        );
        s.push_str("| arm | texts | median chars | median tokens | p90 tokens | max tokens |\n|---|---:|---:|---:|---:|---:|\n");
        for arm in arms {
            if let Some((n, med, p90, max, mc)) = tokens.get(&arm.label()) {
                s.push_str(&format!(
                    "| {} | {n} | {mc} | {med} | {p90} | {max} |\n",
                    arm.label()
                ));
            }
        }
    }

    let subsets: [Subset; 4] = [
        ("all tasks", |_| true),
        ("tasks with a long gold symbol", |r| r.long_gold),
        ("tasks without one", |r| !r.long_gold),
        ("tasks without a leaky goal", |r| !r.leak),
    ];
    for (name, keep) in subsets {
        let n = tasks.values().filter(|r| keep(r)).count();
        if n == 0 {
            continue;
        }
        s.push_str(&format!("\n## Means: {name} (n = {n})\n\n| arm |"));
        for m in METRICS {
            s.push_str(&format!(" {m} |"));
        }
        s.push_str("\n|---|");
        s.push_str(&"---:|".repeat(METRICS.len()));
        s.push('\n');
        for arm in arms {
            let l = arm.label();
            let rs: Vec<&Row> = rows.iter().filter(|r| r.arm == l && keep(r)).collect();
            s.push_str(&format!("| {l} |"));
            for m in METRICS {
                s.push_str(&format!(
                    " {:.3} |",
                    mean(&rs.iter().map(|r| metric(r, m)).collect::<Vec<_>>())
                ));
            }
            s.push('\n');
        }
    }

    for (name, keep) in &subsets[..2] {
        s.push_str(&format!(
            "\n## Paired against `{base}`: {name}\n\nPer task, arm minus `{base}`: mean, 95% \
             bootstrap interval, and tasks better / worse / tied.\n\n| arm | metric | mean diff | 95% CI | better | worse | tied |\n|---|---|---:|---|---:|---:|---:|\n"
        ));
        for arm in arms {
            let l = arm.label();
            if l == base {
                continue;
            }
            for m in METRICS {
                let mut d = vec![];
                for r in rows.iter().filter(|r| r.arm == l && keep(r)) {
                    if let Some(b) = tasks.get(r.task.as_str()) {
                        d.push(metric(r, m) - metric(b, m));
                    }
                }
                if d.is_empty() {
                    continue;
                }
                let ci = bootstrap_mean_ci(&d, 7)
                    .map(|(a, b)| format!("[{a:+.3}, {b:+.3}]"))
                    .unwrap_or_default();
                s.push_str(&format!(
                    "| {l} | {m} | {:+.3} | {ci} | {} | {} | {} |\n",
                    mean(&d),
                    d.iter().filter(|x| **x > 1e-9).count(),
                    d.iter().filter(|x| **x < -1e-9).count(),
                    d.iter().filter(|x| x.abs() <= 1e-9).count()
                ));
            }
        }
    }

    s.push_str("\n## MRR by repo\n\n| arm |");
    for repo in per_repo.keys() {
        s.push_str(&format!(" {repo} |"));
    }
    s.push_str("\n|---|");
    s.push_str(&"---:|".repeat(per_repo.len()));
    s.push('\n');
    for arm in arms {
        let l = arm.label();
        s.push_str(&format!("| {l} |"));
        for repo in per_repo.keys() {
            let v: Vec<f64> = rows
                .iter()
                .filter(|r| r.arm == l && r.repo == *repo)
                .map(|r| r.metrics.mrr)
                .collect();
            s.push_str(&format!(" {:.3} |", mean(&v)));
        }
        s.push('\n');
    }

    s.push_str("\n## Index cost (unique texts embedded over all tasks)\n\n| arm | vectors | chars | chars vs base |\n|---|---:|---:|---:|\n");
    let base_chars: usize = cost
        .get(&base)
        .map(|m| m.values().sum())
        .unwrap_or(1)
        .max(1);
    for arm in arms {
        if let Some(m) = cost.get(&arm.label()) {
            let c: usize = m.values().sum();
            s.push_str(&format!(
                "| {} | {} | {c} | {:.2}x |\n",
                arm.label(),
                m.len(),
                c as f64 / base_chars as f64
            ));
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arms_parse_and_label_round_trip() {
        for a in [
            "trunc:2000",
            "trunc:inf",
            "members:4000",
            "win:2000:200",
            "winmean:1500:0",
        ] {
            assert_eq!(Arm::parse(a).unwrap().label(), a);
        }
        assert_eq!(Arm::parse("trunc:0").unwrap(), Arm::Trunc(0));
        assert!(Arm::parse("win:100:100").is_err());
        assert!(Arm::parse("slice:5").is_err());
    }

    fn files(path: &str, src: &str) -> BTreeMap<String, Vec<u8>> {
        BTreeMap::from([(path.to_owned(), src.as_bytes().to_vec())])
    }

    #[test]
    fn trunc_at_the_cap_is_the_index_chunk() {
        let cap = margincode_index::MAX_EMBED_CHARS;
        let src = format!("def big():\n    return \"{}\"\n", "x".repeat(cap + 3_000));
        let syms = symbols(&files("big.py", &src));
        let text = &Arm::Trunc(cap).texts(&syms[0])[0];
        assert_eq!(text.chars().count(), cap);
        assert!(text.ends_with('…'));
        assert!(text.starts_with("big.py\nbig\n"));
        assert_eq!(Arm::Trunc(0).texts(&syms[0])[0], syms[0].full);
    }

    #[test]
    fn windows_repeat_the_header_and_cover_the_body() {
        let body: String = (0..100)
            .map(|i| format!("    line_{i:03} = {i}\n"))
            .collect();
        let header = "a.py\nf\ndef f():\n";
        let ws = windows(header, &body, 300, 60);
        assert!(ws.len() > 1);
        for w in &ws {
            assert!(w.starts_with(header), "{w}");
            assert!(w.chars().count() <= 300);
        }
        for i in 0..100 {
            let l = format!("line_{i:03} ");
            assert!(ws.iter().any(|w| w.contains(&l)), "{l} lost");
        }
        // The second window starts with the end of the first.
        let last_of_first = ws[0].lines().last().unwrap();
        assert!(ws[1][header.len()..].contains(last_of_first));
        assert_eq!(
            windows(header, "x\n", 300, 60),
            vec![format!("{header}x\n")]
        );
    }

    #[test]
    fn members_keep_the_class_and_the_first_line_of_each_method() {
        let src = "class A:\n    \"\"\"Doc.\"\"\"\n    x = 1\n\n    def f(self):\n        return 1\n\n    def g(self):\n        return 2\n";
        let syms = symbols(&files("a.py", src));
        let a = syms.iter().find(|s| s.qname == "A").unwrap();
        let m = a.members.as_deref().unwrap();
        assert!(m.contains("Doc.") && m.contains("def f(self):") && m.contains("def g(self):"));
        assert!(!m.contains("return 1"), "{m}");
        let f = syms.iter().find(|s| s.qname == "A.f").unwrap();
        assert!(f.members.is_none());
    }

    #[test]
    fn metrics_rank_gold_and_ties_keep_order() {
        let syms: Vec<Sym> = ["a.py", "b.py", "c.py"]
            .iter()
            .map(|p| Sym {
                path: (*p).into(),
                qname: "f".into(),
                kind: "function",
                full: String::new(),
                header: String::new(),
                members: None,
            })
            .collect();
        let (m, ranks) = metrics(&[0.1, 0.9, 0.1], &syms, &[2], &["c.py".into()]);
        assert_eq!(ranks, vec![3]);
        assert!((m.mrr - 1.0 / 3.0).abs() < 1e-9);
        assert_eq!(m.recall, vec![1.0, 1.0, 1.0]);
        assert_eq!(m.file_hit_8, 1.0);
    }
}
