# Changelog

## Unreleased

- **Records on the pull request: `av notes`** (Plan 13 Phase 2, the PR bot). Renders a task's
  decisions, dead ends, notes, review findings, handoffs and checks, plus what other tasks recorded
  about the files it changes, as one Markdown block between `<!-- scholion:notes v1 task=<id> -->`
  markers; reads `/v1` only. `--pr [N]` posts it through `gh`, or edits the comment that already
  holds the task's block (other text and other tasks' blocks stay; an unchanged block is not
  sent). The task is an id, a slug or `repo/slug`; `--format json`. Record text cannot close the
  markers and its `@names` notify no one. The CLI posts because the daemon runs on the
  developer's machine, out of a GitHub runner's reach; `integrations/github/` has the format and a
  sample workflow for a self-hosted runner that can reach it. `integrations/README.md` lists every
  integration.
- **Cold start from git history** (Plan 13 Phase 2). `av repo add NAME PATH --mine-history N`,
  `av init --mine-history N` and `av repo mine NAME [--limit N]` (`POST /v1/repos/{name}/history`)
  turn at most N commits of the repo's history (every ancestor of main, newest first) into
  records: one `note` by the agent `git` per commit with one parent, a subject that is not
  housekeeping, a message that says something (trailers and links dropped) and at most 25 files,
  anchored to the files it touched, with the commit as its `change_id`. They live on one merged
  task per repo, `<repo>/git-history`; no migration. Mining again skips what was mined. Packs rank
  them at half weight (`MINED_FACTOR`, packer version 5), `records/at` lists them after agents'
  records, every rendering names the commit (`note (git 1a2b3c4)`), and `av why` on imported
  history prints the blamed commit's mined record. The commit rules (`housekeeping`, the message
  cleaning) moved from `scholion-bench mine` to `scholion_core::history`, which both use; `Vcs`
  gains `history` (all ancestors, default `trunk_log`).
