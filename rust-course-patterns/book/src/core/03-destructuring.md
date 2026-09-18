# 튜플·구조체·열거형 분해

패턴으로 튜플의 위치, 구조체의 필드 이름, 열거형의 변형을 따라 중첩된 값을 한 번에
구조 분해<sub>destructuring</sub>할 수 있습니다. 튜플에서는 `(first, second)`처럼
각 위치에 있는 값을 변수에 바인딩<sub>binding</sub>하고, 구조체에서는
`Point { x, y }`처럼 필드 이름으로 매칭<sub>matching</sub>합니다.

```rust
{{#rustdoc ../../../examples/03_destructuring.rs body=struct_pattern}}
```

매칭할 필드 이름과 값을 받을 변수 이름이 같으면 `x: x`를 `x`로 줄입니다.
예제의 `y: 0`은 `y`에 새 이름을 붙이지 않고 필드 값이 `0`인지 검사합니다.
`y`가 `0`이 아니면 매칭되지 않으므로 `let else`로 매칭 실패를 처리합니다.

## 필요 없는 필드 생략하기

구조체의 일부 필드만 필요하다면, 나머지 필드는 `..`로 생략합니다.

```rust
{{#rustdoc ../../../examples/03_destructuring.rs body=remaining_fields}}
```

`Point3D { x, .. }`는 `x`만 바인딩하고 `y`와 `z`는 검사하거나 바인딩하지
않습니다. 필드가 나중에 더 많아져도 `..`는 나열하지 않은 필드를 모두 생략합니다.

## 열거형 분해하기

열거형<sub>enum</sub>은 변형<sub>variant</sub>마다 담긴 값의 구조가 다를 수
있습니다. 예제의 `Move`는 이름 있는 필드를 담는 구조체형 변형이고, `Write`는
위치로 구분되는 값 하나를 담는 튜플형 변형입니다.

```rust
{{#rustdoc ../../../examples/03_destructuring.rs item=Message}}
```

각 변형을 선언한 형태에 맞춰 분해합니다.

```rust
{{#rustdoc ../../../examples/03_destructuring.rs body=enum_pattern}}
```

패턴으로 열거형의 변형을 확인하면서 그 안의 값을 변수로 받을 수 있습니다.
`Message::Move { x, y }`는 `Move`에 매칭되며 두 필드의 값을 `x`와 `y`로 받습니다.
`Message::Write(text)`는 `Write`에 매칭되며 문자열을 `text`로 받습니다.
각 갈래에서는 해당 변형에 담긴 값을 사용합니다.
