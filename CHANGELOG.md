# Changelog

## Unreleased

- **Lean disk** (Plan 11). Agents' files no longer pile up:
  - **Finished tasks give their disk back.** A task merged or abandoned for `AV_WS_RETAIN_SECS`
    (default 3600; `0` at once, `never` to keep) loses its workspace and scratch dir; a sweep looks
    every 60 s. The task's commits stay in the store. An undo or revert that puts a task back to
    work restores its workspace at its head. Open, handed-off, in-review and conflicted tasks are
    never touched, and nothing outside `ws_root` is deleted. No migration.
  - **`av disk`** (`GET /v1/disk`): per repo, every workspace with its task's status, size, age and
    whether a sweep would remove it, plus scratch, build cache and store, and what sessions left
    outside any workspace. **`av gc [--dry-run] [--retain SECS] [--build-cache]`** (`POST /v1/gc`)
    reclaims on demand. Both take `--json`.
  - **One cargo build cache per repo.** For a repo with a root `Cargo.toml` the daemon writes
    `<ws_root>/<repo>/.cargo/config.toml` (at import and start-up), so every task workspace builds
    into `<ws_root>/<repo>/.cache/cargo-target` instead of its own `target/`. The repo's own cargo
    config and `CARGO_TARGET_DIR` still win; `AV_BUILD_CACHE=off` writes nothing.
  - **The hooks watch what a session leaves behind.** `SessionStart` gives a task a scratch dir,
    exported as `$AV_SCRATCH` through `$CLAUDE_ENV_FILE`, and takes a snapshot of the build,
    dependency and cache dirs under `$HOME` and the temp dir. `Stop` compares, blocks once when the
    session added `AV_FOOTPRINT_WARN_MB` (default 512) or more outside any workspace, and reports
    it to the daemon (`POST /v1/footprint`), which shows it in `av disk`. `AV_FOOTPRINT=off`,
    `AV_FOOTPRINT_ROOTS`, `AV_FOOTPRINT_SECS`.
  - `Vcs::remove_workspace` and `Vcs::restore_workspace` (jj: forget the workspace, re-create it
    at a commit).
  - `av up` moves a `daemon.log` over 50 MB to `daemon.log.1`.
  - **`scholion-bench run` removes each session's copy after verify**: the git arms' clone, and
    the av arms' import copy (their bench task is abandoned, so the bench daemon reclaims the
    workspace). Invalid sessions keep everything; `--keep-work` keeps every copy.
- **A redesigned dashboard** (`/ui`): a neutral, professional look in place of the pixel-slime
  theme. It adds a row of summary figures (in progress, in review, conflicted, merged, backlog,
  agents busy), agent cards with initials avatars, a state badge, skill tags and a load bar against
  `max_open`, a kanban board whose columns never overflow, side panels for targets, backlog and
  activity, and a slide-in drawer for tasks and agents. Light and dark follow the system, or pick
  one with the theme button (kept in this browser). The page reads the same `/v1` routes as before.
- **A `checkpoint` event**: a snapshot that moves a task's head (`POST /v1/tasks/{id}/checkpoint`,
  a submit, a sync) now lands in the event feed as kind `checkpoint` with the new head, the old one
  (`payload.from`) and how many paths the task changes (`payload.touched`), so `GET /v1/events`,
  `scholion events --follow` and in-process subscribers see work as it happens instead of polling
  `head_change`. A snapshot that moves nothing is no event.
- **Targets** (Plan 10 A3): `av target add --repo R --title T [--due YYYY-MM-DD]` names something
  the team works toward; `av post --target SLUG` (or `av backlog target ITEM SLUG`) counts a
  backlog item toward it, and `av target list` shows how many of its items merged, are being
  worked on or wait. `av target done|drop|reopen`. The dashboard shows each active target's
  progress. Migration `0027_targets`. Targets do not change routing.
- **Office and timelines on the dashboard** (Plan 10 A3): each Teams desk now says what the agent is
  on, what it did last, and whether it is working, reviewing, idle or quiet (open tasks with no write
  for a whole lease). Clicking a desk, or a task's owner, opens the agent's timeline: its records,
  starts, claims, review verdicts, submits and merges. New read-only routes `GET /v1/office` and
  `GET /v1/agents/{name}/timeline`, built from data Scholion already keeps (no migration).
- **`scholion-agentrun`**: `AgentCommand::env_clear()` starts the agent from an empty environment
  (later `env` calls still apply; `path_prepend` adds nothing after the prepended dirs when there
  is no `PATH`), and `stop_flag(flag)` kills the run's process group as soon as another thread
  raises the flag (`RunResult::stopped`; `process::run_until`). A runner uses them to keep host
  credentials away from an agent and to stop it at a budget.

## 0.0.2 (2026-11-01)

Several coding agents (Claude Code, Codex, Cursor, Grok Build, Gemini CLI) now share one backlog:
each takes the work that fits it, reviews the others' work, and merges what it approves.

### Install and run

- **The release now includes `scholion-daemon`, with its own Postgres (lite mode).** `av init`
  in a git repository starts it and adds the repo; no Docker needed. 0.0.1 shipped only the CLI,
  so the installer in the README failed on it.
- `av init --for codex|grok|gemini|cursor` wires those CLIs to the MCP server. Grok, Gemini and
  Cursor also get an `X-Av-Agent` header, so the daemon knows which agent is calling.
  `av --agent NAME init --for mcp-json --path FILE` does the same for any client with an
  `mcpServers` JSON file.
