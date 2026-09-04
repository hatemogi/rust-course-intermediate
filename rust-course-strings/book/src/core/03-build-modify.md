# 문자열 만들고 수정하기

## 빈 문자열 만들기

빈 문자열은 `String::new()`로 만들고, 예상 크기를 안다면
`String::with_capacity()`로 공간을 미리 잡을 수 있습니다.

```rust
{{#include ../../../examples/03_build_modify.rs:create}}
```

`String::with_capacity(32)`로 만든 문자열도 처음에는 비어 있지만, 나중에 문자열을
담을 수 있도록 최소 32바이트의 공간을 미리 확보합니다.

## `&str`에서 `String` 만들기

문자열 리터럴과 같은 `&str`을 소유한 문자열로 바꿀 때는 `String::from`이나
`to_owned`를 사용할 수 있습니다.

```rust
{{#include ../../../examples/03_build_modify.rs:from_str}}
```

두 방법 모두 이 예제에서는 `&str`의 내용을 새 메모리 공간에 복사해 같은
`String`을 만듭니다. `String::from`은 만들 타입이 `String`임을 코드에 직접
드러냅니다. `to_owned`는 빌린 값에 호출하여 그 값의 소유 형태를 만든다는 뜻을
드러냅니다.

`to_owned`의 결과가 언제나 `String`인 것은 아닙니다. `str`의 소유 형태가
`String`이기 때문에 `&str`에 호출했을 때 `String`이 됩니다. 따라서 구체적인
결과 타입을 강조하려면 `String::from`, 빌린 값을 소유 값으로 바꾸는 동작을
강조하려면 `to_owned`를 선택할 수 있습니다.

## 문자열 뒤에 내용 추가하기

`push`는 `char` 하나를, `push_str`은 `&str`을 문자열 뒤에 덧붙입니다.

```rust
{{#include ../../../examples/03_build_modify.rs:modify}}
```

두 메서드 모두 기존 `String`을 직접 바꾸므로 변수는 `mut`로 선언해야 합니다.

## 부분 문자열 바꾸기

문자열 안에서 특정 부분을 다른 문자열로 바꾸려면 `replace`를 사용합니다. 첫 번째
인자는 찾을 문자열, 두 번째 인자는 대신 넣을 문자열입니다.

```rust
{{#include ../../../examples/03_build_modify.rs:replace}}
```

`replace`는 일치하는 부분을 모두 바꾼 새로운 `String`을 반환하며, 원본은 바꾸지
않습니다. 처음 몇 개만 바꿔야 한다면 바꿀 개수를 지정할 수 있는 `replacen`을
사용합니다.

## `+` 연산자로 문자열 연결하기

`+` 연산자는 왼쪽 `String`을 소비하므로 여러 값을 조립할 때 소유권 이동을
확인해야 합니다.

```rust
{{#include ../../../examples/03_build_modify.rs:add}}
```

`+`는 왼쪽 피연산자인 `left`의 소유권을 가져와 결과 문자열인 `combined`로
옮깁니다. 따라서 연산 뒤에는 `left`를 다시 사용할 수 없습니다. 오른쪽의
`right`는 `&right`로 빌려 주었으므로 연산 뒤에도 계속 사용할 수 있습니다.

## `format!`으로 문자열 만들기

```rust
{{#include ../../../examples/03_build_modify.rs:format}}
```

여러 값을 읽기 좋게 조립하려면 `format!`이 편리합니다. 인자를 빌려서 표시하므로
호출 뒤에도 원래 문자열을 사용할 수 있습니다. 값의 폭·정렬·채움과
숫자 표현을 지정하는 방법은 다음 5장에서 더 자세히 알아봅니다.
