# scholion-bench report

Date 2026-11-11 17:28 UTC · claude 2.1.318 (Claude Code) · av unknown · model deepseek-flash · seed 1 · av arms: hooks only · 60 sessions launched, $0.892 spent

Valid sessions per arm: notes 30, av 30.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same tree with what an earlier agent learned on top, one note per task (the one it needs and the others as noise): as commit messages in every arm but `cold` (which gets none of it and is the control), plus a NOTES.md that CLAUDE.md points at in `notes`, plus anchored records of an earlier task in the av arms. Only the channel differs. The notes were written for the benchmark, so this measures whether recorded knowledge reaches the next agent, not whether agents write it. Repeated dead ends count edits that write the approach the task's note warns about; a decision ignored shows as a failed hidden test.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| notes | 30/30 | $0.016 | 47 s | $0.013 ($0.011–$0.015) | 25481 (22232–28119) | 18604 (0) | 0 |
| av | 24/30 | $0.017 | 51 s | $0.011 ($0.009–$0.012) | 21901 (19983–22955) | 16025 (0) | 6 |

## Attribution (relative change)

| comparison | cost / accepted | median tokens_total | median tokens_to_first_edit | success rate |
|---|---|---|---|---|
| relay: av vs notes | +8.0% | -14.0% | -13.9% | -20.0% |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| relay-coupon-remove | notes | 6 | 6 | 15578 | 19425 | 0 | $0.009 |
| relay-coupon-remove | av | 6 | 6 | 14024 | 17476 | 0 | $0.007 |
| relay-line-taxes | notes | 6 | 6 | 27814 | 49198 | 0 | $0.034 |
| relay-line-taxes | av | 6 | 6 | 31252 | 48544 | 2 | $0.031 |
| relay-money-plain | notes | 6 | 6 | 16545 | 21030 | 0 | $0.009 |
| relay-money-plain | av | 6 | 6 | 15459 | 20090 | 0 | $0.010 |
| relay-quote-split | notes | 6 | 6 | 18885 | 27211 | 0 | $0.014 |
| relay-quote-split | av | 6 | 0 | 15761 | 21428 | 4 | $0.010 |
| relay-store-count | notes | 6 | 6 | 19130 | 26549 | 0 | $0.013 |
| relay-store-count | av | 6 | 6 | 16169 | 22922 | 0 | $0.012 |
