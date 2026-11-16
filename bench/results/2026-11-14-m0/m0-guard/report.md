# scholion-bench report

Date 2026-11-16 01:07 UTC · claude 2.1.318 (Claude Code) · av unknown · model claude-haiku-5-5 · seed 1 · av arms: hooks only · agent isolation: user benchagent · 30 sessions launched, $0.234 spent

Valid sessions per arm: av 30.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same single-commit tree with no history (the av arms have no `.git` at all); a real git checkout would also have history, so this comparison is conservative for av. Dead ends are seeded as `attempt` records in the av arms only: the dead-end column measures whether existing records stop repeats, not whether agents write them.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| av | 29/30 | $0.008 | 32 s | $0.007 ($0.006–$0.009) | 20982 (18513–27843) | 11752 (0) | 0 |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| relay-coupon-remove | av | 3 | 3 | 9570 | 14454 | 0 | $0.004 |
| relay-customer-region | av | 3 | 3 | 12051 | 22160 | 0 | $0.007 |
| relay-ledger-refund | av | 3 | 3 | 16240 | 28935 | 0 | $0.010 |
| relay-line-taxes | av | 3 | 3 | 15778 | 31313 | 0 | $0.010 |
| relay-money-plain | av | 3 | 3 | 9866 | 15849 | 0 | $0.005 |
| relay-order-ids | av | 3 | 3 | 16916 | 28686 | 0 | $0.008 |
| relay-product-slug | av | 3 | 3 | 10372 | 16938 | 0 | $0.005 |
| relay-quote-split | av | 3 | 3 | 11377 | 18799 | 0 | $0.006 |
| relay-refund-event | av | 3 | 2 | 21019 | 42871 | 0 | $0.014 |
| relay-store-count | av | 3 | 3 | 11181 | 19602 | 0 | $0.006 |
