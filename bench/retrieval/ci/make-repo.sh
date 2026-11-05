#!/usr/bin/env bash
# Builds the E1 CI fixture as a git repo and prints its tasks as JSON lines (name, repo_path,
# base_rev, fix_rev, goal) for `scholion-bench retrieval`, which needs the revisions that
# tasks.jsonl leaves out. The repo is the one `tasks()` in
# crates/scholion-bench/tests/retrieval_gate.rs builds: `main` is repo/ in one commit, and each
# task's fix (fixes/NAME.patch) is one commit on its own branch NAME off `main`.
# Usage: make-repo.sh DEST > tasks.jsonl   (the nightly workflow runs it; needs git and python3)
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
dest=${1:?usage: make-repo.sh DEST > tasks.jsonl}
if [ -e "$dest" ]; then
    echo "make-repo.sh: $dest exists; remove it or pass another DEST" >&2
    exit 1
fi
git=(git -c user.name=scholion-bench -c user.email=bench@localhost -c commit.gpgsign=false
     -c core.autocrlf=false)
mkdir -p "$dest"
cp -R "$here/repo/." "$dest/"
dest=$(cd "$dest" && pwd)
cd "$dest"
"${git[@]}" init -q -b main
"${git[@]}" add -A
"${git[@]}" commit -qm base
base=$("${git[@]}" rev-parse HEAD)
while IFS= read -r line || [ -n "$line" ]; do
    [ -n "${line//[[:space:]]/}" ] || continue
    name=$(python3 -c 'import json, sys; print(json.loads(sys.argv[1])["name"])' "$line")
    "${git[@]}" checkout -q -b "$name" "$base"
    "${git[@]}" apply "$here/fixes/$name.patch"
    "${git[@]}" commit -qam "$name"
    fix=$("${git[@]}" rev-parse HEAD)
    python3 -c '
import json, sys
t = json.loads(sys.argv[1])
print(json.dumps({"name": t["name"], "repo_path": sys.argv[2], "base_rev": sys.argv[3],
                  "fix_rev": sys.argv[4], "goal": t["goal"]}))
' "$line" "$dest" "$base" "$fix"
done < "$here/tasks.jsonl"
"${git[@]}" checkout -q main
