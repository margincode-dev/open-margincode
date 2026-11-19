#!/bin/bash
# Amendment 2: run one M0 command on Claude Sonnet 5.5. m0hcmd.sh with this run's root
# (/m2b/m0s in m2b-harness), the Sonnet tree and prices, and the Sonnet model pinned in every
# model variable Claude Code reads.
# usage: m0scmd.sh A|B run|dry|first-packs|check <scholion-bench args...>   (root in m2b-harness)
# `run` needs the key as CLAUDE_API_KEY (docker exec -e CLAUDE_API_KEY, by name); it is never
# written down. Each command opens only its own build's bin, closes the other's (free check 5)
# and empties /tmp.
set -u
R=/m2b/m0s
[ -e $R/checks/STOP ] && { echo "refused: $R/checks/STOP"; exit 3; }
b=$1 sub=$2; shift 2
case "$b" in A) other=B port=7461 lc=a ;; B) other=A port=7462 lc=b ;; *) echo "usage: m0scmd.sh A|B ..."; exit 2 ;; esac
chmod 755 $R/$b/bin; chmod 700 $R/$other/bin
find /tmp -mindepth 1 -delete
export HOME=$R PATH=$R/$b/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
export DISABLE_AUTOUPDATER=1
unset ANTHROPIC_BASE_URL ANTHROPIC_AUTH_TOKEN DEEPSEEK_API_KEY ANTHROPIC_API_KEY
if [ "$sub" = run ] && [ -n "${CLAUDE_API_KEY:-}" ]; then export ANTHROPIC_API_KEY="$CLAUDE_API_KEY"; fi
unset CLAUDE_API_KEY
M=claude-sonnet-5-5
export ANTHROPIC_MODEL=$M ANTHROPIC_DEFAULT_HAIKU_MODEL=$M ANTHROPIC_DEFAULT_SONNET_MODEL=$M \
  ANTHROPIC_SMALL_FAST_MODEL=$M CLAUDE_CODE_SUBAGENT_MODEL=$M
S="--av-url http://127.0.0.1:$port --av-bin $R/$b/bin/av --work-root $R/bench-work \
  --import-dir $R/bench-work/src --import-mount $R/bench-work/src"
ISO="--agent-user benchagent --av-ws-root $R/bench-work/ws-$lc"
P="--price-per-mtok 2.00,2.50,0.10,10.00"
H=$R/$b/bin/scholion-bench
cd $R/tree-s
case "$sub" in
  run)
    [ -n "${ANTHROPIC_API_KEY:-}" ] || { echo "refused: CLAUDE_API_KEY not passed in"; rc=3; }
    [ "${rc:-0}" = 3 ] || { $H run "$@" $P $S $ISO --confirm; rc=$?; } ;;
  dry) $H run "$@" $P $S $ISO; rc=$? ;;
  first-packs) $H first-packs "$@" $S; rc=$? ;;
  check) $H check "$@"; rc=$? ;;
  *) echo "usage: m0scmd.sh A|B run|dry|first-packs|check ..."; rc=2 ;;
esac
chmod 755 $R/A/bin $R/B/bin
exit $rc
