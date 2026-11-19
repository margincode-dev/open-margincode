# margincode-bench report

Date 2026-11-19 05:32 UTC · claude 2.1.318 (Claude Code) · av unknown · model claude-haiku-5-5 · seed 1 · av arms: hooks only · agent isolation: user benchagent · plain records · 90 sessions launched, $0.485 spent

Valid sessions per arm: av 90.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same single-commit tree with no history (the av arms have no `.git` at all); a real git checkout would also have history, so this comparison is conservative for av. Dead ends are seeded as `attempt` records in the av arms only: the dead-end column measures whether existing records stop repeats, not whether agents write them.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| av | 47/90 | $0.010 | 40 s | $0.005 ($0.005–$0.005) | 16558 (16142–17935) | 9512 (2) | 42 |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| relay-bus-resubscribe | av | 30 | 10 | 8348 | 15307 | 23 | $0.005 |
| relay-quote-split | av | 30 | 7 | 12344 | 19902 | 19 | $0.006 |
| relay-tax-with-rate | av | 30 | 30 | 9550 | 16295 | 0 | $0.005 |
