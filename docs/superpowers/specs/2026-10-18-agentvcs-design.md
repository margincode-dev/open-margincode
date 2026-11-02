# agentvcs (`av`) — Design Spec (v1: context-first)

Status: approved design, 2026-10-18. Next: implementation plan.


## Context

Agentic coding outpaces git. Three pains: (1) storage churn from many parallel agents, (2) **context budget** — agents burn tokens on cold-start orientation, rediscovering history (what was tried/decided/rejected), picking up others' work, and loading stale/irrelevant context, (3) handoff/PRs that show *what* changed but not *why*.

Root cause of (2) and (3): git records snapshots but discards intent, decisions and dead ends, so agents re-derive them every session. agentvcs makes the "why" a first-class, versioned, retrievable object and serves agents a token-budgeted context pack.

**Agreed with user**
- For the user's own agent fleet: Claude Code + other CLI agents (Codex, Aider…), on one machine (WSL), 2–10 parallel agents.
- Replaces git for day-to-day agent work; pushes to a git remote periodically as **backup only**.
- Rust. VCS engine built on **jj-lib** (Jujutsu). Server runs in **Docker**; agents run on the host.
- Storage: **PostgreSQL + Qdrant** (no SQLite), all free/OSS. Local OSS embedding model.
- First sub-project = context budget. Storage-cost work is a later sub-project.

**Success criterion (measured):** on ~10 replayed real tasks from two private repos, `av`-assisted sessions use materially fewer tokens before the first relevant edit and in total, with no drop in task success and fewer repeated dead ends, vs. baseline Claude Code + git.

## Architecture

```
 HOST (WSL)                                   DOCKER compose stack
 ┌──────────────────────────┐                 ┌──────────────────────────────────────────┐
 │ Claude Code / Codex / …  │──MCP (HTTP)────▶│ daemon (Rust)                            │
 │ `av` CLI (static binary) │──HTTP/JSON─────▶│  ├ VCS engine (jj-lib)  → /store/jj vol  │
 │ ~/ws/<repo>/<task>/  ◀───┼──bind mount────▶│  ├ Intent store         → postgres       │
 └──────────────────────────┘                 │  ├ Code index (tree-sitter) → postgres   │
                                              │  ├ Embedding/outbox worker → qdrant      │
                                              │  ├ Context packer                        │
                                              │  └ MCP + HTTP API                        │
                                              │ postgres:16 · qdrant · text-embeddings-  │
                                              │ inference (open model, CPU-capable)      │
                                              │ ──▶ git remote (backup push)             │
                                              └──────────────────────────────────────────┘
```

Units (each independently testable, communicating through Rust traits):

1. **VCS engine** — the only code touching jj-lib (pinned version). Repos, one jj workspace per task, snapshot, squash, rebase/merge, op log (undo), git push for backup.
2. **Intent store** — Postgres: tasks, records, anchors, conventions.
3. **Code index** — tree-sitter (Rust, TypeScript, Python in v1): symbols, reference graph, per-file skeletons; incremental by content hash on each snapshot. Stored in Postgres; symbol-level code chunks embedded into Qdrant.
4. **Context packer** — deterministic (no LLM in hot path) ranking + budgeted rendering.
5. **Interfaces** — MCP server (streamable HTTP), HTTP/JSON API, thin `av` CLI.

## Data model & storage

| Store | Holds |
|---|---|
| jj repo (volume `/store/jj`) | File content, changes, operation log |
| PostgreSQL (source of truth, `sqlx` migrations) | `tasks`, `records`, `anchors`, `conventions`, `symbols`, `symbol_refs`, `sessions` (ledger), `outbox` |
| Qdrant (derived, rebuildable via `av reindex`) | Collections `records`, `conventions`, `code_chunks`; payload `repo, task_id, record_id, symbol_id, path, change_id, superseded` |

- **Task**: `id, repo, title, goal, status (open|handed_off|in_review|conflicted|merged|abandoned), owner_agent, base_change, head_change, workspace_path`. Replaces branch + PR.
- **Record**: `id, task_id, kind (intent|decision|attempt|note|handoff|review), summary (≤200 chars), body, change_id?, supersedes?, author_agent, created_at`. Append-only; a newer record may *supersede* an older one (hidden from packs, still queryable).
- **Anchor**: `record_id, path, symbol?` → resolved to `symbol_id` via the index, so knowledge resurfaces when that code is touched.
- **Convention**: `id, repo, scope (path glob), text, source` — served only when scope matches the task's files.
- **Consistency**: Postgres write + `outbox` row in one transaction; worker embeds and upserts to Qdrant. Qdrant/embedder outage → packs degrade to structural + Postgres full-text and report `degraded: semantic`; outbox catches up.
- **Snapshots**: on `checkpoint`/`submit`/`handoff` + debounced file-watch → jj op log. On merge, task changes are squashed to a few meaningful changes; only those are pushed to the git backup.

