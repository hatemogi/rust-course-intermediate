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

---
level: 2
---

# 도구마다 쌓는 증거가 다릅니다

<div class="tool-flow">
  <div><strong>정적 확인</strong>형식·타입·소유권·린트<sub>lint</sub></div>
  <div><strong>동작 확인</strong>단위·통합·문서 테스트</div>
  <div><strong>사용법 확인</strong>API 문서</div>
</div>

<div class="takeaway">앞 단계가 빠르게 실패할수록 뒤의 비싼 검사를 실행하기 전에 문제를 좁힐 수 있습니다.</div>

---
level: 2
---

# 전체 흐름은 빠른 검사부터 쌓습니다

<ol class="step-list">
  <li>선택된 툴체인<sub>toolchain</sub>을 확인합니다.</li>
  <li>형식과 클리피<sub>Clippy</sub>를 검사합니다.</li>
  <li>지원하는 피처<sub>feature</sub> 구성을 컴파일합니다.</li>
  <li>단위·통합·문서 테스트를 실행합니다.</li>
  <li>API 문서를 생성하고 직접 읽습니다.</li>
</ol>

---
level: 2
---

# 저장소의 `make check`가 순서를 고정합니다

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

---
level: 2
---

# 종합 실습에서는 공개 함수 하나를 완성합니다

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

---
level: 2
---

# 준비된 통합 테스트부터 실행합니다

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

---
level: 2
---

# 실패를 확인한 뒤 구현과 사용 예제를 완성합니다

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

---
level: 2
---

# 지원하는 세 피처 구성을 실행합니다

```bash
cargo run --example 13_features
cargo run --example 13_features --features binary-search

cargo check --locked
cargo check --no-default-features --locked
cargo check --all-features --locked
```

<div class="takeaway"><code>--all-features</code>에서는 피처를 껐을 때만 들어오는 코드가 빠질 수 있습니다.</div>

---
level: 2
---

# 문서의 설명과 예제도 검증 대상입니다

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

---
level: 2
---

# 세 파일에 네 결과물을 남깁니다

<div class="tool-flow">
  <div><strong>공개 구현</strong><code>src/lib.rs</code></div>
  <div><strong>통합 테스트</strong><code>tests/exercises.rs</code></div>
  <div><strong>문서와 실행 예제</strong><code>src/lib.rs</code><br><code>examples/21_exercises.rs</code></div>
</div>

<div class="takeaway"><code>git diff</code>에서 구현, 테스트, 문서, 실행 예제를 함께 확인합니다.</div>

---
level: 2
---

# AI가 제안한 다른 구현도 같은 기준으로 검토합니다

- 빈 이름을 제외하고 대소문자를 합치는가?
- 소유한 `String`을 불필요하게 반복해서 만드는가?
- 기존 함수를 재사용하는 방식과 직접 순회하는 방식의 차이를 설명하는가?
- 테스트를 없애거나 린트를 `allow`로 덮지 않는가?
- 적용 뒤 통합 테스트와 문서 테스트가 통과하는가?

---
level: 2
---

# 완료 조건을 하나씩 확인합니다

<ul class="check-list">
  <li>섞인 입력과 빈 입력 테스트가 통과합니다.</li>
  <li>문서 예제가 문서 테스트에서 실행됩니다.</li>
  <li>실행 예제에서 이름별 횟수를 확인할 수 있습니다.</li>
  <li>기본 피처를 끈 구성과 모든 피처 구성이 통과합니다.</li>
  <li>rustfmt와 클리피가 지적할 사항이 없습니다.</li>
</ul>

---
level: 2
---

# 마지막에는 한 명령으로 반복합니다

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

---
level: 2
---

# 도구 하나가 다른 도구를 대신하지 않습니다

<table class="compare">
  <tbody>
    <tr><td>rustfmt를 통과함</td><td>잘못된 값을 계산할 수 있음</td></tr>
    <tr><td>클리피 경고가 없음</td><td>경계 조건에서 실패할 수 있음</td></tr>
    <tr><td>테스트를 통과함</td><td>검사하지 않은 입력에서는 실패할 수 있음</td></tr>
  </tbody>
</table>

---
level: 2
layout: center
class: section
---

# 마무리 퀴즈

각 문제에서 가장 알맞은 답을 하나 고른 뒤 클릭해서 풀이를 확인합니다.

---
level: 2
---

# 1. 도구의 역할

<div class="quiz-options">
  <div><strong>①</strong> <code>rustup</code>은 Rust 소스 코드를 컴파일합니다.</div>
  <div><strong>②</strong> <code>rustc</code>는 패키지 의존성을 관리합니다.</div>
  <div><strong>③</strong> 카고<sub>Cargo</sub>는 패키지와 의존성을 관리하고 빌드와 테스트를 실행합니다.</div>
  <div><strong>④</strong> <code>cargo update</code>는 Rust 툴체인을 업데이트합니다.</div>
</div>

<div v-click class="takeaway"><strong>정답: ③</strong> rustup은 툴체인을 관리하고, rustc는 소스 코드를 컴파일합니다.</div>

---
level: 2
---

# 2. 두 Rust 버전 설정

