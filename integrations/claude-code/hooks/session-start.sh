#!/usr/bin/env bash
# Claude Code SessionStart hook: prints a small Scholion context pack for the task of the session's
# directory (stdout is added to the session). Silent outside a task; never fails the session.
command -v av >/dev/null 2>&1 || exit 0
av hook session-start
exit 0
