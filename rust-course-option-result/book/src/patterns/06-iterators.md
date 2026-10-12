# 반복자와 `Option`

반복자<sub>iterator</sub>는 값을 하나씩 꺼내 처리합니다. `.iter()`는 원본을 빌리고,
`.into_iter()`는 값을 넘겨줍니다. `map`은 각 항목을 바꾸고 `collect`는 결과를 모읍니다.
이 장에서는 메서드를 이어 쓰는 방법을 익히고, 자세한 원리는 뒤의 강의에서 다룹니다.

`find`는 조건에 맞는 첫 번째 값을, `position`은 조건에 맞는 첫 번째 값의 위치를
`Option`으로 반환합니다.

```rust
{{#rustdoc ../../../examples/06_iterators.rs body=search}}
```

`collect::<Option<Vec<_>>>()`는 모든 항목이 `Some`일 때만 값을 하나의 `Vec`로 모아
반환합니다. 항목 중 하나라도 `None`이면 결과는 `None`입니다.

```rust
{{#rustdoc ../../../examples/06_iterators.rs body=collect}}
```

## None에서 처리가 멈추는지 확인하기

`inspect`를 쓰면 반복자가 각 항목을 처리할 때 그 값을 확인할 수 있습니다.
아래 예제에서 `visited`에는 `None` 뒤의 항목이 기록되지 않습니다.
`None`을 만나기 전에 모은 값도 최종 결과에는 포함되지 않습니다.

```rust
{{#rustdoc ../../../examples/06_iterators.rs body=stop_at_none}}
```
