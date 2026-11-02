# agentvcs Plan 4: Benchmark Harness — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Measure the spec's success criterion: on ~10 replayed real tasks from two private repos, `av`-assisted agent sessions use materially fewer tokens before the first relevant edit and in total, with no drop in task success and fewer repeated dead ends, compared with baseline Claude Code + git. Also calibrate the packer's token-estimate factor against real API usage.

**Architecture:** A new `av-bench` crate (binary `av-bench`) with three layers. (1) **Task specs**: TOML files pinning a repo commit, a goal prompt, ground-truth relevant files, a verification command and, for some tasks, a seeded dead end. (2) **Runner**: for each task and each arm (`baseline`, `av`) it creates a fresh checkout, runs headless Claude Code (`claude -p --output-format stream-json`), and stores the raw event stream. (3) **Analyzer**: pure functions over the stored event streams produce metrics and a Markdown report. The analyzer is tested on recorded fixtures with no network; only the runner spends money.

**Tech Stack:** Rust (existing workspace), `serde`/`toml`, `clap`, the `claude` CLI in headless mode, `tiktoken-rs` (already a workspace dependency via `av-pack`), the daemon's HTTP API.

**Spec:** `docs/superpowers/specs/2026-10-18-agentvcs-design.md` ("Success criterion", "Testing").

## Global Constraints

