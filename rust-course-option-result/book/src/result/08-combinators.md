# 변환과 연결

`map`은 `Ok` 안의 값을, `map_err`는 `Err` 안의 오류를 바꿉니다. 다음 작업도
`Result`를 반환한다면 `and_then`으로 연결할 수 있습니다.

```rust
{{#rustdoc ../../../examples/08_result_combinators.rs item=parse_port..require_nonzero}}
```

문자열을 숫자로 바꾸는 데는 성공했지만, 포트 번호가 0이어서 다음 검사에서
실패하는 경우도 확인해 봅니다.

```rust
{{#rustdoc_include ../../../examples/08_result_combinators.rs:15:28}}
```

메서드를 길게 이어 써서 코드가 읽기 어려워졌다면, 중간 결과를 변수에 담고
`?`를 사용해 단계별로 풀어 쓰세요.

## 어느 메서드를 쓸까?

아래 표는 `Result<T, E>`에 함수를 넘기는 경우입니다. `U`는 바뀐 값의 타입이고,
`F`는 바뀐 오류의 타입입니다.

| 메서드 | 함수에 넘기는 값 | 함수의 반환 타입 | 메서드의 결과 타입 |
| --- | --- | --- | --- |
| `map` | `Ok` 안의 `T` | `U` | `Result<U, E>` |
| `map_err` | `Err` 안의 `E` | `F` | `Result<T, F>` |
| `and_then` | `Ok` 안의 `T` | `Result<U, E>` | `Result<U, E>` |

`map`과 `and_then`은 `Err`를 그대로 반환하고, `map_err`는 `Ok`를 그대로 반환합니다.
해당하지 않는 경우에는 넘긴 함수를 실행하지 않습니다.
