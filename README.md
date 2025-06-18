# clrun

간단한 C/C++ 컴파일러 실행 스크립트입니다. `clang`이나 `clang++`을 사용해서 컴파일하고 실행까지 자동으로 합니다.

## 설치

### macOS (Intel)

```bash
curl -L https://github.com/kangjiungdev/clrun/releases/download/v0.2.0/clrun-x86_64-apple-darwin.zip -o clrun.zip

unzip clrun.zip

sudo mv clrun /usr/local/bin/

rm clrun.zip
```

### Linux (x86_64)

```bash
curl -L https://github.com/kangjiungdev/clrun/releases/download/v0.2.0/clrun-x86_64-unknown-linux-gnu.tar.gz -o clrun.tar.gz

tar -xzf clrun.tar.gz

sudo mv clrun /usr/local/bin/

rm clrun.tar.gz
```

## 사용법

```bash
  clrun [options]
  clrun <language> <filename>
```

### Languages

- `c` : C 코드 (`clang`)
- `cpp` 또는 `c++` : C++ 코드 (`clang++`)

### Options

- `-h` 또는 `--help` : 도움말
- `-v` 또는는 `--version` : 버전 정보

### 예시

```bash
clrun c main.c
clrun cpp main.cpp
```

## 설명

- `clang` 또는 `clang++`로 컴파일합니다.
- 실행 파일을 만들어서 바로 실행합니다.
- 실행이 끝나면 생성된 파일은 삭제합니다.

## 요구 사항

- macOS 또는 Linux
- `clang`, `clang++` 설치되어 있어야 함
- 실행 가능한 쉘 환경
