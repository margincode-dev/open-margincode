# Contributing to Scholion

Thanks for looking. Scholion is a small, vibe-coded experiment (see "How this was built" in the
README), so expect rough edges, and say so in issues when you hit one.

## Setup

- Rust is pinned in `rust-toolchain.toml` (1.97.1); `rustup` picks it up. Don't bump it in an
  unrelated change.
- Tests need Postgres. With nothing set, `scholion-testutil` downloads an embedded user-space
  Postgres on first run. To use your own server instead, set
  `AV_TEST_DATABASE_URL=postgres://user:pass@127.0.0.1:5432/postgres`; each test creates its own
  database there.
- Qdrant tests skip silently unless `AV_TEST_QDRANT_URL` (gRPC, e.g. `http://127.0.0.1:6334`) is
  set, so a green run does not prove the Qdrant code works.
- Docker with Compose runs the full stack (`deploy/docker-compose.yml`, see the README's "Run").

## Before you open a pull request

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

All three must pass; CI runs the same on every pull request. For changes to the stack or the
CLI's end-to-end flow, also run `scripts/smoke.sh`.

## How changes are made

- **Tests first.** Write the failing test, then the code. Daemon behaviour is tested through the
  HTTP API in `crates/scholion-daemon/tests/api.rs` (helpers in `tests/common/mod.rs`); many
  agents at once in `tests/swarm.rs`; MCP in `tests/mcp.rs`; context-pack output against golden
  files in `crates/scholion-pack/tests/golden/`.
- **Migrations** live in `crates/scholion-store/migrations/`, numbered `000N_*.sql`. Take the next
  number after `master`'s highest; two branches with the same number break every database test.
  A merged migration is never edited, not even a comment: databases store its checksum and refuse
  to start when the file differs. `tests/migrations_frozen.rs` pins each file's hash; add a line for
  a new migration.
- **One change per pull request**, branched from `master`.
- Larger features start as a plan in `docs/superpowers/plans/`, in the style of the existing ones.
  Older plans and the design spec use the project's earlier name (`agentvcs`, crates `av-*`).

## Invariants worth not breaking

- Record summaries are at most 200 characters, and records are append-only: a newer record
  supersedes an older one.
- `submit` and `handoff` are refused without an intent plus at least one decision, attempt, note
  or handoff record.
- `main` advances only through `merge` (and `revert`, which adds a new commit).
- The daemon never runs repository commands. Validations and checks are reported by clients.
- Carrying an approval fails closed: when in doubt, ask for a fresh review.
- Everything in compose binds to `127.0.0.1`. Auth is one optional shared token (`AV_TOKEN`), so
  don't expose the daemon to a network you don't trust.

## Security

Please report security problems privately (a GitHub security advisory on this repository) rather
than in a public issue.
