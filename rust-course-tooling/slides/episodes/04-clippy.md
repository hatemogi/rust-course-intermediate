---
theme: default
title: 클리피 판단과 첫 검증 실습
info: 우아한 Rust 중급 개발 도구 4편
colorSchema: dark
aspectRatio: 16/9
canvasWidth: 1100
transition: fade-out
lineNumbers: true
layout: cover
class: cover
---

<div class="episode-label">우아한 Rust 중급 · 개발 도구 04</div>

# 클리피 판단과<br>첫 검증 실습

린트<sub>lint</sub>를 검토하고 세 검사 명령을 하나의 흐름으로 연결합니다.

<!--
교재 대응: book/src/practice/19-free-workflow.md > 검증 흐름 실습

이제껏 배운 도구들을 한 번에 연결해 봅니다. 이 실습의 목표는 모든 명령을 외우는 것이 아니라, 문제가 생겼을 때 어느 단계로 돌아가야 하는지 판단해보는 것입니다.
프로젝트 환경을 먼저 확인하려면 `rustup show`와 `rustup component list --installed`를 실행합니다. 선택된 툴체인이 `rust-toolchain.toml`과
맞는지 확인하고, rustfmt와 클리피가 없다면 필요한 컴포넌트만 설치합니다.

교재 대응: book/src/practice/19-free-workflow.md > 1. 프로젝트 환경 확인하기
-->

---
level: 2
---

# 클리피 실행하고 결과 읽기

- 실수일 가능성이 있는 표현
- 불필요하게 복잡한 코드
- 더 자연스럽게 표현할 수 있는 Rust 코드

<div class="command">cargo clippy</div>

<div class="takeaway compact">클리피<sub>Clippy</sub>는 컴파일되는 코드를 분석하는 공식 린트 도구이지만 프로그램의 요구 사항까지 알지는 못합니다.</div>

<!--
교재 대응: book/src/clippy/07-running-clippy.md > 클리피 실행하고 결과 읽기

클리피(Clippy)는 컴파일되는 Rust 코드에서 실수일 가능성이 있는 표현, 불필요하게 복잡한 코드와 더 자연스러운 Rust 표현을 찾는 공식 린트(lint)
도구입니다. 클리피는 clippy::map_all_any_identity 린트를 표시하고 두 호출을 다음과 같이 any 하나로 합치라고 제안합니다. 참고: 린트 도구란?
린트 도구는 코드를 실행하지 않고 분석하여, 컴파일은 되지만 실수하기 쉽거나 불필요하게 복잡한 부분을 찾아냅니다. 이때 적용하는 검사 규칙 하나하나를 린트라고 부릅니다.
린트를 모두 통과해도 프로그램이 요구 사항대로 동작한다는 뜻은 아니므로 테스트도 함께 실행해야 합니다.
-->

---
level: 2
---

# 클리피 실행하고 결과 읽기

```bash
cargo clippy --all-targets --locked -- -D warnings
```

<table class="compare">
  <tbody>
    <tr><td><code>--all-targets</code></td><td>예제와 테스트까지 검사</td></tr>
    <tr><td><code>--locked</code></td><td>잠금 파일을 바꿔야 하면 실패</td></tr>
    <tr><td><code>--</code></td><td>뒤의 옵션을 rustc와 클리피에 전달</td></tr>
    <tr><td><code>-D warnings</code></td><td>경고를 검사 실패로 취급</td></tr>
  </tbody>
</table>

<!--
교재 대응: book/src/clippy/07-running-clippy.md > 클리피 실행하고 결과 읽기

클리피(Clippy)는 컴파일되는 Rust 코드에서 실수일 가능성이 있는 표현, 불필요하게 복잡한 코드와 더 자연스러운 Rust 표현을 찾는 공식 린트(lint)
도구입니다. • --all-targets는 카고(Cargo)의 모든 타깃을 검사합니다. • --locked는 Cargo.lock을 임의로 바꾸지 못하게 합니다. • --
-D warnings는 경고를 검사 실패로 취급합니다. -- 뒤의 옵션은 rustc와 클리피에 전달됩니다. 클리피는 clippy::map_all_any_identity
린트를 표시하고 두 호출을 다음과 같이 any 하나로 합치라고 제안합니다.
-->

---
level: 2
---

# 클리피 실행하고 결과 읽기

<<< ../../examples/07_clippy.rs rust {1-6|4-5}{lines:true}

<div class="command">cargo clippy --example 07_clippy --locked</div>

<div class="takeaway"><code>map_all_any_identity</code>: <code>map</code>과 <code>any</code>를 하나의 <code>any</code>로 합칩니다.</div>

<!--
교재 대응: book/src/clippy/07-running-clippy.md > 클리피 실행하고 결과 읽기

클리피(Clippy)는 컴파일되는 Rust 코드에서 실수일 가능성이 있는 표현, 불필요하게 복잡한 코드와 더 자연스러운 Rust 표현을 찾는 공식 린트(lint)
도구입니다. • --all-targets는 카고(Cargo)의 모든 타깃을 검사합니다. • --locked는 Cargo.lock을 임의로 바꾸지 못하게 합니다. • --
-D warnings는 경고를 검사 실패로 취급합니다. -- 뒤의 옵션은 rustc와 클리피에 전달됩니다. 클리피는 clippy::map_all_any_identity
린트를 표시하고 두 호출을 다음과 같이 any 하나로 합치라고 제안합니다.
-->

---
level: 2
---

