# Scholion reference

The full reference: install options, commands, configuration, design, architecture and
measurements. For the short version, see the [README](../README.md).

*σχόλιον (scholion): a note written in the margin of a manuscript. Scholion keeps notes in the
margins of your code.*

Context-first version control for coding agents: tasks in isolated jj workspaces, decisions and
dead ends recorded next to the code, and a token-budgeted context pack for any agent that picks the
work up.

**[Watch a recorded run](https://scholion.openevaluations.si/demo)**: Claude records a decision and a dead
end, hands off; Codex picks the task up from a context pack instead of re-reading the repo; Gemini
reviews. On a small Python repo, Claude Code needed **12.9 % fewer turns per task at the same
cost** ([benchmark](../bench/results/2026-10-27-haiku/), including where it did not help).

[![A recorded run: Codex's context pack leads with Claude's decision and dead end](img/demo-replay.png)](https://scholion.openevaluations.si/demo)

**New in 0.0.3:** agents can propose backlog items for a person to accept, `av attention` lists
what needs a person or an agent, `av repo advance` takes main back from the remote, `av notes`
puts a task's records on its pull request, `--mine-history N` turns a repo's git history into
records on day one, and the dashboard is redesigned. **Since 0.0.2: one backlog for all your
agents.** Post work with tags; each agent (Claude Code, Codex, Cursor, Grok, Gemini) takes what
fits it, another agent reviews it, and the reviewer merges what it approves.
`examples/routing-smoke/deepseek-demo.sh` runs that loop with two Claude Code agents on DeepSeek
Flash for about $0.03; it shows the loop working, not token savings. What changed:
[CHANGELOG.md](../CHANGELOG.md).

**Ask Claude to set it up**: `claude plugin marketplace add vinhqbui/scholion`, then `claude plugin
install scholion@scholion`, then say *"set up Scholion for this repo"* (`/scholion:setup`; `/scholion:team`
for several agents, `/scholion:doctor` when something is broken). The skills are in
`integrations/claude-code/plugin/`.

**Try it by hand** (no Docker: the daemon runs its own Postgres; Linux and macOS):

```bash
curl -fsSL https://raw.githubusercontent.com/vinhqbui/scholion/main/scripts/install.sh | sh
cd your-repo && av init                          # daemon up, this repo added, Claude Code wired
av init --for codex --for gemini --for grok --for cursor   # the same MCP server for other agents
```

Then watch your agents at `http://localhost:7420/ui`. The installer puts `scholion`, `av` and
`scholion-daemon` into `~/.local/bin` from the
[releases page](https://github.com/vinhqbui/scholion/releases). The full stack with semantic search
(Docker; the first build compiles the daemon, so give it a few minutes):

```bash
export UID GID=$(id -g) AV_IMPORT_ROOT=~/projects                # repos to import live here
docker compose -f deploy/docker-compose.yml up -d --build      # daemon on 127.0.0.1:7420
claude mcp add --transport http av http://localhost:7420/mcp   # or any MCP client
av repo add myrepo /import/myrepo                              # path inside the container
```

## Editions

Community is this repository. Commercial is not available yet, and this section names no price.

**Community** (free, [MIT](../LICENSE)). Connect any number of agents: Claude Code, Codex, Grok,
Gemini, Cursor, or any MCP client.

```bash
av init --for claude --for codex --for grok --for gemini --for cursor
av --agent muse init --for mcp-json --path PATH/TO/settings.json
```

They share one memory (records next to the code, and the context pack), a review loop (another
agent reviews, and the reviewer merges), and routing by strength (`av agent set NAME --best …`,
then cost). There is no seat cap. `--max-open` on a profile is a limit you set, not a license.
The daemon does not launch agents and holds no keys for the agents' own models. Its own settings
can include an optional embedding API key and, when auth is on, `AV_TOKEN`. Details:
[Many agents at once](#many-agents-at-once).

**Commercial** (Scholion Cloud, hosted; its private repository was called `agentvcs`) is not available yet. The plan, and only the plan, is a service that
runs agents for you: personas, a model connection that is managed or your own key and endpoint,
budgets in subscription credits, and approval gates. There is nothing to buy.

## How this was built

Scholion is **vibe-coded**: AI coding agents (mostly Claude Code, with some help from other
agents) wrote almost all of the code, tests and docs, with one human steering, choosing the
direction and reviewing. Treat it as an experiment, not as audited infrastructure. The plans the
agents worked from are in `docs/superpowers/plans/`, and the design spec is in
`docs/superpowers/specs/2026-10-18-agentvcs-design.md` (the project was called `agentvcs` then).

Part of the design (Plan 5) is inspired by Anthropic's article
[Redesigning source control](https://www.anthropic.com/research/redesigning-source-control),
which describes Scion and its review tool Antique. **Scholion is an independent project. It is
not affiliated with, endorsed by or derived from code by Anthropic**; it only borrows ideas from
the public article. See [Ideas borrowed from Scion](#ideas-borrowed-from-scion).

The CLI installs as `scholion` with the short alias `av` (the same program). Hooks, agent prompts
and the text inside context packs say `av`, since agents type it often; environment variables keep
the `AV_` prefix.

## Design

### The problem

Several coding agents working on one repository with git hit two walls. Each new session spends
tokens re-orienting: reading the tree, rediscovering what was tried, decided and rejected. And a
handoff or pull request shows *what* changed, never *why*. Git keeps snapshots and throws away
intent, decisions and dead ends, so every agent re-derives them. Scholion stores the "why" next to
the code and hands any agent a deterministic, token-budgeted context pack built from it.

### Concepts

- **Task**: one unit of agent work, replacing branch + pull request. Each task gets its own jj
  workspace on disk (`~/ws/<repo>/<task>/`), branched from trunk. The agent edits files there with
  its usual tools; `scholion checkpoint` snapshots them.
- **Record**: a short, typed note attached to a task: `intent` (the goal), `decision`, `attempt`
  (a dead end and why), `note`, `handoff`, `review`. Summaries are ≤ 200 characters, and records
  are append-only: a newer record *supersedes* an older one instead of editing it. That is also
  how an agent corrects a record that is wrong (`av record --supersedes rec:…`, see
  [Correcting a record](#correcting-a-record)).
- **Anchor**: a record's link to code, as a path or `path:Symbol`. Records resurface whenever a
  later task touches that code. The anchored code's hash is stored when the record is written, so
  a pack marks a claim `(changed since recorded)` once that code has moved on.
- **A record says where it comes from.** Another task's record is printed with what became of
  that task, unless it merged: `- rec:… decision, reverted: …`, `…, abandoned: …`, `…, open: …`
  (in packs, in the pre-read hook's answer and in `av expand`). A claim of work whose merge was
  reverted ranks at 0.4 of its score and one of abandoned work at 0.6; an `attempt` keeps its
  rank, since a dead end is still one when the work around it failed. Merged and open work rank
  as before, and nothing ranks above merged work: a merge shows that code passed review, not that
  a note about it is true. A record written after the revert belongs to the work that follows
  and is `open`. `GET /v1/tasks/{id}/records` and search answer the same as
  `standing: {origin, factor}` on each record.
- **Convention**: a repo rule scoped by path glob (`src/api/**`), shown only to tasks touching
  matching files. A `required` convention is always included, whatever the budget.
- **Context pack**: Markdown with the sections *Task, Map, Decisions & dead ends, Code,
  Conventions, Live work*, built for one task within a token budget. No LLM is involved: seeds
  (the goal, changed files and symbols, an optional focus) pull in candidates (reference-graph
  neighbours, records anchored up to two hops away, semantic hits, other tasks touching the same files), which are
  scored and fitted into per-section shares (map 10 % · task 10 % · history 20 % · code 50 % ·
  conventions + live work 10 %, with unused share spilling over). Each item is shown at the
  richest tier that fits (handle → summary → full); `scholion expand <id>` pulls the rest.
- **Session ledger**: remembers what each agent session was already sent, so later packs refer
  back instead of resending. A session with no id gets no deduplication.
- **Receipt**: every pack is stored as a manifest (`rcp:<8 hex>`): what was sent, at what tier and
  why. A record can cite the receipt its author worked from.

### Task lifecycle

| From | Event | To |
|---|---|---|
| `open` | submit | `in_review` |
| `open` | handoff | `handed_off` |
| `handed_off` | claim | `open` |
| `in_review` | request changes | `open` |
| `in_review` | approve | `in_review` (verdict recorded; merge needs it) |
| `in_review` | merge | `merged`, or `conflicted` on a conflict |
| `conflicted` | resolve | `open` |
| `merged` | undo | `in_review` |
| `open`, `handed_off`, `in_review`, `conflicted` | abandon | `abandoned` (final) |

This table is `scholion-core/src/lifecycle.rs`; any other transition is refused with HTTP 422.
`submit` is refused unless the task has changes, and both `submit` and `handoff` unless it has at
least one decision, attempt, note or handoff record. The refusal lists changed symbols that no
record covers. An approve needs a passing validation at the head and is refused when the head is
already approved in its current review round, since the latest submit (the answer names who
approved it and when); after a resubmit the head is reviewed again.
A review finding the reviewer already stored at the same head is not stored twice.

**Claims.** A handoff stores what is done, remaining, broken and still open as a `handoff` record
and returns a *handoff bundle*: goal, state, records (decisions and dead ends first), changes
(hunks while the budget allows) and open questions, within a token budget. `claim` makes another
agent the owner. It also works on an open task whose owner stopped writing for `AV_LEASE_SECS`.
Every claim increments the task's *generation* and starts a new session. A write sent under an
older generation (`X-Av-Generation`, which the CLI takes from `.av-task`) is refused with "claim
lost". Writes under the current generation renew the lease.

### Invariants

- Postgres is the source of truth. Qdrant is derived and can always be rebuilt (`scholion reindex`).
- `main` advances only through `merge`; agents never write trunk directly.
- Packs are deterministic: the same state gives the same pack, independent of the database's
  locale (every text ordering in SQL uses `COLLATE "C"`).
- If semantic search is down, packs still return, built from structure and Postgres full-text,
  and carry the note `degraded: semantic`.

### Status

Built: tasks and workspaces, checkpoints, records with anchors and freshness, conventions, the
context packer with ledger, receipts and charged expand, the semantic layer (Ollama or TEI
embeddings, Qdrant, outbox worker), `submit` with its record gate, validation records, review
verdicts, merge with conflicts (a Conflicts pack section, crash-safe retry), sync and undo,
handoff and claim with leases and fencing, review bundles and pinned review findings, git backup
push, Claude Code hooks and a generic wrapper, the `AV_TOKEN` auth token, scope and session
cleanup, an MCP server with twenty-four agent tools, and a lifecycle event feed
(`GET /v1/events`). Plan 4 added the `scholion-bench` benchmark harness (`bench/README.md`); the
task set it was built for comes from a private repository and is not included. Plan 5 added the
Scion-inspired features below: a workspace-free read API, carried approvals, enforced conventions,
post-merge checks, revert of any merge, bisect and `why` blame. Plan 6 added the offline retrieval
benchmark (E1); Plan 7, an index that reads main plus each task's own
changes instead of copying it per task; Plan 8, agent profiles, a shared backlog and routing;
Plan 9, the review loop (`next` offers reviews and merges first, rescues handoffs and abandoned
tasks, waits for work) and `av watch`. Lite mode (`av up`, `av init`) runs the daemon with its own
Postgres, and release 0.0.2 ships it.

Measured so far (details in `bench/README.md` and `bench/retrieval/`): offline, on 42 tasks from
six public repositories (E1), the first pack names the files a fix touches best of all methods
(0.96–0.98), and since BM25 and rank fusion went in it also shows more of the changed symbols' 
signatures than BM25 over symbols at 4k and 8k tokens (0.73 vs 0.64 at 4k; level at 2k). With Claude Haiku on a
25-file Python project (E3, 160 sessions), av took about 12 % fewer turns and 14 % less wall time
at equal cost (the cost interval contains zero). Merges are fuzzed (E2): about 1,700 random
cases of concurrent edits, conflicts and reverts, each result checked against a line model and
`git merge-tree`, lost no work and reported no false conflict or false clean merge. Not yet
measured: larger repositories, and whether records carry knowledge to the next agent at scale. In a relay test (E4, 90 sessions on
DeepSeek flash), the earlier agent's lessons cut cost per solved task by about 42 % (30/30 solved
against 18/30 without them), and av delivered them at the same cost as a plain `NOTES.md`. Two
eight-agent stress runs (cheap models, a small Python web app) merged every feature through
Scholion with main green after every merge, which shows the workflow holds up, not that it is
cheaper.

## Ideas borrowed from Scion

What we took from Anthropic's description of Scion and Antique, and where it landed:

| Idea from the article | In Scholion |
|---|---|
| A thin client; the server owns history, merges and rebases | The daemon owns all state; the CLI and MCP tools are thin HTTP clients |
| An API complete enough that jobs need no checkout | `GET /v1/repos/{r}/files`, `/tree`, `/diff`, `/log` and the MCP tools `read_file`, `list_files`, `diff`, `log` (`scholion show`, `scholion diff`) |
| Derived data computed per change from its parent's value | `blame_v1`, cached by immutable commit id; `scholion why path:line` names the task, goal and decisions behind a line |
| No global merge queue: merge, test after, and rely on bisect and revert | `scholion runner` checks every new main, `scholion health` names the merge that broke it, `scholion bisect` finds it, `scholion revert` takes any merge out |
| Approvals with constraints, so a mechanical rebase does not need a fresh review | An approval carries over a rebase when the task's own patch is unchanged, or the only overlap is in files matching `carry_globs` (Markdown by default) |
| Push instead of polling | `GET /v1/events` long-poll and `scholion events --follow` |

The required conventions with check commands, and the refusal to submit leftover conflict markers,
came from our own stress runs rather than from the article.

## Architecture

```mermaid
flowchart LR
    subgraph host["Host"]
        agents["Agents (Claude Code, Codex, …)<br/>edit files in ~/ws/{repo}/{task}/"]
        cli["scholion / av CLI and MCP tools<br/>find their task through .av-task"]
    end
    subgraph docker["Docker: deploy/docker-compose.yml, all on 127.0.0.1"]
        subgraph daemon["scholion-daemon :7420 (axum, owns all state)"]
            api["api.rs / flow.rs<br/>routes, lifecycle ops"]
            vcs["scholion-vcs (jj-lib)"]
            store["scholion-store (sqlx)"]
            index["scholion-index (tree-sitter)"]
            pack["scholion-pack (context packer)"]
            outbox["outbox worker"]
        end
        avstore[("volume av-store")]
        pg[("postgres:16")]
        embedder["embedder (TEI / Ollama)"]
        qdrant[("qdrant")]
    end
    agents <-->|bind mount| vcs
    cli -->|HTTP, JSON| api
    vcs --> avstore
    store --> pg
    index --> pg
    outbox --> embedder
    outbox --> qdrant
```

The daemon can also run on the host (`cargo run -p scholion-daemon`) against the same containers.

### Crates

| Crate | Role |
|---|---|
| `scholion-core` | Shared types, `AvError`, the `Vcs` / `IntentStore` / `Semantic` traits every unit talks through, and the lifecycle state machine. |
| `scholion-vcs` | The only crate that touches jj-lib (pinned `=0.46.0`). Imports a git repo, creates a jj workspace per task, snapshots, diffs and reads files by commit id. |
| `scholion-store` | Postgres `IntentStore`: tasks, records, anchors, conventions, ledger, receipts, outbox. Migrations in `crates/scholion-store/migrations/`. |
| `scholion-index` | Tree-sitter extraction (Rust, TypeScript, Python): symbols (functions, methods, types, and module constants: `UPPER_CASE` in Python/TypeScript, `const`/`static` in Rust), reference graph, skeletons, code chunks, stored per *scope* (`main` or a task) and updated incrementally by content hash. |
| `scholion-semantic` | `Embedder` (TEI, Ollama, a scripted fake) and `VectorIndex` (Qdrant, an in-memory fake). |
| `scholion-pack` | The context packer: seeds → candidates → scoring → budget → tiers → ledger and receipt. Golden-file tests. |
| `scholion-daemon` | Wiring from environment variables, HTTP routes under `/v1`, lifecycle operations (`flow.rs`), indexing glue, the outbox worker. |
| `scholion-cli` | The `scholion` binary and its `av` alias: clap subcommands that mirror the API. |
| `scholion-testutil` | `TestDb`: a throwaway Postgres database per test. |

### Where state lives

| Store | Holds | Notes |
|---|---|---|
| jj repo (volume `av-store`) | File contents, changes, operation log | Each repo is a git import (`git/` bare clone, `main/` default workspace). `ChangeId` is the jj commit id. |
| Task workspaces (`AV_WS_ROOT`) | Each task's working copy with its own build output (`target/`), its scratch dir (`<repo>/.scratch/<slug>`) and a cache every workspace of a repo may share (`<repo>/.cache`) | Bind-mounted so agents edit on the host; clients are given host paths. Removed once the task has been merged or abandoned for `AV_WS_RETAIN_SECS`. |
| Postgres | Tasks, records, anchors, conventions, ledger, receipts, code index, outbox | Source of truth. |
| Qdrant | Vectors for records, conventions and code chunks | Derived; filled from the outbox. |

### Main paths

- **Start a task**: create the jj workspace at trunk, then copy `main`'s index rows into the task's
  scope inside Postgres. If `main`'s index is not recorded at that exact commit, the task is
  indexed from the files instead. Then write the intent record and return the first pack.
- **Checkpoint**: snapshot the workspace (a new jj commit), diff it against the task's base,
  re-index only the changed files, store the new head and the touched paths.
- **Remote checkpoint** (Plan 14): a checkout on another host sends `git diff` against the head it
  last synced to (`POST /v1/tasks/{id}/checkpoint?base=`). The daemon snapshots the workspace
  first (so edits on its own host count), refuses a base whose files are not the head's (`409`
  with the head), applies the patch to the workspace's files in process and then checkpoints as
  above. No git or other command runs on the daemon for it.
- **Record**: compute the anchored code's hashes at the current head, then write the record, its
  anchors and hashes, any cited receipt and an outbox row in one transaction.
- **Context**: gather candidates (Postgres, the index, jj file reads, and semantic search if it is
  configured), score, fit the budget, render, update the session ledger, store the receipt.
- **Outbox worker**: every second, claims up to 32 outbox rows (`FOR UPDATE SKIP LOCKED`, one
  worker per database via an advisory lock), embeds runs of upserts in one call, and writes them
  to Qdrant. A failed row is retried with backoff and never blocks the others.

### Concurrency

Several agents work against one daemon at once:

- **Per task**: checkpoint, record, submit, validate, sync, handoff, claim and merge take an in-process
  lock for that task, so a slower request can never store an older head or index older content
  after a newer one, and no write of a previous owner lands after a claim.
- **Fencing**: claims are one conditional update (of racing claimants exactly one wins) that
  increments the task's generation; writes carrying an older generation are refused. Agents are
  told apart by `--agent`/`AV_AGENT`, so agents sharing one name are not fenced from each other.
- **Per repo**: jj mutations (import, new workspace, snapshot) hold a per-repo mutex. Reads of
  files and diffs address immutable commits by id and take no lock, so packs and indexing never
  wait behind another agent's snapshot. All of them share one jj store per repo.
- **In Postgres**: status changes are conditional updates (of two racing `submit` calls exactly
  one wins); the expand budget is checked and charged in one statement; outbox rows are claimed
  with `SKIP LOCKED`.
- `crates/scholion-daemon/tests/swarm.rs` runs several agents through start → edit → record →
  checkpoint → context → submit at once. Its ignored `bench_swarm_on_a_300_file_repo` prints
  timings.

### Failure behaviour

- Qdrant or the embedder down: packs fall back to structure and full-text (`degraded: semantic`);
  the outbox keeps the work, and the worker catches up when the service returns.
- An index write and its outbox row commit together, so Qdrant can lag but never miss an entity.
- Backup remote down or rejecting the push: the merge still succeeds. The push runs in the
  background (60 s timeout, no credential prompts, never forced); a failure is stored and retried
  after 1 s, 2 s, 4 s … up to 5 min, and `scholion status` / `scholion repo backup` show it. Credentials are the
  daemon's own (for SSH, a read-only mounted key and `AV_BACKUP_SSH_COMMAND` in compose).
  By default main is pushed to a branch of the same name; `scholion repo backup NAME --remote URL
  --branch av-main` pushes it to `refs/heads/av-main` instead (still never forced), for example to
  open a pull request from it. Setting only `--remote` goes back to the trunk's name.
- Main coming back from the remote (Plan 15). When main is pushed to a branch such as `av-main`
  and pull requests are merged on the remote, `av repo advance NAME` (`POST
  /v1/repos/{name}/trunk/advance`, body `{remote?, branch?}`; defaults: the backup remote and
  `main`) fetches that branch into the repo's git dir (`refs/av/advance/<branch>`, never a local
  branch; 120 s timeout, no credential prompts) and, when it descends from main, fast-forwards
  main to it: main's index, the lean-build config and the backup follow, as after a merge, and the
  feed gets a `trunk:advanced` event (`{from, to, remote, branch}`, the remote without any
  credentials in its URL). Answers: `200` `{outcome: "fast_forwarded", from, to}`, `{outcome:
  "up_to_date", head}` or `{outcome: "ahead", trunk, remote}` (the remote is behind main: nothing
  to take); `409` `{outcome: "diverged", trunk, remote, error}` when neither contains the other
  (main is unchanged; a squash merge on the remote does this); `400` with no remote to use. `av
  repo backup NAME --remote URL --advance` (`advance: true` in the backup body; left out = off,
  like `--branch`) makes every backup push advance from the remote's `main` first; an advance
  that fails or finds the histories diverged is logged and the push goes on.
- Starting a task cleans up after itself: if the workspace cannot be created, the task is marked
  abandoned rather than left open.

### Testing and CI

`cargo test --workspace` runs everything against Postgres (see [Test](#test)). GitHub Actions
(`.github/workflows/ci.yml`) runs fmt, clippy with `-D warnings`, and all tests against Postgres
and Qdrant service containers on every pull request and every push to `main`. CI's Postgres
sorts text with an English locale on purpose, which catches orderings that depend on the
database's locale.

## Run

### Lite mode

`av up` starts `scholion-daemon` in the background on `127.0.0.1` (port from `AV_URL`, default
7420) and waits until it answers; `av down` stops it; `av init` calls `av up` when no daemon
answers. With no `AV_DATABASE_URL`, the daemon runs its own user-space Postgres 16: release
builds carry it inside the binary (feature `bundled-postgres`), source builds download it on first
start. Postgres refuses to run as root. Semantic search is off unless you also set `AV_QDRANT_URL`
and an embedder (below); packs then rank by structure and Postgres full-text.

| Variable | Default | Meaning |
|---|---|---|
| `AV_HOME` | `~/.scholion` | daemon data: `pg/` (Postgres), `store/` (jj repositories), `daemon.log` (moved to `daemon.log.1` by `av up` once over 50 MB), `daemon.pid` |
| `AV_WS_ROOT` | `~/ws` | where task workspaces appear (agents edit files here) |
| `AV_WS_RETAIN_SECS` | `3600` | how long a merged or abandoned task keeps its workspace; `0` removes it at once, `never` keeps it (see [Disk](#disk-what-agents-create-and-what-av-reclaims)) |
| `AV_DISK_MIN_FREE_GB` | `20` | free-space floor, in GiB, on the disk under `AV_WS_ROOT`: below it av reclaims finished tasks' workspaces and the build caches itself and starts no new task (HTTP 507) until there is room; `0` turns it off |
| `AV_BUILD_CACHE_MAX_GB` | `0` | build budget, in GiB, for each repo: the most its build output (every task workspace's `target/` plus `<repo>/.cache/`) may take; over it av removes finished tasks' `target/`, then idle ones, then the deps cache (see [Disk](#disk-what-agents-create-and-what-av-reclaims)); `0` turns it off |
| `AV_BUILD_IDLE_SECS` | `1800` | over the build budget, an unfinished (open, handed-off, in-review or conflicted) task's `target/` is removed only when nothing in it changed for this long |
| `AV_BUILD_CACHE` | `on` | `off`: write no cargo config (lean builds) for Rust repos |
| `AV_DATABASE_URL` | (none) | use this Postgres instead of the daemon's own |
| `AV_PG_DIR` | `$AV_HOME/pg` | data, password and binaries of the daemon's own Postgres |
| `AV_PG_INSTALL_DIR` | (none) | an existing PostgreSQL 16 installation (the directory holding `bin/`, e.g. `/usr/lib/postgresql/16`) instead of the bundled or downloaded one |
| `AV_DAEMON_BIN` | next to `av`, else `PATH` | the `scholion-daemon` that `av up` starts |

`av init [--name NAME] [--for claude|codex|grok|gemini|cursor|mcp-json]...` adds the git repository of the current
directory (as an import job it polls, so size is no problem) and wires the named agent CLIs
(default: Claude Code). `--for mcp-json` also needs `--path FILE`. See
[Agent integrations](#agent-integrations) and [Many agents at once](#many-agents-at-once).

### Full stack (Docker)

Prerequisites: Docker with Compose, and a Rust toolchain (for the host-side `scholion` binary).

```bash
export UID GID=$(id -g)          # bash does not export UID; compose needs both
docker compose -f deploy/docker-compose.yml up -d --build
cargo build --release -p scholion-cli  # -> target/release/scholion and target/release/av
```

The stack runs Postgres 16, Qdrant, text-embeddings-inference (TEI, `BAAI/bge-small-en-v1.5` on
CPU, port `${AV_TEI_PORT:-8080}`) and the daemon (`http://localhost:7420`), all in containers.

**Semantic layer.** The daemon embeds records, conventions and code chunks into Qdrant through an
outbox drained every second, and context packs then also rank by semantic similarity. It is on when
`AV_QDRANT_URL` is set and an embedder is found: a host `ollama` CLI with an `embeddinggemma-2` model, else a
`qwen3-embedding` one (the default when running the daemon on the host; `ollama pull
embeddinggemma-2`), otherwise `AV_EMBED_URL` (TEI in compose). The
container has no `ollama` CLI, so in compose it uses TEI. If Qdrant or the embedder is unreachable,
packs still return and carry the note `semantic`: they used Postgres full-text instead. After
changing the embedding model, run `scholion reindex` with no `--repo` (it recreates the collections at
the new dimension); `scholion reindex --repo NAME` rebuilds one repo. Reindex also brings `main`'s
code index up to date (unchanged files are skipped by content hash); after upgrading past migration
`0020` (constants became symbols), run it once so existing repos are extracted again. TEI downloads its model on first
start, which can take minutes; `scripts/smoke.sh` does not wait for it.

Defaults (override with environment variables before `up`):

| Variable | Default | Meaning |
|---|---|---|
| `AV_WS_ROOT` | `~/ws` | host directory where task workspaces appear (agents edit files here) |
| `AV_IMPORT_ROOT` | `~/projects` | host directory mounted read-only at `/import` for `scholion repo add` |
| `AV_PG_PORT` | `5432` | localhost port of the Postgres container |
| `AV_QDRANT_PORT` / `AV_QDRANT_GRPC_PORT` | `6333` / `6334` | localhost ports of the Qdrant container |
| `AV_TEI_PORT` | `8080` | localhost port of the embedding server |
| `AV_DAEMON_PORT` | `7420` | localhost port of the daemon (`scripts/smoke.sh` uses it for `AV_URL`) |
| `AV_WS_MOUNT` / `AV_IMPORT_MOUNT` | `AV_WS_ROOT` / `AV_IMPORT_ROOT` | bind-mount sources, when Docker sees the folders under another path (below) |
| `AV_EMBED_API_KEY` | empty | Bearer token for the embedding API. Used when the `ollama` CLI is not installed. |
| `AV_EMBED_URL` | empty | embedding API base URL. Required when there is no Ollama. |
| `AV_EMBED_PROVIDER` | auto | `ollama` or `api`. Auto uses Ollama when the CLI has an `embeddinggemma-2` or `qwen3-embedding` model. `ollama` with no CLI on `PATH` uses the Ollama server at `AV_OLLAMA_URL` (another container or host) and needs `AV_OLLAMA_EMBED_MODEL`. |
| `AV_OLLAMA_URL` | `http://127.0.0.1:11434` | Ollama server. With the CLI, the model tag comes from `ollama list`; without it (`AV_EMBED_PROVIDER=ollama`), the model is `AV_OLLAMA_EMBED_MODEL` as written. |
| `AV_OLLAMA_EMBED_MODEL` | `embeddinggemma-2`, else `qwen3-embedding` | the one Ollama model to embed with (a name as `ollama list` shows it, with or without the tag). Required with `AV_EMBED_PROVIDER=ollama` and no CLI: then it is sent to the server as written (`embeddinggemma-2` means `embeddinggemma-2:latest`), and a server that is down or lacks the model shows up in the daemon's log at the first embedding, not at start. Run `scholion reindex` after changing it. |
| `AV_PG_MAX_CONNECTIONS` | `16` | daemon's Postgres pool size. Raise it when many agents work at once. |
| `AV_MAX_CONCURRENT_STARTS` | a quarter of the pool | task starts that copy the index at once; the others wait their turn instead of failing. |
| `AV_LEASE_SECS` | `1800` | how long an agent holds an open task without writing to it; after that another agent may `scholion claim` it. |
| `AV_TOKEN` | (none) | shared bearer token for every route but `/healthz` and `/ui`; the CLI sends `$AV_TOKEN`. Unset = no auth. |
| `AV_BACKUP_REMOTES` | (none) | `repo=url` pairs, whitespace-separated: main is pushed there after every merge and undo (`scholion repo backup` sets one at runtime). |
| `AV_WS_RETAIN_SECS` | `3600` | how long a merged or abandoned task keeps its workspace; `0` removes it at once, `never` keeps it. |
| `AV_DISK_MIN_FREE_GB` | `20` | free-space floor, in GiB, on the disk under `AV_WS_ROOT`: below it the daemon reclaims finished tasks' workspaces and the build caches itself and starts no new task (HTTP 507) until there is room; `0` turns it off. |
| `AV_BUILD_CACHE_MAX_GB` | `0` | build budget, in GiB, for each repo: the most its build output (every task workspace's `target/` plus `<repo>/.cache/`) may take; over it the daemon removes finished tasks' `target/`, then idle ones, then the deps cache; `0` turns it off. |
| `AV_BUILD_IDLE_SECS` | `1800` | over the build budget, an unfinished (open, handed-off, in-review or conflicted) task's `target/` is removed only when nothing in it changed for this long. |
| `AV_ATTN_REVIEW_SECS`, `AV_ATTN_APPROVED_SECS`, `AV_ATTN_HANDOFF_SECS`, `AV_ATTN_FOOTPRINT_BYTES`, `AV_ATTN_FOOTPRINT_SECS` | `7200`, `3600`, `3600`, 512 MiB, `86400` | when `GET /v1/attention` lists a waiting task or a footprint (see [Needs attention](#needs-attention-get-v1attention)). |
| `AV_BUILD_CACHE` | `on` | `off`: write no cargo config (lean builds) for Rust repos. |
| `AV_INDEX_MAX_FANOUT` | (none) | a call whose bare name more than N other symbols define (`new`, `get`) links to none of them in the code graph. Cuts the edges of big repos (tokio: 71% at 64) without a measured loss in packs (`bench/results/2026-11-03-e1-fanout-cap/`). Set it before importing: stored edges keep the cap they were built with, and `reindex` skips unchanged files. |

### Docker Desktop without WSL integration

Validated 2026-10-19 with Docker Desktop 29.8 driven from WSL through `docker.exe`. Without WSL
integration for the distro, Docker cannot bind-mount WSL paths (the mount silently lands inside
Docker's VM). Keep the workspace and import folders on a Windows drive and give both names:

```bash
export AV_WS_ROOT=/mnt/c/Users/me/avws       AV_WS_MOUNT='C:\Users\me\avws'
export AV_IMPORT_ROOT=/mnt/c/Users/me/avsrc  AV_IMPORT_MOUNT='C:\Users\me\avsrc'
```

`docker.exe` also does not see WSL env vars unless they are listed in `WSLENV` (for example
`WSLENV=UID:GID:HOME:AV_WS_MOUNT:...`), cannot find `docker-credential-desktop` unless `PATH/l` is
in `WSLENV`, and fails builds with "Incorrect function" when `DOCKER_CONFIG` is on a WSL path.
Enabling WSL integration for the distro in Docker Desktop avoids all of this.

### Docker Desktop memory on Windows

Docker Desktop runs in a WSL 2 VM that by default may grow to half the machine's RAM and keeps
file cache until then: builds write gigabytes of `target/`, and that cache stays charged to
`vmmemWSL` on Windows long after the build. Measured on a 32 GB machine (2026-11-03): two
one-minute cold builds took the VM from 4.0 to 9.4 GB while its processes used 2.4 GB; dropping
the cache brought it back to 4.1 GB. To have WSL give cache back by itself, and to cap the VM,
create `%USERPROFILE%\.wslconfig`:

```ini
[wsl2]
memory=12GB                 # the VM's cap; default half the RAM
[experimental]
autoMemoryReclaim=dropCache # give cache back when the VM is idle (or `gradual`)
```

It takes effect after `wsl --shutdown`, which stops Docker Desktop and every container in it, the
daemon included. To return the cache once, now, without a restart:

```bash
docker run --rm --privileged rust:1.97.1 sh -c 'sync; echo 1 > /proc/sys/vm/drop_caches'
```

One cold `cargo test --workspace --no-run` of scholion needs about 1.6 GB of process memory; how
many agents fit on one host is worked out in Plan 12 ("Memory").

## Use

```bash
scholion repo add pantry /import/pantry            # path as seen inside the container
scholion repo list                                 # every imported repo, its main and source
scholion task start --repo pantry --title "Atomic save" --goal "write pantry.json atomically so a crash cannot truncate it"
cd ~/ws/pantry/atomic-save                   # printed as `workspace:`
scholion checkpoint                                # snapshot + refresh the index
scholion record --kind attempt --summary "tried os.rename; fails on Windows if the file exists" --anchor pantry/store.py
scholion record --kind note --summary "os.replace overwrites on Windows too" --supersedes rec:1a2b3c4d   # correct a stored record
scholion context                                   # 800 tokens of what an agent needs; --budget N for more
scholion context --fresh                           # resend everything (e.g. after context compaction)
scholion submit                                    # to review (needs a decision, attempt or note)
scholion handoff --done "atomic save added" --remaining "tests for the crash path"   # prints the bundle
AV_AGENT=codex scholion claim <task-id>            # another agent takes over; rewrites .av-task
scholion bundle --pr                               # review bundle: tests at head, risks, checklist
scholion search "atomic"                           # full-text over the repo's records
scholion log                                       # main's history, newest first
scholion abandon                                   # end a task for good (owner only)
```

The path given to `repo add` is read by the daemon, not the client: the daemon clones it where it runs. With
the Docker stack (`deploy/docker-compose.yml`) the daemon sees only the `/import` mount, which is the
host's `AV_IMPORT_ROOT` (default `~/projects`, read-only), so put the repo under it and pass
`/import/<dir>`. A path the daemon cannot see (missing, or a directory that is not a git repository)
is refused at once with `400` and that hint, before any import starts.

### Correcting a record

A record is what an earlier agent believed when it wrote it, and it can be wrong. Records are
append-only, so a wrong one is corrected by a new record that supersedes it:

```bash
av record --kind note --summary "os.replace overwrites on Windows too; os.rename was the problem" --supersedes rec:1a2b3c4d
# recorded rec:9f8e7d6c (note), supersedes rec:1a2b3c4d
```

`--supersedes` takes the `rec:…` id that a pack, the pre-read hook and `av search` print. A
prefix of the id (6 or more hex characters) or the full id works too, with or without the id's
hyphens; a prefix two records share is refused. Each entry of `av record --json` may carry
`"supersedes": "rec:…"`, the MCP `record` tool has the same `supersedes` argument, and so has the
body of `POST /v1/tasks/{id}/records` (and each of `submit`'s inline `records`, in the API and in
the MCP tool).

- The old record is neither deleted nor edited. It stays in its task's records
  (`GET /v1/tasks/{id}/records`, with `superseded_by` set) and is no longer shown by packs, the
  pre-read hook or search, which show the new record instead.
- A correction given no anchor takes the anchors of the record it corrects, so it is shown
  wherever the wrong one was. Given anchors of its own, it keeps those.
- Any task of the repo may correct a decision, attempt or note, its own included, and the
  correction is itself a decision, an attempt or a note: those are what packs and the pre-read
  hook show in the old record's place. A record of another kind is refused (`422`, saying which
  kinds may correct), since the claim would vanish instead of being corrected.
- Also refused: an id no record has (`404`); a malformed or ambiguous one (`400`); a record of
  another repo (`422`); a `review` or `validation` record (`422`: a finding or a failed check is
  not hidden by writing a note); another task's `intent` or `handoff` (`422`; a task may restate
  its own, with a record of the same kind only); and a record that is already superseded (`409`,
  naming the live record at the end of the chain, which is the one to correct; a chain longer
  than 64 corrections is not followed to its end, and the answer then names none).
- The store enforces the same rules inside the insert, with the old record's row locked: when
  two tasks supersede one record at the same moment, one is written and the other gets the `409`.
  A record has at most one successor.
- The feed gets a `record:superseded` event (`{old, new, by}`).

Agents are told. A pack whose *Decisions & dead ends* section shows another task's decision,
attempt or note starts that section with one line, inside the token budget: ``Claims by earlier
agents: check against the code; correct wrong ones with `av record --supersedes rec:…` ``. It is
left out when the section holds only the task's own records, or only back-references to an earlier
pack of the session. The pre-read hook's answer ends with the same advice once per session.

### Cold start from git history

A newly added repo has no records, so its first agents learn nothing from its past. Mining is
opt-in and capped:

```bash
av repo add myrepo /import/myrepo --mine-history 500   # import, then mine at most 500 records
av init --mine-history 500                             # the same for the repo of this directory
av repo mine myrepo --limit 2000                       # later, or for a repo imported before
```

The daemon walks every ancestor of main in its own jj store (merged branches too), newest commit
first, and keeps a commit when it has one parent, its subject is not housekeeping (merges,
reverts, bumps, `chore`/`docs`/`ci`/`style`, formatting, typos, `wip`), its message minus trailers
(`Co-Authored-By`, `Signed-off-by`, …) and links is at least 20 characters, and it touched at
most 25 files (more is a sweep: a rename, a reformat). Each becomes one `note` by the agent `git`:
the subject is its summary, the rest of the message its body (with `Mined from git commit
<sha>`), the commit its `change_id`, and the files the commit touched its anchors. They belong to
one task per repo, `<repo>/git-history`, owned by `git` and merged from the start (it has no
workspace and is never offered as work). A commit mined before is skipped, so mining again only
adds what is new; `N` counts records written, not commits looked at. The rules are
`scholion_core::history`, which `scholion-bench mine` shares. `git` is reserved for these records:
an agent using that name would have its records ranked as mined ones.

Mined records rank below what agents wrote (`MINED_FACTOR` 0.5 in packs; after agents' records in
`records/at`, so the Claude Code `PreToolUse` hook shows them last), and they say where they come
from wherever they are printed: `- rec:… note (git 1a2b3c4): <subject>`. They reach agents through
their anchors (the hook on opening a file, a pack once the task changes that file) and through
semantic search when it is on. `av why path:line` on imported history prints the mined record of
the commit it blames. A blame of imported history does not go back commit by commit: it stops at
the newest imported commit that changed the file and gives it every line, so that record is the
file's latest commit, not necessarily the commit that wrote that line.

### Many agents at once

Run as many agents as you like on one daemon. There is no seat cap: each distinct agent
that calls `next` gets the work that fits it.

```bash
av --agent claude init --for claude  # and --for codex, grok, gemini, cursor; --agent names the MCP caller
av --agent muse init --for mcp-json --path PATH/TO/settings.json   # Muse's own path is unverified

av agent set claude --best code,tests --can docs --cost 4
av agent set grok --best docs,review --can code,tests --cost 2
av agent set muse --best ui --can docs --cost 1

av --agent claude watch -- claude
av --agent grok watch --every 30 -- grok -p "take your next av job"
av --agent muse watch -- muse
```

`--agent NAME` (or `AV_AGENT`) is that CLI's name when it calls `next`. `av agent set NAME
--best …` is how `next` routes by strength: an item goes first to the agent whose `best`
tags cover what it needs. `av agent rm NAME` removes a profile (`DELETE /v1/agents/{name}`).
An unknown name is an error. So is removing an agent that still owns an open, handed-off,
in-review or conflicted task, unless `av agent rm NAME --force` (the tasks stay). A profile an extension
manages cannot be removed here, the same as `av agent set`. `av watch`, once per agent,
starts that CLI only when something fits it and sets `AV_AGENT` for the command.
`av watch --max-agents N` (or `AV_MAX_AGENTS`) caps the agents running at once on the host,
across every watch that shares `$AV_HOME`: a watch runs its command only while it holds one of
N lock files in `$AV_HOME/agents`, and the OS releases a dead watch's lock. With no free slot
`--once` exits without running and a plain watch waits. Size N from memory: about 0.8 GB per
agent CLI plus 1.6 GB for each build running at the same moment (Plan 12, "Memory").
`av validate --max-builds N` (or `AV_MAX_BUILDS`) does the same for builds: a validation runs
its command and the required checks only while it holds one of N slots in `$AV_HOME/builds`, and
the others wait (it says so once on stderr), so a burst of checks after a merge to `main` runs N
builds at a time. Only `av validate` takes a build slot; an agent running `cargo test` itself
does not.

The dedicated writers put a fixed `X-Av-Agent` header (`codex`, `grok`, `gemini`, `cursor`).
`--for mcp-json` writes the `--agent` name into that header, so a client without its own
writer still routes as itself. Set a profile under that same name.

The reserve window, review order and `--max-open` are in the next section.

### Several agents, several subscriptions

One daemon serves every agent; each CLI only needs its own config entry for the same `/mcp` URL
(`av init --for …`). To spread work over agents by what each is good at and what it costs:

```bash
av agent set claude --best code,tests --can docs --cost 4     # 1 is the cheapest
av agent set muse --can code,docs --cost 1
av agent set grok --best image --can ui --cost 2 --alias grok-shell-av
av agent set cursor --best code --can tests,ui,docs --cost 3 --max-open 2   # at most 2 open tasks
av post --repo pantry --title "Redraw the logo" --goal "..." --needs image   # to the backlog
av post --repo pantry --title "Use the new logo" --goal "..." --after redraw-the-logo   # not offered until that item merges; `av backlog` says "waiting on"
av target add --repo pantry --title "Ship v1" --due 2026-12-01   # something the team works toward
av post --repo pantry --title "Price labels" --goal "..." --target ship-v1   # counts toward it
av backlog target ITEM ship-v1                                   # or link an item later (no target: unlink)
av post --repo pantry --title "Cache labels" --goal "..." --propose --why "may not pay off"   # waits for a human
av backlog accept ITEM                                           # queue a proposal (av backlog drop ITEM: no)
av repo settings pantry --agent-posts propose                    # every agent's post is a proposal
av target list --repo pantry               # each active target: merged/items, working, queued, due
av target done ship-v1 --repo pantry       # also: drop, reopen
av next                                    # as an agent: the next job that fits it, or none
av next --wait 50                          # ... waiting up to 50 s for one
av next --take review,merge                # only reviews and merges (--take work: never reviews)
av handoff --done … --remaining … --needs image   # hand off to an agent that has the tag
av --agent grok watch -- grok -p "take your next av job"   # start grok only when work fits it
```

An agent's own CLI runs the work on its own subscription; Scholion holds no keys for the agents'
own models. Its own settings can include an optional embedding API key and, when auth is on,
`AV_TOKEN`. The daemon does not launch agents (`av watch` does, on your machine). Agents pull: MCP agents call `start_task` with no arguments, the others
`av next`. Rules (no model involved): an agent can take an item when every tag it needs is in the
agent's `best` or `can`; the best-fitting agent (the item's `--prefer`, then most `best` tags, then
cheapest) has it to itself for `--reserve` seconds (default 600), after which any agent that can do
it may take it, so work moves on when an agent runs out of quota. Untagged work goes to the
cheapest agent.

Each agent CLI is treated as an engineering team: Scholion gives it work, has another team review
it and keeps the shared memory, while the CLI splits the work among its own subagents. Those
subagents carry the team's name (the `X-Av-Agent` header is in the CLI's config), so a team never
reviews its own work. Post items the size of a team's job, a goal with a definition of done, not
single steps. `--max-open N` caps how many open tasks a team holds at once (tasks in review do
not count): at the cap, `next` gives it only reviews, merges and its own abandoned tasks, so one
team running many subagents cannot take the whole backlog.

`next` (and `start_task` with no title or goal) closes the loop. Its order: first reviews of other
agents' submitted work (an agent never reviews its own; it reviews work it could have done, unless
only the owner could, then anyone else), with an approved task offered as "merge it"; then handed-off
tasks (one without tags keeps its item's tags, with the same exception), open tasks whose owner went
quiet for a whole lease (the owner may resume its own), and backlog items. A review offered to one
agent is reserved for it for 10 minutes. `wait_secs` (MCP `wait`, CLI `--wait`, at most 50) holds
the call until something fits; `dry_run` says what it would give without taking it, which is what
`av watch` polls. The MCP task tools take a slug (`louder-hello` or `pantry/louder-hello`) as well as an
id. A **target** (a release, a milestone) groups backlog items: an item counts toward at most one
active target of its repo, and the target's progress is how many of its items merged (a dropped
item stops counting). Targets are never deleted; they are done or dropped and can be reopened. They
do not change routing.

**Proposals** let agents post follow-up work without deciding alone whether it is wanted. A
backlog item posted with `propose: true` (`av post --propose`, the MCP tool `post_backlog` with
`propose`) is `proposed`, with an optional `why` (at most 300 characters: why the poster is
unsure). `next` never offers a proposed item and does not count it as queued; `POST
/v1/backlog/{id}/accept` (`{by}`, `av backlog accept`) makes it `queued` (a `backlog:accepted`
event; waiting `next` calls wake as for a post), and `DELETE /v1/backlog/{id}` (`av backlog drop`)
drops it. Accepting an item that is not proposed is a 422, as is an accept the extension's guard
refuses. An accepted item's reserve window starts when it is accepted, and it ranks as that new.
The repo setting `agent_posts` (`PUT /v1/repos/{name}/settings`, `av repo settings NAME
--agent-posts queue|propose`; default `queue`) decides what becomes of an agent's post: with
`propose`, every post by an agent lands as `proposed` whatever it asked. A post is an agent's when
it comes through `post_backlog` (the tool sends `from_agent: true`) or its `posted_by` is an agent
profile's name or alias; anyone's explicit `propose` is always honoured. A proposed item counts
toward its target only once accepted (`GET /v1/targets` reports it apart, as `proposed`); an item
`--after` a proposal waits until the proposal is accepted, taken and merged. `av backlog` marks a
proposal with who proposed it and why, the dashboard lists proposals under the queue, and `GET
/v1/attention` reports each as `proposed_backlog`.

The dashboard lists the teams (skill, cost, and open tasks against the limit), the queue and who each item is reserved for. Design:
`docs/superpowers/specs/2026-10-31-agent-routing-design.md`; how to measure whether it pays:
`docs/superpowers/plans/2026-10-31-plan-8-grok-evaluation.md` (not yet run: no claim is made).
To try it with real agents, run `examples/routing-smoke/setup.sh`. It sets up a small sample
project with profiles for Claude Code and Grok and a
tagged backlog. Each table in its README says which agent should take which task. With no
subscription, `examples/routing-smoke/deepseek-demo.sh` runs two Claude Code agents on DeepSeek
Flash through the same loop on its own daemon (the run checked for 0.0.2).

### An agent on another host

The daemon owns every workspace and snapshots it; an agent on another machine works in a checkout
of the task there and sends each checkpoint as a patch (Plan 14,
`docs/superpowers/plans/2026-11-04-scholion-plan-14-remote-workspaces.md`). The checkout is a plain
git repository `av` makes and keeps in step; it needs `git` on that host, the daemon needs nothing
new.

```bash
export AV_URL=http://daemon-host:7420 AV_TOKEN=...      # the daemon on the other machine
av task start --repo pantry --title "Atomic save" --goal "..." --checkout ~/src/atomic-save
av remote clone <task-id> ~/src/atomic-save             # or: a checkout of a task that exists
cd ~/src/atomic-save
av checkpoint                    # sends `git diff` since the last sync; the daemon applies + snapshots
av remote pull                   # takes the daemon's head (after `av sync`, a same-host edit, ...)
```

In a checkout, `av checkpoint`, the Claude Code hooks, `av validate` (its checkpoint), `submit`,
`record`, `handoff` and `sync` send the checkout's edits first, so the daemon answers from the
files the agent has. When the daemon's head has moved since the last sync (a checkpoint on its
host, `av sync`'s rebase, `undo` or `revert`), the checkpoint is answered `409` with the head;
`av checkpoint` then pulls (a three-way apply puts the local edits back) and sends once more. Lines
both sides changed get conflict markers in the checkout and the command exits non-zero naming the
files; resolve them and `av checkpoint` again. The local edits are also kept as a git commit
before the pull, named in the message if they could not be put back.

The API, for other clients:

- `POST /v1/tasks/{id}/checkpoint?base=<change>`, body a patch as
  `git diff --binary --full-index --no-renames --src-prefix=a/ --dst-prefix=b/` writes it
  (text, binary `literal`/`delta`, new and deleted files, modes), at most 64 MiB (`413` above).
  `200` is a checkpoint's answer plus `applied` (the paths written) and `tree` (the git tree id
  of the new head, which a client can compare with its own `git write-tree`). `409`: stale base,
  with `head_change` and header `X-Av-Head`. `400`: a patch that is malformed, does not match the
  workspace's files, renames or copies, changes a symlink or submodule, or names a path outside
  the workspace or in `.jj`, `.git` or `.av-task`; nothing is written. `422`: a write under a
  lost claim (`X-Av-Generation`), as for every write. Without `base` the route is the usual
  local checkpoint and ignores its body.
- `GET /v1/tasks/{id}/patch?from=<change>` (`main`, `task:<slug>` or a commit id; without `from`
  the empty tree, so a whole checkout): the task's head as a patch `git apply` accepts, conflicted
  files with their markers; headers `X-Av-Head` and `X-Av-Tree`. Reads the store only.

Not supported: two writers on one task from different hosts (the second one's checkpoints keep
pulling), symlink changes from a checkout, and sending tasks to other hosts (multi-host dispatch).

### Dashboard

Open `http://localhost:7420/ui` to watch agents work: a Teams column left of the board that works
as an office (one desk per agent: its skills and cost, how many open tasks it holds against its
limit, what it is on now, what it did last, and whether it is working, reviewing, idle or *quiet*:
holding open tasks it has not written to for a whole lease, so `next` offers them to others), tasks
by status in the middle, and on the right the repo's targets with their progress (overdue ones in
red), the backlog and a live activity feed (the `/v1/events` long-poll). The three columns need a
window 1580 px wide; narrower, the desks are a row above the board, and below 1180 px everything
stacks. A task's drawer shows its records, changed files and the diff from base to head (on
request, one file at a time), its claim and review state and every review round (each verdict with
the reviewer's findings at that head), a validation record as passed or failed with its command and
output, its own timeline (starts, checkpoints with how many paths they touched, claims, submits,
verdicts, merges, conflicts) and, after "Build context pack", the pack's receipt (each item's tier,
reason, source and tokens). The page reads the repo's event history (`GET /v1/events` in pages of
500, the newest 10 000 ids of a long one, then followed live) and derives time in status, timelines
and review rounds from it; nothing new is recorded. Each card says how long its task has been in its
status ("approved 20m" once its review is given) and goes amber once it is stuck (a conflict, a
submit with no verdict for 2 h, an approval not merged for an hour, a handoff nobody took for an
hour, an open task with no write for a whole lease); a "Needs attention" strip above the board
lists, worst first, a red main, conflicts, those stuck tasks, backlog items no agent can do,
overdue targets and big footprints, at most eight rows. Both read the daemon's
[`GET /v1/attention`](#needs-attention-get-v1attention), so a script sees the same list. The Flow panel shows, over 7, 14 or 30 local days (kept in
this browser), merges per day, the median cycle time (start to merge), the median review wait
(submit to verdict; an approval the daemon carries over a resubmit is left out), rework (changes
requested, conflicts, undos, reverts) and per agent what it started, merged and reviewed, from each
agent's timeline (its newest 500 rows, read at most once a minute; "≥" marks a count cut off there).
Click a desk (or a task's owner) for that agent's
timeline: the records it wrote, the tasks it started and claimed, its review verdicts, its submits
and its merges, newest first. It is built from what Scholion already keeps; nothing extra is
recorded, and no cost is tracked. "Build context pack" shows exactly what the next agent would
receive at a given budget.
Under the board, a Main panel lists main's last commits with their post-merge check and the task
that landed each (read once the history is in, then when a merge, undo or revert arrives, and once
a minute while the page is live, since a post-merge check lands with no event), and its header says whether semantic search is on and how many rows wait in the
outbox, the backup's state (ok, pending, failing), how many conventions the repo has and whether an
import is running; a Disk panel (read at start, on its button and every five minutes while the
page is live and visible, since measuring walks the filesystem) shows what workspaces, build
caches, the store and scratch dirs take, the repo's workspaces largest first with the ones a sweep
would reclaim, and what sessions left outside any workspace (512 MiB or more, the hooks' default,
by a session of this repo's tasks, or of no task, in the last day earns a "Needs attention" row). The page never deletes;
`av gc` does. When the page cannot load (the daemon is away at start, or the history cannot be
read) it asks `/healthz` every five seconds and starts over once it answers; a feed that drops
mid-session keeps reconnecting every few seconds as before.
The page is read-only and built into the daemon (no Node, no build step); with `AV_TOKEN` set it asks
for the token once per browser tab. It reads `GET /v1/repos`, `GET /v1/repos/{name}/health`,
`GET /v1/tasks` (`?repo=`, `?status=`, `?limit=`), `GET /v1/tasks/{id}`,
`GET /v1/tasks/{id}/records`, `GET /v1/tasks/{id}/unrecorded`, `GET /v1/tasks/{id}/context`,
`GET /v1/tasks/{id}/bundle`, `GET /v1/events`, `GET /v1/backlog`, `GET /v1/targets` (`?repo=`,
`?status=active|done|dropped`), `GET /v1/office` (every agent's desk; `?repo=` limits it to one
repo's tasks), `GET /v1/agents/{name}/timeline` (`?repo=`, `?limit=` up to 500, default 100),
`GET /v1/repos/{name}/diff`, `GET /v1/receipts/{id}`, `GET /v1/repos/{name}/log`,
`GET /v1/repos/{name}/outbox`, `GET /v1/repos/{name}/backup` (with `advance`), `GET /v1/repos/{name}/conventions`,
`GET /v1/repos/{name}/import`, `GET /v1/disk`, `GET /v1/attention` and `/healthz`.
Targets are written with `POST /v1/targets`, `PUT /v1/targets/{id or slug}` (`{status, repo}`) and
`PUT /v1/backlog/{id}/target` (`{target}`, null to unlink); a proposed backlog item is accepted
with `POST /v1/backlog/{id}/accept` (`{by}`).

### Needs attention (`GET /v1/attention`)

`GET /v1/attention?repo=R` (every repo without `repo`; an unknown one is a 404; behind the token
like every `/v1` route) answers what needs a person or an agent now, the list the dashboard's strip
shows:

```json
{"items": [{"kind": "stale_review", "severity": 2, "repo": "demo", "task_id": "…",
            "slug": "louder-hello", "agent": "claude", "since": "2026-11-05T09:12:03Z",
            "title": "Louder hello", "detail": "submitted by claude, no verdict yet"}],
 "generated_at": "2026-11-05T12:00:00Z"}
```

Items come worst first: severity (3 act now, 2 stuck, 1 tidy up), then the kind's place in the
table, then the oldest `since` (a stale review and an approval nobody merged share a place).
`task_id`, `slug` and `agent` are there when the item is about a task (or, for a footprint, a
session's agent); `backlog_id`, `target_id` and `bytes` when they apply. `title` is the task's,
item's or target's title, or a one-line summary; `detail` is a sentence for people. The `kind`
values are stable:

| kind | severity | when | `since` |
| --- | --- | --- | --- |
| `disk_low` | 3 | free space on the disk under the workspace root is below `AV_DISK_MIN_FREE_GB` even after the daemon reclaimed what it could, so no new task starts; daemon-wide (`repo` is null, listed whatever `?repo=` says), `bytes` is the free space | when the daemon first measured it below |
| `main_red` | 3 | the post-merge check of main's newest checked commit failed (as `GET /v1/repos/{name}/health` says; `slug` is the task that landed the oldest red commit) | when that commit's failing result was posted |
| `conflicted` | 3 | a task's merge conflicted: its owner must sync, resolve and resubmit (no threshold) | when it entered `conflicted` |
| `stale_review` | 2 | in review with no approval at its head for over `AV_ATTN_REVIEW_SECS` | when it entered review (an approval does not restart the clock) |
| `approved_not_merged` | 2 | approved at its head (an `approve` after its newest submit or undo) and not merged for over `AV_ATTN_APPROVED_SECS` | the approval |
| `unclaimed_handoff` | 2 | handed off and not claimed for over `AV_ATTN_HANDOFF_SECS` | the handoff |
| `quiet_lease` | 2 | open, and not written to for a whole lease (`AV_LEASE_SECS`, reckoned as the office's *quiet* and `next`: from the last write, or the start) | the last write |
| `unroutable_backlog` | 1 | a queued backlog item no profile fits, when it needs a tag or some profile exists (without profiles any agent takes untagged work) | the item was posted |
| `proposed_backlog` | 1 | a proposed backlog item waiting for a human to accept or drop it (no threshold; `agent` is who proposed it, `detail` says why) | the item was posted |
| `overdue_target` | 1 | an active target whose due day (UTC) has passed | the end of its due day, 00:00 UTC the day after |
| `build_over_budget` | 1 | a repo's build output is still over `AV_BUILD_CACHE_MAX_GB` after the daemon removed what it may: only `target/`s touched within `AV_BUILD_IDLE_SECS` are left (`bytes` is the build output, as the last enforcement counted it) | when an enforcement first found it still over |
| `footprint_outside` | 1 | a session reported leaving at least `AV_ATTN_FOOTPRINT_BYTES` outside any workspace within `AV_ATTN_FOOTPRINT_SECS` (`POST /v1/footprint`, kept in memory); with `repo`, only sessions of that repo's tasks or of no task | the report |

The thresholds are the daemon's:

| Variable | Default | Meaning |
| --- | --- | --- |
| `AV_ATTN_REVIEW_SECS` | `7200` | a review nobody gave |
| `AV_ATTN_APPROVED_SECS` | `3600` | an approval nobody merged |
| `AV_ATTN_HANDOFF_SECS` | `3600` | a handoff nobody claimed |
| `AV_ATTN_FOOTPRINT_BYTES` | `536870912` (512 MiB, the hooks' warning size) | what a session may leave outside any workspace |
| `AV_ATTN_FOOTPRINT_SECS` | `86400` | footprints older than this are left out (the files may be gone) |

`av attention [--repo R] [--json]` prints one line per item (`severity kind age repo/slug title ·
detail`, the age on the daemon's clock) or the answer as JSON, and exits 0 whatever it lists.

## Disk: what agents create and what av reclaims

Coding agents make a lot of files by themselves: build output, dependencies, clones, test data,
scratch files. Nothing used to own their lifetime. One machine filled 333 GB this way:

| What the agent made | Size | Why it stayed |
|---|---|---|
| A cargo `target/` wherever it built: one per task workspace, plus `/tmp` and ad-hoc dirs | about 24 GB per build of this workspace | every task is a full checkout with its own target dir, and no workspace was ever removed |
| Task workspaces | a few GB | never removed on merge, abandon, undo or revert |
| Bench clones and venvs | about 1 GB per session | `scholion-bench` kept every clone |
| Ad-hoc dirs outside any workspace (`~/dogfood`, `~/.grok/worktrees`, …) | tens of GB | no hook looked at them and nobody asked about them |

```bash
av disk                     # workspaces (build output apart), scratch, caches and store per repo, and what sessions left outside
av disk --json              # the same, as GET /v1/disk returns it
av disk --fs                # only free and total space, the floor and the build budget (GET /v1/disk?fs=1: no walk)
av gc --dry-run             # what would be removed now
av gc                       # remove workspaces past the retention (POST /v1/gc)
av gc --retain 0            # every merged or abandoned task's workspace, whatever its age
av gc --build-cache         # also each repo's build cache (<repo>/.cache: the built dependencies new tasks start with)
av gc --containers          # also what merged and abandoned tasks left in docker on this host (labelled av.task=<id>)
                            # (with AV_BUILD_CACHE_MAX_GB set, every av gc also holds each repo to its build budget)
```

**Workspaces of finished tasks.** Once a task has been merged or abandoned for
`AV_WS_RETAIN_SECS` (default 3600), the daemon removes its workspace `<ws_root>/<repo>/<slug>` and
its scratch dir, and forgets the jj workspace. The task's commits stay in the store. A sweep looks
every 60 s; `av gc` does it on demand (`--retain SECS` overrides the setting for one run).
`AV_WS_RETAIN_SECS=0` removes the workspace with the merge or abandon itself (their answer says
`workspace: removed`); `never` leaves it to `av gc --retain`. Whatever the retention (but `never`), the merge
or abandon removes the workspace's build output (`target/`) at once: it is the bulk of a Rust
task's disk and rebuilds on demand, while the small workspace stays for an undo or a reviewer's
look (the answer says `build: removed`, `none` or `kept`; the feed gets `build:reclaimed`). An undo or revert that puts a merged
task back to work restores its workspace at the task's head. Open, handed-off, in-review and
conflicted tasks are never touched, and the daemon deletes nothing outside `ws_root`. `av disk`
marks the workspaces a sweep would remove as `reclaimable`.

**The free-space floor.** When the disk under `ws_root` has less free than `AV_DISK_MIN_FREE_GB`
(default 20 GiB, `0` turns it off), the sweep and every task start first reclaim as `av gc --retain 0
--build-cache` would (finished tasks' workspaces oldest first, then each repo's build cache) and,
while it is still below, refuse to start a task with HTTP 507 and a reason naming the floor and
the free bytes; `GET /v1/attention` lists `disk_low`. Tasks already started keep working, and
reviews, handoffs and merges are still offered. `GET /v1/disk` reports `fs_free_bytes` and
`fs_total_bytes` of that filesystem (null where not measured: the check then never refuses),
`min_free_bytes` and `below_floor`; `av disk` prints them on its second line. `GET /v1/disk?fs=1`
(or `fs=true`) answers only those four and `build_budget_bytes`, measured the same way, without
listing repos, workspaces or footprints and without walking a directory, so a monitor can poll it
cheaply (the full answer measures every workspace and `target/`); `av disk --fs` prints that line
and `av disk --fs --json` the object.

**The build budget.** `AV_BUILD_CACHE_MAX_GB` (default `0`, off) caps each repo's build output:
every task workspace's own `target/` plus the repo's `.cache/` (the deps cache and the build
output on its way in or out; what is already moved into `.cache/reclaim/` no longer counts). The
target dirs stay one per workspace: a budget is a cap over what exists, never a shared target dir
(which would let `cargo test` in one workspace run another task's binary, see below). Every
reclaim tick (60 s) and every `av gc` hold each repo to it: while a repo is over, the daemon
removes, in this order, (1) the `target/` of merged and abandoned tasks, the one longest in that
status first, keeping their dependencies in the deps cache as a merge does; (2) the `target/` of
unfinished tasks that nothing touched for `AV_BUILD_IDLE_SECS` (default 1800; the newest
modification time of `target/` and what is at depth 1 and 2 under it, so a build under way, which
writes `debug/.cargo-lock` and new files into `debug/deps/`, counts as touched), least recently
touched first; (3) the repo's deps cache `<repo>/.cache/deps-target`, only if still over. A
`target/` touched more recently is never removed, whatever the budget: it may be building. Each
removal takes the task's lock and reads its status and idle time again; the feed gets
`build:reclaimed` with `reason: budget`, and the daemon's log says what went and why. Workspaces,
scratch dirs and anything outside `ws_root` are never touched; a removed `target/` simply
rebuilds. `av gc` (`POST /v1/gc`) answers what it removed under `build_budget` (`removed`: `what`
`finished`, `idle` or `deps_cache`, with `repo`, `slug`, `status`, `bytes`; `freed_bytes`; `over`:
the repos still over), and `--dry-run` only says it. `GET /v1/disk` gives each repo `build_bytes`
(its workspaces' `target/`; the budget counts these plus `build_cache_bytes`, less what is on its way out in `.cache/reclaim/`),
`build_budget_bytes` (0 = off) and `over_budget`, and the answer `build_budget_bytes` and
`totals.over_budget`; `av disk` prints `build 12.3 GB of 20.0 GB (AV_BUILD_CACHE_MAX_GB)` under
each repo. A repo still over after all that (only recent builds are left) is listed by
`GET /v1/attention` as `build_over_budget`. One value applies to every repo.

**Lean builds, one target dir per task.** Build output is where the disk goes: in the first
dogfood run, a Rust task's `cargo test` left about 20 GB (14 GB of test binaries that were 73%
debug info, 4 GB of incremental cache, 3 GB of dependencies), against about 3 MB of Scholion's
own state per task. When a repo with a `Cargo.toml` at its root is imported (and for every repo at
start-up), the daemon writes `<ws_root>/<repo>/.cargo/config.toml`:

```toml
# written by av: lean builds for every task workspace of this repo.
# Delete this file, or set AV_BUILD_CACHE=off before `av up`, to opt out.
# Each workspace builds into its own target/ (removed with the workspace); never share a
# target dir between workspaces of one repo: cargo would run another task's test binary.
[build]
incremental = false
[profile.dev]
debug = "line-tables-only"
# Dependencies carry no debug info at all; the repo's own crates keep file:line.
[profile.dev.package."*"]
debug = false
```

Cargo reads `.cargo/config.toml` in the current directory and every parent, so this one file
covers every task workspace of the repo without touching the repo or any snapshot. Measured on
scholion's own workspace, a full `cargo test` build is 4.3 GB with it against 21 GB without
(6.7 GB with line tables for dependencies too): line tables keep file:line in panics and
backtraces of the repo's own code at a fraction of full debug info, a panic inside a dependency
still names the dependency's frames, and the incremental cache only pays off for an editor's
rebuild loop, not for validations. A deeper
`.cargo/config.toml` (the repo's own) and `CARGO_*` in the environment still win. A file whose
first line is not av's is never rewritten. `AV_BUILD_CACHE=off` writes none.

The file sets no `target-dir`: every workspace builds into its own `target/`, which `av disk`
shows apart (`build_bytes`) and which goes with the workspace when the task is reclaimed, so the
disk a repo takes is bounded by the tasks in flight. Never point two workspaces of one repo at
one target dir (an earlier av did): cargo names a path package's artifacts by its path relative to
the workspace root, so both produce the same names and `cargo test` in one runs the other's test
binary, a validation that is green or red for the wrong task. A reviewer who validates in the
owner's workspace reuses its warm build. The snapshot never takes a workspace's top-level
`target/`, even in a repo that forgot to ignore it. Python and Node already have global caches
(pip, uv, npm, pnpm), so nothing is written for them.

**Dependencies are built once per repo.** Half of a task's `target/` is dependencies, the same in
every task. When a task's build output is reclaimed (merge, abandon, or the whole workspace), the
daemon first keeps the units that come from outside the repo (registry and git crates, their
build scripts and those scripts' output) in `<ws_root>/<repo>/.cache/deps-target/`, and a new
task's workspace starts with them in its `target/`, so cargo compiles only the repo's own crates.
Measured on this repo: a full `cargo test` build from nothing is 2.4 GB and 38 s; seeded, it adds
1.2 GB and takes 15 s, with no dependency recompiled. The compiled libraries (`*.rlib`, `*.rmeta`,
proc-macros) are hard links, one copy on disk for every task: rustc replaces such a file rather
than writing into it, so a rebuild in one workspace never changes another's. Dep-info, cargo's
fingerprints and build-script output are copied (about 100 MB per task), because those are
rewritten in place. A unit goes into the cache only when its dep-info lists sources from cargo's
registry or git checkouts; the repo's own crates never do (their artifacts would be taken as
fresh in another workspace). An entry already in the cache is kept; the cache only grows until
`av gc --build-cache` removes `<repo>/.cache`, and workspaces keep their links. `av disk` counts a
linked file once, under the repo's build cache. Both steps run in the background, so a merge or a
new task answers at once: the merge moves `target/` aside into `<repo>/.cache/reclaim/` and the
feed's `build:reclaimed` says, when the harvest is done, how much went into the cache
(`cached_bytes`); a new task's `target/` is built in `<repo>/.cache/seed/` and moved into the
workspace in one rename (`build:seeded`), unless the agent's first build made a `target/` first.
While they run, `av disk` shows those dirs under the build cache; a daemon that stops in between
removes them when it starts. Hard links need the
workspaces and the cache on one filesystem (they are: both under `ws_root`); where linking fails,
a workspace simply starts without them. `AV_BUILD_CACHE=off` keeps and seeds nothing.

**Your `CARGO_TARGET_DIR` does not apply inside a task.** A target dir shared by two workspaces
of one repo makes `cargo test` in one run the other's test binary. So in a cargo workspace,
`av validate` and `integrations/generic/av-wrap.sh` set `CARGO_TARGET_DIR` to the workspace's
own `target/` and unset `CARGO_BUILD_TARGET_DIR`, which also overrides `build.target-dir` in
`~/.cargo/config.toml`. When your environment sets either variable, the Claude Code session-start
hook adds shell code to `$CLAUDE_ENV_FILE` that does the same before every Bash command, for the
task workspace the command runs in. That file is then no longer plain assignments, so Claude
Code's PowerShell tool gets none of its variables (`AV_SCRATCH` included). An explicit `--target-dir` in a command still wins; what it leaves
outside the workspace is not reclaimed (the footprint reports it).

To spend less per task still: scope a required convention's check by its glob, so a change to
`docs/` does not pay for the full workspace build. `av convention add --repo scholion --scope
'crates/**' --required --check 'cargo test --workspace' 'Tests pass'` runs only when a changed
path matches `crates/**`. `av convention add` says so when a required check's scope matches every path.

A check command may say `{crates}` where cargo takes packages: `--check 'cargo test {crates}'`.
`av validate` replaces it with `-p a -p b`, the workspace members that hold a changed path plus
every member depending on one of them through a path dependency of any kind (normal, dev,
build), read from `cargo metadata --no-deps --offline` in the workspace. When it cannot place a
change (a path outside every member, such as the root `Cargo.toml`, `Cargo.lock`,
`rust-toolchain.toml`, `.cargo/` or a fixture dir a test reads through `../..`), or cargo is
missing, it is `--workspace`. The check line prints the command as it ran (for a change in `scholion-pack`: `cargo test
-p scholion-bench -p scholion-cli -p scholion-daemon -p scholion-pack`) and the validation record
keeps it, so a reviewer sees what a green check covered. Approval counts a result for such a check only
when its `command` was expanded (run as written, `cargo test {crates}` filters every test out
and passes), so an older `av` cannot pass it. A package at the workspace root owns only its
`src/`, `tests/`, `benches/`, `examples/` and `build.rs`. Choosing packages also unifies features over
those packages only.

**What a session leaves outside its workspace.** The Claude Code hooks (and Grok's, which run the
same `av hook`) take a snapshot at `SessionStart`: under `$HOME`, the temp dir and
`AV_FOOTPRINT_ROOTS` (a path list), they measure every directory named `target`, `node_modules`,
`.venv`, `venv`, `dist`, `build`, `.cache`, `worktrees`, `.gradle` or `.next` down to four levels.
`AV_WS_ROOT` and `$AV_HOME` are left out. `Stop` scans again and diffs. When the session added at
least `AV_FOOTPRINT_WARN_MB` (default 512), it blocks the stop once per session. The agent is told
where, and asked to build inside the workspace or the repo's cache, move it to `$AV_SCRATCH`, or
remove it, and to record where anything kept lives. Every addition is reported to the daemon, and
`av disk` lists it under "outside any workspace" with the agent, task, time and largest paths. The
daemon only shows these paths and never removes them. A scan stops at `AV_FOOTPRINT_SECS`
(default 20) and is then marked `partial`, so some directories may be missing or counted as new.
`AV_FOOTPRINT=off` turns all of this off.

**Containers are footprint.** Agent sessions start containers through docker (builds, test
databases, tool sandboxes) and leave them, and a directory scan does not see them. The
convention: everything a session starts through docker carries the labels `av.task=<task id>`
and `av.agent=<agent>`. In a task, `SessionStart` appends
`export AV_DOCKER_LABELS='--label av.task=… --label av.agent=…'` to `$CLAUDE_ENV_FILE` and, when
`docker` is on `PATH`, tells the agent in one line to add `$AV_DOCKER_LABELS` to every
`docker run` and `docker build` (`docker compose` takes the same through `labels:` in the file).
With `docker` on `PATH`, `Stop` lists what carries the task's label (`docker ps -a --size`,
`docker images` plus `docker image inspect`, `docker volume ls`, all with
`--filter label=av.task=<id>`, read-only, within `AV_FOOTPRINT_SECS`) and sends it with the
footprint as a `containers` section (`{containers: [{id, name, state, size}], images: [{id,
bytes, tags}], volumes: [{name}]}`); `av disk` shows it under the session. When more than
`AV_FOOTPRINT_CONTAINERS` (default 0: any) of those containers are still running, the stop is
blocked once per session with their names and `docker stop`; av never stops a container.
A container started without the labels is caught by a second net. With `docker` on `PATH`,
`SessionStart` also keeps the id of every container then in its snapshot (`container_ids`, from
`docker ps -aq --no-trunc`, read-only, within the same `AV_FOOTPRINT_SECS`), and in a task `Stop`
lists every container (`docker ps -a --no-trunc` with id, name, state and the `av.task` label):
the ones not in the snapshot that carry no `av.task` label go into the same `containers` section
as `unlabelled: [{id, name, state}]`, and `av disk` shows them under the session. Running ones
count toward `AV_FOOTPRINT_CONTAINERS` together with the task's labelled ones; the block names
them, marked `(unlabelled)`, and says to add `$AV_DOCKER_LABELS` next time. A container labelled
for another task is not this net's, and one present at session start is never reported. Without
the ids (docker was not on `PATH`, or did not answer, at session start) nothing is reported as
unlabelled.
`av gc --containers` (the CLI runs docker, the daemon never does) lists everything labelled
`av.task` on the host, asks the daemon each task's status (`GET /v1/tasks/{id}`) and, for the
merged and abandoned ones, removes stopped containers, then dangling images, then volumes, one
docker call each, printing what went; running containers are listed and left running, and what
belongs to open tasks, or to tasks the daemon does not know, is only counted. `--dry-run` lists
the plan and `--json` carries it as `containers`. Without `docker` on `PATH` the flag says so
on stderr and the rest of `gc` runs. Unlabelled containers are never touched, not even the ones
a stop reported as `unlabelled`: nothing says whose they are, so the block tells the agent to
remove its own once stopped.

**`$AV_SCRATCH`.** In a task, `SessionStart` creates `<ws_root>/<repo>/.scratch/<slug>` and
appends `export AV_SCRATCH=…` to Claude Code's `$CLAUDE_ENV_FILE`, so every later Bash command
of the session has it. The pack ends with `Temporary files: $AV_SCRATCH (removed with the task).`
It is removed with the workspace.

**What av does not touch.** Claude Code's own data (`~/.claude`) is swept by Claude Code itself
(`cleanupPeriodDays`, default 30 days). `scholion-bench run` removes each session's copy after verify unless
`--keep-work` is given (see [bench/README.md](../bench/README.md)). The jj store and
Postgres are not garbage-collected. `av up` moves a `daemon.log` over 50 MB to `daemon.log.1`.

## Test

```bash
cargo test --workspace   # starts a throwaway user-space Postgres when no database is given
# or against the container:
AV_TEST_DATABASE_URL=postgres://av:av-dev-only@127.0.0.1:5432/postgres cargo test --workspace
scripts/smoke.sh         # end-to-end against the Docker stack (needs Docker)
```

## Security

Set `AV_TOKEN` to require `Authorization: Bearer <token>` on every route but `/healthz` and `/ui` (compared in constant time; `/ui` is a static page with no data in it);
the `av` CLI and its hooks send `$AV_TOKEN`. Without it the daemon is open, which is why the compose file publishes it
(and Postgres, Qdrant) on `127.0.0.1` only; do not expose these ports either way. Agent names (`--agent`, `AV_AGENT`)
are self-asserted: "the reviewer is not the owner", "only the owner hands off" and claim fencing prevent accidents
between cooperating agents, not impersonation. Repository names are restricted to a single path component,
`scholion repo add` only accepts an existing local directory, and backup remotes that look like git options are refused.
Postgres credentials in the compose file are development-only.

## Post-merge checks

There is no merge queue (the Scion model): main is checked after each merge, not before. Set the
repo's check, then run a runner on the machine where the code should be tested:

```bash
scholion repo settings myrepo --check "python -m pytest -q"
scholion runner --repo myrepo --trust        # checks main, then every merge as it lands
scholion health --repo myrepo                # green, or red since which merge and its task
scholion log --repo myrepo                   # ✓ passed · ✗ failed · · unchecked
scholion bisect --repo myrepo --trust        # which merge broke main, and its task
scholion revert <task> --reason "..."        # take one merge out of main, even an older one
scholion why app/util.py:42 --repo myrepo    # the task, goal and decisions behind a line
```

For a line of imported history, `scholion why` prints the record mined from the commit it blames,
if the repo's history was mined (see [Cold start from git history](#cold-start-from-git-history)).

`scholion why` reads derived data in the Scion style: per path, a blame computed from each commit's
parent blame and cached by commit id, so it is computed once and never needs invalidating.

### Records on the pull request

```bash
av notes                        # the task's records as a Markdown block (this workspace's task)
av notes --pr                   # post it on the current branch's pull request, or update it
av notes demo/atomic-save --pr 123 --gh-repo owner/name
```

`av notes [TASK]` renders a task's decisions, dead ends, notes, review findings, handoffs and
checks, and what other tasks recorded about the files it changes (`records/at` for the first 20,
with no session, so nothing counts as sent), between `<!-- scholion:notes v1 task=<id> -->` and
`<!-- /scholion:notes -->`. It only reads `/v1`. `TASK` is an id, a slug (`--repo` when two repos
share one) or `repo/slug`; without it, `--task` or the nearest `.av-task`. `--format json` adds the
task id, record ids and files. `--pr [N]` posts the block as a comment through `gh` (`AV_GH` names
another program), or edits the comment that already holds this task's block; an unchanged block is
not sent. Exit codes: `0` done (or up to date), `1` daemon, task or `gh` failure, `2` usage. The
daemon runs on the developer's machine, out of a GitHub-hosted runner's reach, so the CLI posts;
`integrations/github/` has the marker format and a workflow for a self-hosted runner that can
reach the daemon. Before a merge `av why` has nothing to blame on main, so the block's records on
the changed files are the pre-merge answer.

The daemon never runs commands. `scholion runner` fetches each new main through the read API (no
checkout), runs the command, and posts the result. Because that command comes from the daemon's
database, the runner refuses to start without `--trust`. While main is red, every context pack
opens with a warning naming the culprit merge.

## Agent integrations

- **MCP** (any MCP client): `claude mcp add --transport http av http://localhost:7420/mcp` (add
  `--header "Authorization: Bearer $AV_TOKEN"` when a token is set). Twenty-four tools: `start_task`, `checkpoint`,
  `record`, `get_context`, `expand`, `read_symbol`, `search_history`, `handoff`, `claim_task`, `submit`,
  `review_task`, `post_review`, `merge`, `undo`, `status`, `validate` (an attestation: the agent runs the
  command, the daemon records its exit code), `sync`, `abandon`, `list_conventions`, `why` (the task, goal and decisions behind a line of
  main), and the workspace-free reads `read_file`, `list_files`, `diff` and `log` (any commit: `main`, `task:<slug>` or a
  commit id). Each MCP connection is its own ledger session and
  remembers the task it started or claimed (and the claim generation its writes carry); the agent name is the
  client's name, or an `X-Av-Agent` header. Checked with Claude Code 2.1.308: `claude mcp list` shows `av`
  connected against a locally started daemon.
- **Claude Code**: `integrations/claude-code/install.sh` (needs `jq`) copies three hooks to `~/.claude/av-hooks/` and
  merges them into `~/.claude/settings.json` (previous file kept as `settings.json.bak`; running it again replaces
  them). `SessionStart` prints an 800-token pack for the task of the session's directory (a fresh pack session at
  start-up, after compaction or after `/clear`, so a new Claude Code process gets everything even when `av next` already
  printed the pack under the task's session; only a resumed session keeps its ledger). Its code items show path, lines and signature, not bodies (`--code full` adds them):
  Claude Code must read a file before it edits it, so a body in the pack is paid for twice (bench/README.md,
  "Haiku runs"). `--no-map` leaves out the repo layout and `--max-code N` keeps the N best code items (each
  item named is a file the agent may open; `GET /v1/tasks/{id}/context?map=false&max_code=N`). `PreToolUse` on `Read|Edit|MultiEdit` adds what other tasks recorded about that file (decisions, dead
  ends, notes; at most 400 tokens, each record once per session and never one the pack already sent; once per
  session it ends with how a wrong one is corrected, `av record --supersedes <id>`) as
  `additionalContext`, so the why arrives when the file is opened instead of up front
  (`GET /v1/tasks/{id}/records/at?path=…&session=…`). `PostToolUse` on `Edit|Write|MultiEdit` checkpoints at most
  every 2 s. `Stop` checkpoints and, when changed symbols have no record of the task anchored to them (what
  `submit` would list, `GET /v1/tasks/{id}/unrecorded`), blocks the stop once per agent session and asks for the
  decisions and dead ends behind them. All four are silent outside a task and never fail the agent
  (`scholion hook session-start`, `scholion hook pre-read`, `scholion hook post-edit`, `scholion hook stop`).

  **Disk.** In a task, `SessionStart` also creates a scratch dir and exports it to the session's Bash
  commands as `$AV_SCRATCH` (through `$CLAUDE_ENV_FILE`); it is removed with the task. Task or not, it
  measures the build, dependency and cache dirs under `$HOME` and the temp dir, and `Stop` measures
  them again: a session that added `AV_FOOTPRINT_WARN_MB` (default 512) or more outside any task
  workspace is stopped once and told where. In a task, `SessionStart` also exports
  `$AV_DOCKER_LABELS` (`--label av.task=… --label av.agent=…`) for the session's `docker run` and
  `docker build`, and with `docker` on `PATH` `Stop` reports what carries the task's label and,
  as `unlabelled`, the containers started since `SessionStart` without any `av.task` label, and
  blocks once while more than `AV_FOOTPRINT_CONTAINERS` (default 0) of both still run, never
  stopping them (`av gc --containers` removes the task's stopped ones once it is done, never an
  unlabelled one).
  `AV_FOOTPRINT=off` turns all of that off. See
  [Disk](#disk-what-agents-create-and-what-av-reclaims).

  **Keep agents busy.** After that check, `Stop` asks the daemon for this agent's next work (a dry run of
  `POST /v1/backlog/next`, the call `av watch` polls). If something fits (queued work, a review, a handoff or an
  abandoned task), it blocks the stop with `av: <what is waiting> - call start_task with no title`, so the agent
  carries on instead of idling. If nothing fits yet it keeps looking every 15 s for up to `AV_HOOK_WAIT_SECS`
  (default 600; `0` turns this off), which costs no model tokens, then lets the stop through. It continues one
  agent session at most `AV_HOOK_MAX_CONTINUES` times (default 20, counted in `.av-task`), and a daemon that
  does not answer allows the stop at once. `install.sh` sets the hook's timeout to `AV_HOOK_WAIT_SECS` + 60 s,
  so run it again after changing the wait: `AV_HOOK_WAIT_SECS=120 integrations/claude-code/install.sh`.
- **Any `mcpServers` JSON client** (Muse, Windsurf, Cline, …): `av --agent NAME init --for mcp-json --path FILE`
  merges `mcpServers.av` into that file (`type` `http`, the `/mcp` URL, `X-Av-Agent` set to `NAME`) and
  keeps the rest. With `AV_TOKEN` set, the header is `Bearer ${AV_TOKEN}`, which the client expands.
  A client that already has a writer above does not need this.
- **Codex, Grok Build, Gemini CLI, Cursor Agent**: `av init --for codex|grok|gemini|cursor` merges the
  MCP server into `$CODEX_HOME/config.toml` (default `~/.codex`), `~/.grok/config.toml`,
  `~/.gemini/settings.json` or `~/.cursor/mcp.json`, keeps everything else in the file and the
  previous one as `*.bak`. With `AV_TOKEN` set, the file only names the variable (Codex
  `bearer_token_env_var`, Grok and Gemini `${AV_TOKEN}`, Cursor `${env:AV_TOKEN}` in a header),
  never the token. All four also get a fixed `X-Av-Agent` header (`codex`, `grok`, `gemini`, `cursor`). Codex, Gemini and Cursor get no hooks: they call `start_task`/`claim_task` and
  `get_context` themselves. Grok gets `SessionStart` and `Stop` in `~/.grok/hooks/av.json` unless
  `~/.claude/settings.json` already loads them (Grok runs both; see `integrations/grok/README.md`
  for the 8-continuation cap and folder trust). The instructions to give an agent are in
  `integrations/<agent>/README.md`. All of them
  share one memory, so a dead end Claude recorded is in the pack Codex gets.
- **Muse Code**: installed and its headless mode checked, but its MCP settings file was not (needs a
  login); see `integrations/muse/README.md` for what works today (`av next` with `av-wrap.sh`).
- **Across vendors**: `scripts/relay-demo.sh` shows one memory shared by different agents. Claude
  Code tries the obvious fix on a small repo, records why it failed and hands off; Codex claims the
  task, and its first pack leads with that dead end (`--first`/`--second` pick other agents;
  `--scripted` runs it with `av` commands in place of the agents, for free). The contract test
  `agents_wired_by_init_can_start_record_checkpoint_and_submit` (`crates/scholion-cli/tests/cli.rs`)
  reads each config `av init --for` wrote, the way that CLI does, and drives the MCP server through
  it with the token taken from the environment.
- **Any other agent**: `integrations/generic/av-wrap.sh <command…>` writes the pack to `$AV_CONTEXT_FILE`, runs the
  command and checkpoints.
- **GitHub pull requests**: `av notes --pr` posts the task's records as one comment, updated in place
  (see [Records on the pull request](#records-on-the-pull-request) and `integrations/github/README.md`).
  `integrations/README.md` lists every integration.
- **Maintenance**: `POST /v1/maintenance/prune` deletes the ledgers of sessions idle for 30 days and parse-cache
  entries no index scope uses. A task's index scope is dropped when it merges or is abandoned (re-indexed on undo).

## Data and shutdown

Named volumes `av-store` (jj repositories), `av-pg` (Postgres), `av-qdrant`, `av-tei` (model cache). `docker compose -f
deploy/docker-compose.yml down` stops the stack; `scripts/smoke.sh --down` also deletes the volumes.
The compose project is called `scholion`. A stack started before the rename ran as project
`agentvcs`, so its volumes are `agentvcs_*`: pass `-p agentvcs` to keep using them.
