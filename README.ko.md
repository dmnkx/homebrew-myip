**언어:** [English](README.md) | 한국어

# myip

> 명령줄에서 로컬 IPv4와 공인 IPv4를 출력합니다.

`myip`는 작은 CLI입니다. 인자 없이 실행하면 두 주소를 모두 출력합니다.

```sh
myip
```

```text
local: 192.168.x.x
public: x.x.x.x
```

## myip이란?

`myip`는 기본 네트워크 경로의 IPv4와, 인터넷에서 보이는 공인 IPv4를 알려 줍니다.

Homebrew 탭(`dmnkx/myip`)으로 배포합니다. 설치 시 GitHub Releases의 미리 빌드된 바이너리를 받습니다.

## 주요 기능

- **로컬·공인 IPv4** — 한 번 실행으로 둘 다 출력
- **조회 재시도** — 공인 IP는 엔드포인트가 실패하면 다른 주소로 조회
- **Homebrew 탭** — `brew tap dmnkx/myip && brew install myip`
- **태그 릴리스** — 테스트 통과 후 macOS(Apple Silicon·Intel)와 Linux(arm·x86_64) 바이너리 빌드

## 요구 사항

macOS에서는 Homebrew와 이 탭을 설치하기 전에 **Xcode Command Line Tools**가 필요합니다.

```sh
xcode-select --install
```

설치 여부는 `xcode-select -p`로 확인할 수 있습니다.

## 설치

**Homebrew**

```sh
brew tap dmnkx/myip
brew install myip
```

**소스에서** (Rust 툴체인 필요):

```sh
cargo install --path .
```

미리 빌드된 아카이브는 [GitHub Releases](https://github.com/dmnkx/homebrew-myip/releases)에도 있습니다.

## 빠른 시작

```sh
myip
myip --version
myip --help
```

## 배포

버전 태그를 푸시하면 됩니다. 릴리스 CI가 `Cargo.toml` / `Cargo.lock`을 그 태그(`v0.1.4` → `0.1.4`)에 맞춘 뒤 테스트를 돌리고, 바이너리를 올리며 Homebrew Formula를 갱신합니다.

Intel용 macOS 바이너리는 Apple Silicon 러너에서 `x86_64-apple-darwin`으로 교차 컴파일합니다.

```sh
git tag v0.1.4
git push origin v0.1.4
```

`cargo test`가 실패하면 릴리스는 만들지 않습니다.

## 라이선스

[MIT](LICENSE)
