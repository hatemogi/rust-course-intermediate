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

<!--
교재 대응: book/src/testing/17-errors-panic-ignore.md > 오류, panic, 제외한 테스트; book/src/testing/18-documentation-tests.md > 문서 주석과 문서 테스트

Rust의 공개 API에는 /// 문서 주석을 붙일 수 있습니다. 모듈이나 크레이트(crate) 전체를 설명할 때는 //!를 사용합니다. 문서 주석 안에서 백틱 세
개(```)로 Rust 코드 블록을 감쌀 수 있고, cargo test --doc는 이 코드 블록을 별도로 컴파일하고 실행합니다. 따라서 문서의 사용법이 실제 코드와
달라지는 일을 줄일 수 있습니다.
-->

---
level: 2
---

# 오류, panic, 제외한 테스트

실패 가능한 함수는 다음 질문도 테스트해야 합니다.

- 어떤 입력에서 실패하는가?
- `Result`, `Option`, panic 가운데 무엇으로 알리는가?
- 오류의 종류와 정보가 호출자에게 충분한가?
- 실패 과정에서 일부 상태만 바뀌지는 않는가?

<!--
교재 대응: book/src/testing/17-errors-panic-ignore.md > 오류, panic, 제외한 테스트


-->

---
level: 2
---

# `Result`를 반환하는 테스트

```rust
#[test]
fn parses_a_port() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!("443".parse::<u16>()?, 443);
    Ok(())
}
```

준비 과정의 오류를 `?`로 전달하기 편리합니다. 그러나 **오류 발생 자체가 요구
사항**이라면 바로 끝내지 말고 오류의 종류와 내용을 명시적으로 검사합니다.

<div class="takeaway compact"><code>Box&lt;dyn std::error::Error&gt;</code>는 여러 구체적인 오류를 하나의 반환 타입으로 다룰 때 사용합니다.</div>

<!--
교재 대응: book/src/testing/17-errors-panic-ignore.md > Result를 반환하는 테스트

테스트 함수는 Result를 반환할 수 있습니다. 성공하면 Ok(())를 반환하고, 오류가 발생하면 ?가 테스트를 실패로 끝냅니다. 준비 과정에서 여러 실패 가능한
작업을 연결할 때 유용합니다. 오류 발생 자체가 요구 사항이라면 ?로 바로 끝내지 말고 오류의 종류와 필요한 정보를 명시적으로 검사하세요. 참고: Box는 무엇인가?
std::error::Error는 여러 오류 타입이 공통으로 구현하는 trait입니다. dyn std::error::Error는 구체적인 오류 타입 대신 이 trait을
구현한 오류를 받겠다는 뜻입니다. 오류 타입마다 크기가 다를 수 있으므로 Box에 넣어 크기가 일정한 포인터로 다룹니다. 이렇게 반환 타입을 쓰면 여러 종류의 오류에
?를 사용할 수 있습니다. 이 예제처럼 정수 변환 오류만 생긴다면 Box 대신 구체적인 ParseIntError를 반환해도 됩니다.
-->

---
level: 2
---

# panic 검사하기

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

<!--
교재 대응: book/src/testing/17-errors-panic-ignore.md > panic 검사하기

panic이 계약에 포함된 API는 #[should_panic]으로 검사할 수 있습니다. 가능하면 expected에 메시지 일부를 적어 엉뚱한 panic으로 테스트가
통과하는 일을 줄입니다. 예상할 수 있는 잘못된 사용자 입력은 panic보다 Result로 처리하는 편이 보통 더 알맞습니다. should_panic을 사용하기 전에
해당 상황에서 panic이 적절한 동작인지 먼저 판단하세요.
-->

---
level: 2
---

# 평소 실행에서 제외하기

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

<!--
교재 대응: book/src/testing/17-errors-panic-ignore.md > 평소 실행에서 제외하기

오래 걸리거나 특별한 외부 환경이 필요한 테스트에는 #[ignore]를 붙일 수 있습니다. #[ignore]가 붙은 테스트는 기본적으로 실행되지 않습니다.
--ignored는 #[ignore]가 붙은 테스트만 실행하며 평소 실행하는 테스트는 제외합니다. 평소 테스트와 #[ignore]가 붙은 테스트를 모두 실행하려면
--include-ignored를 사용합니다. 테스트가 자주 실패한다는 이유만으로 ignore를 붙여서는 안 됩니다. 실행 순서, 공유 상태, 시간, 외부 서비스 때문에
불규칙하게 실패한다면 먼저 원인을 제거해야 합니다.
-->

---
level: 2
---

# 문서 주석과 문서 테스트

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

<!--
교재 대응: book/src/testing/18-documentation-tests.md > 문서 주석과 문서 테스트; book/src/testing/18-documentation-tests.md > 실행되는 사용 예제

Rust의 공개 API에는 /// 문서 주석을 붙일 수 있습니다. 모듈이나 크레이트(crate) 전체를 설명할 때는 //!를 사용합니다. 문서 주석 안에서 백틱 세
개(```)로 Rust 코드 블록을 감쌀 수 있고, cargo test --doc는 이 코드 블록을 별도로 컴파일하고 실행합니다. 따라서 문서의 사용법이 실제 코드와
달라지는 일을 줄일 수 있습니다. 문서 내용에서 #로 시작하는 use 문은 생성된 문서에는 보이지 않지만 문서 테스트에서는 함께 컴파일됩니다. 독자에게 중요하지 않은
준비 코드를 숨기되, 실제 예제가 실행되는 조건은 그대로 검사할 수 있습니다.
-->

