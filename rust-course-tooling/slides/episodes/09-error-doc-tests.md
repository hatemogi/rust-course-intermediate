---
theme: default
title: 오류 테스트와 문서 테스트
info: 우아한 Rust 중급 개발 도구 9편
colorSchema: dark
aspectRatio: 16/9
canvasWidth: 1100
transition: fade-out
lineNumbers: true
layout: cover
class: cover
---

<div class="episode-label">우아한 Rust 중급 · 개발 도구 09</div>

# 오류 테스트와<br>문서 테스트

실패 경로와 공개 API의 사용 예제를 실행 가능한 약속으로 만듭니다.

---
level: 2
---

# 성공 경로만으로는 오류 계약을 알 수 없습니다

실패 가능한 함수는 다음 질문도 테스트해야 합니다.

- 어떤 입력에서 실패하는가?
- `Result`, `Option`, panic 가운데 무엇으로 알리는가?
- 오류의 종류와 정보가 호출자에게 충분한가?
- 실패 과정에서 일부 상태만 바뀌지는 않는가?

---
level: 2
---

# 테스트 함수도 `Result`를 반환할 수 있습니다

```rust
#[test]
fn parses_a_port() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!("443".parse::<u16>()?, 443);
    Ok(())
}
```

준비 과정의 오류를 `?`로 전달하기 편리합니다. 그러나 **오류 발생 자체가 요구
사항**이라면 바로 끝내지 말고 오류의 종류와 내용을 명시적으로 검사합니다.

---
level: 2
---

# panic이 계약일 때만 `should_panic`을 씁니다

```rust
#[test]
#[should_panic(expected = "index out of bounds")]
fn panics_when_the_slice_is_empty() {
    let values: Vec<i32> = Vec::new();
    let _ = values[0];
}
```

`expected`에 메시지 일부를 적으면 엉뚱한 panic으로 테스트가 통과하는 일을 줄일 수
있습니다.

<div class="takeaway">예상 가능한 사용자 입력은 panic보다 <code>Result</code>가 알맞은 경우가 많습니다.</div>

---
level: 2
---

# 제외한 테스트에는 실행 조건이 있어야 합니다

```bash
cargo test -- --ignored
cargo test -- --include-ignored
```

오래 걸리거나 특별한 외부 환경이 필요한 테스트에는 `#[ignore]`를 붙일 수 있습니다.
불규칙하게 실패하는 테스트를 숨기는 용도로 사용해서는 안 됩니다.

```rust {1-2|3-6}{lines:true}
#[test]
#[ignore = "큰 입력을 직접 확인할 때만 실행합니다"]
fn checks_many_ports() {
    for port in 0..=u16::MAX {
        assert_eq!(parse_port(&port.to_string()).unwrap(), port);
    }
}
```

---
level: 2
---

# 문서의 Rust 코드도 테스트할 수 있습니다

```rust
/// ```
/// use rust_course_tooling::normalize_names;
///
/// let names = normalize_names(&[" Ferris ", " RUST "]);
/// assert_eq!(names, ["ferris", "rust"]);
/// ```
pub fn normalize_names(names: &[&str]) -> Vec<String> {
    // ...
}
```

<div class="takeaway">문서 테스트는 공개 API의 대표 사용법이 실제 코드와 달라지는 일을 줄입니다.</div>

---
level: 2
---

# 코드 블록 속성은 실행 약속을 정합니다

<table class="compare">
  <thead><tr><th>속성</th><th>문서 테스트의 동작</th></tr></thead>
  <tbody>
    <tr><td>속성 없음</td><td>컴파일하고 실행</td></tr>
    <tr><td><code>no_run</code></td><td>컴파일하지만 실행하지 않음</td></tr>
    <tr><td><code>compile_fail</code></td><td>컴파일이 실패해야 통과</td></tr>
    <tr><td><code>ignore</code></td><td>문서 테스트에서 제외</td></tr>
  </tbody>
</table>

---
level: 2
---

# 금지된 사용법도 실행 가능한 계약이 됩니다

```rust
//! ```compile_fail
//! let message = String::from("완료");
//! drop(message);
//! println!("{message}");
//! ```
```

`compile_fail`은 소유권이나 타입 제약처럼 허용되지 않는 사용법을 보여 줄 때
유용합니다. 우연히 컴파일되지 않는 내부 구현을 고정하는 데 쓰지 않습니다.

---
level: 2
---

# 문서 테스트만 따로 실행할 수 있습니다

```bash
cargo test --doc --no-default-features --locked
cargo test --doc --all-features --locked
```

문서에는 대표적인 공개 사용법을 두고, 경계 조건과 오류 조합은 단위·통합
테스트에서 자세히 검사합니다.

<div class="takeaway">문서 테스트는 일반 테스트를 대신하지 않고 사용자의 첫 경로를 지킵니다.</div>

---
level: 2
---

# 서로 다른 테스트가 다른 약속을 지킵니다

<div class="tool-flow">
  <div><strong>단위 테스트</strong>작은 내부 규칙과 계산</div>
  <div><strong>통합 테스트</strong>외부에서 보이는 공개 동작</div>
  <div><strong>문서 테스트</strong>대표 사용법과 금지된 사용법</div>
</div>

오래 걸리는 검사와 플랫폼별 검사는 실행 조건을 분명히 하고 전체 검증 흐름에
배치합니다.

---
level: 2
layout: center
class: section
---

# 실패 경로도 API이고<br>문서의 예제도 코드입니다

다음 편에서는 올바른 두 구현을 공정하게 측정하는 방법을 살펴봅니다.
