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

---
level: 2
---

# 클리피는 컴파일되는 코드에서 검토할 부분을 찾습니다

- 실수일 가능성이 있는 표현
- 불필요하게 복잡한 코드
- 더 자연스럽게 표현할 수 있는 Rust 코드

<div class="command">cargo clippy</div>

<div class="takeaway compact">클리피<sub>Clippy</sub>는 컴파일되는 코드를 분석하는 공식 린트 도구이지만 프로그램의 요구 사항까지 알지는 못합니다.</div>

---
level: 2
---

# 검사 범위와 실패 기준을 명시합니다

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

---
level: 2
---

# 클리피는 이어진 호출을 하나의 의도로 묶습니다

<<< ../../examples/07_clippy.rs rust {1-6|4-5}{lines:true}

<div class="command">cargo clippy --example 07_clippy --locked</div>

<div class="takeaway"><code>map_all_any_identity</code>: <code>map</code>과 <code>any</code>를 하나의 <code>any</code>로 합칩니다.</div>

---
level: 2
---

# 린트 하나를 네 단계로 판단합니다

<ol class="step-list">
  <li>경고하는 실제 위험이나 낭비를 이해합니다.</li>
  <li>제안대로 바꿔도 동작과 공개 API가 같은지 확인합니다.</li>
  <li>관련 테스트를 실행합니다.</li>
  <li>적용하지 않으면 좁은 범위에 이유를 남깁니다.</li>
</ol>

---
level: 2
---

# 교육용 코드는 일부러 단계를 보여 줄 수 있습니다

<<< ../../examples/08_clippy_allow.rs rust {1-7|1}{lines:true}

<div class="takeaway">허용이 필요하다면 함수나 모듈처럼 가장 작은 범위에 두고 이유를 적습니다.</div>

---
level: 2
---

# 경고를 숨기는 것은 해결이 아닙니다

```rust
#![allow(clippy::all)]
```

이 코드는 원래 경고가 왜 나왔는지, 어느 제안이 부적절했는지 전혀 설명하지 않습니다.
AI가 이런 수정을 제안했다면 원래 린트를 다시 실행해 하나씩 판단합니다.

---
level: 2
---

# 첫 검증 흐름은 세 질문으로 시작합니다

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

---
level: 2
---

# 통과한 명령마다 확인하지 못한 것이 남습니다

<table class="compare">
  <thead><tr><th>명령</th><th>확인한 내용</th><th>아직 모르는 내용</th></tr></thead>
  <tbody>
    <tr><td><code>cargo check</code></td><td>모든 타깃을 컴파일할 수 있음</td><td>요구 사항대로 동작함</td></tr>
    <tr><td><code>cargo fmt --check</code></td><td>Rust 코드 형식이 맞음</td><td>이름과 설계가 알맞음</td></tr>
    <tr><td><code>cargo clippy</code></td><td>선택한 린트가 남지 않음</td><td>모든 입력에서 올바름</td></tr>
  </tbody>
</table>

---
level: 2
---

# 실패와 판단을 짧게 기록합니다

```text
- 사용한 툴체인<sub>toolchain</sub>:
- 처음 실패한 명령:
- 진단이 알려 준 사실:
- 선택한 수정:
- 채택하지 않은 수정과 이유:
- 다시 실행해 통과한 명령:
```

이 기록은 자동 수정의 diff와 테스트 결과를 검토할 때 판단 근거가 됩니다.

---
level: 2
layout: center
class: section
---

# 컴파일과 린트를 통과한 코드가<br>요구 사항대로 동작하는지는 어떻게 확인할까요?

전체 과정에서는 편집기 자동화, 의존성, 문서, 테스트를 하나의 검증 흐름으로
연결합니다.
