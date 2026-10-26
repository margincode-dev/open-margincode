#!/usr/bin/env bash
# Runs any agent command inside Scholion: writes the task's context pack to $AV_CONTEXT_FILE
# (default: a file in $TMPDIR, outside the workspace so it is never snapshotted), runs the command
# with AV_CONTEXT_FILE exported (the command reads the pack from there), then checkpoints. Exits
# with the command's status. Example: av-wrap.sh sh -c 'my-agent --prompt-file "$AV_CONTEXT_FILE"'
set -uo pipefail
if [ $# -eq 0 ]; then
    echo "usage: av-wrap.sh <command> [args...]" >&2
    exit 2
fi
export AV_CONTEXT_FILE="${AV_CONTEXT_FILE:-${TMPDIR:-/tmp}/av-context-$$.md}"
if ! av context --budget "${AV_CONTEXT_BUDGET:-4000}" >"$AV_CONTEXT_FILE"; then
    echo "av-wrap: no context pack (continuing without one)" >&2
fi
"$@"
status=$?
av checkpoint >/dev/null || echo "av-wrap: checkpoint failed" >&2
exit "$status"
