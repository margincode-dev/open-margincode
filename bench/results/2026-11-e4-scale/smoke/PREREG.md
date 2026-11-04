# Pre-registration: E4-at-scale smoke, 2026-11-04

Written before the first paid session, which started at 16:32:50 UTC.

This is the smoke run from `../DESIGN.md` ("To run"): one task, one session per arm, N = 50.
It checks that the harness, the noise notes, the daemon and the hooks work end to end before any
cell of the sweep is paid for. **It makes no claim about the thesis.** One session per arm is far
below the 30 the decision rule needs.

- Task: `relay-customer-region` (`bench/tasks-relay`). Arms `cold`, `notes`, `av`. Reps 1. Seed 1.
- Noise: `--noise 50`, noise seed 1. `av` is hooks plus `--jit-hooks`, not MCP.
- Model: `deepseek-flash` through `https://api.deepseek.com/anthropic`. No other API key.
- Prices: DESIGN's peak `0.30,0.30,0.006,1.20`. The run starts at 16:32 UTC, inside DeepSeek's
  off-peak window (16:30 to 00:30 UTC), so the real bill may be about half the harness figure.
  The balance delta decides the spend. The sweep should pick one price pair and keep it.
- Budget: `--budget-usd 0.25`, `--session-cap-usd 0.08`. Expected about $0.05. The owner's hard cap
  for this run is $1.00. Starting balance $6.44 (16:32 UTC). Stop if it drops below $5.44.
- Pass: three sessions `status=ok`; the `av` session has a non-empty `hook-count` and a
  `pack.md`; `NOTES.md` in `notes` has 10 + 50 sections; `report.md` names ` · noise 50 · ` and has
  the "Memory at scale" table. Accepted or not is reported and not judged.
- If anything fails, the failure and what was learned go in `NOTES.md` here.
