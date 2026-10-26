#!/usr/bin/env bash
# End-to-end smoke test of the Docker stack: import a repo, start a task, edit, checkpoint, record a
# dead end, and check the context pack and file ownership.
#   scripts/smoke.sh          run the test (SMOKE_REPO=<dir name under $AV_IMPORT_ROOT>, default bench-repo)
#   scripts/smoke.sh --down   stop the stack and delete its volumes
set -euo pipefail
cd "$(dirname "$0")/.."

export UID
GID="$(id -g)"
export GID
COMPOSE=(docker compose -f deploy/docker-compose.yml)

if [[ "${1:-}" == "--down" ]]; then
  "${COMPOSE[@]}" down -v
  exit 0
fi

fail() {
  echo "FAIL: $*" >&2
  exit 1
}

AV=target/release/av
[[ -x $AV ]] || cargo build --release -p scholion-cli

# SMOKE_BUILD=0 reuses a prebuilt scholion-daemon:dev image (e.g. when the build context is unreachable).
if [[ "${SMOKE_BUILD:-1}" == 1 ]]; then
  "${COMPOSE[@]}" up -d --build
else
  "${COMPOSE[@]}" up -d
fi

export AV_URL="http://localhost:${AV_DAEMON_PORT:-7420}"
for _ in $(seq 1 90); do
  curl -sf "$AV_URL/healthz" >/dev/null 2>&1 && break
  sleep 1
done
curl -sf "$AV_URL/healthz" >/dev/null || fail "daemon did not become healthy at $AV_URL"

REPO="${SMOKE_REPO:-bench-repo}"
"$AV" repo add "$REPO" "/import/$REPO"

out="$("$AV" task start --repo "$REPO" --title "smoke test" --goal "append a function")"
ws="$(sed -n 's/^workspace: //p' <<<"$out" | head -n1)"
[[ -n $ws && -d $ws ]] || fail "workspace '$ws' is not visible on the host"

target="$(find "$ws" -name '*.py' -not -path '*/.*' -print -quit)"
[[ -n $target ]] || fail "no .py file in $ws"
printf '\ndef smoke_added_function():\n    return 1\n' >>"$target"

cd "$ws"
"$OLDPWD/$AV" checkpoint
"$OLDPWD/$AV" record --kind attempt --summary "smoke attempt: tried nothing"
# --fresh: the session already received the task header at `task start`
ctx="$("$OLDPWD/$AV" context --budget 4000 --fresh)"

grep -q smoke_added_function <<<"$ctx" || fail "context pack does not mention smoke_added_function"
grep -q "smoke attempt" <<<"$ctx" || fail "context pack does not contain the recorded attempt"
[[ "$(stat -c %u "$target")" == "$(id -u)" ]] || fail "$target is not owned by the current user"

echo "PASS"
