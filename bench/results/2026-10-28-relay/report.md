# scholion-bench report

Date 2026-10-29 02:53 UTC · claude 2.1.312 (Claude Code) · av af7b56b · model deepseek-flash · seed 1 · av arms: hooks + MCP · 90 sessions launched, $1.371 spent

Valid sessions per arm: baseline 30, notes 30, av 30.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same tree with what an earlier agent learned on top, one note per task (the one it needs and the others as noise): as commit messages in every arm, plus a NOTES.md that CLAUDE.md points at in `notes`, plus anchored records of an earlier task in the av arms. Only the channel differs. The notes were written for the benchmark, so this measures whether recorded knowledge reaches the next agent, not whether agents write it. Repeated dead ends count edits that write the approach the task's note warns about; a decision ignored shows as a failed hidden test.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| baseline | 30/30 | $0.014 | 41 s | $0.012 ($0.011–$0.014) | 25399 (22747–27083) | 18532 (0) | 0 |
| notes | 30/30 | $0.014 | 40 s | $0.012 ($0.011–$0.016) | 24627 (22557–28553) | 17978 (0) | 0 |
| av | 30/30 | $0.017 | 57 s | $0.016 ($0.013–$0.018) | 28753 (25990–31262) | 17596 (0) | 0 |

## Attribution (relative change)

| comparison | cost / accepted | median tokens_total | median tokens_to_first_edit | success rate |
|---|---|---|---|---|
| total: av vs baseline | +22.2% | +13.2% | -5.0% | +0.0% |
| relay: notes vs baseline | -1.9% | -3.0% | -3.0% | +0.0% |
| relay: av vs notes | +24.5% | +16.8% | -2.1% | +0.0% |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| relay-coupon-remove | baseline | 3 | 3 | 14743 | 19676 | 0 | $0.009 |
| relay-coupon-remove | notes | 3 | 3 | 15624 | 19883 | 0 | $0.009 |
| relay-coupon-remove | av | 3 | 3 | 15892 | 22424 | 0 | $0.011 |
| relay-customer-region | baseline | 3 | 3 | 18862 | 25564 | 0 | $0.012 |
| relay-customer-region | notes | 3 | 3 | 18165 | 23693 | 0 | $0.011 |
| relay-customer-region | av | 3 | 3 | 17722 | 31390 | 0 | $0.017 |
| relay-ledger-refund | baseline | 3 | 3 | 20649 | 25363 | 0 | $0.012 |
| relay-ledger-refund | notes | 3 | 3 | 20491 | 26153 | 0 | $0.013 |
| relay-ledger-refund | av | 3 | 3 | 22863 | 29661 | 0 | $0.015 |
| relay-line-taxes | baseline | 3 | 3 | 20484 | 34506 | 0 | $0.021 |
| relay-line-taxes | notes | 3 | 3 | 20801 | 37560 | 0 | $0.023 |
| relay-line-taxes | av | 3 | 3 | 21249 | 39705 | 0 | $0.026 |
| relay-money-plain | baseline | 3 | 3 | 15412 | 19926 | 0 | $0.009 |
| relay-money-plain | notes | 3 | 3 | 15511 | 19152 | 0 | $0.009 |
| relay-money-plain | av | 3 | 3 | 14648 | 19285 | 0 | $0.009 |
| relay-order-ids | baseline | 3 | 3 | 20122 | 27259 | 0 | $0.014 |
| relay-order-ids | notes | 3 | 3 | 22103 | 30837 | 0 | $0.016 |
| relay-order-ids | av | 3 | 3 | 20543 | 29997 | 0 | $0.018 |
| relay-product-slug | baseline | 3 | 3 | 15990 | 20460 | 0 | $0.009 |
| relay-product-slug | notes | 3 | 3 | 16575 | 21663 | 0 | $0.010 |
| relay-product-slug | av | 3 | 3 | 15204 | 24322 | 0 | $0.013 |
| relay-quote-split | baseline | 3 | 3 | 17373 | 22815 | 0 | $0.011 |
| relay-quote-split | notes | 3 | 3 | 17539 | 23390 | 0 | $0.011 |
| relay-quote-split | av | 3 | 3 | 16528 | 27177 | 0 | $0.015 |
| relay-refund-event | baseline | 3 | 3 | 24556 | 41378 | 0 | $0.026 |
| relay-refund-event | notes | 3 | 3 | 24128 | 36813 | 0 | $0.020 |
| relay-refund-event | av | 3 | 3 | 31056 | 47594 | 0 | $0.029 |
| relay-store-count | baseline | 3 | 3 | 18362 | 26344 | 0 | $0.013 |
| relay-store-count | notes | 3 | 3 | 17791 | 24470 | 0 | $0.012 |
| relay-store-count | av | 3 | 3 | 17411 | 29306 | 0 | $0.017 |
