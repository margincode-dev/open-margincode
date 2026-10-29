//! `scholion-bench run`: the session plan, the budget cap, resumable result rows and the report.

use std::fmt::Write as _;
use std::io::Write as _;
use std::path::{Path, PathBuf};

use crate::Result;
use crate::events::parse_stream;
use crate::metrics::compute_priced;
use crate::report::{Meta, Row, Status, read_rows, render_report};
use crate::runner::{Arm, Daemon, RunConfig, run_session};
use crate::spec::TaskSpec;
use crate::stats::{Rng, median};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Planned {
    pub rep: u32,
    pub task: usize,
    pub arm: Arm,
}

/// Rep-major, then task; within each (rep, task) the arms in a seeded random order, so drift over
/// the run hits every arm equally.
pub fn plan(n_tasks: usize, arms: &[Arm], reps: u32, seed: u64) -> Vec<Planned> {
    let mut rng = Rng::new(seed);
    let mut out = vec![];
    for rep in 1..=reps {
        for task in 0..n_tasks {
            let mut order = arms.to_vec();
            rng.shuffle(&mut order);
            out.extend(order.into_iter().map(|arm| Planned { rep, task, arm }));
        }
    }
    out
}

pub struct RunArgs {
    pub out: PathBuf,
    pub arms: Vec<Arm>,
    pub reps: u32,
    pub budget_usd: f64,
    pub seed: u64,
    /// Without it nothing is launched (as `dry_run`).
    pub confirm: bool,
    pub dry_run: bool,
    /// Expected cost of one session, for the plan's estimate (a pilot's median is better).
    pub est_session_usd: Option<f64>,
    /// A pilot run's results directory: its median cost per arm is the estimate.
    pub pilot: Option<PathBuf>,
    pub meta: Meta,
}

pub struct RunReport {
    pub rows: Vec<Row>,
    pub plan_text: String,
    /// 0: everything ran; 2: sessions were skipped for budget; 3: dry run.
    pub exit_code: i32,
}

fn arm_estimates(pilot: Option<&Path>) -> Vec<(Arm, f64)> {
    let Some(p) = pilot else {
        return vec![];
    };
    let Ok(rows) = read_rows(&p.join("rows.jsonl")) else {
        return vec![];
    };
    Arm::KNOWN
        .into_iter()
        .filter_map(|a| {
            let costs: Vec<f64> = rows
                .iter()
                .filter(|r| r.arm == a.as_str() && r.status != Status::SkippedBudget)
                .map(|r| r.cost_usd)
                .collect();
            Some((a, median(&costs)?))
        })
        .collect()
}

pub fn plan_text(
    specs: &[TaskSpec],
    args: &RunArgs,
    cfg: &RunConfig,
    sessions: &[Planned],
) -> String {
    let mut t = String::new();
    let _ = writeln!(
        t,
        "plan: {} tasks × {} arms × {} reps = {} sessions (seed {})",
        specs.len(),
        args.arms.len(),
        args.reps,
        sessions.len(),
        args.seed
    );
    let mut models: Vec<&str> = specs.iter().map(|s| s.model.as_str()).collect();
    models.sort();
    models.dedup();
    let _ = writeln!(t, "model: {}", models.join(", "));
    for arm in &args.arms {
        let n = sessions.iter().filter(|p| p.arm == *arm).count();
        let warn = if n < 30 {
            " (below 30: no significance claim)"
        } else {
            ""
        };
        let _ = writeln!(t, "  {}: {n} sessions{warn}", arm.as_str());
    }
    let pilot = arm_estimates(args.pilot.as_deref());
    let est: Option<f64> = if !pilot.is_empty() {
        Some(
            sessions
                .iter()
                .map(|p| {
                    pilot
                        .iter()
                        .find(|(a, _)| *a == p.arm)
                        .map_or(cfg.session_cap_usd, |(_, c)| *c)
                })
                .sum(),
        )
    } else {
        args.est_session_usd.map(|e| e * sessions.len() as f64)
    };
    let worst = cfg.session_cap_usd * sessions.len() as f64;
    match est {
        Some(e) => {
            let _ = writeln!(
                t,
                "estimated cost: ${e:.2}{} (worst case ${worst:.2} at the ${:.2} per-session cap)",
                if pilot.is_empty() {
                    ""
                } else {
                    " from the pilot's medians"
                },
                cfg.session_cap_usd
            );
        }
        None => {
            let _ = writeln!(
                t,
                "estimated cost: unknown (run a pilot or pass --est-session-usd); worst case ${worst:.2} at the ${:.2} per-session cap",
                cfg.session_cap_usd
            );
        }
    }
    let _ = writeln!(
        t,
        "budget: ${:.2}; sessions stop launching once spent + one session's cap would exceed it",
        args.budget_usd
    );
    t
}

