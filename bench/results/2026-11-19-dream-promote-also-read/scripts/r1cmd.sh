#!/bin/bash
# HANDOFF "Next" 2: one real dream with --promote and one with --also-read (root in m2b-harness).
# usage: r1cmd.sh seed TASK OUT | dream REPO HOME [av dream flags...]
# seed: stage B's dream-freeze seeding of one task store, with agent commands that fail at once
#   (no model, $0), so the seeded repository stays and no verdict is written.
# dream: av dream on that repository with stage B's dreamer (Sonnet 5.5, $0.25 per call).
# The key comes in as CLAUDE_API_KEY (docker exec -e CLAUDE_API_KEY, by name), never written down.
set -u
[ -e /m2b/checks/STOP ] && { echo "refused: /m2b/checks/STOP"; exit 3; }
sub=$1; shift
export HOME=/m2b PATH=/m2b/main-21f30dc/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
export DISABLE_AUTOUPDATER=1
unset ANTHROPIC_BASE_URL ANTHROPIC_AUTH_TOKEN DEEPSEEK_API_KEY ANTHROPIC_API_KEY AV_TOKEN
if [ -n "${CLAUDE_API_KEY:-}" ]; then export ANTHROPIC_API_KEY="$CLAUDE_API_KEY"; fi
unset CLAUDE_API_KEY
URL=http://127.0.0.1:7491
S="--av-url $URL --work-root /m2b/r1/bench-work --import-dir /m2b/r1/bench-work/src \
  --import-mount /m2b/r1/bench-work/src --av-bin /m2b/main-21f30dc/bin/av"
cd /m2b/main-21f30dc/tree-h
case "$sub" in
  seed)
    unset ANTHROPIC_API_KEY
    task=$1 out=$2
    /m2b/main-21f30dc/H/bin/margincode-bench dream-freeze bench/tasks-m0 --only "$task" --limit 50 \
      --dream-cmd 'exit 1' --verify-cmd 'exit 1' --out "$out" $S ;;
  dream)
    [ -n "${ANTHROPIC_API_KEY:-}" ] || { echo "refused: CLAUDE_API_KEY not passed in"; exit 3; }
    repo=$1 home=$2; shift 2
    mkdir -p "$home"
    AV_URL=$URL AV_AGENT=dreamer AV_HOME="$home" M2_MODEL=claude-sonnet-5-5 M2_RUN_CAP=0.25 \
      /m2b/main-21f30dc/bin/av dream --repo "$repo" --limit 50 --timeout 1800 "$@" \
      -- sh -c 'sh /m2b/dreamer.sh' ;;
  *) echo "usage: r1cmd.sh seed TASK OUT | dream REPO HOME [flags]"; exit 2 ;;
esac
