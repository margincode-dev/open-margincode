#!/bin/bash
# Amendment 1: run one M0 command on Claude Haiku 5.5.
# usage: m0hcmd.sh A|B <scholion-bench args...>   (run as root in lp-harness)
# The key comes in as CLAUDE_API_KEY (docker exec -e CLAUDE_API_KEY, by name) and is never
# written down. Each command closes the other build's bin and empties /tmp (free check 5).
set -u
[ -e /m0/checks/STOP-H ] && { echo "refused: /m0/checks/STOP-H"; exit 3; }
[ -n "${CLAUDE_API_KEY:-}" ] || { echo "refused: CLAUDE_API_KEY not passed in"; exit 3; }
b=$1; shift
other=$([ "$b" = A ] && echo B || echo A)
lc=$(echo "$b" | tr AB ab); port=$([ "$b" = A ] && echo 7461 || echo 7462)
chmod 755 /m0/$b/bin; chmod 700 /m0/$other/bin
find /tmp -mindepth 1 -delete
export HOME=/m0 PATH=/m0/$b/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
unset ANTHROPIC_BASE_URL ANTHROPIC_AUTH_TOKEN DEEPSEEK_API_KEY
export ANTHROPIC_API_KEY="$CLAUDE_API_KEY"
unset CLAUDE_API_KEY
export ANTHROPIC_DEFAULT_HAIKU_MODEL=claude-haiku-5-5 ANTHROPIC_SMALL_FAST_MODEL=claude-haiku-5-5 \
  CLAUDE_CODE_SUBAGENT_MODEL=claude-haiku-5-5
cd /m0/tree-h
/m0/$b/bin/scholion-bench run "$@" --price-per-mtok 0.10,0.125,0.01,0.50 \
  --av-url http://127.0.0.1:$port --av-bin /m0/$b/bin/av --work-root /m0/bench-work \
  --import-dir /m0/bench-work/src --import-mount /m0/bench-work/src \
  --agent-user benchagent --av-ws-root /m0/bench-work/ws-$lc --confirm
rc=$?
chmod 755 /m0/A/bin /m0/B/bin
exit $rc
