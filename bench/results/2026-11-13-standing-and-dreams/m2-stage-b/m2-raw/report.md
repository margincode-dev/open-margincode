# margincode-bench report

Date 2026-11-18 04:58 UTC · claude 2.1.318 (Claude Code) · av unknown · model claude-haiku-5-5 · seed 1 · av arms: hooks only · agent isolation: user benchagent · plain records · 30 sessions launched, $0.162 spent

Valid sessions per arm: av 30.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same single-commit tree with no history (the av arms have no `.git` at all); a real git checkout would also have history, so this comparison is conservative for av. Dead ends are seeded as `attempt` records in the av arms only: the dead-end column measures whether existing records stop repeats, not whether agents write them.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| av | 14/30 | $0.012 | 46 s | $0.005 ($0.005–$0.006) | 16957 (15959–18462) | 8976 (1) | 18 |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| relay-bus-resubscribe | av | 10 | 4 | 8202 | 14904 | 9 | $0.005 |
| relay-quote-split | av | 10 | 0 | 11737 | 19250 | 9 | $0.006 |
| relay-tax-with-rate | av | 10 | 10 | 8730 | 16049 | 0 | $0.005 |
