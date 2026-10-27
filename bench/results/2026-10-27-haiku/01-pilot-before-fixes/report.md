# scholion-bench report

Date 2026-10-27 10:05 UTC · claude 2.1.311 (Claude Code) · av 0a08e6e · model claude-haiku-5-5 · seed 1 · 24 sessions launched, $0.148 spent

Valid sessions per arm: baseline 8, av_no_packs 8, av 8.
**No significance claim**: fewer than 30 valid sessions in at least one arm; the intervals below are bootstrap 95% CIs of the median and are wide at this n.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same single-commit tree with no history (the av arms have no `.git` at all); a real git checkout would also have history, so this comparison is conservative for av. Dead ends are seeded as `attempt` records in the av arms only: the dead-end column measures whether existing records stop repeats, not whether agents write them.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| baseline | 8/8 | $0.006 | 25 s | $0.005 ($0.004–$0.007) | 15070 (9125–20374) | 7263 (0) | 0 |
| av_no_packs | 8/8 | $0.006 | 26 s | $0.007 ($0.004–$0.008) | 18372 (9873–21119) | 7369 (1) | 0 |
| av | 8/8 | $0.006 | 25 s | $0.006 ($0.003–$0.007) | 18176 (9422–18921) | 8485 (0) | 0 |

## Attribution (relative change)

| comparison | cost / accepted | median tokens_total | median tokens_to_first_edit | success rate |
|---|---|---|---|---|
| engine: av_no_packs vs baseline | +10.6% | +21.9% | +1.5% | +0.0% |
| context compiler: av vs av_no_packs | -4.1% | -1.1% | +15.1% | +0.0% |
| total: av vs baseline | +6.0% | +20.6% | +16.8% | +0.0% |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| compound-discounts | baseline | 2 | 2 | 9355 | 16297 | 0 | $0.006 |
| compound-discounts | av_no_packs | 2 | 2 | 13421 | 19522 | 0 | $0.007 |
| compound-discounts | av | 2 | 2 | 19067 | 27750 | 0 | $0.009 |
| csv-escaping | baseline | 2 | 2 | 5122 | 9123 | 0 | $0.004 |
| csv-escaping | av_no_packs | 2 | 2 | 5681 | 9391 | 0 | $0.004 |
| csv-escaping | av | 2 | 2 | 5986 | 9409 | 0 | $0.003 |
| paginated-scan | baseline | 2 | 2 | 13394 | 22336 | 0 | $0.008 |
| paginated-scan | av_no_packs | 2 | 2 | 13793 | 21373 | 0 | $0.008 |
| paginated-scan | av | 2 | 2 | 11954 | 18475 | 0 | $0.006 |
| summarize-abbrev | baseline | 2 | 2 | 5253 | 13819 | 0 | $0.005 |
| summarize-abbrev | av_no_packs | 2 | 2 | 7017 | 17709 | 0 | $0.007 |
| summarize-abbrev | av | 2 | 2 | 6454 | 17547 | 0 | $0.007 |
