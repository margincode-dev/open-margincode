#!/bin/bash
# M2 stage B: run one margincode-bench command on Claude Haiku 5.5 (M1 Part 2's m1cmd.sh, for
# one build). usage: m2bcmd.sh run|first-packs|dream-freeze <args...>   (root in m2b-harness)
# The harness is /m2b/H/bin/margincode-bench (root only); av and the daemon are /m2b/bin. The key
# comes in as CLAUDE_API_KEY (docker exec -e CLAUDE_API_KEY, by name) and is never written down.
set -u
[ -e /m2b/checks/STOP ] && { echo "refused: /m2b/checks/STOP"; exit 3; }
sub=$1; shift
find /tmp -mindepth 1 -delete
export HOME=/m2b PATH=/m2b/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
export DISABLE_AUTOUPDATER=1
unset ANTHROPIC_BASE_URL ANTHROPIC_AUTH_TOKEN DEEPSEEK_API_KEY ANTHROPIC_API_KEY
if [ -n "${CLAUDE_API_KEY:-}" ]; then export ANTHROPIC_API_KEY="$CLAUDE_API_KEY"; fi
unset CLAUDE_API_KEY
export ANTHROPIC_DEFAULT_HAIKU_MODEL=claude-haiku-5-5 ANTHROPIC_SMALL_FAST_MODEL=claude-haiku-5-5 \
  CLAUDE_CODE_SUBAGENT_MODEL=claude-haiku-5-5
S="--av-url http://127.0.0.1:7490 --work-root /m2b/bench-work --import-dir /m2b/bench-work/src \
  --import-mount /m2b/bench-work/src --av-bin /m2b/bin/av"
cd /m2b/tree-h
case "$sub" in
  run)
    [ -n "${ANTHROPIC_API_KEY:-}" ] || { echo "refused: CLAUDE_API_KEY not passed in"; exit 3; }
    /m2b/H/bin/margincode-bench run "$@" --price-per-mtok 0.10,0.125,0.01,0.50 $S \
      --agent-user benchagent --av-ws-root /m2b/bench-work/ws --confirm ;;
  dream-freeze)
    [ -n "${ANTHROPIC_API_KEY:-}" ] || { echo "refused: CLAUDE_API_KEY not passed in"; exit 3; }
    M2_MODEL=claude-sonnet-5-5 M2_RUN_CAP=0.25 /m2b/H/bin/margincode-bench dream-freeze "$@" $S ;;
  first-packs)
    unset ANTHROPIC_API_KEY
    /m2b/H/bin/margincode-bench first-packs "$@" $S ;;
  *) echo "usage: m2bcmd.sh run|first-packs|dream-freeze ..."; exit 2 ;;
esac
