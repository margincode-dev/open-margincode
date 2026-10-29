# scholion-bench report

Date 2026-10-29 04:31 UTC · claude 2.1.312 (Claude Code) · av 08ed850 · model deepseek-flash · seed 1 · av arms: hooks + MCP · 19 sessions launched, $0.325 spent

Valid sessions per arm: cold 19.
**No significance claim**: fewer than 30 valid sessions in at least one arm; the intervals below are bootstrap 95% CIs of the median and are wide at this n.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same tree with what an earlier agent learned on top, one note per task (the one it needs and the others as noise): as commit messages in every arm but `cold` (which gets none of it and is the control), plus a NOTES.md that CLAUDE.md points at in `notes`, plus anchored records of an earlier task in the av arms. Only the channel differs. The notes were written for the benchmark, so this measures whether recorded knowledge reaches the next agent, not whether agents write it. Repeated dead ends count edits that write the approach the task's note warns about; a decision ignored shows as a failed hidden test.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| cold | 11/19 | $0.030 | 86 s | $0.012 ($0.010–$0.024) | 24051 (21051–42510) | 17209 (0) | 0 |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| relay-coupon-remove | cold | 2 | 2 | 15791 | 20268 | 0 | $0.009 |
| relay-customer-region | cold | 2 | 0 | 15649 | 20093 | 0 | $0.009 |
| relay-ledger-refund | cold | 2 | 0 | 19672 | 27849 | 0 | $0.015 |
| relay-line-taxes | cold | 2 | 1 | 26284 | 48378 | 0 | $0.034 |
| relay-money-plain | cold | 2 | 2 | 14639 | 21283 | 0 | $0.011 |
| relay-order-ids | cold | 2 | 2 | 36670 | 48581 | 0 | $0.027 |
| relay-product-slug | cold | 2 | 0 | 16133 | 21368 | 0 | $0.010 |
| relay-quote-split | cold | 2 | 2 | 17024 | 23723 | 0 | $0.012 |
| relay-refund-event | cold | 2 | 1 | 26188 | 45053 | 0 | $0.029 |
| relay-store-count | cold | 1 | 1 | 18126 | 24051 | 0 | $0.011 |

## Skipped (budget)

- relay-store-count · cold · rep 2
- relay-coupon-remove · cold · rep 3
- relay-customer-region · cold · rep 3
- relay-ledger-refund · cold · rep 3
- relay-line-taxes · cold · rep 3
- relay-money-plain · cold · rep 3
- relay-order-ids · cold · rep 3
- relay-product-slug · cold · rep 3
- relay-quote-split · cold · rep 3
- relay-refund-event · cold · rep 3
- relay-store-count · cold · rep 3
