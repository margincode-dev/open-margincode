#!/usr/bin/env bash
# Claude Code PreToolUse hook (Read|Edit|MultiEdit): adds what other tasks recorded about the file
# (decisions, dead ends) next to it, each record once per session. Never fails the tool call.
command -v av >/dev/null 2>&1 || exit 0
av hook pre-read
exit 0
