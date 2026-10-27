# scholion-bench report

Date 2026-10-27 11:48 UTC · claude 2.1.311 (Claude Code) · av 0a08e6e · model claude-haiku-5-5 · seed 1 · av arms: hooks + MCP · 24 sessions launched, $0.133 spent

Valid sessions per arm: av 24.
**No significance claim**: fewer than 30 valid sessions in at least one arm; the intervals below are bootstrap 95% CIs of the median and are wide at this n.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same single-commit tree with no history (the av arms have no `.git` at all); a real git checkout would also have history, so this comparison is conservative for av. Dead ends are seeded as `attempt` records in the av arms only: the dead-end column measures whether existing records stop repeats, not whether agents write them.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| av | 24/24 | $0.006 | 25 s | $0.006 ($0.005–$0.006) | 17026 (14888–17451) | 8914 (0) | 0 |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| compound-discounts | av | 6 | 6 | 10676 | 17631 | 0 | $0.006 |
| csv-escaping | av | 6 | 6 | 5950 | 9604 | 0 | $0.004 |
| paginated-scan | av | 6 | 6 | 10578 | 16026 | 0 | $0.005 |
| summarize-abbrev | av | 6 | 6 | 8635 | 17615 | 0 | $0.007 |
