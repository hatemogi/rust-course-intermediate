# 종합 실습

`src/lib.rs`의 이름 정돈과 단어 집계 함수를 대상으로 전체 검증 흐름을 연습합니다.

```rust
{{#include ../../../src/lib.rs}}
```

## 실습 순서

1. `examples/04_exercises.rs`를 실행해 현재 동작을 확인합니다.
2. 테스트에서 빈 입력, 빈 문자열, 공백뿐인 문자열, 대소문자와 입력 순서를 각각
   확인합니다.
3. `cargo fmt --check`와 Clippy를 실행합니다.
4. Clippy 제안을 적용했다면 동작이 달라지지 않았는지 테스트합니다.
5. `cargo tree`를 실행하고 현재 프로젝트의 직접 의존성이 있는지 확인합니다.
6. `examples/07_features.rs`를 기본 구성과 `binary-search` feature를 켠 구성으로
   각각 실행하고 어떤 함수가 컴파일되는지 설명합니다.
7. `cargo doc --no-deps --open`으로 API 문서를 열고 `normalize_names`의 설명과
   예제가 함께 보이는지 확인합니다.
8. 선형 검색과 이진 검색을 서로 비교하는 데서 그치지 않고 기대 결과도 함께
   확인해야 하는 이유를 설명합니다.
9. `cargo bench --bench search`를 실행하고 입력 크기를 바꿔 결과를 비교합니다.
10. 이진 검색을 쓰기 위해 정렬 비용이 필요해지는 상황을 한 가지 적습니다.

```bash
cargo run --example 04_exercises
cargo run --example 07_features
cargo run --example 07_features --features binary-search
cargo tree
cargo test --all-features --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo doc --no-deps --open
cargo bench --bench search --locked
```

## AI와 함께 검토하기

AI에게 `normalize_names`를 더 짧게 고쳐 달라고 요청한 뒤 다음을 확인하세요.

- 빈 이름을 제외하는 동작이 유지되는가?
- 소유한 `String`을 불필요하게 여러 번 만드는가?
- Clippy 경고를 `allow`로 덮었는가?
- 자동 수정을 제안했다면 적용 뒤 diff를 확인했는가?
- 기본 feature와 모든 feature 구성에서 같은 요구 사항을 만족하는가?
- 기존 테스트만으로 변경된 동작을 모두 확인할 수 있는가?
- 두 구현이 우연히 같은 오답을 내도 통과하는 테스트가 남아 있지 않은가?

AI의 답을 채택했다면 어떤 테스트와 lint 결과를 근거로 삼았는지 기록하세요.