fn append_row(path: &Path, row: &Row) -> Result<()> {
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| format!("{}: {e}", path.display()))?;
    writeln!(f, "{}", serde_json::to_string(row).expect("json")).map_err(|e| e.to_string())
}

/// The whole run. Sessions already in `out/rows.jsonl` (an interrupted run) are not repeated and
/// their cost counts toward the budget.
pub fn run(specs: &[TaskSpec], args: &RunArgs, cfg: &RunConfig) -> Result<RunReport> {
    let sessions = plan(specs.len(), &args.arms, args.reps, args.seed);
    let text = plan_text(specs, args, cfg, &sessions);
    if args.dry_run || !args.confirm {
        return Ok(RunReport {
            rows: vec![],
            plan_text: text,
            exit_code: 3,
        });
    }
    if args.budget_usd <= 0.0 {
        return Err("--budget-usd must be positive".into());
    }
    if args.arms.iter().any(|a| a.uses_av()) {
        let url = cfg.av_url.as_deref().ok_or("the av arms need --av-url")?;
        Daemon::new(url, cfg.av_token.as_deref()).preflight()?;
    }
    std::fs::create_dir_all(&args.out).map_err(|e| format!("{}: {e}", args.out.display()))?;
    let rows_path = args.out.join("rows.jsonl");
    let mut rows = if rows_path.exists() {
        read_rows(&rows_path)?
    } else {
        vec![]
    };
    // sessions an earlier run skipped for budget are tried again (with this run's budget)
    if rows.iter().any(|r| r.status == Status::SkippedBudget) {
        rows.retain(|r| r.status != Status::SkippedBudget);
        let kept: String = rows
            .iter()
            .map(|r| serde_json::to_string(r).expect("json") + "\n")
            .collect();
        std::fs::write(&rows_path, kept).map_err(|e| e.to_string())?;
    }
    let mut spent: f64 = rows.iter().map(|r| r.cost_usd).sum();
    let run_id = args
        .out
        .file_name()
        .map(|n| {
            n.to_string_lossy()
                .replace(|c: char| !c.is_ascii_alphanumeric(), "")
        })
        .filter(|n| !n.is_empty())
        .unwrap_or_else(|| "run".into())
        .to_lowercase();
    std::fs::write(
        args.out.join("meta.json"),
        serde_json::to_vec_pretty(&args.meta).expect("json"),
    )
    .map_err(|e| e.to_string())?;
    let mut over_budget = false;
    for p in &sessions {
        let spec = &specs[p.task];
        let done = rows
            .iter()
            .any(|r| r.task == spec.name && r.arm == p.arm.as_str() && r.rep == p.rep);
        if done {
            continue;
        }
        let row = if over_budget || spent + cfg.session_cap_usd > args.budget_usd + 1e-9 {
            over_budget = true;
            Row {
                task: spec.name.clone(),
                arm: p.arm.as_str().into(),
                rep: p.rep,
                status: Status::SkippedBudget,
                reason: None,
                success: false,
                metrics: None,
                wall_secs: 0.0,
                cost_usd: 0.0,
            }
        } else {
            eprintln!(
                "scholion-bench: {} · {} · rep {} (spent ${spent:.2} of ${:.2})",
                spec.name,
                p.arm.as_str(),
                p.rep,
                args.budget_usd
            );
            let o = run_session(spec, p.arm, p.rep, &run_id, cfg)?;
            let events = std::fs::read_to_string(&o.events_path).unwrap_or_default();
            let m = compute_priced(
                spec,
                &parse_stream(&events),
                cfg.session_cap_usd,
                cfg.prices.as_ref(),
            );
            spent += m.cost_usd;
            let reason = o.invalid_reason.clone().or_else(|| {
                o.verify_timed_out
                    .then(|| "verify_cmd timed out".to_owned())
            });
            Row {
                task: spec.name.clone(),
                arm: p.arm.as_str().into(),
                rep: p.rep,
                status: if o.valid { Status::Ok } else { Status::Invalid },
                reason,
                success: o.success(),
                cost_usd: m.cost_usd,
                metrics: Some(m),
                wall_secs: o.wall_secs,
            }
        };
        append_row(&rows_path, &row)?;
        rows.push(row);
    }
    std::fs::write(args.out.join("report.md"), render_report(&rows, &args.meta))
        .map_err(|e| e.to_string())?;
    Ok(RunReport {
        rows,
        plan_text: text,
        exit_code: if over_budget { 2 } else { 0 },
    })
}
