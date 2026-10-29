# scholion-bench report

Date 2026-10-29 11:56 UTC · claude 2.1.312 (Claude Code) · av 367aadc · model deepseek-flash · seed 1 · av arms: hooks only · 90 sessions launched, $1.171 spent

Valid sessions per arm: cold 30, notes 30, av 30.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same tree with what an earlier agent learned on top, one note per task (the one it needs and the others as noise): as commit messages in every arm but `cold` (which gets none of it and is the control), plus a NOTES.md that CLAUDE.md points at in `notes`, plus anchored records of an earlier task in the av arms. Only the channel differs. The notes were written for the benchmark, so this measures whether recorded knowledge reaches the next agent, not whether agents write it. Repeated dead ends count edits that write the approach the task's note warns about; a decision ignored shows as a failed hidden test.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| cold | 18/30 | $0.022 | 63 s | $0.010 ($0.008–$0.016) | 21664 (20018–29472) | 17219 (0) | 1 |
| notes | 30/30 | $0.013 | 36 s | $0.011 ($0.009–$0.014) | 23333 (21031–27936) | 17970 (0) | 0 |
| av | 30/30 | $0.013 | 38 s | $0.010 ($0.010–$0.012) | 22384 (21451–23720) | 16190 (0) | 0 |

## Attribution (relative change)

| comparison | cost / accepted | median tokens_total | median tokens_to_first_edit | success rate |
|---|---|---|---|---|
| relay: av vs notes | -1.8% | -4.1% | -9.9% | +0.0% |
| relay: notes vs cold | -41.1% | +7.7% | +4.4% | +66.7% |
| relay: av vs cold | -42.2% | +3.3% | -6.0% | +66.7% |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| relay-coupon-remove | cold | 3 | 3 | 15636 | 18476 | 0 | $0.008 |
| relay-coupon-remove | notes | 3 | 3 | 15905 | 18849 | 0 | $0.008 |
| relay-coupon-remove | av | 3 | 3 | 14371 | 18125 | 0 | $0.008 |
| relay-customer-region | cold | 3 | 0 | 17113 | 19745 | 0 | $0.008 |
| relay-customer-region | notes | 3 | 3 | 16798 | 21829 | 0 | $0.010 |
| relay-customer-region | av | 3 | 3 | 15593 | 22827 | 0 | $0.012 |
| relay-ledger-refund | cold | 3 | 2 | 20997 | 27501 | 0 | $0.014 |
| relay-ledger-refund | notes | 3 | 3 | 22091 | 29679 | 0 | $0.016 |
| relay-ledger-refund | av | 3 | 3 | 17942 | 22358 | 0 | $0.010 |
| relay-line-taxes | cold | 3 | 2 | 22880 | 34107 | 1 | $0.020 |
| relay-line-taxes | notes | 3 | 3 | 21236 | 34410 | 0 | $0.020 |
| relay-line-taxes | av | 3 | 3 | 19958 | 34791 | 0 | $0.022 |
| relay-money-plain | cold | 3 | 3 | 14546 | 16904 | 0 | $0.007 |
| relay-money-plain | notes | 3 | 3 | 15465 | 18420 | 0 | $0.008 |
| relay-money-plain | av | 3 | 3 | 14430 | 17698 | 0 | $0.007 |
| relay-order-ids | cold | 3 | 2 | 32213 | 40689 | 0 | $0.021 |
| relay-order-ids | notes | 3 | 3 | 20904 | 27936 | 0 | $0.015 |
| relay-order-ids | av | 3 | 3 | 18930 | 27917 | 0 | $0.017 |
| relay-product-slug | cold | 3 | 0 | 16129 | 20291 | 0 | $0.009 |
| relay-product-slug | notes | 3 | 3 | 16797 | 20645 | 0 | $0.009 |
| relay-product-slug | av | 3 | 3 | 16541 | 21264 | 0 | $0.010 |
| relay-quote-split | cold | 3 | 3 | 16899 | 21273 | 0 | $0.010 |
| relay-quote-split | notes | 3 | 3 | 17684 | 21784 | 0 | $0.009 |
| relay-quote-split | av | 3 | 3 | 16127 | 21637 | 0 | $0.010 |
| relay-refund-event | cold | 3 | 0 | 23484 | 34179 | 0 | $0.019 |
| relay-refund-event | notes | 3 | 3 | 26405 | 40146 | 0 | $0.024 |
| relay-refund-event | av | 3 | 3 | 22170 | 35965 | 0 | $0.022 |
| relay-store-count | cold | 3 | 3 | 17226 | 22054 | 0 | $0.010 |
| relay-store-count | notes | 3 | 3 | 17806 | 23685 | 0 | $0.011 |
| relay-store-count | av | 3 | 3 | 15878 | 22415 | 0 | $0.011 |
