# scholion-bench report

Date 2026-11-16 00:39 UTC · claude 2.1.318 (Claude Code) · av unknown · model claude-haiku-5-5 · seed 1 · av arms: hooks only · agent isolation: user benchagent · 60 sessions launched, $0.321 spent

Valid sessions per arm: notes 30, av 30.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same tree with what an earlier agent learned on top, one note per task (the one it needs and the others as noise): as commit messages in every arm but `cold` (which gets none of it and is the control), plus a NOTES.md that CLAUDE.md points at in `notes`, plus anchored records of an earlier task in the av arms. Only the channel differs. The notes were written for the benchmark, so this measures whether recorded knowledge reaches the next agent, not whether agents write it. Repeated dead ends count edits that write the approach the task's note warns about; a decision ignored shows as a failed hidden test.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| notes | 12/30 | $0.011 | 45 s | $0.005 ($0.004–$0.005) | 16421 (15049–17946) | 10276 (1) | 16 |
| av | 30/30 | $0.006 | 25 s | $0.006 ($0.006–$0.007) | 19146 (17561–20293) | 9458 (0) | 0 |

## Attribution (relative change)

| comparison | cost / accepted | median tokens_total | median tokens_to_first_edit | success rate |
|---|---|---|---|---|
| relay: av vs notes | -45.0% | +16.6% | -8.0% | +150.0% |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| relay-bus-resubscribe | notes | 10 | 0 | 9551 | 12886 | 11 | $0.003 |
| relay-bus-resubscribe | av | 10 | 10 | 8518 | 15851 | 0 | $0.005 |
| relay-quote-split | notes | 10 | 2 | 13306 | 20095 | 5 | $0.005 |
| relay-quote-split | av | 10 | 10 | 11739 | 20798 | 0 | $0.007 |
| relay-tax-with-rate | notes | 10 | 10 | 10335 | 16421 | 0 | $0.005 |
| relay-tax-with-rate | av | 10 | 10 | 9359 | 19664 | 0 | $0.007 |
