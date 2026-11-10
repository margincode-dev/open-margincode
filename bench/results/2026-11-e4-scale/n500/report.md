# scholion-bench report

Date 2026-11-10 12:07 UTC · claude 2.1.318 (Claude Code) · av fbc5bb0 · model deepseek-flash · seed 1 · av arms: hooks only · noise 500 · 60 sessions launched, $1.140 spent

Valid sessions per arm: notes 30, av 30.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same tree with what an earlier agent learned on top, one note per task (the one it needs and the others as noise): as commit messages in every arm but `cold` (which gets none of it and is the control), plus a NOTES.md that CLAUDE.md points at in `notes`, plus anchored records of an earlier task in the av arms. Only the channel differs. The notes were written for the benchmark, so this measures whether recorded knowledge reaches the next agent, not whether agents write it. Repeated dead ends count edits that write the approach the task's note warns about; a decision ignored shows as a failed hidden test.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| notes | 30/30 | $0.024 | 42 s | $0.021 ($0.018–$0.025) | 50981 (47977–56884) | 45180 (0) | 0 |
| av | 30/30 | $0.014 | 41 s | $0.011 ($0.010–$0.016) | 24400 (22460–31138) | 17496 (0) | 0 |

## Attribution (relative change)

| comparison | cost / accepted | median tokens_total | median tokens_to_first_edit | success rate |
|---|---|---|---|---|
| relay: av vs notes | -39.9% | -52.1% | -61.3% | +0.0% |

## Memory at scale (noise 500)

Every relay task's pool had 500 irrelevant notes added, the same ones in `notes` (`NOTES.md`) and the av arms (records anchored to the base tree's files). Input tokens include cache reads: the whole context the model read, turn after turn. Memory tokens (cl100k) are what the channel hands over before the agent starts: `NOTES.md` in `notes`, the pack the av task starts with in the av arms (what the pre-read hook adds later is in the input tokens).

| arm | n | solved | median input tokens | median memory tokens | median turns |
|---|---|---|---|---|---|
| notes | 30 | 30/30 | 439830 | 27487 | 19 |
| av | 30 | 30/30 | 228843 | 710 | 19 |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| relay-coupon-remove | notes | 3 | 3 | 44774 | 47989 | 0 | $0.018 |
| relay-coupon-remove | av | 3 | 3 | 14340 | 17706 | 0 | $0.007 |
| relay-customer-region | notes | 3 | 3 | 44219 | 48934 | 0 | $0.020 |
| relay-customer-region | av | 3 | 3 | 17284 | 22828 | 0 | $0.010 |
| relay-ledger-refund | notes | 3 | 3 | 49368 | 55439 | 0 | $0.024 |
| relay-ledger-refund | av | 3 | 3 | 24015 | 31648 | 0 | $0.016 |
| relay-line-taxes | notes | 3 | 3 | 45612 | 61410 | 0 | $0.032 |
| relay-line-taxes | av | 3 | 3 | 21602 | 35677 | 0 | $0.021 |
| relay-money-plain | notes | 3 | 3 | 42077 | 44888 | 0 | $0.016 |
| relay-money-plain | av | 3 | 3 | 14609 | 18481 | 0 | $0.008 |
| relay-order-ids | notes | 3 | 3 | 48647 | 58329 | 0 | $0.027 |
| relay-order-ids | av | 3 | 3 | 32291 | 39183 | 0 | $0.020 |
| relay-product-slug | notes | 3 | 3 | 43327 | 46452 | 0 | $0.017 |
| relay-product-slug | av | 3 | 3 | 16536 | 22436 | 0 | $0.010 |
| relay-quote-split | notes | 3 | 3 | 43717 | 47820 | 0 | $0.018 |
| relay-quote-split | av | 3 | 3 | 17232 | 22483 | 0 | $0.010 |
| relay-refund-event | notes | 3 | 3 | 58742 | 78018 | 0 | $0.040 |
| relay-refund-event | av | 3 | 3 | 30667 | 46466 | 0 | $0.027 |
| relay-store-count | notes | 3 | 3 | 45316 | 51080 | 0 | $0.021 |
| relay-store-count | av | 3 | 3 | 17512 | 25414 | 0 | $0.012 |
