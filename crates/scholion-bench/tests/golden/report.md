# scholion-bench report

Date 2026-10-22 20:00 UTC · claude 2.1.308 (Claude Code) · av abc1234 · model claude-sonnet-5-5 · seed 1 · av arms: hooks + MCP · 10 sessions launched, $7.750 spent

Valid sessions per arm: baseline 3, av_no_packs 3, av 3.
**No significance claim**: fewer than 30 valid sessions in at least one arm; the intervals below are bootstrap 95% CIs of the median and are wide at this n.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same single-commit tree with no history (the av arms have no `.git` at all); a real git checkout would also have history, so this comparison is conservative for av. Dead ends are seeded as `attempt` records in the av arms only: the dead-end column measures whether existing records stop repeats, not whether agents write them.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| baseline | 2/3 | $1.550 | 120 s | $1.100 ($0.800–$1.200) | 52000 (40000–61000) | 10500 (1) | 3 |
| av_no_packs | 2/3 | $1.300 | 120 s | $0.750 ($0.700–$1.150) | 38000 (36000–60000) | 7600 (0) | 1 |
| av | 3/3 | $0.650 | 80 s | $0.550 ($0.500–$0.900) | 27000 (25000–41000) | 3400 (0) | 0 |

## Attribution (relative change)

| comparison | cost / accepted | median tokens_total | median tokens_to_first_edit | success rate |
|---|---|---|---|---|
| engine: av_no_packs vs baseline | -16.1% | -26.9% | -27.6% | +0.0% |
| context compiler: av vs av_no_packs | -50.0% | -28.9% | -55.3% | +50.0% |
| total: av vs baseline | -58.1% | -48.1% | -67.6% | +50.0% |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| alpha | baseline | 2 | 1 | 9000 | 46000 | 3 | $0.950 |
| alpha | av_no_packs | 2 | 2 | 7300 | 37000 | 1 | $0.725 |
| alpha | av | 2 | 2 | 3200 | 26000 | 0 | $0.525 |
| beta | baseline | 1 | 1 | 12000 | 61000 | 0 | $1.200 |
| beta | av_no_packs | 1 | 0 | 11000 | 60000 | 0 | $1.150 |
| beta | av | 1 | 1 | 5000 | 41000 | 0 | $0.900 |

## Skipped (budget)

- beta · baseline · rep 2

## Invalid

- beta · av · rep 2: the av hooks never fired
