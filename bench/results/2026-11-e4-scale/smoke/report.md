# scholion-bench report

Date 2026-11-04 16:32 UTC · claude 2.1.318 (Claude Code) · av 669080e · model deepseek-flash · seed 1 · av arms: hooks only · noise 50 · 3 sessions launched, $0.027 spent

Valid sessions per arm: cold 1, notes 1, av 1.
**No significance claim**: fewer than 30 valid sessions in at least one arm; the intervals below are bootstrap 95% CIs of the median and are wide at this n.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same tree with what an earlier agent learned on top, one note per task (the one it needs and the others as noise): as commit messages in every arm but `cold` (which gets none of it and is the control), plus a NOTES.md that CLAUDE.md points at in `notes`, plus anchored records of an earlier task in the av arms. Only the channel differs. The notes were written for the benchmark, so this measures whether recorded knowledge reaches the next agent, not whether agents write it. Repeated dead ends count edits that write the approach the task's note warns about; a decision ignored shows as a failed hidden test.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| cold | 0/1 | n/a | n/a | $0.008 | 18731 | 16166 (0) | 0 |
| notes | 1/1 | $0.011 | 37 s | $0.011 | 24667 | 20265 (0) | 0 |
| av | 1/1 | $0.009 | 47 s | $0.009 | 20160 | 15695 (0) | 0 |

## Attribution (relative change)

| comparison | cost / accepted | median tokens_total | median tokens_to_first_edit | success rate |
|---|---|---|---|---|
| relay: av vs notes | -16.0% | -18.3% | -22.6% | +0.0% |
| relay: notes vs cold | n/a | +31.7% | +25.4% | n/a |
| relay: av vs cold | n/a | +7.6% | -2.9% | n/a |

## Memory at scale (noise 50)

Every relay task's pool had 50 irrelevant notes added, the same ones in `notes` (`NOTES.md`) and the av arms (records anchored to the base tree's files). Input tokens include cache reads: the whole context the model read, turn after turn. Memory tokens (cl100k) are what the channel hands over before the agent starts: `NOTES.md` in `notes`, the pack the av task starts with in the av arms (what the pre-read hook adds later is in the input tokens).

| arm | n | solved | median input tokens | median memory tokens | median turns |
|---|---|---|---|---|---|
| cold | 1 | 0/1 | 126500 | n/a | 13 |
| notes | 1 | 1/1 | 222518 | 3570 | 18 |
| av | 1 | 1/1 | 167132 | 711 | 15 |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| relay-customer-region | cold | 1 | 0 | 16166 | 18731 | 0 | $0.008 |
| relay-customer-region | notes | 1 | 1 | 20265 | 24667 | 0 | $0.011 |
| relay-customer-region | av | 1 | 1 | 15695 | 20160 | 0 | $0.009 |