## Context packer

`get_context(task_id, budget_tokens, focus?)`:

1. **Seeds**: task goal/intent, changed files+symbols (jj diff vs `base_change`), `focus` (query/path/symbol).
2. **Candidates** (parallel): structural (1–2 hop ref-graph neighbors), anchored records (+ parent tasks), semantic (Qdrant over records/code_chunks/conventions, filtered by repo and `superseded=false`), live work (other open tasks touching overlapping symbols → 1-line conflict warnings).
3. **Score** = weighted(structural distance, semantic similarity, recency, kind priority); `attempt` records on seed symbols rank highest.
4. **Budget allocation** with spillover: map 10% · task/intent 10% · history 20% · code 50% · conventions+live work 10%.
5. **Progressive tiers** per item: handle (1 line + id) → summary → full. Fill top items at richest fitting tier; rest as handles. Agent pulls more via `expand(id)` / `read_symbol(id)`.
6. **Session ledger**: track what each agent session already received; later packs reference instead of resend.

Output: Markdown sections `Task / Map / Decisions & dead ends / Code / Conventions / Live work`, plus token count and omitted handles. Token counts = tiktoken-based estimate × per-agent-family factor, 10% margin. Target < 300 ms warm.

## Agent workflow, handoff, PR

MCP tools (mirrored by `av` subcommands): `start_task`, `checkpoint`, `record`, `get_context`, `expand`, `read_symbol`, `search_history`, `handoff`, `claim_task`, `submit`, `review_task`, `post_review`, `merge`, `undo`, `status`.

- `start_task` creates `~/ws/<repo>/<task>/` (jj workspace at `main@`), returns task id + first pack.
- **Handoff bundle**: goal, state (done / remaining / currently broken), all task records, per-symbol diff summary (hunks within budget), open questions.
- **PR bundle**: handoff bundle + review checklist from in-scope conventions, reported test status, risk areas (decisions without alternatives, high-fan-in symbols touched).
- **Review**: reviewer posts anchored `review` records + verdict. Approve → squash, rebase onto main, reindex, status `merged`, backup push. Request changes → back to owner, review records pinned at top of their next pack.
- **Conflicts**: jj first-class conflicts; task → `conflicted`, pack includes conflicting symbols and the other task's records.
- **Record discipline**: `submit`/`handoff` refused without intent + summary; response lists changed symbols with no record. Ship Claude Code hooks (auto-checkpoint after edits; inject short pack at session start) + generic shell wrapper for other CLIs. Tool descriptions instruct agents to log dead ends as `attempt` immediately.

## Failure handling & deployment

- Postgres down → refuse record writes (queue in memory briefly); jj ops continue. Daemon crash → resume from jj op log; `av undo` maps to op log.
- Daemon runs as host UID/GID (`user:` in compose) so bind-mounted files are never root-owned.
- Backup push retried with backoff, surfaced in `av status`, never blocks local work.
- `docker compose up` → `daemon`, `postgres:16`, `qdrant`, `text-embeddings-inference`. Named volumes for data. Host: static `av` binary; `claude mcp add --transport http av http://localhost:7420/mcp`.
- `AV_EMBED_API_KEY`, when set, is sent to the embedding API as `Authorization: Bearer`. Blank means a local embedder with no key. The key is not logged.
- If the daemon finds the `ollama` CLI and a `qwen3-embedding` model, that model is the default embedder. With no Ollama CLI, embeddings use `AV_EMBED_URL` and `AV_EMBED_API_KEY`.

## Testing

- Unit: packer scoring/budget allocation/tier rendering, anchor resolution, outbox worker, ledger.
- Integration: testcontainers (Postgres, Qdrant) against small fixture repos; golden-file tests for pack output.
- Benchmark harness (the success criterion): replay ~10 real tasks from two private repos; compare baseline vs `av` on tokens-before-first-relevant-edit, total tokens, repeated dead ends, task success.

## Scope

**v1 (this spec):** VCS engine wrapper, intent store, code index (Rust/TS/Python), context packer, MCP + HTTP + CLI, task/handoff/review/merge, git backup, Docker compose, benchmark harness.

**Later, separate specs:** (1) custom jj storage backend (chunk dedup + compression) for the storage pain; (2) copy-on-write workspaces for heavy ignored dirs (`node_modules`, `.venv`); (3) automatic record extraction from agent transcripts; (4) integration with an external dashboard UI.

