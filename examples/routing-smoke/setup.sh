#!/usr/bin/env bash
# Smoke test for Plan 8 routing: one daemon, two agents (Claude Code, Grok Build), one backlog.
# Makes a git repo of the example project (pantry/ next to this script) at $SRC unless one is there,
# starts `av up` on :7420 if no daemon answers, registers the repo, sets the two profiles and posts
# seven tasks (once: if the repo already has backlog items, nothing more is posted).
# Spends nothing; the agents spend their own subscriptions later. See README.md here.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../.." && pwd)
AV=${AV:-$root/target/release/av}
REPO=${REPO:-pantry}
SRC=${SRC:-$HOME/projects/pantry}
export AV_URL=${AV_URL:-http://127.0.0.1:7420}

[ -x "$AV" ] || { echo "no $AV: cargo build --release -p scholion-cli -p scholion-daemon" >&2; exit 1; }

if [ ! -d "$SRC/.git" ]; then
    mkdir -p "$SRC"
    cp -r "$here/pantry/." "$SRC/"
    git -C "$SRC" init -q -b main
    git -C "$SRC" add -A
    git -C "$SRC" -c user.name=pantry -c user.email=pantry@example.com commit -qm "pantry 0.1.0"
    echo "created $SRC"
fi

# `av up` from $HOME: the daemon keeps the directory it started in, and git imports fail if it is deleted
curl -fs "$AV_URL/healthz" >/dev/null || (cd "$HOME" && "$AV" up)
# true when the repo already has backlog items (any state)
has_items() { "$AV" backlog --repo "$1" | grep -qv '^the backlog is empty$'; }
"$AV" repo add "$REPO" "$SRC" 2>/dev/null || echo "(repo $REPO already registered)"

# Profiles. 1 is the cheapest. An alias is the name the CLI gives itself in MCP `initialize`
# (the X-Av-Agent header wins when the config sets one).
"$AV" agent set claude --best code,tests --can docs,ui --cost 4 --alias claude-code
"$AV" agent set grok   --best image --can ui,docs,code --cost 2 --alias grok-shell-av

if has_items "$REPO"; then
    echo "(backlog of $REPO is not empty; not posting again)"
else
    post() { "$AV" post --repo "$REPO" "$@"; }
    post --priority 2 --needs code,tests --title "Refuse invalid dates" \
      --goal "store.parse_date clamps the day to 28, so 2026-02-30 becomes 2026-02-28. Refuse any date that is not a real YYYY-MM-DD with a ValueError naming the input; the add command prints it and exits 2. Add unittest cases."
    post --priority 1 --needs code --title "Category totals, then the chart" \
      --goal "Add pantry/stats.py totals_by_category(items) -> dict of category to total quantity, with tests. Then do NOT draw the chart yourself: handoff with needs [image] and say in remaining: draw assets/chart.svg, a bar chart of totals_by_category over sample.json."
    post --needs code --title "Add an expiring command" \
      --goal "python -m pantry expiring --days N lists items that expire within N days of today (default 3), soonest first; already-expired items are marked EXPIRED. Tests for the date logic."
    post --needs docs --prefer claude --title "Write docs/USAGE.md" \
      --goal "A short user guide: install, every command with one example, the JSON file format. Keep it under 80 lines."
    post --priority 1 --needs image --title "Draw the app icon" \
      --goal "Create assets/icon.svg: a 256x256 flat icon of a pantry jar, at most 3 colours, no text, no external references. Show it at the top of README.md."
    post --needs ui --title "Style the HTML report" \
      --goal "report.render: add inline CSS so the table is readable (zebra rows, header row, works on a phone) and highlight rows that expire within 3 days. No JavaScript, no external files."
    post --title "Fix the README typos" --goal "Proofread README.md and fix the spelling mistakes. Change nothing else."
fi

# Second project: the Sunken Crown game (Godot/GDScript, Python tools), if it is on this machine.
# av imports it; the original checkout is never changed. GDScript gets records and packs but no
# symbol index (av-index parses Rust, TypeScript and Python only).
GAME=${GAME:-sunken-crown}
GAME_SRC=${GAME_SRC:-$HOME/projects/sunken-crown}
if [ -d "$GAME_SRC/.git" ]; then
    "$AV" repo add "$GAME" "$GAME_SRC" 2>/dev/null || echo "(repo $GAME already registered)"
    if has_items "$GAME"; then
        echo "(backlog of $GAME is not empty; not posting again)"
    else
        rules="Read HANDOFF.md first, but do not edit it and do not push: av reviews and merges."
        gpost() { "$AV" post --repo "$GAME" "$@"; }
        gpost --priority 1 --needs code,tests --title "Validator: every art path must exist" \
          --goal "tools/validate_content.py: report an error for each content/ reference to a portrait, battle picture or icon file that does not exist under assets/ or art/. Add a test under tests/ for the check (Python, no Godot). python3 tools/validate_content.py must still print Content OK. $rules"
        gpost --needs code --title "Remove the duplicate _is_enemy" \
          --goal "tools/validate_content.py defines _is_enemy twice (the second has noqa F811). Keep one correct version, make sure every caller still behaves the same, and say why in a decision record. $rules"
        gpost --priority 1 --needs image --title "Review the shard icons at 48 px" \
          --goal "Look at the six PNGs in assets/icons/shards/ shrunk to 48x48. HANDOFF says they differ mainly by colour. Write docs/art/review-shard-icons.md: for each icon, whether it is readable at 48 px and for colour-blind players, and one concrete shape change. Do not change the PNGs. $rules"
        gpost --needs image --title "Check the weak spots of art batches 6-10" \
          --goal "HANDOFF.md lists weak spots per art batch (e.g. kyoto_nishiki_mortar reads as a pot, siab_mortar as a hand cannon). Open each named picture and write docs/art/weak-spots-check.md: for each, confirmed or not, and how bad on a 1-3 scale. Do not change the art. $rules"
        gpost --title "Explain the tools folder" \
          --goal "tools/ has no README. Write tools/README.md: one line per script saying what it does and how to run it, read from each script. Change no script. $rules"
    fi
    "$AV" backlog --repo "$GAME"
fi

"$AV" agent list
"$AV" backlog --repo "$REPO"
