//! `scholion-bench`: check task specs, run the benchmark, render the report, calibrate the token factor.

use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};

use clap::{Args, Parser, Subcommand};
use scholion_bench::calibrate::{Probe, cl100k_tokens, estimate_error, recommended_factor};
use scholion_bench::events::parse_stream;
use scholion_bench::mine::{MineArgs, mine};
use scholion_bench::orchestrate::{RunArgs, run};
use scholion_bench::report::{Meta, read_rows, render_report};
use scholion_bench::retrieval::{self, RetrievalArgs};
use scholion_bench::runner::{Arm, Daemon, RunConfig};
use scholion_bench::spec::{TaskSpec, check_spec, check_warnings, load_spec_with_root, spec_files};

#[derive(Parser)]
#[command(
    name = "scholion-bench",
    about = "Scholion benchmark harness (Plans 4 and 6)"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Args, Clone)]
struct TaskArgs {
    /// Task spec files, or directories of them.
    #[arg(required = true)]
    tasks: Vec<PathBuf>,
    /// Where the source repos live, side by side (replaces each spec's repo_path parent).
    #[arg(long, env = "AV_BENCH_REPOS")]
    repos: Option<PathBuf>,
}

#[derive(Args, Clone)]
struct SessionArgs {
    #[arg(long, default_value = "~/bench-work")]
    work_root: PathBuf,
    #[arg(long, default_value = "claude")]
    claude_bin: String,
    /// The av CLI the hooks run.
    #[arg(long, default_value = "av")]
    av_bin: String,
    /// The bench daemon (its own compose project; never your real one).
    #[arg(long, env = "AV_BENCH_URL")]
    av_url: Option<String>,
    #[arg(long, env = "AV_TOKEN", hide_env_values = true)]
    av_token: Option<String>,
    /// Host directory mounted as the bench daemon's import root [default: WORK_ROOT/src].
    #[arg(long)]
    import_dir: Option<PathBuf>,
    /// The same directory as the daemon sees it.
    #[arg(long, default_value = "/import")]
    import_mount: String,
    #[arg(
        long,
        value_delimiter = ',',
        default_value = "Read,Edit,Write,MultiEdit,Glob,Grep,LS,Bash"
    )]
    allowed_tools: Vec<String>,
    #[arg(long, default_value = "acceptEdits")]
    permission_mode: String,
    /// Per-session cap passed to claude --max-budget-usd.
    #[arg(long, default_value_t = 2.0)]
    session_cap_usd: f64,
    /// Claude config dir to copy .credentials.json from.
    #[arg(long, default_value = "~/.claude")]
    claude_home: PathBuf,
    #[arg(long, default_value_t = 800)]
    hook_budget: u32,
    /// Run the av arms against a daemon without a semantic layer.
    #[arg(long)]
    allow_no_semantic: bool,
    /// Wait this long for a new repo's embeddings before its session.
    #[arg(long, default_value_t = 1800)]
    index_wait_secs: u64,
    /// Also give the av arms the av MCP server (default: hooks and CLI only, the shipped Claude
    /// Code integration).
    #[arg(long)]
    av_mcp: bool,
    /// `--code` for the session-start hook: `full` or `signatures` (default: the hook's own).
    #[arg(long, value_parser = ["full", "signatures"])]
    hook_code: Option<String>,
    /// One more argument for the session-start hook, repeatable: `--hook-arg=--no-map
    /// --hook-arg=--max-code=3`.
    #[arg(long = "hook-arg", allow_hyphen_values = true)]
    hook_args: Vec<String>,
    /// Also give the av arms the just-in-time hooks: `av hook pre-read` on reads and edits,
    /// `av hook stop` when the agent stops.
    #[arg(long)]
    jit_hooks: bool,
    /// Cost sessions from token usage at these prices, dollars per million tokens:
    /// `input,cache_write,cache_read,output`. For models Claude Code cannot price (DeepSeek
    /// through its Anthropic API); claude's own --max-budget-usd is then not passed.
    #[arg(long, value_parser = scholion_bench::metrics::Prices::parse)]
    price_per_mtok: Option<scholion_bench::metrics::Prices>,
}

