#!/bin/bash
# M1 Part 2: commands 1 and 2 back to back, nothing between them. Run as root in lp-harness,
# with CLAUDE_API_KEY passed in by name. Each command's run and results directories are closed
# (mode 700) when it ends, so the next command's preflight finds them closed.
set -u
L=/m1/checks; C=/m1/m1cmd.sh
close() { chmod 700 /m1/bench-work/runs/$1 /m1/results/$2 2>/dev/null; }
run() { # name build out runid args...
  n=$1 b=$2 out=$3 id=$4; shift 4
  echo "$(date -u +%TZ) start $n" >> $L/chain.log
  $C $b "$@" --out /m1/results/$out > $L/$out.log 2>&1; rc=$?
  echo "$(date -u +%TZ) end $n exit=$rc" >> $L/chain.log
  close $id $out
  return $rc
}
A="bench/tasks-m0 --arms av --jit-hooks --reps 10 --wrong-reverted --session-cap-usd 0.08 --budget-usd 0.70"
# an error in command 1 (no row written) is an infrastructure failure: stop, fix, rerun it
run 1 P0 m1-p0 m1p0 $A || { touch $L/chain.done; exit 1; }
run 2 P1 m1-p1 m1p1 $A
touch $L/chain.done
