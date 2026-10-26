#!/usr/bin/env bash
# Claude Code PostToolUse hook (Edit|Write|MultiEdit): checkpoints the task at most every 2 s, so
# the index and the other agents' live-work view follow the edits. Never fails the tool call.
command -v av >/dev/null 2>&1 || exit 0
av hook post-edit >/dev/null
exit 0
