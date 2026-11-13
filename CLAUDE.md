# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.
It is loaded into every session, so it stays short: each module's detail is in
`docs/architecture.md` (read the section for the module you touch), the user-facing reference in
`docs/reference.md`. When a doc and the code disagree, the code wins.

## What this is

Margincode: context-first version control for coding agents. Each task is an isolated jj workspace;
decisions, attempts (dead ends) and notes are records anchored to code; a deterministic,
token-budgeted context pack hands any agent what it needs. CLI `margincode`, alias `av` (same
program); env vars keep the `AV_` prefix. Design: `docs/superpowers/specs/2026-10-18-agentvcs-design.md`;
plans in `docs/superpowers/plans/` (their checkboxes and signatures lag the code; where each plan
stands is in `docs/architecture.md`). Current release: `release/0.0.3` (`CHANGELOG.md`; add an
`## Unreleased` entry for user-visible changes). The repository is `margincode-dev/open-margincode`.

The project was Scholion until 2026-11-12 (crates `scholion-*`), and before that `agentvcs` (crates
`av-*`). The spec, `docs/superpowers/` (file names too), `bench/results/`, `bench/tasks-scholion` and
released `CHANGELOG.md` entries keep those names. Three stored identities keep the old name **on
purpose** (a new one would start an existing install on empty data): lite mode's database
(`embedded_pg::DATABASE`), the Compose project name, and an existing `~/.scholion` (used when
`~/.margincode` is absent: `slots::av_home_with`, `wiring::home_dir_in`).

## Commands

Rust is pinned to 1.97.1 (`rust-toolchain.toml`); do not change it in an unrelated change.

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace                       # all green is the bar before any commit
cargo test -p margincode-daemon --test it api::submit_  # one crate's test binary, module / name filter
cargo test -p margincode-semantic --test it qdrant::NAME -- --exact --nocapture
cargo build --release -p margincode-cli           # -> target/release/margincode and av
scripts/smoke.sh                             # end-to-end against the Docker stack; --down also deletes volumes
docker compose -f deploy/docker-compose.yml up -d --build   # needs `export UID GID=$(id -g)` first
```

Test environment:
- Postgres tests use `margincode_testutil::TestDb`: a fresh database on `AV_TEST_DATABASE_URL` if set; else, when a PostgreSQL is installed on the machine (`installed_bin`: `AV_TEST_PG_INSTALL_DIR`, `pg_config` on `PATH`, the usual install directories; `AV_TEST_PG_INSTALL_DIR=none` skips this), a private server started from its binaries in a temporary directory, one per `TestDb`, as user `postgres` or `nobody` when the tests run as root, stopped and removed on drop (`installed.rs`; no download, the installed service is not touched); otherwise an embedded user-space Postgres (downloaded on first run). No Docker needed.
- Qdrant tests silently skip (pass in ~0s) unless `AV_TEST_QDRANT_URL` (gRPC, e.g. `http://127.0.0.1:6334`) is set. A green workspace run does not prove Qdrant works.
- `warm_build_under_300ms` is `#[ignore]`d (timing).
- Scheduled, never on pull requests: `.github/workflows/nightly-scale.yml` (`margincode-bench scale` on tokio and `retrieval` on `bench/retrieval/ci/`, release daemon, no semantic layer, floors in the file with their sources) and, by hand only for now, `e4-smoke.yml` (E4 smoke on DeepSeek, weekly schedule off; needs the secret `LLM_API_KEY`, skips with a notice without it). Results are the runs' artifacts.
- Docker Desktop without WSL integration needs Windows-drive paths for bind mounts; see the section of that name in `docs/reference.md` (the README is the short introduction; the reference holds the details).

## Module map

Cargo workspace `crates/*`, edition 2024. A daemon (axum, `:7420`) owns all state; the CLI is a
thin HTTP client. Each crate's detail is in `docs/architecture.md`, "Modules".

