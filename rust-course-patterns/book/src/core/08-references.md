# 패턴에서 값과 참조 다루기

패턴으로 `String` 같은 값을 꺼낼 때는 그 값을 **옮길지**, **빌릴지** 먼저
정해야 합니다. 판단 기준은 `match` 뒤에 놓인 값의 타입입니다.

- `Option<String>`을 매칭하면 기본적으로 내부 `String`을 옮깁니다.
- `&Option<String>`을 매칭하면 내부 `String`을 빌립니다.
- `&mut Option<String>`을 매칭하면 내부 `String`을 변경 가능하게 빌립니다.

이 세 경우를 구분하면 `ref`, `&` 패턴, 매치 참조 편의 문법을 한꺼번에 외울
필요가 없습니다.

## 소유한 값을 매칭하기

`name`의 타입이 `Option<String>`일 때 `Some(value)`로 매칭하면 내부
`String`이 `value`로 이동합니다. 이동한 뒤에는 원래의 `name`을 다시 사용할 수
없습니다.

```rust
{{#rustdoc ../../../examples/08_references.rs body=move_value}}
```

원본을 계속 사용하려면 바인딩 앞에 `ref`를 붙여 내부 값만 빌립니다.
`Some(ref value)`에서 `value`의 타입은 `&String`입니다.

```rust
{{#rustdoc ../../../examples/08_references.rs body=match_with_ref}}
```

여기서 중요한 점은 `match name`이라고 썼다고 해서 내부 값이 반드시 이동하지는
않는다는 것입니다. 실제로 이동할지 빌릴지는 `Some(value)`와
`Some(ref value)` 가운데 어떤 패턴을 썼는지가 결정합니다.

## 참조를 매칭하기

원본 전체를 빌린 뒤 매칭하는 방법도 있습니다. 다음 코드에서 매칭 대상은
`&name`이므로 타입이 `&Option<String>`입니다.

```rust
{{#rustdoc ../../../examples/08_references.rs body=borrow}}
```

패턴에는 바깥 참조를 분해하는 `&`나 내부 값을 빌리는 `ref`가 보이지 않습니다.
Rust가 참조를 따라가며 패턴을 맞추고, `value`를 `&String`으로 바인딩하기
때문입니다. 이를 매치 참조 편의 문법<sub>match ergonomics</sub>이라고 합니다.

따라서 다음 두 선택은 같은 결과를 냅니다.

| 매칭 대상 | 패턴 | `value`의 타입 |
|---|---|---|
| `name` | `Some(ref value)` | `&String` |
| `&name` | `Some(value)` | `&String` |

소유한 값을 매칭하면서 필요한 부분만 빌리려면 첫 번째 형태를 쓰고, 원본 전체를
빌려서 살펴보려면 두 번째 형태를 쓰면 됩니다. 보통은 두 번째 형태가 더 간결합니다.

## 생략된 참조를 모두 적어 보기

편의 문법이 없다면 `&Option<String>`의 바깥 참조를 패턴의 `&`로 분해하고,
내부 `String`은 `ref`로 빌려야 합니다.

```rust
{{#rustdoc ../../../examples/08_references.rs body=explicit_match}}
```

편의 문법은 이 코드의 `&Some(ref value)`를 `Some(value)`로, `&None`을
`None`으로 줄여 줍니다. 이 완전한 형태는 생략되는 과정을 이해할 때 유용하지만,
일반적인 코드에서는 앞 절의 간결한 형태를 쓰면 됩니다.

표현식과 패턴에서 `&`의 방향이 반대라는 점도 기억해 두세요. 표현식의 `&number`는
참조를 만들고, 패턴의 `&copied`는 참조를 분해합니다.

```rust
{{#rustdoc ../../../examples/08_references.rs body=explicit_reference}}
```

여기서 `copied`는 `&i32`가 아니라 `i32`입니다. `i32`는 `Copy` 트레이트를
구현하므로 참조가 가리키는 값을 복사할 수 있습니다.

## 빌린 내부 값 변경하기

읽기만 할 때와 같은 기준을 가변 참조에도 적용할 수 있습니다.
`&mut Option<String>`을 매칭하면 `text`는 `&mut String`이 됩니다.

```rust
{{#rustdoc ../../../examples/08_references.rs body=mutable}}
```

소유한 `Option<String>`을 직접 매칭한다면 `ref mut`로 내부 값만 변경 가능하게
빌릴 수 있습니다.

```rust
{{#rustdoc ../../../examples/08_references.rs body=ref_mut}}
```

두 형태의 관계는 공유 참조를 사용할 때와 같습니다.

| 매칭 대상 | 패턴 | `text`의 타입 |
|---|---|---|
| `message` | `Some(ref mut text)` | `&mut String` |
| `&mut message` | `Some(text)` | `&mut String` |

## `Copy` 트레이트를 구현한 타입은 복사됨

값으로 바인딩한다고 언제나 원본을 사용할 수 없게 되는 것은 아닙니다. `i32`처럼
`Copy` 트레이트를 구현한 타입은 바인딩할 때 복사됩니다.

```rust
{{#rustdoc ../../../examples/08_references.rs body=copy_value}}
```

`Some(value)`라는 같은 패턴이라도 `Option<String>`에서는 내부 값이 이동하고,
`Option<i32>`에서는 내부 값이 복사됩니다. 패턴만 보지 말고 바인딩되는 값의 타입도
함께 확인해야 합니다.

## Rust 2024에서 바인딩 방식 명시하기

이 강의는 `edition = "2024"`를 사용합니다. `&name`에 `Some(value)`를 맞추면
편의 문법에 따라 이미 참조 바인딩이 적용됩니다. 이 안에 `ref`를 다시 적은
`Some(ref value)`는 허용되지 않습니다. `ref`를 직접 쓰고 싶다면 매칭 대상도
`name`으로 바꿔 `Some(ref value)`와 조합해야 합니다.

가변 참조도 마찬가지입니다. 다음 두 조합 가운데 하나를 선택합니다.

- `message`와 `Some(ref mut text)`
- `&mut message`와 `Some(text)`

[공식 에디션 가이드](https://doc.rust-lang.org/edition-guide/rust-2024/match-ergonomics.html)에서
Rust 2024가 명시적인 바인딩 방식을 제한하는 규칙과 이전 에디션의 차이를 확인할
수 있습니다.

패턴을 작성할 때는 먼저 값을 소비할지, 읽기만 할지, 변경할지 정하세요. 그다음
매칭 대상을 값, 공유 참조, 가변 참조 가운데 하나로 고르면 바인딩 타입을 예측하기
쉬워집니다.
