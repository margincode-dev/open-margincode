# E4: knowledge relay

Does what one agent learned reach the next one? Agent A worked on tallyho earlier and left ten
lessons behind: five dead ends ("tried X, it breaks Y") and five decisions ("we do it this way").
Agent B gets one task per lesson and starts with no memory of A. Each task has a hidden test that
the obvious approach fails and the lesson's approach passes (`reference/check.py` proves it for all
ten).

| arm | where A's lessons are |
|---|---|
| `baseline` | commit messages on top of the base (`git log` shows them, nothing points at them) |
| `notes` | the same commits, plus `NOTES.md` with all ten lessons, which `CLAUDE.md` tells B to read |
| `av` | the same commits, plus each lesson as a record of an earlier av task, anchored to its code; the pack and the `PreToolUse` hook (`--jit-hooks`) bring them up |

Every arm gets all ten lessons, so for each task one is the one B needs and nine are noise. The
lessons are in `pool/notes.toml`; `relay_pair` in each spec names its own.

The measures are the usual ones (accepted = hidden test passes; cost and wall time per accepted
task) plus **dead ends repeated**: the `[[dead_end]]` patterns match the obvious wrong edit in B's
diff. The report adds the rows `relay: notes vs baseline` and `relay: av vs notes`.

## Running it

The tasks use DeepSeek's Anthropic-compatible API (`deepseek-flash`) to keep a full run under
about $4. Claude Code cannot price it, so `--price-per-mtok` gives the prices
(`input,cache_write,cache_read,output`, dollars per million tokens; DeepSeek bills cache writes as
plain input) and `--max-budget-usd` is not passed. `--session-cap-usd` then only sets what the
budget check reserves for the next session (`max_turns` = 30 is the real limit), so set it near a
pilot session's cost. Check the real spend against
`GET https://api.deepseek.com/user/balance` before and after.

```bash
bench/tallyho/make-repo.sh                        # ~/bench-repos/tallyho
~/bench-work/venv/bin/python bench/tasks-relay/reference/check.py   # all 10 pairs ok
scholion-bench check bench/tasks-relay

unset ANTHROPIC_API_KEY
export ANTHROPIC_BASE_URL=https://api.deepseek.com/anthropic
export ANTHROPIC_AUTH_TOKEN="$(cat ~/.config/deepseek/key)"   # the key alone, chmod 600
export ANTHROPIC_DEFAULT_HAIKU_MODEL=deepseek-flash ANTHROPIC_SMALL_FAST_MODEL=deepseek-flash
export CLAUDE_CODE_SUBAGENT_MODEL=deepseek-flash

scholion-bench run bench/tasks-relay --arms baseline,notes,av --jit-hooks --reps 2 \
  --price-per-mtok 0.30,0.30,0.006,1.20 --session-cap-usd 0.30 --budget-usd 4 \
  --out bench/results/$(date +%F)-relay $COMMON --confirm
```

`$COMMON` is the bench stack setup from `bench/README.md` (daemon URL, token). Run one pair first
(`--only relay-quote-split --reps 1`) and check the spend before the full run.
