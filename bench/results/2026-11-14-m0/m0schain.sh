#!/bin/bash
# Amendment 2: the Sonnet run's paid commands, logged to /m2b/m0s/checks/chain.log.
# Root in m2b-harness, CLAUDE_API_KEY passed in by name.
#   m0schain.sh smoke A|B CAP BUDGET [OUT]   one smoke session (bench/tasks-relay/order-ids.toml, av)
#   m0schain.sh arms CAP CEILING RATIO       av-before (build A), then av-phase0 (build B), back to back
# In `arms` each command's budget is (CEILING - spent so far) / RATIO, rounded down to the cent,
# with "spent" as sonnet_spend.py counts it (costs only, no accepted count is read). Each
# command's run, results and workspace directories are closed (mode 700) when it ends.
set -u
R=/m2b/m0s; L=$R/checks; C=$R/m0scmd.sh
close() { chmod 700 $R/results/$1 $R/bench-work/runs/$(echo $1 | tr -d -) 2>/dev/null
  for d in $R/bench-work/ws-a/* $R/bench-work/ws-b/*; do [ -e "$d" ] && chmod 700 "$d"; done; }
one() { # build out budget args...
  b=$1 out=$2 budget=$3; shift 3
  echo "$(date -u +%FT%TZ) start $out build=$b budget=$budget" >> $L/chain.log
  $C $b run "$@" --budget-usd $budget --out $R/results/$out > $L/$out.log 2>&1; rc=$?
  echo "$(date -u +%FT%TZ) end $out exit=$rc" >> $L/chain.log
  close $out
  return $rc
}
room() { # ceiling ratio -> budget in dollars, rounded down to the cent
  python3 $R/sonnet_spend.py $R --ceiling $1 | tail -1 >> $L/chain.log
  python3 - "$1" "$2" <<'PY'
import json, math, sys
from pathlib import Path
sys.path.insert(0, "/m2b/m0s")
import sonnet_spend as s
root = Path("/m2b/m0s"); total = 0.0
for out in s.OUTS:
    p = root / "results" / out / "rows.jsonl"
    if not p.exists():
        continue
    run = root / "bench-work" / "runs" / out.replace("-", "")
    for line in p.read_text().splitlines():
        if line:
            r = json.loads(line)
            cc = s.session(run / f"{r['task']}-{r['arm']}-r{r['rep']}")["cc"] or 0.0
            total += max(r.get("cost_usd") or 0.0, cc)
print(f"{math.floor((float(sys.argv[1]) - total) / float(sys.argv[2]) * 100) / 100:.2f}")
PY
}
case "${1:-}" in
  smoke)
    b=$2 cap=$3 budget=$4 out=${5:-smoke-s-$(echo $2 | tr AB ab)}
    one $b $out $budget bench/tasks-relay/order-ids.toml --arms av --jit-hooks --reps 1 \
      --session-cap-usd $cap ;;
  arms)
    cap=$2 ceiling=$3 ratio=$4
    M0="bench/tasks-m0 --arms av --jit-hooks --reps 10 --session-cap-usd $cap"
    b1=$(room $ceiling $ratio)
    one A m0s-av-before $b1 $M0
    b2=$(room $ceiling $ratio)
    one B m0s-phase0 $b2 $M0
    touch $L/arms.done ;;
  *) echo "usage: m0schain.sh smoke A|B CAP BUDGET [OUT] | arms CAP CEILING RATIO"; exit 2 ;;
esac
