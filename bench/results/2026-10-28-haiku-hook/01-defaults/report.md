# scholion-bench report

Date 2026-10-28 00:50 UTC · claude 2.1.312 (Claude Code) · av 1af5282 · model claude-haiku-5-5 · seed 1 · av arms: hooks only · 80 sessions launched, $0.569 spent

Valid sessions per arm: baseline 40, av 40.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same single-commit tree with no history (the av arms have no `.git` at all); a real git checkout would also have history, so this comparison is conservative for av. Dead ends are seeded as `attempt` records in the av arms only: the dead-end column measures whether existing records stop repeats, not whether agents write them.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| baseline | 40/40 | $0.007 | 32 s | $0.007 ($0.007–$0.008) | 20474 (18484–22491) | 10429 (0) | 0 |
| av | 40/40 | $0.007 | 26 s | $0.007 ($0.007–$0.008) | 20422 (19188–22316) | 12065 (0) | 0 |

## Attribution (relative change)

| comparison | cost / accepted | median tokens_total | median tokens_to_first_edit | success rate |
|---|---|---|---|---|
| total: av vs baseline | -3.7% | -0.3% | +15.7% | +0.0% |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| compound-discounts | baseline | 10 | 10 | 12736 | 24715 | 0 | $0.009 |
| compound-discounts | av | 10 | 10 | 13199 | 23270 | 0 | $0.009 |
| csv-escaping | baseline | 10 | 10 | 7839 | 12940 | 0 | $0.005 |
| csv-escaping | av | 10 | 10 | 8228 | 13521 | 0 | $0.005 |
| paginated-scan | baseline | 10 | 10 | 13378 | 21667 | 0 | $0.008 |
| paginated-scan | av | 10 | 10 | 13134 | 22463 | 0 | $0.008 |
| summarize-abbrev | baseline | 10 | 10 | 8440 | 19052 | 0 | $0.007 |
| summarize-abbrev | av | 10 | 10 | 8753 | 19439 | 0 | $0.007 |