- **Ask Claude to set it up**: this repository is a Claude Code plugin marketplace. Run
  `claude plugin marketplace add vinhqbui/scholion`, then `claude plugin install
  scholion@scholion`. It adds three skills: `/scholion:setup` (install, `av init`, checks),
  `/scholion:team` (several agents on one backlog: profiles, first items, `av watch`) and
  `/scholion:doctor` (finds what is broken).
- **A short README**: what Scholion is for, told as a story, how to try it and what it measured.
  The full reference moved, unchanged, to `docs/reference.md`.
- `av repo backup NAME --set URL --branch BR` pushes main to a named branch of a backup remote.

### Route work to the agent that fits (Plan 8)

- **Agent profiles**: `av agent set NAME --best TAGS --can TAGS --cost N`. 1 is the cheapest.
- **A backlog**: `av post --repo R --title T --goal G [--needs TAGS] [--prefer NAME]`, then
  `av backlog`. Tags are set by people; the routing rules are deterministic, with no LLM call.
- **`av next`, or `start_task` with no title and goal**, gives an agent the next task that fits
  it. A task is reserved for its best agent for 10 minutes, then any agent that can do it may
  take it.
- `av handoff --needs TAGS` hands a task to an agent with those skills.
- **Order the backlog**: `av post --after ITEM[,ITEM]` (an item id or the slug of its title).
  `av next` does not offer the item, and `start_task` without a title does not take it, until
  every item it lists has merged; `av backlog` shows it as `waiting on X`.

### Close the review loop (Plan 9)

- `next` offers, in order: reviews of other agents' work, handoffs, abandoned tasks, then new
  work. An owner never reviews its own task. The reviewer merges what it approves.
- `av next --wait N` (MCP: `wait`) waits up to 50 s for work instead of returning empty.
- `av watch -- CMD` starts an agent CLI only when there is work for it. It runs on your machine;
  the daemon never starts processes.
- **A team's limit**: `av agent set NAME ... --max-open N`. At the limit, an agent is offered only
  reviews and its own unfinished tasks until it submits or hands one off.
- **Take only some kinds of work**: `av next --take review,merge` (or `--take work`). A
  review-only agent is never given backlog work, and a worker is never offered (or reserved)
  other agents' reviews.
- **A conflicted task goes back to its owner**: an approved task whose merge conflicted is
  offered to its owner with what to do (sync onto main, resolve, re-check, submit again), so
  `av watch` picks it up. If the owner does not act within the lease, any agent that fits may
  claim it.
- **Keep agents busy**: after its record check, the Claude Code `Stop` hook asks for this agent's
  next work. If something fits, the agent carries on instead of stopping. It waits for work up to
  `AV_HOOK_WAIT_SECS` (default 600, `0` turns it off), and continues a session at most
  `AV_HOOK_MAX_CONTINUES` times (default 20). Grok Build gets the same hook.
- MCP tools take a task's slug (or `repo/slug`) as well as its id. The CLI still takes the id.
- One MCP session can hold several tasks: Claude Code's subagents share their parent's
  session. Without `task_id`, a call uses the session's only open task, or asks which one.
  Before, a subagent's `start_task` sent the parent's later calls to the wrong task without an
  error.

### Packs, index and scale

- Work packs are 800 tokens of signatures by default. Lexical (BM25) and semantic hits are
  combined by rank fusion.
- Overlay index (Plan 7): a task reads main's index plus its own changes and no longer copies the
  index at start. A start that waits for its turn creates nothing until it gets one.
- Hooks show other tasks' records when an agent opens a file.

### For code built on the daemon

- Extension points in `scholion_daemon`: `router_with`, `with_extension`, `subscribe_events`,
  `MergeGuard` (asked before submit, approve and merge), `with_mcp_tools`, `UsageSink`
  (`POST /v1/agents/{name}/usage`), `put_profile` and `AppState::migrate_extension`. An extension
  runs its own migrations in its own Postgres schema, so its migration numbers never clash with
  the daemon's. `run_with` is the daemon's whole start-up as a library call, so an extension
  binary adds its routes and services without copying `main.rs`.
- `scholion-agentrun`: a library for running an agent CLI (argv, environment, deadline that kills
  the process group, parsed usage). The benchmark uses it.

### Upgrading from 0.0.1

- Stop the daemon and install the new binaries (rerun the installer, or `git pull` and
  `docker compose -f deploy/docker-compose.yml up -d --build`). The daemon adds its new tables
  (migrations 0021-0026) at start-up; existing tasks and records stay as they were.
- Restart your agent CLIs so they load the new tool parameters.

### Evidence

- A real run: two Claude Code agents on DeepSeek Flash took tasks from the backlog, reviewed each
  other's work and merged it. Reproduce it with `examples/routing-smoke/deepseek-demo.sh`; that
  run cost about $0.03. It shows that the loop works. It is not a measurement of token savings.
- Benchmarks and their limits: `bench/results/`. Plain notes files delivered recorded lessons as
  well as av did at 10 notes (E4). The task set with 300 notes (`bench/tasks-relay-scale`) is
  ready but has not been run, so av's advantage at scale is not measured yet.

## 0.0.1 (2026-10-28)

First release: the `scholion` CLI (alias `av`). The daemon ran from Docker
(`deploy/docker-compose.yml`).
