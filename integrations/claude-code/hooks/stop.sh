#!/usr/bin/env bash
# Claude Code Stop hook: when the task has changes no record explains, asks the agent once per
# session to record its decisions and dead ends before it stops. Never fails the agent.
command -v av >/dev/null 2>&1 || exit 0
av hook stop
exit 0