<div class="quiz-options">
  <div><strong>①</strong> 두 설정은 항상 같은 버전이어야 합니다.</div>
  <div><strong>②</strong> <code>rust-toolchain.toml</code>은 사용할 툴체인, <code>rust-version</code>은 가장 오래된 지원 버전을 나타냅니다.</div>
  <div><strong>③</strong> <code>rust-version</code>이 rustup의 기본값을 바꿉니다.</div>
  <div><strong>④</strong> <code>rust-toolchain.toml</code>이 의존성 버전을 고정합니다.</div>
</div>

<div v-click class="takeaway"><strong>정답: ②</strong> 두 설정은 대상과 목적이 다르므로 반드시 같을 필요가 없습니다.</div>

---
level: 2
---

# 3. 모든 카고 타깃 검사

<div class="quiz-options">
  <div><strong>①</strong> <code>cargo check --all-targets --locked</code>를 실행하고 첫 번째 오류부터 읽습니다.</div>
  <div><strong>②</strong> <code>cargo run</code>을 실행하고 마지막 오류부터 읽습니다.</div>
  <div><strong>③</strong> <code>cargo fmt --check</code>만 실행합니다.</div>
  <div><strong>④</strong> <code>rustc src/lib.rs</code>만 실행합니다.</div>
</div>

<div v-click class="takeaway"><strong>정답: ①</strong> 예제와 테스트도 타깃이며, 뒤의 오류는 첫 오류에서 이어졌을 수 있습니다.</div>

---
level: 2
---

# 4. 형식과 클리피 검사

<div class="quiz-options">
  <div><strong>①</strong> CI에서 <code>cargo fmt</code>로 파일을 고칩니다.</div>
  <div><strong>②</strong> 모든 클리피 경고를 프로젝트 전체에서 허용합니다.</div>
  <div><strong>③</strong> <code>cargo fmt --check</code>를 사용하고 클리피 제안은 목적과 테스트를 살펴본 뒤 적용합니다.</div>
  <div><strong>④</strong> rustfmt를 통과하면 동작도 올바릅니다.</div>
</div>

<div v-click class="takeaway"><strong>정답: ③</strong> rustfmt는 형식을 검사하고, 클리피는 판단할 제안을 보여 줍니다.</div>

---
level: 2
---

# 5. 자동 수정 뒤의 작업

<div class="quiz-options">
  <div><strong>①</strong> 명령이 성공하면 바로 배포합니다.</div>
  <div><strong>②</strong> diff를 읽고 피처 검사와 테스트를 다시 실행합니다.</div>
  <div><strong>③</strong> 바뀐 파일을 확인하지 않고 모두 스테이징합니다.</div>
  <div><strong>④</strong> 같은 경고가 나오지 않도록 모든 린트를 허용합니다.</div>
</div>

<div v-click class="takeaway"><strong>정답: ②</strong> 자동 수정도 코드 변경이므로 범위와 동작을 다시 검증합니다.</div>

---
level: 2
---

# 6. 의존성이 들어온 경로

직접 추가하지 않은 `serde`가 어떤 경로로 들어왔는지 찾는 명령은 무엇인가요?

<div class="quiz-options">
  <div><strong>①</strong> <code>cargo update -p serde</code></div>
  <div><strong>②</strong> <code>cargo remove serde</code></div>
  <div><strong>③</strong> <code>cargo tree -i serde</code></div>
  <div><strong>④</strong> <code>rustup component list --installed</code></div>
</div>

<div v-click class="takeaway"><strong>정답: ③</strong> <code>-i</code>는 지정한 패키지를 사용하는 상위 의존성을 거꾸로 보여 줍니다.</div>

---
level: 2
---

# 7. `--all-features`의 범위

<div class="quiz-options">
  <div><strong>①</strong> 모든 운영체제에서 모든 코드를 검사합니다.</div>
  <div><strong>②</strong> 모든 카고 타깃과 모든 피처 조합을 검사합니다.</div>
  <div><strong>③</strong> 모든 피처를 함께 켜지만 피처를 껐을 때만 들어오는 코드는 빠질 수 있습니다.</div>
  <div><strong>④</strong> 기본 피처를 끈 구성까지 모두 검사합니다.</div>
</div>

<div v-click class="takeaway"><strong>정답: ③</strong> 타깃 범위와 지원하는 피처 구성은 별도로 지정해 검사합니다.</div>

---
level: 2
---

# 8. 문서의 예제와 설명

<div class="quiz-options">
  <div><strong>①</strong> 문서 테스트만 실행하면 설명의 가독성도 알 수 있습니다.</div>
  <div><strong>②</strong> <code>cargo doc</code>만 실행하면 예제의 동작도 알 수 있습니다.</div>
  <div><strong>③</strong> <code>cargo test --doc</code>로 예제를 검사하고 생성한 API 문서도 직접 읽습니다.</div>
  <div><strong>④</strong> 비공개 함수의 단위 테스트만 실행합니다.</div>
</div>

<div v-click class="takeaway"><strong>정답: ③</strong> 실행되는 예제와 사람이 읽는 설명은 서로 다른 방법으로 확인합니다.</div>

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

---
level: 2
layout: center
class: section
---

# 도구의 출력을 따르지 말고<br>근거를 연결해 결정하세요

환경, diff, 테스트 결과가 함께 있을 때 코드를 믿을 수 있는 범위를 설명할 수
있습니다.
