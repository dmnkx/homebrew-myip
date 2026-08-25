#!/usr/bin/env python3
"""Write Formula/myip.rb for prebuilt binaries (no Rust / Xcode on install)."""

from __future__ import annotations

import argparse
import pathlib
import re

TRIPLES = (
    "aarch64-apple-darwin",
    "x86_64-apple-darwin",
    "aarch64-unknown-linux-gnu",
    "x86_64-unknown-linux-gnu",
)

PLACEHOLDER = "0" * 64
REPO = "https://github.com/dmnkx/homebrew-myip"


def parse_checksums(path: pathlib.Path | None) -> dict[str, str]:
    hashes = {triple: PLACEHOLDER for triple in TRIPLES}
    if path is None:
        return hashes
    known = "|".join(re.escape(triple) for triple in TRIPLES)
    for line in path.read_text().splitlines():
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        parts = line.split()
        if len(parts) < 2:
            raise SystemExit(f"invalid checksum line: {line}")
        digest, name = parts[0], parts[-1]
        match = re.search(rf"({known})", name)
        if not match:
            raise SystemExit(f"could not parse target from: {name}")
        if not re.fullmatch(r"[0-9a-f]{64}", digest):
            raise SystemExit(f"invalid sha256: {digest}")
        hashes[match.group(1)] = digest
    return hashes


def render(version: str, hashes: dict[str, str]) -> str:
    base = f"{REPO}/releases/download/v{version}"
    return f'''class Myip < Formula
  desc "Print local and public IP addresses"
  homepage "{REPO}"
  version "{version}"
  license "MIT"

  livecheck do
    url :homepage
    strategy :github_latest
  end

  on_macos do
    on_arm do
      url "{base}/myip-{version}-aarch64-apple-darwin.tar.gz"
      sha256 "{hashes["aarch64-apple-darwin"]}"
    end
    on_intel do
      url "{base}/myip-{version}-x86_64-apple-darwin.tar.gz"
      sha256 "{hashes["x86_64-apple-darwin"]}"
    end
  end

  on_linux do
    on_arm do
      url "{base}/myip-{version}-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "{hashes["aarch64-unknown-linux-gnu"]}"
    end
    on_intel do
      url "{base}/myip-{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "{hashes["x86_64-unknown-linux-gnu"]}"
    end
  end

  def install
    bin.install "myip"
  end

  test do
    assert_match(/myip \\d+\\.\\d+\\.\\d+/, shell_output("#{{bin}}/myip --version"))
  end
end
'''


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("version")
    parser.add_argument("--checksums", type=pathlib.Path)
    parser.add_argument(
        "--output",
        type=pathlib.Path,
        default=pathlib.Path(__file__).resolve().parent.parent / "Formula" / "myip.rb",
    )
    parser.add_argument("--require-all", action="store_true")
    args = parser.parse_args()

    hashes = parse_checksums(args.checksums)
    if args.require_all:
        missing = [triple for triple, digest in hashes.items() if digest == PLACEHOLDER]
        if missing:
            raise SystemExit("missing checksums for: " + ", ".join(missing))

    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(render(args.version, hashes))
    print(f"wrote {args.output}")


if __name__ == "__main__":
    main()
