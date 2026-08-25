#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 ]]; then
  echo "usage: $0 <version> [--checksums <file>] [--require-all]" >&2
  exit 1
fi

root="$(cd "$(dirname "$0")/.." && pwd)"
exec python3 "${root}/scripts/generate-formula.py" "$@"
