#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"

version="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)"
if [[ -z "${version}" ]]; then
  echo "could not read version from Cargo.toml" >&2
  exit 1
fi

if [[ -n "${GITHUB_REF_NAME:-}" && "${GITHUB_REF_NAME}" == v* ]]; then
  tag_version="${GITHUB_REF_NAME#v}"
  if [[ "${tag_version}" != "${version}" ]]; then
    echo "tag ${GITHUB_REF_NAME} does not match Cargo.toml version ${version}" >&2
    exit 1
  fi
fi

name="myip-${version}"

cargo test --locked

stage="$(mktemp -d)"
trap 'rm -rf "${stage}"' EXIT

mkdir -p "${stage}/${name}" dist
cp Cargo.toml Cargo.lock LICENSE "${stage}/${name}/"
cp -R src "${stage}/${name}/"

tar -czf "dist/${name}.tar.gz" -C "${stage}" "${name}"
echo "wrote dist/${name}.tar.gz"
