# Pre-registration: E4 relay on the rebuilt bench image, 2026-10-31

The `av` arm uses the Docker bench stack (`scholion-bench`): Postgres, Qdrant, TEI
(`bge-small-en-v1.5`, 384-d), and `scholion-daemon:dev` rebuilt at 2026-10-31T16:04:52Z.
That image booted against this database (`/healthz` ok, semantic layer on). The earlier
host-binary run in `bench/results/2026-10-31-relay-embed/` (60/90) and the no-semantic
partial are not part of this result.

Workspaces for every arm are on `/mnt/c/Users/vinhq/bench-work`, the disk the Docker mounts
use. The daemon sees imports at `/import`.

Question: on this image, do `notes` and `av` still accept more relay tasks than `cold`,
as on 2026-10-29 (cold 18/30, notes 30/30, av 30/30)? Secondary: does `av` cost less per
accepted task than `notes` (that run was −1.8%)?

- Model: `deepseek-flash` through Claude Code's Anthropic-compatible endpoint. No other API key.
- Arms: `cold`, `notes`, `av`. Reps: 3. Ten tasks. Seed 1. `av` is hooks + `--jit-hooks`, not MCP.
- Prices are the off-peak Flash rates (Saturday): input and cache write $0.15 / M, cache read
  $0.003 / M, output $0.60 / M. DeepSeek bills cache writes as input.
- Primary metric: tasks accepted by the hidden pytest, per arm.
- A one-session `av` smoke check is written to `bench/results/2026-10-31-relay-image-pilot/`
  and is not counted here.
- Budget: `--budget-usd 1.10` and `--session-cap-usd 0.08`. Starting balance $2.27. Stop if
  the balance goes under $1.05.
- Decision: with fewer than 30 valid sessions in an arm, report the counts and make no
  significance claim. If `cold` accepts at least as many as `notes`, the tasks no longer
  measure the relay.
