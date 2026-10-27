# scholion-bench report

Date 2026-10-27 10:30 UTC · claude 2.1.311 (Claude Code) · av 0a08e6e · model claude-haiku-5-5 · seed 1 · av arms: hooks + MCP · 32 sessions launched, $0.184 spent

Valid sessions per arm: baseline 16, av 16.
**No significance claim**: fewer than 30 valid sessions in at least one arm; the intervals below are bootstrap 95% CIs of the median and are wide at this n.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same single-commit tree with no history (the av arms have no `.git` at all); a real git checkout would also have history, so this comparison is conservative for av. Dead ends are seeded as `attempt` records in the av arms only: the dead-end column measures whether existing records stop repeats, not whether agents write them.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| baseline | 15/16 | $0.006 | 26 s | $0.006 ($0.005–$0.007) | 15187 (12148–17820) | 7366 (0) | 0 |
| av | 16/16 | $0.006 | 25 s | $0.006 ($0.006–$0.007) | 17335 (15593–17882) | 8576 (0) | 0 |

## Attribution (relative change)

| comparison | cost / accepted | median tokens_total | median tokens_to_first_edit | success rate |
|---|---|---|---|---|
| total: av vs baseline | -4.9% | +14.1% | +16.4% | +6.7% |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| compound-discounts | baseline | 4 | 3 | 9902 | 17619 | 0 | $0.006 |
| compound-discounts | av | 4 | 4 | 9593 | 18056 | 0 | $0.007 |
| csv-escaping | baseline | 4 | 4 | 4830 | 8483 | 0 | $0.003 |
| csv-escaping | av | 4 | 4 | 6015 | 9832 | 0 | $0.004 |
| paginated-scan | baseline | 4 | 4 | 12995 | 20819 | 0 | $0.008 |
| paginated-scan | av | 4 | 4 | 11646 | 17387 | 0 | $0.006 |
| summarize-abbrev | baseline | 4 | 4 | 5266 | 14118 | 0 | $0.006 |
| summarize-abbrev | av | 4 | 4 | 7446 | 16351 | 0 | $0.006 |
