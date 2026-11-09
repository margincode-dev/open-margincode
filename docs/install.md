# Install

The one-line installer in the [README](../README.md) is the shortest route. These are the other
channels, all built from the same release assets
(`margincode-v<version>-<target>.tar.gz`, with `margincode`, `av` and `margincode-daemon`).

## Homebrew

macOS on Apple silicon, and Linux (x86_64 and arm64):

```bash
brew tap vinhqbui/scholion https://github.com/vinhqbui/scholion
brew install margincode
```

The tap is this repository: Homebrew reads `Formula/margincode.rb` from it. The formula pins the
version and the SHA-256 of each archive, so it is updated by hand for every release (the sums are
in the release's `.sha256` files). There is no Intel macOS build.

## cargo-binstall

```bash
cargo binstall --git https://github.com/vinhqbui/scholion margincode-cli      # margincode and av
cargo binstall --git https://github.com/vinhqbui/scholion margincode-daemon   # the daemon
```

This downloads the release archive instead of compiling. The `--git` form is needed because the
crates are not published to crates.io. The metadata is in each crate's `Cargo.toml`
(`[package.metadata.binstall]`) and expects the release tag `release/<version>` matching the crate
version.

## Docker

```bash
docker run -d --name margincode -p 127.0.0.1:7420:7420 -e AV_LISTEN_OPEN=1 -v margincode:/data ghcr.io/vinhqbui/scholion
```

`AV_LISTEN_OPEN=1` tells the daemon that the port is published on this host's `127.0.0.1` only;
without it (or `-e AV_TOKEN=...`, which the CLI then needs too) a daemon on an open address refuses
to start. Publish the port wider only with a token.

The image holds `margincode-daemon`, `margincode` and `av`. With no `AV_DATABASE_URL` the daemon runs
its own Postgres under `/data/pg` (lite mode); keep `/data` on a volume. The dashboard is at
`http://localhost:7420/ui`. To use Compose with a separate Postgres and Qdrant, see
`deploy/docker-compose.yml`. Tags: the release version (e.g. `0.0.3`) and `latest`.
