# scholion-bench report

Date 2026-11-16 00:21 UTC · claude 2.1.318 (Claude Code) · av unknown · model claude-haiku-5-5 · seed 1 · av arms: hooks only · agent isolation: user benchagent · 30 sessions launched, $0.173 spent

Valid sessions per arm: av 30.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same single-commit tree with no history (the av arms have no `.git` at all); a real git checkout would also have history, so this comparison is conservative for av. Dead ends are seeded as `attempt` records in the av arms only: the dead-end column measures whether existing records stop repeats, not whether agents write them.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| av | 11/30 | $0.016 | 64 s | $0.006 ($0.005–$0.006) | 17557 (16262–19620) | 9353 (0) | 19 |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| relay-bus-resubscribe | av | 10 | 1 | 8225 | 14975 | 10 | $0.005 |
| relay-quote-split | av | 10 | 0 | 12467 | 20824 | 9 | $0.006 |
| relay-tax-with-rate | av | 10 | 10 | 9309 | 16854 | 0 | $0.005 |
