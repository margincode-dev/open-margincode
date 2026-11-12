# scholion-bench report

Date 2026-11-12 04:51 UTC · claude 2.1.318 (Claude Code) · av unknown · model deepseek-flash · seed 1 · av arms: hooks only · agent isolation: user benchagent · 60 sessions launched, $0.798 spent

Valid sessions per arm: notes 30, av 30.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same tree with what an earlier agent learned on top, one note per task (the one it needs and the others as noise): as commit messages in every arm but `cold` (which gets none of it and is the control), plus a NOTES.md that CLAUDE.md points at in `notes`, plus anchored records of an earlier task in the av arms. Only the channel differs. The notes were written for the benchmark, so this measures whether recorded knowledge reaches the next agent, not whether agents write it. Repeated dead ends count edits that write the approach the task's note warns about; a decision ignored shows as a failed hidden test.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| notes | 27/30 | $0.017 | 47 s | $0.015 ($0.014–$0.016) | 28058 (25706–29538) | 19282 (0) | 3 |
| av | 2/30 | $0.175 | 497 s | $0.011 ($0.010–$0.011) | 21773 (21149–22117) | 15986 (0) | 26 |

## Attribution (relative change)

| comparison | cost / accepted | median tokens_total | median tokens_to_first_edit | success rate |
|---|---|---|---|---|
| relay: av vs notes | +954.3% | -22.4% | -17.1% | -92.6% |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| relay-quote-split | notes | 30 | 27 | 19282 | 28058 | 3 | $0.015 |
| relay-quote-split | av | 30 | 2 | 15986 | 21773 | 26 | $0.011 |
