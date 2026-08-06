---
theme: default
title: 전체 검증 흐름과 종합 실습
info: 우아한 Rust 중급 개발 도구 10편
colorSchema: dark
aspectRatio: 16/9
canvasWidth: 1100
transition: fade-out
lineNumbers: true
layout: cover
class: cover
---

<div class="episode-label">우아한 Rust 중급 · 개발 도구 10</div>

# 전체 검증 흐름과<br>종합 실습

도구를 순서대로 연결하고 선택의 근거를 기록합니다.

<!--
교재 대응: book/src/practice/21-exercises.md > 종합 실습

지금까지 사용한 도구를 연결해 이름별 횟수를 세는 공개 함수를 완성합니다. 명령을 실행하는 데서 끝내지 않고 구현, 테스트, API 문서, 실행 예제를 결과물로
남깁니다. 다음 src/lib.rs를 시작점으로 사용합니다.
-->

---
level: 2
---

# 포맷·린트·테스트 연결하기

<div class="tool-flow">
  <div><strong>정적 확인</strong>형식·타입·소유권·린트<sub>lint</sub></div>
  <div><strong>동작 확인</strong>단위·통합·문서 테스트</div>
  <div><strong>사용법 확인</strong>API 문서</div>
</div>

<div class="takeaway">앞 단계가 빠르게 실패할수록 뒤의 비싼 검사를 실행하기 전에 문제를 좁힐 수 있습니다.</div>

<!--
교재 대응: book/src/practice/20-workflow.md > 포맷·린트·테스트 연결하기

다음 순서로 도구들을 활용하면 문제 범위를 빨리 좁힐 수 있습니다. 코드를 작성하는 동안에는 소스 코드를 저장할 때 rustfmt로 형식을 맞추고 클리피(Clippy)로
린트(lint)를 빠르게 확인합니다. 변경 작업을 마치면 편집기 진단 내용에만 의존하지 않고 다음 명령을 차례로 실행합니다. rustup show
active-toolchain으로 프로젝트에 선택된 툴체인(toolchain)을 확인합니다. cargo fmt --check로 코드 형식을 점검합니다. 클리피를
--no-default-features와 --all-features로 각각 실행해 피처(feature)를 켰을 때 들어오는 코드와 빠지는 코드를 함께 검사합니다.
cargo check를 기본 구성, --no-default-features와 --all-features로 각각 실행해 지원하는 피처 구성을 컴파일합니다. cargo
test도 세 가지 피처 구성으로 실행해 동작을 검증합니다. 문서 테스트도 --no-default-features와 --all-features로 각각 실행합니다. API
문서도 두 피처 구성으로 생성해 공개 항목과 연결을 확인합니다.
-->

---
level: 2
---

# 포맷·린트·테스트 연결하기

<ol class="step-list">
  <li>선택된 툴체인<sub>toolchain</sub>을 확인합니다.</li>
  <li>형식과 클리피<sub>Clippy</sub>를 검사합니다.</li>
  <li>지원하는 피처<sub>feature</sub> 구성을 컴파일합니다.</li>
  <li>단위·통합·문서 테스트를 실행합니다.</li>
  <li>API 문서를 생성하고 직접 읽습니다.</li>
</ol>

<!--
교재 대응: book/src/practice/20-workflow.md > 포맷·린트·테스트 연결하기

코드를 작성하는 동안에는 소스 코드를 저장할 때 rustfmt로 형식을 맞추고 클리피(Clippy)로 린트(lint)를 빠르게 확인합니다. 변경 작업을 마치면 편집기
진단 내용에만 의존하지 않고 다음 명령을 차례로 실행합니다. rustup show active-toolchain으로 프로젝트에 선택된 툴체인(toolchain)을
확인합니다. cargo fmt --check로 코드 형식을 점검합니다. 클리피를 --no-default-features와 --all-features로 각각 실행해
피처(feature)를 켰을 때 들어오는 코드와 빠지는 코드를 함께 검사합니다. cargo check를 기본 구성, --no-default-features와
--all-features로 각각 실행해 지원하는 피처 구성을 컴파일합니다. cargo test도 세 가지 피처 구성으로 실행해 동작을 검증합니다. 문서 테스트도
--no-default-features와 --all-features로 각각 실행합니다. API 문서도 두 피처 구성으로 생성해 공개 항목과 연결을 확인합니다. 이
프로젝트의 make check는 툴체인을 바꾸지 않으며 2번부터 7번, mdBook 빌드까지 실행합니다. 툴체인 설치와 업데이트는 사용자 환경을 바꾸므로 자동 검사에
넣지 않고 프로젝트를 시작할 때 별도로 확인합니다.
-->

---
level: 2
---

# 포맷·린트·테스트 연결하기

<ol class="step-list">
  <li><code>cargo fmt --check</code>로 형식을 검사합니다.</li>
  <li><code>cargo clippy</code>로 두 피처 구성을 검사합니다.</li>
  <li><code>cargo check</code>로 세 피처 구성을 컴파일합니다.</li>
  <li><code>cargo test</code>로 세 피처 구성을 테스트합니다.</li>
  <li>문서 테스트와 API 문서를 두 피처 구성으로 검사합니다.</li>
  <li>마지막으로 mdBook을 빌드합니다.</li>
</ol>

