# syntax=docker/dockerfile:1
# Build context is the repository root (see docker-compose.yml).
# 1.97.1 matches rust-toolchain.toml (jj-lib 0.46 needs rustc >= 1.97.1).
#
# Two ways to run the image:
#   * compose (deploy/docker-compose.yml): Postgres is a separate container, `AV_DATABASE_URL` is set.
#   * alone, lite mode: `docker run -p 127.0.0.1:7420:7420 -v scholion:/data ghcr.io/vinhqbui/scholion`
#     with no `AV_DATABASE_URL`, so the daemon runs its own Postgres under $AV_HOME/pg (/data/pg).
FROM rust:1.97.1-bookworm AS build
WORKDIR /src
# Lite mode's Postgres: `--build-arg FEATURES=scholion-daemon/bundled-postgres` compiles it into the
# daemon (the release image does; the build script lists Postgres releases through the GitHub API, so
# pass a token: `--secret id=github_token,env=GITHUB_TOKEN`). Left empty, as compose builds it, the
# daemon downloads Postgres on first start instead.
ARG FEATURES=""
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/src/target \
    --mount=type=secret,id=github_token,env=GITHUB_TOKEN,required=false \
    cargo build --release --locked -p scholion-cli -p scholion-daemon ${FEATURES:+--features "$FEATURES"} \
    && cp target/release/scholion target/release/av target/release/scholion-daemon /usr/local/bin/

FROM debian:bookworm-slim
# git: the daemon shells out to `git clone --bare` when importing a repository.
RUN apt-get update \
    && apt-get install -y --no-install-recommends git ca-certificates curl \
    && rm -rf /var/lib/apt/lists/* \
    # Imported repos are mounted read-only and owned by whoever owns them on the host. git only
    # honours safe.directory from system/global config, so set it here, not through the env.
    && git config --system --add safe.directory '*' \
    # Lite mode's Postgres refuses to run as root: a plain `docker run` uses this user.
    && useradd --create-home --uid 1000 scholion \
    && mkdir -p /data && chown scholion:scholion /data
COPY --from=build /usr/local/bin/scholion /usr/local/bin/av /usr/local/bin/scholion-daemon /usr/local/bin/
# Defaults for a plain `docker run`; compose sets its own store, workspace root and user.
ENV AV_HOME=/data \
    AV_STORE_ROOT=/data/store \
    AV_WS_ROOT=/data/ws
VOLUME /data
EXPOSE 7420
# Compose runs it as the host user (`user:`), so it needs no root and writes host-owned files.
USER scholion
ENTRYPOINT ["scholion-daemon"]
