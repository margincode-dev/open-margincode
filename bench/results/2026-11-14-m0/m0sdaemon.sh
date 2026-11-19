#!/bin/bash
# Amendment 2: start one build's daemon for the Sonnet run: m0sdaemon.sh A|B (root in m2b-harness).
# Build A on 127.0.0.1:7461 with database m0sa, build B on 7462 with m0sb; store and workspace
# root under /m2b/m0s. The database password is read from /m2b/main-21f30dc/pgpw (600) and never
# printed.
set -eu
R=/m2b/m0s
b=$1
case "$b" in A) port=7461 db=m0sa lc=a ;; B) port=7462 db=m0sb lc=b ;; *) echo "usage: m0sdaemon.sh A|B"; exit 2 ;; esac
umask 022
export AV_LISTEN=127.0.0.1:$port
export AV_DATABASE_URL="postgres://av:$(cat /m2b/main-21f30dc/pgpw)@lp-pg:5432/$db"
export AV_STORE_ROOT=$R/$b/store AV_WS_ROOT=$R/bench-work/ws-$lc AV_WS_HOST_ROOT=$R/bench-work/ws-$lc
export AV_QDRANT_URL=http://lp-qdrant:6334 AV_EMBED_URL=http://lp-tei:80
export HOME=$R
unset AV_TOKEN
exec $R/$b/bin/scholion-daemon
