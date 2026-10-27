# scholion-bench report

Date 2026-10-27 13:12 UTC · claude 2.1.311 (Claude Code) · av 0a08e6e · model claude-haiku-5-5 · seed 1 · av arms: hooks + MCP · 40 sessions launched, $0.227 spent

Valid sessions per arm: av 40.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same single-commit tree with no history (the av arms have no `.git` at all); a real git checkout would also have history, so this comparison is conservative for av. Dead ends are seeded as `attempt` records in the av arms only: the dead-end column measures whether existing records stop repeats, not whether agents write them.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| av | 40/40 | $0.006 | 24 s | $0.006 ($0.006–$0.006) | 16847 (15491–17565) | 7983 (0) | 0 |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| compound-discounts | av | 10 | 10 | 10276 | 17709 | 0 | $0.007 |
| csv-escaping | av | 10 | 10 | 5763 | 9348 | 0 | $0.003 |
| paginated-scan | av | 10 | 10 | 10777 | 17836 | 0 | $0.006 |
| summarize-abbrev | av | 10 | 10 | 6217 | 16791 | 0 | $0.006 |
