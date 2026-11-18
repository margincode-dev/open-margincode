# margincode-bench report

Date 2026-11-18 05:14 UTC · claude 2.1.318 (Claude Code) · av unknown · model claude-haiku-5-5 · seed 1 · av arms: hooks only · agent isolation: user benchagent · memory: dreamed sha256:a7daeb192c351585e6d8b816e9c47e4b0a74d02f7c098a35525be244b8041516 · plain records · 30 sessions launched, $0.150 spent

Valid sessions per arm: av 30.

Token counts exclude cache reads in every arm, so per-turn re-reads of the same context are not counted. `tokens_to_first_edit` counts output tokens as each message reports them, which can lag the session totals. Every arm starts from the same single-commit tree with no history (the av arms have no `.git` at all); a real git checkout would also have history, so this comparison is conservative for av. Dead ends are seeded as `attempt` records in the av arms only: the dead-end column measures whether existing records stop repeats, not whether agents write them.

## Cost and wall time per accepted task

| arm | accepted | cost / accepted | wall / accepted | median cost (95% CI) | median tokens_total (95% CI) | median tokens_to_first_edit (no edit) | repeated dead ends |
|---|---|---|---|---|---|---|---|
| av | 21/30 | $0.007 | 26 s | $0.005 ($0.005–$0.005) | 16524 (15554–17429) | 9480 (0) | 7 |

## Per task

| task | arm | n | accepted | median tokens_to_first_edit | median tokens_total | repeated dead ends | median cost |
|---|---|---|---|---|---|---|---|
| relay-bus-resubscribe | av | 10 | 10 | 8246 | 14410 | 0 | $0.004 |
| relay-quote-split | av | 10 | 1 | 11875 | 19149 | 7 | $0.006 |
| relay-tax-with-rate | av | 10 | 10 | 9480 | 16524 | 0 | $0.005 |