- Hard cost control: `av-bench run` has **no default budget**: `--budget-usd` is required, and `--confirm` is required before any paid session launches (without it the command behaves as `--dry-run`). It stops launching sessions once the running total (reported cost, or the per-session cap when a result event is missing) would exceed the budget, and SIGKILLs `claude` when a session exceeds its timeout. The user sees the plan (task count × arms × estimated cost) and says go before `--confirm` is used.
- Every run is reproducible: task spec pins `repo_path`, `base_rev` (a git commit), model name and `max_turns`; results record the `claude --version`, model, `av` git revision and the date.
- **Three arms** (external-design note item 7): `baseline` (git), `av_no_packs` (the av engine and workflow with context packs reduced to the Task section) and `av` (full). Gains are attributed to the engine (baseline → av_no_packs) vs the context compiler (av_no_packs → av).
- **Headline metric is cost and wall time per accepted task** (accepted = verify passes), including the daemon's packer time and `expand` tokens; raw token counts are secondary. Report medians with bootstrap 95% confidence intervals per arm.
- **Concurrency sweep** (after the single-agent runs): 1, 4 and 8 agents on the same repo with a task mix that includes two deliberately conflicting tasks; measure accepted tasks/hour and failed integrations.
- Within each pair of arms, arms differ in exactly one thing. `baseline`: plain checkout, no `av`. `av`: same checkout registered with the daemon, the task started via `av task start`, the Claude Code hooks installed (Plan 3 Task 9) and MCP configured. The goal prompt text is identical.
- **No future leakage, same history surface:** each arm's source is built from the repo by `git clone --no-hardlinks --no-tags`, checkout of `<base_rev>`, then collapsed to a single orphan commit (`git checkout --orphan bench && git commit -m base`), with every other ref and `origin` removed and `git reflog expire --expire=now --all && git gc --prune=now` run. Neither arm can read the real fix or earlier reverted approaches from history (the av arm has no `.git` at all), so differences come from `av`'s records and packs, not from history access. The report states that a real-world git baseline would also have history, which makes this comparison conservative for `av`. Both arms.
- **Own stack:** the av arm talks to a separate compose project (`docker compose -p agentvcs-bench`) with `AV_IMPORT_ROOT=~/bench-work/src` and `AV_WS_ROOT=~/bench-work/ws`, because the daemon can only import from its mounted `/import` and must not touch the user's real database and workspaces.
- **Same environment:** `TaskSpec.setup_cmd` (e.g. `pip install -r requirements.txt`, `pnpm install --frozen-lockfile`) runs in each arm's working directory before the session, and `verify_cmd` runs in that same directory. The `av` arm's directory is a jj workspace (no `.git`); that and the injected context are the treatment, and nothing else may differ.
- **Isolated Claude config:** both arms run with `CLAUDE_CONFIG_DIR` pointing at an empty temp dir (credentials file copied in), `--setting-sources project`, `--strict-mcp-config --mcp-config <file>` (empty for baseline, the `av` server for the av arm), an explicit `--permission-mode` and `--no-session-persistence`, and `--max-budget-usd <per-session cap>`. The runner writes the av arm's hooks into the working dir's `.claude/settings.json` itself (Plan 3's `install.sh` only targets `~/.claude`).
- Runs happen in throwaway copies under `~/bench-work/`; the source repos are never modified. The benchmark never uses `--dangerously-skip-permissions` outside those copies, and passes an explicit `--allowedTools` list.
- Success is judged by the task's verification command exit code, run by the harness after the session, never by the agent's own claim.
- Results are reported with n, per-task values and medians; no claim of significance is made from n < 30 sessions per arm (the report says so in its header).
- Commits end with `Co-Authored-By: Claude Sonnet 5.5 <noreply@anthropic.com>`.

## Metrics (exact definitions)

| Metric | Definition |
|---|---|
| `tokens_to_first_edit` | Sum of `input + cache_creation + output` tokens (excluding `cache_read`) over assistant **messages deduplicated by `message.id`** (Claude Code emits one event per content block, each repeating the same `usage`; take each id's usage once) before the first `Edit`/`Write`/`MultiEdit` tool call whose target path is in the task's `relevant_files`. `None` if there is no such edit. |
| `tokens_total` | Same sum over the whole session. |
| `cost_usd` | The session's reported `total_cost_usd` from the final `result` event. If the stream has no `result` event (killed on timeout), charge the per-session cap passed as `--max-budget-usd`. |
| `success` | Verification command exit code == 0 after the session. |
| `repeated_dead_ends` | Count of seeded dead ends (task spec `[[dead_end]]` with `file_glob` + `pattern` regex) that the session re-introduces: an `Edit` `new_string`, a `Write` `content`, or a `MultiEdit` `edits[].new_string` that matches `pattern` on a `file_path` matching `file_glob`. `old_string` never counts (removing the dead-end code is not repeating it). Counted once per tool call. |
| `turns`, `tool_calls` | Counts from the event stream. |
| `estimate_error` | `(tiktoken_estimate(pack text) − measured) / measured`, where `measured` is the probe call's input tokens (same usage fields as above) minus the empty-prompt overhead. Calibration only (Task 6); `n/a` when `measured == 0`. |

## Review Focus

1. **Session ends with no relevant edit (agent gave up or edited other files)** → `tokens_to_first_edit = None`, session still counted in totals and success; medians skip `None` but the report prints the count of `None`s (Task 3).
2. **Truncated or partially written event stream** (killed by timeout) → analyzer returns metrics for the parsed prefix and flags `truncated = true`; never panics (Task 3).
3. **Budget cap reached midway** → remaining sessions are listed as `skipped (budget)`, the report is still produced, exit code non-zero (Task 5).
4. **Verification command hangs** → killed after the spec's `verify_timeout_secs`, counted as failure (Task 4).
5. **The `av` arm's daemon is down** → run aborts before spending anything with a clear message; an `av` session in which hooks silently failed is detected (the hook's counter file is empty) and marked `invalid`, not counted (Task 4).
6. **Task spec points at a commit that no longer exists / repo path missing** → `av-bench check` fails fast with the task name (Task 1).

## File Structure

```
crates/av-bench/   src/{main,spec,events,metrics,report,runner,calibrate}.rs
                   tests/{spec,metrics,report,runner}.rs   tests/fixtures/*.jsonl
bench/tasks/*.toml            # the ~10 real tasks (authored in Task 2)
bench/README.md
```

---

### Task 1: Task spec format and `av-bench check`

**Files:** Create `crates/av-bench/{Cargo.toml,src/main.rs,src/spec.rs}`, `crates/av-bench/tests/spec.rs`.

**Interfaces — Produces:**

```rust
pub struct TaskSpec {
    pub name: String, pub repo_path: PathBuf, pub base_rev: String, pub goal: String,
    pub relevant_files: Vec<String>, pub created_files: Vec<String>, pub verify_cmd: String, pub verify_timeout_secs: u64,
    pub max_turns: u32, pub model: String, pub setup_cmd: String, pub dead_ends: Vec<DeadEnd>,
}
pub struct DeadEnd { pub description: String, pub file_glob: String, pub pattern: String }  // pattern is a regex
pub fn load_spec(path: &Path) -> Result<TaskSpec>;
pub fn check_spec(spec: &TaskSpec) -> Result<()>;   // repo exists, base_rev resolves via `git rev-parse --verify <rev>^{commit}`, every `relevant_files` entry exists at base_rev (`created_files` are files the real fix adds: they must NOT exist at base_rev, and an edit to either list counts as a relevant edit), every pattern compiles
```

- [ ] **Step 1: Failing tests** (fixture git repo created in a tempdir): `load_spec_parses_all_fields`, `check_spec_ok_for_valid_task`, `check_rejects_missing_repo_naming_the_task` (Review Focus 6), `check_rejects_unknown_rev`, `check_rejects_relevant_file_absent_at_rev`, `check_accepts_created_file_absent_at_rev_and_rejects_it_if_present`, `check_rejects_invalid_regex`, `check_rejects_rev_starting_with_dash` (no option injection into git).
- [ ] **Step 2:** `cargo test -p av-bench --test spec` → FAIL. **Step 3:** Implement with `toml`, `regex`, and `git` via `std::process::Command` with `--` separators. **Step 4:** PASS. **Step 5:** Commit `feat(bench): task spec format and check`.

### Task 2: Author the ten tasks

**Files:** Create `bench/tasks/*.toml` (10 files), `bench/README.md`.

This task is research on the user's real repos, not code. Choose from the git history of two private repos (5 tasks each): commits that fixed a bug or added a small feature (diff touching 1–4 files) and that have a runnable verification (test file or typecheck). For each: `base_rev` = the commit's parent, `goal` = a one-paragraph description written from the commit message **without naming files**, `relevant_files` = files the real commit changed, `verify_cmd` = the narrowest command that fails at `base_rev` and passes at the commit (confirm both by running it). Pick at least 3 tasks where git history shows a *reverted or abandoned approach* and add it as a `[[dead_end]]`.

- [ ] **Step 1:** List candidate commits (`git log --oneline --stat`), shortlist 15, and show the shortlist to the user in one message with the one-line reason for each; the user's picks (or "your choice") finalize the ten. (The commits are the user's code and the dead-end choices are judgement calls; this is the one place this plan asks.)
- [ ] **Step 2:** For each chosen task write the TOML, then confirm the verify command **fails** at `base_rev` and **passes** at the real commit in a throwaway `git clone --no-hardlinks` under `~/bench-work` (never `git worktree add`, which writes into the source repo's `.git`), deleted afterwards. Record both exit codes in `bench/README.md`.
- [ ] **Step 3:** `cargo run -p av-bench -- check bench/tasks/*.toml` → every task `ok`.
- [ ] **Step 4:** Commit `bench: ten tasks from two private repos`.

### Task 3: Event parsing and metrics

**Files:** Create `crates/av-bench/src/{events,metrics}.rs`, `crates/av-bench/tests/metrics.rs`, `crates/av-bench/tests/fixtures/{edit_first.jsonl,no_edit.jsonl,truncated.jsonl,dead_end.jsonl}`.

**Interfaces — Consumes:** `TaskSpec` (Task 1). **Produces:**

```rust
pub struct SessionMetrics {
    pub tokens_to_first_edit: Option<u64>, pub tokens_total: u64, pub cost_usd: f64,
    pub turns: u32, pub tool_calls: u32, pub repeated_dead_ends: u32, pub truncated: bool,
}
pub fn parse_stream(jsonl: &str) -> ParsedSession;      // tolerant: bad/partial lines are counted, not fatal
pub fn compute(spec: &TaskSpec, s: &ParsedSession) -> SessionMetrics;
```

Fixtures are hand-written minimal `stream-json` transcripts matching the shapes the installed `claude` actually emits; **Step 1 generates one real tiny session with `claude -p --output-format stream-json --verbose "say hi"` (a few cents) and builds the fixtures' field names from it** instead of from memory.

- [ ] **Step 1:** Capture the real event shapes (assistant `message.usage` fields, `tool_use` blocks with `name` and `input.file_path`, the final `result` event with `total_cost_usd`); write them into `bench/README.md`.
- [ ] **Step 2: Failing tests:** `first_edit_counts_only_tokens_before_it_excluding_cache_reads`, `edit_to_non_relevant_file_does_not_count_as_first_edit`, `no_relevant_edit_gives_none_but_totals_are_counted` (Review Focus 1), `truncated_stream_returns_prefix_metrics_and_flag` (Review Focus 2), `dead_end_pattern_match_counts_once_per_tool_call`, `write_and_multiedit_count_as_edits`.
- [ ] **Step 3:** Run → FAIL. **Step 4:** Implement. **Step 5:** PASS; commit `feat(bench): event parsing and metrics`.

### Task 4: Runner

**Files:** Create `crates/av-bench/src/runner.rs`, `crates/av-bench/tests/runner.rs`.

**Interfaces — Produces:**

```rust
pub enum Arm { Baseline, AvNoPacks, Av }   // AvNoPacks: av workspaces/records/hooks, but session-start and `get_context` return only the Task section
pub struct RunOutcome { pub task: String, pub arm: Arm, pub events_path: PathBuf, pub verify_exit: Option<i32>,
                        pub valid: bool, pub invalid_reason: Option<String> }
pub async fn run_session(spec: &TaskSpec, arm: Arm, cfg: &RunConfig) -> Result<RunOutcome>;
pub struct RunConfig { pub work_root: PathBuf, pub claude_bin: String, pub av_url: Option<String>, pub allowed_tools: Vec<String> }
```

Steps inside `run_session`: `git worktree`-free copy (`git clone --no-hardlinks --no-tags <repo> <work>`, `git checkout --detach <base_rev>`, then the history scrub from Global Constraints); for `Av`, the daemon's `init_repo` bare-clones an existing local directory under its `/import` mount and imports that clone's HEAD branch as trunk. So the scrubbed single-commit copy (above) is placed at `~/bench-work/src/<task>` (the bench stack's import root), and `POST /v1/repos` is sent with the in-container path `/import/<task>`; the task workspace then contains exactly the `base_rev` tree (assert byte equality of every file against `git show <base_rev>:<path>` before the session). Then `av task start`, seed the dead ends as `attempt` records (see below), install the hooks into the work dir's `.claude/settings.json` (project-level, not the user's global file), add MCP config; run `claude -p "<goal>" --output-format stream-json --verbose --max-turns N --model M --allowedTools …` with cwd = workspace (for `Av`, the task workspace path); on exit run `verify_cmd` with `verify_timeout_secs`; for `Av`, after the session `POST /v1/tasks/{id}/checkpoint` then `GET /v1/tasks/{id}` and mark the run `invalid` if the hooks never fired (the hook script appends a line to `<workspace>/.av-hook-count` on every checkpoint; `head_change == base_change` alone is not evidence of failure, because an agent that edits nothing is a valid result). Hooks and the project-scoped MCP config (`<workspace>/.mcp.json`, passed with `--mcp-config`; never `claude mcp add`, which writes `~/.claude.json`) are written **only inside the workspace**, which is also claude's cwd, `setup_cmd`'s cwd and `verify_cmd`'s cwd. When `AV_TOKEN` is set, forward it on all daemon and MCP calls and abort before spending if the daemon answers 401 (Review Focus 5). **Dead-end seeding (av arm only):** after `av task start`, each `[[dead_end]]` is written with `POST /v1/tasks/{id}/records` as `kind=attempt`, anchored to its `file_glob` files. This measures whether *existing* attempt records stop repeats, not whether agents write them; the report says so.

- [ ] **Step 1: Failing tests** using a **fake `claude`** shell script (the `claude_bin` setting) that writes a canned stream and edits a file: `baseline_run_checks_out_base_rev_and_stores_events`, `verify_exit_code_recorded`, `verify_timeout_kills_command_and_counts_failure` (Review Focus 4), `source_repo_is_untouched_after_run`, `av_arm_aborts_before_spending_when_daemon_down` (Review Focus 5; the fake claude must never be invoked — assert via a marker file), `av_arm_without_hook_counter_is_marked_invalid`, `av_arm_workspace_bytes_equal_base_rev` (driven through the real daemon router + `TestDb` on a fixture repo, not a stub, so request bodies and routes are the real ones), `fake_claude_must_find_av_task_and_settings_in_its_cwd` (the fake `claude` script records `pwd` and fails unless `.av-task`, `.claude/settings.json` and `.mcp.json` are in it).
- [ ] **Step 2:** Run → FAIL. **Step 3:** Implement. **Step 4:** PASS. **Step 5:** Commit `feat(bench): session runner for baseline and av arms`.

### Task 5: Orchestration, budget cap, report

**Files:** Create `crates/av-bench/src/report.rs`; modify `main.rs`. Test: `crates/av-bench/tests/report.rs`.

**Interfaces — Produces:** `av-bench run --tasks bench/tasks --reps 3 --budget-usd 20 --out bench/results/<date>` (order: rep-major, for rep in 1..R, for each task, both arms in a shuffled order seeded by `--seed`, so drift hits both arms equally; before launching, the report prints how many valid sessions per arm the run is expected to yield and warns if that is below 30), `av-bench report <results dir>`; `pub fn render_report(rows: &[Row]) -> String` with a header stating n per arm, the "no significance claim for n < 30" note, and that `tokens_*` exclude `cache_read` (so per-turn re-reads of context are not counted for either arm); per-task table (both arms: tokens_to_first_edit, tokens_total, success rate, repeated dead ends, cost) and an overall median row plus the relative change `(av − baseline) / baseline` (printed as `n/a` when the baseline median is missing or 0; never divide by zero); list of skipped (budget) and invalid runs.

- [ ] **Step 1: Failing tests:** `report_matches_golden_for_fixture_rows`, `median_skips_none_and_prints_none_count`, `budget_cap_skips_remaining_sessions_and_exits_nonzero` (Review Focus 3; fake claude reporting a fixed cost), `dry_run_prints_plan_with_session_count_and_estimated_cost_without_running` (`--dry-run`).
- [ ] **Step 2:** Run → FAIL. **Step 3:** Implement. **Step 4:** PASS. **Step 5:** Commit `feat(bench): orchestration, budget cap and report`.

### Task 6: Token-factor calibration

**Files:** Create `crates/av-bench/src/calibrate.rs`. Test: `crates/av-bench/tests/` unit tests in the module.

**Interfaces — Produces:** `av-bench calibrate --packs <dir> [--model M]`: for each stored pack text, count tiktoken cl100k tokens, then measure real input tokens via one `claude -p --output-format json --max-turns 1` call (system prompt overhead measured once with an empty prompt and subtracted); outputs `measured_input_tokens / cl100k_count` per pack and a recommended `AV_TOKEN_FACTOR_CLAUDE` (median of those ratios, rounded up to two decimals). Packs in the bench stack are built with `AgentFamily::Claude` (pass `family=claude` on context requests; `POST /v1/tasks` currently defaults to `Other`, so the bench runner sets it), and the factor is read once at daemon start, so restart the bench daemon after changing it. Plan 1 reads `AV_TOKEN_FACTOR_CLAUDE` / `AV_TOKEN_FACTOR_OTHER` (`crates/av-pack/src/tokens.rs`); this task only recommends a value.

- [ ] **Step 1: Failing tests:** `ratio_and_median_computed_from_pairs`, `recommended_factor_rounds_up_to_two_decimals`, `overhead_is_subtracted_once`, `empty_input_set_is_an_error_not_a_nan`.
- [ ] **Step 2:** Run → FAIL. **Step 3:** Implement the pure parts; the CLI call is a thin wrapper. **Step 4:** PASS. **Step 5:** Commit `feat(bench): token factor calibration`.

### Task 7: The real run (needs the user)

Not automatable end to end: it needs Docker (daemon stack), the user's Claude Code login and money.

- [ ] **Step 1:** `av-bench run … --dry-run` and show the user: sessions (10 tasks × 2 arms × 3 reps = 60), model, and estimated cost range from a 2-task pilot (run the pilot first with `--reps 1 --budget-usd 5`).
- [ ] **Step 2:** On the user's explicit go-ahead only, run the full set with the agreed `--budget-usd`.
- [ ] **Step 3:** `av-bench report`, commit `bench/results/<date>/report.md` (raw event streams stay out of git; `.gitignore` them), and write the conclusion into `docs/`: which metrics moved, by how much, with n and the caveats. If the criterion is not met, say so and list the three biggest token sinks seen in `av`-arm sessions as input for the next iteration.
- [ ] **Step 4:** Commit `docs: benchmark results`.
