# `cargo test`와 기본 assertion

`cargo test`는 테스트에 필요한 코드를 컴파일하고, 그중 `#[test]`가 붙은 함수를
각각 하나의 테스트로 실행합니다.

```bash
cargo test
```

테스트 함수 안에서는 assertion macro로 실제 결과와 기대한 결과를 비교합니다.

- `assert!(condition)`은 조건이 `true`인지 확인합니다.
- `assert_eq!(left, right)`는 두 값이 같은지 확인합니다.
- `assert_ne!(left, right)`는 두 값이 다른지 확인합니다.

다음 예제는 정상적인 나눗셈과 0으로 나누는 경우를 따로 검사합니다.

```rust
{{#include ../../../examples/15_testing.rs}}
```

예제 안의 테스트만 실행하려면 다음 명령을 사용합니다.

```bash
cargo test --example 15_testing
```

## 테스트 골라서 실행하기

`cargo test`에 테스트 이름을 문자열로 전달하면, 선택한 테스트 대상에서 이름에
그 문자열이 들어간 테스트만 실행합니다.

```bash
cargo test --example 15_testing divides
cargo test --example 15_testing returns_none
```

## 테스트 프로그램에 옵션 전달하기

테스트 프로그램에 옵션을 전달할 때는 `--`로 카고<sub>Cargo</sub> 옵션과
구분합니다. 테스트가
통과했을 때도 표준 출력(stdout)과 표준 오류(stderr)에 내보낸 내용을 숨기지
않으려면 다음과 같이 실행합니다.

```bash
cargo test -- --no-capture
```

## 테스트 이름 짓기

테스트 이름에는 구현 방법보다 확인하려는 동작을 적으세요.
`test_checked_divide`보다 `returns_none_when_divisor_is_zero`가 실패 원인을 더 잘
알려 줍니다.
