# 검증 흐름 실습

이제껏 배운 도구들을 한 번에 연결해 봅니다. 이 실습의 목표는 모든 명령을 외우는
것이 아니라, 문제가 생겼을 때 어느 단계로 돌아가야 하는지 판단해보는 것입니다.

## 1. 프로젝트 환경 확인하기

강의 프로젝트의 루트에서 다음 명령을 실행합니다.

```bash
rustup show active-toolchain
rustc --version
cargo --version
```

출력에서 현재 선택된 툴체인<sub>toolchain</sub>을 확인합니다. 이 강의 저장소의
`rust-toolchain.toml`처럼 프로젝트에 설정 파일을 추가했다면 선택된 버전이 파일에
적은 값과 맞는지도 확인합니다. rustfmt와 클리피<sub>Clippy</sub>가 없다면 필요한
컴포넌트<sub>component</sub>만 설치합니다.

```bash
rustup component add rustfmt clippy
```

## 2. 컴파일 오류 설명하기

`examples/compile_fail/moved_value.rs`는 컴파일 오류를 살펴보기 위해 일부러 잘못된
코드를 넣은 예제입니다.

```rust,compile_fail
{{#include ../../../examples/compile_fail/moved_value.rs}}
```

코드를 읽고 다음 질문에 답합니다.

1. 소유권은 어느 호출에서 이동하는가?
2. 컴파일러는 어느 줄의 사용을 오류로 표시하는가?
3. `print_message`가 문자열을 읽기만 한다면 매개변수 타입을 어떻게 바꿀 수 있는가?
4. `clone()`을 추가하는 방법이 이 예제에 알맞지 않은 이유는 무엇인가?

필요하면 오류 코드의 설명을 확인합니다.

```bash
rustc --explain E0382
```

## 3. 프로젝트 전체 검사하기

다음 명령을 순서대로 실행합니다.

```bash
cargo check --all-targets --locked
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
```

각 명령이 통과하면 다음 사실을 확인한 것입니다.

| 명령 | 확인한 내용 | 확인하지 못한 내용 |
| --- | --- | --- |
| `cargo check` | 모든 카고<sub>Cargo</sub> 타깃을 컴파일할 수 있다 | 요구 사항대로 동작한다 |
| `cargo fmt --check` | Rust 코드 형식이 표준이다 | 코드의 뜻과 이름이 알맞다 |
| `cargo clippy` | 린트<sub>lint</sub>가 지적할 사항이 없다 | 모든 입력에서 올바르게 동작한다 |

어떤 명령이 실패했다면 첫 번째 진단 사항부터 확인해 수정한 뒤 같은 명령을 다시
실행합니다.
수정한 내용은 `git diff`로 확인하고, 실패했던 명령을 다시 실행해 문제가
해결됐는지 확인합니다.
