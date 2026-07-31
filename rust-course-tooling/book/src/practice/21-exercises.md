# 종합 실습

지금까지 사용한 도구를 연결해 이름별 횟수를 세는 공개 함수를 완성합니다. 명령을
실행하는 데서 끝내지 않고 구현, 테스트, API 문서, 실행 예제를 결과물로 남깁니다.

다음 `src/lib.rs`를 시작점으로 사용합니다.

```rust
{{#include ../../../src/lib.rs}}
```

## 완성할 기능

`src/lib.rs`에는 다음 공개 함수의 시그니처와 `todo!()`가 준비되어 있습니다.
`todo!()`를 실제 구현으로 바꾸세요.

```rust
pub fn count_normalized_names(names: &[&str]) -> BTreeMap<String, usize> {
    // 아래 todo!()를 제거하고 실제 구현을 작성하세요.
    todo!("주어진 이름별 횟수를 세는 함수를 구현하세요: {names:?}")
}
```

이 함수는 다음 요구 사항을 모두 만족해야 합니다.

- 이름 앞뒤의 공백을 없앱니다.
- 빈 문자열과 공백뿐인 이름을 제외합니다.
- 대소문자를 구분하지 않도록 소문자로 바꿉니다.
- 같은 이름이 여러 번 나오면 횟수를 더합니다.
- 결과를 이름순으로 반환합니다.

예를 들어 `[" Ferris ", "ferris", "", " RUST "]`를 전달하면 다음 결과가
나와야 합니다.

```rust
BTreeMap::from([
    ("ferris".to_owned(), 2),
    ("rust".to_owned(), 1),
])
```

기존 `normalize_names`를 조합해도 되고, 이름을 순회하면서 한 번에 계산해도 됩니다.
두 방식 가운데 요구 사항을 더 분명하게 드러낸다고 판단한 구현을 선택하세요.

## 남겨야 할 결과물

실습을 마치면 Git diff에 다음 변경이 보여야 합니다.

1. `src/lib.rs`: `count_normalized_names`의 공개 구현
2. `tests/exercises.rs`: 섞인 입력과 빈 입력을 확인하는 통합 테스트
3. `src/lib.rs`: 함수의 동작을 설명하고 실행 가능한 예제를 포함한 문서 주석
4. `examples/21_exercises.rs`: 새 함수를 호출하고 이름별 횟수를 출력하는 코드

`tests/exercises.rs`에는 통과해야 할 통합 테스트가 `#[ignore]` 표시와 함께 미리
준비되어 있습니다. 테스트에는 구현끼리 비교한 결과가 아니라 기대 결과가 직접 적혀
있습니다. 구현을 시작할 때 두 테스트의 `#[ignore]`를 제거하세요.

```rust
#[test]
#[ignore]
fn counts_normalized_names() {
    assert_eq!(
        count_normalized_names(&[" Ferris ", "ferris", "", " RUST "]),
        BTreeMap::from([
            ("ferris".to_owned(), 2),
            ("rust".to_owned(), 1),
        ])
    );
}

#[test]
#[ignore]
fn returns_no_name_counts_for_an_empty_input() {
    assert!(count_normalized_names(&[]).is_empty());
}
```

## 작업 순서

1. 두 통합 테스트의 `#[ignore]`를 제거합니다.
2. 통합 테스트를 실행해 `todo!()`에서 아직 구현하지 않았다는 메시지와 함께
   실패하는 것을 확인합니다.
3. `src/lib.rs`의 `todo!()`를 실제 구현으로 바꾸고 테스트를 다시 실행합니다.
4. 공개 함수에 설명과 대표 사용 예제를 문서 주석으로 작성합니다.
5. `examples/21_exercises.rs`에서 새 함수를 호출해 결과를 출력합니다.
6. rustfmt와 클리피<sub>Clippy</sub>의 결과를 읽고 필요한 부분을 수정합니다.
7. 기본 구성, 기본 피처<sub>feature</sub>를 끈 구성, 모든 피처를 켠 구성에서 컴파일과
   테스트를 실행합니다.
8. 문서 테스트를 실행하고 생성한 API 문서에서 새 함수의 설명과 예제를 읽습니다.
9. `git diff`에서 구현, 테스트, 문서, 예제 이외의 변경이 섞이지 않았는지
   확인합니다.

```bash
cargo test --test exercises --locked
cargo run --example 21_exercises
cargo fmt --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo check --no-default-features --locked
cargo check --all-features --locked
cargo test --no-default-features --locked
cargo test --all-features --locked
cargo test --doc --all-features --locked
cargo doc --no-deps --open
git diff
```

## 완료 조건

다음 질문에 모두 “예”라고 답할 수 있으면 실습을 마친 것입니다.

- 섞인 입력과 빈 입력 테스트가 통과하는가?
- 구현이 기존 공개 함수의 동작을 불필요하게 중복하지 않는가?
- 문서의 예제가 문서 테스트에서 실행되는가?
- 실행 예제에서 이름별 횟수를 확인할 수 있는가?
- 기본 피처를 끈 구성과 모든 피처를 켠 구성이 모두 통과하는가?
- rustfmt와 클리피가 지적할 사항이 남아 있지 않은가?

## AI와 함께 검토하기

먼저 직접 구현한 뒤 AI에게 다른 구현을 제안해 달라고 요청하세요. 제안받은 코드는
바로 적용하지 말고 다음 내용을 확인합니다.

- 빈 이름을 제외하고 대소문자를 합치는가?
- 불필요한 `String`을 반복해서 만드는가?
- 기존 `normalize_names`를 재사용했을 때와 직접 순회할 때의 차이를 설명하는가?
- 테스트를 없애거나 린트<sub>lint</sub>를 `allow`로 덮지 않는가?
- 적용 뒤 통합 테스트와 문서 테스트가 그대로 통과하는가?

AI의 제안을 적용했다면 `git diff`로 직접 작성한 구현에서 무엇이 달라졌는지
확인하세요.
