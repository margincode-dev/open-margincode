#!/usr/bin/env bash
# healthy.sh BIN_DIR N : times each hook N times against a real daemon on 127.0.0.1:7610 (container only)
set -u
bin=$1; n=${2:-20}
root=/tmp/healthy-$$; mkdir -p "$root/home" "$root/store" "$root/ws"
export PATH="$bin:$PATH" HOME="$root/home" AV_HOME="$root/home/.margincode" AV_URL=http://127.0.0.1:7610
psql_url=${PG_URL:-postgres://postgres:pg@127.0.0.1:5432}
# the database `healthy` must exist on that server (createdb healthy)
AV_DATABASE_URL="$psql_url/healthy" AV_LISTEN=127.0.0.1:7610 AV_STORE_ROOT="$root/store" AV_WS_ROOT="$root/ws" \
  nohup "$bin/margincode-daemon" >"$root/daemon.log" 2>&1 &
dpid=$!
for i in $(seq 1 100); do curl -sf -m1 $AV_URL/healthz >/dev/null && break; sleep 0.3; done
src=$root/src; mkdir -p "$src"; cp -r /src/crates "$src/"; (cd "$src" && git init -q -b main && git add -A && git -c user.name=t -c user.email=t@t commit -qm init)
av repo add demo "$src" >/dev/null
av task start --repo demo --title "Other" --goal "other" >/dev/null
other=$(ls -d "$root"/ws/demo/other)
(cd "$other" && av record --kind decision --summary "hook timing note" --anchor crates/margincode-cli/src/hook.rs >/dev/null)
av task start --repo demo --title "Timed" --goal "time the hooks" >/dev/null
ws=$(ls -d "$root"/ws/demo/timed)
file="$ws/crates/margincode-cli/src/hook.rs"
t() { local s e; s=$(date +%s%N); "$@" >/dev/null 2>&1; e=$(date +%s%N); echo $(((e - s) / 1000000)); }
for h in session-start pre-read post-edit stop; do
  times=()
  for i in $(seq 1 "$n"); do
    case $h in
      session-start) in="{\"session_id\":\"h$i\",\"source\":\"startup\",\"cwd\":\"$ws\"}" ;;
      pre-read) in="{\"hook_event_name\":\"PreToolUse\",\"tool_input\":{\"file_path\":\"$file\"},\"cwd\":\"$ws\"}" ;;
      post-edit) echo "// $i" >>"$file"; python3 -c "import json;p='$ws/.av-task';d=json.load(open(p));d.pop('last_checkpoint_ms',None);json.dump(d,open(p,'w'))"
        in="{\"hook_event_name\":\"PostToolUse\",\"tool_input\":{\"file_path\":\"$file\"},\"cwd\":\"$ws\"}" ;;
      stop) in="{\"hook_event_name\":\"Stop\",\"session_id\":\"h$i\",\"stop_hook_active\":false,\"cwd\":\"$ws\"}" ;;
    esac
    times+=("$(cd "$ws" && AV_HOOK_WAIT_SECS=0 t av hook "$h" <<<"$in")")
  done
  printf '%s\n' "${times[@]}" | sort -n | awk -v h="$h" '{a[NR]=$1} END {printf "%s: n %d median %d ms p95 %d ms max %d ms\n", h, NR, a[int((NR+1)/2)], a[int(NR*0.95+0.5)], a[NR]}'
done
kill $dpid
