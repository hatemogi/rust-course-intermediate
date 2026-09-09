# 모듈 구성 요소의 공개 범위

Rust의 항목은 기본적으로 비공개입니다. 모듈<sub>module</sub> 바깥에서도 항목에
접근할 수 있게 하려면 `pub` 공개 범위 문법을 사용합니다.

## 기본 비공개 규칙 이해하기

| 문법 | 접근할 수 있는 범위 |
|---|---|
| `pub` 없음 | 항목을 선언한 현재 모듈과 그 자식 모듈에서 접근할 수 있습니다. |
| `pub` | 공개된 상위 경로를 통해 접근할 수 있는 모든 곳에서 접근할 수 있습니다. |

비공개 항목은 같은 모듈과 그 아래 자식 모듈에서 사용할 수 있습니다. 따라서 자식
모듈은 부모 모듈의 비공개 항목을 사용할 수 있지만, 부모 모듈이나 형제 모듈은 자식
모듈의 비공개 항목을 사용할 수 없습니다.

## 자식 모듈에서 부모의 비공개 항목 사용하기

다음 코드는 `service` 모듈과 그 안에 있는 항목을 함께 보여줍니다.

```rust
{{#include ../../../examples/03_privacy.rs:service_open}}
{{#include ../../../examples/03_privacy.rs:child_access}}
{{#include ../../../examples/03_privacy.rs:service_close}}
```

`api::key_length`는 자식 모듈에 있으므로 부모인 `service`의 비공개 함수
`private_key`를 사용할 수 있습니다.

## 공개 경로 끝까지 열기

외부 크레이트에서 어떤 항목에 접근하려면 크레이트 루트부터 그 항목까지 이어지는
모든 모듈이 공개되어야 합니다. 위 예제에서 `service::api::key_length` 경로를
외부에 공개하려면 `service`, `api`, `key_length`에 모두 `pub`이 필요합니다.

## 비공개 모듈에 공개 함수로 접근하기

다음 코드도 `service` 모듈 선언과 함께 살펴봅니다.

```rust
{{#include ../../../examples/03_privacy.rs:service_open}}
{{#include ../../../examples/03_privacy.rs:private_path}}
{{#include ../../../examples/03_privacy.rs:service_close}}
```

`internal` 모듈 안의 `trace_id` 함수에 `pub`이 붙어 있어도 `internal` 모듈 자체가
비공개이므로 `service::internal::trace_id` 경로는 바깥에서 사용할 수 없습니다.
`service` 안에서는 `internal::trace_id`를 호출해 공개 함수의 구현에 사용할 수
있습니다.

`pub`은 항목이 놓인 경로의 비공개 규칙을 건너뛰지 않습니다. 내부 구현 모듈은
비공개로 유지하면서 그 안의 항목만 다른 경로로 공개하려면 뒤에서 배울 재공개
`pub use`를 사용합니다.

## 크레이트 안에서 범위 제한하기

| 문법 | 접근할 수 있는 범위 |
|---|---|
| `pub` | 상위 경로가 모두 공개되어 있다면 외부 크레이트에서도 접근할 수 있습니다. |
| `pub(self)` | 현재 모듈과 그 자식 모듈에서 접근할 수 있습니다. `pub`을 쓰지 않은 경우와 같습니다. |
| `pub(crate)` | 현재 크레이트<sub>crate</sub> 안에서 접근할 수 있습니다. |
| `pub(super)` | 부모 모듈 안에서 접근할 수 있습니다. |
| `pub(in crate::경로)` | 지정한 조상 모듈과 그 자식 모듈에서 접근할 수 있습니다. |

일반 `pub`은 현재 크레이트 안으로 범위를 제한하지 않습니다. 다만 항목에 `pub`을
붙였더라도 그 항목으로 이어지는 상위 모듈 가운데 하나가 비공개라면 외부
크레이트에서 해당 경로를 사용할 수 없습니다.

외부 API가 아닌 크레이트 내부 협력에는 `pub(crate)`나 `pub(super)`처럼 제한된
공개 범위를 고려합니다. `pub(in crate::경로)`의 경로는 항목을 감싸는 조상
모듈이어야 합니다. 형제 모듈이나 자식 모듈을 지정해 공개 범위를 옆이나 아래로
넓힐 수는 없습니다.

## 최소한의 범위로 공개하기

```rust
{{#include ../../../examples/03_privacy.rs:restricted_visibility}}
```

`parser::field_count`는 `pub(in crate::jobs)`이므로 `jobs` 모듈 안에서는 사용할 수
있지만 크레이트 루트에서는 직접 호출할 수 없습니다. `jobs::field_count`는
`pub(crate)`이므로 현재 크레이트의 다른 모듈에서도 호출할 수 있고, 외부
크레이트에는 공개되지 않습니다.

공개 범위는 나중에 좁히면 기존 사용자 코드가 컴파일되지 않을 수 있는 API의
일부입니다. 처음부터 가장 넓게 열기보다 실제로 접근해야 하는 범위까지만 여는
편이 좋습니다.

## 비공개 필드로 규칙 지키기

```rust
{{#include ../../../examples/03_privacy.rs:account}}
```

구조체 자체가 `pub`이어도 필드는 각각 따로 공개해야 합니다. 필드를 비공개로 두고
생성자와 메서드를 통해 접근시키면 타입이 지켜야 할 규칙을 한곳에서 확인할 수
있습니다. 이처럼 내부 상태를 감추고 정해진 메서드로만 다루게 하는 방식을
캡슐화<sub>encapsulation</sub>라고 합니다.

```rust
{{#include ../../../examples/03_privacy.rs:use}}
```

## 열거형의 배리언트 공개 범위

구조체와 달리 열거형<sub>enum</sub>은 배리언트<sub>variant</sub>마다 공개 범위를
따로 정하지 않습니다. 열거형을 `pub`으로 선언하면 모든 배리언트와 각 배리언트의
필드도 열거형과 같은 범위에 공개됩니다. 배리언트나 그 필드 앞에는 `pub`을 붙일 수
없습니다.

```rust
{{#include ../../../examples/03_privacy.rs:enum_visibility}}
```

`TaskState`가 공개되어 있으므로 `task` 모듈 밖에서도 `Ready`, `Running`, `Failed`를
생성하고 패턴 매칭할 수 있습니다. 구조체의 필드와 달리 `Running`의 `worker`나
`Failed`의 문자열만 비공개로 만들 수는 없습니다.

## 공개 열거형 사용하기

공개 열거형은 모듈 밖에서 모든 배리언트를 생성할 수 있습니다.

```rust
{{#include ../../../examples/03_privacy.rs:enum_construct}}
```

패턴 매칭할 때는 배리언트의 필드도 꺼내서 사용할 수 있습니다.

```rust
{{#include ../../../examples/03_privacy.rs:enum_match}}
```

열거형 자체를 비공개로 선언하면 모든 배리언트도 모듈 밖에서 사용할 수 없습니다.
공개 범위를 크레이트 안으로 제한하려면 열거형 선언에 `pub(crate) enum`처럼
지정합니다. 이때 배리언트도 같은 범위로 제한됩니다.
