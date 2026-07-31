# 오류, panic, 제외한 테스트

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

```rust
#[test]
fn rejects_a_port_with_letters() {
    let error = "http"
        .parse::<u16>()
        .expect_err("숫자가 아닌 포트는 거부해야 합니다");

    assert_eq!(error.kind(), &std::num::IntErrorKind::InvalidDigit);
}
```

`expect_err`는 변환이 예상과 달리 성공하면 테스트를 실패시키고, 실패하면 오류를
꺼냅니다. 이어서 `kind()`의 결과를 비교해 숫자가 아닌 문자가 원인이었는지
확인합니다.

> **참고: `Box<dyn std::error::Error>`는 무엇인가?**
>
> `std::error::Error`는 여러 오류 타입이 공통으로 구현하는 trait입니다.
> `dyn std::error::Error`는 구체적인 오류 타입 대신 이 trait을 구현한 오류를
> 받겠다는 뜻입니다. 오류 타입마다 크기가 다를 수 있으므로 `Box`에 넣어 크기가
> 일정한 포인터로 다룹니다. 이렇게 반환 타입을 쓰면 여러 종류의 오류에 `?`를
> 사용할 수 있습니다. 이 예제처럼 정수 변환 오류만 생긴다면
> `Box<dyn std::error::Error>` 대신 구체적인 `ParseIntError`를 반환해도 됩니다.

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
알맞습니다. `should_panic`을 사용하기 전에 해당 상황에서 panic이 적절한 동작인지
먼저 판단하세요.

## 평소 실행에서 제외하기

오래 걸리거나 특별한 외부 환경이 필요한 테스트에는 `#[ignore]`를 붙일 수 있습니다.
`#[ignore]`가 붙은 테스트는 기본적으로 실행되지 않습니다.

```bash
cargo test -- --ignored
```

`--ignored`는 `#[ignore]`가 붙은 테스트만 실행하며 평소 실행하는 테스트는
제외합니다. 평소 테스트와 `#[ignore]`가 붙은 테스트를 모두 실행하려면
`--include-ignored`를 사용합니다.

```bash
cargo test -- --include-ignored
```

다음 실행 가능한 예제에서 `Result`, `should_panic`, `ignore`를 함께 확인할 수
있습니다.

```rust
{{#include ../../../examples/17_special_tests.rs}}
```

```bash
cargo test
```

테스트가 자주 실패한다는 이유만으로 `ignore`를 붙여서는 안 됩니다. 실행 순서, 공유
상태, 시간, 외부 서비스 때문에 불규칙하게 실패한다면 먼저 원인을 제거해야 합니다.
