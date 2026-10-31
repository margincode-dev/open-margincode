#!/usr/bin/env bash
# Routing demo: three agents pull work from one backlog and each gets what fits it. Claude Code
# takes the code task, Grok the image task, Muse the untagged one; then Claude hands a task off
# because the rest needs images, and Grok (not Muse) is the one offered it.
#
#   scripts/route-demo.sh --scripted            # no agents: `av` commands stand in for them (free)
#   scripts/route-demo.sh                       # real claude, grok and muse CLIs (their subscriptions)
#   scripts/route-demo.sh --agents "claude codex muse"
#
# Needs `av` on PATH and a daemon at $AV_URL (default http://127.0.0.1:7420; `av up` starts one).
# The daemon must see this machine's /tmp: lite mode does, the Docker stack does not.
# --scripted spends nothing and never touches your agents' config files (it uses `av repo add`).
# Real agents spend quota on their own accounts; each is told to take its next task and stop.
set -euo pipefail

scripted=0
agents="claude grok muse"
while [ $# -gt 0 ]; do
    case "$1" in
        --scripted) scripted=1; shift ;;
        --agents) agents=$2; shift 2 ;;
        -h | --help) sed -n '2,13p' "$0"; exit 0 ;;
        *) echo "route-demo: unknown argument $1" >&2; exit 2 ;;
    esac
done

say() { printf '\n\033[1m== %s\033[0m\n' "$*"; }
die() { echo "route-demo: $*" >&2; exit 1; }
command -v av >/dev/null || die "av is not on PATH"
set -- $agents
[ $# -eq 3 ] || die "--agents takes three names: the code, the image and the cheap one"
code_agent=$1; image_agent=$2; cheap_agent=$3

run_agent() {
    local agent=$1 prompt=$2
    case "$agent" in
        claude) claude -p "$prompt" --permission-mode acceptEdits --allowedTools "Bash" ;;
        codex) codex exec --full-auto "$prompt" ;;
        gemini) gemini --yolo -p "$prompt" ;;
        grok) grok -p "$prompt" ;;
        muse) muse exec "$prompt" ;;
        *) die "no command for agent $agent" ;;
    esac
}
if [ "$scripted" = 0 ]; then
    for a in $agents; do command -v "$a" >/dev/null || die "$a is not installed (or use --scripted)"; done
fi

repo=$(mktemp -d /tmp/route-demo.XXXXXX)
name=route-$(date +%s)
mkdir -p "$repo/app"
printf 'def hello():\n    return "hello"\n' >"$repo/app/greeter.py"
printf '# Demo\n\nA small app.\n' >"$repo/README.md"
git -C "$repo" init -q -b main
git -C "$repo" add -A
git -C "$repo" -c user.name=demo -c user.email=demo@example.com commit -qm "demo"

# what av prints as `workspace: PATH` for a command run as $1
ws_of() { sed -n 's/^workspace: //p' | head -1; }
# the title of the task the agent holds: `av status` in its workspace
title_of() { (cd "$1" && AV_AGENT=$2 av status) | sed -n 's/^task: *//p' | head -1; }

say "1. Profiles: what each agent is good at, and what it costs (1 is the cheapest)"
av agent set "$code_agent" --best code,tests --can docs --cost 4
av agent set "$image_agent" --best image --can ui --cost 2
av agent set "$cheap_agent" --can code,docs --cost 1
av repo add "$name" "$repo" >/dev/null

say "2. Post three tasks to the backlog"
av post --repo "$name" --title "Make hello louder" --goal "hello() should shout" --needs code
av post --repo "$name" --title "Redraw the logo" --goal "Replace logo.png with a flat version" --needs image
av post --repo "$name" --title "Fix the README typos" --goal "Proofread README.md"
av backlog --repo "$name"

say "3. Each agent asks for its next task"
declare -A held
for a in "$code_agent" "$image_agent" "$cheap_agent"; do
    if [ "$scripted" = 1 ]; then
        out=$(AV_AGENT=$a av next --repo "$name")
    else
        run_agent "$a" "Run 'av next --repo $name'. It prints the task that fits you and its workspace. \
Do the task in that workspace, record why with 'av record', then stop. If it prints that no task is \
waiting, say so and stop." >/dev/null || true
        out=$(AV_AGENT=$a av backlog --repo "$name" --state taken | grep "taken by $a" || true)
    fi
    printf '%s\n' "$out" | head -3
    if [ "$scripted" = 1 ]; then
        w=$(printf '%s\n' "$out" | ws_of)
        [ -d "$w" ] || die "$a got no workspace: $out"
        held[$a]=$w
    fi
done
taken=$(av backlog --repo "$name" --state taken)
printf '%s\n' "$taken"
printf '%s\n' "$taken" | grep "Make hello louder" | grep -q "taken by $code_agent" ||
    die "the code task did not go to $code_agent"
printf '%s\n' "$taken" | grep "Redraw the logo" | grep -q "taken by $image_agent" ||
    die "the image task did not go to $image_agent"
printf '%s\n' "$taken" | grep "Fix the README typos" | grep -q "taken by $cheap_agent" ||
    die "the untagged task did not go to $cheap_agent (the cheapest)"

say "4. $code_agent finds the rest needs an image, and hands off with the tag"
if [ "$scripted" = 1 ]; then
    w=${held[$code_agent]}
    (cd "$w" && AV_AGENT=$code_agent av record --kind decision \
        --summary "hello() shouts; the logo in the README still needs redrawing" --anchor app/greeter.py)
    (cd "$w" && AV_AGENT=$code_agent av handoff --done "hello shouts" \
        --remaining "redraw the README logo" --needs image >/dev/null)
else
    run_agent "$code_agent" "In the workspace of your task: record why with 'av record', then hand off \
with: av handoff --done 'hello shouts' --remaining 'redraw the README logo' --needs image"
fi

say "5. Only an agent that has the tag is offered it"
nothing=$(AV_AGENT=$cheap_agent av next --repo "$name")
printf '%s\n' "$nothing" | grep -q "No task is waiting for you" ||
    die "$cheap_agent was offered the image task: $nothing"
echo "$cheap_agent: $nothing"
claimed=$(AV_AGENT=$image_agent av next --repo "$name")
printf '%s\n' "$claimed" | grep -q "generation: 2" || die "$image_agent did not claim the handoff: $claimed"
printf '%s\n' "$claimed" | grep -q "redraw the README logo" || die "the handoff bundle is missing: $claimed"
printf '%s\n' "$claimed" | sed -n '1,12p'

say "Done"
echo "$code_agent took code, $image_agent images, $cheap_agent the cheap untagged work; the handoff"
echo "tagged image went to $image_agent. Dashboard: ${AV_URL:-http://127.0.0.1:7420}/ui"
