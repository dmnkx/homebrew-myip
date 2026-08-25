#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"

version="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)"
host="$(rustc -vV | awk '/^host:/{print $2}')"
bin="target/release/myip"

cargo build --release --locked

if [[ "$(uname -s)" == Darwin ]]; then
  strip -x "${bin}"
else
  strip "${bin}"
fi

mkdir -p dist
asset="myip-${version}-${host}.tar.gz"
tar -czf "dist/${asset}" -C target/release myip
echo "wrote dist/${asset}"
