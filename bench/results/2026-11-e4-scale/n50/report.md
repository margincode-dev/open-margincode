# scholion-bench report

Date 2026-11-10 14:24 UTC · claude 2.1.318 (Claude Code) · av fbc5bb0 · model deepseek-flash · seed 1 · av arms: hooks only · noise 50 · 60 sessions launched, $0.807 spent

Valid sessions per arm: notes 30, av 30.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same tree with what an earlier agent learned on top, one note per task (the one it needs and the others as noise): as commit messages in every arm but `cold` (which gets none of it and is the control), plus a NOTES.md that CLAUDE.md points at in `notes`, plus anchored records of an earlier task in the av arms. Only the channel differs. The notes were written for the benchmark, so this measures whether recorded knowledge reaches the next agent, not whether agents write it. Repeated dead ends count edits that write the approach the task's note warns about; a decision ignored shows as a failed hidden test.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| notes | 30/30 | $0.014 | 34 s | $0.011 ($0.010–$0.015) | 25912 (24012–29074) | 20756 (0) | 0 |
| av | 30/30 | $0.013 | 36 s | $0.010 ($0.009–$0.012) | 22243 (20974–25026) | 16600 (0) | 0 |

## Attribution (relative change)

| comparison | cost / accepted | median tokens_total | median tokens_to_first_edit | success rate |
|---|---|---|---|---|
| relay: av vs notes | -6.4% | -14.2% | -20.0% | +0.0% |

## Memory at scale (noise 50)

Every relay task's pool had 50 irrelevant notes added, the same ones in `notes` (`NOTES.md`) and the av arms (records anchored to the base tree's files). Input tokens include cache reads: the whole context the model read, turn after turn. Memory tokens (cl100k) are what the channel hands over before the agent starts: `NOTES.md` in `notes`, the pack the av task starts with in the av arms (what the pre-read hook adds later is in the input tokens).

| arm | n | solved | median input tokens | median memory tokens | median turns |
|---|---|---|---|---|---|
| notes | 30 | 30/30 | 228393 | 3570 | 17 |
| av | 30 | 30/30 | 190650 | 706 | 17 |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| relay-coupon-remove | notes | 3 | 3 | 18665 | 21426 | 0 | $0.008 |
| relay-coupon-remove | av | 3 | 3 | 14227 | 17473 | 0 | $0.007 |
| relay-customer-region | notes | 3 | 3 | 21229 | 27360 | 0 | $0.013 |
| relay-customer-region | av | 3 | 3 | 15873 | 20228 | 0 | $0.009 |
| relay-ledger-refund | notes | 3 | 3 | 21405 | 28101 | 0 | $0.014 |
| relay-ledger-refund | av | 3 | 3 | 19150 | 24837 | 0 | $0.011 |
| relay-line-taxes | notes | 3 | 3 | 23321 | 34060 | 0 | $0.019 |
| relay-line-taxes | av | 3 | 3 | 20428 | 33691 | 0 | $0.020 |
| relay-money-plain | notes | 3 | 3 | 19558 | 22415 | 0 | $0.009 |
| relay-money-plain | av | 3 | 3 | 14502 | 17714 | 0 | $0.007 |
| relay-order-ids | notes | 3 | 3 | 25230 | 34533 | 0 | $0.018 |
| relay-order-ids | av | 3 | 3 | 19105 | 25860 | 0 | $0.013 |
| relay-product-slug | notes | 3 | 3 | 19406 | 23091 | 0 | $0.009 |
| relay-product-slug | av | 3 | 3 | 16433 | 20931 | 0 | $0.009 |
| relay-quote-split | notes | 3 | 3 | 20292 | 24032 | 0 | $0.010 |
| relay-quote-split | av | 3 | 3 | 16633 | 21979 | 0 | $0.011 |
| relay-refund-event | notes | 3 | 3 | 28210 | 47157 | 0 | $0.029 |
| relay-refund-event | av | 3 | 3 | 27761 | 47514 | 0 | $0.030 |
| relay-store-count | notes | 3 | 3 | 20511 | 25456 | 0 | $0.011 |
| relay-store-count | av | 3 | 3 | 16502 | 22506 | 0 | $0.011 |
