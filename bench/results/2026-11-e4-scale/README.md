# E4 at scale: full sweep, 2026-11-10

The sweep from `DESIGN.md`, pre-registered in `PREREG.md` (committed and pushed at 03:43 UTC,
before the first paid session; Amendment 1 and two dated notes added during the run). Levels
N = 0, 50 and 500 irrelevant notes. N = 200 was dropped before any of its sessions ran
(Amendment 1). Every cell has 30 valid sessions out of 30. `cold/`, `n0/`, `n50/` and `n500/` hold
the harness's own `rows.jsonl`, `meta.json` and `report.md`, copied unchanged (checked by sha256
against the files in the run container).

## Answer

**As N grew from 0 to 500, the flat `NOTES.md` did not lose solved tasks, but it got more
expensive: cost per accepted task rose 87.7 % ($0.0126 → $0.0237) and the median session cost moved
outside its N = 0 interval. The anchored records held: av's cost per accepted task rose 14.0 %
($0.0125 → $0.0143), and its N = 500 median session cost stayed inside its N = 0 interval. By the
pre-registered rule, the thesis holds on cost. It does not hold on solved tasks: both arms solved
30/30 at every level, so this task set cannot show a loss of solved tasks.**

At N = 0 the two arms are equal (av vs notes −1.1 % cost per accepted task, as in the earlier
relay runs). At N = 50 the gap is small (−6.4 %). At N = 500 it is −39.9 %.

## Results

Accepted means the task's hidden tests pass after the session. Costs are at DeepSeek's peak prices
`0.30,0.30,0.006,1.20` $/M tokens (input, cache write, cache read, output) for every session.
The Wilson 95 % interval is computed outside the report, with the formula of
`scholion_bench::stats::wilson_interval`. Cost per accepted task, median cost with its bootstrap
interval (2000 resamples, seed 1) and memory tokens come from the harness. Memory tokens are the
median cl100k size of `NOTES.md` in `notes`, or of the av task's first pack in `av`.

| N | arm | accepted / 30 | Wilson 95 % | cost / accepted | total cost | median session cost (95 % CI) | median memory tokens | median input tokens (with cache reads) |
|---|---|---|---|---|---|---|---|---|
| (any) | cold | 18/30 | 0.423–0.754 | $0.0216 | $0.3891 | $0.010 ($0.008–$0.014) | n/a | 161,977 |
| 0 | notes | 30/30 | 0.886–1.000 | $0.0126 | $0.3793 | $0.010 ($0.010–$0.013) | 858 | 186,767 |
| 0 | av | 30/30 | 0.886–1.000 | $0.0125 | $0.3751 | $0.011 ($0.009–$0.013) | 708 | 240,737 |
| 50 | notes | 30/30 | 0.886–1.000 | $0.0139 | $0.4168 | $0.011 ($0.010–$0.015) | 3,570 | 228,393 |
| 50 | av | 30/30 | 0.886–1.000 | $0.0130 | $0.3900 | $0.010 ($0.009–$0.012) | 706 | 190,650 |
| 500 | notes | 30/30 | 0.886–1.000 | $0.0237 | $0.7120 | $0.021 ($0.018–$0.025) | 27,487 | 439,830 |
| 500 | av | 30/30 | 0.886–1.000 | $0.0143 | $0.4276 | $0.011 ($0.010–$0.016) | 710 | 228,843 |

The harness rounds dollars to three places in `report.md`. Cost per accepted task here has four
places, computed from the same rows (total cost / accepted).

Other measures, from the reports:

| N | `relay: av vs notes` cost / accepted | median tokens_total | median tokens to first edit | median turns notes / av |
|---|---|---|---|---|
| 0 | −1.1 % | −2.4 % | −10.8 % | 18 / 18 |
| 50 | −6.4 % | −14.2 % | −20.0 % | 17 / 17 |
| 500 | −39.9 % | −52.1 % | −61.3 % | 19 / 19 |

`notes`' median tokens to first edit went from 18,538 (N = 0) to 45,180 (N = 500): the agent reads the
27k-token file before it edits. av's went from 16,542 to 17,496. No session in any cell repeated a dead end.

## The pre-registered decision rule, applied

N = 0 against N = 500, within each arm (`PREREG.md`, "Decision rule"):

| test | N = 0 | N = 500 | outcome |
|---|---|---|---|
| `notes` loses solved tasks: N = 500 rate below the N = 0 Wilson lower end | 0.886–1.000 | 1.000 | **no** |
| `notes` costs more: N = 500 median cost above the N = 0 CI upper end | $0.010–$0.013 | $0.021 | **yes** |
| `av` holds on solved tasks: N = 500 rate inside the N = 0 Wilson interval | 0.886–1.000 | 1.000 | **yes** |
| `av` holds on cost: N = 500 median cost inside the N = 0 CI | $0.009–$0.013 | $0.011 | **yes** |

