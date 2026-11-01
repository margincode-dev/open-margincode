#!/usr/bin/env bash
# Two Claude Code agents on DeepSeek Flash share one backlog: they take the work that fits them,
# review each other's submissions and merge what they approve. The run checked for v0.0.2.
#
#   deepseek-demo.sh up                 # a separate daemon on :7441 (data in ~/scholion-demo)
#   deepseek-demo.sh setup [pilot]      # repo copy, two profiles, the backlog (pilot: one task)
#   deepseek-demo.sh agent NAME         # run one agent through `av watch` (coder or writer)
#   deepseek-demo.sh down
#
# DEMO and AV_URL override the data dir and the daemon; keep them the same for every command.
# Needs ~/.config/deepseek/key (the key alone, chmod 600). Spends DeepSeek credit: check
# https://api.deepseek.com/user/balance before and after. The daemon never starts the agents;
# `av watch` does, on this machine, only when `next` has work for that agent.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../.." && pwd)
AV=${AV:-$root/target/release/av}
DEMO=${DEMO:-$HOME/scholion-demo}
export AV_URL=${AV_URL:-http://127.0.0.1:7441}
export AV_HOME=$DEMO/home AV_WS_ROOT=$DEMO/ws
SRC=$DEMO/pantry
MODEL=${MODEL:-deepseek-flash}

prompt() {
  cat <<'EOF'
You are working through the av MCP server. Loop: call `start_task` with no title or goal and do what it returns.
- A review: `review_task`, run `python3 -m unittest` in its workspace, `validate` with that command and its exit code, `post_review` (approve, or request changes saying why), and if you approved, `merge`.
- A task: work ONLY in the workspace path it returns (git status there shows nothing; that is expected), read its pack, do the goal, run `python3 -m unittest` there, `validate`, then `submit` with at least one `decision` or `note` record saying why.
Pass `task_id` (the slug works) on every call. When `start_task` returns no task, call it once more with `wait` 50; if still nothing, stop and list what you did in one line per task.
EOF
}

case "${1:-}" in
up)
  (cd "$HOME" && "$AV" up)
  ;;
down)
  (cd "$HOME" && "$AV" down)
  ;;
setup)
  if [ ! -d "$SRC/.git" ]; then
    mkdir -p "$SRC"
    cp -r "$here/pantry/." "$SRC/"
    git -C "$SRC" init -q -b main
    git -C "$SRC" add -A
    git -C "$SRC" -c user.name=pantry -c user.email=pantry@example.com commit -qm "pantry 0.1.0"
  fi
  "$AV" repo add pantry "$SRC" 2>/dev/null || echo "(pantry already registered)"
  # two teams; 1 is the cheapest. Both run Claude Code on DeepSeek Flash.
  "$AV" agent set coder  --best code,tests --can docs     --cost 2 --max-open 1
  "$AV" agent set writer --best docs,ui    --can code,tests --cost 1 --max-open 1
  post() { "$AV" post --repo pantry "$@"; }
  if [ "${2:-}" = pilot ]; then
    post --title "Fix the README typos" --goal "Proofread README.md and fix the spelling mistakes. Change nothing else."
  else
    post --priority 2 --needs code,tests --title "Refuse invalid dates" \
      --goal "store.parse_date clamps the day to 28, so 2026-02-30 becomes 2026-02-28. Refuse any date that is not a real YYYY-MM-DD with a ValueError naming the input; the add command prints it and exits 2. Add unittest cases."
    post --priority 1 --needs code --title "Add an expiring command" \
      --goal "python -m pantry expiring --days N lists items that expire within N days of today (default 3), soonest first; already-expired items are marked EXPIRED. Tests for the date logic."
    post --needs docs --title "Write docs/USAGE.md" \
      --goal "A short user guide: install, every command with one example, the JSON file format. Keep it under 80 lines."
    post --needs ui --title "Style the HTML report" \
      --goal "report.render: add inline CSS so the table is readable (zebra rows, header row, works on a phone) and highlight rows that expire within 3 days. No JavaScript, no external files."
    post --title "Fix the README typos" --goal "Proofread README.md and fix the spelling mistakes. Change nothing else."
  fi
  "$AV" backlog --repo pantry
  ;;
agent)
  name=${2:?agent name: coder or writer}
  key=$HOME/.config/deepseek/key
  [ -r "$key" ] || { echo "no $key" >&2; exit 1; }
  dir=$DEMO/agents/$name
  mkdir -p "$dir/config" "$DEMO/ws"
  printf '{"mcpServers":{"av":{"type":"http","url":"%s/mcp","headers":{"X-Av-Agent":"%s"}}}}\n' \
    "$AV_URL" "$name" >"$dir/mcp.json"
  prompt >"$dir/prompt.txt"
  # Claude Code pointed at DeepSeek's Anthropic-compatible API, with its own config dir (no user
  # hooks, plugins or MCP servers). It runs in the workspaces' parent so edits stay inside it.
  cat >"$dir/run.sh" <<EOF
#!/usr/bin/env bash
set -euo pipefail
unset ANTHROPIC_API_KEY
export ANTHROPIC_BASE_URL=https://api.deepseek.com/anthropic
export ANTHROPIC_AUTH_TOKEN="\$(cat $key)"
export ANTHROPIC_DEFAULT_HAIKU_MODEL=$MODEL ANTHROPIC_SMALL_FAST_MODEL=$MODEL CLAUDE_CODE_SUBAGENT_MODEL=$MODEL
export CLAUDE_CONFIG_DIR=$dir/config PYTHONDONTWRITEBYTECODE=1
cd $DEMO/ws
exec claude -p "\$(cat $dir/prompt.txt)" --model $MODEL --max-turns 80 \\
  --strict-mcp-config --mcp-config $dir/mcp.json --setting-sources project \\
  --permission-mode acceptEdits --no-session-persistence \\
  --output-format stream-json --verbose \\
  --allowedTools mcp__av Read Edit Write Glob Grep "Bash(python3:*)" "Bash(cd:*)" "Bash(ls:*)" "Bash(cat:*)" \\
  >>$dir/session-\$(date +%H%M%S).jsonl 2>>$dir/stderr.log
EOF
  chmod +x "$dir/run.sh"
  exec "$AV" --agent "$name" watch --repo pantry --every 10 ${MAX_RUNS:+--max-runs $MAX_RUNS} -- "$dir/run.sh"
  ;;
*)
  sed -n 2,12p "$0"; exit 2
  ;;
esac
