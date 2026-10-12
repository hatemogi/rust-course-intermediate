# `Option` 이해하고 처리하기

## 값이 있거나 없는 상태 표현하기

Rust는 값이 없는 상태를 `null`로 두지 않고, 다음과 같은 열거형으로 나타냅니다.

```rust
enum Option<T> { Some(T), None }
```

`Some(T)`는 값이 있는 경우이고, `None`은 값이 없는 경우입니다.
함수의 반환형이 `Option<T>`이면, 함수를 호출하는 쪽에서는 값이 없는 경우도
처리해야 한다는 것을 알 수 있습니다.

## 사용자 이름 반환하기

다음 함수는 사용자를 찾으면 이름을 담은 `Some(String)`을, 찾지 못하면
`None`을 반환합니다.

```rust
{{#rustdoc ../../../examples/01_mental_model.rs item=find_user}}
```

`Some`과 `None`은 모두 정상적인 결과일 수 있습니다. 값이 없는 이유까지
호출자에게 알려야 한다면 `Result<T, E>`가 더 잘 어울립니다.

## `match`로 두 경우 모두 처리하기

`find_user`의 결과에 따라 사용자 이름이나 안내 문구를 만들 수 있습니다.
패턴 매칭<sub>pattern matching</sub>에 `match`를 쓰면 `Some`과 `None`을
모두 다루었는지 컴파일러가 확인해 줍니다.

```rust
{{#rustdoc_include ../../../examples/01_mental_model.rs:9:14}}
```

`Some(name)`에서는 안에 담긴 `String`을 꺼내 `name`으로 사용합니다.
`None`에서는 이름이 없으므로 사용자를 찾지 못했다는 문구를 만듭니다.

## `if let`으로 필요한 경우만 처리하기

사용자를 찾았을 때만 인사하고, 찾지 못했을 때는 아무 일도 하지 않으려면
`if let`을 쓸 수 있습니다.

```rust
{{#rustdoc_include ../../../examples/01_mental_model.rs:16:20}}
```

`match`는 두 경우의 처리를 모두 적을 때, `if let`은 특정 패턴에 해당하는
경우만 처리할 때 사용합니다. 다음 장에서는 값이 없을 때 기본값을 사용하는
방법을 살펴봅니다.
