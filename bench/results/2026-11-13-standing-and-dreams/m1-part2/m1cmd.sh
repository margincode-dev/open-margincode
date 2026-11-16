#!/bin/bash
# M1 Part 2: run one command on Claude Haiku 5.5 (M0's m0hcmd.sh, for this run's builds).
# usage: m1cmd.sh P0|P1 <margincode-bench run args...>   (run as root in lp-harness)
# One harness for both arms (/m1/H/bin/margincode-bench, main at 7dd5208); the daemon and the
# av the hooks and the agent run are the build's (/m1/P0/bin or /m1/P1/bin). The key comes in as
# CLAUDE_API_KEY (docker exec -e CLAUDE_API_KEY, by name) and is never written down. Each command
# closes the other build's bin and empties /tmp.
set -u
[ -e /m1/checks/STOP ] && { echo "refused: /m1/checks/STOP"; exit 3; }
[ -n "${CLAUDE_API_KEY:-}" ] || { echo "refused: CLAUDE_API_KEY not passed in"; exit 3; }
b=$1; shift
case "$b" in P0) other=P1 port=7481 ws=ws-p0 ;; P1) other=P0 port=7482 ws=ws-p1 ;;
  *) echo "usage: m1cmd.sh P0|P1 ..."; exit 2 ;; esac
chmod 755 /m1/$b/bin; chmod 700 /m1/$other/bin
find /tmp -mindepth 1 -delete
export HOME=/m1 PATH=/m1/$b/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
unset ANTHROPIC_BASE_URL ANTHROPIC_AUTH_TOKEN DEEPSEEK_API_KEY
export ANTHROPIC_API_KEY="$CLAUDE_API_KEY"
unset CLAUDE_API_KEY
export ANTHROPIC_DEFAULT_HAIKU_MODEL=claude-haiku-5-5 ANTHROPIC_SMALL_FAST_MODEL=claude-haiku-5-5 \
  CLAUDE_CODE_SUBAGENT_MODEL=claude-haiku-5-5
cd /m1/tree-h
/m1/H/bin/margincode-bench run "$@" --price-per-mtok 0.10,0.125,0.01,0.50 \
  --av-url http://127.0.0.1:$port --av-bin /m1/$b/bin/av --work-root /m1/bench-work \
  --import-dir /m1/bench-work/src --import-mount /m1/bench-work/src \
  --agent-user benchagent --av-ws-root /m1/bench-work/$ws --confirm
rc=$?
chmod 755 /m1/P0/bin /m1/P1/bin
exit $rc
