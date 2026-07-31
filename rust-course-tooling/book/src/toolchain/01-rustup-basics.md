# `rustup`, `rustc`와 카고의 역할

Rust 개발 환경에는 `rustup`·`rustc`·카고<sub>Cargo</sub>처럼 이름이 비슷하지만
역할이 다른 도구가 함께 설치됩니다.

- `rustup`은 Rust 툴체인<sub>toolchain</sub>과 컴포넌트<sub>component</sub>를
  설치하고 선택합니다.
- `rustc`는 Rust 소스 코드를 컴파일합니다.
- `cargo`는 패키지, 의존성, 빌드와 테스트 작업을 관리합니다.

각 도구의 버전은 따로 확인해야 합니다.

```bash
rustup --version
rustc --version
cargo --version
```

`rustup --version`은 rustup 자체의 버전입니다. 현재 사용하는 Rust 컴파일러 버전은
`rustc --version`으로 확인하세요.

> **참고: 툴체인과 컴포넌트**
>
> 툴체인은 특정 Rust 버전의 `rustc`, `cargo`, 표준 라이브러리, 관련 도구를 묶은
> 설치 단위입니다. `stable`, `nightly`, `1.94.0`처럼 툴체인을 구분할 수 있습니다.
> 컴포넌트는 툴체인을 이루는 개별 항목입니다. 선택 컴포넌트는 필요에 따라
> 추가하거나 제거할 수 있으며, 같은 컴포넌트도 툴체인마다 따로 설치합니다.

## 현재 설치 상태 확인하기

다음 명령은 기본 호스트, 설치된 툴체인, 현재 선택된 툴체인, 설치된 컴파일 대상을
보여 줍니다.

```bash
rustup show
```

필요한 목록만 따로 확인할 수도 있습니다.

```bash
rustup toolchain list
rustup component list --installed
rustup target list --installed
```

일반적인 프로젝트에서는 안정판인 `stable`을 사용합니다. 실험 기능이 꼭 필요한
경우에만 범위를 좁혀 `nightly`를 검토하세요.

## 툴체인 설치하고 업데이트하기

```bash
rustup toolchain install stable
rustup update stable
rustup default stable
```

- `toolchain install`은 해당 툴체인이 없으면 설치합니다.
- `update stable`은 현재 컴퓨터에 설치된 stable 툴체인을 stable 채널의 최신
  릴리스로 바꿉니다.
- `default`는 별도 지정이 없는 디렉터리에서 사용할 기본 툴체인을 정합니다.

이 명령들은 사용자 컴퓨터의 Rust 환경을 바꾸고 일부는 파일을 내려받습니다. 팀
프로젝트의 명령을 따라 하기 전에 변경 범위와 필요한 버전을 확인하세요.

## 한 번만 다른 툴체인 사용하기

명령 이름 뒤에 `+toolchain`을 붙이면 기본값을 바꾸지 않고 그 명령에만 다른
툴체인을 사용할 수 있습니다.

```bash
rustc +stable --version
cargo +stable check
cargo +nightly fmt
```

> **참고: 설치되지 않은 툴체인**
>
> 기본 설정에서는 `nightly`가 설치되어 있지 않다면 rustup이 먼저 설치한 뒤 명령을
> 실행합니다.
> 단순히 최신 버전처럼 보인다는 이유로 프로젝트 전체의 기본값을 nightly로 바꾸지
> 마세요.
> `nightly`는 매일 업데이트되는 Rust 개발 채널이며, stable에 아직 포함되지 않은 실험
> 기능이 꼭 필요할 때 주로 사용합니다.

## 컴포넌트 관리하기

다음 두 도구는 툴체인에 추가하는 컴포넌트입니다.

```bash
rustup component add rustfmt clippy
rustup component list --installed
```

특정 툴체인에 추가하려면 `--toolchain`을 사용합니다.

```bash
rustup component add --toolchain stable rustfmt clippy
```

> **참고: rustfmt와 클리피**
>
> `rustfmt`는 Rust 코드를 정해진 형식으로 자동 정리합니다.
> 클리피<sub>Clippy</sub>는 코드를 분석해 실수 가능성이나 불필요하게 복잡한
> 표현을 알려 주는 공식 린트 도구입니다.

## 컴파일 타깃 관리하기

다른 운영체제나 웹어셈블리<sub>WebAssembly</sub>용 결과물이 필요하면 타깃을
추가합니다.

```bash
rustup target list --installed
rustup target add wasm32-unknown-unknown
rustup target remove wasm32-unknown-unknown
```

> **참고: 크로스 컴파일 준비**
>
> 크로스 컴파일<sub>cross-compilation</sub>은 현재 컴퓨터가 아닌 다른
> 운영체제·CPU 아키텍처·실행 환경에서 사용할 결과물을 만드는 작업입니다.
> 타깃을 추가하는 것만으로 모든 크로스 컴파일 준비가 끝나는 것은 아닙니다.
> 대상에 따라 링커와 시스템 라이브러리가 별도로 필요할 수 있습니다.

## 혼동하기 쉬운 두 업데이트 명령

```bash
rustup update stable
cargo update
```

첫 번째 명령은 Rust 컴파일러와 툴체인을 업데이트합니다. 두 번째 명령은 현재
프로젝트가 사용하는 크레이트<sub>crate</sub> 버전을 다시 선택하고 `Cargo.lock`을
업데이트합니다. 이름은 비슷하지만 바꾸는 대상이 완전히 다릅니다.

> **참고: 크레이트**
>
> 크레이트는 Rust 컴파일러가 한 번에 컴파일하는 코드 단위입니다. 라이브러리나
> 실행 파일 형태로 만들 수 있습니다. `crates.io`에는 외부 라이브러리 패키지가
> 크레이트 단위로 공개되어 있으며, 이를 `Cargo.toml`의 의존성에 추가해 사용할 수
> 있습니다.
