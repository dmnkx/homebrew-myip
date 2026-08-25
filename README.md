# myip

로컬 IPv4와 공인 IPv4를 출력하는 CLI입니다. Homebrew 탭 `dmnkx/myip`로 배포합니다.

## 설치

```bash
brew tap dmnkx/myip
brew install --HEAD myip
```

첫 릴리스(`v0.1.0` 태그) 이후에는 `--HEAD` 없이 설치할 수 있습니다.

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

2. 태그를 올리면 GitHub Actions가 소스 tar.gz 릴리스를 만들고 `Formula/myip.rb`의 `url`/`sha256`을 `main`에 커밋합니다.

```bash
git tag v0.1.0
git push origin v0.1.0
```

`Cargo.toml`의 `version`과 태그(`v0.1.0`)는 같아야 합니다.