- **Bench: E4 at scale, `run --noise N`** (Plan 13 Phase 1, harness only; nothing run yet). Adds N
  deterministic irrelevant notes to every relay task, the same ones in the `notes` arm's
  `NOTES.md` and as the `av` arm's anchored records: a seeded generator over the base tree's own
  files and symbols, anchored everywhere (the lessons' files too), spread among the lessons, each
  level the first N of the next. `scholion-bench noise` prints them. Rows gain `input_tokens`
  (cache reads included) and `memory_tokens` (`NOTES.md` or the start pack); a noise run's report
  names its level and adds a "Memory at scale" table; a results directory holds one level. Design,
  levels, metrics and a per-cell cost estimate: `bench/results/2026-11-e4-scale/DESIGN.md`.
- **Remote workspaces by checkpoint-by-patch** (Plan 14). An agent on another host works in a git
  checkout of its task (`av remote clone TASK DIR`, `av task start --checkout DIR`) and each
  checkpoint goes to the daemon as a patch (`POST /v1/tasks/{id}/checkpoint?base=`), which the
  daemon applies in process to the workspace it still owns and snapshots as a local checkpoint;
  a stale base is `409` with the head and `av checkpoint` pulls (`GET /v1/tasks/{id}/patch?from=`,
  `av remote pull`) and resends. Hooks, `submit`, `record`, `handoff`, `validate` and `sync` in
  a checkout send its edits first. The daemon runs no git for it. Reference: "An agent on another
  host"; plan and measurement: `docs/superpowers/plans/2026-11-04-scholion-plan-14-remote-workspaces.md`.
- **Name fan-out cap** (`AV_INDEX_MAX_FANOUT=N`, off by default). The code graph links a call by its
  bare name to every symbol of that name, so `new` alone gave tokio 500k of its 896k edges. With a
  cap, a name more than N other symbols define links to none of them; overlay views apply it to
  the names as the task sees them, and the overlay equivalence proptest runs under a cap of 1 too.
  E1 on the public task set: pack scores unchanged at caps 64 and 16 but for one task at one
  budget, edges −40% / −54%; on tokio a cap of 64 drops 71% of the edges
  (`bench/results/2026-11-03-e1-fanout-cap/`). It applies to repos imported after it is set.
- **Lean builds, one target dir per task** (Plan 12). Build output was 99.99% of the disk the
  first dogfood run took: about 20 GB per Rust task (14 GB of test binaries that were 73% debug
  info, 4 GB of incremental cache) against 3 MB of Scholion's own state.
  - The per-repo cargo config the daemon writes now sets `incremental = false`,
    `debug = "line-tables-only"` (no debug info at all for dependencies) and **no `target-dir`**:
    every task workspace builds into its own `target/`, removed with the workspace. Measured on
    this repo, a full `cargo test` build is 4.3 GB instead of 21 GB.
  - **Fixes Plan 11's shared build cache**, which pointed every workspace of a repo at one target
    dir: cargo names a path package's artifacts by its path relative to the workspace root, so
    `cargo test` in one workspace ran another task's test binary (a false green or red
    validation; hit twice in the run). An av-written config from before is rewritten at start-up.
    A test runs `cargo metadata` in two workspaces of one repo and checks their target dirs differ.
  - **Build output goes with the merge.** A merge or abandon removes the workspace's `target/` at
    once (the answer says `build: removed|none|kept`; the feed gets `build:reclaimed` with the
    bytes) while the workspace itself waits for `AV_WS_RETAIN_SECS` as before, for an undo or a
    reviewer's look; `never` keeps both. A finished task no longer holds gigabytes for an hour.
  - `GET /v1/disk` and `av disk` show each workspace's build output apart (`build_bytes`, a `BUILD`
    column, a total); `av gc` says how much of a reclaimed workspace was build output. The
    dashboard's Disk card shows the same.
  - The snapshot never takes a workspace's top-level `target/`, even in a repo whose `.gitignore`
    forgot it (`/.av-task` was already ignored this way).
  - **One integration-test binary per crate** (this repo, not the product). Its `tests/*.rs` files
    are now modules of one test binary per crate, `it` (`tests/main.rs`; `cargo test -p
    scholion-daemon --test it api::NAME`), instead of 45 binaries that each linked every
    dependency: a full `cargo test` build of this workspace is 2.4 GB instead of 4.3 GB.
  - **Dependencies are built once per repo** (Task 5). When a task's build output is reclaimed,
    the units built from registry and git crates go into `<repo>/.cache/deps-target/`, and a new
    task's `target/` starts with them: compiled libraries hard-linked (one copy on disk for every
    task; rustc replaces them, never writes into them), dep-info, fingerprints and build-script
    output copied (those are rewritten in place). Measured on this repo: a seeded full `cargo
    test` build adds 1.2 GB instead of 2.4 GB and takes 15 s instead of 38 s, no dependency
    recompiled. The repo's own crates never enter the cache. `av disk` counts a linked file
    once; both steps run in the background (`build:reclaimed` with `cached_bytes`, `build:seeded`); `av gc
    --build-cache` empties the cache; `AV_BUILD_CACHE=off` keeps and seeds nothing.
  - **`av convention add` warns when a required check is scoped to every path** (`**`): such a
    check runs for every task, so a docs-only change pays the full build (the run's docs-only task
    paid 20 GB and 10 min). The hint says to scope it (`--scope 'crates/**'`); a change outside
    the scope then skips it, as `av validate` already does.
  - **`{crates}` in a check runs only the crates a change can affect** (Task 2). `av validate`
    replaces it with `-p a -p b`: the workspace members holding a changed path plus every member
    that depends on one of them, by any kind of path dependency (dev and build too), from
    `cargo metadata --no-deps` in the workspace. A changed path outside every member (root
    `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.cargo/`, `bench/`) or a failed
    `cargo metadata` gives `--workspace`. The line prints the command that ran, and the check's
    result keeps it (`command`); `GET /v1/tasks/{id}/checks` adds `changed`, the task's changed
    paths. Approval counts a result for a `{crates}` check only with an expanded `command`: run
    as written, `cargo test {crates}` filters every test out and passes, so an older `av` (or an
    MCP agent that leaves `command` out) cannot pass it.
  - **A task's builds stay in its own `target/`** (Task 3). A `CARGO_TARGET_DIR` in the
    user's environment (or `build.target-dir` in `~/.cargo/config.toml`) put every task workspace
    of a repo into one target dir again, where `cargo test` runs another task's test binary. In a
    cargo workspace, `av validate` (the command and every check) and `av-wrap.sh` set
    `CARGO_TARGET_DIR` to the workspace's `target/` and unset `CARGO_BUILD_TARGET_DIR`; when the
    user's environment names a target dir, the session-start hook adds shell code to
    `$CLAUDE_ENV_FILE` that does the same for every Bash command, for the task workspace it runs in
    (a session the Stop hook moves to another task follows it). That is the dir the dependency cache seeds and the merge
    reclaims. An explicit `--target-dir` still wins.
