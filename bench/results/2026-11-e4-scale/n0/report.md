# scholion-bench report

Date 2026-11-10 15:08 UTC · claude 2.1.318 (Claude Code) · av fbc5bb0 · model deepseek-flash · seed 1 · av arms: hooks only · noise 0 · 60 sessions launched, $0.754 spent

Valid sessions per arm: notes 30, av 30.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same tree with what an earlier agent learned on top, one note per task (the one it needs and the others as noise): as commit messages in every arm but `cold` (which gets none of it and is the control), plus a NOTES.md that CLAUDE.md points at in `notes`, plus anchored records of an earlier task in the av arms. Only the channel differs. The notes were written for the benchmark, so this measures whether recorded knowledge reaches the next agent, not whether agents write it. Repeated dead ends count edits that write the approach the task's note warns about; a decision ignored shows as a failed hidden test.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| notes | 30/30 | $0.013 | 33 s | $0.010 ($0.010–$0.013) | 23304 (20959–25880) | 18538 (0) | 0 |
| av | 30/30 | $0.013 | 36 s | $0.011 ($0.009–$0.013) | 22746 (20805–25283) | 16542 (0) | 0 |

## Attribution (relative change)

| comparison | cost / accepted | median tokens_total | median tokens_to_first_edit | success rate |
|---|---|---|---|---|
| relay: av vs notes | -1.1% | -2.4% | -10.8% | +0.0% |

## Memory at scale (noise 0)

Every relay task's pool had 0 irrelevant notes added, the same ones in `notes` (`NOTES.md`) and the av arms (records anchored to the base tree's files). Input tokens include cache reads: the whole context the model read, turn after turn. Memory tokens (cl100k) are what the channel hands over before the agent starts: `NOTES.md` in `notes`, the pack the av task starts with in the av arms (what the pre-read hook adds later is in the input tokens).

| arm | n | solved | median input tokens | median memory tokens | median turns |
|---|---|---|---|---|---|
| notes | 30 | 30/30 | 186767 | 858 | 18 |
| av | 30 | 30/30 | 240737 | 708 | 18 |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| relay-coupon-remove | notes | 3 | 3 | 15320 | 17633 | 0 | $0.007 |
| relay-coupon-remove | av | 3 | 3 | 14101 | 16786 | 0 | $0.007 |
| relay-customer-region | notes | 3 | 3 | 18468 | 23249 | 0 | $0.010 |
| relay-customer-region | av | 3 | 3 | 15793 | 20689 | 0 | $0.009 |
| relay-ledger-refund | notes | 3 | 3 | 20994 | 25532 | 0 | $0.012 |
| relay-ledger-refund | av | 3 | 3 | 20908 | 27641 | 0 | $0.014 |
| relay-line-taxes | notes | 3 | 3 | 19133 | 31532 | 0 | $0.018 |
| relay-line-taxes | av | 3 | 3 | 18318 | 29824 | 0 | $0.018 |
| relay-money-plain | notes | 3 | 3 | 16296 | 19136 | 0 | $0.008 |
| relay-money-plain | av | 3 | 3 | 14360 | 17726 | 0 | $0.007 |
| relay-order-ids | notes | 3 | 3 | 20165 | 25880 | 0 | $0.013 |
| relay-order-ids | av | 3 | 3 | 18098 | 24011 | 0 | $0.012 |
| relay-product-slug | notes | 3 | 3 | 16146 | 18961 | 0 | $0.008 |
| relay-product-slug | av | 3 | 3 | 14611 | 19429 | 0 | $0.009 |
| relay-quote-split | notes | 3 | 3 | 17379 | 21178 | 0 | $0.010 |
| relay-quote-split | av | 3 | 3 | 16622 | 22675 | 0 | $0.011 |
| relay-refund-event | notes | 3 | 3 | 25877 | 40312 | 0 | $0.024 |
| relay-refund-event | av | 3 | 3 | 20401 | 35846 | 0 | $0.023 |
| relay-store-count | notes | 3 | 3 | 18607 | 23683 | 0 | $0.011 |
| relay-store-count | av | 3 | 3 | 16280 | 25283 | 0 | $0.013 |
