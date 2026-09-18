# `if let`, `let else`, `while let`

한 패턴만 관심 있고 나머지는 한꺼번에 처리한다면 `if let`이 간결합니다. `match`와
달리 모든 경우를 갈래<sub>branch</sub>로 나열하지 않아도 되지만,
매칭<sub>matching</sub>되지 않은 값을 별도로 처리해야 한다면 `else`를 붙이거나
`match`를 사용하는 편이 뜻을 분명하게 드러냅니다.

```rust
{{#rustdoc ../../../examples/04_control_flow.rs body=if_let_pattern}}
```

`let else`는 패턴이 매칭되지 않을 때 현재 함수에서 반환하거나 반복을 중단하게
하고, 매칭된 값을 뒤 코드에서 사용합니다. `else` 블록은 `return`,
`break`, `continue`, 패닉<sub>panic</sub>처럼 이후 코드로 되돌아오지 않아야
합니다. 그래야 블록 다음에서 바인딩된 값이 반드시 존재한다고 판단할 수 있습니다.

```rust
{{#rustdoc ../../../examples/04_control_flow.rs body=double_number_or_zero}}
```

`while let`은 패턴이 계속 매칭되는 동안 반복합니다. 반복할 때마다 오른쪽 식을
다시 평가하고, 패턴이 처음 매칭되지 않는 순간 루프를 끝냅니다.

```rust
{{#rustdoc ../../../examples/04_control_flow.rs body=while_let_pattern}}
```

예제에서는 `pop`이 `Some(value)`를 반환하는 동안 꺼낸 값을 저장합니다. 스택이
비면 `None`이 되어 반복이 끝나므로 별도의 종료 조건이 필요하지 않습니다.
