//! Scale and latency (Plan 6, E2): imports one repo into a bench daemon and times what agents
//! wait on. No agent and no money; a load generator stands in for them.
//!
//! - import: `POST /v1/repos` (clone and index), by repo size;
//! - latency: for `samples` tasks, the start pack, `reps` context packs, two checkpoints after a
//!   one-line edit (the second incremental), and `why` on a line (cold, then cached);
//! - levels: `n` agents at once run a whole task (start, write a file, record, checkpoint,
//!   context, submit, validate, review, merge), for each `n` in `agents`. Merges take a global
//!   lock, so this measures the queue as well as the work.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;
use std::time::Instant;

use serde::Serialize;
use serde_json::json;

use crate::Result;
use crate::runner::Daemon;

const AGENT: &str = "scale-agent";
const CODE_EXT: &[&str] = &["py", "rs", "ts", "tsx"];

pub struct ScaleArgs {
    /// The repo's name in the daemon.
    pub name: String,
    /// The clone, as this process sees it.
    pub repo: PathBuf,
    /// The same clone, as the daemon sees it.
    pub git_source: String,
    pub out: PathBuf,
    pub agents: Vec<usize>,
    pub samples: usize,
    pub reps: usize,
    /// Reads the daemon's resident memory from `/proc` when given.
    pub daemon_pid: Option<u32>,
    /// Reads the database size with `psql` when given.
    pub database_url: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Stat {
    pub n: usize,
    pub p50_ms: f64,
    pub p95_ms: f64,
    pub max_ms: f64,
}

#[derive(Debug, Serialize)]
pub struct Level {
    pub agents: usize,
    pub merged: usize,
    pub wall_s: f64,
    pub merges_per_s: f64,
    pub ops: BTreeMap<String, Stat>,
    pub errors: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ScaleReport {
    pub repo: String,
    pub files: usize,
    pub code_files: usize,
    pub indexed_files: u64,
    pub import_s: f64,
    pub latency: BTreeMap<String, Stat>,
    pub levels: Vec<Level>,
    pub daemon_rss_kb: Option<u64>,
    pub daemon_peak_rss_kb: Option<u64>,
    pub database_mb: Option<f64>,
    /// Every task's workspace is a full checkout and stays after merge.
    pub workspaces: usize,
    pub workspaces_mb: Option<f64>,
}

/// Nearest-rank percentiles of durations in milliseconds.
pub fn stat(ms: &[f64]) -> Option<Stat> {
    if ms.is_empty() {
        return None;
    }
    let mut v = ms.to_vec();
    v.sort_by(f64::total_cmp);
    let at = |p: f64| v[((p * v.len() as f64).ceil() as usize).clamp(1, v.len()) - 1];
    Some(Stat {
        n: v.len(),
        p50_ms: at(0.5),
        p95_ms: at(0.95),
        max_ms: v[v.len() - 1],
    })
}

/// Durations by operation, shared between agent threads.
#[derive(Default)]
struct Timings(Mutex<BTreeMap<String, Vec<f64>>>);

impl Timings {
    fn time<T>(&self, op: &str, f: impl FnOnce() -> Result<T>) -> Result<T> {
        let t = Instant::now();
        let out = f();
        if out.is_ok() {
            let ms = t.elapsed().as_secs_f64() * 1000.0;
            self.0
                .lock()
                .unwrap()
                .entry(op.to_owned())
                .or_default()
                .push(ms);
        }
        out.map_err(|e| format!("{op}: {e}"))
    }

