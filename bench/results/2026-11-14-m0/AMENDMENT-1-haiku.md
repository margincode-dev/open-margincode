# M0, amendment 1: the run moves to Claude Haiku 5.5

**Committed 2026-11-16, before the first Haiku session.** `PREREG.md` is not edited. Where this
file and `PREREG.md` disagree, this file holds for every session from here on. Everything it
does not name stays as `PREREG.md` registered it: the question, the arms, the tasks and the
pool, the decision rule and its bars, the gate, the guard, the secondary measures, the
validity rules, the leak and content checks, and the analysis scripts as committed in `223ae29`.

## Why

At 00:01 UTC on 2026-11-16 the owner moved track 2 (M0, M1 Part 2, M2 stage B) from DeepSeek
to an Anthropic API key: "For track 2, use claude or anthropic api key pls". When asked which
model, the owner chose Claude Haiku 5.5. At that point the smokes and command 1 (`cold`) had run
on DeepSeek, and command 2 (`av-before`, build A) was running. Command 3 had not started, and a
stop file kept it from starting on DeepSeek.

A model change in the middle of M0 would make the arms differ by model, and the primary
compares `av-before` with `av-phase0`. So **M0 runs again in full on Haiku 5.5**: the smokes,
the gate, all four `av`/`notes` arms and the guard. Nothing from the DeepSeek sessions goes into
an M0 result. They are kept under `deepseek/` as a described-only record (see "Results
directories" below). M1 Part 2 and M2 stage B copy "the model and stack as the M0 run", so they
run on Haiku 5.5 as well.

## What changes

1. **Model.** `claude-haiku-5-5` on Anthropic's first-party API. `ANTHROPIC_BASE_URL` is not
   set. The key is passed into the container by name at `docker exec` and given to Claude Code
   as `ANTHROPIC_API_KEY`. It is never written down. `ANTHROPIC_DEFAULT_HAIKU_MODEL`,
   `ANTHROPIC_SMALL_FAST_MODEL` and `CLAUDE_CODE_SUBAGENT_MODEL` are set to `claude-haiku-5-5`,
   so Claude Code's background calls use the same model. Claude Code's own defaults for
   thinking and effort apply. The harness sets neither, as before.
2. **Task specs.** The harness takes `--model` from each task file. The Haiku run tree is
   `/m0/tree-h`, a copy of the run tree (`7dd5208`) in which the `model` line of the 13 task
   files under `bench/tasks-m0/` and `bench/tasks-relay/` reads `model = "claude-haiku-5-5"`.
   Nothing else in the tree differs. `haiku-tree.sh` makes it and checks that `diff -r` shows
   only those 13 lines. The task directories keep their names, so `leak.py`, `analyze.py` and
   `content.py` run unchanged.
3. **Prices.** `--price-per-mtok 0.10,0.125,0.01,0.50`: Haiku 5.5's first-party list prices
   (input, 5-minute cache write at 1.25× input, cache read at 0.1× input, output), for
   prompts up to 100K tokens. Above 100K the list price is $0.50 in and $2.50 out, which the
   harness cannot express. The smokes check the largest single request's prompt (below). Claude
   Code's own `cost_usd` for each session is in `rows.jsonl` too, and the README reports both
   totals.
4. **Fresh stores.** Builds A and B get new daemons with new, empty databases (`m0ha`, `m0hb`).
   They reuse the same ports (7461, 7462), store paths and workspace roots as before, so the
   leak check's paths stay true. The DeepSeek daemons are stopped first. Their stores, their
   workspace roots and their run directories under `/m0/bench-work/runs` are moved into
   `/m0/results/deepseek/`, which `leak.py` already counts as `source tree`. The Haiku run ids
   are then the DeepSeek ones again (`m0cold`, …) without colliding. The store export after
   commands 2 and 3 reads only the Haiku databases.
5. **Stopping rules.** The session cap ($0.08), the per-command budgets ($0.30, $0.10, $0.40,
   $0.70, $1.40, $0.60) and `max_turns` 30 are unchanged. The $0.50 balance floor was a DeepSeek
   prepaid-balance rule, and it is dropped. The Anthropic key has an owner-set monthly cap,
   tracked outside this repository. A command starts only if what has been spent this month
   plus the command's budget stays under that cap.
6. **The smokes decide whether to go on.** The smokes run first, as registered. Besides the
   registered pass conditions, they must show (a) that Claude Code 2.1.318 runs
   `claude-haiku-5-5` at all, (b) no single request with a prompt above 100K tokens, and (c) no
   session above $0.04 (half the cap). If (b) or (c) fails, the run stops before command 1 and
   the owner is asked. Raising the cap or the budgets would be a second amendment, committed
   before command 1.
7. **The price window** deviation does not apply: Anthropic has a single price. Commands 2 and
   3 still run back to back, as registered.

## Cost estimate

The DeepSeek cells cost $0.009 to $0.015 a session at $0.30 / $1.20 per MTok. Haiku 5.5 lists
a third of DeepSeek's input price and under half its output price, while cache reads cost more
($0.01 against $0.006), and Claude Code sends Anthropic models a longer system prompt. The
smokes will measure the real cost. Until then, the guess is about the same per session as on
DeepSeek: **about $2 for the 154 sessions**. The registered budgets, $3.50, remain the ceiling.

## Results directories

- `deepseek/`: what ran on DeepSeek. That is `smoke-a/`, `smoke-b/`, `m0-cold/` (the gate passed
  30/30) and `m0-av-before/` (command 2, which finished or stopped as the README says). These
  are described only and go into no M0 table.
- The Haiku run writes the directory names `PREREG.md` registered (`smoke-a/`, `smoke-b/`,
  `m0-cold/`, `m0-av-before/`, `m0-wrong/`, `m0-guard/`). The scripts read them as committed.
