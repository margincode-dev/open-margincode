# Scholion Plan 6: Evaluation — what to measure, against what, and in which order

**Why:** Scholion makes five claims and has evidence for none of them beyond "the workflow holds up".
The README says so. Plan 4 built a single-agent benchmark that has never run. Two eight-agent
DeepSeek stress runs showed coordination works, but they had no git baseline. In those runs the
Plan 5 safety features (carried approvals, convention checks, the post-merge runner) never fired.
This plan says what to measure, what to compare it with, and what result changes what we do. The
cheap, deterministic evals come first, so money is spent only on questions they cannot answer.

## The claims under test

| # | Claim | Evidence today |
|---|---|---|
| C1 | A context pack makes a cold-start agent cheaper or more successful | none (Plan 4 harness built, not run) |
| C2 | Recorded decisions and dead ends stop the next agent repeating them | anecdotal: 203 pack exposures, 1 expand |
| C3 | Many agents can share one repo through Scholion with main kept green | two runs, 8/8 merged, but no git baseline |
| C4 | Review with bundles, conventions and checks catches more real problems | one conflict-marker bug caught by a reviewer, missed once |
| C5 | Plan 5 features (carry, checks, runner, bisect, revert, `why`) do their job | unit and API tests only; none fired in the field |

## Principles

- **Strong baselines, not straw men.** For C1 the baseline is a git checkout plus a good
  `AGENTS.md`/`CLAUDE.md`. For C2 it is git plus the previous agent's commit message *and* a
  `NOTES.md` it was asked to write. For C3 it is git worktrees with an orchestrator that merges,
  tests and sends conflicts back. Beating bare git proves little; beating a careful human setup is
  the claim worth making.
- **Paired designs.** Each task runs in every arm, so arms are compared per task: Wilcoxon
  signed-rank on cost and tokens, McNemar on accepted or not, and a bootstrap over tasks for
  confidence intervals. Pairing needs far fewer sessions than independent samples.
- **Pre-register.** Before any paid run, write down the hypotheses, the primary metric (cost per
  accepted task) and the decision rule in `bench/results/<date>/PREREG.md`. Nothing is added after
  seeing the results without being labelled exploratory.
- **Count Scholion's overhead honestly.** The MCP server adds about 24 tool schemas to every turn,
  and hooks add pack text. Report total tokens, not just tokens saved in the reading phase.
- **Cheap model wide, expensive model narrow.** Sweep with DeepSeek (about $0.02 per session in
  the stress runs), then confirm the headline numbers on Claude Sonnet with fewer, chosen tasks.
- **Reproducible by strangers.** The public repo must contain the task sets that results are
  claimed on. The Plan 4 tasks come from a private repo, so they stay a private pilot.

## Evals, cheapest first

### E1. Offline pack retrieval (free, deterministic, minutes per run) — inner loop for C1

There is no agent in this eval. For a task with a known fix, the files and symbols the fix
changed are the gold set. Build the pack for the task's goal at the base commit and measure:
- **Recall:** the share of gold files and gold symbols the pack shows (by tier: handle, summary or
  full) within budgets of 2k, 4k and 8k tokens.
- **Precision:** the share of pack tokens that belong to gold code.
- **The rank of the first gold item.**

Compare with three baselines that need no Scholion:
- BM25 over files;
- embedding top-k over chunks (the same embedder);
- an Aider-style repo map (tree-sitter symbol graph ranked by PageRank, the same budget).

Task sources:
- the ten Plan 4 tasks (private);
- about 100 instances of a public SWE-bench Verified subset (only the repo, the base commit and
  the gold patch are needed; no Docker images);
- fixes mined from public repos after June 2026, which reduces training-data contamination.

Python, TypeScript and Rust are all covered, since the index parses all three.

New code: `scholion-bench retrieval TASKS --budget 2000,4000,8000`, which writes `retrieval.jsonl`
and a report. It reuses `Packer` in-process, like the daemon tests do.

Decision rule:
- If pack recall at 4k is **not** at least the best baseline's, fix the packer (scoring, seeds,
  section shares) before E3. Spending money to measure a packer that loses offline is waste.
- Later, check whether recall predicts the agent results of E3. If it does, E1 becomes the cheap
  proxy for every packer change, and runs in CI on a fixed task set as a regression gate.

### E2. System properties (free, deterministic) — the foundation under C3 and C5