툴체인 설치와 업데이트는 사용자 환경을 바꾸므로 자동 검사에 넣지 않고 프로젝트를
시작할 때 별도로 확인합니다.

<!--
교재 대응: book/src/practice/20-workflow.md > 포맷·린트·테스트 연결하기

코드를 작성하는 동안에는 소스 코드를 저장할 때 rustfmt로 형식을 맞추고 클리피(Clippy)로 린트(lint)를 빠르게 확인합니다. 변경 작업을 마치면 편집기
진단 내용에만 의존하지 않고 다음 명령을 차례로 실행합니다. rustup show active-toolchain으로 프로젝트에 선택된 툴체인(toolchain)을
확인합니다. cargo fmt --check로 코드 형식을 점검합니다. 클리피를 --no-default-features와 --all-features로 각각 실행해
피처(feature)를 켰을 때 들어오는 코드와 빠지는 코드를 함께 검사합니다. cargo check를 기본 구성, --no-default-features와
--all-features로 각각 실행해 지원하는 피처 구성을 컴파일합니다. cargo test도 세 가지 피처 구성으로 실행해 동작을 검증합니다. 문서 테스트도
--no-default-features와 --all-features로 각각 실행합니다. API 문서도 두 피처 구성으로 생성해 공개 항목과 연결을 확인합니다. 이
프로젝트의 make check는 툴체인을 바꾸지 않으며 2번부터 7번, mdBook 빌드까지 실행합니다. 툴체인 설치와 업데이트는 사용자 환경을 바꾸므로 자동 검사에
넣지 않고 프로젝트를 시작할 때 별도로 확인합니다.
-->

---
level: 2
---

# 완성할 기능

```rust
pub fn count_normalized_names(
    names: &[&str],
) -> BTreeMap<String, usize> {
    todo!("주어진 이름별 횟수를 세는 함수를 구현하세요: {names:?}")
}
```

완성할 요구 사항:

- 앞뒤 공백을 없앱니다.
- 빈 이름을 제외합니다.
- 소문자로 바꿔 같은 이름의 횟수를 더합니다.
- 결과를 이름순으로 반환합니다.

<!--
교재 대응: book/src/practice/21-exercises.md > 완성할 기능

src/lib.rs에는 다음 공개 함수의 시그니처와 todo!()가 준비되어 있습니다. todo!()를 실제 구현으로 바꾸세요. • 이름 앞뒤의 공백을 없앱니다. • 빈
문자열과 공백뿐인 이름을 제외합니다. • 대소문자를 구분하지 않도록 소문자로 바꿉니다. • 같은 이름이 여러 번 나오면 횟수를 더합니다. • 결과를 이름순으로
반환합니다. 기존 normalize_names를 조합해도 되고, 이름을 순회하면서 한 번에 계산해도 됩니다. 두 방식 가운데 요구 사항을 더 분명하게 드러낸다고 판단한
구현을 선택하세요.
-->

---
level: 2
---

# 작업 순서

```rust {1-11}{lines:true}
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
```

<div class="command">cargo test --test exercises --locked</div>

`#[ignore]`를 제거한 뒤 실행해 `todo!()`에서 실패하는 것을 먼저 확인합니다.

<!--
교재 대응: book/src/practice/21-exercises.md > 작업 순서

두 통합 테스트의 #[ignore]를 제거합니다. 통합 테스트를 실행해 todo!()에서 아직 구현하지 않았다는 메시지와 함께 실패하는 것을 확인합니다.
src/lib.rs의 todo!()를 실제 구현으로 바꾸고 테스트를 다시 실행합니다. 공개 함수에 설명과 대표 사용 예제를 문서 주석으로 작성합니다.
examples/21_exercises.rs에서 새 함수를 호출해 결과를 출력합니다. rustfmt와 클리피(Clippy)의 결과를 읽고 필요한 부분을 수정합니다. 기본
구성, 기본 피처(feature)를 끈 구성, 모든 피처를 켠 구성에서 컴파일과 테스트를 실행합니다. 문서 테스트를 실행하고 생성한 API 문서에서 새 함수의 설명과
예제를 읽습니다. git diff에서 구현, 테스트, 문서, 예제 이외의 변경이 섞이지 않았는지 확인합니다.
-->

---
level: 2
---

# 작업 순서

<ol class="step-list">
  <li><code>todo!()</code>를 실제 구현으로 바꿉니다.</li>
  <li>같은 통합 테스트를 다시 실행합니다.</li>
  <li>공개 함수에 설명과 실행 가능한 예제를 작성합니다.</li>
  <li>실행 예제에서 이름별 횟수를 확인합니다.</li>
</ol>

```bash
cargo test --test exercises --locked
cargo run --example 21_exercises
```

<!--
교재 대응: book/src/practice/21-exercises.md > 작업 순서

두 통합 테스트의 #[ignore]를 제거합니다. 통합 테스트를 실행해 todo!()에서 아직 구현하지 않았다는 메시지와 함께 실패하는 것을 확인합니다.
src/lib.rs의 todo!()를 실제 구현으로 바꾸고 테스트를 다시 실행합니다. 공개 함수에 설명과 대표 사용 예제를 문서 주석으로 작성합니다.
examples/21_exercises.rs에서 새 함수를 호출해 결과를 출력합니다. rustfmt와 클리피(Clippy)의 결과를 읽고 필요한 부분을 수정합니다. 기본
구성, 기본 피처(feature)를 끈 구성, 모든 피처를 켠 구성에서 컴파일과 테스트를 실행합니다. 문서 테스트를 실행하고 생성한 API 문서에서 새 함수의 설명과
예제를 읽습니다. git diff에서 구현, 테스트, 문서, 예제 이외의 변경이 섞이지 않았는지 확인합니다.
-->

