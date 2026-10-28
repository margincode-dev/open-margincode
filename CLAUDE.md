# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

Scholion: context-first version control for coding agents. Each task is an isolated jj workspace; decisions, attempts (dead ends) and notes are stored as records anchored to code; a deterministic, token-budgeted context pack hands any agent what it needs. The CLI is `scholion` with the alias `av` (same program); env vars keep the `AV_` prefix. Design: `docs/superpowers/specs/2026-10-18-agentvcs-design.md`. Implementation plans: `docs/superpowers/plans/` (Plans 1–5 are done). The spec and older plans use the earlier name `agentvcs` and crates `av-*`. The plan files' checkboxes and some signatures lag the code; when they disagree, the code wins.

## Commands

Rust is pinned to 1.97.1 (`rust-toolchain.toml`); do not change it in an unrelated change.

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace                       # all green is the bar before any commit
cargo test -p scholion-daemon --test api submit_  # one test file / name filter
cargo test -p scholion-semantic --test qdrant NAME -- --exact --nocapture
cargo build --release -p scholion-cli           # -> target/release/scholion and av
scripts/smoke.sh                             # end-to-end against the Docker stack; --down also deletes volumes
docker compose -f deploy/docker-compose.yml up -d --build   # needs `export UID GID=$(id -g)` first
```

Test environment:
- Postgres tests use `scholion_testutil::TestDb`: a fresh database on `AV_TEST_DATABASE_URL` if set, otherwise an embedded user-space Postgres (downloaded on first run). No Docker needed.
- Qdrant tests silently skip (pass in ~0s) unless `AV_TEST_QDRANT_URL` (gRPC, e.g. `http://127.0.0.1:6334`) is set. A green workspace run does not prove Qdrant works.
- `warm_build_under_300ms` is `#[ignore]`d (timing).
- Docker Desktop without WSL integration needs Windows-drive paths for bind mounts; see the README section of that name.

## Architecture

Cargo workspace, `crates/*`, edition 2024. A daemon (axum, `:7420`) owns all state; the `scholion` CLI (alias `av`) is a thin HTTP client.

- `scholion-core` — shared types, `AvError`, and the traits every unit talks through (`traits.rs`: `Vcs`, `IntentStore`, `Semantic`). `lifecycle.rs` is the pure task state machine (`next(status, Event)`); illegal transitions are `AvError::IllegalState` → HTTP 422 (`Invalid` stays 400 = malformed input).
- `scholion-vcs` — **the only crate that touches jj-lib** (pinned `=0.46.0`). Repos are git imports; one jj workspace per task. After any jj-lib rewrite call `rebase_descendants()`. `ChangeId` is the jj *commit id* hex, so any snapshot/squash/rebase produces a new id.
- `scholion-store` — Postgres `IntentStore` (sqlx, migrations in `crates/scholion-store/migrations/`, numbered `000N_*.sql`, embedded via `sqlx::migrate!`; two branches adding the same number break every DB test, so check `master`'s highest number before merging). Postgres is the source of truth. Writes that need embedding enqueue an `outbox` row in the same transaction (`outbox.rs`).
- `scholion-index` — tree-sitter (Rust, TS, Python) symbols, reference graph, skeletons and code chunks, stored in Postgres, incremental by content hash.
- `scholion-semantic` — `Embedder` (TEI, Ollama, `ScriptedEmbedder` fake) and `VectorIndex` (Qdrant, `MemoryIndex` fake). `Embedder::dimension` is `async` and fallible (the plan text says sync; the code is right). Qdrant is derived data, rebuildable with `av reindex`.
- `scholion-pack` — deterministic context packer, no LLM: seeds → candidates (structural, anchored records, semantic, live work) → scoring → budget split by section with spillover → progressive tiers (handle/summary/full) → session ledger so later packs reference instead of resend. Each pack stores a receipt (`rcp:<8 hex>`). Golden output in `tests/golden/`.
- `scholion-daemon` — `wiring.rs` builds `Config` from env and the `AppState`; `api.rs` routes (all under `/v1`); `worker.rs` drains the outbox into Qdrant every second; `embed_choice.rs` picks host Ollama `qwen3-embedding` first, else `AV_EMBED_URL`/`AV_EMBED_API_KEY`, else semantic off. If Qdrant/embedder fails, packs still return and are marked `degraded: semantic` (Postgres full-text instead). `ui.rs` + `ui.html`: the read-only dashboard at `/ui` (one static page, vanilla JS, embedded with `include_str!`; it only reads `/v1`).
  - **`flow.rs` holds the Plan 3 lifecycle operations (submit, and next handoff/claim/review/merge/undo/backup).** The plan says a separate `av-flow` crate; that was overruled because each op needs `snapshot_and_index` from `api.rs`. Their tests go in `crates/scholion-daemon/tests/api.rs`; concurrency tests (many agents at once) in `tests/swarm.rs`. Both share `tests/common/mod.rs` (`env()`, `env_with()`, `env_with_vcs()`, `start_task`, `submit`, `edit_greeter`, `call`). Workspace operations on one task take `AppState::lock_task` first, then `flow::fence` (refuses writes under an older claim generation, header `X-Av-Generation`).
  - `mcp.rs` — the MCP server at `/mcp` (JSON-RPC over streamable HTTP, plain JSON responses; hand-rolled, no SDK). Each tool is an in-process request to `api::routes()`, so tools behave exactly like the HTTP API. Tests in `tests/mcp.rs`.
- `scholion-bench` — the Plan 4 benchmark harness (`scholion-bench check|run|report|calibrate`). Pure analyzer (`events`, `metrics`, `report`, `stats`, `calibrate`); only `runner`/`orchestrate` start processes and only `run --confirm` spends money. Tests use a fake `claude` script and, for the av arms, a real daemon. Tasks and the run guide: `bench/` (`bench/README.md`).
- `scholion-cli` — clap subcommands mirroring the API. Finds its task via the nearest `.av-task` file in the workspace; `AV_URL` / `AV_SESSION` override.
- `scholion-testutil` — `TestDb` only.

Invariants worth not breaking: record summaries ≤ 200 chars and records are append-only (newer ones *supersede*); `submit`/`handoff` are refused without an intent plus at least one decision/attempt/note/handoff record; `main` advances only via `merge`; workspace paths returned to clients are host paths (`ws_host_root`), not container paths; auth is the optional shared `AV_TOKEN` (every route but `/healthz` and `/ui`), so compose still binds everything to `127.0.0.1`.

## Working conventions

- `master` is the base. Start each change on a branch off `master` and land it through a pull request; prefer one PR per change over stacked PRs (a PR stacked on another merges into that branch, not `master`).
- Work is TDD: failing test first, then implementation, then fmt + clippy + tests. See `CONTRIBUTING.md`.
- Agent-facing text (pack output, MCP tool descriptions, hook messages) says `av`, not `scholion`: it is shorter for agents to type and both names are always installed.
