# syntax=docker/dockerfile:1
# Build context is the repository root (see docker-compose.yml).
# 1.97.1 matches rust-toolchain.toml (jj-lib 0.46 needs rustc >= 1.97.1).
FROM rust:1.97.1-bookworm AS build
WORKDIR /src
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/src/target \
    cargo build --release -p scholion-daemon \
    && cp target/release/scholion-daemon /usr/local/bin/scholion-daemon

FROM debian:bookworm-slim
# git: the daemon shells out to `git clone --bare` when importing a repository.
RUN apt-get update \
    && apt-get install -y --no-install-recommends git ca-certificates curl \
    && rm -rf /var/lib/apt/lists/* \
    # Imported repos are mounted read-only and owned by whoever owns them on the host. git only
    # honours safe.directory from system/global config, so set it here, not through the env.
    && git config --system --add safe.directory '*'
COPY --from=build /usr/local/bin/scholion-daemon /usr/local/bin/scholion-daemon
EXPOSE 7420
# Runs as the host user (compose `user:`), so it needs no root and writes host-owned files.
ENTRYPOINT ["scholion-daemon"]
