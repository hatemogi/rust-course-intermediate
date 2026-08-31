# 종합 실습

이 실습에서는 이미 작성된 `model`, `parser`, `report` 구현을 하나의 라이브러리
크레이트<sub>crate</sub>로 연결합니다. 학습자가 수정할 파일은 크레이트 루트인
`src/lib.rs` 하나입니다. `src/model.rs`, `src/parser.rs`, `src/report.rs`, `tests`는
수정하지 않습니다.

실습 시작 상태의 `src/lib.rs`에는 모듈 연결과 재공개 선언이 비어 있습니다.
따라서 코드를 작성하기 전에 `cargo test`가 컴파일 오류로 실패하는 것이
정상입니다. 아래 작업을 차례로 마치면 실패 원인이 줄어들고 마지막에는 모든
테스트가 통과합니다.

## 해야 할 일

`src/lib.rs`에서 다음 선언을 작성하세요.

1. `model.rs`, `parser.rs`, `report.rs`를 각각 `model`, `parser`, `report` 모듈로
   연결합니다. 세 모듈은 외부 사용자가 직접 접근하지 못하도록 비공개로 둡니다.
2. `Task`, `TaskStatus`를 크레이트 루트에 재공개합니다.
3. `ParseTaskError`, `parse_task`를 크레이트 루트에 재공개합니다.
4. `render_report`를 크레이트 루트에 재공개합니다.

모듈 선언에 `pub`을 붙여 내부 구조를 그대로 공개하면 안 됩니다. 외부 사용자는
`rust_course_modules_crates::Task`처럼 필요한 이름을 크레이트 루트에서 바로
가져올 수 있어야 합니다.

## 완료 조건

다음 명령으로 결과를 확인합니다.

```bash
cargo test
```

테스트는 다음 조건을 각각 판별합니다.

- `Task`, `TaskStatus`를 크레이트 루트에서 가져올 수 있습니다.
- `ParseTaskError`, `parse_task`를 크레이트 루트에서 가져올 수 있습니다.
- `render_report`를 크레이트 루트에서 가져올 수 있습니다.
- `model`, `parser`, `report` 모듈에는 크레이트 외부에서 직접 접근할 수 없습니다.

## 테스트 실패 해석하기

모듈 선언이 빠지면 해당 파일을 찾지 못하거나 재공개 경로를 해석하지 못했다는
컴파일 오류가 발생합니다. 재공개가 빠지면 테스트 코드의 `use` 선언에서 어떤
이름을 찾지 못했는지 확인할 수 있습니다. 모듈에 `pub`을 붙이면 비공개 경로를
검사하는 컴파일 실패 테스트가 실패합니다.

## 공개 API 사용하기

다음 예제는 내부 모듈 경로를 사용하지 않고 크레이트의 공개 API만 호출합니다.

```rust
{{#include ../../../examples/05_exercises.rs:exercise}}
```
