#!/bin/bash
# M2 stage B, amendment 2: one arm (raw or dreamed) of 90 sessions. Stage B's m2bchain.sh, one
# arm per call, because the host's build lock is checked between the arms. Root in m2b-harness,
# CLAUDE_API_KEY passed in by name. usage: m2rchain.sh raw|dreamed BUDGET
# The arm's run and results directories, and the kept workspaces, are closed (mode 700) when it ends.
set -u
L=/m2b/r2/checks; C=/m2b/r2/m2rcmd.sh; D=/m2b/r2/results/dreamed.json
arm=$1 budget=$2
close() { chmod 700 /m2b/r2/results/$1; for d in /m2b/r2/bench-work/runs/* /m2b/r2/bench-work/ws/*; do [ -e "$d" ] && chmod 700 "$d"; done; }
A="bench/tasks-m0 --arms av --jit-hooks --reps 30 --session-cap-usd 0.08 --budget-usd $budget"
case "$arm" in
  raw) out=m2-raw-r2; extra="" ;;
  dreamed) out=m2-dreamed-r2; extra="--dreamed $D" ;;
  *) echo "usage: m2rchain.sh raw|dreamed BUDGET"; exit 2 ;;
esac
echo "$(date -u +%TZ) start $arm budget=$budget" >> $L/chain.log
$C run $A $extra --out /m2b/r2/results/$out > $L/$out.log 2>&1; rc=$?
echo "$(date -u +%TZ) end $arm exit=$rc" >> $L/chain.log
close $out
touch $L/$arm.done
exit $rc
