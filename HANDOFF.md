# Handoff: 2026-11-03

Delete this once you've acted on the open items. It links; the detail lives in the files named.

## 1. Merged

All on `main`; per-feature detail is in `CHANGELOG.md` (the `Unreleased` section and `0.0.2`).

- **Plan 7**, overlay index scopes: `bench/results/2026-10-30-e2-overlay/`.
- **Plan 8**, routing (backlog, agent profiles, `av next`) and **Plan 9**, the review loop (`next`
  offers reviews and merges first; `av watch`): `docs/superpowers/plans/2026-10-31-scholion-plan-8-routing.md`,
  `docs/superpowers/plans/2026-10-31-scholion-plan-9-review-loop.md`.
- **Plan 10 A1-A4** (the Community side of the editions plan: ordering with `after`, `max_open`,
  targets, office and timelines, `checkpoint` events, `scholion-agentrun`, the redesigned
  dashboard): `CHANGELOG.md`. The plan itself is in the agentvcs repo
  (`docs/superpowers/plans/2026-10-31-agentvcs-plan-10-editions-mvp.md`).
- **Plan 11**, lean disk (workspaces reclaimed, one cargo cache per repo, `av disk`/`av gc`, the
  session footprint): `docs/superpowers/plans/2026-11-02-scholion-plan-11-lean-disk.md`. Its
  shared per-repo target dir was unsafe and is replaced by Plan 12.
- **Plan 12** so far (#71–#83): one `target/` per task workspace with a lean profile (about 20 GB
  per Rust task down to 1.2 GB while open, 0 B once merged), build output removed at merge,
  dependencies built once per repo, one test binary per crate, `next --wait` woken by events,
  and per-host caps `av watch --max-agents` / `av validate --max-builds` (`scholion-agentrun::slots`).
- **Release 0.0.2** (tag `release/0.0.2`), the first to ship `scholion-daemon` with bundled Postgres.

## 2. Open

Each item names what it waits on.

1. **Plan 12, lean builds** (`docs/superpowers/plans/2026-11-03-scholion-plan-12-lean-builds.md`).
   Tasks 1, 5 and 6 are merged. Task 2 (`{crates}` in a check) is #84 and Task 3 (a task's builds
   in its own `target/` whatever `CARGO_TARGET_DIR` says) is #85; both wait on a merge. Task 4,
   workspace slots with warm targets, waits on a measurement run: is the cold build still the
   long pole now that dependencies are seeded?
2. **Plan 6 evaluation**: E4-at-scale (the DeepSeek relay run, the product claim) and E5-E7
   (swarm against a git baseline, planted-defect review, Plan 5 features under load). They spend
   API money, so they wait on the owner's go.
   `docs/superpowers/plans/2026-10-25-scholion-plan-6-evaluation.md`; run guide `bench/README.md`.
3. **Plan 8 Grok evaluation**, written, never run. E-G0 (Grok as an `av` agent) was observed on
   2026-10-31 (`bench/results/2026-10-31-routing-grok/conformance.md`); the routing runs (G1 and
   later) spend money and wait on the owner's go.
   `docs/superpowers/plans/2026-10-31-plan-8-grok-evaluation.md`.
4. **Name fan-out cap** (rust-lang/rust import stalled at ~4 GB of `main`'s edges). A retrieval
   change: not linking names with more than N candidates touches `rebuild_edges` and both view
   edge queries, and can move the `overlay_equiv` proptest and the pack goldens. It waits on an
   E1 run (no API cost; a bench daemon and the six repos of `bench/retrieval/README.md`) and on a
   choice of N. Plan 7's Risks section.
5. **Sparse checkouts** and **checkpoint by patch**: not planned; only if workspace disk or
   snapshot time becomes the limiter (build output, not checkouts, was 99.99% of a task's disk).
6. **Adapter verification against the real CLIs.** `av init --for codex|gemini|cursor` is tested
   against fixtures only (`integrations/*/README.md`); it waits on those CLIs being installed.
   Muse Code's settings path is still unknown (`integrations/muse/README.md`). Grok is verified.
7. **Dashboard agent timeline** (Plan 10 A3 remainder): the owner's decision; the recommendation
   is to keep Community `/ui` read-only with its Teams panel.

## 3. Rules that bite

- Branch off `main`, one PR per change; fmt, clippy `-D warnings` and `cargo test --workspace`
  green first (`CLAUDE.md`).
- A merged migration is never edited; check `main`'s highest number before adding one.
- Code flows scholion to agentvcs, never back.
