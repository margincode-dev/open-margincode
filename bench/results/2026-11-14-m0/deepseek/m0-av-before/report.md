# scholion-bench report

Date 2026-11-15 23:49 UTC · claude 2.1.318 (Claude Code) · av unknown · model deepseek-flash · seed 1 · av arms: hooks only · agent isolation: user benchagent · 30 sessions launched, $0.317 spent

Valid sessions per arm: av 30.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same single-commit tree with no history (the av arms have no `.git` at all); a real git checkout would also have history, so this comparison is conservative for av. Dead ends are seeded as `attempt` records in the av arms only: the dead-end column measures whether existing records stop repeats, not whether agents write them.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| av | 11/30 | $0.029 | 82 s | $0.010 ($0.010–$0.011) | 21008 (19937–21914) | 14455 (0) | 26 |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| relay-bus-resubscribe | av | 10 | 1 | 13441 | 19697 | 16 | $0.010 |
| relay-quote-split | av | 10 | 0 | 16114 | 22234 | 10 | $0.011 |
| relay-tax-with-rate | av | 10 | 10 | 14405 | 20088 | 0 | $0.010 |
