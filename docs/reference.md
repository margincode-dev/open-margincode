# Scholion reference

The full reference: install options, commands, configuration, design, architecture and
measurements. For the short version, see the [README](../README.md).

*σχόλιον (scholion): a note written in the margin of a manuscript. Scholion keeps notes in the
margins of your code.*

Context-first version control for coding agents: tasks in isolated jj workspaces, decisions and
dead ends recorded next to the code, and a token-budgeted context pack for any agent that picks the
work up.

**[Watch a recorded run](https://openevaluations.si/demo)**: Claude records a decision and a dead
end, hands off; Codex picks the task up from a context pack instead of re-reading the repo; Gemini
reviews. On a small Python repo, Claude Code needed **12.9 % fewer turns per task at the same
cost** ([benchmark](../bench/results/2026-10-27-haiku/), including where it did not help).

[![A recorded run: Codex's context pack leads with Claude's decision and dead end](img/demo-replay.png)](https://openevaluations.si/demo)

**New in 0.0.2: one backlog for all your agents.** Post work with tags; each agent (Claude Code,
Codex, Cursor, Grok, Gemini) takes what fits it, another agent reviews it, and the reviewer merges
what it approves. `examples/routing-smoke/deepseek-demo.sh` runs that loop with two Claude Code
agents on DeepSeek Flash for about $0.03; it shows the loop working, not token savings. What
changed: [CHANGELOG.md](../CHANGELOG.md).

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

**Commercial** (agentvcs, hosted) is not available. The plan, and only the plan, is a service that
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
intent, decisions and dead ends, so every agent re-derives them. agentvcs stores the "why" next to
the code and hands any agent a deterministic, token-budgeted context pack built from it.

### Concepts

- **Task**: one unit of agent work, replacing branch + pull request. Each task gets its own jj
  workspace on disk (`~/ws/<repo>/<task>/`), branched from trunk. The agent edits files there with
  its usual tools; `scholion checkpoint` snapshots them.
- **Record**: a short, typed note attached to a task: `intent` (the goal), `decision`, `attempt`
  (a dead end and why), `note`, `handoff`, `review`. Summaries are ≤ 200 characters, and records
  are append-only: a newer record *supersedes* an older one instead of editing it.
- **Anchor**: a record's link to code, as a path or `path:Symbol`. Records resurface whenever a
  later task touches that code. The anchored code's hash is stored when the record is written, so
  a pack marks a claim `(changed since recorded)` once that code has moved on.
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
record covers.

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
| Task workspaces (`AV_WS_ROOT`) | Each task's working copy | Bind-mounted so agents edit on the host; clients are given host paths. |
| Postgres | Tasks, records, anchors, conventions, ledger, receipts, code index, outbox | Source of truth. |
| Qdrant | Vectors for records, conventions and code chunks | Derived; filled from the outbox. |

### Main paths

- **Start a task**: create the jj workspace at trunk, then copy `main`'s index rows into the task's
  scope inside Postgres. If `main`'s index is not recorded at that exact commit, the task is
  indexed from the files instead. Then write the intent record and return the first pack.
- **Checkpoint**: snapshot the workspace (a new jj commit), diff it against the task's base,
  re-index only the changed files, store the new head and the touched paths.
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
| `AV_HOME` | `~/.scholion` | daemon data: `pg/` (Postgres), `store/` (jj repositories), `daemon.log`, `daemon.pid` |
| `AV_WS_ROOT` | `~/ws` | where task workspaces appear (agents edit files here) |
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
`AV_QDRANT_URL` is set and an embedder is found: a host `ollama` CLI with a `qwen3-embedding` model
(the default when running the daemon on the host), otherwise `AV_EMBED_URL` (TEI in compose). The
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
| `AV_EMBED_PROVIDER` | auto | `ollama` or `api`. Auto uses Ollama when the CLI has a `qwen3-embedding` model. |
| `AV_OLLAMA_URL` | `http://127.0.0.1:11434` | Ollama server. The model tag comes from `ollama list` (`qwen3-embedding` by default). |
| `AV_PG_MAX_CONNECTIONS` | `16` | daemon's Postgres pool size. Raise it when many agents work at once. |
| `AV_MAX_CONCURRENT_STARTS` | a quarter of the pool | task starts that copy the index at once; the others wait their turn instead of failing. |
| `AV_LEASE_SECS` | `1800` | how long an agent holds an open task without writing to it; after that another agent may `scholion claim` it. |
| `AV_TOKEN` | (none) | shared bearer token for every route but `/healthz` and `/ui`; the CLI sends `$AV_TOKEN`. Unset = no auth. |
| `AV_BACKUP_REMOTES` | (none) | `repo=url` pairs, whitespace-separated: main is pushed there after every merge and undo (`scholion repo backup` sets one at runtime). |

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

## Use

```bash
scholion repo add pantry /import/pantry            # path as seen inside the container
scholion repo list                                 # every imported repo, its main and source
scholion task start --repo pantry --title "Atomic save" --goal "write pantry.json atomically so a crash cannot truncate it"
cd ~/ws/pantry/atomic-save                   # printed as `workspace:`
scholion checkpoint                                # snapshot + refresh the index
scholion record --kind attempt --summary "tried os.rename; fails on Windows if the file exists" --anchor pantry/store.py
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

### Many agents at once

Run as many agents as you like on one daemon. There is no seat cap: each distinct agent
that calls `next` gets the work that fits it.

```bash
av init --for claude                 # and --for codex, grok, gemini, cursor
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
tags cover what it needs. `av watch`, once per agent, starts that CLI only when something
fits it and sets `AV_AGENT` for the command.

The dedicated writers put a fixed `X-Av-Agent` header (`grok`, `gemini`, `cursor`).
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
id. The dashboard lists the teams (skill, cost, and open tasks against the limit), the queue and who each item is reserved for. Design:
`docs/superpowers/specs/2026-10-31-agent-routing-design.md`; how to measure whether it pays:
`docs/superpowers/plans/2026-10-31-plan-8-grok-evaluation.md` (not yet run: no claim is made).
To try it with real agents, run `examples/routing-smoke/setup.sh`. It sets up a small sample
project with profiles for Claude Code and Grok and a
tagged backlog. Each table in its README says which agent should take which task. With no
subscription, `examples/routing-smoke/deepseek-demo.sh` runs two Claude Code agents on DeepSeek
Flash through the same loop on its own daemon (the run checked for 0.0.2).

### Dashboard

Open `http://localhost:7420/ui` to watch agents work: tasks by status, a Teams panel (each profile's
skills and cost, how many open tasks it holds, and a full chip at its limit), the backlog, a live
activity feed (the `/v1/events` long-poll), and for each task its records, changed files, claim and
review state. "Build context pack" shows exactly what the next agent would receive at a given budget.
The page is read-only and built into the daemon (no Node, no build step); with `AV_TOKEN` set it asks
for the token once per browser tab. It reads `GET /v1/tasks` (`?repo=`, `?status=`, `?limit=`),
`GET /v1/tasks/{id}/records` and `GET /v1/agents`.

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

`scholion why` reads derived data in the Scion style: per path, a blame computed from each commit's
parent blame and cached by commit id, so it is computed once and never needs invalidating.

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
  them). `SessionStart` prints an 800-token pack for the task of the session's directory (a fresh session after
  compaction or `/clear`). Its code items show path, lines and signature, not bodies (`--code full` adds them):
  Claude Code must read a file before it edits it, so a body in the pack is paid for twice (bench/README.md,
  "Haiku runs"). `--no-map` leaves out the repo layout and `--max-code N` keeps the N best code items (each
  item named is a file the agent may open; `GET /v1/tasks/{id}/context?map=false&max_code=N`). `PreToolUse` on `Read|Edit|MultiEdit` adds what other tasks recorded about that file (decisions, dead
  ends, notes; at most 400 tokens, each record once per session and never one the pack already sent) as
  `additionalContext`, so the why arrives when the file is opened instead of up front
  (`GET /v1/tasks/{id}/records/at?path=…&session=…`). `PostToolUse` on `Edit|Write|MultiEdit` checkpoints at most
  every 2 s. `Stop` checkpoints and, when changed symbols have no record of the task anchored to them (what
  `submit` would list, `GET /v1/tasks/{id}/unrecorded`), blocks the stop once per agent session and asks for the
  decisions and dead ends behind them. All four are silent outside a task and never fail the agent
  (`scholion hook session-start`, `scholion hook pre-read`, `scholion hook post-edit`, `scholion hook stop`).

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
  never the token. Codex, Gemini and Cursor get no hooks: they call `start_task`/`claim_task` and
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
- **Maintenance**: `POST /v1/maintenance/prune` deletes the ledgers of sessions idle for 30 days and parse-cache
  entries no index scope uses. A task's index scope is dropped when it merges or is abandoned (re-indexed on undo).

## Data and shutdown

Named volumes `av-store` (jj repositories), `av-pg` (Postgres), `av-qdrant`, `av-tei` (model cache). `docker compose -f
deploy/docker-compose.yml down` stops the stack; `scripts/smoke.sh --down` also deletes the volumes.
The compose project is called `scholion`. A stack started before the rename ran as project
`agentvcs`, so its volumes are `agentvcs_*`: pass `-p agentvcs` to keep using them.
