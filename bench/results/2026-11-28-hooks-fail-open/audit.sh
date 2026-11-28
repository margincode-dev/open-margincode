#!/usr/bin/env bash
# audit.sh BIN_DIR OUT_FILE [CONDS] : times every hook under every failure condition, in parallel.
# Runs inside the build container only. Never touches 127.0.0.1:7420.
set -u
bin=$1; outf=$2; conds=${3:-"refused hang drop 500 401 slow"}
here=/audit/tools
root=/tmp/aud-$$; mkdir -p "$root"
export PATH="$bin:$PATH"
url_of() {
  case $1 in
    refused) echo http://127.0.0.1:7499 ;; hang) echo http://127.0.0.1:7501 ;;
    drop) echo http://192.0.2.1:7420 ;; 500) echo http://127.0.0.1:7502 ;;
    401) echo http://127.0.0.1:7503 ;; slow) echo http://127.0.0.1:7504 ;;
  esac
}
input_of() { # hook ws
  case $1 in
    session-start) echo "{\"session_id\":\"cc-1\",\"source\":\"startup\",\"cwd\":\"$2\",\"hook_event_name\":\"SessionStart\"}" ;;
    pre-read) echo "{\"hook_event_name\":\"PreToolUse\",\"tool_name\":\"Read\",\"tool_input\":{\"file_path\":\"$2/app.py\"},\"cwd\":\"$2\"}" ;;
    post-edit) echo "{\"hook_event_name\":\"PostToolUse\",\"tool_name\":\"Edit\",\"tool_input\":{\"file_path\":\"$2/app.py\"},\"cwd\":\"$2\"}" ;;
    stop) echo "{\"hook_event_name\":\"Stop\",\"session_id\":\"cc-1\",\"stop_hook_active\":false,\"cwd\":\"$2\"}" ;;
  esac
}
oc_script() { # hook ws
  case $1 in
    session-start) step='{"hook":"prompt","event":{"sessionID":"o1"}}' ;;
    pre-read) step='{"hook":"tool.execute.after","event":{"sessionID":"o1","tool":"read","status":"completed","input":{"path":"app.py"},"result":{"output":"x"}}}' ;;
    post-edit) step='{"hook":"tool.execute.after","event":{"sessionID":"o1","tool":"edit","status":"completed","input":{"path":"app.py"},"result":{"output":"x"}}}' ;;
    stop) step='{"hook":"experimental.idle","event":{"sessionID":"o1","background":[]}}' ;;
  esac
  echo "{\"sessions\":{\"o1\":{\"directory\":\"$2\"}},\"steps\":[$step]}"
}
one() { # cond wrapper hook
  local c=$1 w=$2 h=$3 d="$root/$1-$2-$3"
  mkdir -p "$d/home" "$d/ws"
  echo '{"task":"t-1","session":"s-1"}' >"$d/ws/.av-task"
  echo 'x = 1' >"$d/ws/app.py"
  local env=(HOME="$d/home" AV_URL="$(url_of "$c")" XDG_CONFIG_HOME="$d/home/.config")
  [ "$c" = slow ] && [ "$h" = stop ] && env+=(AV_HOOK_WAIT_SECS=0)
  local t0 t1 code
  t0=$(date +%s.%N)
  case $w in
    cli) (cd "$d/ws" && env "${env[@]}" timeout 1000 av hook "$h" <<<"$(input_of "$h" "$d/ws")" >"$d/out" 2>"$d/err"); code=$? ;;
    claude) (cd "$d/ws" && env "${env[@]}" timeout 1000 sh -c "command -v av >/dev/null 2>&1 && av hook $h; exit 0" <<<"$(input_of "$h" "$d/ws")" >"$d/out" 2>"$d/err"); code=$? ;;
    opencode) oc_script "$h" "$d/ws" >"$d/script.json"
      (cd "$d/ws" && env "${env[@]}" timeout 1000 node "$here/harness.mjs" "$here/av.js" "$d/script.json" >"$d/out" 2>"$d/err"); code=$? ;;
  esac
  t1=$(date +%s.%N)
  local o e
  o=$(head -c 160 "$d/out" | tr '\n' ' ')
  e=$(head -n1 "$d/err" | cut -c1-140)
  printf '%s\t%s\t%s\t%.1f\t%s\t%s\t%s\n' "$c" "$w" "$h" "$(echo "$t1 - $t0" | bc -l 2>/dev/null || python3 -c "print($t1-$t0)")" "$code" "$o" "$e" >>"$outf"
}
: >"$outf"
for c in $conds; do
  for w in cli claude opencode; do
    for h in session-start pre-read post-edit stop; do one "$c" "$w" "$h" & done
  done
done
wait
sort -o "$outf" "$outf"
rm -rf "$root"
