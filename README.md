# Scholion

*σχόλιον (scholion): a note written in the margin of a manuscript. Scholion keeps notes in the
margins of your code.*

Context-first version control for coding agents: tasks in isolated jj workspaces, decisions and
dead ends recorded next to the code, and a token-budgeted context pack for any agent that picks the
work up.

[![CI](https://github.com/vinhqbui/scholion/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/vinhqbui/scholion/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Release](https://img.shields.io/github/v/release/vinhqbui/scholion?include_prereleases)](https://github.com/vinhqbui/scholion/releases)
[![MCP server](https://img.shields.io/badge/MCP-server-8A2BE2)](#agent-integrations)

**[Watch a recorded run](https://openevaluations.si/demo)**: Claude records a decision and a dead
end, hands off; Codex picks the task up from a context pack instead of re-reading the repo; Gemini
reviews. On a small Python repo, Claude Code needed **12.9 % fewer turns per task at the same
cost** ([benchmark](bench/results/2026-10-27-haiku/), including where it did not help).

[![A recorded run: Codex's context pack leads with Claude's decision and dead end](docs/img/demo-replay.png)](https://openevaluations.si/demo)

**Try it** (Docker; the first build compiles the daemon, so give it a few minutes):

```bash
export UID GID=$(id -g) AV_IMPORT_ROOT=~/projects                # repos to import live here
docker compose -f deploy/docker-compose.yml up -d --build      # daemon on 127.0.0.1:7420
claude mcp add --transport http av http://localhost:7420/mcp   # or any MCP client
av repo add myrepo /import/myrepo                              # path inside the container
```

Then watch your agents at `http://localhost:7420/ui`. Prebuilt `av` CLI binaries (Linux x86-64
and arm64, macOS arm64) are on the [releases page](https://github.com/vinhqbui/scholion/releases),
or install them into `~/.local/bin` with:

```bash
curl -fsSL https://raw.githubusercontent.com/vinhqbui/scholion/main/scripts/install.sh | sh
```

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
post-merge checks, revert of any merge, bisect and `why` blame.

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

```
 HOST                                  DOCKER (deploy/docker-compose.yml, all on 127.0.0.1)
 ┌───────────────────────────────┐     ┌───────────────────────────────────────────────────┐
 │ agents (Claude Code, Codex …) │     │ scholion-daemon :7420  (axum, owns all state)     │
 │   edit files in               │ HTTP│  ├─ api.rs / flow.rs   routes, lifecycle ops      │
 │   ~/ws/<repo>/<task>/  ◀──────┼bind─┼─▶├─ scholion-vcs   (jj-lib)  ─▶ volume av-store   │
 │                               │mount│  ├─ scholion-store (sqlx)    ─▶ postgres:16       │
 │ scholion / av CLI ────────────┼JSON▶│  ├─ scholion-index (tree-sitter) ─▶ postgres      │
 │   finds its task via .av-task │     │  ├─ scholion-pack  (context packer)               │
 └───────────────────────────────┘     │  └─ outbox worker ─▶ embedder (TEI / Ollama)      │
                                       │                   ─▶ qdrant                       │
                                       └───────────────────────────────────────────────────┘
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
- Starting a task cleans up after itself: if the workspace cannot be created, the task is marked
  abandoned rather than left open.

### Testing and CI

`cargo test --workspace` runs everything against Postgres (see [Test](#test)). GitHub Actions
(`.github/workflows/ci.yml`) runs fmt, clippy with `-D warnings`, and all tests against Postgres
and Qdrant service containers on every pull request and every push to `main`. CI's Postgres
sorts text with an English locale on purpose, which catches orderings that depend on the
database's locale.

## Run

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
scholion repo add bench-repo /import/bench-repo      # path as seen inside the container
scholion task start --repo bench-repo --title "Add retry" --goal "retry embedder calls with backoff"
cd ~/ws/bench-repo/add-retry                  # printed as `workspace:`
scholion checkpoint                                # snapshot + refresh the index
scholion record --kind attempt --summary "tried tenacity; not installed" --anchor shared/text.py
scholion context                                   # 800 tokens of what an agent needs; --budget N for more
scholion context --fresh                           # resend everything (e.g. after context compaction)
scholion submit                                    # to review (needs a decision, attempt or note)
scholion handoff --done "retries added" --remaining "tests for the 429 path"   # prints the bundle
AV_AGENT=codex scholion claim <task-id>            # another agent takes over; rewrites .av-task
scholion bundle --pr                               # review bundle: tests at head, risks, checklist
scholion search "rate limit"                       # full-text over the repo's records
scholion log                                       # main's history, newest first
scholion abandon                                   # end a task for good (owner only)
```

### Dashboard

Open `http://localhost:7420/ui` to watch agents work: tasks by status, a live activity feed (the
`/v1/events` long-poll), and for each task its records, changed files, claim and review state.
"Build context pack" shows exactly what the next agent would receive at a given budget. The page
is read-only and built into the daemon (no Node, no build step); with `AV_TOKEN` set it asks for the
token once per browser tab. It reads two listing routes added for it: `GET /v1/tasks`
(`?repo=`, `?status=`, `?limit=`) and `GET /v1/tasks/{id}/records`.

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
- **Any other agent**: `integrations/generic/av-wrap.sh <command…>` writes the pack to `$AV_CONTEXT_FILE`, runs the
  command and checkpoints.
- **Maintenance**: `POST /v1/maintenance/prune` deletes the ledgers of sessions idle for 30 days and parse-cache
  entries no index scope uses. A task's index scope is dropped when it merges or is abandoned (re-indexed on undo).

## Data and shutdown

Named volumes `av-store` (jj repositories), `av-pg` (Postgres), `av-qdrant`, `av-tei` (model cache). `docker compose -f
deploy/docker-compose.yml down` stops the stack; `scripts/smoke.sh --down` also deletes the volumes.
The compose project is called `scholion`. A stack started before the rename ran as project
`agentvcs`, so its volumes are `agentvcs_*`: pass `-p agentvcs` to keep using them.
