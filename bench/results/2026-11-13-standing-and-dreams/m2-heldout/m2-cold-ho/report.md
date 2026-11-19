# margincode-bench report

Date 2026-11-19 15:40 UTC · claude 2.1.318 (Claude Code) · av unknown · model claude-haiku-5-5 · seed 1 · av arms: hooks only · agent isolation: user benchagent · 50 sessions launched, $0.232 spent

Valid sessions per arm: cold 50.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same tree with what an earlier agent learned on top, one note per task (the one it needs and the others as noise): as commit messages in every arm but `cold` (which gets none of it and is the control; a note the task set marks `no_commit` is in no commit message, only in the two channels that follow), plus a NOTES.md that CLAUDE.md points at in `notes`, plus anchored records of an earlier task in the av arms. Only the channel differs. The notes were written for the benchmark, so this measures whether recorded knowledge reaches the next agent, not whether agents write it. Repeated dead ends count edits that write the approach the task's note warns about; a decision ignored shows as a failed hidden test.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| cold | 50/50 | $0.005 | 16 s | $0.005 ($0.004–$0.005) | 16661 (14666–19198) | 11717 (2) | 0 |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| relay-coupon-days-left | cold | 10 | 10 | 9043 | 12440 | 0 | $0.004 |
| relay-delete-order | cold | 10 | 10 | 14380 | 20925 | 0 | $0.005 |
| relay-ledger-revenue | cold | 10 | 10 | 10238 | 15256 | 0 | $0.004 |
| relay-order-can-cancel | cold | 10 | 10 | 10205 | 14195 | 0 | $0.004 |
| relay-order-comments | cold | 10 | 10 | 14192 | 22437 | 0 | $0.006 |