---
level: 2
---

# 포맷·린트·테스트 연결하기

```bash
cargo run --example 13_features
cargo run --example 13_features --features binary-search

cargo check --locked
cargo check --no-default-features --locked
cargo check --all-features --locked
```

<div class="takeaway"><code>--all-features</code>에서는 피처를 껐을 때만 들어오는 코드가 빠질 수 있습니다.</div>

<!--
교재 대응: book/src/practice/20-workflow.md > 포맷·린트·테스트 연결하기

다음 순서로 도구들을 활용하면 문제 범위를 빨리 좁힐 수 있습니다. rustup show active-toolchain으로 프로젝트에 선택된 툴체인(toolchain)을
확인합니다. cargo fmt --check로 코드 형식을 점검합니다. 클리피를 --no-default-features와 --all-features로 각각 실행해
피처(feature)를 켰을 때 들어오는 코드와 빠지는 코드를 함께 검사합니다. cargo check를 기본 구성, --no-default-features와
--all-features로 각각 실행해 지원하는 피처 구성을 컴파일합니다. cargo test도 세 가지 피처 구성으로 실행해 동작을 검증합니다. 문서 테스트도
--no-default-features와 --all-features로 각각 실행합니다. API 문서도 두 피처 구성으로 생성해 공개 항목과 연결을 확인합니다.
--all-features를 실행해도 cfg(not(feature = "...")) 분기는 컴파일되지 않을 수 있습니다. 옵션 이름만 보고 모든 조건부 코드를 검사했다고
판단하지 말고 실제 cfg 조건과 프로젝트가 지원하는 조합을 확인하세요.
-->

---
level: 2
---

# 작업 순서

```bash
cargo test --doc --no-default-features --locked
cargo test --doc --all-features --locked
cargo doc --no-deps --no-default-features --locked
cargo doc --no-deps --all-features --locked
```

생성된 문서에서 `count_normalized_names`를 찾아 다음을 확인합니다.

- 설명만 읽고 입력과 결과를 예상할 수 있는가?
- 대표 예제가 실제 사용 순서를 보여 주는가?
- 피처별 공개 API가 예상대로 나타나는가?

<!--
교재 대응: book/src/practice/21-exercises.md > 작업 순서

두 통합 테스트의 #[ignore]를 제거합니다. 통합 테스트를 실행해 todo!()에서 아직 구현하지 않았다는 메시지와 함께 실패하는 것을 확인합니다.
src/lib.rs의 todo!()를 실제 구현으로 바꾸고 테스트를 다시 실행합니다. 공개 함수에 설명과 대표 사용 예제를 문서 주석으로 작성합니다.
examples/21_exercises.rs에서 새 함수를 호출해 결과를 출력합니다. rustfmt와 클리피(Clippy)의 결과를 읽고 필요한 부분을 수정합니다. 기본
구성, 기본 피처(feature)를 끈 구성, 모든 피처를 켠 구성에서 컴파일과 테스트를 실행합니다. 문서 테스트를 실행하고 생성한 API 문서에서 새 함수의 설명과
예제를 읽습니다. git diff에서 구현, 테스트, 문서, 예제 이외의 변경이 섞이지 않았는지 확인합니다.
-->

---
level: 2
---

# 남겨야 할 결과물

<div class="tool-flow">
  <div><strong>공개 구현</strong><code>src/lib.rs</code></div>
  <div><strong>통합 테스트</strong><code>tests/exercises.rs</code></div>
  <div><strong>문서와 실행 예제</strong><code>src/lib.rs</code><br><code>examples/21_exercises.rs</code></div>
</div>

<div class="takeaway"><code>git diff</code>에서 구현, 테스트, 문서, 실행 예제를 함께 확인합니다.</div>

<!--
교재 대응: book/src/practice/21-exercises.md > 남겨야 할 결과물

실습을 마치면 Git diff에 다음 변경이 보여야 합니다. src/lib.rs: count_normalized_names의 공개 구현
tests/exercises.rs: 섞인 입력과 빈 입력을 확인하는 통합 테스트 src/lib.rs: 함수의 동작을 설명하고 실행 가능한 예제를 포함한 문서 주석
examples/21_exercises.rs: 새 함수를 호출하고 이름별 횟수를 출력하는 코드 tests/exercises.rs에는 통과해야 할 통합 테스트가
#[ignore] 표시와 함께 미리 준비되어 있습니다. 테스트에는 구현끼리 비교한 결과가 아니라 기대 결과가 직접 적혀 있습니다. 구현을 시작할 때 두 테스트의
#[ignore]를 제거하세요.
-->

---
level: 2
---

# AI와 함께 검토하기

- 빈 이름을 제외하고 대소문자를 합치는가?
- 소유한 `String`을 불필요하게 반복해서 만드는가?
- 기존 함수를 재사용하는 방식과 직접 순회하는 방식의 차이를 설명하는가?
- 테스트를 없애거나 린트를 `allow`로 덮지 않는가?
- 적용 뒤 통합 테스트와 문서 테스트가 통과하는가?

