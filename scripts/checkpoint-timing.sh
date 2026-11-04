#!/usr/bin/env bash
# Plan 14's measurement: times `av checkpoint` N times in DIR, appending a line to FILE before
# each, and prints every wall time and the median in milliseconds. Run it once in a task workspace
# on the daemon's host (a local checkpoint) and once in a checkout made by `av remote clone` on
# another host (a remote one, patch and round trip included), then compare the medians.
#
#   scripts/checkpoint-timing.sh DIR FILE [N]      # AV_URL / AV_TOKEN as for any av command
set -euo pipefail
dir=${1:?usage: checkpoint-timing.sh DIR FILE [N]}
file=${2:?usage: checkpoint-timing.sh DIR FILE [N]}
n=${3:-10}
av=${AV:-av}
cd "$dir"
times=()
for i in $(seq 1 "$n"); do
  echo "// checkpoint-timing $i $$" >>"$file"
  start=$(date +%s%N)
  "$av" checkpoint >/dev/null
  end=$(date +%s%N)
  ms=$(((end - start) / 1000000))
  times+=("$ms")
  echo "run $i: ${ms} ms"
done
median=$(printf '%s\n' "${times[@]}" | sort -n | awk '{a[NR]=$1} END {print (NR%2 ? a[(NR+1)/2] : (a[NR/2]+a[NR/2+1])/2)}')
echo "median: ${median} ms over $n checkpoints in $dir"
