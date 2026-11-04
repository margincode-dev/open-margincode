# E4 at scale: design (Plan 13, Phase 1)

Status: **harness ready; smoke run 2026-11-04 passed** (`smoke/README.md`, 3 sessions, $0.027 at peak
prices). The full sweep has not been run and still waits for the owner's budget OK.

## The question

E4 measured so far (`../2026-10-29-relay-lean/`, `../2026-10-31-relay-image/`): passing lessons on
cuts cost per solved task by about 41–47 % against no memory, and Scholion's anchored records tie
a hand-written flat `NOTES.md` (av vs notes −1.8 % and −2.5 % cost per accepted task, 30/30 each).
Both runs had ten notes, which a flat file holds easily.

The untested thesis is scale. With hundreds of mostly irrelevant notes:

- a flat `NOTES.md` should get worse: the agent reads all of it every time, the context grows, the
  one note that matters is buried (at N = 500 the file is about 3,000 lines, past the 2,000 lines
  Claude Code's `Read` returns by default);
- anchored records should hold: the session-start pack is token-budgeted (800 tokens in the bench)
  and the pre-read hook only brings records anchored to the file being opened, within 400 tokens.

The run answers: as N grows, does `notes` lose solved tasks or cost more per solved task, and does
`av` not?

## Arms

| arm | what agent B gets | depends on N? |
|---|---|---|
| `cold` | the base tree only (the control) | no: run once |
| `notes` | the ten lessons as commits, plus `NOTES.md` with the ten lessons **and N noise notes**, which `CLAUDE.md` says to read | yes |
| `av` | the ten lessons as commits, plus the ten lessons **and the same N noise notes** as records of an earlier av task, anchored to their files; pack + `--jit-hooks` (pre-read and stop) | yes |

The task set is `bench/tasks-relay` (the ten tallyho relay tasks and `pool/notes.toml`), not
`bench/tasks-relay-scale`: the noise now comes from `--noise N`, not from a fixed pool (below).
Every arm keeps the existing lean-E4 setup otherwise; `--noise 0` is today's run exactly (the
tests check that `NOTES.md` is byte-identical and the report golden is unchanged).

## The noise notes

`scholion-bench run --noise N [--noise-seed S]` adds N notes to every relay task's pool before the
first session (`crates/scholion-bench/src/noise.rs`). `scholion-bench noise REPO --rev REV -n N`
prints them as a pool, with the size they add to `NOTES.md`; `noise-sample-50.toml` here is
`noise /tmp/tallyho --rev main -n 50 --seed 1`.

- **Deterministic, no LLM.** A seeded generator, not mined history: tallyho is one commit on
  `main` (`bench/tallyho/make-repo.sh`), so there is no history to mine. The anchors are the base
  tree's own source files (test files left out) and every function, method and type
  `scholion_index::extract` finds in them: 161 anchors on tallyho `main` (2,898 anchor × template pairs). 18 templates (6 dead
  ends, 6 decisions, 6 notes) of generic engineering lessons: logging, keyword-only arguments,
  caching, thread safety, encodings, deprecation warnings, test fixtures. None touches what the ten
  lessons are about (money, rounding, currencies, taxes, coupons, slugs, ids, events, refunds,
  regions, pagination, error wording).
- **Same text in both arms.** The notes are generated once per (repo, base) and appended to the
  task's `relay_notes`, which the runner already turns into `NOTES.md` sections (`notes`) and
  anchored records (`av`). Noise is never its own commit (the existing `noise = true` rule), so
  the git-log channel is the same at every N.
- **Nested levels.** Every (anchor, template) pair is shuffled once by the seed and the first N are
  taken, so the 50 notes of N = 50 are the first 50 of N = 200 and N = 500. Levels differ only by
  count.
- **Anchored everywhere, the lessons' files included.** This differs from the 290-note pool of
  `tasks-relay-scale` (`pool/gen_scale_notes.py`), which kept noise off the ten lessons' files "so
  the PreToolUse hook is not flooded". That gives av the win by construction: the pack and the hook
  never have to choose. Here noise falls on files in proportion to their symbols, so the hot files
  get the most (at N = 500: `models.py` 58, `money.py` 47, `repository.py` 44, `validate.py` 14),
  and av has to pick the lesson out of them, as it would in a real repo.
- **The lesson is not on top.** The noise is spread through the pool at seeded positions; lessons
  and noise each keep their own order. In `NOTES.md` that means the lesson is somewhere in the
  file, not in the first ten sections. For av, records are created in that order, and
  `records_at` ranks a file's records by kind (dead end, then decision, then note) and newest
  first within 400 tokens, so noise seeded after a lesson can push it out of the hook's answer.
  That is a real weakness the run can expose, not something to tune away before running.

Size of `NOTES.md` (cl100k; the ten lessons are about 0.9k tokens and 63 lines):