H1 holds: `notes` costs more and `av` holds. The stricter check also passes. `notes`' two
median-cost intervals do not overlap ($0.010–$0.013 at N = 0, $0.018–$0.025 at N = 500), so that
rise is the one difference called significant. The bootstrap treats the 30 sessions of a cell as
independent, though they are 10 tasks × 3 reps, so the intervals are somewhat narrower than a
task-clustered bootstrap would give. av's intervals overlap ($0.009–$0.013 and $0.010–$0.016), so
no change is claimed for av. The rise in cost per accepted task (+87.7 % notes, +14.0 % av) has no interval in the harness and is reported as a number only. No paired or
task-stratified test was run.

## Caveats

- **Ceiling on solved tasks.** Both channels solved every task at every level, so the solved-task
  half of the thesis was not tested. The lessons also reach `notes` and `av` as commit messages,
  the same at every N (DESIGN), and `cold` solved 18/30 without them. At N = 500 `NOTES.md` (about
  3,060 lines) is already past the 2,000 lines Claude Code's `Read` returns by default, and `notes`
  still solved 30/30. Testing whether a flat file loses solved tasks needs harder tasks or larger N.
- **Cache reads are cheap on DeepSeek** (2 % of input), as DESIGN warned, so dollars understate the
  context difference. In input tokens with cache reads, `notes` grew from 186,767 to 439,830 per
  session (×2.35), and av's went from 240,737 to 228,843. At N = 0 av read more input tokens than
  `notes` (240,737 vs 186,767) at the same cost per accepted task. With a model that caches less
  cheaply, the same token counts cost more.
- **The lesson is often not in av's first pack.** The pack (budget 800 tokens) had a "Decisions &
  dead ends" section in 18 of 30 sessions at each level. The task's own lesson was in `pack.md` in
  12/30 sessions at N = 0, 12/30 at N = 50 and 6/30 at N = 500. The measure is the first 50
  characters of the lesson's summary found anywhere in `pack.md`; it would miss a summary the pack
  cut to fewer than 50 characters. Otherwise the agent gets the lesson from the pre-read hook or
  the commit messages. The hook's output is not in the transcript
  (`smoke/README.md`), so which of the two it was cannot be told.
- **One repo, one model, ten tasks**: tallyho, `deepseek-flash`, Claude Code 2.1.318, seeded noise
  templates (DESIGN, "The noise notes").
- **N = 200 was not run** (Amendment 1, budget). N = 50 is the only point between the ends.
- **Interruptions.** The run was interrupted twice (PREREG notes), and an interrupted session was
  re-run each time:
  - a host power loss at 04:29–12:00 UTC, during `n500`;
  - a planned host restart at about 13:00 UTC, during `n0`.

  After the second, the half-imported av repo of the session in flight had to be removed by hand,
  with the daemon stopped. Scholion has no way to remove a repo. The rows and Qdrant ids of that
  repo are in `n0/stale-repo-removed.json`. The Postgres and Qdrant counts fell by exactly that
  repo's 12 records and 166 code chunks. Because of that, `n50` ran (complete) between `n0`'s
  first 23 sessions and its last 37, instead of after `n0`.
- **`meta.json`'s `date`** is the start of the last resume of that cell. The harness rewrites
  `meta.json` on every run of a command, so `n500`'s says 12:07 UTC and `n0`'s 15:08 UTC.
- One `n0` av session (`relay-coupon-remove`, rep 3) is flagged `truncated`: its stream has no
  `result` event. It is valid and accepted. Its cost ($0.0054) is priced from the summed message
  usage, not the cap, and it is counted as the harness wrote it.

## Spend

| | |
|---|---|
| harness total (peak prices) | **$3.0899** (cold $0.3891, N = 500 $1.1396, N = 0 $0.7544, N = 50 $0.8068) |
| DeepSeek balance before | $5.82 (2026-11-10 03:38 UTC) |
| DeepSeek balance after | $3.48 (15:40 UTC, 7 minutes after the last session; $3.49 at 15:35) |
| balance delta | **$2.34** |

The balance endpoint reports whole cents and lags by a few minutes. The delta includes the two
sessions cut off by the interruptions, which have no row. It may also include other use of the
same key in the same hours. That makes $2.34 an upper bound on this sweep's bill. The bill is lower than the harness
figure even though every session ran at peak hours (03:43–04:29 and 12:07–15:33 UTC; the off-peak
window is 16:30–00:30 UTC). So the harness's peak-price figure is an upper bound too, not the
bill. Possible reasons, not investigated: DeepSeek may count more cache hits than Claude Code
reports, or price `deepseek-flash` below the flags. The stopping rules were never reached: each command ran far
under its budget ($0.60 for `cold`, $1.50 per level), and the balance stayed far above the $0.50
floor.

## Exact commands

Inside the harness container, from the root of a clone of this branch at `fbc5bb0` (`main`). The
key is passed in with `docker exec -e DEEPSEEK_API_KEY` and never written down:

