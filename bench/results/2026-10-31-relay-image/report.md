# scholion-bench report

Date 2026-10-31 16:12 UTC · claude 2.1.316 (Claude Code) · av af16465 · model deepseek-flash · seed 1 · av arms: hooks only · 90 sessions launched, $0.590 spent

Valid sessions per arm: cold 30, notes 30, av 30.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same tree with what an earlier agent learned on top, one note per task (the one it needs and the others as noise): as commit messages in every arm but `cold` (which gets none of it and is the control), plus a NOTES.md that CLAUDE.md points at in `notes`, plus anchored records of an earlier task in the av arms. Only the channel differs. The notes were written for the benchmark, so this measures whether recorded knowledge reaches the next agent, not whether agents write it. Repeated dead ends count edits that write the approach the task's note warns about; a decision ignored shows as a failed hidden test.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| cold | 16/30 | $0.012 | 79 s | $0.005 ($0.004–$0.008) | 22604 (19280–31352) | 17264 (0) | 0 |
| notes | 30/30 | $0.007 | 37 s | $0.005 ($0.005–$0.007) | 22559 (21564–27717) | 18008 (0) | 0 |
| av | 30/30 | $0.006 | 38 s | $0.006 ($0.005–$0.006) | 22770 (21564–24416) | 16701 (0) | 0 |

## Attribution (relative change)

| comparison | cost / accepted | median tokens_total | median tokens_to_first_edit | success rate |
|---|---|---|---|---|
| relay: av vs notes | -2.5% | +0.9% | -7.3% | +0.0% |
| relay: notes vs cold | -47.3% | -0.2% | +4.3% | +87.5% |
| relay: av vs cold | -48.6% | +0.7% | -3.3% | +87.5% |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| relay-coupon-remove | cold | 3 | 3 | 14869 | 18474 | 0 | $0.004 |
| relay-coupon-remove | notes | 3 | 3 | 15384 | 18017 | 0 | $0.004 |
| relay-coupon-remove | av | 3 | 3 | 14040 | 18048 | 0 | $0.004 |
| relay-customer-region | cold | 3 | 0 | 15310 | 18532 | 0 | $0.004 |
| relay-customer-region | notes | 3 | 3 | 17750 | 21973 | 0 | $0.005 |
| relay-customer-region | av | 3 | 3 | 15949 | 21150 | 0 | $0.005 |
| relay-ledger-refund | cold | 3 | 2 | 24407 | 31732 | 0 | $0.008 |
| relay-ledger-refund | notes | 3 | 3 | 23040 | 28849 | 0 | $0.007 |
| relay-ledger-refund | av | 3 | 3 | 17978 | 24109 | 0 | $0.006 |
| relay-line-taxes | cold | 3 | 1 | 20499 | 30972 | 0 | $0.009 |
| relay-line-taxes | notes | 3 | 3 | 22527 | 33590 | 0 | $0.010 |
| relay-line-taxes | av | 3 | 3 | 18073 | 29689 | 0 | $0.009 |
| relay-money-plain | cold | 3 | 3 | 16158 | 18966 | 0 | $0.004 |
| relay-money-plain | notes | 3 | 3 | 17841 | 20715 | 0 | $0.004 |
| relay-money-plain | av | 3 | 3 | 14422 | 17495 | 0 | $0.004 |
| relay-order-ids | cold | 3 | 0 | 24825 | 32946 | 0 | $0.009 |
| relay-order-ids | notes | 3 | 3 | 20887 | 27936 | 0 | $0.007 |
| relay-order-ids | av | 3 | 3 | 18789 | 27483 | 0 | $0.008 |
| relay-product-slug | cold | 3 | 0 | 16464 | 19594 | 0 | $0.004 |
| relay-product-slug | notes | 3 | 3 | 16749 | 21110 | 0 | $0.005 |
| relay-product-slug | av | 3 | 3 | 16077 | 20879 | 0 | $0.005 |
| relay-quote-split | cold | 3 | 3 | 16821 | 21378 | 0 | $0.005 |
| relay-quote-split | notes | 3 | 3 | 16538 | 21090 | 0 | $0.005 |
| relay-quote-split | av | 3 | 3 | 16716 | 23119 | 0 | $0.006 |
| relay-refund-event | cold | 3 | 1 | 26137 | 41753 | 0 | $0.011 |
| relay-refund-event | notes | 3 | 3 | 25518 | 43587 | 0 | $0.014 |
| relay-refund-event | av | 3 | 3 | 22485 | 41725 | 0 | $0.014 |
| relay-store-count | cold | 3 | 3 | 17549 | 22139 | 0 | $0.005 |
| relay-store-count | notes | 3 | 3 | 18174 | 23934 | 0 | $0.005 |
| relay-store-count | av | 3 | 3 | 15766 | 22421 | 0 | $0.006 |