<!--
교재 대응: book/src/practice/21-exercises.md > AI와 함께 검토하기

먼저 직접 구현한 뒤 AI에게 다른 구현을 제안해 달라고 요청하세요. 제안받은 코드는 바로 적용하지 말고 다음 내용을 확인합니다. • 빈 이름을 제외하고 대소문자를
합치는가? • 불필요한 String을 반복해서 만드는가? • 기존 normalize_names를 재사용했을 때와 직접 순회할 때의 차이를 설명하는가? • 테스트를
없애거나 린트(lint)를 allow로 덮지 않는가? • 적용 뒤 통합 테스트와 문서 테스트가 그대로 통과하는가? AI의 제안을 적용했다면 git diff로 직접 작성한
구현에서 무엇이 달라졌는지 확인하세요.
-->

---
level: 2
---

# 완료 조건

<ul class="check-list">
  <li>섞인 입력과 빈 입력 테스트가 통과합니다.</li>
  <li>문서 예제가 문서 테스트에서 실행됩니다.</li>
  <li>실행 예제에서 이름별 횟수를 확인할 수 있습니다.</li>
  <li>기본 피처를 끈 구성과 모든 피처 구성이 통과합니다.</li>
  <li>rustfmt와 클리피가 지적할 사항이 없습니다.</li>
</ul>

<!--
교재 대응: book/src/practice/21-exercises.md > 완료 조건

다음 질문에 모두 “예”라고 답할 수 있으면 실습을 마친 것입니다. • 섞인 입력과 빈 입력 테스트가 통과하는가? • 구현이 기존 공개 함수의 동작을 불필요하게 중복하지
않는가? • 문서의 예제가 문서 테스트에서 실행되는가? • 실행 예제에서 이름별 횟수를 확인할 수 있는가? • 기본 피처를 끈 구성과 모든 피처를 켠 구성이 모두
통과하는가? • rustfmt와 클리피가 지적할 사항이 남아 있지 않은가?
-->

---
level: 2
---

# 포맷·린트·테스트 연결하기

<div class="grid grid-cols-2 gap-6">
  <div class="command">make check</div>
  <div class="command">make msrv</div>
</div>

두 명령은 현재 툴체인의 전체 흐름과 패키지가 지원하는 가장 오래된 Rust를
나누어 확인합니다.

- 형식과 클리피
- 피처별 컴파일과 테스트
- 문서 테스트와 API 문서 생성
- mdBook 빌드

<!--
교재 대응: book/src/practice/20-workflow.md > 포맷·린트·테스트 연결하기

rustup show active-toolchain으로 프로젝트에 선택된 툴체인(toolchain)을 확인합니다. cargo fmt --check로 코드 형식을
점검합니다. 클리피를 --no-default-features와 --all-features로 각각 실행해 피처(feature)를 켰을 때 들어오는 코드와 빠지는 코드를
함께 검사합니다. cargo check를 기본 구성, --no-default-features와 --all-features로 각각 실행해 지원하는 피처 구성을
컴파일합니다. cargo test도 세 가지 피처 구성으로 실행해 동작을 검증합니다. 문서 테스트도 --no-default-features와 --all-features로
각각 실행합니다. API 문서도 두 피처 구성으로 생성해 공개 항목과 연결을 확인합니다. 이 프로젝트의 make check는 툴체인을 바꾸지 않으며 2번부터 7번,
mdBook 빌드까지 실행합니다. 툴체인 설치와 업데이트는 사용자 환경을 바꾸므로 자동 검사에 넣지 않고 프로젝트를 시작할 때 별도로 확인합니다. 패키지가 지원하는 가장
오래된 Rust 버전은 해당 툴체인이 설치된 환경에서 별도로 확인합니다.
-->

---
level: 2
---

# 포맷·린트·테스트 연결하기

<table class="compare">
  <tbody>
    <tr><td>rustfmt를 통과함</td><td>잘못된 값을 계산할 수 있음</td></tr>
    <tr><td>클리피 경고가 없음</td><td>경계 조건에서 실패할 수 있음</td></tr>
    <tr><td>테스트를 통과함</td><td>검사하지 않은 입력에서는 실패할 수 있음</td></tr>
  </tbody>
</table>

<!--
교재 대응: book/src/practice/20-workflow.md > 포맷·린트·테스트 연결하기

다음 순서로 도구들을 활용하면 문제 범위를 빨리 좁힐 수 있습니다. 코드를 작성하는 동안에는 소스 코드를 저장할 때 rustfmt로 형식을 맞추고 클리피(Clippy)로
린트(lint)를 빠르게 확인합니다. 변경 작업을 마치면 편집기 진단 내용에만 의존하지 않고 다음 명령을 차례로 실행합니다. 도구들이 모든 문제를 해결하는 것은
아닙니다. rustfmt를 통과한 코드도 잘못된 결과를 낼 수 있고, 클리피 경고가 없는 코드도 요구 사항을 어길 수 있습니다.
-->

---
level: 2
layout: center
class: section
---

# 개발 도구 퀴즈와 풀이

각 문제에서 가장 알맞은 답을 하나 고른 뒤 클릭해서 풀이를 확인합니다.

<!--
교재 대응: book/src/quiz/23-quiz.md > 개발 도구 퀴즈와 풀이; book/src/quiz/23-quiz.md > 문제; book/src/quiz/23-quiz.md > 정답과 풀이

