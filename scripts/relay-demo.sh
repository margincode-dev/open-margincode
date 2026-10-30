#!/usr/bin/env bash
# The cross-vendor relay: one agent records a dead end and hands the task off; an agent from
# another vendor claims it, and its context pack leads with that dead end.
#
#   scripts/relay-demo.sh                       # Claude Code, then Codex (needs both CLIs and keys)
#   scripts/relay-demo.sh --first claude --second gemini
#   scripts/relay-demo.sh --scripted            # no agents: `av` commands stand in for them (free)
#
# Needs `av` on PATH and a daemon at $AV_URL (default http://127.0.0.1:7420; `av up` starts one).
# The daemon must see this machine's /tmp: lite mode does, the Docker stack does not.
# Real agents spend money on their own accounts; --scripted spends nothing.
set -euo pipefail

first=claude
second=codex
scripted=0
while [ $# -gt 0 ]; do
    case "$1" in
        --first) first=$2; shift 2 ;;
        --second) second=$2; shift 2 ;;
        --scripted) scripted=1; shift ;;
        -h | --help) sed -n '2,12p' "$0"; exit 0 ;;
        *) echo "relay-demo: unknown argument $1" >&2; exit 2 ;;
    esac
done

say() { printf '\n\033[1m== %s\033[0m\n' "$*"; }
die() { echo "relay-demo: $*" >&2; exit 1; }
command -v av >/dev/null || die "av is not on PATH"

# How each agent CLI runs one prompt without a human. Edits and shell commands are allowed:
# the agent works only in its task's workspace.
run_agent() {
    local agent=$1 prompt=$2
    case "$agent" in
        claude) claude -p "$prompt" --permission-mode acceptEdits --allowedTools "Bash" ;;
        codex) codex exec --full-auto "$prompt" ;;
        gemini) gemini --yolo -p "$prompt" ;;
        grok) grok -p "$prompt" ;;
        *) die "no command for agent $agent" ;;
    esac
}

if [ "$scripted" = 0 ]; then
    for a in "$first" "$second"; do
        command -v "$a" >/dev/null || die "$a is not installed (or use --scripted)"
    done
fi

# A small repo where the obvious approach is wrong: rounding every line breaks invoice totals.
repo=$(mktemp -d /tmp/relay-demo.XXXXXX)
name=relay-$(date +%s)
mkdir -p "$repo/shop"
cat >"$repo/shop/pricing.py" <<'PY'
"""Prices, discounts and totals for a cart."""
from decimal import Decimal, ROUND_HALF_UP

CENT = Decimal("0.01")


def line_total(price: Decimal, qty: int) -> Decimal:
    return price * qty


def total(lines: list[tuple[Decimal, int]]) -> Decimal:
    """Sum of all lines, rounded once to the cent (invoices do the same)."""
    subtotal = sum((line_total(p, q) for p, q in lines), Decimal(0))
    return subtotal.quantize(CENT, rounding=ROUND_HALF_UP)
PY
cat >"$repo/shop/test_pricing.py" <<'PY'
import unittest
from decimal import Decimal

from pricing import total


class TotalTest(unittest.TestCase):
    def test_total_rounds_once(self):
        third = Decimal("0.333")
        self.assertEqual(total([(third, 1)] * 3), Decimal("1.00"))


if __name__ == "__main__":
    unittest.main()
PY
git -C "$repo" init -q -b main
git -C "$repo" add -A
git -C "$repo" -c user.name=demo -c user.email=demo@example.com commit -qm "shop"

say "1. Import the repo and wire both agents"
(cd "$repo" && av init --name "$name" --for "$first" --for "$second")

say "2. $first starts the task"
goal="Orders of 10 or more units of one item get 5% off that line. Totals must still match invoices to the cent (shop/test_pricing.py)."
started=$(cd "$repo" && AV_AGENT=$first av task start --repo "$name" --title "Bulk discount" --goal "$goal")
ws=$(printf '%s\n' "$started" | sed -n 's/^workspace: //p')
task=$(printf '%s\n' "$started" | sed -n 's/^task: \([^ ]*\).*/\1/p')
[ -d "$ws" ] || die "the daemon's workspace $ws is not visible here"
echo "workspace: $ws"

if [ "$scripted" = 1 ]; then
    (cd "$ws" && AV_AGENT=$first av record --kind attempt \
        --summary "Tried rounding every line to the cent before summing: totals drift 1-2 cents from invoices" \
        --body "test_total_rounds_once fails: 3 x 0.333 gives 0.99, not 1.00. Round the discount only, and the total once." \
        --anchor shop/pricing.py:total)
    (cd "$ws" && AV_AGENT=$first av handoff --done "Explored where to round" \
        --remaining "Implement the discount in line_total; keep one rounding in total" >/dev/null)
else
    run_agent "$first" "You are working in $ws on this task: $goal
First try the obvious approach: round each line to the cent inside line_total. Run
'cd shop && python3 -m unittest' and see whether it passes. Whatever fails, record it as a dead end:
av record --kind attempt --summary '<what you tried and why it failed, one line>' --anchor shop/pricing.py:total
Then stop without finishing and hand off: av handoff --done '<what is done>' --remaining '<what is left>'"
fi

say "3. $second claims the task; this is the start of its context"
(cd "$ws" && AV_AGENT=$second av claim "$task" >/dev/null)
pack=$(cd "$ws" && AV_AGENT=$second av context --budget 2000)
printf '%s\n' "$pack" | sed -n '1,30p'
printf '%s\n' "$pack" | grep -q "attempt:" ||
    die "the pack does not show $first's dead end"

if [ "$scripted" = 0 ]; then
    say "4. $second finishes the task"
    run_agent "$second" "You are continuing a task another agent handed off, in $ws. Read the
context first (the av MCP server's get_context tool, or 'av context'): it lists what was already
tried and failed. Finish the task: $goal Run 'cd shop && python3 -m unittest', then 'av checkpoint'
and 'av submit'."
fi

say "Done"
echo "The dead end $first recorded reached $second in its first pack, across vendors."
echo "Full context: (cd $ws && av context --fresh)   Dashboard: ${AV_URL:-http://127.0.0.1:7420}/ui"
