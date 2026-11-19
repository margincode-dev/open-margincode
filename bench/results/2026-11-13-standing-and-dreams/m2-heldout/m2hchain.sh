#!/bin/bash
# M2 stage B, amendment 3: one command of the run (the smoke or one arm), logged to chain.log.
# Root in m2b-harness, CLAUDE_API_KEY passed in by name. usage: m2hchain.sh STEP BUDGET
# STEP: smoke | raw-ho | dreamed-ho | raw-m0c | dreamed-m0c | cold-ho. BUDGET is the smaller of
# the written budget and $5.00 less what has been spent, worked out before the call.
# The step's run and results directories, and the kept workspaces, are closed (mode 700) when it ends.
set -u
L=/m2b/r3/checks; C=/m2b/r3/m2hcmd.sh; R=/m2b/r3/results
step=$1 budget=$2
close() { chmod 700 $R/$1; for d in /m2b/r3/bench-work/runs/* /m2b/r3/bench-work/ws/*; do [ -e "$d" ] && chmod 700 "$d"; done; }
HO="bench/tasks-heldout --arms av --jit-hooks --reps 20 --session-cap-usd 0.08 --budget-usd $budget"
M0="bench/tasks-m0 --arms av --jit-hooks --reps 10 --session-cap-usd 0.08 --budget-usd $budget"
case "$step" in
  smoke) out=smoke-r3; a="bench/tasks-relay/order-ids.toml --arms av --jit-hooks --reps 1 --session-cap-usd 0.08 --budget-usd $budget" ;;
  raw-ho) out=m2-raw-ho; a="$HO" ;;
  dreamed-ho) out=m2-dreamed-ho; a="$HO --dreamed $R/dreamed-ho.json" ;;
  raw-m0c) out=m2-raw-m0c; a="$M0" ;;
  dreamed-m0c) out=m2-dreamed-m0c; a="$M0 --dreamed $R/dreamed-a2.json" ;;
  cold-ho) out=m2-cold-ho; a="bench/tasks-heldout --arms cold --reps 10 --session-cap-usd 0.08 --budget-usd $budget" ;;
  *) echo "usage: m2hchain.sh smoke|raw-ho|dreamed-ho|raw-m0c|dreamed-m0c|cold-ho BUDGET"; exit 2 ;;
esac
echo "$(date -u +%TZ) start $step budget=$budget" >> $L/chain.log
$C run $a --out $R/$out > $L/$out.log 2>&1; rc=$?
echo "$(date -u +%TZ) end $step exit=$rc" >> $L/chain.log
close $out
touch $L/$step.done
exit $rc
