#!/usr/bin/env bash
# session.sh BIN_DIR URL : one simulated session (session-start, 30 x (pre-read, post-edit) 2 s apart, stop)
# in one HOME against URL; prints the time each hook took and the total time spent in hooks.
set -u
bin=$1; url=$2
d=/tmp/session-$$; mkdir -p "$d/home" "$d/ws"
echo '{"task":"t-1","session":"s-1"}' >"$d/ws/.av-task"; echo 'x = 1' >"$d/ws/app.py"
export PATH="$bin:$PATH" HOME="$d/home" AV_URL="$url"
cd "$d/ws"
total=0
run() {
  local s e ms
  s=$(date +%s%N); av hook "$1" <<<"$2" >/dev/null 2>&1; e=$(date +%s%N)
  ms=$(((e - s) / 1000000)); total=$((total + ms)); echo "$1 $ms"
}
run session-start "{\"session_id\":\"cc-1\",\"source\":\"startup\",\"cwd\":\"$d/ws\"}"
for i in $(seq 1 30); do
  run pre-read "{\"hook_event_name\":\"PreToolUse\",\"tool_input\":{\"file_path\":\"$d/ws/app.py\"},\"cwd\":\"$d/ws\"}"
  python3 -c "import json;p='$d/ws/.av-task';x=json.load(open(p));x.pop('last_checkpoint_ms',None);json.dump(x,open(p,'w'))"
  run post-edit "{\"hook_event_name\":\"PostToolUse\",\"tool_input\":{\"file_path\":\"$d/ws/app.py\"},\"cwd\":\"$d/ws\"}"
  sleep 2
done
run stop "{\"hook_event_name\":\"Stop\",\"session_id\":\"cc-1\",\"stop_hook_active\":false,\"cwd\":\"$d/ws\"}"
echo "total in hooks: $total ms"
rm -rf "$d"
