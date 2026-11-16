#!/bin/bash
# Amendment 1: make the Haiku run tree /m0/tree-h from the run tree /m0/tree (7dd5208).
# The only change is the `model` line of the task files under bench/tasks-m0 and
# bench/tasks-relay. Run as root in lp-harness. Exits non-zero if anything else differs.
set -euo pipefail
SRC=${1:-/m0/tree}
DST=${2:-/m0/tree-h}
MODEL=claude-haiku-5-5
[ -e "$DST" ] && { echo "refused: $DST exists"; exit 2; }
cp -a "$SRC" "$DST"
n=0
for f in "$DST"/bench/tasks-m0/*.toml "$DST"/bench/tasks-relay/*.toml; do
  grep -q '^model = "deepseek-flash"$' "$f" || { echo "no model line: $f"; exit 1; }
  sed -i "s/^model = \"deepseek-flash\"$/model = \"$MODEL\"/" "$f"
  n=$((n + 1))
done
[ "$n" -eq 13 ] || { echo "expected 13 task files, changed $n"; exit 1; }
# diff -r must show exactly the 13 changed lines and nothing else.
d=$(diff -r "$SRC" "$DST" || true)
changed=$(printf '%s\n' "$d" | grep -c "^> model = \"$MODEL\"$" || true)
other=$(printf '%s\n' "$d" | grep -E '^[<>]' | grep -v -E "^< model = \"deepseek-flash\"$|^> model = \"$MODEL\"$" || true)
[ "$changed" -eq 13 ] && [ -z "$other" ] || { echo "unexpected diff:"; printf '%s\n' "$d"; exit 1; }
echo "ok: $DST differs from $SRC in the model line of $n task files"
