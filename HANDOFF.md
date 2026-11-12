# Handoff: 2026-11-12

Delete this once you've acted on the open items. It links; the detail lives in the files named.

## 0. Since 2026-11-12: the project is Margincode, and this repository is its new home

The project was Scholion until 2026-11-12. This repository, `margincode-dev/open-margincode`,
continues `vinhqbui/scholion` with its whole history; the old one still holds the releases, the
issues and the pull-request discussions. `CLAUDE.md` ("What this is") says which names were kept
on purpose.

Landed on `main` with the move (detail in `CHANGELOG.md`, `Unreleased`):

- correctable records (`av record --supersedes`), review follow-ups, and the summary limit said
  before an agent writes;
- tests use an installed PostgreSQL before downloading one;
- a record's standing: its origin in ranking and in its line (`margincode_core::standing`);
- dreams and verdicts, the daemon's side (`dream.rs`, migration 0031; `docs/reference.md`, "A
  second look at records");
- the wrong-lesson benchmark, three runs (`bench/results/2026-11-wrong-lesson*`): with a wrong
  note as a record an agent was accepted 12 of 30 times once it could correct records (2 of 30
  before), against 28 of 30 with the same note in a notes file.

None of it had a second reviewer: the tested tree became the base of the new repository. The
gate was green on it (fmt, clippy, 1160 tests) before the rename's last commit.

Next:

1. **The agent side of dreaming**: `av dream [--verify] [--area P] [--limit N] -- <agent
   command>`, `av dream undo`, `av verdict list|show|accept|reject|retract`, and the
   instructions file the dreaming agent reads (`integrations/dream/DREAM.md`; its hash goes in
   `instructions_sha`). Tests with a fake agent script, as `av watch` has. Until it exists the
   verdict routes cannot be tried with a real model.
2. **A guard for memory writes** in `ext.rs` (`Guarded::MemoryWrite`), asked before a verdict
   takes effect, with a dummy-guard test.
3. **Measurements, prepared and not run**: seed a wrong note as the record of a reverted task
   (the free replay for standing), and pre-register raw memory against dreamed memory.

Left open on purpose:

- **Branch `loopback-default`**: the daemon refuses an open listen address without a token. It
  also rewrites `CLAUDE.md`, so it needs a rebase onto the renamed tree. Merge it before this
  repository is public.
- **Branch `wrong-lesson-cell`**: an older draft of the first benchmark run, probably superseded.
- **Release tags** are not here yet, and there is no release: `scripts/install.sh`, the README,
  the Docker image and the plugin command still name the old repository on purpose. Point them
  here with the first release (`release/0.0.4`).
- `scripts/install.sh` was syntax-checked only after the rename; `site/og.png` and the
  `demo-replay.png` images were not regenerated.

Sections 1 to 3 below are the state of 2026-11-03 and still hold where this section does not
say otherwise.

## 1. Merged

All on `main`; per-feature detail is in `CHANGELOG.md` (the `Unreleased` section, `0.0.3` and `0.0.2`).

- **Plan 7**, overlay index scopes: `bench/results/2026-10-30-e2-overlay/`.
- **Plan 8**, routing (backlog, agent profiles, `av next`) and **Plan 9**, the review loop (`next`
  offers reviews and merges first; `av watch`): `docs/superpowers/plans/2026-10-31-scholion-plan-8-routing.md`,
  `docs/superpowers/plans/2026-10-31-scholion-plan-9-review-loop.md`.
- **Plan 10 A1-A4** (the Community side of the editions plan: ordering with `after`, `max_open`,
  targets, office and timelines, `checkpoint` events, `margincode-agentrun`, the redesigned
  dashboard): `CHANGELOG.md`. The plan itself is in the agentvcs repo
  (`docs/superpowers/plans/2026-10-31-agentvcs-plan-10-editions-mvp.md`).
- **Plan 11**, lean disk (workspaces reclaimed, one cargo cache per repo, `av disk`/`av gc`, the
  session footprint): `docs/superpowers/plans/2026-11-02-scholion-plan-11-lean-disk.md`. Its
  shared per-repo target dir was unsafe and is replaced by Plan 12.
- **Plan 12** so far (#71–#83): one `target/` per task workspace with a lean profile (about 20 GB
  per Rust task down to 1.2 GB while open, 0 B once merged), build output removed at merge,
  dependencies built once per repo, one test binary per crate, `next --wait` woken by events,
  and per-host caps `av watch --max-agents` / `av validate --max-builds` (`margincode-agentrun::slots`).
- **Release 0.0.2** (tag `release/0.0.2`), the first to ship `margincode-daemon` with bundled Postgres.
- **Release 0.0.3** (tag `release/0.0.3`, 2026-11-06): proposals, `av attention`, `av repo advance`, `av notes`, cold start
  from git history, `embeddinggemma-2` as the default embedder, one approval per head and the redesigned dashboard.

## 2. Open

Each item names what it waits on.

1. **Plan 12, lean builds** (`docs/superpowers/plans/2026-11-03-scholion-plan-12-lean-builds.md`).
   Tasks 1, 2 (`{crates}` in a check, #84), 3 (a task's builds in its own `target/`, #85), 5 and 6
   are merged. Task 4 was measured (#87): the test run, not the build, is the long pole, so
   workspace slots are deferred until a measurement says otherwise.
2. **Plan 6 evaluation**: E4-at-scale (the DeepSeek relay run, the product claim) and E5-E7
   (swarm against a git baseline, planted-defect review, Plan 5 features under load). They spend
   API money, so they wait on the owner's go.
   `docs/superpowers/plans/2026-10-25-scholion-plan-6-evaluation.md`; run guide `bench/README.md`.
3. **Plan 8 Grok evaluation**, written, never run. E-G0 (Grok as an `av` agent) was observed on
   2026-10-31 (`bench/results/2026-10-31-routing-grok/conformance.md`); the routing runs (G1 and
   later) spend money and wait on the owner's go.
   `docs/superpowers/plans/2026-10-31-plan-8-grok-evaluation.md`.
4. **Name fan-out cap**: built, off by default (`AV_INDEX_MAX_FANOUT=N`). E1 at caps 64 and 16
   left pack scores unchanged but for one task at one budget and cut edges 40% / 54%
   (`bench/results/2026-11-03-e1-fanout-cap/`). Waits on the owner's choice of N and of a
   default, and on a rust-lang/rust import under the cap (it stalled at ~4 GB uncapped).
5. **Sparse checkouts**: not planned; only if workspace disk or snapshot time becomes the limiter
   (build output, not checkouts, was 99.99% of a task's disk). **Checkpoint by patch** (remote
   workspaces, Plan 13 Phase 3) is Plan 14, on branch `remote-checkpoint`
   (`docs/superpowers/plans/2026-11-04-scholion-plan-14-remote-workspaces.md`).
6. **Adapter verification against the real CLIs.** `av init --for codex|gemini|cursor` is tested
   against fixtures only (`integrations/*/README.md`); it waits on those CLIs being installed.
   Muse Code's settings path is still unknown (`integrations/muse/README.md`). Grok is verified.
7. **Dashboard agent timeline** (Plan 10 A3 remainder): the owner's decision; the recommendation
   is to keep Community `/ui` read-only with its Teams panel.

## 3. Rules that bite

- Branch off `main`, one PR per change; fmt, clippy `-D warnings` and `cargo test --workspace`
  green first (`CLAUDE.md`).
- A merged migration is never edited; check `main`'s highest number before adding one.
- Code flows margincode to agentvcs, never back.
