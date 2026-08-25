# myip

로컬 IPv4와 공인 IPv4를 출력하는 CLI입니다. Homebrew 탭 `dmnkx/myip`로 배포합니다.

설치 시 Rust나 Xcode Command Line Tools를 요구하지 않습니다. GitHub Release의 미리 빌드된 바이너리만 받습니다.

## 설치

릴리스 태그(`v0.1.3` 등)가 올라간 뒤:

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

태그만 올리면 됩니다. 릴리스 CI가 `Cargo.toml`/`Cargo.lock` 버전을 태그(`v0.1.3` → `0.1.3`)에 맞춘 뒤 테스트하고 바이너리를 만듭니다. macOS Intel은 Apple Silicon 러너에서 `x86_64-apple-darwin`으로 교차 컴파일합니다.

```bash
git tag v0.1.3
git push origin v0.1.3
```

`cargo test`가 실패하면 릴리스는 만들지 않습니다.
