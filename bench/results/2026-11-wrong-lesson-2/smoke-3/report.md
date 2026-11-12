# scholion-bench report

Date 2026-11-12 04:44 UTC · claude 2.1.318 (Claude Code) · av unknown · model deepseek-flash · seed 1 · av arms: hooks only · agent isolation: user benchagent · 3 sessions launched, $0.044 spent

Valid sessions per arm: cold 1, notes 1, av 1.
**No significance claim**: fewer than 30 valid sessions in at least one arm; the intervals below are bootstrap 95% CIs of the median and are wide at this n.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same tree with what an earlier agent learned on top, one note per task (the one it needs and the others as noise): as commit messages in every arm but `cold` (which gets none of it and is the control), plus a NOTES.md that CLAUDE.md points at in `notes`, plus anchored records of an earlier task in the av arms. Only the channel differs. The notes were written for the benchmark, so this measures whether recorded knowledge reaches the next agent, not whether agents write it. Repeated dead ends count edits that write the approach the task's note warns about; a decision ignored shows as a failed hidden test.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| cold | 0/1 | n/a | n/a | $0.016 | 29574 | 22381 (0) | 0 |
| notes | 1/1 | $0.016 | 42 s | $0.016 | 29765 | 21907 (0) | 0 |
| av | 1/1 | $0.012 | 34 s | $0.012 | 24651 | 18639 (0) | 0 |

## Attribution (relative change)

| comparison | cost / accepted | median tokens_total | median tokens_to_first_edit | success rate |
|---|---|---|---|---|
| relay: av vs notes | -20.9% | -17.2% | -14.9% | +0.0% |
| relay: notes vs cold | n/a | +0.6% | -2.1% | n/a |
| relay: av vs cold | n/a | -16.6% | -16.7% | n/a |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| relay-order-ids | cold | 1 | 0 | 22381 | 29574 | 0 | $0.016 |
| relay-order-ids | notes | 1 | 1 | 21907 | 29765 | 0 | $0.016 |
| relay-order-ids | av | 1 | 1 | 18639 | 24651 | 0 | $0.012 |
