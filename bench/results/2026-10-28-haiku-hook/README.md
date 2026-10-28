# Haiku runs, 2026-10-28: does a smaller session-start pack pay off?

The follow-up that `2026-10-27-haiku/README.md` asked for: `av hook session-start --no-map
--max-code N` against the shipped hook defaults. Same setup as before: `claude-haiku-5-5`,
`bench/tasks-tallyho` (4 tasks × 10 reps), a local daemon built from this branch without the
semantic layer (`--allow-no-semantic`), `--no-av-mcp` (hooks only, as shipped).

The three runs ran side by side, so they saw the same API latency:
- `01-defaults`: `baseline` and `av` with the hook's defaults (map, signatures, about 12 code items);
- `02-nomap-max3`: `av` with `--hook-arg=--no-map --hook-arg=--max-code=3`;
- `03-nomap-max5`: `av` with `--hook-arg=--no-map --hook-arg=--max-code=5`.

All 160 sessions were accepted, for $1.14 in total. Tables from `../2026-10-27-haiku/paired.py`,
with `01-defaults`' baseline as the base for every arm:

| av vs baseline | defaults | no map, 3 code items | no map, 5 code items |
|---|---|---|---|
| cost | −3.3% [−8.8, +2.8] | +0.6% [−6.3, +8.0] | −0.4% [−6.6, +6.2] |
| tokens (no cache reads) | +1.3% [−3.9, +8.0] | −0.5% [−5.9, +4.8] | +0.7% [−4.1, +5.9] |
| turns | −11.7% [−17.5, −5.2] | −11.9% [−19.1, −4.4] | −9.3% [−16.1, −1.0] |
| wall time | −14.2% [−22.5, −0.5] | −4.0% [−14.5, +12.9] | −6.0% [−15.7, +9.5] |
| `Read` calls per session (baseline 1.27) | 1.87 | 1.25 | 1.40 |

To reproduce the `Read` row, count `Read` tool uses in each session's `events.jsonl`. The
transcripts stay in `~/bench-work/runs`.

## What it says

- The smaller pack fixes the extra file opening it was aimed at. With three code items, av agents
  read as many files as baseline agents.
- It does not make the session cheaper. Every arm matches baseline on cost and tokens, and every
  interval contains zero. All three save the same 9 to 12% of turns.
- The defaults stay as they are. On a 25-file repo, opening fewer files is worth little, because
  each file is small. The flags remain available for larger repos.
- The defaults did better than in `04-final-shipped-*` (tokens +1.3% against +5.9%). That run
  used a different build and ran at a different time, so this is not a like-for-like comparison.
- The open measurement is still `bench/tasks-scholion` (20k lines of Rust), where a pack can save
  more navigation.
