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
# The Stop hook may wait this long for work (see `av hook stop`); Claude Code must allow it more.
wait="${AV_HOOK_WAIT_SECS:-600}"
case "$wait" in
    '' | *[!0-9]*) echo "install.sh: AV_HOOK_WAIT_SECS must be a whole number of seconds, got '$wait'; nothing changed" >&2; exit 1 ;;
esac
stop_timeout=$((wait + 60))
mkdir -p "$dest"
install -m 0755 "$here/hooks/session-start.sh" "$here/hooks/pre-read.sh" "$here/hooks/post-edit.sh" "$here/hooks/stop.sh" "$dest/"
[ -f "$settings" ] || echo '{}' >"$settings"
jq empty "$settings" || { echo "install.sh: $settings is not valid JSON; nothing changed" >&2; exit 1; }
cp "$settings" "$settings.bak"
snippet=$(jq --arg dir "$dest" --argjson stop_timeout "$stop_timeout" \
    'walk(if type == "string" then sub("__AV_HOOKS__"; $dir) else . end)
     | .hooks.Stop[].hooks[].timeout = $stop_timeout' \
    "$here/settings.snippet.json")
jq --argjson add "$snippet" '
  def ours: (.hooks // []) | any(.command | test("/av-hooks/"));
  .hooks = (.hooks // {})
  | reduce ($add.hooks | to_entries[]) as $e (.;
      .hooks[$e.key] = (((.hooks[$e.key] // []) | map(select(ours | not))) + $e.value))
' "$settings.bak" >"$settings.tmp"
mv "$settings.tmp" "$settings"
echo "Scholion hooks installed: $dest (settings: $settings, previous: $settings.bak)"