#[derive(Subcommand)]
enum Cmd {
    /// Validate task specs against their repos.
    Check {
        #[command(flatten)]
        tasks: TaskArgs,
    },
    /// Run sessions. Without --confirm this only prints the plan.
    Run {
        #[command(flatten)]
        tasks: TaskArgs,
        #[command(flatten)]
        session: SessionArgs,
        /// Results directory (rows.jsonl, report.md, meta.json); rerun to resume.
        #[arg(long)]
        out: PathBuf,
        #[arg(long, default_value_t = 3)]
        reps: u32,
        /// Required: sessions stop launching before this would be exceeded.
        #[arg(long)]
        budget_usd: f64,
        #[arg(long, value_delimiter = ',', default_value = "baseline,av_no_packs,av")]
        arms: Vec<String>,
        #[arg(long, default_value_t = 1)]
        seed: u64,
        /// Launch paid sessions.
        #[arg(long)]
        confirm: bool,
        #[arg(long)]
        dry_run: bool,
        #[arg(long)]
        est_session_usd: Option<f64>,
        /// A pilot's results directory: its median cost per arm is the estimate.
        #[arg(long)]
        pilot: Option<PathBuf>,
        /// Only these tasks (by name).
        #[arg(long, value_delimiter = ',')]
        only: Vec<String>,
    },
    /// Render report.md from a results directory.
    Report { results: PathBuf },
    /// Offline pack retrieval (Plan 6, E1): score first packs and three baselines against each
    /// task's fix. No agent and no money; the pack method needs a daemon.
    Retrieval {
        /// `.toml` specs, `.jsonl` retrieval sets, or directories of specs.
        #[arg(required = true)]
        tasks: Vec<PathBuf>,
        /// Where the source repos live, side by side (looked up by directory name).
        #[arg(long, env = "AV_BENCH_REPOS")]
        repos: Option<PathBuf>,
        #[arg(long, value_delimiter = ',', default_value = "2000,4000,8000")]
        budgets: Vec<u32>,
        /// Results directory (rows.jsonl, gold.jsonl, texts/, report.md).
        #[arg(long)]
        out: PathBuf,
        /// The daemon that builds the packs (a bench daemon, never your real one).
        #[arg(long, env = "AV_BENCH_URL")]
        url: Option<String>,
        #[arg(long, env = "AV_TOKEN", hide_env_values = true)]
        token: Option<String>,
        /// Host directory the daemon imports from.
        #[arg(long, default_value = "~/bench-work/src")]
        import_dir: PathBuf,
        /// The same directory as the daemon sees it.
        #[arg(long, default_value = "/import")]
        import_mount: String,
        /// Measure full-text-only packs when the daemon has no semantic layer.
        #[arg(long)]
        allow_no_semantic: bool,
        #[arg(long, default_value_t = 1800)]
        index_wait_secs: u64,
        /// Baselines only (no daemon).
        #[arg(long)]
        no_pack: bool,
        /// Reuse the repos the run with this id imported (the `r<id>-` prefix of its repo names)
        /// into the same daemon: measures a new packer without importing and embedding again.
        #[arg(long)]
        reuse_run: Option<String>,
    },
    /// Mine retrieval tasks from a repo's history (small commits to non-test source files).
    Mine {
        /// The local clone to mine.
        repo: PathBuf,
        /// What to write as each task's repo_path [default: REPO as given].
        #[arg(long)]
        repo_path: Option<String>,
        /// Task name prefix [default: the repo's directory name].
        #[arg(long)]
        prefix: Option<String>,
        #[arg(long)]
        since: Option<String>,
        #[arg(long, default_value_t = 3)]
        max_files: usize,
        #[arg(long, default_value_t = 200)]
        max_lines: usize,
        #[arg(long, default_value_t = 50)]
        limit: usize,
        /// Output `.jsonl` (stdout when omitted).
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Recommend AV_TOKEN_FACTOR_CLAUDE from stored packs (pack.md files under DIR).
    Calibrate {
        #[arg(long)]
        packs: PathBuf,
        #[arg(long, default_value = "claude-sonnet-5-5")]
        model: String,
        #[command(flatten)]
        session: SessionArgs,
        #[arg(long)]
        budget_usd: f64,
        #[arg(long)]
        confirm: bool,
    },
}

fn expand(p: &Path) -> PathBuf {
    match (p.strip_prefix("~"), std::env::var_os("HOME")) {
        (Ok(rest), Some(home)) => PathBuf::from(home).join(rest),
        _ => p.to_path_buf(),
    }
}

fn load_tasks(t: &TaskArgs) -> Result<Vec<TaskSpec>, String> {
    let mut files = vec![];
    for p in &t.tasks {
        if p.is_dir() {
            files.extend(spec_files(p)?);
        } else {
            files.push(p.clone());
        }
    }
    let root = t.repos.as_deref().map(expand);
    files
        .iter()
        .map(|f| load_spec_with_root(f, root.as_deref()))
        .collect()
}

fn config(s: &SessionArgs) -> RunConfig {
    let work_root = expand(&s.work_root);
    RunConfig {
        import_dir: s
            .import_dir
            .as_deref()
            .map(expand)
            .unwrap_or_else(|| work_root.join("src")),
        work_root,
        claude_bin: s.claude_bin.clone(),
        av_bin: s.av_bin.clone(),
        av_url: s.av_url.clone(),
        av_token: s.av_token.clone(),
        import_mount: s.import_mount.clone(),
        allowed_tools: s.allowed_tools.clone(),
        permission_mode: s.permission_mode.clone(),
        session_cap_usd: s.session_cap_usd,
        claude_home: Some(expand(&s.claude_home)),
        hook_budget: s.hook_budget,
        allow_no_semantic: s.allow_no_semantic,
        index_wait_secs: s.index_wait_secs,
        av_mcp: s.av_mcp,
        jit_hooks: s.jit_hooks,
        prices: s.price_per_mtok,
        hook_code: s.hook_code.clone(),
        hook_args: s.hook_args.clone(),
    }
}

fn output(cmd: &mut Command) -> String {
    cmd.stderr(Stdio::null())
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
        .unwrap_or_else(|| "unknown".into())
}

fn meta(cfg: &RunConfig, specs: &[TaskSpec], seed: u64) -> Meta {
    let mut models: Vec<String> = specs.iter().map(|s| s.model.clone()).collect();
    models.sort();
    models.dedup();
    Meta {
        date: output(Command::new("date").arg("-u").arg("+%Y-%m-%d %H:%M UTC")),
        claude_version: output(Command::new(&cfg.claude_bin).arg("--version")),
        av_rev: output(
            Command::new("git")
                .arg("-C")
                .arg(env!("CARGO_MANIFEST_DIR"))
                .args(["rev-parse", "--short", "HEAD"]),
        ),
        models,
        seed,
        av_mcp: Some(cfg.av_mcp),
        hook_args: cfg
            .hook_code
            .iter()
            .map(|c| format!("--code={c}"))
            .chain(cfg.hook_args.iter().cloned())
            .collect(),
    }
}

/// Every `pack.md` under `dir`.
fn pack_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            pack_files(&p, out);
        } else if p.file_name().is_some_and(|n| n == "pack.md") {
            out.push(p);
        }
    }
}

