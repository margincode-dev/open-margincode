# scholion-bench report

Date 2026-11-12 06:58 UTC · claude 2.1.318 (Claude Code) · av unknown · model deepseek-flash · seed 1 · av arms: hooks only · agent isolation: user benchagent · 3 sessions launched, $0.053 spent

Valid sessions per arm: cold 1, notes 1, av 1.
**No significance claim**: fewer than 30 valid sessions in at least one arm; the intervals below are bootstrap 95% CIs of the median and are wide at this n.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same tree with what an earlier agent learned on top, one note per task (the one it needs and the others as noise): as commit messages in every arm but `cold` (which gets none of it and is the control), plus a NOTES.md that CLAUDE.md points at in `notes`, plus anchored records of an earlier task in the av arms. Only the channel differs. The notes were written for the benchmark, so this measures whether recorded knowledge reaches the next agent, not whether agents write it. Repeated dead ends count edits that write the approach the task's note warns about; a decision ignored shows as a failed hidden test.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| cold | 0/1 | n/a | n/a | $0.021 | 36452 | 25392 (0) | 0 |
| notes | 1/1 | $0.015 | 46 s | $0.015 | 28748 | 20510 (0) | 0 |
| av | 1/1 | $0.017 | 47 s | $0.017 | 29569 | 20128 (0) | 0 |

## Attribution (relative change)

| comparison | cost / accepted | median tokens_total | median tokens_to_first_edit | success rate |
|---|---|---|---|---|
| relay: av vs notes | +6.9% | +2.9% | -1.9% | +0.0% |
| relay: notes vs cold | n/a | -21.1% | -19.2% | n/a |
| relay: av vs cold | n/a | -18.9% | -20.7% | n/a |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| relay-order-ids | cold | 1 | 0 | 25392 | 36452 | 0 | $0.021 |
| relay-order-ids | notes | 1 | 1 | 20510 | 28748 | 0 | $0.015 |
| relay-order-ids | av | 1 | 1 | 20128 | 29569 | 0 | $0.017 |
