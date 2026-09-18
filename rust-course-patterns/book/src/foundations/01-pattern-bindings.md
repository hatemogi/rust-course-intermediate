# 패턴과 바인딩

패턴<sub>pattern</sub>은 값의 모양을 확인하고 필요한 부분에 이름을 붙입니다.
변수를 선언할 때도 패턴을 쓸 수 있습니다.

```rust
{{#rustdoc ../../../examples/01_bindings.rs body=tuple_binding}}
```

왼쪽의 `(x, y)`가 오른쪽 튜플의 구조에 매칭<sub>matching</sub>되며 두 원소에 각각
이름을 붙입니다. 이처럼 내부 값에 이름을 붙이는 일을 바인딩<sub>binding</sub>이라고
합니다. `x`와 `y`의 타입은 대응하는 튜플 원소의 타입에서 정해집니다.

함수 매개변수와 `for` 루프에서도 구조 분해<sub>destructuring</sub>할 수 있습니다.
반복할 값의 각 원소가 같은 튜플 모양이므로 루프가 돌 때마다 `name`을 바로 사용할
수 있습니다.

```rust
{{#rustdoc ../../../examples/01_bindings.rs body=loop_binding}}
```

## `_`와 `_name`의 차이

`_`는 값을 사용하지 않고 무시합니다. `_name`은 값에 이름을 붙이므로 소유권
이동이 일어날 수 있지만, `_`는 이름을 붙이지 않는다는 차이가 있습니다.

```rust
{{#rustdoc ../../../examples/01_bindings.rs body=underscore_ignored}}
```

`_`는 내부 `String`을 바인딩하지 않으므로 `kept`를 뒤에서 다시 사용할 수 있습니다.

## `_name`은 실제 바인딩

```rust
{{#rustdoc ../../../examples/01_bindings.rs body=underscore_bound}}
```

이름 앞에 밑줄이 있어도 `_text`는 실제 바인딩이므로 내부 `String`을 이동시킵니다.
그 뒤 `moved` 전체를 다시 읽으면 컴파일 오류가 납니다. 경고만 없애려고
`_name`을 쓰다가 뜻하지 않게 값을 이동시키지 않도록 주의하세요.
