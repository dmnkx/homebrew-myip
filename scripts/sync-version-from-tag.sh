#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"

tag="${1:-${GITHUB_REF_NAME:-}}"
if [[ -z "${tag}" ]]; then
  echo "usage: $0 <vX.Y.Z>  (or set GITHUB_REF_NAME)" >&2
  exit 1
fi

if [[ "${tag}" != v* ]]; then
  echo "tag must look like v0.1.3, got: ${tag}" >&2
  exit 1
fi

version="${tag#v}"
if [[ ! "${version}" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  echo "version must look like 0.1.3, got: ${version}" >&2
  exit 1
fi

python3 - "${version}" <<'PY'
import pathlib
import re
import sys

version = sys.argv[1]

toml = pathlib.Path("Cargo.toml")
text = toml.read_text()
updated, n = re.subn(r'^version = ".*"', f'version = "{version}"', text, count=1, flags=re.M)
if n != 1:
    raise SystemExit("failed to update Cargo.toml version")
toml.write_text(updated)

lock = pathlib.Path("Cargo.lock")
lock_text = lock.read_text()
lock_updated, n = re.subn(
    r'(name = "myip"\nversion = ")[^"]+(")',
    rf"\g<1>{version}\2",
    lock_text,
    count=1,
)
if n != 1:
    raise SystemExit("failed to update Cargo.lock myip version")
lock.write_text(lock_updated)
PY

echo "synced crate version to ${version} from ${tag}"
