#!/bin/bash
# Amendment 2: make the Sonnet run tree /m2b/m0s/tree-s from the run tree /m2b/m0s/tree (7dd5208).
# haiku-tree.sh with the Sonnet model: the only change is the `model` line of the task files
# under bench/tasks-m0 and bench/tasks-relay. Run as root in m2b-harness. Exits non-zero if
# anything else differs.
set -euo pipefail
SRC=${1:-/m2b/m0s/tree}
DST=${2:-/m2b/m0s/tree-s}
MODEL=claude-sonnet-5-5
[ -e "$DST" ] && { echo "refused: $DST exists"; exit 2; }
cp -a "$SRC" "$DST"
n=0
for f in "$DST"/bench/tasks-m0/*.toml "$DST"/bench/tasks-relay/*.toml; do
  grep -q '^model = "deepseek-flash"$' "$f" || { echo "no model line: $f"; exit 1; }
  sed -i "s/^model = \"deepseek-flash\"$/model = \"$MODEL\"/" "$f"
  n=$((n + 1))
done
[ "$n" -eq 13 ] || { echo "expected 13 task files, changed $n"; exit 1; }
d=$(diff -r "$SRC" "$DST" || true)
changed=$(printf '%s\n' "$d" | grep -c "^> model = \"$MODEL\"$" || true)
other=$(printf '%s\n' "$d" | grep -E '^[<>]' | grep -v -E "^< model = \"deepseek-flash\"$|^> model = \"$MODEL\"$" || true)
[ "$changed" -eq 13 ] && [ -z "$other" ] || { echo "unexpected diff:"; printf '%s\n' "$d"; exit 1; }
echo "ok: $DST differs from $SRC in the model line of $n task files"
