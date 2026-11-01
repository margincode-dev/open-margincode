#!/usr/bin/env bash
# Claude Code Stop hook: when the task has changes no record explains, asks the agent once per
# session to record its decisions and dead ends before it stops; then, if av has work that fits
# the agent, keeps it going (waits up to AV_HOOK_WAIT_SECS, default 600, 0 = off; install.sh sets
# this hook's timeout above that). Never fails the agent.
command -v av >/dev/null 2>&1 || exit 0
av hook stop
exit 0
