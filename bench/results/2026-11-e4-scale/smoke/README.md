# E4-at-scale smoke, 2026-11-04

The smoke run from `../DESIGN.md` ("To run"). One task, one session per arm, N = 50. It checks
the pipeline and makes **no claim about the thesis**. `PREREG.md` was written before the run.
`report.md`, `rows.jsonl` and `meta.json` are the harness's own output, copied unchanged.

**Pass.** All 3 sessions `status=ok` and valid. `NOTES.md` in `notes` had 60 sections (10 lessons + 50
noise, 364 lines, 3,570 cl100k tokens; DESIGN predicted about 3.6k and about 360 lines). The `av` session's
`hook-count` is non-empty (1 session-start, 8 pre-read, 4 post-edit, 2 stop) and it has a `pack.md`.
The report header says ` · noise 50 · ` and the report has the "Memory at scale" table.

## Result (n = 1 per arm, not a result)

| arm | solved | cost (peak prices) | wall | turns | input tokens (with cache reads) | memory tokens | tokens to first edit |
|---|---|---|---|---|---|---|---|
| cold | 0/1 | $0.0075 | 34 s | 13 | 126,500 | n/a | 16,166 |
| notes | 1/1 | $0.0107 | 37 s | 18 | 222,518 | 3,570 | 20,265 |
| av | 1/1 | $0.0090 | 47 s | 15 | 167,132 | 711 | 15,695 |

`cold` failed 4 of the hidden tests: it echoed the rejected region in the error, which is the trap this task
sets. That matches cold's 0/3 on this task on 2026-10-29 and 2026-10-31.

## Spend

- Harness total **$0.027** at the peak prices `0.30,0.30,0.006,1.20`.
- DeepSeek balance **$6.44 before (16:32 UTC), $6.42 a few minutes after the run**: **$0.02 spent**,
  well under the owner's $1.00 cap. Right after the run (16:35 UTC) the balance still read $6.44, so
  billing lags by a few minutes. Wait before reading the "after" balance. The endpoint reports whole
  cents. The run fell in DeepSeek's off-peak window (16:30 to 00:30 UTC), so the harness's peak-price
  figure is an upper bound.
- Per session that is $0.008 to $0.011 at peak prices, in line with DESIGN's $0.013 a session.

## Exact command

From the repo root of a clone of this branch at `669080e`, inside the harness container (below):

```bash
export PATH=/target/release:$PATH
unset ANTHROPIC_API_KEY
export ANTHROPIC_BASE_URL=https://api.deepseek.com/anthropic
export ANTHROPIC_AUTH_TOKEN="$DEEPSEEK_API_KEY"   # passed in with `docker exec -e DEEPSEEK_API_KEY`, never written down
export ANTHROPIC_DEFAULT_HAIKU_MODEL=deepseek-flash ANTHROPIC_SMALL_FAST_MODEL=deepseek-flash
export CLAUDE_CODE_SUBAGENT_MODEL=deepseek-flash
P="--price-per-mtok 0.30,0.30,0.006,1.20"
OUT=bench/results/2026-11-e4-scale
COMMON="--av-url http://e4-daemon:7450 --work-root /root/bench-work --import-dir /root/bench-work/src --import-mount /root/bench-work/src"
scholion-bench run bench/tasks-relay --only relay-customer-region --arms cold,notes,av \
  --noise 50 --jit-hooks --reps 1 $P --session-cap-usd 0.08 --budget-usd 0.25 \
  --out $OUT/smoke $COMMON --confirm
```

Before that, with nothing paid: `reference/check.py` reported all 10 pairs ok, `scholion-bench check
bench/tasks-relay` reported all 10 tasks ok, and the same command without `--confirm` printed the plan
(3 sessions, the noise line, a $0.24 worst case).

## Setup (no Linux host, all in Docker Desktop)

There is no WSL distro with the old bench setup, so everything runs in containers on one network,
`e4-bench`. No ports are published, so 7430 (`av-vibe`) is not involved.

| container | image | role |
|---|---|---|
| `e4-pg` | `postgres:16`, fresh volume `e4-bench-pg` | daemon database |
| `e4-qdrant` | `qdrant/qdrant:v1.13.6`, no volume | semantic layer |
| `e4-tei` | `text-embeddings-inference:cpu-1.7.2`, `BAAI/bge-small-en-v1.5`, model cache from `scholion_av-tei` | embedder |
| `e4-daemon` | `scholion-daemon:dev` `sha256:eca3ec27…` (built 2026-11-03T19:57:53Z), on `0.0.0.0:7450` | bench daemon, semantic layer on |
| `e4-harness` | `rust:1.97.1-bookworm` + Node 22 + Claude Code + python3-venv + jj, `--init` | `scholion-bench`, `av`, `claude`, tallyho, the venv |