/// One `claude -p --max-turns 1` call: (all input tokens, cost).
fn probe(cfg: &RunConfig, model: &str, prompt: &str, dir: &Path) -> Result<(u64, f64), String> {
    let config_dir = dir.join("claude-config");
    std::fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    if let Some(home) = &cfg.claude_home
        && home.join(".credentials.json").exists()
    {
        let _ = std::fs::copy(
            home.join(".credentials.json"),
            config_dir.join(".credentials.json"),
        );
    }
    let empty = dir.join("mcp.json");
    std::fs::write(&empty, r#"{"mcpServers":{}}"#).map_err(|e| e.to_string())?;
    let out = Command::new(&cfg.claude_bin)
        .arg("-p")
        .arg(prompt)
        .args([
            "--output-format",
            "stream-json",
            "--verbose",
            "--max-turns",
            "1",
        ])
        .args([
            "--model",
            model,
            "--setting-sources",
            "project",
            "--strict-mcp-config",
        ])
        .arg("--mcp-config")
        .arg(&empty)
        .args(["--no-session-persistence", "--tools", ""])
        .args(["--max-budget-usd", &format!("{:.2}", cfg.session_cap_usd)])
        .current_dir(dir)
        .env("CLAUDE_CONFIG_DIR", &config_dir)
        .stdin(Stdio::null())
        .output()
        .map_err(|e| format!("{}: {e}", cfg.claude_bin))?;
    let parsed = parse_stream(&String::from_utf8_lossy(&out.stdout));
    let r = parsed.result.ok_or("claude returned no result event")?;
    Ok((r.usage.all_input(), r.cost_usd))
}

fn calibrate(
    packs: &Path,
    model: &str,
    cfg: &RunConfig,
    budget: f64,
    confirm: bool,
) -> Result<(), String> {
    let mut files = vec![];
    pack_files(packs, &mut files);
    files.sort();
    if files.is_empty() {
        return Err(format!("no pack.md under {}", packs.display()));
    }
    println!(
        "calibrate: {} packs + 1 overhead probe, model {model}, at most ${budget:.2}",
        files.len()
    );
    if !confirm {
        println!("dry run: pass --confirm to make the probe calls");
        return Ok(());
    }
    let dir = cfg.work_root.join("calibrate");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let (overhead, mut spent) = probe(cfg, model, ".", &dir)?;
    let mut probes = vec![];
    for f in &files {
        if spent + cfg.session_cap_usd > budget {
            println!("budget reached after {} packs", probes.len());
            break;
        }
        let text = std::fs::read_to_string(f).map_err(|e| e.to_string())?;
        let (measured, cost) = probe(cfg, model, &text, &dir)?;
        spent += cost;
        let p = Probe {
            cl100k: cl100k_tokens(&text),
            measured_with_overhead: measured,
        };
        let m = measured.saturating_sub(overhead);
        println!(
            "{}: cl100k {} measured {m} ratio {} estimate_error {}",
            f.display(),
            p.cl100k,
            if p.cl100k > 0 {
                format!("{:.3}", m as f64 / p.cl100k as f64)
            } else {
                "n/a".into()
            },
            estimate_error(p.cl100k, m).map_or("n/a".into(), |e| format!("{:+.3}", e)),
        );
        probes.push(p);
    }
    let factor = recommended_factor(&probes, overhead)?;
    println!("overhead {overhead} tokens; spent ${spent:.3}");
    println!(
        "recommended AV_TOKEN_FACTOR_CLAUDE={factor:.2} (restart the bench daemon after setting it)"
    );
    Ok(())
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result: Result<i32, String> = match cli.cmd {
        Cmd::Check { tasks } => load_tasks(&tasks).map(|specs| {
            let mut bad = 0;
            for s in &specs {
                match check_spec(s) {
                    Ok(()) => {
                        println!("{}: ok", s.name);
                        for w in check_warnings(s) {
                            println!("{w}");
                        }
                    }
                    Err(e) => {
                        println!("{e}");
                        bad += 1;
                    }
                }
            }
            if bad > 0 { 1 } else { 0 }
        }),
        Cmd::Run {
            tasks,
            session,
            out,
            reps,
            budget_usd,
            arms,
            seed,
            confirm,
            dry_run,
            est_session_usd,
            pilot,
            only,
        } => (|| {
            let mut specs = load_tasks(&tasks)?;
            if !only.is_empty() {
                specs.retain(|s| only.contains(&s.name));
            }
            for s in &specs {
                check_spec(s)?;
            }
            let arms = arms
                .iter()
                .map(|a| Arm::parse(a))
                .collect::<Result<Vec<_>, _>>()?;
            let cfg = config(&session);
            let args = RunArgs {
                out: expand(&out),
                arms,
                reps,
                budget_usd,
                seed,
                confirm,
                dry_run,
                est_session_usd,
                pilot: pilot.as_deref().map(expand),
                meta: meta(&cfg, &specs, seed),
            };
            let r = run(&specs, &args, &cfg)?;
            print!("{}", r.plan_text);
            match r.exit_code {
                3 => println!("dry run: nothing launched (pass --confirm to run)"),
                code => {
                    println!("report: {}", args.out.join("report.md").display());
                    if code == 2 {
                        println!("some sessions were skipped: the budget ran out");
                    }
                }
            }
            Ok(if r.exit_code == 3 { 0 } else { r.exit_code })
        })(),
        Cmd::Report { results } => (|| {
            let dir = expand(&results);
            let rows = read_rows(&dir.join("rows.jsonl"))?;
            let meta: Meta = std::fs::read(dir.join("meta.json"))
                .ok()
                .and_then(|b| serde_json::from_slice(&b).ok())
                .unwrap_or_default();
            let md = render_report(&rows, &meta);
            std::fs::write(dir.join("report.md"), &md).map_err(|e| e.to_string())?;
            print!("{md}");
            Ok(0)
        })(),
        Cmd::Retrieval {
            tasks,
            repos,
            budgets,
            out,
            url,
            token,
            import_dir,
            import_mount,
            allow_no_semantic,
            index_wait_secs,
            no_pack,
            reuse_run,
        } => (|| {
            let tasks = retrieval::load_tasks(&tasks, repos.as_deref().map(expand).as_deref())?;
            let daemon = url.as_deref().map(|u| Daemon::new(u, token.as_deref()));
            if let Some(d) = &daemon {
                d.preflight()?;
            }
            let args = RetrievalArgs {
                budgets,
                out: expand(&out),
                import_dir: expand(&import_dir),
                import_mount,
                allow_no_semantic,
                index_wait_secs,
                no_pack,
                reuse_run,
            };
            retrieval::run(&tasks, daemon.as_ref(), &args)?;
            println!("report: {}", args.out.join("report.md").display());
            Ok(0)
        })(),
        Cmd::Mine {
            repo,
            repo_path,
            prefix,
            since,
            max_files,
            max_lines,
            limit,
            out,
        } => (|| {
            let repo = expand(&repo);
            let name = repo
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| "repo".into());
            let args = MineArgs {
                repo_path: repo_path.unwrap_or_else(|| repo.display().to_string()),
                prefix: prefix.unwrap_or(name),
                since,
                max_files,
                max_lines,
                limit,
            };
            let tasks = mine(&repo, &args)?;
            let text: String = tasks
                .iter()
                .map(|t| serde_json::to_string(t).map(|l| l + "\n"))
                .collect::<Result<_, _>>()
                .map_err(|e| e.to_string())?;
            match out {
                Some(p) => {
                    std::fs::write(expand(&p), text).map_err(|e| e.to_string())?;
                    eprintln!("{} tasks -> {}", tasks.len(), p.display());
                }
                None => print!("{text}"),
            }
            Ok(0)
        })(),
        Cmd::Calibrate {
            packs,
            model,
            session,
            budget_usd,
            confirm,
        } => calibrate(
            &expand(&packs),
            &model,
            &config(&session),
            budget_usd,
            confirm,
        )
        .map(|()| 0),
    };
    match result {
        Ok(0) => ExitCode::SUCCESS,
        Ok(code) => ExitCode::from(code as u8),
        Err(e) => {
            eprintln!("scholion-bench: {e}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_config(extra: &[&str]) -> RunConfig {
        let mut args = vec![
            "scholion-bench",
            "run",
            "tasks",
            "--out",
            "o",
            "--budget-usd",
            "1",
        ];
        args.extend_from_slice(extra);
        match Cli::try_parse_from(args).unwrap().cmd {
            Cmd::Run { session, .. } => config(&session),
            _ => unreachable!(),
        }
    }

    #[test]
    fn av_arms_run_without_mcp_unless_asked() {
        assert!(!run_config(&[]).av_mcp);
        assert!(run_config(&["--av-mcp"]).av_mcp);
    }
}
