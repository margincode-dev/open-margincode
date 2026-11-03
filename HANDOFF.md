# Handoff: 2026-11-02

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
  session footprint): `docs/superpowers/plans/2026-11-02-scholion-plan-11-lean-disk.md`.
- **Release 0.0.2** (tag `release/0.0.2`), the first to ship `scholion-daemon` with bundled Postgres.

## 2. Open

1. **Plan 6 evaluation**, still in progress: E4-at-scale (the DeepSeek relay run, the product
   claim) and E5-E7 (swarm against a git baseline, planted-defect review, Plan 5 features under
   load). `docs/superpowers/plans/2026-10-25-scholion-plan-6-evaluation.md`; run guide `bench/README.md`.
2. **Plan 8 Grok evaluation**, written, never run. E-G0 (Grok as an `av` agent) was observed on
   2026-10-31 (`bench/results/2026-10-31-routing-grok/conformance.md`); the routing runs (G1 and
   later) were not. `docs/superpowers/plans/2026-10-31-plan-8-grok-evaluation.md`.
3. **rust-lang/rust import and name fan-out.** The import job survives but did not finish at ~4 GB;
   capping fan-out in `main`'s edges is the fix. Plan 7's Risks section
   (`docs/superpowers/plans/2026-10-30-scholion-plan-7-overlay-index.md`).
4. **Sparse checkouts**, if workspace disk is the next limiter. Left out of Plan 7 and Plan 11
   (both name it as the next plan).
5. **Checkpoint by patch**: a checkpoint snapshots the whole workspace; recording only a patch is
   not built. See the checkpoint event in `CHANGELOG.md` for what exists.
6. **Adapter verification against the real CLIs.** `av init --for codex|gemini|cursor` is tested
   against fixtures only (`integrations/*/README.md`); Muse Code's settings path is still unknown
   (`integrations/muse/README.md`). Grok is verified.

## 3. Rules that bite

- Branch off `main`, one PR per change; fmt, clippy `-D warnings` and `cargo test --workspace`
  green first (`CLAUDE.md`).
- A merged migration is never edited; check `main`'s highest number before adding one.
- Code flows scholion to agentvcs, never back.
