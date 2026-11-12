# scholion-bench report

Date 2026-11-12 07:07 UTC · claude 2.1.318 (Claude Code) · av unknown · model deepseek-flash · seed 1 · av arms: hooks only · agent isolation: user benchagent · 60 sessions launched, $0.876 spent

Valid sessions per arm: notes 30, av 30.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same tree with what an earlier agent learned on top, one note per task (the one it needs and the others as noise): as commit messages in every arm but `cold` (which gets none of it and is the control), plus a NOTES.md that CLAUDE.md points at in `notes`, plus anchored records of an earlier task in the av arms. Only the channel differs. The notes were written for the benchmark, so this measures whether recorded knowledge reaches the next agent, not whether agents write it. Repeated dead ends count edits that write the approach the task's note warns about; a decision ignored shows as a failed hidden test.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| notes | 28/30 | $0.016 | 45 s | $0.014 ($0.014–$0.015) | 27205 (26148–28082) | 18901 (0) | 2 |
| av | 12/30 | $0.037 | 115 s | $0.013 ($0.012–$0.015) | 25096 (23657–26580) | 16821 (0) | 18 |

## Attribution (relative change)

| comparison | cost / accepted | median tokens_total | median tokens_to_first_edit | success rate |
|---|---|---|---|---|
| relay: av vs notes | +134.5% | -7.8% | -11.0% | -57.1% |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| relay-quote-split | notes | 30 | 28 | 18901 | 27205 | 2 | $0.014 |
| relay-quote-split | av | 30 | 12 | 16821 | 25096 | 18 | $0.013 |