각 문제에서 가장 알맞은 답을 하나 고르세요. 먼저 아홉 문제를 모두 푼 뒤 정답과 풀이에서 판단 근거를 확인합니다.
-->

---
level: 2
---

# 1. Rust 개발 도구의 역할

<div class="quiz-options">
  <div><strong>①</strong> <code>rustup</code>은 Rust 소스 코드를 컴파일합니다.</div>
  <div><strong>②</strong> <code>rustc</code>는 패키지 의존성을 관리합니다.</div>
  <div><strong>③</strong> 카고<sub>Cargo</sub>는 패키지와 의존성을 관리하고 빌드와 테스트를 실행합니다.</div>
  <div><strong>④</strong> <code>cargo update</code>는 Rust 툴체인을 업데이트합니다.</div>
</div>

<div v-click class="takeaway"><strong>정답: ③</strong> rustup은 툴체인을 관리하고, rustc는 소스 코드를 컴파일합니다.</div>

<!--
교재 대응: book/src/quiz/23-quiz.md > 1. Rust 개발 도구의 역할; book/src/quiz/23-quiz.md > 1. 정답: ③

• ① rustup은 Rust 소스 코드를 실행 파일로 컴파일합니다. • ② rustc는 패키지의 의존성을 추가하고 제거합니다. • ③ 카고(Cargo)는 패키지와
의존성을 관리하고 빌드와 테스트를 실행합니다. • ④ cargo update는 설치된 Rust 툴체인(toolchain)을 업데이트합니다. 카고는 패키지와 의존성을
관리하고 빌드와 테스트 같은 작업을 실행합니다. rustup은 툴체인과 컴포넌트(component)를 설치하고 선택하며, rustc는 Rust 소스 코드를 컴파일합니다.
Rust 툴체인은 cargo update가 아니라 rustup update로 업데이트합니다.
-->

---
level: 2
---

# 2. 프로젝트에서 사용하는 Rust 버전

<div class="quiz-options">
  <div><strong>①</strong> 두 설정은 항상 같은 버전이어야 합니다.</div>
  <div><strong>②</strong> <code>rust-toolchain.toml</code>은 사용할 툴체인, <code>rust-version</code>은 가장 오래된 지원 버전을 나타냅니다.</div>
  <div><strong>③</strong> <code>rust-version</code>이 rustup의 기본값을 바꿉니다.</div>
  <div><strong>④</strong> <code>rust-toolchain.toml</code>이 의존성 버전을 고정합니다.</div>
</div>

<div v-click class="takeaway"><strong>정답: ②</strong> 두 설정은 대상과 목적이 다르므로 반드시 같을 필요가 없습니다.</div>

<!--
교재 대응: book/src/quiz/23-quiz.md > 2. 프로젝트에서 사용하는 Rust 버전; book/src/quiz/23-quiz.md > 2. 정답: ②

rust-toolchain.toml과 Cargo.toml의 rust-version을 올바르게 설명한 것은 무엇인가요? • ① 두 설정은 항상 같은 버전이어야 합니다. •
② rust-toolchain.toml은 이 저장소에서 사용할 툴체인을 정하고, rust-version은 패키지가 지원하는 가장 오래된 Rust 버전을 나타냅니다. •
③ rust-version을 바꾸면 rustup이 해당 버전을 자동으로 기본값으로 선택합니다. • ④ rust-toolchain.toml은 crates.io에 배포할
의존성 버전을 고정합니다. rust-toolchain.toml은 저장소에서 명령을 실행할 때 rustup이 선택할 툴체인을 정합니다. rust-version은 패키지가
지원한다고 약속하는 가장 오래된 Rust 버전을 카고에 알립니다. 두 값의 목적이 다르므로 반드시 같을 필요는 없습니다.
-->

---
level: 2
---

# 3. 카고 타깃과 컴파일 오류

<div class="quiz-options">
  <div><strong>①</strong> <code>cargo check --all-targets --locked</code>를 실행하고 첫 번째 오류부터 읽습니다.</div>
  <div><strong>②</strong> <code>cargo run</code>을 실행하고 마지막 오류부터 읽습니다.</div>
  <div><strong>③</strong> <code>cargo fmt --check</code>만 실행합니다.</div>
  <div><strong>④</strong> <code>rustc src/lib.rs</code>만 실행합니다.</div>
</div>

<div v-click class="takeaway"><strong>정답: ①</strong> 예제와 테스트도 타깃이며, 뒤의 오류는 첫 오류에서 이어졌을 수 있습니다.</div>

<!--
교재 대응: book/src/quiz/23-quiz.md > 3. 카고 타깃과 컴파일 오류; book/src/quiz/23-quiz.md > 3. 정답: ①

새 예제와 테스트를 추가한 뒤 모든 카고 타깃이 컴파일되는지 검사하려고 합니다. 가장 알맞은 작업은 무엇인가요? • ① cargo check --all-targets
--locked를 실행하고 첫 번째 오류부터 읽습니다. • ② cargo run --locked를 실행하고 마지막 오류부터 읽습니다. • ③ cargo fmt
--check만 실행합니다. • ④ rustc src/lib.rs만 실행합니다. --all-targets는 라이브러리뿐 아니라 실행 파일, 예제, 테스트까지 검사 범위를
넓힙니다. --locked는 검사 도중 Cargo.lock을 바꿔야 하면 명령을 실패시킵니다. 오류가 여러 개라면 뒤의 오류가 첫 오류에서 이어졌을 수 있으므로 첫 번째
오류부터 고칩니다.
-->

