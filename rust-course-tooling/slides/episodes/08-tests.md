---
theme: default
title: 단위 테스트와 통합 테스트
info: 우아한 Rust 중급 개발 도구 8편
colorSchema: dark
aspectRatio: 16/9
canvasWidth: 1100
transition: fade-out
lineNumbers: true
layout: cover
class: cover
---

<div class="episode-label">우아한 Rust 중급 · 개발 도구 08</div>

# 단위 테스트와<br>통합 테스트

구현 세부와 공개 동작을 서로 알맞은 위치에서 검사합니다.

<!--
교재 대응: book/src/testing/16-unit-integration.md > 단위 테스트와 통합 테스트

Rust에서는 테스트의 위치에 따라 접근할 수 있는 코드와 컴파일 방식이 달라집니다.
-->

---
level: 2
---

# `cargo test`와 기본 assertion

`cargo check`는 타입·소유권·문법 규칙을 만족하는지 확인합니다. 계산 결과와 오류
처리가 요구 사항에 맞는지는 테스트가 별도로 확인해야 합니다.

<div class="command">cargo test</div>

`#[test]`가 붙은 함수를 찾아 컴파일하고 실행합니다.

<!--
교재 대응: book/src/testing/15-cargo-test.md > cargo test와 기본 assertion

cargo test는 테스트에 필요한 코드를 컴파일하고, 그중 #[test]가 붙은 함수를 각각 하나의 테스트로 실행합니다. 테스트 함수 안에서는 assertion
macro로 실제 결과와 기대한 결과를 비교합니다. • assert!(condition)은 조건이 true인지 확인합니다. • assert_eq!(left,
right)는 두 값이 같은지 확인합니다. • assert_ne!(left, right)는 두 값이 다른지 확인합니다.
-->

---
level: 2
---

# `cargo test`와 기본 assertion

- `assert!(condition)`: 조건이 `true`인지 확인합니다.
- `assert_eq!(left, right)`: 두 값이 같은지 확인합니다.
- `assert_ne!(left, right)`: 두 값이 다른지 확인합니다.

```rust {1-5|7-11}{lines:true}
#[test]
fn divides_two_integers() {
    assert_eq!(checked_divide(12, 3), Some(4));
    assert_ne!(checked_divide(12, 3), Some(5));
}

#[test]
fn returns_none_when_divisor_is_zero() {
    assert!(checked_divide(12, 0).is_none());
}
```

<!--
교재 대응: book/src/testing/15-cargo-test.md > cargo test와 기본 assertion

cargo test는 테스트에 필요한 코드를 컴파일하고, 그중 #[test]가 붙은 함수를 각각 하나의 테스트로 실행합니다. 테스트 함수 안에서는 assertion
macro로 실제 결과와 기대한 결과를 비교합니다. • assert!(condition)은 조건이 true인지 확인합니다. • assert_eq!(left,
right)는 두 값이 같은지 확인합니다. • assert_ne!(left, right)는 두 값이 다른지 확인합니다.
-->

---
level: 2
---

# 테스트 이름 짓기

<div class="grid grid-cols-2 gap-10 mt-12">
  <div>
    <h3>피하고 싶은 이름</h3>
    <code>test_checked_divide</code>
    <p class="muted">무엇이 깨졌는지 드러나지 않습니다.</p>
  </div>
  <div>
    <h3>동작을 보여 주는 이름</h3>
    <code>returns_none_when_divisor_is_zero</code>
    <p class="muted">실패한 요구 사항을 바로 알 수 있습니다.</p>
  </div>
</div>

<!--
교재 대응: book/src/testing/15-cargo-test.md > 테스트 이름 짓기

테스트 이름에는 구현 방법보다 확인하려는 동작을 적으세요. test_checked_divide보다 returns_none_when_divisor_is_zero가 실패
원인을 더 잘 알려 줍니다.
-->

---
level: 2
---

# 테스트 프로그램에 옵션 전달하기

```bash
cargo test --example 15_testing
cargo test --example 15_testing divides
cargo test --example 15_testing returns_none
cargo test -- --no-capture
```

`--` 뒤에는 테스트 프로그램이 받을 옵션을 적습니다. `--no-capture`는 테스트가
출력한 내용을 숨기지 않습니다.

<!--
교재 대응: book/src/testing/15-cargo-test.md > 테스트 골라서 실행하기; book/src/testing/15-cargo-test.md > 테스트 프로그램에 옵션 전달하기

cargo test에 테스트 이름을 문자열로 전달하면, 선택한 테스트 대상에서 이름에 그 문자열이 들어간 테스트만 실행합니다. 예를 들어 divides와
returns_none으로 필요한 테스트만 골라 실행할 수 있습니다. 테스트 프로그램에 옵션을 전달할 때는 --로 카고(Cargo) 옵션과 구분합니다. 테스트가
통과했을 때도 표준 출력(stdout)과 표준 오류(stderr)에 내보낸 내용을 숨기지 않으려면 --no-capture를 사용합니다.
-->

---
level: 2
---

# 단위 테스트

```rust
#[cfg(test)]
mod tests {
    use super::normalized_name;

    #[test]
    fn normalizes_a_non_empty_name() {
        assert_eq!(normalized_name(" Ferris "), Some("ferris".to_owned()));
    }
}
```