```bash
export PATH=/target/release:$PATH
unset ANTHROPIC_API_KEY
export ANTHROPIC_BASE_URL=https://api.deepseek.com/anthropic
export ANTHROPIC_AUTH_TOKEN="$DEEPSEEK_API_KEY"
export ANTHROPIC_DEFAULT_HAIKU_MODEL=deepseek-flash ANTHROPIC_SMALL_FAST_MODEL=deepseek-flash
export CLAUDE_CODE_SUBAGENT_MODEL=deepseek-flash
P="--price-per-mtok 0.30,0.30,0.006,1.20"
OUT=bench/results/2026-11-e4-scale
COMMON="--av-url http://e4-daemon:7450 --work-root /root/bench-work --import-dir /root/bench-work/src --import-mount /root/bench-work/src"
scholion-bench run bench/tasks-relay --arms cold --jit-hooks --reps 3 $P \
  --session-cap-usd 0.08 --budget-usd 0.60 --out $OUT/cold $COMMON --confirm
for N in 500 0 50; do      # as pre-registered; n0 was finished after n50 (see Caveats)
  scholion-bench run bench/tasks-relay --arms notes,av --noise $N --jit-hooks --reps 3 $P \
    --session-cap-usd 0.08 --budget-usd 1.50 --out $OUT/n$N $COMMON --confirm
done
```

`n500` was run twice and `n0` three times. Each later run was the same command, resuming after an
interruption. Free checks before the first paid session: `reference/check.py` passed all 10 pairs,
`scholion-bench check bench/tasks-relay` passed all 10 tasks, and `scholion-bench noise … -n 50
--seed 1` is byte-identical to `noise-sample-50.toml`. Every command printed its plan without
`--confirm`.

## Setup

This is the setup of `smoke/README.md`, rebuilt, on a Windows 11 host with Docker Desktop and no
WSL distro. Five containers ran on the network `e4-bench`, with no published ports:

| container | image | role |
|---|---|---|
| `e4-pg` | `postgres:16`, fresh volume `e4-bench-pg` | daemon database |
| `e4-qdrant` | `qdrant/qdrant:v1.13.6` | semantic layer |
| `e4-tei` | `ghcr.io/huggingface/text-embeddings-inference:cpu-1.7.2`, `BAAI/bge-small-en-v1.5`, model cache from the existing `scholion_av-tei` volume | embedder |
| `e4-daemon` | `scholion-e4-harness:local` running `/target/release/scholion-daemon`, built from `fbc5bb0` in the harness, as root, on `0.0.0.0:7450` | bench daemon, semantic layer on |
| `e4-harness` | `scholion-e4-harness:local` (the smoke's Dockerfile, `rust:1.97.1-bookworm` + Node 22 + Claude Code **pinned to 2.1.318** + python3-venv + jj 0.46.0) | `scholion-bench`, `av`, `claude`, tallyho, venv |

- The daemon was not the smoke's `scholion-daemon:dev` image, which no longer existed. It was this
  run's own build of `scholion-daemon` from `fbc5bb0` (`cargo build --release --locked -p
  scholion-bench -p scholion-cli -p scholion-daemon`, `CARGO_TARGET_DIR=/target` on the volume
  `e4-bench-target`). It ran in the harness image and was the same binary for the whole sweep.
  `av` and the harness are at `fbc5bb0` too, where the smoke was at `669080e`. Changes since then
  include #104 (full first pack at start-up) and the disk floor (#128). `GET /v1/disk?fs=1` said
  `below_floor: false` (842 GB free), so the floor was left at its default.
- Daemon env as in the smoke: `AV_DATABASE_URL=postgres://av:av-dev-only@e4-pg:5432/av`,
  `AV_STORE_ROOT=/store/jj` (volume `e4-bench-store`), `AV_WS_ROOT=AV_WS_HOST_ROOT=/root/bench-work/ws`,
  `AV_QDRANT_URL=http://e4-qdrant:6334`, `AV_EMBED_URL=http://e4-tei:80`, `AV_LISTEN=0.0.0.0:7450`,
  `HOME=/tmp`. The harness and the daemon both mount `e4-bench-work` at `/root/bench-work`.
- Source: `git clone --branch e4-scale-sweep` of the host repo (mounted read-only) into the volume
  `e4-bench-src`. A real clone gives `meta.json` the correct `av_rev`. `bench/tallyho/make-repo.sh`
  made `/root/bench-repos/tallyho` (`main` = `0440e87`; the hash depends on the build time and does
  not matter, since the noise is anchored on files and symbols). The venv `/root/bench-work/venv`
  has pytest 9.2.0 on Python 3.11.2.
- Versions: Claude Code 2.1.318, Node 22, rustc 1.97.1, jj 0.46.0, model `deepseek-flash`.
- Check of the first av session at N = 500, before the rest of that cell ran: `status=ok`; the
  hook-count was session-start 1, pre-read 3, post-edit 1, stop 2; a `pack.md` (651 of 800 tokens)
  was written; the earlier task held 511 records (166 attempts, 179 decisions, 165 notes: 10 lessons and 500 noise, plus the intent); `notes`'
  `memory_tokens` was 27,487, as DESIGN predicted (27.5k).
