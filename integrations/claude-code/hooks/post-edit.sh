#!/usr/bin/env bash
# Claude Code PostToolUse hook (Edit|Write|MultiEdit): checkpoints the task at most every 2 s, so
# the index and the other agents' live-work view follow the edits. Never fails the tool call.
command -v av >/dev/null 2>&1 || exit 0
# its stdout is the hook's JSON answer (notices, a takeover), which Claude Code reads
av hook post-edit
exit 0
