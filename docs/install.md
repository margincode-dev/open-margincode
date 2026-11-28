# Install

The one-line installer in the [README](../README.md) is the shortest route. These are the other
channels, all built from the same release assets
(`margincode-v<version>-<target>.tar.gz`, with `margincode`, `av` and `margincode-daemon`).

The Linux binaries are built on Ubuntu 24.04 and need glibc 2.39 or newer. On an older system
(Ubuntu 22.04 has 2.35) they install and then stop with ``version `GLIBC_2.39' not found``;
build from source there (`cargo build --release -p margincode-cli -p margincode-daemon`).

## Homebrew

macOS on Apple silicon, and Linux (x86_64 and arm64):

```bash
brew tap margincode-dev/open-margincode https://github.com/margincode-dev/open-margincode
brew install margincode
```

The tap is this repository: Homebrew reads `Formula/margincode.rb` from it. The formula pins the
version and the SHA-256 of each archive, so it is updated by hand for every release (the sums are
in the release's `.sha256` files). There is no Intel macOS build.

## cargo-binstall

```bash
cargo binstall --git https://github.com/margincode-dev/open-margincode margincode-cli      # margincode and av
cargo binstall --git https://github.com/margincode-dev/open-margincode margincode-daemon   # the daemon
```

This downloads the release archive instead of compiling. The `--git` form is needed because the
crates are not published to crates.io. The metadata is in each crate's `Cargo.toml`
(`[package.metadata.binstall]`) and expects the release tag `release/<version>` matching the crate
version.

## Docker

```bash
docker run -d --name margincode -p 127.0.0.1:7420:7420 -e AV_LISTEN_OPEN=1 -v margincode:/data ghcr.io/margincode-dev/open-margincode
```

`AV_LISTEN_OPEN=1` tells the daemon that the port is published on this host's `127.0.0.1` only;
without it (or `-e AV_TOKEN=...`, which the CLI then needs too) a daemon on an open address refuses
to start. Publish the port wider only with a token.

The image holds `margincode-daemon`, `margincode` and `av`. With no `AV_DATABASE_URL` the daemon runs
its own Postgres under `/data/pg` (lite mode); keep `/data` on a volume. The dashboard is at
`http://localhost:7420/ui`. To use Compose with a separate Postgres and Qdrant, see
`deploy/docker-compose.yml`. Tags: the release version (e.g. `0.0.5`) and `latest`.

## Uninstall

1. `av uninstall` takes out what `av init` (and `integrations/claude-code/install.sh`) added to
   each agent's config, entry by entry: av's hooks, the MCP server named `av`, the OpenCode
   plugin `plugins/av.js`. Everything else in those files stays; a file left with nothing in it
   is deleted. `--dry-run` shows what it would change, `--for AGENT` limits it to one agent, and
   `--path FILE` adds a `--for mcp-json` file. It needs no daemon, and it deletes no data.
2. `av down` stops a daemon `av up` (or `av init`) started. For the Docker image:
   `docker rm -f margincode`; for Compose: `docker compose -f deploy/docker-compose.yml down`.
3. Remove the binaries the way they were installed:
   - the install script: delete `margincode`, `av` and `margincode-daemon` from
     `~/.local/bin` (or `MARGINCODE_INSTALL_DIR`);
   - Homebrew: `brew uninstall margincode`, then `brew untap margincode-dev/open-margincode`;
   - cargo-binstall: `cargo uninstall margincode-cli margincode-daemon`;
   - the Claude Code plugin: `claude plugin uninstall margincode@margincode`, then
     `claude plugin marketplace remove margincode`.
4. Optional, and it cannot be undone: delete the data. That is the records and repositories in
   `~/.margincode` (`AV_HOME`; an older install may use `~/.scholion`), the task workspaces in
   `~/ws` (`AV_WS_ROOT`), and, for Docker, the volume `margincode` (`docker volume rm margincode`)
   or the Compose volumes (`docker compose -f deploy/docker-compose.yml down -v`). `av init` also
   left `*.bak` copies beside each config it changed; `av uninstall` names them and keeps them.
