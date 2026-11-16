#!/bin/bash
# Haiku M0 chain: command 1, the gate, then commands 2 and 3 back to back. Run as root in lp-harness.
set -u
L=/m0/checks; C=/m0/m0hcmd.sh
close() { chmod 700 /m0/bench-work/runs/$1 /m0/results/$2 2>/dev/null; }
chmod 700 /m0/bench-work/runs/smokea /m0/results/smoke-a 2>/dev/null
run() { # name build out runid args...
  n=$1 b=$2 out=$3 id=$4; shift 4
  echo "$(date -u +%TZ) start $n" >> $L/h-chain.log
  $C $b "$@" --out /m0/results/$out > $L/h-$out.log 2>&1; rc=$?
  echo "$(date -u +%TZ) end $n exit=$rc" >> $L/h-chain.log
  close $id $out
  return $rc
}
run 1 B m0-cold m0cold bench/tasks-m0 --arms cold --jit-hooks --reps 10 --session-cap-usd 0.08 --budget-usd 0.40 || { touch $L/h-123.done; exit 1; }
python3 - <<'PY' >> $L/h-chain.log 2>&1 || { echo "gate FAILED: commands 2-3 not run" >> $L/h-chain.log; touch $L/h-123.done; exit 1; }
import json,collections,sys
ok=collections.Counter(); n=collections.Counter()
for l in open("/m0/results/m0-cold/rows.jsonl"):
    r=json.loads(l); n[r["task"]]+=1; ok[r["task"]]+= bool(r["success"])
print("gate:", {t:f"{ok[t]}/{n[t]}" for t in sorted(n)})
sys.exit(0 if len(n)==3 and all(ok[t]>=8 for t in n) else 1)
PY
echo "gate passes" >> $L/h-chain.log
run 2 A m0-av-before m0avbefore bench/tasks-m0 --arms av --jit-hooks --reps 10 --session-cap-usd 0.08 --budget-usd 0.70
run 3 B m0-wrong m0wrong bench/tasks-m0 --arms notes,av --jit-hooks --reps 10 --session-cap-usd 0.08 --budget-usd 1.40
touch $L/h-123.done
