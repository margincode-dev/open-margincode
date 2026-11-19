# margincode-bench report

Date 2026-11-19 06:21 UTC · claude 2.1.318 (Claude Code) · av unknown · model claude-haiku-5-5 · seed 1 · av arms: hooks only · agent isolation: user benchagent · memory: dreamed sha256:fe405a5b1131f4ab295b63e02564cfc8473500c7cea127bf606241ae42dbec7b · plain records · 90 sessions launched, $0.448 spent

Valid sessions per arm: av 90.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same single-commit tree with no history (the av arms have no `.git` at all); a real git checkout would also have history, so this comparison is conservative for av. Dead ends are seeded as `attempt` records in the av arms only: the dead-end column measures whether existing records stop repeats, not whether agents write them.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| av | 90/90 | $0.005 | 18 s | $0.005 ($0.005–$0.005) | 16743 (16413–17193) | 9731 (0) | 0 |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| relay-bus-resubscribe | av | 30 | 30 | 8749 | 14258 | 0 | $0.004 |
| relay-quote-split | av | 30 | 30 | 12184 | 19368 | 0 | $0.005 |
| relay-tax-with-rate | av | 30 | 30 | 9712 | 16712 | 0 | $0.005 |