---
level: 2
---

# 4. rustfmt와 클리피

<div class="quiz-options">
  <div><strong>①</strong> CI에서 <code>cargo fmt</code>로 파일을 고칩니다.</div>
  <div><strong>②</strong> 모든 클리피 경고를 프로젝트 전체에서 허용합니다.</div>
  <div><strong>③</strong> <code>cargo fmt --check</code>를 사용하고 클리피 제안은 목적과 테스트를 살펴본 뒤 적용합니다.</div>
  <div><strong>④</strong> rustfmt를 통과하면 동작도 올바릅니다.</div>
</div>

<div v-click class="takeaway"><strong>정답: ③</strong> rustfmt는 형식을 검사하고, 클리피는 판단할 제안을 보여 줍니다.</div>

<!--
교재 대응: book/src/quiz/23-quiz.md > 4. rustfmt와 클리피; book/src/quiz/23-quiz.md > 4. 정답: ③

자동 검사에서 코드 형식과 클리피(Clippy) 경고를 확인하는 방법으로 알맞은 것은 무엇인가요? • ① cargo fmt가 파일을 고치도록 한 뒤 변경 내용을 자동으로
커밋합니다. • ② 클리피 경고가 나오면 프로젝트 전체에 #[allow(clippy::all)]을 추가합니다. • ③ cargo fmt --check로 형식 차이를 찾고
클리피 제안은 코드의 목적과 테스트를 살펴본 뒤 적용합니다. • ④ rustfmt를 통과하면 코드가 요구 사항대로 동작한다고 판단합니다. 자동 검사에서는 파일을 바꾸지
않는 cargo fmt --check를 사용합니다. 클리피는 검토할 부분을 알려 주지만 프로그램의 요구 사항까지 알지는 못하므로 제안을 적용할지 직접 판단해야 합니다.
rustfmt는 형식만 맞추며 코드의 동작은 검사하지 않습니다.
-->

---
level: 2
---

# 5. 자동 수정

<div class="quiz-options">
  <div><strong>①</strong> 명령이 성공하면 바로 배포합니다.</div>
  <div><strong>②</strong> diff를 읽고 피처 검사와 테스트를 다시 실행합니다.</div>
  <div><strong>③</strong> 바뀐 파일을 확인하지 않고 모두 스테이징합니다.</div>
  <div><strong>④</strong> 같은 경고가 나오지 않도록 모든 린트를 허용합니다.</div>
</div>

<div v-click class="takeaway"><strong>정답: ②</strong> 자동 수정도 코드 변경이므로 범위와 동작을 다시 검증합니다.</div>

<!--
교재 대응: book/src/quiz/23-quiz.md > 5. 자동 수정; book/src/quiz/23-quiz.md > 5. 정답: ②

cargo fix나 cargo clippy --fix를 실행한 직후 해야 할 일로 가장 알맞은 것은 무엇인가요? • ① 명령이 성공했으므로 바로 배포합니다. • ②
git diff로 변경을 읽고 관련 피처(feature) 검사와 테스트를 다시 실행합니다. • ③ 자동 수정이 바꾼 파일을 확인하지 않고 모두 스테이징합니다. • ④
같은 경고가 나오지 않도록 모든 린트(lint)를 허용합니다. 자동 수정도 소스 파일을 직접 바꾸는 코드 변경입니다. git diff로 바뀐 범위를 읽고 공개 API와
오류 처리 방식이 그대로인지 살펴본 뒤 형식, 클리피, 피처별 컴파일과 관련 테스트를 다시 실행합니다.
-->

---
level: 2
---

# 6. 의존성 조사

직접 추가하지 않은 `serde`가 어떤 경로로 들어왔는지 찾는 명령은 무엇인가요?

<div class="quiz-options">
  <div><strong>①</strong> <code>cargo update -p serde</code></div>
  <div><strong>②</strong> <code>cargo remove serde</code></div>
  <div><strong>③</strong> <code>cargo tree -i serde</code></div>
  <div><strong>④</strong> <code>rustup component list --installed</code></div>
</div>

<div v-click class="takeaway"><strong>정답: ③</strong> <code>-i</code>는 지정한 패키지를 사용하는 상위 의존성을 거꾸로 보여 줍니다.</div>

<!--
교재 대응: book/src/quiz/23-quiz.md > 6. 의존성 조사; book/src/quiz/23-quiz.md > 6. 정답: ③

직접 추가하지 않은 serde가 어떤 의존성을 거쳐 들어왔는지 찾으려고 합니다. 어떤 명령이 가장 알맞은가요? • ① cargo update -p serde • ②
cargo remove serde • ③ cargo tree -i serde • ④ rustup component list --installed cargo tree -i
serde는 serde를 사용하는 상위 의존성을 거꾸로 보여 줍니다. cargo update는 허용된 범위에서 버전을 다시 선택하고, cargo remove는 직접
의존성을 제거합니다.
-->

---
level: 2
---

# 7. 카고 피처 검사

