#!/usr/bin/env bash
# Builds the scholion benchmark repo (bench/tasks-scholion): this repository at BASE as `main`
# (the reference fix of every task), and each bugs/<name>.patch as a `bug-<name>` branch, the
# commit a session starts from.
# Usage: make-repo.sh [DEST]   (default ~/bench-repos/scholion-t, which the task files point at)
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
dest=${1:-$HOME/bench-repos/scholion-t}
base=0a08e6e
if [ -e "$dest" ]; then
    echo "make-repo.sh: $dest exists; remove it or pass another DEST" >&2
    exit 1
fi
git=(git -c user.name=scholion-bench -c user.email=bench@localhost -c commit.gpgsign=false)
"${git[@]}" clone -q --no-hardlinks "$(git -C "$here" rev-parse --show-toplevel)" "$dest"
cd "$dest"
"${git[@]}" checkout -q -B main "$base"
for p in "$here"/bugs/*.patch; do
    b="bug-$(basename "$p" .patch)"
    "${git[@]}" checkout -q -b "$b" main
    "${git[@]}" apply "$p"
    "${git[@]}" commit -qam "$b"
done
"${git[@]}" checkout -q main
echo "scholion benchmark repo: $dest"