- The harness and daemon both mount volume `e4-bench-work` at `/root/bench-work`, so `AV_WS_ROOT`,
  `AV_WS_HOST_ROOT` and the import folder are the same path on both sides. Daemon env:
  `AV_DATABASE_URL=postgres://av:av-dev-only@e4-pg:5432/av`, `AV_STORE_ROOT=/store/jj` (volume
  `e4-bench-store`), `AV_WS_ROOT=AV_WS_HOST_ROOT=/root/bench-work/ws`, `AV_QDRANT_URL=http://e4-qdrant:6334`,
  `AV_EMBED_URL=http://e4-tei:80`, `AV_LISTEN=0.0.0.0:7450`, `HOME=/tmp`.
- Source: `git clone --branch e4-smoke` of the host repo, mounted read-only, into `/src`. A real clone
  is needed: the Windows worktree's `.git` file points at a `D:/` path, and `meta.json`'s `av_rev` would
  then come out as `unknown`. `cargo build --release -p scholion-bench -p scholion-cli` with
  `CARGO_TARGET_DIR=/target` on the reused `e4-target` volume.
- `bench/tallyho/make-repo.sh` wrote `/root/bench-repos/tallyho`; its `main` hash depends on the build
  time (`cddaf0e` here). The noise is anchored on files and symbols, so the hash does not matter.
  Venv `/root/bench-work/venv` with pytest 9.2.0 (Python 3.11.2).
- The harness image, built locally as `scholion-e4-harness:local` (no secrets in it):

```dockerfile
FROM rust:1.97.1-bookworm
COPY --from=node:22 /usr/local/bin/node /usr/local/bin/node
COPY --from=node:22 /usr/local/lib/node_modules /usr/local/lib/node_modules
RUN ln -s /usr/local/lib/node_modules/npm/bin/npm-cli.js /usr/local/bin/npm \
 && ln -s /usr/local/lib/node_modules/npm/bin/npx-cli.js /usr/local/bin/npx \
 && apt-get update && apt-get install -y --no-install-recommends python3 python3-venv curl ca-certificates \
 && rm -rf /var/lib/apt/lists/* \
 && npm i -g @anthropic-ai/claude-code \
 && git config --system --add safe.directory '*'
RUN curl -fsSL https://github.com/jj-vcs/jj/releases/download/v0.46.0/jj-v0.46.0-x86_64-unknown-linux-musl.tar.gz \
    | tar -xz -C /usr/local/bin ./jj
```

Versions: Claude Code 2.1.318 (the two earlier relay runs used 2.1.316), Node 22.23.4, rustc 1.97.1,
jj 0.46.0, git 2.39.5, harness and `av` at `669080e`, model `deepseek-flash`.

## What the smoke showed for the sweep

- **The lesson did not fit in the av pack.** At N = 50 the session-start pack (711 of 800 tokens) gave
  its "Decisions & dead ends" slots to four noise records. The customer-region lesson was the first of 48
  "omitted lower-ranked items", and only its truncated one-line summary was shown. The agent picked it up
  from there or from the pre-read hook on `validate.py`; its reasoning mentions the echoed value. This is
  the ordering weakness DESIGN expects ("the lesson is not on top"), already visible at N = 50.
- **The pre-read hook's output is not in the transcript.** `events.jsonl` keeps the SessionStart hook's
  output (`hook_response`) but nothing from PreToolUse hooks. `hook-count` only counts calls. DESIGN's
  fallback for an `av` loss ("check `pack.md` and the hook output in the transcripts") cannot be done for
  the pre-read channel as the harness stands. Running `av hook pre-read` again by hand after the session
  printed nothing; the task had been abandoned by then, and the cause was not investigated further. If that diagnosis matters, have the hook command
  in `settings()` tee its stdout next to `hook-count` before the sweep. That is a harness change with a
  test, and is left out of this PR.
- The progress line prints `spent $-0.00` before the first session. It is cosmetic and was left alone.
- The containers above were removed after the run. The volumes `e4-target`, `e4-bench-work`, `e4-bench-pg`
  and `e4-bench-store`, and the image `scholion-e4-harness:local`, were kept, so the next run only needs
  `docker run` for the five containers. Use a new `--out` for each level. `e4-bench-pg` and
  `e4-bench-store` hold this smoke's repos, which is harmless because repo names carry the run name.