- **`av agent rm NAME`** (`DELETE /v1/agents/{name}`). Removes a profile. An unknown name is
  404. An agent that still owns an open, handed-off, in-review or conflicted task is 409 unless `--force`
  (`?force=true`); the tasks stay. A profile an extension manages is refused, the same way
  `av agent set` is.
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
  - **A cargo config per Rust repo.** For a repo with a root `Cargo.toml` the daemon writes
    `<ws_root>/<repo>/.cargo/config.toml` (at import and start-up), which every task workspace
    inherits (what it holds: see Plan 12 above; the shared target dir it first set was withdrawn
    before release). The repo's own cargo config and `CARGO_*` still win; `AV_BUILD_CACHE=off`
    writes nothing.
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
- **Main, the daemon and the disk on the dashboard.** Under the board, a Main panel lists main's
  last commits with their post-merge check (passed, failed with the exit code, or unchecked) and
  the task that landed each; its header says whether semantic search is on and how many rows wait
  to be embedded, the backup's state, how many conventions the repo has, and an import still
  running. A Disk panel (`GET /v1/disk`, read at start, on its button and every five minutes)
  shows what workspaces, build caches, the store and scratch dirs take, the repo's workspaces
  largest first with the ones a sweep would reclaim, and what sessions left outside any
  workspace; a session of this repo's tasks (or of no task) that left 512 MiB or more (the hooks'
  default) in the last day gets a "Needs attention" row. The page never deletes. When the page cannot load (the daemon
  is away at start, or the history cannot be read) it asks `/healthz` every five seconds and
  starts over once it answers; a feed that drops mid-session keeps reconnecting as before.
- **What needs attention, and how the work flows.** Every task card says how long it has been in
  its status ("in review 3h", "approved 20m" once its review is given), and a card goes amber once
  it is stuck (a submit with no verdict for 2 h, an approval not merged for an hour, a handoff
  nobody took for an hour, a conflict unresolved for an hour, an open task with no write for a
  whole lease). A "Needs attention" strip above the board lists, worst first, a red main,
  conflicted tasks, those stuck tasks, backlog items no agent can do and overdue targets (from
  the health, backlog and targets the page already reads); it is hidden when there is nothing
  and shows at most eight rows. A Flow panel shows, over 7, 14 or 30 local days, merges per day
  as a sparkline, the median cycle time (start to merge), the median review wait (submit to
  verdict, carried approvals left out), rework (changes requested, conflicts, undos, reverts),
  and per agent what it started, merged and reviewed, from each agent's timeline (read at most
  once a minute). No cost or model usage is shown.
- **The task drawer shows the work itself.** The diff from base to head, one file at a time with
  its added and removed lines; the task's own timeline (started, every checkpoint with how many
  paths it touched, claims, submits, verdicts, merges, conflicts); each review round (the verdict
  with the reviewer's findings at that head); a validation record as passed or failed, with its
  command and output; and, after "Build context pack", the pack's receipt: every item with its
  tier, why it is there, where it comes from and its tokens. The page reads the repo's event
  history (`GET /v1/events` in pages of 500, the newest 10 000 ids of a long one) and derives
  these from it; the daemon records nothing new.
- **The teams stand left of the board.** From 1580 px wide the dashboard is three columns: the
  agents' desks on the left, tasks by status in the middle, targets, backlog and activity on the
  right. Narrower, the desks are a row above the board, as before; below 1180 px everything stacks.
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