| N | noise tokens | `NOTES.md` total | lines |
|---|---|---|---|
| 0 | 0 | 0.9k | 63 |
| 50 | 2.7k | 3.6k | ~360 |
| 200 | 10.7k | 11.6k | ~1,260 |
| 500 | 26.6k | 27.5k | ~3,060 |

## Levels and cells

N ∈ {0, 50, 200, 500}. One cell = one (arm, N): 10 tasks × 3 reps = **30 sessions**, the minimum
before any claim. `cold` does not depend on N and is one cell. Sweep: 1 + 2 × 4 = **9 cells,
270 sessions**.

Rows do not record their N, so each level gets its own `--out` directory; the harness refuses to
resume a results directory written at another noise level, and the report header names the level
(` · noise 200 · `).

## Metrics

Per cell, from `rows.jsonl` and the report:

1. **solved / 30** (hidden pytest passes): the primary outcome.
2. **cost per solved task** (`cost / accepted`), at the prices below.
3. **input tokens per task**, cache reads included (`metrics.input_tokens`, new): the whole context
   the model read, turn after turn. The existing `tokens_total` excludes cache reads and would hide
   most of what a large `NOTES.md` costs.
4. **memory tokens** (`memory_tokens` in each row, new): cl100k size of what the channel hands over
   up front, `NOTES.md` in `notes`, the pack the av task starts with in `av` (`pack.md`). The report adds a "Memory at
   scale" table (median input tokens, median memory tokens, median turns per arm) when `--noise` is
   set.
5. Kept from lean E4: tokens to first edit, turns, repeated dead ends, wall time.

**The cost metric may not separate the arms.** DeepSeek bills a cache read at 2 % of input
(0.006 vs 0.30 $/M at peak). A 27k-token `NOTES.md` is paid in full once, then re-read for almost
nothing: at N = 500 it adds about $0.01 a session (below), on a session of about $0.013. If the flat
file hurts, it should show first in solved / 30, turns and tokens to first edit, and in input
tokens, before it shows in dollars. Read cost per solved task with that in mind; with a model that
caches less cheaply, the same token counts cost more.

**Decision rule (to pre-register with the run).** No significance claim below 30 valid sessions in
a cell. The thesis holds if, from N = 0 to N = 500, `notes` loses solved tasks or its cost per solved
task rises by more than its median-cost CI, while `av` stays within its N = 0 CI on both. If both
arms hold, a flat file is good enough up to 500 notes on this repo, and that is the result. If `av`
degrades too, check `pack.md` and the hook output in the transcripts: whether the lesson was cut by
the 400-token hook cap (an ordering problem) or never ranked (a retrieval problem).

## Model and prices

`deepseek-flash` through Claude Code's Anthropic-compatible endpoint, as in both earlier relay runs,
costed by `--price-per-mtok` (`input,cache_write,cache_read,output`; DeepSeek bills cache writes as
input). Peak `0.30,0.30,0.006,1.20`; off-peak `0.15,0.15,0.003,0.60`. Pick one for the whole sweep
and say which in the PREREG; the two earlier runs used different ones, so compare tokens across
dates, not dollars.

## Cost estimate per cell

From the earlier runs' rows: 2026-10-29 (peak) $1.171 / 90 sessions = **$0.013 a session** (median
$0.010–0.011 per arm); 2026-10-31 (off-peak) $0.590 / 90 = **$0.0066 a session**. The `notes` arm
adds `NOTES.md`: read once at input price, then about 15 turns of cache reads, i.e. per 1k tokens of
file $0.00039 at peak ($0.0002 off-peak). `av` is held roughly flat: its pack is capped at 800
tokens and the hook at 400 per file. The upper column adds 15 % session-to-session spread to every
cell (for `av` that also covers longer hook output) and doubles the `NOTES.md` extra (cache
misses, a second read).

| cell | sessions | peak estimate | peak upper | off-peak estimate |
|---|---|---|---|---|
| `cold` (once) | 30 | $0.39 | $0.45 | $0.20 |
| `notes`, N = 0 | 30 | $0.39 | $0.45 | $0.20 |
| `notes`, N = 50 | 30 | $0.43 | $0.52 | $0.22 |
| `notes`, N = 200 | 30 | $0.53 | $0.71 | $0.27 |
| `notes`, N = 500 | 30 | $0.70 | $1.07 | $0.36 |
| `av`, any N | 30 | $0.39 | $0.45 | $0.20 |
| **sweep, 9 cells** | 270 | **≈ $4.0** | **≈ $5.0** | **≈ $2.0** |

So about **$0.40–0.70 per cell at peak (≤ $1.07), $0.20–0.36 off-peak**. Spend nothing beyond
that without a pilot: run the smoke below, then one `notes` and one `av` cell at N = 500 (the most
expensive and the most informative), check the balance, then the rest.

## To run

