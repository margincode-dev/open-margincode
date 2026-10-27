#!/usr/bin/env bash
# Builds the tallyho benchmark repo (bench/tasks-tallyho): `main` is the base tree, and each
# fixes/<name>.patch becomes a `fix-<name>` branch, the reference fix its hidden tests pass on.
# Usage: make-repo.sh [DEST]   (default ~/bench-repos/tallyho, which the task files point at)
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
dest=${1:-$HOME/bench-repos/tallyho}
if [ -e "$dest" ]; then
    echo "make-repo.sh: $dest exists; remove it or pass another DEST" >&2
    exit 1
fi
git=(git -c user.name=scholion-bench -c user.email=bench@localhost -c commit.gpgsign=false)
mkdir -p "$dest"
cp -R "$here/src/." "$dest/"
cd "$dest"
"${git[@]}" init -q -b main
"${git[@]}" add -A
"${git[@]}" commit -qm "tallyho 0.4.0"
for p in "$here"/fixes/*.patch; do
    b="fix-$(basename "$p" .patch)"
    "${git[@]}" checkout -q -b "$b" main
    "${git[@]}" apply "$p"
    "${git[@]}" commit -qam "$b"
done
"${git[@]}" checkout -q main
echo "tallyho benchmark repo: $dest"