    fn stats(self) -> BTreeMap<String, Stat> {
        self.0
            .into_inner()
            .unwrap()
            .into_iter()
            .filter_map(|(op, ms)| Some((op, stat(&ms)?)))
            .collect()
    }
}

fn tracked_files(repo: &Path) -> Result<Vec<String>> {
    let out = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(["ls-files", "-z"])
        .output()
        .map_err(|e| format!("git ls-files: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "git ls-files in {}: {}",
            repo.display(),
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    let mut files: Vec<String> = String::from_utf8_lossy(&out.stdout)
        .split('\0')
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .collect();
    files.sort();
    Ok(files)
}

fn is_code(path: &str) -> bool {
    path.rsplit_once('.')
        .is_some_and(|(_, ext)| CODE_EXT.contains(&ext))
}

/// `n` paths spread evenly over `files` (deterministic).
fn spread(files: &[String], n: usize) -> Vec<String> {
    if files.is_empty() || n == 0 {
        return vec![];
    }
    let step = (files.len() as f64 / n as f64).max(1.0);
    (0..n.min(files.len()))
        .map(|i| files[(i as f64 * step) as usize].clone())
        .collect()
}

/// The comment marker for a source file's language.
fn comment(path: &str) -> &'static str {
    if path.ends_with(".py") { "#" } else { "//" }
}

struct Started {
    id: String,
    generation: i64,
    workspace: PathBuf,
}

fn start(d: &Daemon, repo: &str, title: &str, goal: &str, session: &str) -> Result<Started> {
    let v = d.post(
        "/v1/tasks",
        json!({"repo": repo, "title": title, "goal": goal, "agent": AGENT, "session": session}),
        None,
    )?;
    let t = &v["task"];
    Ok(Started {
        id: t["id"].as_str().ok_or("no task id")?.to_owned(),
        generation: t["generation"].as_i64().unwrap_or(1),
        workspace: PathBuf::from(t["workspace_path"].as_str().ok_or("no workspace path")?),
    })
}

fn append_line(ws: &Path, path: &str, text: &str) -> Result<()> {
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new()
        .append(true)
        .open(ws.join(path))
        .map_err(|e| format!("{path}: {e}"))?;
    writeln!(f, "{} {text}", comment(path)).map_err(|e| format!("{path}: {e}"))
}

/// The latencies, and the directory that holds the repo's workspaces.
fn latency(
    d: &Daemon,
    a: &ScaleArgs,
    samples: &[String],
) -> Result<(BTreeMap<String, Stat>, Option<PathBuf>)> {
    let t = Timings::default();
    let mut ws_root = None;
    for (i, path) in samples.iter().enumerate() {
        let stem = Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("it");
        let goal = format!("{stem} mishandles empty input; make it return early instead");
        let s = t.time("start", || {
            start(
                d,
                &a.name,
                &format!("latency {i}"),
                &goal,
                &format!("lat-{i}"),
            )
        })?;
        for r in 0..a.reps {
            t.time("context", || {
                d.get(&format!(
                    "/v1/tasks/{}/context?budget=800&session=lat-{i}-{r}",
                    s.id
                ))
            })?;
        }
        ws_root = s.workspace.parent().map(Path::to_path_buf);
        let path_q = path.replace('%', "%25").replace(' ', "%20");
        for (op, n) in [("checkpoint", 1), ("checkpoint_again", 2)] {
            append_line(&s.workspace, path, &format!("scale edit {n}"))?;
            t.time(op, || {
                d.post(
                    &format!("/v1/tasks/{}/checkpoint", s.id),
                    json!({}),
                    Some(s.generation),
                )
            })?;
        }
        for op in ["why", "why_cached"] {
            t.time(op, || {
                d.get(&format!("/v1/repos/{}/why?path={path_q}&line=1", a.name))
            })?;
        }
    }
    Ok((t.stats(), ws_root))
}

/// One agent's whole task, from start to merge.
fn agent_task(
    d: &Daemon,
    a: &ScaleArgs,
    level: usize,
    n: usize,
    ext: &str,
    t: &Timings,
) -> Result<()> {
    let title = format!("scale {level}-{n}");
    let s = t.time("start", || {
        start(
            d,
            &a.name,
            &title,
            &format!("Add scale helper {level}-{n}"),
            &format!("scale-{level}-{n}"),
        )
    })?;
    let rel = format!("scale_l{level}_a{n}.{ext}");
    let body = if ext == "py" {
        format!("def scale_{level}_{n}(x):\n    return x + {n}\n")
    } else {
        format!("// scale helper {level}-{n}\n")
    };
    std::fs::write(s.workspace.join(&rel), body).map_err(|e| format!("{rel}: {e}"))?;
    let task = |p: &str| format!("/v1/tasks/{}/{p}", s.id);
    let g = Some(s.generation);
    t.time("record", || {
        d.post(
            &task("records"),
            json!({"kind": "decision", "summary": format!("helper {level}-{n} gets its own file"),
                   "agent": AGENT, "anchors": [{"path": rel}]}),
            g,
        )
    })?;
    t.time("checkpoint", || d.post(&task("checkpoint"), json!({}), g))?;
    t.time("context", || {
        d.get(&format!(
            "{}?budget=800&session=scale-{level}-{n}",
            task("context")
        ))
    })?;
    t.time("submit", || {
        d.post(&task("submit"), json!({"agent": AGENT}), g)
    })?;
    t.time("validate", || {
        d.post(
            &task("validate"),
            json!({"agent": AGENT, "command": "true", "exit_code": 0}),
            g,
        )
    })?;
    t.time("review", || {
        d.post(
            &task("review"),
            json!({"reviewer": "scale-reviewer", "verdict": "approve"}),
            None,
        )
    })?;
    let merged = t.time("merge", || {
        d.post(&task("merge"), json!({"agent": "scale-merger"}), None)
    })?;
    if merged["status"] != "merged" {
        return Err(format!("{title} did not merge: {merged}"));
    }
    Ok(())
}

fn level(d: &Daemon, a: &ScaleArgs, agents: usize, ext: &str) -> Level {
    let t = Timings::default();
    let errors = Mutex::new(vec![]);
    let started = Instant::now();
    std::thread::scope(|scope| {
        for n in 0..agents {
            let (t, errors) = (&t, &errors);
            scope.spawn(move || {
                if let Err(e) = agent_task(d, a, agents, n, ext, t) {
                    errors.lock().unwrap().push(format!("agent {n}: {e}"));
                }
            });
        }
    });
    let wall_s = started.elapsed().as_secs_f64();
    let ops = t.stats();
    let merged = ops.get("merge").map_or(0, |s| s.n);
    Level {
        agents,
        merged,
        wall_s,
        merges_per_s: merged as f64 / wall_s,
        ops,
        errors: errors.into_inner().unwrap(),
    }
}

/// `(VmRSS, VmHWM)` in kB.
fn memory(pid: u32) -> (Option<u64>, Option<u64>) {
    let Ok(status) = std::fs::read_to_string(format!("/proc/{pid}/status")) else {
        return (None, None);
    };
    let field = |name: &str| {
        status
            .lines()
            .find_map(|l| l.strip_prefix(name))
            .and_then(|v| v.split_whitespace().next()?.parse().ok())
    };
    (field("VmRSS:"), field("VmHWM:"))
}

/// Disk used under `dir`, in MB (`du`).
fn disk_mb(dir: &Path) -> Option<f64> {
    let out = Command::new("du").arg("-sk").arg(dir).output().ok()?;
    let kb: f64 = String::from_utf8_lossy(&out.stdout)
        .split_whitespace()
        .next()?
        .parse()
        .ok()?;
    Some(kb / 1024.0)
}

fn database_mb(url: &str) -> Option<f64> {
    let out = Command::new("psql")
        .args([url, "-tAc", "SELECT pg_database_size(current_database())"])
        .output()
        .ok()?;
    let bytes: f64 = String::from_utf8_lossy(&out.stdout).trim().parse().ok()?;
    Some(bytes / 1_048_576.0)
}

pub fn run(d: &Daemon, a: &ScaleArgs) -> Result<ScaleReport> {
    std::fs::create_dir_all(&a.out).map_err(|e| format!("{}: {e}", a.out.display()))?;
    let files = tracked_files(&a.repo)?;
    let code: Vec<String> = files.iter().filter(|f| is_code(f)).cloned().collect();
    let ext = code
        .iter()
        .filter_map(|f| f.rsplit_once('.').map(|(_, e)| e))
        .fold(BTreeMap::<&str, usize>::new(), |mut m, e| {
            *m.entry(e).or_default() += 1;
            m
        })
        .into_iter()
        .max_by_key(|(_, n)| *n)
        .map_or("py", |(e, _)| e)
        .to_owned();

    eprintln!(
        "{}: importing {} files ({} code)",
        a.name,
        files.len(),
        code.len()
    );
    let t = Instant::now();
    let imported = d.post(
        "/v1/repos",
        json!({"name": a.name, "git_source": a.git_source}),
        None,
    )?;
    let import_s = t.elapsed().as_secs_f64();
    let indexed_files = imported["indexed_files"].as_u64().unwrap_or(0);
    eprintln!("{}: imported in {import_s:.1} s", a.name);

    let samples = spread(&code, a.samples);
    let (latency, ws_root) = latency(d, a, &samples)?;
    let mut levels = vec![];
    for &n in &a.agents {
        let l = level(d, a, n, &ext);
        eprintln!(
            "{}: {n} agents, {} merged in {:.1} s, {} errors",
            a.name,
            l.merged,
            l.wall_s,
            l.errors.len()
        );
        levels.push(l);
    }
    let (daemon_rss_kb, daemon_peak_rss_kb) = a.daemon_pid.map_or((None, None), memory);
    let report = ScaleReport {
        repo: a.name.clone(),
        files: files.len(),
        code_files: code.len(),
        indexed_files,
        import_s,
        latency,
        levels,
        daemon_rss_kb,
        daemon_peak_rss_kb,
        database_mb: a.database_url.as_deref().and_then(database_mb),
        workspaces: samples.len() + a.agents.iter().sum::<usize>(),
        workspaces_mb: ws_root.as_deref().and_then(disk_mb),
    };
    let json = serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?;
    std::fs::write(a.out.join("scale.json"), json).map_err(|e| e.to_string())?;
    std::fs::write(a.out.join("report.md"), render(&report)).map_err(|e| e.to_string())?;
    Ok(report)
}

fn row(op: &str, s: &Stat) -> String {
    format!(
        "| {op} | {} | {:.0} | {:.0} | {:.0} |\n",
        s.n, s.p50_ms, s.p95_ms, s.max_ms
    )
}

pub fn render(r: &ScaleReport) -> String {
    let mut md = format!(
        "# Scale: {}\n\n{} files tracked, {} code files, {} indexed. Import and index: {:.1} s.\n",
        r.repo, r.files, r.code_files, r.indexed_files, r.import_s
    );
    if let (Some(rss), Some(peak)) = (r.daemon_rss_kb, r.daemon_peak_rss_kb) {
        md += &format!(
            "Daemon memory at the end: {:.0} MB (peak {:.0} MB).\n",
            rss as f64 / 1024.0,
            peak as f64 / 1024.0
        );
    }
    if let Some(mb) = r.database_mb {
        md += &format!("Database size at the end: {mb:.0} MB.\n");
    }
    if let Some(mb) = r.workspaces_mb {
        md += &format!(
            "Workspaces on disk: {mb:.0} MB for {} tasks (merged ones stay).\n",
            r.workspaces
        );
    }
    md += "\n## Latency, one agent at a time (ms)\n\n| op | n | p50 | p95 | max |\n|---|---|---|---|---|\n";
    for (op, s) in &r.latency {
        md += &row(op, s);
    }
    for l in &r.levels {
        md += &format!(
            "\n## {} agents at once: {} merged in {:.1} s ({:.2} merges/s)\n\n| op | n | p50 | p95 | max |\n|---|---|---|---|---|\n",
            l.agents, l.merged, l.wall_s, l.merges_per_s
        );
        for (op, s) in &l.ops {
            md += &row(op, s);
        }
        for e in l.errors.iter().take(10) {
            md += &format!("\nerror: {e}\n");
        }
    }
    md
}