- **Merge correctness, fuzzed.** Generate random concurrent task edits on random trees, then merge
  them in random orders. The invariants:
  - no committed line is lost;
  - main advances only through merge and revert;
  - a conflict is reported if and only if two edits overlap;
  - the result agrees with `git merge-tree` on clean cases.

  Put this in a `proptest` suite in `scholion-vcs`, with a fixed seed in CI and a long run
  offline.
- **Determinism.** The same inputs produce byte-identical packs, after the clock and ids in the
  receipt line are removed.
- **Scale and latency.** Import public repos of about 1k, 10k and 50k files (for example
  `ripgrep`, `django`, `tokio`) and measure:
  - import and index time;
  - incremental checkpoint time;
  - pack latency at p50 and p95;
  - `why` latency (cold and cached);
  - merge throughput with 1, 8 and 32 concurrent tasks;
  - daemon memory and Postgres size.

  A no-LLM load generator (the `tests/swarm.rs` pattern, as a binary) simulates 100 agents.
- **Token estimate accuracy.** Compare the packer's estimate with the real tokenizer count (the
  `calibrate` path already exists) for each model family.

Decision rule: any lost-work or false-clean merge is a release blocker. Latency targets come from
the first measurement (record it, then do not regress more than 20 %).

### E3. Single-agent cold start (Plan 4 harness) — C1

- **Arms:**
  - `baseline`: git;
  - `baseline+agents_md`: git plus a hand-written project guide of the length a careful
    maintainer would write, about 1–2k tokens. This arm is new; it is the honest competitor;
  - `av_no_packs`;
  - `av`.
- **Pilot:** the ten private tasks × 4 arms × 1 rep on DeepSeek (about $1), then on Sonnet (the
  existing $5 cap). The pilot checks the plumbing and the variance, not the hypotheses.
- **Main run:** the public task set from E1, limited to instances whose tests run with `pip`
  (no Docker), with 30–50 tasks × 4 arms × 2 reps:
  - on DeepSeek, roughly $5–10;
  - then on Sonnet, the 20 tasks with the largest DeepSeek arm differences (both directions),
    × 4 arms × 1 rep, roughly $40–80.
- **Metrics:**
  - primary: cost per accepted task;
  - secondary: acceptance rate, total tokens, tokens to first relevant edit, wall time, turns,
    and repeated dead ends.
- **New harness work:**
  - a `--provider` flag (base URL and model passed through; the key read from a file outside the
    repo, never printed);
  - the `baseline+agents_md` arm;
  - task specs for public repos.

Decision rule:
- If `av` does not beat `baseline+agents_md` on cost per accepted task (paired 95 % CI excluding
  zero) on the main run, the README stops implying token savings. Scholion is then positioned on
  coordination and audit (C3–C5).
- If `av_no_packs` already wins, the engine (workspaces and tools) carries the value, not the
  packer.

### E4. Knowledge relay — C2, the core thesis

Pairs of tasks on the same code, where task A's agent learns something task B's agent needs:
- **Trap tasks:** the obvious approach fails for a reason found only by trying (a dependency quirk,
  a hidden invariant, a performance cliff). Seed each from a real commit pair where history shows
  a reverted attempt, like the Plan 4 tasks `summary-context-overflow` and
  `no-results-disclosure`, or construct it.
- **Decision tasks:** A settles a design choice with a reason (an error format, a naming rule). B
  must follow it in a different file.
- **Interrupted handoff:** A is stopped at half its budget, and B finishes the task.