<div class="quiz-options">
  <div><strong>①</strong> 모든 운영체제에서 모든 코드를 검사합니다.</div>
  <div><strong>②</strong> 모든 카고 타깃과 모든 피처 조합을 검사합니다.</div>
  <div><strong>③</strong> 모든 피처를 함께 켜지만 피처를 껐을 때만 들어오는 코드는 빠질 수 있습니다.</div>
  <div><strong>④</strong> 기본 피처를 끈 구성까지 모두 검사합니다.</div>
</div>

<div v-click class="takeaway"><strong>정답: ③</strong> 타깃 범위와 지원하는 피처 구성은 별도로 지정해 검사합니다.</div>

<!--
교재 대응: book/src/quiz/23-quiz.md > 7. 카고 피처 검사; book/src/quiz/23-quiz.md > 7. 정답: ③

cargo check --all-features가 성공했습니다. 이 결과를 올바르게 해석한 것은 무엇인가요? • ① 모든 운영체제에서 모든 코드를 검사했습니다. • ②
모든 카고 타깃과 모든 피처 조합을 검사했습니다. • ③ 모든 피처를 함께 켠 구성은 검사했지만 피처를 껐을 때만 컴파일되는 코드는 빠질 수 있습니다. • ④ 기본
피처를 끈 구성까지 모두 검사했습니다. --all-features는 모든 피처를 동시에 켭니다. 이때 cfg(not(feature = "..."))처럼 피처를 껐을 때만
들어오는 코드는 컴파일되지 않을 수 있습니다. 카고 타깃의 범위는 --all-targets로 별도로 넓혀야 하며, 프로젝트가 지원하는 피처 구성도 각각 검사해야 합니다.
-->

---
level: 2
---

# 8. 테스트와 API 문서

<div class="quiz-options">
  <div><strong>①</strong> 문서 테스트만 실행하면 설명의 가독성도 알 수 있습니다.</div>
  <div><strong>②</strong> <code>cargo doc</code>만 실행하면 예제의 동작도 알 수 있습니다.</div>
  <div><strong>③</strong> <code>cargo test --doc</code>로 예제를 검사하고 생성한 API 문서도 직접 읽습니다.</div>
  <div><strong>④</strong> 비공개 함수의 단위 테스트만 실행합니다.</div>
</div>

<div v-click class="takeaway"><strong>정답: ③</strong> 실행되는 예제와 사람이 읽는 설명은 서로 다른 방법으로 확인합니다.</div>

<!--
교재 대응: book/src/quiz/23-quiz.md > 8. 테스트와 API 문서; book/src/quiz/23-quiz.md > 8. 정답: ③

공개 API의 대표 사용 예제가 실제 코드와 맞고, 생성된 문서에서 설명과 연결도 자연스러운지 확인하려고 합니다. 가장 알맞은 방법은 무엇인가요? • ① 문서 테스트만
실행하면 두 가지를 모두 확인할 수 있습니다. • ② cargo doc만 실행하면 예제의 실행 결과까지 확인할 수 있습니다. • ③ cargo test --doc로
예제를 검사하고 cargo doc으로 만든 문서를 직접 읽습니다. • ④ 비공개 함수의 단위 테스트만 실행합니다. 문서 테스트는 Rust 코드 블록이 컴파일되고
실행되는지 검사합니다. 생성된 API 문서는 공개 항목을 찾기 쉬운지, 설명과 링크가 자연스럽게 이어지는지 사람이 직접 읽어야 합니다. 두 검사는 서로 다른 문제를
찾습니다.
-->

---
level: 2
---

# 9. 전체 검증 흐름

<div class="quiz-options">
  <div><strong>①</strong> API 문서 생성 → 배포 → 필요하면 테스트</div>
  <div><strong>②</strong> 형식과 린트 → 피처별 컴파일과 테스트 → 문서 검사</div>
  <div><strong>③</strong> API 문서 → 모든 경고 허용 → 자동 수정</div>
  <div><strong>④</strong> 형식 수정 → 결과를 보지 않고 자동 커밋</div>
</div>

<div v-click class="takeaway"><strong>정답: ②</strong> 빠르게 실패하는 검사부터 실행하면 문제 범위를 일찍 좁힐 수 있습니다.</div>

<!--
교재 대응: book/src/quiz/23-quiz.md > 9. 전체 검증 흐름; book/src/quiz/23-quiz.md > 9. 정답: ②

변경을 마친 뒤 도구를 연결하는 순서로 가장 적절한 것은 무엇인가요? • ① API 문서 생성 → 배포 → 필요하면 테스트 • ② 형식과 린트 검사 → 피처별 컴파일과
테스트 → 문서 검사 • ③ API 문서 생성 → 모든 경고 허용 → 자동 수정 • ④ cargo fmt 실행 → 결과를 보지 않고 자동 커밋 빠르게 실패하는 형식과
린트 검사부터 시작하면 뒤의 검사 전에 문제를 좁힐 수 있습니다. 이어서 지원하는 피처 구성을 컴파일하고 테스트하며, 문서 테스트와 API 문서를 확인합니다.
-->

---
level: 2
layout: center
class: section
---

# 포맷·린트·테스트 연결하기

환경, diff, 테스트 결과가 함께 있을 때 코드를 믿을 수 있는 범위를 설명할 수
있습니다.

<!--
교재 대응: book/src/practice/20-workflow.md > 포맷·린트·테스트 연결하기

