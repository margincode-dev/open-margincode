# scholion-bench: running the benchmark

> **The task set is not included.** The ten tasks this guide was written for come from a private
> repository, so `bench/tasks/` is left out of the public repo. The harness works with any task
> specs in the same format (see `crates/scholion-bench/src/spec.rs`); a minimal one:
>
> ```toml
> name = "my-task"
> repo_path = "~/projects/my-repo"   # a local git repo
> base_rev = "abc1234^"              # where the agent starts
> fix_rev = "abc1234"                # a known-good solution, for checking the tests
> goal = "What the agent should do, in a paragraph."
> relevant_files = ["src/app.py"]
> verify_cmd = "python -m pytest -q tests/test_my_task.py"
> verify_timeout_secs = 300
> max_turns = 40
> ```


> Two public task sets ship instead, each with a script that builds its repo
> ([Public task sets](#public-task-sets)): `bench/tasks-tallyho` (a small Python shop library)
> and `bench/tasks-scholion` (this repository with injected bugs).

Plan 4 (`docs/superpowers/plans/2026-10-18-agentvcs-plan-4-benchmark.md`). Ten real tasks from
`bench-repo` run with headless Claude Code in three arms that start from the same scrubbed
single-commit tree:

| arm | what the agent gets |
|---|---|
| `baseline` | a git checkout (one commit, no history) and nothing else |
| `av_no_packs` | an av task workspace, av hooks and the `av` CLI (the av MCP tools only with `--av-mcp`), but every pack is just the Task section |
| `av` | the same with full context packs |

The report leads with **cost and wall time per accepted task** (accepted = the task's hidden tests
pass after the session), and splits the change into engine (`baseline → av_no_packs`) and
context compiler (`av_no_packs → av`).

**Only `scholion-bench run --confirm` spends money.** Without `--confirm` it prints the plan and exits.
`--budget-usd` is required, and sessions stop launching once the money spent so far plus one
session's cap (`--session-cap-usd`, default $2.00, enforced by `claude --max-budget-usd`) would
exceed it.

## Tonight, step by step

### 0. Build

```bash
cd scholion && git pull
cargo build --release -p scholion-bench -p scholion-cli
export PATH=$PWD/target/release:$PATH        # scholion-bench and av (the hooks run `av`)
```

### 1. A Python env for the bench-repo tests

`setup_cmd` and `verify_cmd` run with the PATH you start `scholion-bench` with. The agent (`claude`)
gets the same PATH plus any directories a task lists in `path_prepend` (put first, `~/` expanded;
`check` warns about one that is missing); `path_prepend` does not apply to `setup_cmd` or
`verify_cmd`. The tallyho and relay tasks use `~/bench-work/venv/bin` there so the agent can run
pytest. Either put a venv on the PATH yourself or activate one that can run bench-repo's unit tests
at every task's base commit (both the Chroma era and the Qdrant era), because `verify_cmd` needs it
too:

```bash
python3 -m venv ~/bench-work/venv && . ~/bench-work/venv/bin/activate
pip install -r requirements.txt pytest
```

`sentence-transformers` is in the requirements but the tests fake it, so leaving it out (and
torch with it) is fine.

### 2. The bench stack (never your real one)

A separate compose project with its own volumes, ports and folders, and **with the semantic
layer on**: av arms refuse to start against a daemon without one (`--allow-no-semantic`
overrides), and each av session waits until its repo's embeddings are in Qdrant.

```bash
export UID GID=$(id -g)
export AV_DAEMON_PORT=7430 AV_PG_PORT=5433 AV_QDRANT_PORT=6343 AV_QDRANT_GRPC_PORT=6344 AV_TEI_PORT=8081
export AV_WS_ROOT=~/bench-work/ws AV_IMPORT_ROOT=~/bench-work/src
mkdir -p ~/bench-work/ws ~/bench-work/src
docker compose -p scholion-bench -f deploy/docker-compose.yml up -d --build
curl -s http://127.0.0.1:7430/healthz        # ok
```

Docker Desktop without WSL integration cannot mount WSL paths (README, "Docker Desktop without
WSL integration"). Then put **all** of bench-work on the Windows drive, the baseline copies
included, so every arm works on the same filesystem (`/mnt/c` is slower, and wall time is a
headline metric):

```bash
export AV_WS_ROOT=/mnt/c/Users/<you>/bench-work/ws   AV_WS_MOUNT='C:\Users\<you>\bench-work\ws'
export AV_IMPORT_ROOT=/mnt/c/Users/<you>/bench-work/src AV_IMPORT_MOUNT='C:\Users\<you>\bench-work\src'
BENCH="--work-root /mnt/c/Users/<you>/bench-work --import-dir /mnt/c/Users/<you>/bench-work/src"
```

If the stack has `AV_TOKEN` set, export the same `AV_TOKEN` for scholion-bench.

### 3. Check the tasks

```bash
scholion-bench check bench/tasks                  # every task: ok
```

Task files point at `~/projects/bench-repo`; `--repos DIR` (or `AV_BENCH_REPOS`) uses `DIR/bench-repo`
instead.

### 4. Pilot (6 sessions, about $5 at most)

```bash
COMMON="--av-url http://127.0.0.1:7430 --session-cap-usd 2 $BENCH"
scholion-bench run bench/tasks --only request-top-k,chunk-overlap-guard --reps 1 \
  --budget-usd 5 --out bench/results/pilot $COMMON              # prints the plan, spends nothing
scholion-bench run bench/tasks --only request-top-k,chunk-overlap-guard --reps 1 \
  --budget-usd 5 --out bench/results/pilot $COMMON --confirm
```

Look at `bench/results/pilot/report.md` and at one session directory per arm under
`~/bench-work/runs/pilot/`: `events.jsonl` (the transcript), `claude.stderr`, `verify.log`,
`hook-count` (av arms; empty = invalid run), `pack.md`.

### 5. The full run

```bash
scholion-bench run bench/tasks --reps 3 --budget-usd 100 --out bench/results/$(date +%F) \
  --pilot bench/results/pilot $COMMON            # plan + estimate from the pilot's medians
scholion-bench run bench/tasks --reps 3 --budget-usd 100 --out bench/results/$(date +%F) \
  --pilot bench/results/pilot $COMMON --confirm
```

10 tasks × 3 arms × 3 reps = 90 sessions; the report says no significance is claimed below 30
valid sessions per arm. The order is rep by rep, task by task, with the arms shuffled per task
(`--seed`). Interrupted? Run the same command again: finished sessions in `rows.jsonl` are kept
and count toward the budget, sessions skipped for budget are retried. `scholion-bench report DIR`
re-renders `report.md`.

Commit `bench/results/<date>/` (`rows.jsonl`, `meta.json`, `report.md`); the transcripts stay in
`~/bench-work/runs`.

### 6. Token-factor calibration (optional, cents)

```bash
scholion-bench calibrate --packs ~/bench-work/runs --budget-usd 1 $BENCH              # plan
scholion-bench calibrate --packs ~/bench-work/runs --budget-usd 1 $BENCH --confirm    # one probe per pack
```

It prints the recommended `AV_TOKEN_FACTOR_CLAUDE`; set it on the bench daemon and restart it.

## How a session runs

1. **Source**: `git clone --no-hardlinks --no-tags` of the repo, checkout of `base_rev`, collapsed
   to one orphan commit with every other ref, the remote, the reflog and unreachable objects
   removed. Neither arm can read the real fix or earlier approaches from history.
2. **av arms**: that copy goes to the import folder, is registered with the bench daemon as its
   own repo (`b<run>-<task>-<arm>-<rep>`), and a task is started (`pack_mode: task_only` for
   `av_no_packs`). The workspace is checked byte for byte against `base_rev`, the task's
   `[[dead_end]]`s are recorded as `attempt` records, `.av-task` is written, and the runner waits
   for the embeddings.
3. **Claude Code**: `claude -p "<goal>" --output-format stream-json --verbose` with an empty
   `CLAUDE_CONFIG_DIR` (your `~/.claude/.credentials.json` copied in), `--setting-sources project`,
   `--settings` and `--strict-mcp-config --mcp-config` files kept outside the workspace (empty for
   the baseline; the av hooks otherwise, plus the av MCP server with `X-Av-Task` only if you pass
   `--av-mcp` (the default is hooks and CLI alone, as in the shipped Claude Code integration), and `--hook-code
   full|signatures` is passed to the session-start hook as `--code`; each `--hook-arg=X`, e.g.
   `--hook-arg=--no-map --hook-arg=--max-code=3`, is passed to it as well and named in the
   report header),
   `--permission-mode acceptEdits`, `--allowedTools Read,Edit,Write,MultiEdit,Glob,Grep,LS,Bash`
   (plus `mcp__av` in the av arms with `--av-mcp`), `--no-session-persistence`, `--max-budget-usd`, and a timeout
   that kills the whole process group (`session_timeout_secs`, default 30 min). The goal text is
   the same in every arm.
4. **Verify**: av arms checkpoint, and a run whose hooks never fired is `invalid`. Then the
   task's hidden tests are copied in (`hidden_tests` from the real fix, `local_tests` written for
   the benchmark), overwriting whatever the agent left at those paths, and `verify_cmd` runs with
   `verify_timeout_secs`. Exit 0 = accepted.

Bash is allowed, so agents can run tests; it runs inside the throwaway copies under bench-work.

## The ten tasks

All from `bench-repo` (`~/projects/bench-repo`). Each verify command was confirmed to **fail at
`base_rev` and pass at `fix_rev`** (2026-10-22, Python 3.13, a fresh clone per check), and a
rehearsal with an oracle `claude` that applies the real fix accepted all 30 sessions (10 tasks ×
3 arms) through the real daemon and hooks.

| task | base → fix | hidden tests | at base | at fix | dead end |
|---|---|---|---|---|---|
| no-results-disclosure | `0eb9117^` → `0eb9117` | the commit's `test_agent_runtime.py` | exit 1 | exit 0 | yes |
| tax-source-links | `983083a^` → `983083a` | the commit's `test_openai_compat.py` | exit 1 | exit 0 | |
| warmup-reranker | `bec1f1e^` → `bec1f1e` | the commit's `test_main_api.py` | exit 1 | exit 0 | |
| chunk-overlap-guard | `bac3390^` → `bac3390` | written: 8 tests | 5 fail | 8 pass | |
| request-top-k | `d41137a^` → `d41137a` | written: 2 tests | 2 fail | 2 pass | |
| paginated-collection-scan | `1816d7d^` → `1816d7d` | written: 4 tests | 4 fail | 4 pass | |
| summary-context-overflow | `cf460a4^` → `cf460a4` | written: 4 tests (fake vLLM) | 3 fail | 4 pass | yes (history) |
| retrieve-overrides | `18ce9a2^` → `18ce9a2` | written: 3 tests | 2 fail | 3 pass | |
| per-pdf-progress | `ff3a2b6^` → `ff3a2b6` | written: 3 tests | 1 fail | 3 pass | |
| adaptive-top-k | `f8e4ddc^` → `0a05a74` | the repo's own tests for the feature, minus one on a private helper | 5 fail, 1 error | 6 pass | |

Notes on the choices:

- **Hidden tests decide acceptance.** Six of the real fixes had no test, so the benchmark has its
  own in `bench/tasks/files/`. They test behaviour through code that exists at `base_rev` and avoid
  helper names the fix invented. Where the fix's own tests need a name the agent cannot guess,
  the goal states it (`NO_GROUNDING_ANSWER`, the `_reranker` warm-up key, the `adaptive_*`
  settings).
- **adaptive-top-k** uses `0a05a74` as `fix_rev`: `f8e4ddc`'s own tests patch helpers that
  only exist one commit later, where `0a05a74` repaired them.
- **summary-context-overflow** carries the one dead end that git history records: `ed0c1b5`
  lowered the tiktoken cap 60000 → 55000 and the overflow came back (`cf460a4` explains why).
  **no-results-disclosure**'s dead end comes from its commit message, which rejects the additive
  `error` event in favour of the content-replacing `correction` event. The plan asks for three
  dead-end tasks; history supports two, and no third was invented.
- **second-repo is not in the set.** It has 17 commits and no tests; the only small fix with a
  checkable result (`ef3dd06`, TypeScript errors in the bridge) still leaves type errors in the
  same two files at the fix commit, so no verify command can tell its fix from its base.
- Models: every task uses `claude-sonnet-5-5`; change `model` in the task files to compare
  another.

## Metrics and the real event stream

Built from a real capture (Claude Code 2.1.308, `tests/fixtures/real_edit.jsonl`, paths and ids
removed):

- `{"type":"system","subtype":"init","cwd":…}` first; tool paths are made relative to `cwd`.
- One `{"type":"assistant","message":{"id":…,"content":[one block],"usage":{…}}}` event **per
  content block**, each repeating the message's id and usage. Messages are deduplicated by id.
  Their `output_tokens` lag the session's (91 summed over messages vs 469 in the result of the
  capture), so `tokens_total` comes from the `result` event's usage, and `tokens_to_first_edit`
  (summed over messages up to the first edit of a relevant file) undercounts output tokens.
- `stream_event`, `rate_limit_event`, `user` tool results and other `system` events are skipped.
- `{"type":"result","total_cost_usd":…,"num_turns":…,"usage":{…}}` last; without it (killed on
  timeout) the session is `truncated` and charged the per-session cap.
- Tokens exclude cache reads in every arm. Dead ends count `Edit.new_string`, `Write.content` and
  `MultiEdit.edits[].new_string` matching the pattern on a file matching the glob, once per call;
  `old_string` never counts, and edits made through Bash are not seen.

## Public task sets

| set | repo | tasks | build the repo |
|---|---|---|---|
| `bench/tasks-tallyho` | tallyho, a 25-file Python library written for the benchmark | 4 bugs, one file each, hidden pytest files | `bench/tallyho/make-repo.sh` |
| `bench/tasks-scholion` | this repository at `0a08e6e`, about 20k lines of Rust | 4 injected bugs, hidden integration tests | `bench/tasks-scholion/make-repo.sh` |
| `bench/tasks-relay` | tallyho | E4 knowledge relay: 10 tasks, each needing one lesson an earlier agent left (arms `baseline`, `notes`, `av`); see its README | `bench/tallyho/make-repo.sh` |

Each script writes the repo where the task files expect it (`~/bench-repos/…`). The tallyho
tasks need a Python with pytest at `~/bench-work/venv`. The scholion tasks build Rust in every
session: export `CARGO_TARGET_DIR` (one shared target) and `AV_TEST_DATABASE_URL`, and plan on
about 1 GB of disk per session. The tallyho and scholion tasks use `claude-haiku-5-5`; the relay
tasks use DeepSeek (`bench/tasks-relay/README.md`).

```bash
bench/tallyho/make-repo.sh && python3 -m venv ~/bench-work/venv && ~/bench-work/venv/bin/pip install pytest
scholion-bench check bench/tasks-tallyho
scholion-bench run bench/tasks-tallyho --reps 10 --arms baseline,av --budget-usd 5 \
  --out bench/results/$(date +%F)-tallyho $COMMON --confirm
```

### Haiku runs (2026-10-27)

`bench/results/2026-10-27-haiku/README.md` has the numbers, the analysis and the fixes they led
to. In short: on tallyho, av used to cost more than plain git (+21% tokens). Agents never called
the MCP tools, and the extra cost came from the pack. It repeated the goal and pasted bodies
Claude Code reads again before every edit, and a cold start pulled in nothing the target uses.
After the fixes, av matches baseline on cost with 13% fewer turns; it still uses about 6% more
tokens.