Arms for B:
- git with A's commits and messages;
- git plus a `NOTES.md` that A was told to keep;
- Scholion (A's records reach B through packs and the handoff bundle).

A runs once per pair, and its output is frozen and reused in every B arm, so only the channel
differs.

Metrics:
- **Repeated dead end:** B tries what A rejected. Detected from tool calls, with the existing
  `repeated_dead_ends` matcher.
- **Decision violated:** checked by a hidden test.
- **Cost and acceptance for B.**

Size: 20 pairs × 3 arms × 2 reps on DeepSeek, roughly $3.

Decision rule: if Scholion does not cut repeated dead ends against `NOTES.md` by a clear margin,
the "why next to the code" thesis is unproven. Look at whether records were shown but ignored (a
pack problem) or never written (a prompting problem). Each failure has a different fix.

### E5. Multi-agent coordination with a git baseline — C3

- Move the stress orchestrator into the repo as `scholion-bench swarm`: specs for features and
  rounds, an event log, and the collaboration analyzer. Scrub local paths; the provider comes from
  flags.
- **Add a git arm:**
  - agents work in `git worktree`s;
  - the orchestrator merges each branch into main, runs the tests, and returns conflicts or red
    tests to the owner;
  - the same prompts apart from the tool instructions;
  - the same reviewer.
- **Vary the number of agents** (2, 4, 8, 16) and the hot-file pressure: features that share
  `store.py` and `style.css`, against disjoint features. Run 3 seeds per cell on DeepSeek, roughly
  $1 per 8-agent run, so about $25 for the grid.
- **Metrics:**
  - features merged;
  - rounds to all merged;
  - conflicts, and who resolved them;
  - resolution correctness (hidden tests after each merge);
  - main-green rate;
  - semantic breaks (clean merges that fail tests);
  - re-reviews;
  - cost and wall time;
  - knowledge flow (pack exposures against explicit expands).

Decision rule: if the git arm matches Scholion on merged features, green main and cost at 8 and
16 agents, the coordination layer adds nothing over worktrees for this workload. Then either find
the workload where it does (hot files, long-lived tasks, handoffs), or drop the claim.

### E6. Review: planted defects — C4

Take merged features from E5 and plant one known defect each:
- leftover conflict markers;
- a convention violation;
- a missing cross-feature obligation (like comments never calling activity `_log` in the stress
  run);
- an off-by-one with tests left passing;
- a security slip (an unescaped template value).

Mix in an equal number of clean submissions.

The reviewer sees one of two things:
- a plain diff;
- the Scholion bundle (the diff, records, conventions, check results and the interdiff since the
  last approval).

Metrics: detection rate per defect class, false request-changes on clean submissions, and review
cost. Size: 60 submissions × 2 arms × 2 reps on DeepSeek, under $2.

### E7. Plan 5 features under the conditions they were built for — C5

Each feature gets one scripted scenario. Run it through the API with no LLM, or with a single
cheap agent where judgement is needed:
- **Carried approval:**
  - Saving: README-only resolutions and pure rebases, counting the re-reviews saved.
  - Safety: twenty attempts to sneak a code change through a carry (same hunk count, whitespace
    games, mode changes, a change hidden in a carried glob). The required result is **zero**
    carries.
- **Enforced conventions:** submissions that violate a check-bearing convention. Approval must be
  refused every time, and allowed once the check passes.
- **Runner, bisect and revert:** two textually clean merges that break main together. Measure:
  - time from the bad merge to red health;
  - whether `bisect` names the right merge;
  - whether `revert` restores green;
  - whether the task can be re-merged after a fix.
- **`why` against git blame:** 30 questions of the form "why is this line like this?" with a known
  answer (the decision record behind it). An agent answers with `av why`, or with
  `git blame` + `git log`. Grade the answers against the record, with a fixed rubric and a second
  model as grader, plus spot checks by hand.

## Order, cost and deliverables

| Phase | Evals | Money | Output |
|---|---|---|---|
| 0 | E1, E2 | $0 | `scholion-bench retrieval`, the merge proptest suite, the load generator, the public task set, a baseline report |
| 1 | E3 pilot, E7 | ≈ $6 | harness `--provider` and `agents_md` arm, pilot report, Plan 5 feature report |
| 2 | E4, E6 | ≈ $5 | relay and review task sets, reports |
| 3 | E5 | ≈ $25 | `scholion-bench swarm` with the git arm, scaling report |
| 4 | E3 main (DeepSeek, then Sonnet) | ≈ $50–90 | headline numbers for the README, with PREREG and raw rows committed |

Each phase ends with a published report (an artifact page plus `bench/results/<date>/`) and a
go/no-go for the next. Phase 4 runs only if E1 shows the packer at least matching its baselines,
and the Phase 1 pilot shows the harness is sound.

## Threats to validity

- **Contamination:** public fixes may be in training data. Prefer commits after the models'
  cutoffs, and report the results split by commit date.
- **Author bias:** the private tasks come from the author's own repo, and the author's agents
  wrote Scholion. Public tasks and pre-registration are the guard.
- **Prompt asymmetry:** the arms differ only in tool instructions. Keep the goal text identical,
  and publish all prompts.
- **Model and agent coverage:** Claude Code with DeepSeek and Sonnet only at first. A Codex-CLI or
  MCP-only arm is a later extension.
- **Small samples:** state the number of sessions next to every number. Claim no significance
  below the paired-test threshold.
- **Budget truncation:** sessions killed by the cost cap count as not accepted, and are reported
  separately.

## Out of scope

- Human-developer user studies.
- Hosted multi-tenant load.
- Comparisons with other agent-VCS products (none is publicly runnable).
