# scholion-bench report

Date 2026-10-28 00:50 UTC · claude 2.1.312 (Claude Code) · av 1af5282 · model claude-haiku-5-5 · seed 1 · av arms: hooks only · hook: --no-map --max-code=3 · 40 sessions launched, $0.287 spent

Valid sessions per arm: av 40.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same single-commit tree with no history (the av arms have no `.git` at all); a real git checkout would also have history, so this comparison is conservative for av. Dead ends are seeded as `attempt` records in the av arms only: the dead-end column measures whether existing records stop repeats, not whether agents write them.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| av | 40/40 | $0.007 | 29 s | $0.007 ($0.006–$0.008) | 20466 (17710–21740) | 11142 (0) | 0 |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| compound-discounts | av | 10 | 10 | 11747 | 21827 | 0 | $0.008 |
| csv-escaping | av | 10 | 10 | 7619 | 14333 | 0 | $0.005 |
| paginated-scan | av | 10 | 10 | 13748 | 22174 | 0 | $0.008 |
| summarize-abbrev | av | 10 | 10 | 9762 | 18256 | 0 | $0.007 |