# 클리피 제안을 판단하는 기준

<ol class="step-list">
  <li>경고하는 실제 위험이나 낭비를 이해합니다.</li>
  <li>제안대로 바꿔도 동작과 공개 API가 같은지 확인합니다.</li>
  <li>관련 테스트를 실행합니다.</li>
  <li>적용하지 않으면 좁은 범위에 이유를 남깁니다.</li>
</ol>

<!--
교재 대응: book/src/clippy/08-judging-lints.md > 클리피 제안을 판단하는 기준

클리피(Clippy)는 검토할 부분을 알려 주지만 프로그램의 요구 사항까지 알지는 못합니다. 다음 순서로 제안을 적용할지 판단합니다. 린트(lint)가 경고하는 실제
위험이나 낭비가 무엇인지 이해합니다. 제안대로 바꿨을 때 동작과 공개 API가 그대로인지 확인합니다. 관련 테스트를 실행합니다. 적용하지 않는다면 가능한 한 좁은
범위에서 허용하고 이유를 적습니다. 파일 전체나 프로젝트 전체에서 린트를 끄기 전에 함수나 모듈처럼 더 작은 범위에서 허용할 수 있는지 확인하세요. 이유 없는
allow는 이후의 실제 문제까지 숨깁니다.
-->

---
level: 2
---

# 클리피 제안을 판단하는 기준

<<< ../../examples/08_clippy_allow.rs rust {1-7|1}{lines:true}

<div class="takeaway">허용이 필요하다면 함수나 모듈처럼 가장 작은 범위에 두고 이유를 적습니다.</div>

<!--
교재 대응: book/src/clippy/08-judging-lints.md > 클리피 제안을 판단하는 기준

린트(lint)가 경고하는 실제 위험이나 낭비가 무엇인지 이해합니다. 제안대로 바꿨을 때 동작과 공개 API가 그대로인지 확인합니다. 관련 테스트를 실행합니다. 적용하지
않는다면 가능한 한 좁은 범위에서 허용하고 이유를 적습니다. 예를 들어 Vec::new() 뒤에 push를 연이어 호출한 코드는 vec![]로 줄일 수 있습니다. 그러나
Vec::push를 처음 설명하는 교육용 예제라면 원래 코드가 목적에 더 맞을 수 있습니다. 파일 전체나 프로젝트 전체에서 린트를 끄기 전에 함수나 모듈처럼 더 작은
범위에서 허용할 수 있는지 확인하세요. 이유 없는 allow는 이후의 실제 문제까지 숨깁니다.
-->

---
level: 2
---

# AI 코드 검토하기

```rust
#![allow(clippy::all)]
```

이 코드는 원래 경고가 왜 나왔는지, 어느 제안이 부적절했는지 전혀 설명하지 않습니다.
AI가 이런 수정을 제안했다면 원래 린트를 다시 실행해 하나씩 판단합니다.

<!--
교재 대응: book/src/clippy/08-judging-lints.md > AI 코드 검토하기

AI가 클리피 경고를 없애기 위해 #[allow(clippy::all)]을 추가했다면 경고가 해결된 것이 아닙니다. 원래 린트를 다시 실행하고, 각각 수정할지 제한적으로
허용할지 나누어 판단하세요.
-->

---
level: 2
---

# 2. 컴파일 오류 설명하기

```bash
cargo check --all-targets --locked
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
```

<div class="tool-flow">
  <div><strong>컴파일</strong>타입과 소유권 규칙을 만족하는가?</div>
  <div><strong>형식</strong>정해진 코드 형식을 따르는가?</div>
  <div><strong>린트</strong>선택한 범위에 의심스러운 표현이 남았는가?</div>
</div>

<!--
교재 대응: book/src/practice/19-free-workflow.md > 2. 컴파일 오류 설명하기

examples/compile_fail/moved_value.rs는 컴파일 오류를 살펴보기 위해 일부러 잘못된 코드를 넣은 예제입니다. 소유권은 어느 호출에서
이동하는가? 컴파일러는 어느 줄의 사용을 오류로 표시하는가? print_message가 문자열을 읽기만 한다면 매개변수 타입을 어떻게 바꿀 수 있는가? clone()을
추가하는 방법이 이 예제에 알맞지 않은 이유는 무엇인가?
-->

---
level: 2
---

# 3. 프로젝트 전체 검사하기

<table class="compare">
  <thead><tr><th>명령</th><th>확인한 내용</th><th>아직 모르는 내용</th></tr></thead>
  <tbody>
    <tr><td><code>cargo check</code></td><td>모든 타깃을 컴파일할 수 있음</td><td>요구 사항대로 동작함</td></tr>
    <tr><td><code>cargo fmt --check</code></td><td>Rust 코드 형식이 맞음</td><td>이름과 설계가 알맞음</td></tr>
    <tr><td><code>cargo clippy</code></td><td>선택한 린트가 남지 않음</td><td>모든 입력에서 올바름</td></tr>
  </tbody>
</table>

<!--
교재 대응: book/src/practice/19-free-workflow.md > 3. 프로젝트 전체 검사하기

각 명령이 통과하면 다음 사실을 확인한 것입니다. 어떤 명령이 실패했다면 첫 번째 진단 사항부터 확인해 수정한 뒤 같은 명령을 다시 실행합니다. 수정한 내용은 git
diff로 확인하고, 실패했던 명령을 다시 실행해 문제가 해결됐는지 확인합니다.
-->