| Crate | Role | Must know |
|---|---|---|
| `margincode-core` | types, `AvError`, traits (`Vcs`, `IntentStore`, `Semantic`), pure lifecycle and routing rules | illegal transition = `IllegalState` → 422; malformed input = `Invalid` → 400 |
| `margincode-vcs` | **the only crate that touches jj-lib** (`=0.46.0`); one jj workspace per task; `patch.rs` renders/applies git patches | call `rebase_descendants()` after any rewrite; `ChangeId` is the jj *commit* id, so every snapshot/rebase makes a new one |
| `margincode-store` | Postgres `IntentStore` (sqlx), migrations `migrations/000N_*.sql` | check `main`'s highest number before adding one; a merged migration is never edited (checksums; add a line to `tests/migrations_frozen.rs`) |
| `margincode-index` | tree-sitter symbols, refs, skeletons, chunks in Postgres | refs link by bare name to every same-named symbol |
| `margincode-semantic` | embedders and Qdrant (derived data, `av reindex`) | `Embedder::dimension` is async and fallible |
| `margincode-pack` | deterministic context packer, no LLM | golden output in `tests/golden/` |
| `margincode-daemon` | `wiring.rs` (config), `api.rs` (routes under `/v1`), `flow.rs` (lifecycle ops), `routing_api.rs` (`next`, backlog), `disk.rs`, `history.rs`, `dream.rs`, `remote.rs`, `mcp.rs`, `ext.rs`, … | see below |
| `margincode-agentrun` | runs agent CLIs (`AgentCommand`), host-wide slots | knows no tasks or daemons |
| `margincode-bench` | benchmark harness; only `run --confirm` spends money | `bench/README.md` |
| `margincode-cli` | clap subcommands mirroring the API, hooks, `av up`/`init`/`watch`/`remote`/`dream` | finds its task via the nearest `.av-task` |
| `margincode-testutil` | `TestDb` only | |

Daemon rules:
- Workspace operations on one task take `AppState::lock_task` first, then `flow::fence` (claim
  generation, header `X-Av-Generation`).
- Lifecycle op tests go in `tests/api.rs`, concurrency in `tests/swarm.rs`; helpers in
  `tests/common/mod.rs`. Each crate's `tests/*.rs` are modules of one test binary `it`
  (`tests/main.rs`, `autotests = false`): **add a `mod` line for a new test file.**
- `ext.rs` is the extension surface the Commercial edition (margincode-cloud) builds on: keep it
  stable; a change there is a breaking change downstream.
- Deletes on disk go only through `under_ws_root`.
- The MCP tool list is size-guarded in `tests/mcp.rs`.

`integrations/` holds the agent glue (Claude Code hooks, a generic wrapper); see
`docs/architecture.md`.

## Invariants

The full rules, with the functions that enforce them, are in `docs/architecture.md`, "Invariants in detail".

- Record summaries ≤ 200 chars; records are append-only (newer ones *supersede*;
  `margincode_core::supersede_refusal` is the one rule for what may supersede what).
- `submit`/`handoff` need an intent plus at least one decision/attempt/note/handoff record.
- `main` advances only via `merge`, `revert` (adds a commit), or fast-forward from the backup
  remote.
- The daemon never runs repository commands: validations are attestations reported by clients;
  a remote checkpoint's patch is applied in process (`margincode-vcs::patch`), never `git apply`.
- Workspace paths returned to clients are host paths (`ws_host_root`), not container paths.
- Auth is the optional shared `AV_TOKEN` (every route but `/healthz` and `/ui`); `AV_LISTEN`
  defaults to `127.0.0.1:7420`, and a non-loopback one needs `AV_TOKEN` or `AV_LISTEN_OPEN=1`
  (`wiring::check_listen`).

## Working conventions

- `main` is the base. Start each change on a branch off `main` and land it through a pull request; prefer one PR per change over stacked PRs (a PR stacked on another merges into that branch, not `main`).
- Work is TDD: failing test first, then implementation, then fmt + clippy + tests. See `CONTRIBUTING.md`.
- Agent-facing text (pack output, MCP tool descriptions, hook messages) says `av`, not `margincode`: it is shorter for agents to type and both names are always installed.