- `cfg(test)` 모듈은 테스트할 때만 컴파일됩니다.
- 자식 모듈이므로 `super`를 통해 비공개 함수도 검사할 수 있습니다.

<!--
교재 대응: book/src/testing/16-unit-integration.md > 단위 테스트

단위 테스트는 보통 검사할 코드와 같은 파일의 #[cfg(test)] mod tests 안에 둡니다. cfg(test)가 붙은 모듈은 테스트할 때만 컴파일됩니다. 자식
모듈이므로 super를 통해 비공개 함수도 검사할 수 있습니다. 이 강의의 src/lib.rs에는 이름 하나를 정돈하는 비공개 함수를 검사하는 단위 테스트가 있습니다.
단위 테스트는 작은 계산이나 공개 API를 구성하는 내부 규칙을 빠르게 확인하는 데 알맞습니다. 모든 비공개 함수를 각각 검사해야 한다는 뜻은 아닙니다. 공개 API를
사용해 충분히 확인되는 구현 세부 사항까지 테스트하면 리팩터링할 때 테스트가 불필요하게 깨질 수 있습니다.
-->

---
level: 2
---

# 통합 테스트

`tests/*.rs`는 각각 별도의 크레이트<sub>crate</sub>로 컴파일되므로 공개 API만
사용할 수 있습니다.

```rust {1|3-8}{lines:true}
use rust_course_tooling::normalize_names;

#[test]
fn trims_and_lowercases_names() {
    assert_eq!(normalize_names(&[" Ferris ", " RUST "]), ["ferris", "rust"]);
}
```

<div class="command">cargo test --test exercises</div>

<!--
교재 대응: book/src/testing/16-unit-integration.md > 통합 테스트; book/src/testing/16-unit-integration.md > 어디에 둘지 결정하기

프로젝트 루트의 tests 디렉터리에 둔 Rust 파일은 각각 별도의 crate로 컴파일됩니다. 따라서 외부 사용자가 라이브러리를 쓰는 것처럼 공개 API만 사용할 수
있습니다. 다음 명령은 이름이 exercises인 통합 테스트 타깃만 실행합니다. 작은 내부 규칙을 직접 확인해야 한다면 같은 파일의 단위 테스트를 사용합니다. 공개 API
여러 개를 조합한 사용법을 확인한다면 통합 테스트를 사용합니다. 버그가 공개 API로 재현된다면 구현 세부 사항보다 공개 동작을 검사하는 편이 리팩터링에 강합니다.
-->

---
level: 2
---

# 단위 테스트와 통합 테스트

<div class="crate-boundary-visual">
  <div class="integration-zone">
    <strong>통합 테스트</strong>
    <code>tests/*.rs</code>
    <span>외부 사용자의 위치</span>
  </div>

  <div class="public-call">
    <span>공개 API 호출</span>
    <b>→</b>
  </div>

  <div class="crate-shell">
    <div class="crate-label">라이브러리 크레이트</div>
    <div class="crate-code">
      <div class="private-item"><code>fn normalized_name</code><span>비공개 구현</span></div>
      <div class="public-item"><code>pub fn normalize_names</code><span>공개 동작</span></div>
    </div>
    <div class="unit-zone">
      <strong>단위 테스트</strong>
      <span>같은 크레이트 안에서 내부 규칙까지 검사</span>
    </div>
  </div>
</div>

<div class="takeaway">구현 세부를 지나치게 고정하면 안전한 리팩터링에도 테스트가 깨집니다.</div>

<!--
교재 대응: book/src/testing/16-unit-integration.md > 단위 테스트와 통합 테스트

Rust에서는 테스트의 위치에 따라 접근할 수 있는 코드와 컴파일 방식이 달라집니다.
-->

---
level: 2
---

# 테스트 이름 짓기

`normalize_names`라면 다음 입력을 각각 확인할 수 있습니다.

- 앞뒤에 공백이 있는 이름
- 빈 문자열과 공백뿐인 문자열
- 대문자와 소문자가 섞인 이름
- 여러 이름의 순서

<div class="question">테스트가 실패했을 때 어느 요구 사항이 깨졌는지 이름만 보고 알 수 있나요?</div>

<!--
교재 대응: book/src/testing/15-cargo-test.md > 테스트 이름 짓기

테스트 이름에는 구현 방법보다 확인하려는 동작을 적으세요. test_checked_divide보다 returns_none_when_divisor_is_zero가 실패
원인을 더 잘 알려 줍니다.
-->

---
level: 2
---

# 통합 테스트

<div class="tool-flow">
  <div><strong>작성 중</strong>이름 필터로 관련 테스트만 실행합니다.</div>
  <div><strong>변경 뒤</strong>해당 타깃과 통합 테스트를 실행합니다.</div>
  <div><strong>완료 전</strong>지원하는 피처<sub>feature</sub>의 전체 테스트를 실행합니다.</div>
</div>

<!--
교재 대응: book/src/testing/16-unit-integration.md > 통합 테스트

프로젝트 루트의 tests 디렉터리에 둔 Rust 파일은 각각 별도의 crate로 컴파일됩니다. 따라서 외부 사용자가 라이브러리를 쓰는 것처럼 공개 API만 사용할 수
있습니다. 다음 명령은 이름이 exercises인 통합 테스트 타깃만 실행합니다.
-->
