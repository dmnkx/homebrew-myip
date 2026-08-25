#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "usage: $0 <version>" >&2
  echo "example: $0 0.1.0" >&2
  exit 1
fi

version="$1"
if [[ ! "${version}" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  echo "version must look like 0.1.0" >&2
  exit 1
fi

root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"

python3 - "${version}" <<'PY'
import pathlib
import re
import sys

version = sys.argv[1]
path = pathlib.Path("Cargo.toml")
text = path.read_text()
updated, n = re.subn(r'^version = ".*"', f'version = "{version}"', text, count=1, flags=re.M)
if n != 1:
    raise SystemExit("failed to update Cargo.toml version")
path.write_text(updated)
PY

cargo generate-lockfile

echo "Cargo.toml is now ${version}."
echo "Commit the version bump, then:"
echo "  git tag v${version}"
echo "  git push origin main --tags"
