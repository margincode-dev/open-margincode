# scholion-bench report

Date 2026-11-10 03:43 UTC · claude 2.1.318 (Claude Code) · av fbc5bb0 · model deepseek-flash · seed 1 · av arms: hooks only · 30 sessions launched, $0.389 spent

Valid sessions per arm: cold 30.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same tree with what an earlier agent learned on top, one note per task (the one it needs and the others as noise): as commit messages in every arm but `cold` (which gets none of it and is the control), plus a NOTES.md that CLAUDE.md points at in `notes`, plus anchored records of an earlier task in the av arms. Only the channel differs. The notes were written for the benchmark, so this measures whether recorded knowledge reaches the next agent, not whether agents write it. Repeated dead ends count edits that write the approach the task's note warns about; a decision ignored shows as a failed hidden test.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| cold | 18/30 | $0.022 | 67 s | $0.010 ($0.008–$0.014) | 20956 (19596–28359) | 16817 (0) | 0 |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| relay-coupon-remove | cold | 3 | 3 | 15848 | 18310 | 0 | $0.007 |
| relay-customer-region | cold | 3 | 0 | 15953 | 19133 | 0 | $0.008 |
| relay-ledger-refund | cold | 3 | 2 | 21367 | 28546 | 0 | $0.014 |
| relay-line-taxes | cold | 3 | 3 | 25937 | 35664 | 0 | $0.021 |
| relay-money-plain | cold | 3 | 3 | 14364 | 17003 | 0 | $0.007 |
| relay-order-ids | cold | 3 | 1 | 24727 | 36945 | 0 | $0.023 |
| relay-product-slug | cold | 3 | 0 | 16592 | 20135 | 0 | $0.009 |
| relay-quote-split | cold | 3 | 3 | 16199 | 20211 | 0 | $0.009 |
| relay-refund-event | cold | 3 | 0 | 21314 | 31950 | 0 | $0.017 |
| relay-store-count | cold | 3 | 3 | 16609 | 20823 | 0 | $0.010 |
