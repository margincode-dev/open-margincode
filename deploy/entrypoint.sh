#!/bin/sh
# The image's entrypoint: the daemon, with a passwd entry for the user it runs as.
# Compose runs the daemon as the host's UID, which has no entry in the image, and OpenSSH (git's
# transport for an SSH backup remote) refuses to run for a UID without one ("No user exists for
# uid ..."). For such a UID, nss_wrapper gives the daemon and everything it starts (git, ssh) a
# passwd and group file of their own with that entry; /etc/passwd stays as it is.
set -eu
uid=$(id -u)
gid=$(id -g)
if ! getent passwd "$uid" >/dev/null 2>&1; then
    dir=$(mktemp -d)
    getent passwd >"$dir/passwd"
    echo "margincode:x:$uid:$gid:margincode:${HOME:-/tmp}:/bin/sh" >>"$dir/passwd"
    getent group >"$dir/group"
    getent group "$gid" >/dev/null 2>&1 || echo "margincode:x:$gid:" >>"$dir/group"
    export LD_PRELOAD=libnss_wrapper.so NSS_WRAPPER_PASSWD="$dir/passwd" NSS_WRAPPER_GROUP="$dir/group"
fi
exec margincode-daemon "$@"