---
level: 2
---

# 코드 블록 속성

<table class="compare">
  <thead><tr><th>속성</th><th>문서 테스트의 동작</th></tr></thead>
  <tbody>
    <tr><td>속성 없음</td><td>컴파일하고 실행</td></tr>
    <tr><td><code>no_run</code></td><td>컴파일하지만 실행하지 않음</td></tr>
    <tr><td><code>compile_fail</code></td><td>컴파일이 실패해야 통과</td></tr>
    <tr><td><code>ignore</code></td><td>문서 테스트에서 제외</td></tr>
  </tbody>
</table>

<!--
교재 대응: book/src/testing/18-documentation-tests.md > 코드 블록 속성

코드 블록의 여는 표시 뒤에는 no_run이나 compile_fail 같은 속성을 적을 수 있습니다. • 아무 속성이 없는 Rust 블록은 컴파일하고 실행합니다. •
no_run은 컴파일하지만 실행하지 않습니다. • compile_fail은 컴파일이 실패해야 테스트가 통과합니다. • ignore는 문서 테스트에서 제외합니다.
no_run /// std::fs::write("report.txt", "완료")?; /// # Ok::(()) ///
-->

---
level: 2
---

# 코드 블록 속성

```rust
//! ```compile_fail
//! let message = String::from("완료");
//! drop(message);
//! println!("{message}");
//! ```
```

`compile_fail`은 소유권이나 타입 제약처럼 허용되지 않는 사용법을 보여 줄 때
유용합니다. 우연히 컴파일되지 않는 내부 구현을 고정하는 데 쓰지 않습니다.

<!--
교재 대응: book/src/testing/18-documentation-tests.md > 코드 블록 속성

코드 블록의 여는 표시 뒤에는 no_run이나 compile_fail 같은 속성을 적을 수 있습니다. compile_fail /// let message =
String::from("완료"); /// drop(message); /// println!("{message}"); /// compile_fail은 소유권이나 타입
제약 때문에 허용되지 않는 사용법을 검사할 때 유용합니다. 공개 API에서 계속 금지해야 하는 사용법에만 적용하고, 구현을 고치면 사라질 일시적인 컴파일 오류에는
사용하지 마세요.
-->

---
level: 2
---

# 실행하기

```bash
cargo test --doc --locked
cargo test --doc --no-default-features --locked
cargo test --doc --all-features --locked
```

문서에는 대표적인 공개 사용법을 두고, 경계 조건과 오류 조합은 단위·통합
테스트에서 자세히 검사합니다.

<div class="takeaway">문서 테스트는 일반 테스트를 대신하지 않고 사용자의 첫 경로를 지킵니다.</div>

<!--
교재 대응: book/src/testing/18-documentation-tests.md > 실행하기; book/src/testing/18-documentation-tests.md > 문서 만들기

라이브러리의 문서 테스트만 실행하려면 다음 명령을 사용합니다. 피처(feature)에 따라 문서와 공개 API가 달라진다면 지원하는 구성을 각각 검사합니다. 문서
테스트가 일반 단위 테스트를 대신하지는 않습니다. 문서에는 대표적인 공개 사용법을 두고, 경계 조건과 오류 조합은 단위·통합 테스트에서 자세히 검사하세요. 현재 패키지와
의존성의 API 문서를 만들고 브라우저로 열 때는 cargo doc --open을 사용합니다. CI처럼 브라우저를 열지 않는 환경에서는 cargo doc --no-deps로
현재 패키지의 문서가 만들어지는지만 확인할 수 있습니다.
-->

---
level: 2
---

# 문서 주석과 문서 테스트

<div class="tool-flow">
  <div><strong>단위 테스트</strong>작은 내부 규칙과 계산</div>
  <div><strong>통합 테스트</strong>외부에서 보이는 공개 동작</div>
  <div><strong>문서 테스트</strong>대표 사용법과 금지된 사용법</div>
</div>

오래 걸리는 검사와 플랫폼별 검사는 실행 조건을 분명히 하고 전체 검증 흐름에
배치합니다.

<!--
교재 대응: book/src/testing/18-documentation-tests.md > 문서 주석과 문서 테스트

Rust의 공개 API에는 /// 문서 주석을 붙일 수 있습니다. 모듈이나 크레이트(crate) 전체를 설명할 때는 //!를 사용합니다. 문서 주석 안에서 백틱 세
개(```)로 Rust 코드 블록을 감쌀 수 있고, cargo test --doc는 이 코드 블록을 별도로 컴파일하고 실행합니다. 따라서 문서의 사용법이 실제 코드와
달라지는 일을 줄일 수 있습니다.
-->
