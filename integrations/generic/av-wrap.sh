#!/usr/bin/env bash
# Runs any agent command inside Margincode: writes the task's context pack to $AV_CONTEXT_FILE
# (default: a file in $TMPDIR, outside the workspace so it is never snapshotted), runs the command
# with AV_CONTEXT_FILE exported (the command reads the pack from there), then checkpoints. Exits
# with the command's status. Example: av-wrap.sh sh -c 'my-agent --prompt-file "$AV_CONTEXT_FILE"'
set -uo pipefail
if [ $# -eq 0 ]; then
    echo "usage: av-wrap.sh <command> [args...]" >&2
    exit 2
fi
export AV_CONTEXT_FILE="${AV_CONTEXT_FILE:-${TMPDIR:-/tmp}/av-context-$$.md}"
# In a cargo workspace, builds go to the task workspace's own target/ (the nearest dir with a
# .av-task), whatever CARGO_TARGET_DIR the caller had: a dir shared by two workspaces of one repo
# makes `cargo test` in one run the other's test binary.
root="$PWD"
while [ "$root" != / ] && [ ! -f "$root/.av-task" ]; do root="$(dirname "$root")"; done
if [ -f "$root/.av-task" ] && [ -f "$root/Cargo.toml" ]; then
    export CARGO_TARGET_DIR="$root/target"
    unset CARGO_BUILD_TARGET_DIR
fi
if ! av context --budget "${AV_CONTEXT_BUDGET:-800}" >"$AV_CONTEXT_FILE"; then
    echo "av-wrap: no context pack (continuing without one)" >&2
fi
"$@"
status=$?
av checkpoint >/dev/null || echo "av-wrap: checkpoint failed" >&2
exit "$status"
