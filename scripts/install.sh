#!/bin/sh
# Installs the prebuilt `margincode` CLI, its alias `av` and `margincode-daemon` from a GitHub release:
#
#   curl -fsSL https://raw.githubusercontent.com/vinhqbui/scholion/main/scripts/install.sh | sh
#
# MARGINCODE_VERSION   a release tag (`release/0.0.1`, `v0.1.0`); default: the newest release,
#                    pre-releases included
# MARGINCODE_INSTALL_DIR  where the binaries go; default: ~/.local/bin
#
# `av init` in a git repository then starts the daemon with its own Postgres (lite mode).
set -eu

repo=vinhqbui/scholion
dir=${MARGINCODE_INSTALL_DIR:-${SCHOLION_INSTALL_DIR:-$HOME/.local/bin}}

say() { printf 'margincode install: %s\n' "$*" >&2; }
die() { say "$*"; exit 1; }

command -v curl >/dev/null 2>&1 || die "curl is required"
command -v tar >/dev/null 2>&1 || die "tar is required"

# The targets .github/workflows/release.yml builds.
case "$(uname -s)-$(uname -m)" in
    Linux-x86_64 | Linux-amd64) target=x86_64-unknown-linux-gnu ;;
    Linux-aarch64 | Linux-arm64) target=aarch64-unknown-linux-gnu ;;
    Darwin-arm64) target=aarch64-apple-darwin ;;
    *) die "no prebuilt binary for $(uname -s) $(uname -m); build from source: cargo build --release -p margincode-cli -p margincode-daemon" ;;
esac

# `/releases/latest` skips pre-releases, so take the first (newest) entry of the list instead.
tag=${MARGINCODE_VERSION:-${SCHOLION_VERSION:-}}
if [ -z "$tag" ]; then
    tag=$(curl -fsSL "https://api.github.com/repos/$repo/releases?per_page=1" |
        sed -n 's/.*"tag_name": *"\([^"]*\)".*/\1/p' | head -n 1)
    [ -n "$tag" ] || die "cannot find a release of $repo"
fi

# As release.yml names archives: release/0.1.0 -> v0.1.0
ver=${tag#release/}
ver=v${ver#v}
base="https://github.com/$repo/releases/download/$tag"

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT INT TERM

# Releases up to 0.0.3 were made under the project's earlier name: their archive and binaries
# carry it. Such a release still installs, with `margincode` as the name of its CLI.
name="margincode-$ver-$target"
old=
say "downloading $name ($tag)"
if ! curl -fsSL -o "$tmp/$name.tar.gz" "$base/$name.tar.gz"; then
    name="scholion-$ver-$target"
    old=scholion
    curl -fsSL -o "$tmp/$name.tar.gz" "$base/$name.tar.gz" ||
        die "download failed: $base/margincode-$ver-$target.tar.gz"
fi
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
for bin in margincode av margincode-daemon; do
    src=$bin
    [ -z "$old" ] || [ "$bin" = av ] || src=$old${bin#margincode}
    cp "$tmp/$name/$src" "$dir/$bin.tmp"
    chmod 0755 "$dir/$bin.tmp"
    mv "$dir/$bin.tmp" "$dir/$bin"
done
say "installed margincode, av and margincode-daemon into $dir"

case ":$PATH:" in
    *":$dir:"*) ;;
    *) say "$dir is not on PATH; add it, e.g.: export PATH=\"$dir:\$PATH\"" ;;
esac
say "next: cd into a git repository and run: av init"
say "or, in Claude Code: claude plugin marketplace add vinhqbui/scholion && claude plugin install margincode@margincode, then ask Claude to set up Margincode"
say "if Margincode helps, a star helps others find it: https://github.com/vinhqbui/scholion"
