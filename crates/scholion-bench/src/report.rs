//! Result rows (`rows.jsonl`) and the Markdown report.

use std::collections::BTreeSet;
use std::fmt::Write as _;

use serde::{Deserialize, Serialize};

use crate::metrics::SessionMetrics;
use crate::stats::{bootstrap_median_ci, median, relative_change};

pub const ARMS: [&str; 3] = ["baseline", "av_no_packs", "av"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    /// Ran and counts.
    Ok,
    /// Ran but does not count (setup failed, hooks never fired, workspace mismatch, …).
    Invalid,
    /// Not launched: the budget would have been exceeded.
    SkippedBudget,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Row {
    pub task: String,
    pub arm: String,
    pub rep: u32,
    pub status: Status,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// `verify_cmd` exited 0 after the session.
    #[serde(default)]
    pub success: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<SessionMetrics>,
    /// The `claude` process's wall time, hooks and packer included.
    #[serde(default)]
    pub wall_secs: f64,
    /// Money spent, whether or not the session counts.
    #[serde(default)]
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Meta {
    pub date: String,
    pub claude_version: String,
    pub av_rev: String,
    pub models: Vec<String>,
    pub seed: u64,
}

fn valid<'a>(rows: &'a [Row], arm: &str) -> Vec<&'a Row> {
    rows.iter()
        .filter(|r| r.arm == arm && r.status == Status::Ok)
        .collect()
}

fn usd(v: Option<f64>) -> String {
    v.map_or("n/a".into(), |v| format!("${v:.3}"))
}

fn int(v: Option<f64>) -> String {
    v.map_or("n/a".into(), |v| format!("{}", v.round() as i64))
}

fn pct(v: Option<f64>) -> String {
    v.map_or("n/a".into(), |v| format!("{:+.1}%", v * 100.0))
}

fn ci(values: &[f64], fmt: fn(Option<f64>) -> String, seed: u64) -> String {
    let m = fmt(median(values));
    match bootstrap_median_ci(values, seed) {
        Some((lo, hi)) => format!("{m} ({}–{})", fmt(Some(lo)), fmt(Some(hi))),
        None => m,
    }
}

struct ArmStats {
    n: usize,
    accepted: usize,
    cost_per_accepted: Option<f64>,
    wall_per_accepted: Option<f64>,
    costs: Vec<f64>,
    totals: Vec<f64>,
    first_edits: Vec<f64>,
    no_first_edit: usize,
    dead_ends: u32,
}

fn arm_stats(rows: &[Row], arm: &str) -> ArmStats {
    let v = valid(rows, arm);
    let accepted = v.iter().filter(|r| r.success).count();
    let metrics: Vec<&SessionMetrics> = v.iter().filter_map(|r| r.metrics.as_ref()).collect();
    let spent: f64 = v.iter().map(|r| r.cost_usd).sum();
    let wall: f64 = v.iter().map(|r| r.wall_secs).sum();
    ArmStats {
        n: v.len(),
        accepted,
        cost_per_accepted: (accepted > 0).then(|| spent / accepted as f64),
        wall_per_accepted: (accepted > 0).then(|| wall / accepted as f64),
        costs: v.iter().map(|r| r.cost_usd).collect(),
        totals: metrics.iter().map(|m| m.tokens_total as f64).collect(),
        first_edits: metrics
            .iter()
            .filter_map(|m| m.tokens_to_first_edit.map(|t| t as f64))
            .collect(),
        no_first_edit: metrics
            .iter()
            .filter(|m| m.tokens_to_first_edit.is_none())
            .count(),
        dead_ends: metrics.iter().map(|m| m.repeated_dead_ends).sum(),
    }
}

fn success_rate(s: &ArmStats) -> Option<f64> {
    (s.n > 0).then(|| s.accepted as f64 / s.n as f64)
}

