# scholion-bench report

Date 2026-11-11 17:13 UTC · claude 2.1.318 (Claude Code) · av unknown · model deepseek-flash · seed 1 · av arms: hooks only · 30 sessions launched, $0.321 spent

Valid sessions per arm: cold 30.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same tree with what an earlier agent learned on top, one note per task (the one it needs and the others as noise): as commit messages in every arm but `cold` (which gets none of it and is the control), plus a NOTES.md that CLAUDE.md points at in `notes`, plus anchored records of an earlier task in the av arms. Only the channel differs. The notes were written for the benchmark, so this measures whether recorded knowledge reaches the next agent, not whether agents write it. Repeated dead ends count edits that write the approach the task's note warns about; a decision ignored shows as a failed hidden test.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| cold | 27/30 | $0.012 | 31 s | $0.009 ($0.008–$0.009) | 20878 (18125–21356) | 16784 (0) | 3 |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| relay-coupon-remove | cold | 6 | 6 | 14435 | 16884 | 0 | $0.007 |
| relay-line-taxes | cold | 6 | 3 | 20812 | 33402 | 3 | $0.020 |
| relay-money-plain | cold | 6 | 6 | 14849 | 17250 | 0 | $0.007 |
| relay-quote-split | cold | 6 | 6 | 16856 | 21099 | 0 | $0.009 |
| relay-store-count | cold | 6 | 6 | 16986 | 21148 | 0 | $0.009 |
