# Plan 12: lean builds (storage per successful task)

**Goal:** the disk a Rust task costs is bounded, small and accounted for. The first dogfood run
(`D:\Projects\agent-vcs\dogfood\2026-11-03`, report and `fable-storage-brief.md`) measured where
the bytes went; this plan takes them back, in order of payoff over effort. The north-star metric is
**infrastructure cost per successful task** (merged, check green, review passed), and storage is
its biggest term.

**What the run measured** (5 Rust tasks on scholion, 3–4 agents at once, Docker on Windows):

| Store | Per task | Note |
|---|---|---|
| jj store, Postgres, Qdrant | ≈ 2–3 MB | content-addressed, tiny |
| Workspace checkout | 6 MB | |
| **cargo target dir** | **≈ 20 GB** | 14 GB of 96 test binaries (73% debug info), 3–4 GB `incremental/`, 2.8 GB dependency rlibs, 0.3 GB build scripts |

Build output is 99.99% of the disk. The storage problem is a build-artifact problem, not a
context-storage problem. Two further findings: a target dir **shared by workspaces of one repo is
unsafe** (cargo names a path package's artifacts by its path relative to the workspace root, so
both workspaces produce the same names and `cargo test` in one ran the other's binary; Plan 11's
`.cache/cargo-target` had exactly this flaw), and a docs-only task paid the full build because the
required check's glob matched every path.

Anatomy of the same tree built two ways (scholion at `aedb7e3`, `cargo test --workspace
--no-run`, measured 2026-11-03 in the build container):

| | default profile | lean (`debug = "line-tables-only"`, `incremental = false`) | lean + `package."*".debug = false` (shipped) |
|---|---|---|---|
| total | **21 GB** | **6.7 GB** | **4.3 GB** |
| test/bin executables | 14 GB (65) | 4.9 GB (63) | 3.0 GB (63) |
| `incremental/` | 4.0 GB | 0 | 0 |
| dependency rlib + rmeta | 2.7 GB | 1.4 GB | 1.3 GB |
| build scripts | 270 MB | 240 MB | ≈ 240 MB |

## Task 1: one target dir per task, lean profile — done (this plan's first PR)

- `build_cache.rs` writes, for a repo whose trunk has a root `Cargo.toml`,
  `<ws_root>/<repo>/.cargo/config.toml` with `[build] incremental = false`, `[profile.dev]
  debug = "line-tables-only"`, `[profile.dev.package."*"] debug = false` (dependencies carry no
  debug info; the repo's own crates keep file:line), and **no `target-dir`**. Each workspace builds into its own
  `target/` (cargo's default), which `remove_workspace` deletes with the workspace. A file whose
  first line starts with `# written by av:` is rewritten, so the Plan 11 file goes at start-up.
- `scholion-vcs` adds `/target/` to `AGENT_IGNORES`: a workspace's top-level build output is never
  snapshotted, even in a repo whose `.gitignore` forgot it.
- `GET /v1/disk` reports `build_bytes` per workspace and in totals; `POST /v1/gc` reports it per
  reclaimed workspace; `av disk` has a `BUILD` column; the dashboard's Disk card shows it.
- Tests: `two_workspaces_of_one_repo_never_share_a_target_dir` (runs `cargo metadata` in two
  workspaces of one imported cargo repo), `disk_and_gc_count_a_workspaces_build_output_apart`,
  `build_output_is_never_snapshotted_even_when_the_repo_does_not_ignore_it`.

Expected effect on the run above: ≈ 20 → ≈ 4.5 GB per task (−78%), no false validations, and a
reviewer validating in the owner's workspace still gets a warm build.

## Task 2: checks scoped to what changed

Conventions already carry a `scope_glob`, and `flow::applicable_checks` keeps only the required
conventions whose glob matches a changed path. So "docs-only → no build" needs no code: the
required convention's scope must be `crates/**` (or similar), not `**`. To do:

- [ ] `bench/` and `examples/routing-smoke` set-up scripts and the dogfood brief use a scoped
      glob for the full bar; `docs/reference.md` says so (done in the first PR's text).
- [ ] A change inside one crate: `cargo test -p <crate>` plus its reverse dependencies. The daemon
      has the changed paths at submit and the workspace's crate graph (`Cargo.toml` members); a
      check command may carry `{crates}` which `av validate` expands to `-p a -p b`. Needs a
      design note first: the reverse-dependency closure must come from `cargo metadata`, not
      guesses, or a green check can miss a dependant.

## Task 3: `av disk` and the sweep own build dirs outside `ws_root`

The run's agents built into `CARGO_TARGET_DIR=/target/ws-<slug>` (a Docker volume, for speed on a
bind mount). Nothing of av's knew those dirs; they were deleted by hand.

- [ ] A task may register a build dir (`POST /v1/tasks/{id}/build-dir`, or the session-start hook
      posts `CARGO_TARGET_DIR` when set) so `av disk` counts it under the task and the reclaim
      removes it with the workspace. Deletes stay under a root the daemon was given
      (`AV_BUILD_ROOT`), never arbitrary paths.
- [ ] The footprint already shows a session's `target` dirs outside any workspace; `outside`
      entries that name a task's registered build dir move under the task.

## Task 4: workspace slots with warm targets (the 2× concurrency lever)

N stable workspace directories per repo (N = concurrency), each with its own warm `target/`. A new
task is checked out *into* a free slot (jj rewrites the files and bumps mtimes, so cargo's
freshness is right). Disk becomes N × slot, bounded by concurrency rather than by the number of
tasks, and every build is incremental-warm (the run saw a warm review build in 4 min against
10–15 cold).

- [ ] Design note: the workspace path is today `<ws_root>/<repo>/<slug>` and stored per task; a
      slot means a stable path and a task ↔ slot mapping, plus the invariant that a slot holds one
      open task at a time (`lock_task` is per task, so a slot lock is new).
- [ ] Measure before deciding: with Task 1, is the cold build (and its 6.7 GB) still the long pole?

## Task 5: share dependencies

The 1.4 GB of dependency rlibs per task is the only part that is the same in every task's target
dir. Options, in order of simplicity: hard-link the `deps/*.rlib`/`*.rmeta` of a per-repo "deps at
main" build into a new task's `target/` (same filesystem; cargo's fingerprints are by content
hash and path, so links are valid as long as the path-relative names match, which they do for
registry crates), or `sccache` with a local disk backend. Measure the saving against Task 1 first:
it is at most 1.4 of 6.7 GB.

## Task 6: scholion's own test binaries

63 integration-test binaries, each linking everything. Merging each crate's `tests/*.rs` into one
binary per crate (`tests/main.rs` with `mod` lines) would cut both disk and link time for the
dogfood repo itself. A repo change, not a product change; cheap, but it changes every
`cargo test --test NAME` invocation in `CLAUDE.md` and `CONTRIBUTING.md`.

## How to tell it worked

Run the same five backlog items again on this daemon and compare, per merged task: GB peak in
target dirs (`av disk` now reports it), minutes per validation, validations failed for reasons
outside the code (0 expected from the shared-target class), review rounds. `scholion-bench`'s
plain-git arm gives the baseline without av.

## Live check of Task 1 (2026-11-03, daemon from this branch, build container)

Imported this repo (205 files, 7 s), started two tasks, built in both at once: `cargo test
--workspace --no-run` in one, `cargo test -p scholion-core --no-run` in the other, with the
daemon's config in force and no `CARGO_TARGET_DIR`.

- `cargo metadata` in each workspace resolved `<ws_root>/scholion/<slug>/target`: two different
  dirs, both inside their workspace.
- `av disk` while both were open: `build-the-whole-workspace` 4.6 GB (build 4.5 GB),
  `build-one-crate` 95 MB (build 90 MB); store 5 MB.
- `cargo test -p scholion-core --offline` in the first workspace ran its own warm binaries: 23
  passed.
- `POST /abandon` on both with `AV_WS_RETAIN_SECS=0` answered `workspace: removed`; `av disk`
  afterwards: workspaces 0 B, build output 0 B.

So with this branch a Rust task's disk is its own `target/` (4.5 GB for this repo's full bar,
2.4 GB once #72 lands), it is counted, and it is gone with the task.

## The live stack (2026-11-03, after #71–#73 merged)

- The compose daemon (`deploy/docker-compose.yml`, project `scholion`, `:7420`) was rebuilt from
  `main` (ae19a2b) and recreated with the run's mounts (`AV_WS_MOUNT='D:\ws'`,
  `AV_IMPORT_MOUNT='C:\Users\vinhq\projects'`, `AV_WS_ROOT='D:/ws'` for the host root, the same
  `UID:GID`). At start-up it rewrote `D:\ws\scholion\.cargo\config.toml` from Plan 11's shared
  target dir to the lean per-task config, and the sweep reclaimed run 1's five merged workspaces.
  The old image is kept as `scholion-daemon:pre-plan12` for rollback. In bash, pass the ids
  through `env UID=… GID=… docker compose …`: `UID` is read-only in the shell.
