# Contributing to Margincode

Thanks for looking. Margincode is a small, vibe-coded experiment (see "How this was built" in the
README), so expect rough edges, and say so in issues when you hit one.

## Setup

- Rust is pinned in `rust-toolchain.toml` (1.97.1); `rustup` picks it up. Don't bump it in an
  unrelated change.
- Tests need Postgres. With nothing set, `margincode-testutil` uses a PostgreSQL installed on the
  machine if there is one: each test starts a private server from its binaries in a temporary
  directory and removes it afterwards, so nothing is downloaded and your own service is not
  touched (`AV_TEST_PG_INSTALL_DIR` names an install; `none` skips this). Without one it
  downloads an embedded user-space Postgres on first run. To use your own server instead, set
  `AV_TEST_DATABASE_URL=postgres://user:pass@127.0.0.1:5432/postgres`; each test creates its own
  database there.
- Qdrant tests skip silently unless `AV_TEST_QDRANT_URL` (gRPC, e.g. `http://127.0.0.1:6334`) is
  set, so a green run does not prove the Qdrant code works.
- Docker with Compose runs the full stack (`deploy/docker-compose.yml`, see "Run" in `docs/reference.md`).

## Before you open a pull request

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

All three must pass; CI runs the same on every pull request. For changes to the stack or the
CLI's end-to-end flow, also run `scripts/smoke.sh`.

### Running the bar in a container

- Run as a non-root user if you can. As root, the tests need `AV_TEST_DATABASE_URL` or a
  PostgreSQL installed in the container (`margincode-testutil` runs it as `postgres` or
  `nobody`). The two `embedded_pg` tests start lite mode's Postgres, which refuses root: as root
  they skip with a `SKIPPED embedded_pg` line, so a green run as root does not prove lite mode
  starts. They also skip unless `AV_TEST_PG_INSTALL_DIR` names an installation.
- To run it on an av task, mount the repo's directory under the workspace root
  (`<ws_root>/<repo>`) and set the working directory to the task's workspace inside it
  (`<repo>/<slug>`). av's lean-build config is `<ws_root>/<repo>/.cargo/config.toml`, and cargo
  reads `.cargo/config.toml` only in the working directory and its parents, so a container that
  mounts only the workspace builds without it. Mounted this way, the build goes to the
  workspace's own `target/`, which av counts and removes with the task.
- For a control run on a clean `main`, clone the backup remote: `git clone -b av-main
  <export>.git control` is exactly `main` and starts no task. A merged task's workspace is
  removed after `AV_WS_RETAIN_SECS` (an hour by default), and its `target/` at once.

## How changes are made

- **`constitution.md` first.** It holds the rules every change follows; what is below is how.
- **Tests first.** Write the failing test, then the code. Daemon behaviour is tested through the
  HTTP API in `crates/margincode-daemon/tests/api.rs` (helpers in `tests/common/mod.rs`); many
  agents at once in `tests/swarm.rs`; MCP in `tests/mcp.rs`; context-pack output against golden
  files in `crates/margincode-pack/tests/golden/`.
  Each crate's `tests/*.rs` files are modules of one test binary, `it` (declared in `tests/main.rs`;
  add a `mod` line for a new file): `cargo test -p margincode-daemon --test it api::NAME`. One
  executable per crate instead of one per file keeps a task's build output small.
- **Migrations** live in `crates/margincode-store/migrations/`, numbered `000N_*.sql`. Take the next
  number after `main`'s highest; two branches with the same number break every database test.
  A merged migration is never edited, not even a comment: databases store its checksum and refuse
  to start when the file differs. `tests/migrations_frozen.rs` pins each file's hash; add a line for
  a new migration.
- **One change per pull request**, branched from `main`.
- Larger features start as a plan in `docs/superpowers/plans/`, in the style of the existing ones.
  Older plans and the design spec use the project's earlier name (`agentvcs`, crates `av-*`).
- **Editions.** The hosted Commercial edition is called Margincode Cloud and lives in a separate private repository; this repository is only the Community edition.

## Invariants worth not breaking

- Record summaries are at most 200 characters, and records are append-only: a newer record
  supersedes an older one.
- `submit` and `handoff` are refused without an intent plus at least one decision, attempt, note
  or handoff record.
- `main` advances only through `merge` (and `revert`, which adds a new commit), or by
  fast-forward to the backup remote's branch (`av repo advance`), never past a divergence.
- The daemon never runs repository commands. Validations and checks are reported by clients.
- Carrying an approval fails closed: when in doubt, ask for a fresh review.
- Everything in compose binds to `127.0.0.1`. Auth is one optional shared token (`AV_TOKEN`), so
  don't expose the daemon to a network you don't trust.

## Security

Please report security problems privately (a GitHub security advisory on this repository) rather
than in a public issue.
