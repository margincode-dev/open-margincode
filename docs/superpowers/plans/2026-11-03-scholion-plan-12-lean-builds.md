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

## Task 5: share dependencies — done (`deps_cache.rs`)

### Measured first (build container, main 5bea076, lean config, `cargo test --workspace --no-run`)

| | on disk | time (16 cores, warm registry) |
|---|---|---|
| full bar, fresh `target/` | 2.4 GB | 38 s on the container fs, 76 s on the `D:\ws` 9p bind mount |
| of which the repo's own crates (libs, bins, test binaries) | 1.1 GB | |
| of which dependencies | 1.4 GB: 1286 MB rlib/rmeta/so, 94 MB `build/`, 3 MB `.d`, <1 MB `.fingerprint/` | |
| the same bar in a fresh `target/` seeded from another checkout's dependencies, that checkout's `target/` deleted first | 1.2 GB | 15 s; cargo compiled the 11 own crates and 0 dependencies; `cargo test -p scholion-core` ran |

What the design rests on, each checked:
- **Which files cargo and rustc rewrite in place.** After seeding with everything hard-linked and
  making cargo rebuild 30 dependencies in the seeded workspace, 166 of the cache's
  `.fingerprint/` files and 39 of its `deps/*.d` files had changed through the shared inodes; its
  `*.rlib`, `*.rmeta` and `*.so` had not (the rebuilt workspace's copies had link count 1: rustc
  writes a new file and renames it over). So libraries are linked and everything else is copied.
- **Build-script output needs no rewriting.** `build/<pkg>-<hash>/output` names its `out` dir by
  the absolute path of the target dir it ran in (54 files did); cargo keeps that path in
  `root-output` and re-points it to the current target dir. With the original checkout's
  `target/` deleted, the seeded one built and linked (ring, psm, tree-sitter's grammars) without
  any edit; rewriting the paths by hand made 30 dependencies rebuild. No workspace refers to the
  cache's paths, so `av gc --build-cache` can remove it at any time.
- **Hard links on `D:\ws`.** Through the 9p mount from the agents' container: a 512 MB file and a
  hard link to it used 512 MB of D: (`fsutil hardlink list` names both); a copy used another
  512 MB.
- **What may be shared.** Cargo 1.97 records registry crates with `CheckDepInfo` like path crates,
  so the fingerprint does not say; and a path package's fingerprint hashes its location relative
  to the workspace root, which is how run 1's shared target dir ran another task's test binary. A
  `deps/` group (`<crate>-<hash>.*`) is shared only when its dep-info lists at least one source
  under cargo's `registry/src/` or `git/checkouts/` and nothing else but files under a build
  script's `out` dir and artifacts. A build script goes by its own dep-info, its run (no
  dep-info) with its package's scripts; a fingerprint dir when it is a lib or build-script unit
  whose artifacts go. Executables are never shared. This needs no `Cargo.toml` parsing and keeps
  a repo's own crates out whatever path its workspace is mounted at.
- `sccache` would save compile time only: each `target/` still holds its own copy of every rlib.

### What was built

- `deps_cache.rs`: `harvest(target, cache)` and `seed(cache, target)`, file operations only (the
  daemon still never runs cargo), with unit tests for the dep-info rule and the names.
- `disk.rs`: both run in the background, because on `D:\ws` (9p) each took 53–73 s of small-file
  work and the merge or `start_task` waited for it. `retire_build` renames a task's `target/` into
  `<repo>/.cache/reclaim/<slug>-…` (one rename; the merge answers at once), then harvests from
  there and removes it; `build:reclaimed` carries `cached_bytes` when that is done.
  `seed_dependencies` (from `prepare_workspace` in `api.rs` and `ensure_workspace`) builds the new
  `target/` in `<repo>/.cache/seed/<slug>-…` and, under `lock_task`, renames it into the workspace
  only if the workspace still exists and has no `target/` (an agent that builds first keeps its
  own); `build:seeded` says `linked`/`copied` or `skipped`. `sync_repos` removes both staging dirs
  at start-up. `GET /v1/disk` counts a file with several links once, measuring each repo's build
  cache (staging included) before its workspaces. All behind `AV_BUILD_CACHE` (on by default).
