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

---
level: 2
---

# 컴파일 성공은 동작을 확인하지 않습니다

`cargo check`는 타입, 소유권과 문법 규칙을 만족하는지 확인합니다. 계산 결과와 오류
처리가 요구 사항에 맞는지는 테스트가 별도로 확인해야 합니다.

<div class="command">cargo test</div>

`#[test]`가 붙은 함수를 찾아 컴파일하고 실행합니다.

---
level: 2
---

# assertion은 실제 결과와 기대를 비교합니다

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

---
level: 2
---

# 테스트 이름에는 확인하는 동작을 적습니다

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

---
level: 2
---

# 필요한 테스트만 골라 빠르게 반복합니다

```bash
cargo test --example 05_testing
cargo test divides
cargo test returns_none
cargo test -- --no-capture
```

`--` 뒤에는 테스트 프로그램이 받을 옵션을 적습니다. `--no-capture`는 테스트가
출력한 내용을 숨기지 않습니다.

---
level: 2
---

# 단위 테스트는 같은 crate의 내부 규칙을 봅니다

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

---
level: 2
---

# 통합 테스트는 외부 사용자의 위치에서 봅니다

`tests/*.rs`는 각각 별도의 crate로 컴파일되므로 공개 API만 사용할 수 있습니다.

```rust {1|3-8}{lines:true}
use rust_course_tooling::normalize_names;

#[test]
fn trims_and_lowercases_names() {
    assert_eq!(normalize_names(&[" Ferris ", " RUST "]), ["ferris", "rust"]);
}
```

<div class="command">cargo test --test exercises</div>

---
level: 2
---

# 위치는 테스트가 지켜야 할 약속으로 정합니다

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
    <div class="crate-label">library crate</div>
    <div class="crate-code">
      <div class="private-item"><code>fn normalized_name</code><span>비공개 구현</span></div>
      <div class="public-item"><code>pub fn normalize_names</code><span>공개 동작</span></div>
    </div>
    <div class="unit-zone">
      <strong>단위 테스트</strong>
      <span>같은 crate 안에서 내부 규칙까지 검사</span>
    </div>
  </div>
</div>

<div class="takeaway">구현 세부를 지나치게 고정하면 안전한 리팩터링에도 테스트가 깨집니다.</div>

---
level: 2
---

# 독립된 요구 사항은 테스트에서도 따로 드러냅니다

`normalize_names`라면 다음 입력을 각각 확인할 수 있습니다.

- 앞뒤에 공백이 있는 이름
- 빈 문자열과 공백뿐인 문자열
- 대문자와 소문자가 섞인 이름
- 여러 이름의 순서

<div class="question">테스트가 실패했을 때 어느 요구 사항이 깨졌는지 이름만 보고 알 수 있나요?</div>

---
level: 2
---

# 테스트는 빠른 반복과 전체 확인을 함께 씁니다

<div class="tool-flow">
  <div><strong>작성 중</strong>이름 필터로 관련 테스트만 실행합니다.</div>
  <div><strong>변경 뒤</strong>해당 타깃과 통합 테스트를 실행합니다.</div>
  <div><strong>완료 전</strong>지원 feature의 전체 테스트를 실행합니다.</div>
</div>

---
level: 2
layout: center
class: section
---

# 테스트 위치는<br>무엇을 약속하는지 보여 줍니다

다음 편에서는 오류, panic, 제외한 테스트와 문서 테스트를 다룹니다.
