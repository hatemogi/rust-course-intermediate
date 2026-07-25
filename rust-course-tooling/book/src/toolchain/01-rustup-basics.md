# `rustup`, `rustc`와 Cargo의 역할

Rust 개발 환경에는 이름이 비슷하지만 역할이 다른 도구가 함께 설치됩니다.

- `rustup`은 Rust 도구 체인과 구성 요소를 설치하고 선택합니다.
- `rustc`는 Rust 소스 코드를 컴파일합니다.
- Cargo는 패키지, 의존성, 빌드와 테스트 작업을 관리합니다.

각 도구의 버전은 따로 확인해야 합니다.

```bash
rustup --version
rustc --version
cargo --version
```

`rustup --version`은 rustup 자체의 버전입니다. 현재 사용하는 Rust 컴파일러 버전은
`rustc --version`으로 확인하세요.

## 현재 설치 상태 확인하기

다음 명령은 기본 호스트, 설치된 도구 체인, 현재 선택된 도구 체인과 설치된 컴파일
대상을 보여 줍니다.

```bash
rustup show
```

필요한 목록만 따로 확인할 수도 있습니다.

```bash
rustup toolchain list
rustup component list --installed
rustup target list --installed
```

도구 체인은 `rustc`, 표준 라이브러리와 관련 도구를 묶은 설치 단위입니다. 일반적인
프로젝트에서는 안정판인 `stable`을 사용합니다. 실험 기능이 꼭 필요한 경우에만
범위를 좁혀 `nightly`를 검토하세요.

## 도구 체인 설치하고 갱신하기

```bash
rustup toolchain install stable
rustup update stable
rustup default stable
```

- `toolchain install`은 해당 도구 체인이 없으면 설치합니다.
- `update`는 설치된 채널을 새 버전으로 갱신합니다.
- `default`는 별도 지정이 없는 디렉터리에서 사용할 기본 도구 체인을 정합니다.

이 명령들은 사용자 컴퓨터의 Rust 환경을 바꾸고 일부는 파일을 내려받습니다. 팀
프로젝트의 명령을 따라 하기 전에 변경 범위와 필요한 버전을 확인하세요.

## 한 번만 다른 도구 체인 사용하기

명령 이름 뒤에 `+toolchain`을 붙이면 기본값을 바꾸지 않고 그 명령에만 다른 도구
체인을 사용할 수 있습니다.

```bash
rustc +stable --version
cargo +stable check
cargo +nightly fmt
```

마지막 명령은 nightly가 설치되어 있을 때만 실행됩니다. 단순히 최신 버전처럼
보인다는 이유로 프로젝트 전체의 기본값을 nightly로 바꾸지 마세요.

## 구성 요소 관리하기

`rustfmt`와 Clippy는 도구 체인에 추가하는 구성 요소입니다.

```bash
rustup component add rustfmt clippy
rustup component list --installed
```

특정 도구 체인에 추가하려면 `--toolchain`을 사용합니다.

```bash
rustup component add --toolchain stable rustfmt clippy
```

## 컴파일 대상 관리하기

다른 운영체제나 WebAssembly용 표준 라이브러리가 필요하면 target을 추가합니다.

```bash
rustup target list --installed
rustup target add wasm32-unknown-unknown
rustup target remove wasm32-unknown-unknown
```

target을 추가하는 것만으로 모든 크로스 컴파일 준비가 끝나는 것은 아닙니다. 대상에
따라 linker와 시스템 라이브러리가 별도로 필요할 수 있습니다.

## 혼동하기 쉬운 두 갱신 명령

```bash
rustup update stable
cargo update
```

첫 번째 명령은 Rust 컴파일러와 도구 체인을 갱신합니다. 두 번째 명령은 현재
프로젝트가 사용하는 crate 버전을 다시 선택하고 `Cargo.lock`을 갱신합니다. 이름은
비슷하지만 바꾸는 대상이 완전히 다릅니다.