- **File times.** `std::fs::copy` stamps a copy "now", and cargo takes a unit as stale when a
  dependency's output is newer than its own, so copies keep the source's times (`filetime`). On
  `D:\ws` that was still not enough: the mount keeps the fraction of a second of a file it
  creates but drops it from a time that is set, so a copied build script landed up to a second
  before the linked library it was built after, and 18 dependencies above `generic-array`
  rebuilt (cargo's fingerprint log: `StaleDependency`, dep `…177.242` vs `…177.000`). Every file
  the harvest places in the cache, linked or copied, is now rounded down to the whole second:
  rounding keeps every order (`a ≤ b` stays `a ≤ b`) and cargo takes equal times as fresh.
- Tests in `tests/disk.rs`: a cargo-like fixture `target/` (a registry crate with its build script
  and run, the repo's own crate with its test binary and build script): the merge keeps the
  former and never the latter; a new task gets libraries linked (same inode) and `.d`,
  fingerprints and `build/` copied; an entry already in the cache is never replaced; retention 0
  harvests too; `AV_BUILD_CACHE=off` keeps and seeds nothing; `av disk` counts the links once;
  start-up removes what a stopped daemon left in `reclaim/` and `seed/`. Unit tests: copies keep
  their times, and the cache holds whole seconds.

### Live check (2026-11-03, daemon from this branch, ws root on the `D:\ws` 9p mount)

Two tasks on a copy of this repo, the bar `cargo test --workspace --no-run`, 6 build jobs:

| | before the background jobs and the rounding | now |
|---|---|---|
| task A, full build from nothing | 91 s, 416 crates, 2.4 GB | same |
| A's merge answers | 73 s | 1–5 s; the harvest finishes 72–80 s later in the background |
| task B's `start_task` answers | 68 s | 6–7 s; the seed finishes 60–67 s later |
| B's full build, seeded | 49 s, 29 crates (18 of them dependencies) | 42 s, 11 crates: the repo's own, no dependency |
| D: used by B's `target/` | 1.33 GB | 1.21 GB (2.4 GB apparent) |
| cache | 1.3 GB, 3881 files, no own crate | same |
| cache files changed by B's build | 0 | 0 |

So run 1's 20 GB per task is 1.2 GB while a task is open and 0 B once it is merged, plus one
1.3 GB cache per repo.

Left open: a seed takes about a minute on `D:\ws`; an agent that runs cargo before it lands
builds its own `target/` from nothing (the seed is then skipped, never mixed in). Workspace slots
(Task 4) would keep a warm `target/` instead and need no seed.

## Task 6: scholion's own test binaries — done

Each crate's `tests/*.rs` are modules of one test binary, `it` (`tests/main.rs`, `autotests =
false`); `CLAUDE.md` and `CONTRIBUTING.md` give the `--test it` invocations. A full lean
`cargo test --workspace --no-run` of `main` (ca3ec33) is 2.4 GB.

## Memory: what an agent and a build hold (measured 2026-11-03)

Run 1 ended with Windows short of memory (5.4 GB free of 31 GB, the WSL VM holding 13 GB of which
its processes used 2.3 GB). Measured afterwards in the build container (16 cores, Rust 1.97.1,
scholion at ca3ec33, av's lean config, a fresh target dir, `cargo test --workspace --no-run`,
process RSS sampled every 0.25 s with the container's cgroup `memory.stat` beside it):

| | jobs 16 (cargo's default here) | jobs 4 |
|---|---|---|
| wall time, cold | 37 s | 58 s |
| process memory at peak (cgroup `anon`) | 1.6 GB | 1.7 GB |
| sum of process RSS at peak (shared pages counted twice) | 3.4 GB, 19 processes | 3.0 GB, 11 processes |
| largest single process | 1.2 GB (rustc) | 1.1 GB (rustc) |
| file cache charged to the container at peak | 2.8 GB | 5.4 GB (started with the first run's cache) |

Running the tests afterwards (`cargo test --workspace` on that warm target, against the test
Postgres container): 97 s, 266 MB of process memory at peak, the test Postgres about 0.2 GB. So
the build, not the test run, is what a full-bar check costs in memory.

The rest of the stack, idle: daemon 15 MB, its Postgres 37 MB, Qdrant 103 MB, TEI 1.2 GB (one
each per host, not per agent). One Claude Code CLI on Windows: 0.5 GB working set, 0.8 GB private.

What this says:

1. **The shortage was the WSL VM keeping page cache, not processes.** The two builds above grew
   `vmmemWSL` on Windows from 4.0 to 9.4 GB while the VM's processes used 2.4 GB. Dropping the
   VM's page cache (`echo 1 > /proc/sys/vm/drop_caches` from a privileged container) brought it
   back to 4.1 GB within 20 s. Without a `.wslconfig` the VM keeps cache until it reaches its
   cap (half the RAM, 15.5 GB here), so every build of a long day adds to it. This is host
   configuration; `docs/reference.md` ("Docker Desktop memory on Windows") says what to set.
2. **A cargo job limit buys no memory.** Jobs 4 is 1.6× slower for the same peak: the peak is the
   last few large crates and the link of `it`, which run nearly alone at any job count. So av
   writes no `build.jobs` into the repo's cargo config.
3. **Per agent, the CLI outweighs its build.** The CLI holds 0.8 GB for the whole session; a build
   holds 1.6 GB for about 40 s (less once the deps cache has seeded the workspace).

### Sizing many agents

On one host, with A agents running and B of them building at the same moment:

    memory ≥ shared + A × cli + B × build        shared ≈ 1.5 GB, cli ≈ 0.8 GB, build ≈ 1.6 GB

`build` is this repo's; for another, measure one cold `cargo test --no-run` (peak `anon` of the
cgroup it runs in). B follows the duty cycle, not A: an agent builds for seconds between minutes of
model turns. At 1000 agents and B = 10% of A, that is 0.8 TB for the CLIs plus 160 GB for builds,
about 1 TB: eight hosts of 128 GB, not one machine. The big term is the agent process, so the
lever at that scale is agent density, not build tuning.

What 1000 agents need that does not exist yet, in the order it would bite:

- [ ] **Admission where agents start.** Whoever starts agents (`av watch`, the commercial
      runner) starts at most A = (memory − shared − B × build) / cli on a host. The daemon's
      `max_open` caps a *team's* open tasks, not a host's load; it never starts processes, so it
      cannot be the limit.
- [x] **`next --wait` polled.** Each waiting agent re-ran `choose` every 2 s, so 1000 waiting
      agents were about 500 `choose` queries a second against a pool of `AV_PG_MAX_CONNECTIONS`
      (default 16). Now a waiter is woken by what can change its answer: an event of
      `WAKING_KINDS` (`submit`, `approve`, `merged`, `handoff`, `abandon`, …, of its repo when it
      names one) from the one in-process event feed (a single 200 ms reader of the events table
      for all subscribers), or `AppState::work_changed` (backlog post or drop, profile change);
      else every `WAIT_TICK` (10 s) for what only time releases. Idle waiters cost 0.1 `choose`
      a second each instead of 0.5, and work reaches them at once instead of within 2 s. Left:
      one waking event still makes every waiter look (N `choose` per event), so this wins only
      while waking events come slower than one per 2 s per waiter; past that, one `choose` per
      wake handing out to all waiters is the next step. `av watch` and the Stop hook still poll
      a dry-run `next` from the client; they gain nothing here.
- [ ] **More than one host.** The daemon writes every workspace under its own `ws_root`, so all
      agents build on the daemon's filesystem. Several hosts need either that root shared (slow
      over the network for builds, as 9p already is at 2×) or workspaces created on the agent's
      host by something the daemon talks to. A design question for when one host is full.
- [ ] **Build spikes.** If B spikes (every agent re-syncs and builds after one merge to `main`), a
      per-host gate on check commands (N build slots, taken by the CLI around a check, never by
      the daemon) bounds it. Only worth building once a run measures B above 10% of A.

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

## Live check of main after #71–#75 (2026-11-03 15:17 UTC, daemon from 16b1dd7, build container)

The same two-task run, now with every merged default in force (per-task `target/`, lean profile,
one test binary per crate, build output removed at merge) and the default retention (3600 s).

- `cargo metadata` in each workspace: `<ws_root>/scholion/<slug>/target`, two different dirs.
- Both built at once (`cargo test --workspace --no-run` in one, `-p scholion-core` in the other),
  416 crates, 37 s on 16 cores with a warm registry. `av disk` while both were open:
  `full-bar-in-one-task` 2.6 GB (build 2.6 GB), `one-crate-in-another` 95 MB (build 90 MB).
- In the first: an edit, a decision record, submit, `cargo test -p scholion-core --offline` on its
  own binaries (23 passed), approve, merge. The merge answered `status: merged, workspace: kept,
  build: removed`; the feed has `build:reclaimed {slug, bytes: 2884986242}`.
- `av disk` after the merge: the merged workspace 5 MB with no build column, the open one
  unchanged. Abandoning the second answered `workspace: kept, build: removed` too; totals then:
  workspaces 10 MB, build output 0 B, store 6 MB.

So on main a Rust task's build output is 2.6 GB for this repo's full bar (21 GB in run 1), it
shows in `av disk` while the task is open, and it is gone the moment the task is merged or
abandoned; the 5 MB workspace waits for the retention.

## The live stack (2026-11-03, after #71–#73 merged)

- The compose daemon (`deploy/docker-compose.yml`, project `scholion`, `:7420`) was rebuilt from
  `main` (ae19a2b) and recreated with the run's mounts (`AV_WS_MOUNT='D:\ws'`,
  `AV_IMPORT_MOUNT='C:\Users\vinhq\projects'`, `AV_WS_ROOT='D:/ws'` for the host root, the same
  `UID:GID`). At start-up it rewrote `D:\ws\scholion\.cargo\config.toml` from Plan 11's shared
  target dir to the lean per-task config, and the sweep reclaimed run 1's five merged workspaces.
  The old image is kept as `scholion-daemon:pre-plan12` for rollback. In bash, pass the ids
  through `env UID=… GID=… docker compose …`: `UID` is read-only in the shell.
- **Two things the daemon cannot do, both done by hand on 2026-11-03 ~15:40 UTC (`sch-ws` recreated as below; the bar is now `conv:7` with the scope below, `conv:1` removed):**
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
  Builds on the `D:\ws` bind mount (9p) take 2× the container-fs time (76 s vs 38 s for the full
  bar), measured in the build container; tolerable, so Task 3 stays after Task 5.
- The live repo's trunk is run 1's import (before #72), so a task's full bar there costs about
  4.3 GB until the repo is re-imported from current `main`; after that, 2.4 GB.
- Docker's `D:\DockerData\DockerDesktopWSL\disk\docker_data.vhdx` is 175 GB for about 15 GB of
  volumes. It shrinks only with Docker Desktop stopped: `wsl --shutdown`, then in an elevated
  shell `diskpart` → `select vdisk file=…` → `attach vdisk readonly` → `compact vdisk` →
  `detach vdisk`. Needs someone at the machine for the UAC prompt.
