#!/bin/bash
# Start the main 21f30dc daemon for one run: daemon.sh PORT DB ROOT (root in m2b-harness).
# The database password is read from /m2b/main-21f30dc/pgpw (600) and never printed.
set -eu
port=$1 db=$2 root=$3
export AV_LISTEN=127.0.0.1:$port
export AV_DATABASE_URL="postgres://av:$(cat /m2b/main-21f30dc/pgpw)@lp-pg:5432/$db"
export AV_STORE_ROOT=$root/store AV_WS_ROOT=$root/bench-work/ws AV_WS_HOST_ROOT=$root/bench-work/ws
export AV_QDRANT_URL=http://lp-qdrant:6334 AV_EMBED_URL=http://lp-tei:80 AV_PLAIN_RECORDS=on
export HOME=/m2b
unset AV_TOKEN
exec /m2b/main-21f30dc/bin/margincode-daemon
