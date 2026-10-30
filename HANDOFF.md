# Handoff — 2026-10-30

State of Scholion after the cloud-agent session that built E4-at-scale. Delete this file once
you've acted on it (ran the scale relay, or decided the next item).

## 1. Where things stand

- **Default branch is `main`.** Plans 1–5 are built. Plan 6 evaluation is in progress. Plan 7
  groundwork is on `main` (`ScopeView`, migration `0021`, differential test still `#[ignore]`d
  until overlay reads/writes land — Tasks 3–5 of
  `docs/superpowers/plans/2026-10-30-scholion-plan-7-overlay-index.md`).
- **Since the 2026-10-29 handoff:** extension points, queued starts, import jobs, lite mode,
  `av init` for Codex/Grok/Gemini (#24); start-queue cancel-before-create (#27); Community scale
  re-run on tokio **32/32 merged** after the queue (#28); adapter contract test; cross-vendor
  `scripts/relay-demo.sh`.
- **This branch / PR:** E4-at-scale harness (not yet run against DeepSeek — no key in the
  cloud agent). See §1a.

### 1a. E4 at scale — harness ready, run pending

Lean E4 (`bench/results/2026-10-29-relay-lean/`) showed av matching `NOTES.md` on cost with ten
notes; it did not show av beating a plain file. The missing test is hundreds of notes, most
irrelevant.

| piece | where |
|---|---|
| Pool | `bench/tasks-relay/pool/notes-scale.toml` — 10 signal + 290 noise (`noise = true`) |
| Generator | `bench/tasks-relay/pool/gen_scale_notes.py` (re-run instead of editing noise by hand) |
| Task set | `bench/tasks-relay-scale/` (same ten tasks, shared files via `../tasks-relay/`) |
| How to run | `bench/tasks-relay-scale/README.md` |

Noise fills `NOTES.md` and av records; it is **not** one empty git commit each (`RelayNote.noise`).
Noise anchors avoid the ten signal (path, symbol) pairs so PreToolUse on a task file is not flooded.

**Attempted 2026-10-30 (cloud agent):** harness is green without a key; the paid run is not.

- Lite daemon on `:7430`, tallyho repo, pytest venv, Claude Code 2.1.314 installed.
- `scholion-bench check bench/tasks-relay-scale` — all 10 ok.
- Pilot session *did* launch: `NOTES.md` had **300** sections; git history stayed at **12**
  commits (base + 10 signal + NOTES.md commit); av earlier-work task got **`record_count`: 301**.
- Claude then exited immediately: `apiKeySource: none`, stderr
  `[claude-code:unrecognized_model] {"model":"deepseek-flash"}`. No DeepSeek key in this
  environment (`~/.config/deepseek/key` missing, no `ANTHROPIC_AUTH_TOKEN`).
- Host-daemon tip: pass `--import-mount` equal to `--import-dir` (default `/import` is the
  compose mount). Use `--allow-no-semantic` unless Qdrant+embedder are up.
- Pack peek on the failed av smoke: start pack’s “Decisions & dead ends” led with **noise**
  (`ValidationError` attempts), not the customer-region lesson — so when you run for real,
  watch whether targeted delivery still wins or noise crowds the pack.

**Your job:** put a DeepSeek key at `~/.config/deepseek/key` (or export `ANTHROPIC_AUTH_TOKEN`),
then run the pilot and full `notes,av` set per `bench/tasks-relay-scale/README.md`. Commit
`bench/results/<date>-relay-scale/` and say whether av beats `notes`. Check
`GET https://api.deepseek.com/user/balance` first.

### 1b. Lean E4 result (unchanged, for context)

| arm | accepted | cost / accepted | wall / accepted |
|---|---|---|---|
| cold | 18/30 | $0.022 | 63 s |
| notes | 30/30 | $0.013 | 36 s |
| av | 30/30 | $0.013 | 38 s |

Recorded knowledge pays (~42 % cheaper than cold). av ≈ notes at ten notes.

### Not verified by hand

Hooks still not exercised in an interactive Claude Code session. E4-at-scale not run (no API key
here). Plan 7 overlay reads/writes not started.

## 2. Kick-start on your machine

Prereqs: `rustup` (1.97.1 from `rust-toolchain.toml`), Docker + Compose, `git`, `jq`. Optional:
`ollama` + `qwen3-embedding`.

```bash
git clone git@github.com:vinhqbui/scholion.git && cd scholion
git checkout <this-branch>   # or main once merged

cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

export UID GID=$(id -g) AV_IMPORT_ROOT=~/projects AV_WS_ROOT=~/ws
docker compose -f deploy/docker-compose.yml up -d --build
cargo build --release -p scholion-cli
scripts/smoke.sh
```

Host daemon against compose Postgres/Qdrant: see the previous handoff pattern (`av_host` DB,
one daemon per database — E18).

Gotchas:
- Qdrant tests need `AV_TEST_QDRANT_URL=http://127.0.0.1:6334`.
- New migration? Next number is **`0022`** (0021 is overlay), and add its hash to
  `tests/migrations_frozen.rs`.

## 3. What to improve next — ranked

1. **Run E4-at-scale** (§1a) — decides whether av earns its place over `NOTES.md`.
2. **Plan 7 Tasks 3–5** — overlay reads/writes, drop `clone_scope` from start; Community scale
   said copying is still the limit even with the start queue.
3. **Adoption** (C9 git notes on merge, C11 no-Docker solo, C12 plugin) if scale shows a gain
   worth installing for.
4. `tasks-scholion` / E5 when there is disk and budget.

Older ranked list (retrieval PageRank/rerank, Mergiraf, multi-instance locks, …) remains in git
history of the 2026-10-29 handoff if you need it.
