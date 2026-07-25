# 첫 번째 검증 흐름 실습

도구 체인, Cargo, 형식과 정적 검사에서 배운 도구를 한 번에 연결해 봅니다. 이
실습의 목표는 모든 명령을 외우는 것이 아니라, 문제가 생겼을 때 어느 단계로
돌아가야 하는지 설명하는 것입니다.

## 1. 프로젝트 환경 확인하기

강의 프로젝트의 루트에서 다음 명령을 실행합니다.

```bash
rustup show active-toolchain
rustc --version
cargo --version
```

출력에서 현재 선택된 도구 체인을 확인합니다. 이 강의 저장소의
`examples/toolchain/rust-toolchain.toml`처럼 프로젝트에 설정 파일을 추가했다면
선택된 버전이 선언과 맞는지도 확인합니다. rustfmt와 Clippy가 없다면 필요한 구성
요소만 설치합니다.

```bash
rustup component add rustfmt clippy
```

## 2. 컴파일 오류 설명하기

`examples/compile_fail/moved_value.rs`는 일부러 컴파일되지 않게 작성한
예제입니다.

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
| `cargo check` | 모든 Cargo 타깃을 컴파일할 수 있다 | 요구 사항대로 동작한다 |
| `cargo fmt --check` | Rust 코드 형식이 맞다 | 코드의 뜻과 이름이 알맞다 |
| `cargo clippy` | 선택한 범위에서 lint가 남지 않았다 | 모든 입력에서 올바르게 동작한다 |

어느 명령도 다른 명령을 대신하지 않습니다. 명령이 실패했다면 첫 번째 진단부터
읽고, 수정한 뒤 같은 명령을 다시 실행합니다.

## 4. 검토 결과 기록하기

다음 형식으로 짧게 기록합니다.

```text
- 사용한 도구 체인:
- 처음 실패한 명령:
- 진단이 알려 준 사실:
- 선택한 수정:
- 채택하지 않은 수정과 이유:
- 다시 실행해 통과한 명령:
```

이 기록은 뒤에서 자동 수정의 diff를 검토하고 테스트와 벤치마크까지 연결할 때
다시 사용합니다.
