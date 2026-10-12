# transpose로 Option과 Result의 안팎 바꾸기

값이 없어도 괜찮지만, 값이 있다면 올바른지 검사해야 하는 경우에는 `Option`과
`Result`를 겹쳐 씁니다. `transpose()`는 값과 오류를 보존하면서 두 타입의 안팎을
바꿉니다. 설정 검사와 여러 결과를 모으는 예제로 양쪽 방향을 살펴봅니다.

## 설정이 있을 때만 검사하기

`Option<Result<T, E>>`에 `transpose()`를 호출하면 `Result<Option<T>, E>`로 바꿀 수
있습니다. 입력값이 없어도 괜찮지만, 값이 있다면 올바른 값인지 확인해야 할 때
유용합니다.

```rust
{{#rustdoc ../../../examples/15_transpose.rs item=optional_port}}
```

## transpose의 세 가지 경우

| 변환 전: `Option<Result<T, E>>` | 변환 후: `Result<Option<T>, E>` | 뜻 |
| --- | --- | --- |
| `None` | `Ok(None)` | 설정이 없으므로 검사할 값도 없음 |
| `Some(Ok(value))` | `Ok(Some(value))` | 설정이 있고 올바른 값임 |
| `Some(Err(error))` | `Err(error)` | 설정이 있지만 검사에 실패함 |

`transpose`가 직접 값을 검사하는 것은 아닙니다. 앞서 `map(str::parse)`에서 얻은
검사 결과의 바깥 타입을 바꾸어, 설정 없음과 실패를 구분하기 쉽게 만듭니다.

## 반대로 Option을 바깥으로 꺼내기

`Result`에도 `transpose()`가 있습니다. `Result<Option<T>, E>`를
`Option<Result<T, E>>`로 바꾸어, 값이 없는 경우를 바깥의 `None`으로 나타냅니다.

| 변환 전: `Result<Option<T>, E>` | 변환 후: `Option<Result<T, E>>` | 뜻 |
| --- | --- | --- |
| `Ok(None)` | `None` | 정상적으로 처리했지만 값이 없음 |
| `Ok(Some(value))` | `Some(Ok(value))` | 정상적으로 처리했고 값이 있음 |
| `Err(error)` | `Some(Err(error))` | 오류를 버리지 않고 남김 |

두 방향 모두 값과 오류를 보존합니다. `transpose().transpose()`를 호출하면
원래 값으로 돌아옵니다.

## 설정이 없는 항목만 제외하고 오류는 남기기

여러 서비스의 포트 설정을 검사한다고 생각해 봅시다. 앞서 만든 `optional_port`는
설정이 없으면 `Ok(None)`, 올바르면 `Ok(Some(포트 번호))`, 잘못되었으면 `Err`를
반환합니다. 이 결과에서 설정이 없는 항목만 제외하고 싶을 수 있습니다.

```rust
{{#rustdoc ../../../examples/15_transpose.rs item=optional_port}}

{{#rustdoc ../../../examples/15_transpose.rs item=keep_configured_ports}}
```

`filter_map`은 넘긴 함수가 `None`을 반환하면 그 항목을 제외하고,
`Some`을 반환하면 그 안의 값을 다음 단계로 넘깁니다. 여기에 `Result::transpose`를
넘기면 `Ok(None)`만 제외됩니다. `Ok(Some(443))`은 `Ok(443)`으로,
`Err(error)`는 `Err(error)`로 남습니다.

마지막 `collect()`는 `Vec`으로 모으므로 오류가 있어도 끝까지 처리합니다.
결과에는 입력 순서대로 `Ok(443)`, `Err(error)`, `Ok(8080)`이 담깁니다.
반면 `.filter_map(Result::ok)`를 쓰면 오류가 사라지고 `Ok(None)`의 안쪽 값인
`None`은 남으므로, 이 예제의 목적과 다릅니다.
