#!/bin/bash
# M2 stage B: arm raw, then arm dreamed, back to back, nothing between them. Root in m2b-harness,
# CLAUDE_API_KEY passed in by name. Each command's run and results directories are closed
# (mode 700) when it ends.
set -u
L=/m2b/checks; C=/m2b/m2bcmd.sh; D=/m2b/results/m2-stage-b/dreamed.json
close() { chmod 700 /m2b/results/$1; for d in /m2b/bench-work/runs/*; do chmod 700 "$d"; done; }
run() { # name out args...
  n=$1 out=$2; shift 2
  echo "$(date -u +%TZ) start $n" >> $L/chain.log
  $C run "$@" --out /m2b/results/$out > $L/$out.log 2>&1; rc=$?
  echo "$(date -u +%TZ) end $n exit=$rc" >> $L/chain.log
  close $out
  return $rc
}
A="bench/tasks-m0 --arms av --jit-hooks --reps 10 --session-cap-usd 0.08 --budget-usd 0.70"
run raw m2-raw $A || { touch $L/chain.done; exit 1; }
run dreamed m2-dreamed $A --dreamed $D
touch $L/chain.done
