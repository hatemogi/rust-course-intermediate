# 종합 실습

`src/lib.rs`에는 완성 예제가 들어 있습니다. 아래 요구 사항을 보고 함수를 직접
작성한 뒤 기존 구현과 비교하세요. 테스트는 `tests/exercises.rs`에 있습니다.
`Display`와 `Error` 구현은 작성된 코드를 그대로 두고, 실습에서는 함수 본문에
집중하세요. 두 구현의 역할은 트레이트를 배운 뒤 다룰 예정입니다.

```bash
cargo run --example 16_exercises
cargo test --test exercises
```

## Option 실습

1. `percentage`: 전체 값이 0이면 계산하지 않고 `None`을 반환하세요. 그 외에는
   부분 값이 전체 값에서 차지하는 백분율을 구해 `Some`으로 반환하세요.
   예를 들어 `percentage(1.0, 4.0)`은 `Some(25.0)`입니다. 입력은 유한한 실수라고
   가정합니다.
2. `first_email`: 문자열 목록에서 `@`를 포함한 첫 항목을 찾으세요.
3. `parse_setting`: `KEY=VALUE`에서 `VALUE` 부분을 `u16`으로 변환하세요. 구분자가 없거나
   숫자로 바꿀 수 없으면 `None`을 반환합니다.
4. `parse_all`: 모든 문자열을 `i32`로 변환할 수 있을 때만 목록을 반환하세요.
   하나라도 실패하면 `None`입니다.

## Result 실습

5. `evaluate_division`: `LEFT/RIGHT`를 읽고 구분자가 없는 경우, 숫자로 바꿀 수 없는 경우,
   0으로 나누는 경우, 계산 결과가 범위를 벗어나는 경우를 각각 구분하세요.
   각 숫자 앞뒤의 공백은 무시하세요. `i32::MIN / -1`도 오류로 반환해야 합니다.
6. `parse_ports`: 쉼표로 구분된 포트 번호를 모두 `u16`으로 변환하세요. 하나라도 실패하면
   해당 오류를 반환합니다. 각 숫자 앞뒤의 공백은 무시하세요.
7. `total_payments`: 결제 금액이 0인 경우와 합계가 `u32` 범위를 벗어나는 경우를
   서로 다른 오류로 반환하세요.
   빈 목록의 합계는 0입니다. `checked_add`와 `?`를 사용해 보세요.

## 통합 실습

8. `optional_port`: `Option<&str>`을 `Result<Option<u16>, _>`로 바꾸세요.
   설정이 없으면 `Ok(None)`, 올바른 값이면 `Ok(Some(port))`, 잘못된 값이면
   `Err`를 반환합니다.

## `Option` / `Result` 한눈에 보기

| 목적 | API |
| --- | --- |
| 모든 경우를 나누어 처리 | `match` |
| `Some`/`Ok` 안의 값 변환 | `map` |
| 실패할 수 있는 다음 작업 연결 | `and_then` |
| 값이 없거나 작업이 실패했을 때 기본값 사용 | `unwrap_or`, `unwrap_or_else` |
| 소유권을 옮기지 않고 안의 값 사용 | `as_ref`, `as_mut` |
| `Option` → `Result` | `ok_or`, `ok_or_else` |
| `Result` → `Option` | `ok` |
| 오류에 입력값이나 작업 정보를 덧붙여 변환 | `map_err` |
| 중첩된 `Option`과 `Result`의 안팎 바꾸기 | `Option<Result<T, E>>::transpose` |
| `None`/`Err`에서 즉시 반환 | `?` |
| 모든 항목이 `Some` 또는 `Ok`일 때만 값을 모음 | `collect::<Option<_>>()`, `collect::<Result<_, _>>()` |

값이 없는 것도 정상적인 결과라면 `Option`을, 실패 이유를 알려야 한다면 `Result`를
선택합니다.

`From`으로 오류를 자동 변환하고 `Error::source`로 원인을 연결하는 방법은
트레이트와 타입 변환을 배운 뒤 다룰 예정입니다.
