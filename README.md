**Language:** English | [한국어](README.ko.md)

# myip

> Print your local and public IPv4 addresses from the command line.

`myip` is a small CLI. Run it with no arguments and it prints both addresses.

```sh
myip
```

```text
local: 192.168.x.x
public: x.x.x.x
```

## What is myip?

`myip` reports the IPv4 address of the default network route and the public IPv4 address seen by the internet.

It is distributed as a Homebrew tap (`dmnkx/myip`). Installs download a prebuilt binary from GitHub Releases.

## Key Features

- **Local and public IPv4** — prints both in one invocation
- **Fallback lookups** — public IP is fetched from several endpoints if one fails
- **Homebrew tap** — `brew tap dmnkx/myip && brew install myip`
- **Tagged releases** — CI builds macOS (Apple Silicon and Intel) and Linux (arm and x86_64) binaries after tests pass

## Requirements

On macOS, **Xcode Command Line Tools** are required before Homebrew and this tap will install cleanly:

```sh
xcode-select --install
```

Confirm with `xcode-select -p`.

## Install

**Homebrew**

```sh
brew tap dmnkx/myip
brew install myip
```

**From source** (Rust toolchain required):

```sh
cargo install --path .
```

Prebuilt archives are also on [GitHub Releases](https://github.com/dmnkx/homebrew-myip/releases).

## Quick Start

```sh
myip
myip --version
myip --help
```

## Releasing

Push a version tag. Release CI sets `Cargo.toml` / `Cargo.lock` to that tag (`v0.1.4` → `0.1.4`), runs tests, publishes binaries, and updates the Homebrew formula.

Intel macOS binaries are cross-compiled as `x86_64-apple-darwin` on Apple Silicon runners.

```sh
git tag v0.1.4
git push origin v0.1.4
```

Releases are not published if `cargo test` fails.

## License

[MIT](LICENSE)
