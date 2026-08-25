# myip

로컬 IPv4와 공인 IPv4를 출력하는 CLI입니다. Homebrew 탭 `dmnkx/myip`로 배포합니다.

설치 시 Rust나 Xcode Command Line Tools를 요구하지 않습니다. GitHub Release의 미리 빌드된 바이너리만 받습니다.

## 설치

첫 릴리스 태그(`v0.1.0`)가 올라간 뒤:

```bash
brew tap dmnkx/myip
brew install myip
```

## 사용

```bash
myip
# local: 192.168.x.x
# public: x.x.x.x

myip --version
```

## 배포

1. 버전을 올립니다.

```bash
bash scripts/bump-version.sh 0.1.0
git add Cargo.toml Cargo.lock
git commit -m "chore: release 0.1.0"
git push origin main
```

2. 태그를 올리면 테스트 통과 후 바이너리를 만듭니다. macOS Intel은 중단된 `macos-13` 대신 Apple Silicon 러너에서 `x86_64-apple-darwin`으로 교차 컴파일합니다.

```bash
git tag v0.1.0
git push origin v0.1.0
```

`Cargo.toml`의 `version`과 태그(`v0.1.0`)는 같아야 합니다. `cargo test`가 실패하면 릴리스는 만들지 않습니다.
