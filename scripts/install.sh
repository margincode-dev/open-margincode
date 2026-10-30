#!/bin/sh
# Installs the prebuilt `scholion` CLI, its alias `av` and `scholion-daemon` from a GitHub release:
#
#   curl -fsSL https://raw.githubusercontent.com/vinhqbui/scholion/main/scripts/install.sh | sh
#
# SCHOLION_VERSION   a release tag (`release/0.0.1`, `v0.1.0`); default: the newest release,
#                    pre-releases included
# SCHOLION_INSTALL_DIR  where the binaries go; default: ~/.local/bin
#
# `av init` in a git repository then starts the daemon with its own Postgres (lite mode).
set -eu

repo=vinhqbui/scholion
dir=${SCHOLION_INSTALL_DIR:-$HOME/.local/bin}

say() { printf 'scholion install: %s\n' "$*" >&2; }
die() { say "$*"; exit 1; }

command -v curl >/dev/null 2>&1 || die "curl is required"
command -v tar >/dev/null 2>&1 || die "tar is required"

# The targets .github/workflows/release.yml builds.
case "$(uname -s)-$(uname -m)" in
    Linux-x86_64 | Linux-amd64) target=x86_64-unknown-linux-gnu ;;
    Linux-aarch64 | Linux-arm64) target=aarch64-unknown-linux-gnu ;;
    Darwin-arm64) target=aarch64-apple-darwin ;;
    *) die "no prebuilt binary for $(uname -s) $(uname -m); build from source: cargo build --release -p scholion-cli -p scholion-daemon" ;;
esac

# `/releases/latest` skips pre-releases, so take the first (newest) entry of the list instead.
tag=${SCHOLION_VERSION:-}
if [ -z "$tag" ]; then
    tag=$(curl -fsSL "https://api.github.com/repos/$repo/releases?per_page=1" |
        sed -n 's/.*"tag_name": *"\([^"]*\)".*/\1/p' | head -n 1)
    [ -n "$tag" ] || die "cannot find a release of $repo"
fi

# As release.yml names archives: release/0.1.0 -> v0.1.0
ver=${tag#release/}
ver=v${ver#v}
name="scholion-$ver-$target"
base="https://github.com/$repo/releases/download/$tag"

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT INT TERM

say "downloading $name ($tag)"
curl -fsSL -o "$tmp/$name.tar.gz" "$base/$name.tar.gz" || die "download failed: $base/$name.tar.gz"
curl -fsSL -o "$tmp/$name.tar.gz.sha256" "$base/$name.tar.gz.sha256" || die "checksum download failed"

expected=$(cut -d ' ' -f 1 <"$tmp/$name.tar.gz.sha256")
if command -v sha256sum >/dev/null 2>&1; then
    actual=$(sha256sum "$tmp/$name.tar.gz" | cut -d ' ' -f 1)
else
    actual=$(shasum -a 256 "$tmp/$name.tar.gz" | cut -d ' ' -f 1)
fi
[ "$expected" = "$actual" ] || die "checksum mismatch for $name.tar.gz"

tar -xzf "$tmp/$name.tar.gz" -C "$tmp"
mkdir -p "$dir"
for bin in scholion av scholion-daemon; do
    cp "$tmp/$name/$bin" "$dir/$bin.tmp"
    chmod 0755 "$dir/$bin.tmp"
    mv "$dir/$bin.tmp" "$dir/$bin"
done
say "installed scholion, av and scholion-daemon into $dir"

case ":$PATH:" in
    *":$dir:"*) ;;
    *) say "$dir is not on PATH; add it, e.g.: export PATH=\"$dir:\$PATH\"" ;;
esac
say "next: cd into a git repository and run: av init"