pub fn render_report(rows: &[Row], meta: &Meta) -> String {
    let arms: Vec<&str> = ARMS
        .iter()
        .copied()
        .filter(|a| rows.iter().any(|r| r.arm == *a))
        .collect();
    let stats: Vec<(&str, ArmStats)> = arms.iter().map(|a| (*a, arm_stats(rows, a))).collect();
    let mut out = String::new();
    let _ = writeln!(out, "# scholion-bench report\n");
    let _ = writeln!(
        out,
        "Date {} · claude {} · av {} · model {} · seed {} · {} sessions launched, {} spent\n",
        meta.date,
        meta.claude_version,
        meta.av_rev,
        meta.models.join(", "),
        meta.seed,
        rows.iter()
            .filter(|r| r.status != Status::SkippedBudget)
            .count(),
        usd(Some(rows.iter().map(|r| r.cost_usd).sum())),
    );
    let ns: Vec<String> = stats.iter().map(|(a, s)| format!("{a} {}", s.n)).collect();
    let _ = writeln!(out, "Valid sessions per arm: {}.", ns.join(", "));
    if stats.iter().any(|(_, s)| s.n < 30) {
        let _ = writeln!(
            out,
            "**No significance claim**: fewer than 30 valid sessions in at least one arm; the \
             intervals below are bootstrap 95% CIs of the median and are wide at this n."
        );
    }
    let _ = writeln!(
        out,
        "\nToken counts exclude cache reads in every arm, so per-turn re-reads of the same context \
         are not counted. `tokens_to_first_edit` counts output tokens as each message reports \
         them, which can lag the session totals. Every arm starts from the same single-commit \
         tree with no history (the av arms have no `.git` at all); a real git checkout would also \
         have history, so this comparison is conservative for av. Dead ends are seeded as \
         `attempt` records in the av arms only: the dead-end column measures whether existing \
         records stop repeats, not whether agents write them.\n"
    );

    let _ = writeln!(out, "## Cost and wall time per accepted task\n");
    let _ = writeln!(
        out,
        "| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |"
    );
    let _ = writeln!(out, "|---|---|---|---|---|---|---|---|");
    for (arm, s) in &stats {
        let _ = writeln!(
            out,
            "| {arm} | {}/{} | {} | {} | {} | {} | {} ({}) | {} |",
            s.accepted,
            s.n,
            usd(s.cost_per_accepted),
            s.wall_per_accepted
                .map_or("n/a".into(), |w| format!("{w:.0} s")),
            ci(&s.costs, usd, meta.seed),
            ci(&s.totals, int, meta.seed),
            int(median(&s.first_edits)),
            s.no_first_edit,
            s.dead_ends,
        );
    }

    let get = |a: &str| stats.iter().find(|(x, _)| *x == a).map(|(_, s)| s);
    let pairs = [
        ("engine: av_no_packs vs baseline", "baseline", "av_no_packs"),
        ("context compiler: av vs av_no_packs", "av_no_packs", "av"),
        ("total: av vs baseline", "baseline", "av"),
    ];
    let present: Vec<_> = pairs
        .iter()
        .filter_map(|(label, b, n)| Some((label, get(b)?, get(n)?)))
        .collect();
    if !present.is_empty() {
        let _ = writeln!(out, "\n## Attribution (relative change)\n");
        let _ = writeln!(
            out,
            "| comparison | cost / accepted | median tokens_total | median tokens_to_first_edit | success rate |"
        );
        let _ = writeln!(out, "|---|---|---|---|---|");
        for (label, b, n) in present {
            let _ = writeln!(
                out,
                "| {label} | {} | {} | {} | {} |",
                pct(relative_change(b.cost_per_accepted, n.cost_per_accepted)),
                pct(relative_change(median(&b.totals), median(&n.totals))),
                pct(relative_change(
                    median(&b.first_edits),
                    median(&n.first_edits)
                )),
                pct(relative_change(success_rate(b), success_rate(n))),
            );
        }
    }

    let tasks: BTreeSet<&str> = rows.iter().map(|r| r.task.as_str()).collect();
    let _ = writeln!(out, "\n## Per task\n");
    let _ = writeln!(
        out,
        "| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |"
    );
    let _ = writeln!(out, "|---|---|---|---|---|---|---|---|");
    for task in &tasks {
        for arm in &arms {
            let only: Vec<Row> = rows.iter().filter(|r| r.task == *task).cloned().collect();
            let s = arm_stats(&only, arm);
            if s.n == 0 {
                continue;
            }
            let _ = writeln!(
                out,
                "| {task} | {arm} | {} | {} | {} | {} | {} | {} |",
                s.n,
                s.accepted,
                int(median(&s.first_edits)),
                int(median(&s.totals)),
                s.dead_ends,
                usd(median(&s.costs)),
            );
        }
    }

    let listed = |status: Status| -> Vec<String> {
        rows.iter()
            .filter(|r| r.status == status)
            .map(|r| match &r.reason {
                Some(why) => format!("- {} · {} · rep {}: {why}", r.task, r.arm, r.rep),
                None => format!("- {} · {} · rep {}", r.task, r.arm, r.rep),
            })
            .collect()
    };
    for (title, status) in [
        ("Skipped (budget)", Status::SkippedBudget),
        ("Invalid", Status::Invalid),
    ] {
        let items = listed(status);
        if !items.is_empty() {
            let _ = writeln!(out, "\n## {title}\n\n{}", items.join("\n"));
        }
    }
    out
}

pub fn read_rows(path: &std::path::Path) -> crate::Result<Vec<Row>> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).map_err(|e| format!("{}: {e}", path.display())))
        .collect()
}
