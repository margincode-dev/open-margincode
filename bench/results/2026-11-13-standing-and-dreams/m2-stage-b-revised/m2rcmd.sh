#!/bin/bash
# M2 stage B, amendment 2: run one margincode-bench command on Claude Haiku 5.5. Stage B's
# m2bcmd.sh with this run's build (main 21f30dc) and directories (/m2b/r2, daemon :7492).
# usage: m2rcmd.sh run|first-packs|dream-freeze <args...>   (root in m2b-harness)
# The harness is /m2b/main-21f30dc/H/bin/margincode-bench (root only); av and the daemon are
# /m2b/main-21f30dc/bin. The key comes in as CLAUDE_API_KEY (docker exec -e CLAUDE_API_KEY, by
# name) and is never written down.
set -u
[ -e /m2b/checks/STOP ] && { echo "refused: /m2b/checks/STOP"; exit 3; }
sub=$1; shift
find /tmp -mindepth 1 -delete
export HOME=/m2b PATH=/m2b/main-21f30dc/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
export DISABLE_AUTOUPDATER=1
unset ANTHROPIC_BASE_URL ANTHROPIC_AUTH_TOKEN DEEPSEEK_API_KEY ANTHROPIC_API_KEY
if [ -n "${CLAUDE_API_KEY:-}" ]; then export ANTHROPIC_API_KEY="$CLAUDE_API_KEY"; fi
unset CLAUDE_API_KEY
export ANTHROPIC_DEFAULT_HAIKU_MODEL=claude-haiku-5-5 ANTHROPIC_SMALL_FAST_MODEL=claude-haiku-5-5 \
  CLAUDE_CODE_SUBAGENT_MODEL=claude-haiku-5-5
S="--av-url http://127.0.0.1:7492 --work-root /m2b/r2/bench-work --import-dir /m2b/r2/bench-work/src \
  --import-mount /m2b/r2/bench-work/src --av-bin /m2b/main-21f30dc/bin/av"
H=/m2b/main-21f30dc/H/bin/margincode-bench
cd /m2b/main-21f30dc/tree-h
case "$sub" in
  run)
    [ -n "${ANTHROPIC_API_KEY:-}" ] || { echo "refused: CLAUDE_API_KEY not passed in"; exit 3; }
    $H run "$@" --price-per-mtok 0.10,0.125,0.01,0.50 $S \
      --agent-user benchagent --av-ws-root /m2b/r2/bench-work/ws --confirm ;;
  dream-freeze)
    [ -n "${ANTHROPIC_API_KEY:-}" ] || { echo "refused: CLAUDE_API_KEY not passed in"; exit 3; }
    M2_MODEL=claude-sonnet-5-5 M2_RUN_CAP=0.25 $H dream-freeze "$@" $S ;;
  first-packs)
    unset ANTHROPIC_API_KEY
    $H first-packs "$@" $S ;;
  *) echo "usage: m2rcmd.sh run|first-packs|dream-freeze ..."; exit 2 ;;
esac