### What was missing on 2026-11-04 (why the smoke run did not happen at first)

Later the same day the smoke ran without a Linux host, entirely in Docker Desktop containers
(harness, daemon, Postgres, Qdrant, TEI on one network). `smoke/README.md` has the setup to reuse.

`DEEPSEEK_API_KEY` is set in the environment this branch was written in. The rest of what
`bench/README.md` and `bench/tasks-relay/README.md` require is not there, and was not worked around:

- **No Linux host for the harness.** The earlier runs ran `scholion-bench`, `av` and `claude` in a
  WSL Ubuntu with `bench-work` on `/mnt/c/Users/vinhq/bench-work`. `wsl -l` now lists only
  `docker-desktop`; the `claude` on PATH is the Windows build, and the harness needs `sh`, POSIX
  paths and process groups.
- **No tallyho repo** at `~/bench-repos/tallyho` (`bench/tallyho/make-repo.sh` on that host).
- **No venv with pytest** at `~/bench-work/venv` (each spec's `path_prepend` and `verify_cmd`).
- **No bench daemon stack.** The `scholion-bench` compose project (Postgres, Qdrant, TEI, daemon
  on 7430) is not running. Port 7430 is held by `av-vibe`, the agentvcs dogfood container, which
  must not serve as a bench daemon. A `scholion-daemon:dev` image from 2026-11-03 exists; this
  branch changes nothing in the daemon.
- **Key wiring.** The relay README reads the key from `~/.config/deepseek/key` (absent) into
  `ANTHROPIC_AUTH_TOKEN`; with the variable here, `export ANTHROPIC_AUTH_TOKEN="$DEEPSEEK_API_KEY"`.

### Commands, once that exists

Build and setup as in `bench/README.md` (steps 0–3) and `bench/tasks-relay/README.md` (the
DeepSeek `ANTHROPIC_*` variables); `$COMMON` is the bench stack's `--av-url`, `--work-root`,
`--import-dir` (and `--import-mount` for a host daemon). Each av session at N = 500 posts 510
records and waits for their embeddings; the default `--index-wait-secs 1800` is enough for TEI.

```bash
P="--price-per-mtok 0.30,0.30,0.006,1.20"   # or the off-peak 0.15,0.15,0.003,0.60, for the whole sweep
OUT=bench/results/2026-11-e4-scale

# smoke: one task, one session per arm, N=50, capped at $0.25 (expected ≈ $0.05)
scholion-bench run bench/tasks-relay --only relay-customer-region --arms cold,notes,av \
  --noise 50 --jit-hooks --reps 1 $P --session-cap-usd 0.08 --budget-usd 0.25 \
  --out $OUT/smoke $COMMON --confirm

# the sweep: cold once, then one --out per level (rerun a command to resume it)
scholion-bench run bench/tasks-relay --arms cold --jit-hooks --reps 3 $P \
  --session-cap-usd 0.08 --budget-usd 0.60 --out $OUT/cold $COMMON --confirm
for N in 500 0 50 200; do
  scholion-bench run bench/tasks-relay --arms notes,av --noise $N --jit-hooks --reps 3 $P \
    --session-cap-usd 0.08 --budget-usd 1.50 --out $OUT/n$N $COMMON --confirm
done
```

Without `--confirm` each command prints its plan (sessions per arm, the noise line, the estimate)
and spends nothing. Commit each directory's `rows.jsonl`, `meta.json` and `report.md`, plus a
PREREG written before the first paid session, and check the spend against
`GET https://api.deepseek.com/user/balance` before and after.

## What the harness change is tested for

`cargo test -p scholion-bench` (fake `claude` script, a real daemon for the av arm):

- `noise::*`: anchors are the base tree's source files and symbols, not its tests; the notes are
  deterministic, the first 50 of N = 200 are N = 50, another seed gives other notes; every note is
  a valid record (kind, summary ≤ 200 characters, a body) on a real anchor, distinct even past
  anchors × templates; interleaving keeps both orders and leaves the pool alone at N = 0; every
  task on one base gets the same noise; `--noise` refuses a task without `relay_pool`.
- `runner::noise_adds_n_sections_to_notes_md_and_no_commits`: N = 50 gives `NOTES.md` 50 more
  sections, the same number of commits, and a larger `memory_tokens`.
- `runner::noise_reaches_the_av_arm_as_n_more_records_with_the_notes_arms_text`: the earlier task
  holds 2 + 50 records, and their summaries are exactly the `NOTES.md` sections' summaries.
- `runner::zero_noise_writes_the_same_notes_md_as_no_noise`, and the report golden unchanged.
- `report::a_noise_run_names_its_level_and_reports_memory_size_per_arm`,
  `report::a_run_refuses_to_resume_into_results_of_another_noise_level`,
  `metrics::first_edit_counts_only_tokens_before_it_excluding_cache_reads` (input tokens include
  cache reads).
