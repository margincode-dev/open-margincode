//! `margincode-bench`: check task specs, run the benchmark, render the report, calibrate the token factor.

use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};

use clap::{Args, Parser, Subcommand};
use margincode_bench::calibrate::{Probe, cl100k_tokens, estimate_error, recommended_factor};
use margincode_bench::dreamed;
use margincode_bench::events::parse_stream;
use margincode_bench::isolate::AgentUser;
use margincode_bench::mine::{MineArgs, mine};
use margincode_bench::noise::{add_noise, anchors_at, generate};
use margincode_bench::orchestrate::{RunArgs, run};
use margincode_bench::report::{Meta, read_rows, render_report};
use margincode_bench::retrieval::{self, RetrievalArgs};
use margincode_bench::runner::{
    Arm, Daemon, RunConfig, Vendor, first_pack, first_pack_dir, first_packs_report, notes_md,
};
use margincode_bench::scale::{self, ScaleArgs};
use margincode_bench::spec::{
    TaskSpec, check_spec, check_warnings, load_spec_with_root, spec_files,
};

#[derive(Parser)]
#[command(
    name = "margincode-bench",
    about = "Margincode benchmark harness (Plans 4, 6 and 8)"
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
    /// Agent CLI. `grok` runs `grok -p PROMPT --output-format json --always-approve --max-turns N
    /// --cwd DIR` and never passes `--max-budget-usd` (the turn cap stands in for it).
    #[arg(long, default_value = "claude", value_parser = ["claude", "grok"])]
    vendor: String,
    /// grok binary, when --vendor grok.
    #[arg(long, default_value = "grok")]
    grok_bin: String,
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
    /// Plan 16 M1: seed the pool's `no_commit` notes (the wrong notes) as the records of an
    /// earlier task whose merge was reverted, so their standing is `reverted`.
    #[arg(long)]
    wrong_reverted: bool,
    /// Plan 16 M2 stage B: a file `dream-freeze` wrote. Each av session's store gets that
    /// file's verdicts and reviews once it is seeded, before the agent's task starts (the
    /// dreamed arm). Without it, the raw store.
    #[arg(long)]
    dreamed: Option<PathBuf>,
    /// Also give the av arms the just-in-time hooks: `av hook pre-read` on reads and edits,
    /// `av hook stop` when the agent stops.
    #[arg(long)]
    jit_hooks: bool,
    /// Cost sessions from token usage at these prices, dollars per million tokens:
    /// `input,cache_write,cache_read,output`. For models Claude Code cannot price (DeepSeek
    /// through its Anthropic API); claude's own --max-budget-usd is then not passed.
    #[arg(long, value_parser = margincode_bench::metrics::Prices::parse)]
    price_per_mtok: Option<margincode_bench::metrics::Prices>,
    /// Keep each session's clone after verify (default: removed, and an av task abandoned;
    /// an invalid session always keeps its work).
    #[arg(long)]
    keep_work: bool,
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
        /// E4 at scale: add this many irrelevant notes to every relay task's pool, the same ones
        /// in `NOTES.md` and as av records (see `margincode-bench noise`). One `--out` per level.
        #[arg(long)]
        noise: Option<usize>,
        /// Seed of the noise notes and of where they fall among the lessons.
        #[arg(long, default_value_t = 1)]
        noise_seed: u64,
        /// Start the agent CLI as this user (Unix; the harness must run as root). The user gets
        /// its own working copy and nothing else: not the bench tree, not the grading tests,
        /// not earlier sessions. Before the first session the harness tries those paths as that
        /// user and refuses to run if one can be read. Use a user nothing else runs as: its
        /// processes are killed after every session. See bench/README.md, "Agent isolation".
        #[arg(long)]
        agent_user: Option<String>,
        /// The bench daemon's workspace root as the harness sees it; needed with --agent-user
        /// for the av arms.
        #[arg(long, env = "AV_WS_ROOT")]
        av_ws_root: Option<PathBuf>,
    },
    /// Print the noise notes `run --noise N` would add for a repo, as a relay pool (TOML), and
    /// how large they make `NOTES.md`. No daemon, no agent.
    Noise {
        /// The repo (a local git clone).
        repo: PathBuf,
        /// The base revision the notes are anchored in.
        #[arg(long, default_value = "main")]
        rev: String,
        /// How many notes.
        #[arg(short, long)]
        n: usize,
        #[arg(long, default_value_t = 1)]
        seed: u64,
    },
    /// E4 at scale, free: set each relay task up as `run --arms av` does and report where its
    /// own lesson stands in the packs the hooks would print. No agent, no money; needs the bench
    /// daemon.
    FirstPacks {
        #[command(flatten)]
        tasks: TaskArgs,
        #[command(flatten)]
        session: SessionArgs,
        /// Results directory (first-packs.jsonl, first-packs.md, packs/).
        #[arg(long)]
        out: PathBuf,
        /// Only these tasks (by name).
        #[arg(long, value_delimiter = ',')]
        only: Vec<String>,
        /// As `run --noise`.
        #[arg(long, default_value_t = 0)]
        noise: usize,
        #[arg(long, default_value_t = 1)]
        noise_seed: u64,
    },
    /// Plan 16 M2: seed each relay task's store as `run --arms av` does, run `av dream` and
    /// then `av dream --verify` on it as two agents, and write what they ended with to `--out`
    /// for `run --dreamed` (every session of the dreamed arm then reads that one memory). Prints
    /// stage A's tally per task. Spends only what the two agent commands spend.
    DreamFreeze {
        #[command(flatten)]
        tasks: TaskArgs,
        #[command(flatten)]
        session: SessionArgs,
        /// The file to write (JSON).
        #[arg(long)]
        out: PathBuf,
        /// Only these tasks (by name).
        #[arg(long, value_delimiter = ',')]
        only: Vec<String>,
        /// As `run --noise`: a session replays a frozen dream only on the same pool.
        #[arg(long, default_value_t = 0)]
        noise: usize,
        #[arg(long, default_value_t = 1)]
        noise_seed: u64,
        /// The agent name of the review pass.
        #[arg(long, default_value = "dreamer")]
        dreamer: String,
        /// Its command, run with `sh -c` in the run's directory beside DREAM.md, e.g.
        /// `claude -p "Read DREAM.md and do what it says"`.
        #[arg(long)]
        dream_cmd: String,
        /// The agent name of the verify pass (another agent).
        #[arg(long, default_value = "verifier")]
        verifier: String,
        #[arg(long)]
        verify_cmd: String,
        /// `av dream --limit` (records, or verdicts, per pass).
        #[arg(long)]
        limit: Option<u32>,
        /// `av dream --timeout`, seconds.
        #[arg(long, default_value_t = 1800)]
        timeout: u64,
        /// `av dream --fix-rounds`.
        #[arg(long, default_value_t = 1)]
        fix_rounds: u32,
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
        #[arg(long, value_delimiter = ',', default_value = "800,2000,4000,8000")]
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
    /// Scale and latency (Plan 6, E2): import one repo into a bench daemon, time packs,
    /// checkpoints and `why`, then run whole tasks with many agents at once. No agent, no money.
    Scale {
        /// The clone to import.
        repo: PathBuf,
        /// Its name in the daemon [default: the directory name].
        #[arg(long)]
        name: Option<String>,
        /// The clone's path as the daemon sees it [default: REPO].
        #[arg(long)]
        git_source: Option<String>,
        /// The daemon (a bench daemon, never your real one).
        #[arg(long, env = "AV_BENCH_URL")]
        url: String,
        #[arg(long, env = "AV_TOKEN", hide_env_values = true)]
        token: Option<String>,
        /// Results directory (scale.json, report.md).
        #[arg(long)]
        out: PathBuf,
        /// Agents at once, one run per value.
        #[arg(long, value_delimiter = ',', default_value = "1,8,32,100")]
        agents: Vec<usize>,
        /// Tasks for the one-at-a-time latencies.
        #[arg(long, default_value_t = 20)]
        samples: usize,
        /// Context packs per latency task.
        #[arg(long, default_value_t = 3)]
        reps: usize,
        /// The daemon's process id, to read its memory.
        #[arg(long)]
        daemon_pid: Option<u32>,
        /// The daemon's database, to read its size with psql.
        #[arg(long)]
        database_url: Option<String>,
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
    /// Print the E-G1 table, Wilson intervals and `av agent set` commands.
    /// No daemon and no network. The file is JSONL (one cell per line; `#` comments allowed)
    /// or a JSON array. Fields: `tag`, `agent`, `trials`, `accepted`, `tokens_per_accepted`
    /// (null when nothing was accepted). A tie, or a cell with fewer than 10 trials, is
    /// inconclusive and does not emit a command for that agent.
    RoutingProfile {
        /// Results file.
        results: PathBuf,
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
        grok_bin: s.grok_bin.clone(),
        vendor: match s.vendor.as_str() {
            "grok" => Vendor::Grok,
            _ => Vendor::Claude,
        },
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
        keep_work: s.keep_work,
        agent_user: None,
        av_ws_root: None,
        wrong_reverted: s.wrong_reverted,
        dreamed: s.dreamed.as_deref().map(expand),
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
        noise: None,
        noise_seed: None,
        // `orchestrate::run` fills it in from the configuration
        agent_isolation: None,
        memory: None,
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
        Cmd::Check { tasks } => {
            if let Err(e) = ensure_generated_repos(&tasks.tasks) {
                Err(e)
            } else {
                load_tasks(&tasks).map(|specs| {
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
                })
            }
        }
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
            noise,
            noise_seed,
            agent_user,
            av_ws_root,
        } => (|| {
            let mut specs = load_tasks(&tasks)?;
            if !only.is_empty() {
                specs.retain(|s| only.contains(&s.name));
            }
            for s in &specs {
                check_spec(s)?;
            }
            if let Some(n) = noise {
                add_noise(&mut specs, n, noise_seed)?;
            }
            let arms = arms
                .iter()
                .map(|a| Arm::parse(a))
                .collect::<Result<Vec<_>, _>>()?;
            let cfg = RunConfig {
                agent_user: agent_user.as_deref().map(AgentUser::resolve).transpose()?,
                av_ws_root: av_ws_root.as_deref().map(expand),
                ..config(&session)
            };
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
                meta: Meta {
                    noise,
                    noise_seed: noise.map(|_| noise_seed),
                    ..meta(&cfg, &specs, seed)
                },
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
        Cmd::FirstPacks {
            tasks,
            session,
            out,
            only,
            noise,
            noise_seed,
        } => (|| {
            let mut specs = load_tasks(&tasks)?;
            if !only.is_empty() {
                specs.retain(|s| only.contains(&s.name));
            }
            for s in &specs {
                check_spec(s)?;
            }
            add_noise(&mut specs, noise, noise_seed)?;
            let cfg = config(&session);
            let out = expand(&out);
            let packs = out.join("packs");
            std::fs::create_dir_all(&packs).map_err(|e| format!("{}: {e}", packs.display()))?;
            let run_id = format!("fp{:x}", std::process::id());
            let (mut rows, mut lines) = (vec![], String::new());
            for spec in &specs {
                let row = first_pack(spec, &run_id, &cfg)?;
                let dir = first_pack_dir(&cfg, &run_id, &spec.name);
                for (from, to) in [
                    ("pack.md", "start.md"),
                    ("receipt.json", "start.receipt.json"),
                    ("hook-pack.md", "hook.md"),
                    ("hook-receipt.json", "hook.receipt.json"),
                ] {
                    let to = packs.join(format!("{}-{to}", spec.name));
                    std::fs::copy(dir.join(from), &to)
                        .map_err(|e| format!("{}: {e}", to.display()))?;
                }
                println!(
                    "{}: at start {}, hook pack {}, in full {}",
                    row.task,
                    row.at_start.place(),
                    row.at_hook.place(),
                    row.in_full()
                );
                lines.push_str(&serde_json::to_string(&row).map_err(|e| e.to_string())?);
                lines.push('\n');
                rows.push(row);
            }
            std::fs::write(out.join("first-packs.jsonl"), lines).map_err(|e| e.to_string())?;
            let report = first_packs_report(&rows, noise);
            std::fs::write(out.join("first-packs.md"), &report).map_err(|e| e.to_string())?;
            print!("{report}");
            Ok(0)
        })(),
        Cmd::DreamFreeze {
            tasks,
            session,
            out,
            only,
            noise,
            noise_seed,
            dreamer,
            dream_cmd,
            verifier,
            verify_cmd,
            limit,
            timeout,
            fix_rounds,
        } => (|| {
            let mut specs = load_tasks(&tasks)?;
            if !only.is_empty() {
                specs.retain(|s| only.contains(&s.name));
            }
            for s in &specs {
                check_spec(s)?;
            }
            add_noise(&mut specs, noise, noise_seed)?;
            let cfg = config(&session);
            if cfg.dreamed.is_some() {
                return Err("dream-freeze dreams on the raw store: drop --dreamed".into());
            }
            let url = cfg
                .av_url
                .clone()
                .ok_or("dream-freeze needs the bench daemon (--av-url)")?;
            let daemon = Daemon::new(&url, cfg.av_token.as_deref());
            let out = expand(&out);
            let run_id = format!("df{:x}", std::process::id());
            let mut frozen = dreamed::Frozen {
                version: dreamed::VERSION,
                tasks: Default::default(),
            };
            for spec in &specs {
                let args = dreamed::FreezeArgs {
                    dreamer: dreamer.clone(),
                    dream_cmd: dream_cmd.clone(),
                    verifier: verifier.clone(),
                    verify_cmd: verify_cmd.clone(),
                    limit,
                    timeout: std::time::Duration::from_secs(timeout),
                    fix_rounds,
                    home: cfg.work_root.join(&run_id).join(&spec.name),
                };
                let name = format!("d{run_id}-{}", spec.name);
                let t = dreamed::freeze(spec, &cfg, &daemon, &name, &args)?;
                let a = &t.tally;
                println!(
                    "{}: {} verdicts, {} effective; wrong notes disputed {}/{}; true lessons \
                     disputed or outdated {}/{} (dreams in {})",
                    spec.name,
                    a.verdicts,
                    a.effective,
                    a.wrong_disputed,
                    a.wrong_notes,
                    a.true_hit,
                    a.true_notes,
                    args.home.display()
                );
                frozen.tasks.insert(spec.name.clone(), t);
            }
            if let Some(dir) = out.parent() {
                std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
            }
            let text = serde_json::to_string_pretty(&frozen).map_err(|e| e.to_string())?;
            std::fs::write(&out, text).map_err(|e| format!("{}: {e}", out.display()))?;
            println!("frozen: {}", out.display());
            Ok(0)
        })(),
        Cmd::Noise { repo, rev, n, seed } => (|| {
            let anchors = anchors_at(&expand(&repo), &rev)?;
            let notes = generate(&anchors, n, seed);
            #[derive(serde::Serialize)]
            struct Pool<'a> {
                note: &'a [margincode_bench::spec::RelayNote],
            }
            let toml = toml::to_string(&Pool { note: &notes }).map_err(|e| e.to_string())?;
            print!("{toml}");
            eprintln!(
                "{} notes on {} anchors; NOTES.md with only these: {} cl100k tokens",
                notes.len(),
                anchors.len(),
                cl100k_tokens(&notes_md(&notes))
            );
            Ok(0)
        })(),
        Cmd::RoutingProfile { results } => (|| {
            let text = std::fs::read_to_string(expand(&results))
                .map_err(|e| format!("{}: {e}", results.display()))?;
            print!("{}", margincode_bench::routing_profile::report(&text)?);
            Ok(0)
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
        Cmd::Scale {
            repo,
            name,
            git_source,
            url,
            token,
            out,
            agents,
            samples,
            reps,
            daemon_pid,
            database_url,
        } => (|| {
            let repo = expand(&repo);
            let daemon = Daemon::new(&url, token.as_deref());
            daemon.preflight()?;
            let args = ScaleArgs {
                name: name.unwrap_or_else(|| {
                    repo.file_name()
                        .map_or("repo".into(), |n| n.to_string_lossy().into_owned())
                }),
                git_source: git_source.unwrap_or_else(|| repo.display().to_string()),
                repo,
                out: expand(&out),
                agents,
                samples,
                reps,
                daemon_pid,
                database_url,
            };
            scale::run(&daemon, &args)?;
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
            eprintln!("margincode-bench: {e}");
            ExitCode::FAILURE
        }
    }
}

/// `bench/tasks-routing/make-repo.py` builds the pilot git checkout the specs point at.
/// A clean tree has no `repo/` (it is generated, and not committed).
fn ensure_generated_repos(tasks: &[PathBuf]) -> Result<(), String> {
    for p in tasks {
        let dir = if p.is_dir() {
            p.clone()
        } else {
            p.parent().unwrap_or(Path::new(".")).to_path_buf()
        };
        let script = dir.join("make-repo.py");
        if !script.is_file() {
            continue;
        }
        let repo = dir.join("repo");
        if repo.join(".git").exists() {
            continue;
        }
        let py = ["python3", "python"]
            .into_iter()
            .find(|bin| {
                Command::new(bin)
                    .arg("--version")
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status()
                    .is_ok_and(|s| s.success())
            })
            .ok_or_else(|| format!("python is required to build {}", repo.display()))?;
        let out = Command::new(py)
            .arg(&script)
            .arg(&repo)
            .output()
            .map_err(|e| format!("{}: {e}", script.display()))?;
        if !out.status.success() {
            return Err(format!(
                "{}: {}",
                script.display(),
                String::from_utf8_lossy(&out.stderr).trim()
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_config(extra: &[&str]) -> RunConfig {
        let mut args = vec![
            "margincode-bench",
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
    fn work_is_removed_unless_keep_work() {
        assert!(!run_config(&[]).keep_work);
        assert!(run_config(&["--keep-work"]).keep_work);
    }

    #[test]
    fn the_agent_runs_as_the_harness_user_unless_agent_user_is_given() {
        let parse = |extra: &[&str]| {
            let mut args = vec![
                "margincode-bench",
                "run",
                "t",
                "--out",
                "o",
                "--budget-usd",
                "1",
            ];
            args.extend_from_slice(extra);
            match Cli::try_parse_from(args).unwrap().cmd {
                Cmd::Run {
                    agent_user,
                    av_ws_root,
                    ..
                } => (agent_user, av_ws_root),
                _ => unreachable!(),
            }
        };
        assert_eq!(parse(&[]).0, None);
        assert!(run_config(&[]).agent_user.is_none());
        let (user, ws_root) = parse(&["--agent-user", "nobody", "--av-ws-root", "/ws"]);
        assert_eq!(user.as_deref(), Some("nobody"));
        assert_eq!(ws_root, Some(PathBuf::from("/ws")));
        // only `run` has the flag: `calibrate` shares the session options and must not
        assert!(
            Cli::try_parse_from([
                "margincode-bench",
                "calibrate",
                "--packs",
                "p",
                "--budget-usd",
                "1",
                "--agent-user",
                "nobody"
            ])
            .is_err()
        );
    }

    #[test]
    fn av_arms_run_without_mcp_unless_asked() {
        assert!(!run_config(&[]).av_mcp);
        assert!(run_config(&["--av-mcp"]).av_mcp);
    }

    #[test]
    fn grok_is_a_vendor_and_routing_profile_reads_a_file() {
        let c = run_config(&["--vendor", "grok", "--grok-bin", "grok-test"]);
        assert_eq!(c.vendor, Vendor::Grok);
        assert_eq!(c.grok_bin, "grok-test");
        assert!(matches!(
            Cli::try_parse_from(["margincode-bench", "routing-profile", "cells.jsonl"])
                .unwrap()
                .cmd,
            Cmd::RoutingProfile { .. }
        ));
    }
}