- **Before the next run, two things the daemon cannot do:**
  1. The agents' container `sch-ws` exports `CARGO_TARGET_DIR=/target`. The environment beats
     every cargo config, so agents there would still share one target dir. Recreate it without
     that variable (builds then land in `/ws/scholion/<slug>/target`, reclaimed with the task), or
     give each agent `CARGO_TARGET_DIR=/target/ws-<slug>` as run 1's lead did (then `av disk`
     does not see those dirs: Task 3):
     ```
     docker rm -f sch-ws
     docker run -d --init --name sch-ws --network sch-test --add-host host.docker.internal:host-gateway \
       -v sch-cargo:/usr/local/cargo/registry -v sch-target:/target -v 'D:\ws:/ws' \
       -e AV_TEST_DATABASE_URL=postgres://postgres:pg@sch-test-pg:5432/postgres \
       -e AV_URL=http://host.docker.internal:7420 -e AV_FOOTPRINT=off -w /ws rust:1.97.1 sleep infinity
     ```
  2. The live repo's required full-bar convention (`conv:1`) is scoped to `**`, so a docs-only
     task runs the whole bar. Replace it with one scoped to the code
     (`{crates/**,Cargo.toml,Cargo.lock,rust-toolchain.toml}`; globset takes braces), and drop the
     old one:
     ```
     av convention add --repo scholion --required \
       --scope '{crates/**,Cargo.toml,Cargo.lock,rust-toolchain.toml}' \
       --check 'cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace' \
       'The bar: fmt, clippy -D warnings and cargo test --workspace all green. Build in the workspace itself (its own target/); never point CARGO_TARGET_DIR at a dir another task uses.'
     av convention rm 1 --repo scholion
     ```
- The live repo's trunk is run 1's import (before #72), so a task's full bar there costs about
  4.3 GB until the repo is re-imported from current `main`; after that, 2.4 GB.
- Docker's `D:\DockerData\DockerDesktopWSL\disk\docker_data.vhdx` is 175 GB for about 15 GB of
  volumes. It shrinks only with Docker Desktop stopped: `wsl --shutdown`, then in an elevated
  shell `diskpart` → `select vdisk file=…` → `attach vdisk readonly` → `compact vdisk` →
  `detach vdisk`. Needs someone at the machine for the UAC prompt.
