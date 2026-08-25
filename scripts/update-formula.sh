#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 2 ]]; then
  echo "usage: $0 <version> <sha256>" >&2
  exit 1
fi

version="$1"
sha256="$2"
root="$(cd "$(dirname "$0")/.." && pwd)"
formula="${root}/Formula/myip.rb"
url="https://github.com/dmnkx/homebrew-myip/releases/download/v${version}/myip-${version}.tar.gz"
block="  url \"${url}\"
  sha256 \"${sha256}\"
"

python3 - "${formula}" "${block}" <<'PY'
import pathlib
import re
import sys

path = pathlib.Path(sys.argv[1])
block = sys.argv[2]
text = path.read_text()
if re.search(r'^  url "', text, re.M):
    text = re.sub(r'^  url ".*"\n  sha256 ".*"\n', block, text, count=1, flags=re.M)
else:
    text = text.replace('  license "MIT"\n', '  license "MIT"\n' + block, 1)
    if "livecheck do" not in text:
        livecheck = """
  livecheck do
    url :homepage
    strategy :github_latest
  end
"""
        text = text.replace('  depends_on "rust" => :build\n', livecheck + '  depends_on "rust" => :build\n', 1)
path.write_text(text)
PY
