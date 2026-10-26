#!/usr/bin/env bash
# Installs the Scholion hooks for Claude Code: copies them to ~/.claude/av-hooks and merges
# settings.snippet.json into ~/.claude/settings.json (the previous file is kept as settings.json.bak).
# Running it again replaces the Scholion hooks instead of adding them twice.
set -euo pipefail
if ! command -v jq >/dev/null 2>&1; then
    echo "install.sh: jq is required (https://jqlang.github.io/jq/)" >&2
    exit 1
fi
here=$(cd "$(dirname "$0")" && pwd)
claude="$HOME/.claude"
dest="$claude/av-hooks"
settings="$claude/settings.json"
mkdir -p "$dest"
install -m 0755 "$here/hooks/session-start.sh" "$here/hooks/post-edit.sh" "$dest/"
[ -f "$settings" ] || echo '{}' >"$settings"
jq empty "$settings" || { echo "install.sh: $settings is not valid JSON; nothing changed" >&2; exit 1; }
cp "$settings" "$settings.bak"
snippet=$(jq --arg dir "$dest" 'walk(if type == "string" then sub("__AV_HOOKS__"; $dir) else . end)' \
    "$here/settings.snippet.json")
jq --argjson add "$snippet" '
  def ours: (.hooks // []) | any(.command | test("/av-hooks/"));
  .hooks = (.hooks // {})
  | reduce ($add.hooks | to_entries[]) as $e (.;
      .hooks[$e.key] = (((.hooks[$e.key] // []) | map(select(ours | not))) + $e.value))
' "$settings.bak" >"$settings.tmp"
mv "$settings.tmp" "$settings"
echo "Scholion hooks installed: $dest (settings: $settings, previous: $settings.bak)"
