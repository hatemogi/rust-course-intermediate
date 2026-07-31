# 프로젝트에서 Rust 버전을 선택하고 확인하기

`rustup default`는 사용자 컴퓨터 전체의 기본값을 바꿉니다. 프로젝트마다 필요한 Rust
버전이 다르다면 저장소 루트의 `rust-toolchain.toml`에 사용할
툴체인<sub>toolchain</sub>을 설정할 수 있습니다. 파일이 있는 디렉터리와 그 아래에서
rustup이 알맞은 툴체인을
선택합니다.

## 프로젝트 설정 파일 만들기

프로젝트 저장소 루트에 `rust-toolchain.toml` 파일을 만들고 다음 내용을 저장합니다.
이 강의에서는 정확한 버전을 지정합니다.

```toml
{{#include ../../../rust-toolchain.toml}}
```

- `channel`은 사용할 Rust 채널이나 정확한 버전을 정합니다.
- `profile = "minimal"`은 문서 같은 추가 컴포넌트<sub>component</sub>를 제외한
  최소한의 설치 구성을 선택합니다.
- `components`는 프로젝트 검증에 필요한 rustfmt와 클리피<sub>Clippy</sub>를
  함께 설치합니다.

크로스 컴파일 대상까지 공통으로 필요하다면 `targets`를 추가할 수 있습니다.

```toml
[toolchain]
channel = "1.94.0"
profile = "minimal"
components = ["rustfmt", "clippy"]
targets = ["wasm32-unknown-unknown"]
```

저장소에 이 파일을 커밋하면 다른 개발자나 CI가 모두 같은 툴체인 설정을
사용합니다.

> **참고: 처음 사용할 때 필요한 네트워크**
>
> 아직 설치되지 않은 툴체인이나 컴포넌트는 처음 명령을 실행할 때 내려받을 수
> 있습니다. 네트워크가 제한된 환경에서는 미리 설치 상태를 확인해야 합니다.

## `stable`과 정확한 버전 가운데 고르기

```toml
channel = "stable"
```

`stable`은 환경을 설치하거나 업데이트한 시점에 따라 실제 컴파일러 버전이 달라질 수
있습니다. 새 Rust 개선을 자연스럽게 따라갈 수 있지만 모든 사람이 같은 버전을 쓴다는
보장은 약합니다.

```toml
channel = "1.94.0"
```

정확한 버전을 지정하면 개발자와 CI가 같은 컴파일러로 검사할 수 있습니다. 다만
컴파일러의 버그 수정이나 새 린트<sub>lint</sub>를 적용하려면 `channel`의 버전을 직접 올린 뒤 전체
검증을 실행해야 합니다. 같은 결과를 재현하기 쉬운 점과 버전을 관리하는 수고를 함께
고려해 선택하세요.

## `rust-version`과 구분하기

`rust-toolchain.toml`은 이 저장소에서 명령을 실행할 때 rustup이 선택할 툴체인을
정합니다. 반면 `Cargo.toml`의 `rust-version`은 패키지가 지원하는 가장 오래된 Rust
버전을 카고<sub>Cargo</sub>에 알립니다.

```toml
[package]
edition = "2024"
rust-version = "1.85"
```

두 값은 목적이 다릅니다. 개발자와 CI가 예제처럼 `1.94.0`으로 작업하더라도 패키지가
`rust-version = "1.85"`를 적었다면, 지원 범위를 지키는지 확인하려고 1.85에서도
별도 검사를 실행할 수 있습니다. `rust-toolchain.toml`의 버전을 올렸다고 패키지가
지원하는 가장 오래된 버전까지 저절로 바뀌지는 않습니다.

`rust-version`을 적어 두면 그보다 오래된 카고가 패키지를 빌드하려 할 때 분명한
오류를 낼 수 있고, 의존성을 고를 때도 이 정보를 사용할 수 있습니다. 실제로
지원하지 않는 오래된 버전을 추측해 적지 말고 자동 검사에서 확인하는 버전을
기록하세요. 자세한 동작은 [카고의 Rust 버전 문서](https://doc.rust-lang.org/cargo/reference/rust-version.html)에서
확인할 수 있습니다.

MSRV는 Minimum Supported Rust Version의 약자이며, 패키지가 지원하는 가장 오래된
Rust 버전을 뜻합니다.

이 강의 프로젝트에서는 1.85 툴체인을 설치한 환경에서 다음 명령으로 지원 범위를
확인합니다.

```bash
make msrv
```

`make check`는 현재 선택된 툴체인에서 전체 검증을 실행하고, `make msrv`는
`rust-version`에 적은 버전에서 컴파일·테스트·문서 테스트를 실행합니다.

> **참고: `make`는 무엇인가요?**
>
> `make` 명령은 Makefile에 적힌 내용을 바탕으로 여러 명령을 작업 이름별로
> 실행합니다. 예를 들어 `check:` 아래에 여러 검사 명령을 적어 두면 `make check`
> 한 번으로 정해진 순서에 따라 실행할 수 있습니다. `make`와 Makefile은 Rust 전용
> 도구가 아니며, 이 강의 프로젝트에서는 긴 카고 명령을 매번 입력하지 않고 누구나
> 같은 검증 절차를 반복하도록 사용합니다.

## 디렉터리 override

파일을 만들지 않고 현재 디렉터리에 툴체인을 지정할 수도 있습니다.

```bash
rustup override set stable
rustup override list
rustup override unset
```

override는 사용자 rustup 설정에 저장되고 Git으로 공유되지 않습니다. 잠깐 로컬에서
확인할 때는 편리하지만 팀이 함께 따라야 하는 버전은 대개 `rust-toolchain.toml`이 더
분명합니다. 실습 뒤에는 불필요한 override가 남지 않았는지 확인하세요.

## 선택된 툴체인 확인하기

프로젝트 디렉터리에서 다음 명령을 실행합니다.

```bash
rustup show active-toolchain
rustc --version
cargo --version
```

설정 파일을 추가했다고 끝내지 말고 실제 선택된 버전과 필요한 컴포넌트를
확인하세요. 버전을 올릴 때는 포맷, 클리피, 컴파일, 테스트, 문서 테스트를 모두
실행해 새 컴파일러에서 달라진 결과가 없는지 검토합니다.

## AI가 만든 설정 검토하기

AI가 습관적으로 `nightly`나 오래된 정확한 버전을 고정할 수 있습니다. 제안을
받았다면 다음을 확인하세요.

- nightly 기능을 실제로 사용하는가?
- `rustfmt`와 클리피가 필요한 컴포넌트에 포함됐는가?
- 필요하지 않은 target을 모든 개발자에게 설치하게 하지는 않는가?
- 정확한 버전을 누가 언제 업데이트할 것인가?
- `rust-version`으로 약속한 가장 오래된 버전을 자동 검사에서 확인하는가?
- CI도 같은 파일을 읽도록 구성됐는가?