다음 순서로 도구들을 활용하면 문제 범위를 빨리 좁힐 수 있습니다. rustup show active-toolchain으로 프로젝트에 선택된 툴체인(toolchain)을
확인합니다. cargo fmt --check로 코드 형식을 점검합니다. 클리피를 --no-default-features와 --all-features로 각각 실행해
피처(feature)를 켰을 때 들어오는 코드와 빠지는 코드를 함께 검사합니다. cargo check를 기본 구성, --no-default-features와
--all-features로 각각 실행해 지원하는 피처 구성을 컴파일합니다. cargo test도 세 가지 피처 구성으로 실행해 동작을 검증합니다. 문서 테스트도
--no-default-features와 --all-features로 각각 실행합니다. API 문서도 두 피처 구성으로 생성해 공개 항목과 연결을 확인합니다.
--all-features를 실행해도 cfg(not(feature = "...")) 분기는 컴파일되지 않을 수 있습니다. 옵션 이름만 보고 모든 조건부 코드를 검사했다고
판단하지 말고 실제 cfg 조건과 프로젝트가 지원하는 조합을 확인하세요.
-->

---
level: 2
---

# 문제가 생기면 범위를 좁힙니다

<div class="tool-flow">
  <div><strong>출력</strong>첫 컴파일 오류와 린트 이름부터 읽기</div>
  <div><strong>변경</strong><code>git diff</code>와 잠금 파일 함께 보기</div>
  <div><strong>경로</strong><code>cargo tree -i</code>로 의존성 추적하기</div>
  <div><strong>검증</strong>피처별 테스트와 생성된 문서 확인하기</div>
</div>

<!--
교재 대응: book/src/appendix/22-cheatsheet.md > 개발 도구 명령 한눈에 보기; book/src/appendix/22-cheatsheet.md > 편집기 설정 한눈에 보기; book/src/appendix/22-cheatsheet.md > 문제를 만났을 때

개발 도구 명령 한눈에 보기: rustup과 활성 툴체인은 rustup show, 설치한 툴체인은 rustup toolchain list, 설치한 컴포넌트는 rustup
component list --installed로 확인합니다. stable에 rustfmt와 클리피를 추가할 때는 rustup component add --toolchain stable
rustfmt clippy, 일회성으로 stable을 지정할 때는 cargo +stable check를 사용합니다. 형식은 cargo fmt로 맞추고 cargo fmt
--check로 검사합니다. 컴파일러 제안은 cargo fix --locked, 클리피 제안은 cargo clippy --fix --locked로 적용합니다. 의존성은
cargo add와 cargo remove로 바꾸고, cargo update -p로 지정한 패키지만 업데이트합니다. cargo tree는 관계를, cargo tree -i는
들어온 경로를, cargo tree --duplicates는 중복 버전을, cargo tree -e features는 활성화된 피처 관계를 보여 줍니다. 클리피는
--all-targets와 --no-default-features 또는 --all-features를 조합하고 --locked -- -D warnings로 경고를 오류로 처리합니다.
cargo check도 --all-targets, --no-default-features, --all-features와 --locked를 조합합니다. 라이브러리·통합 테스트·예제는 cargo
test --lib --tests --examples --locked, 문서 테스트는 cargo test --doc --locked로 실행합니다. 현재 패키지의 API 문서는
cargo doc --no-deps --locked로 만들고, 오류 코드 설명은 rustc --explain E0382처럼 확인합니다.

편집기 설정 한눈에 보기: VS Code의 프로젝트 설정 위치는 .vscode/settings.json이며 핵심 설정은 editor.formatOnSave와
rust-analyzer.check.command입니다. Zed의 프로젝트 설정 위치는 .zed/settings.json이며 핵심 설정은 format_on_save,
formatter, check.command입니다. 두 편집기 모두 저장할 때 rustfmt를 실행하고 rust-analyzer의 검사 명령을 클리피로 바꿀 수 있습니다.

문제를 만났을 때: rustfmt가 파일을 바꿨다면 diff와 의도하지 않은 파일을 확인합니다. 클리피 린트 이름과 경고 원인을 읽은 뒤 수정 여부를 정합니다. 자동 수정
뒤에는 diff를 읽고 피처별 검사와 테스트를 다시 실행합니다. 의존성을 업데이트했다면 Cargo.toml과 Cargo.lock을 함께 확인하고, 예상하지 않은 의존성은
cargo tree -i로 경로를 찾습니다. 컴파일 오류는 첫 번째 오류부터 고치고 다시 검사합니다. allow가 필요하면 가장 좁은 범위에 이유를 남깁니다. 공개 API의
대표 사용법이 문서 테스트로 실행되는지 확인하고, 생성된 문서에서 항목을 찾기 쉽고 설명이 자연스러운지 읽습니다. 두 구현을 비교하는 테스트에는 기대 결과를 함께 적어
같은 오답을 놓치지 않습니다.
-->

---
level: 2
layout: center
class: section
---

# 감사합니다

<!--
교재 대응: book/src/index.md > 우아한 Rust 중급: 개발 도구

이 강의에서 익힌 도구는 각각 다른 범위를 검사합니다. 작업을 마칠 때 어떤 환경에서 어떤 코드와 동작을 확인했는지 설명할 수 있다면, 도구의 성공 여부를 넘어 변경을
믿을 근거를 남길 수 있습니다.
-->
