# E4 at scale: hundreds of notes, most irrelevant

Same ten relay tasks as [`../tasks-relay`](../tasks-relay), but the pool is
[`../tasks-relay/pool/notes-scale.toml`](../tasks-relay/pool/notes-scale.toml): the ten real
lessons plus **290 noise notes**. The question lean E4 left open:

> av does not beat `notes` with ten notes a plain file can hold. What about hundreds, where
> reading them all costs more than av's targeted delivery?

| arm | what B gets |
|---|---|
| `cold` | base tree only (control; same as lean E4) |
| `notes` | the ten lessons as commits, plus a `NOTES.md` of all **300** notes |
| `av` | the ten lessons as commits, plus all 300 as anchored records of an earlier task |

Noise notes set `noise = true`: they fill `NOTES.md` and av records, but are **not** one empty
git commit each (setup would otherwise add hundreds of commits per session). Regenerate the
pool with `python3 bench/tasks-relay/pool/gen_scale_notes.py`.

Noise is anchored on tallyho files that are **not** the ten signal (path, symbol) pairs, so a
`PreToolUse` on a task's relevant file is not flooded with filler.

## Running it

Same DeepSeek setup as lean E4 (`../tasks-relay/README.md`). Prefer `notes,av` first (the
comparison that can show av winning); add `cold` when you want the control. Budget is higher
because `NOTES.md` is large and av seeds 300 records.

```bash
bench/tallyho/make-repo.sh
scholion-bench check bench/tasks-relay-scale

unset ANTHROPIC_API_KEY
export ANTHROPIC_BASE_URL=https://api.deepseek.com/anthropic
export ANTHROPIC_AUTH_TOKEN="$(cat ~/.config/deepseek/key)"   # required; empty sessions without it
export ANTHROPIC_DEFAULT_HAIKU_MODEL=deepseek-flash ANTHROPIC_SMALL_FAST_MODEL=deepseek-flash
export CLAUDE_CODE_SUBAGENT_MODEL=deepseek-flash

# Against a host/lite daemon (not compose), set --import-mount = --import-dir and
# --allow-no-semantic if Qdrant is off, e.g.:
#   COMMON="--av-url http://127.0.0.1:7430 --allow-no-semantic \
#     --work-root ~/bench-work --import-dir ~/bench-work/src --import-mount ~/bench-work/src"

# Pilot one memory-critical pair (cold fails these) before a full run:
scholion-bench run bench/tasks-relay-scale --only relay-customer-region \
  --arms notes,av --jit-hooks --reps 1 \
  --price-per-mtok 0.30,0.30,0.006,1.20 --session-cap-usd 0.50 --budget-usd 1 \
  --out bench/results/$(date +%F)-relay-scale-pilot $COMMON --confirm

# Full set (10 tasks × 2 arms × 3 reps ≈ 60 sessions; raise --budget-usd after the pilot):
scholion-bench run bench/tasks-relay-scale --arms notes,av --jit-hooks --reps 3 \
  --price-per-mtok 0.30,0.30,0.006,1.20 --session-cap-usd 0.50 --budget-usd 8 \
  --out bench/results/$(date +%F)-relay-scale $COMMON --confirm
```

`$COMMON` is the bench stack from `bench/README.md` (compose), or the host form above.
Commit the report under `bench/results/<date>-relay-scale/` and update `HANDOFF.md`.
