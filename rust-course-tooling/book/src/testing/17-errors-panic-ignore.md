# 오류, panic과 제외한 테스트

## `Result`를 반환하는 테스트

테스트 함수는 `Result<(), E>`를 반환할 수 있습니다. 성공하면 `Ok(())`를 반환하고,
오류가 발생하면 `?`가 테스트를 실패로 끝냅니다. 준비 과정에서 여러 실패 가능한
작업을 연결할 때 유용합니다.

```rust
#[test]
fn parses_a_port() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!("443".parse::<u16>()?, 443);
    Ok(())
}
```

오류 발생 자체가 요구 사항이라면 `?`로 바로 끝내지 말고 오류의 종류와 필요한
정보를 명시적으로 검사하세요.

## panic 검사하기

panic이 계약에 포함된 API는 `#[should_panic]`으로 검사할 수 있습니다. 가능하면
`expected`에 메시지 일부를 적어 엉뚱한 panic으로 테스트가 통과하는 일을 줄입니다.

```rust
#[test]
#[should_panic(expected = "index out of bounds")]
fn panics_when_the_slice_is_empty() {
    let values: Vec<i32> = Vec::new();
    let _ = values[0];
}
```

예상할 수 있는 잘못된 사용자 입력은 panic보다 `Result`로 처리하는 편이 보통 더
알맞습니다. `should_panic`을 사용하기 전에 panic 자체가 올바른 계약인지 판단하세요.

## 평소 실행에서 제외하기

오래 걸리거나 특별한 외부 환경이 필요한 테스트에는 `#[ignore]`를 붙일 수 있습니다.

```bash
cargo test -- --ignored
```

평소 테스트까지 포함해 모두 실행하려면 다음 명령을 사용합니다.

```bash
cargo test -- --include-ignored
```

다음 실행 가능한 예제에서 `Result`, `should_panic`, `ignore`를 함께 확인할 수
있습니다.

```rust
{{#include ../../../examples/06_special_tests.rs}}
```

```bash
cargo test --example 06_special_tests
cargo test --example 06_special_tests -- --ignored
```

테스트가 자주 실패한다는 이유만으로 `ignore`를 붙여서는 안 됩니다. 실행 순서, 공유
상태, 시간과 외부 서비스 때문에 불규칙하게 실패한다면 먼저 원인을 제거해야 합니다.
