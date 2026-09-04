# 그 외 데이터와 문자열 변환

숫자와 날짜 외에도 여러 표준 타입이 문자열 변환을 지원합니다. 문자열을 값으로
바꿀 때는 주로 `parse`, 값을 문자열로 바꿀 때는 `to_string`을 사용합니다.

## 불리언 값 변환하기

`bool`은 문자열 `"true"`와 `"false"`를 값으로 바꿀 수 있습니다.

```rust
{{#include ../../../examples/09_other_conversions.rs:boolean}}
```

대문자가 섞인 `"True"`나 숫자 `"1"`은 `bool`로 변환되지 않습니다. 이런 입력도
허용하려면 변환 전에 별도의 규칙으로 문자열을 처리해야 합니다.

## 문자 하나 변환하기

문자열에 유니코드 스칼라 값이 정확히 하나 들어 있다면 `char`로 변환할 수
있습니다.

```rust
{{#include ../../../examples/09_other_conversions.rs:character}}
```

빈 문자열이나 문자가 둘 이상인 문자열은 `char` 하나로 바꿀 수 없습니다.

## 네트워크 주소 변환하기

표준 라이브러리의 `IpAddr`와 `SocketAddr`도 `parse`와 `to_string`을 지원합니다.

```rust
{{#include ../../../examples/09_other_conversions.rs:network}}
```

`IpAddr`는 IP 주소만 나타내고, `SocketAddr`는 IP 주소와 포트 번호를 함께
나타냅니다. 형식이 잘못되었거나 포트가 범위를 벗어나면 변환에 실패합니다.

## 사용자 정의 타입 변환하기

사용자 정의 타입에 `FromStr`을 구현하면 `parse`로 문자열을 값으로 바꿀 수
있습니다. 반대 방향은 `Display`를 구현하면 `to_string`으로 변환할 수 있습니다.

```rust
{{#include ../../../examples/09_other_conversions.rs:log_level_type}}
```

구현한 타입은 표준 타입과 같은 방식으로 사용할 수 있습니다.

```rust
{{#include ../../../examples/09_other_conversions.rs:log_level_usage}}
```

`FromStr`의 `Err` 연관 타입에는 변환에 실패했을 때 반환할 오류 타입을 지정합니다.
실제 프로그램에서는 어떤 입력이 잘못되었는지 알 수 있는 구체적인 오류 타입을
만드는 편이 좋습니다.
