# margincode-bench report

Date 2026-11-19 14:07 UTC · claude 2.1.318 (Claude Code) · av unknown · model claude-haiku-5-5 · seed 1 · av arms: hooks only · agent isolation: user benchagent · memory: dreamed sha256:ad8d047cf78c4781cf464aa230c9111188193fde92ab554cde0dea3a85f4ddab · plain records · 100 sessions launched, $0.618 spent

Valid sessions per arm: av 100.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same single-commit tree with no history (the av arms have no `.git` at all); a real git checkout would also have history, so this comparison is conservative for av. Dead ends are seeded as `attempt` records in the av arms only: the dead-end column measures whether existing records stop repeats, not whether agents write them.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| av | 100/100 | $0.006 | 22 s | $0.006 ($0.006–$0.006) | 19077 (18120–20019) | 11550 (4) | 2 |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| relay-coupon-days-left | av | 20 | 20 | 10605 | 17155 | 0 | $0.005 |
| relay-delete-order | av | 20 | 20 | 13251 | 21576 | 0 | $0.006 |
| relay-ledger-revenue | av | 20 | 20 | 11116 | 18248 | 0 | $0.006 |
| relay-order-can-cancel | av | 20 | 20 | 11076 | 17554 | 0 | $0.005 |
| relay-order-comments | av | 20 | 20 | 15382 | 27461 | 2 | $0.008 |
