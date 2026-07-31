---
theme: default
title: cargo check와 컴파일 오류 읽기
info: 우아한 Rust 중급 개발 도구 2편
colorSchema: dark
aspectRatio: 16/9
canvasWidth: 1100
transition: fade-out
lineNumbers: true
layout: cover
class: cover
---

<div class="episode-label">우아한 Rust 중급 · 개발 도구 02</div>

# `cargo check`와<br>컴파일 오류 읽기

검사할 카고<sub>Cargo</sub> 타깃을 정하고 첫 번째 오류부터 원인을 찾습니다.

---
level: 2
---

# 세 명령은 결과물이 다릅니다

<table class="compare">
  <thead>
    <tr><th>명령</th><th>주된 목적</th><th>실행 파일</th></tr>
  </thead>
  <tbody>
    <tr><td><code>cargo check</code></td><td>빠르게 컴파일 가능 여부 확인</td><td>만들지 않음</td></tr>
    <tr><td><code>cargo build</code></td><td>실행하거나 배포할 파일 생성</td><td>만듦</td></tr>
    <tr><td><code>cargo run</code></td><td>빌드한 뒤 바로 실행</td><td>만들고 실행함</td></tr>
  </tbody>
</table>

<div class="takeaway">코드를 작성하는 동안에는 실행 파일이 필요하지 않은 검사가 더 빠릅니다.</div>

---
level: 2
---

# 한 패키지에는 여러 타깃이 들어갑니다

<div class="tool-flow">
  <div><strong>라이브러리·실행 파일</strong><code>src/lib.rs</code><br><code>src/main.rs</code></div>
  <div><strong>학습·검증 코드</strong><code>examples/*.rs</code><br><code>tests/*.rs</code></div>
</div>

<div class="takeaway">기본 <code>cargo check</code>는 라이브러리와 실행 파일 타깃만 검사합니다.</div>

---
level: 2
---

# 강의 저장소는 모든 타깃을 검사합니다

```bash
cargo check --all-targets --locked
```

- `--all-targets`: 라이브러리, 실행 파일, 예제, 테스트를 검사합니다.
- `Cargo.lock`: 카고가 선택한 직접·간접 의존성의 정확한 버전과 출처를 기록합니다.
- `--locked`: 다른 의존성 선택이 필요하면 `Cargo.lock`을 바꾸지 않고 실패합니다.

필요한 범위만 좁힐 수도 있습니다.

```bash
cargo check --lib
cargo check --examples
cargo check --tests
```

---
level: 2
---

# 오류가 여러 개여도 첫 번째부터 읽습니다

<div class="diagnostic-anatomy">
  <div class="diagnostic-output" aria-label="moved_value.rs에서 발생한 rustc E0382 진단">
    <div class="diag-line diag-error"><span>1</span><code>error[E0382]: borrow of moved value: `message`</code></div>
    <div class="diag-line"><span>2</span><code> --&gt; moved_value.rs:4:23</code></div>
    <div class="diag-source"><code>3 | print_message(message);</code><em>value moved here</em></div>
    <div class="diag-source"><code>4 | println!("다시 출력: {message}");</code></div>
    <div class="diag-line diag-caret">
      <span>3</span>
      <code aria-label="message 아래의 오류 표시"><i aria-hidden="true">4 | println!("다시 출력: {</i>^^^^^^^</code>
    </div>
    <div class="diag-line diag-note"><span>4</span><code>note: parameter takes ownership of the value</code></div>
    <div class="diag-line diag-help"><span>5</span><code>help: consider borrowing or cloning the value</code></div>
  </div>

  <ol class="diagnostic-key">
    <li><strong>오류</strong><span>코드와 한 줄 설명</span></li>
    <li><strong>위치</strong><span>파일과 줄·열</span></li>
    <li><strong>표시</strong><span>문제가 된 값</span></li>
    <li><strong>비고</strong><span>앞서 일어난 이동</span></li>
    <li><strong>도움말</strong><span>의도와 맞는지 검토할 제안</span></li>
  </ol>
</div>

<div class="visual-caption">이 예제의 실제 진단을 위에서 아래로 읽습니다.</div>

---
level: 2
---

# 이동한 값을 다시 사용하면 `E0382`가 발생합니다

<<< ../../examples/compile_fail/moved_value.rs rust {2|4|6}{lines:true}

<div class="command text-sm">rustc examples/compile_fail/moved_value.rs</div>

<div v-click class="takeaway"><code>print_message</code>가 <code>String</code>을 값으로 받으면서 소유권이 이동합니다.</div>

<!--
코드를 먼저 보여 주고 어느 줄에서 이동하는지 질문합니다.
rust-course-tooling 디렉터리에서 명령을 실행합니다. 이 예제는 컴파일 실패가
목적이므로 실행 파일이 만들어지지 않고 E0382가 나오면 정상입니다.
그 다음 실제 rustc 오류로 전환해 비고와 도움말을 함께 읽습니다.
-->

---
level: 2
---

# 오류 설명은 더 자세히 읽을 수 있습니다

<div class="command">rustc --explain E0382</div>

오류 코드 설명은 문법적인 규칙을 이해하는 데 도움이 됩니다. 하지만 어느 해결책이
프로그램의 요구 사항에 맞는지는 정해 주지 않습니다.

<div class="grid grid-cols-2 gap-10 mt-8">
  <div><h3>읽기만 한다면</h3><code>&str</code>이나 <code>&String</code>을 받도록 바꿉니다.</div>
  <div><h3>함수가 보관해야 한다면</h3>호출 뒤 값을 다시 쓰지 않거나 소유 구조를 다시 설계합니다.</div>
</div>

---
level: 2
---

# `clone()`은 마지막 답이 아니라 설계 선택입니다

- 복제가 실제 요구 사항인가?
- 함수가 소유권을 가져가야 하는가?
- 호출자는 이후에도 같은 값을 사용해야 하는가?
- 공개 API와 오류 처리 방식이 달라지는가?

<div class="takeaway">컴파일러가 제안한 수정도 비용과 의도를 사람이 검토해야 합니다.</div>

---
level: 2
---

# AI에는 오류 한 줄보다 재현 조건을 전달합니다

<ol class="step-list">
  <li>최소 재현 코드</li>
  <li>실행한 정확한 명령</li>
  <li>첫 번째 오류의 전체 내용</li>
  <li>프로그램이 원래 해야 하는 일</li>
</ol>

수정안에 불필요한 `clone()`, `unwrap()`, 넓은 `allow`가 들어오지 않았는지도 다시
확인합니다.

---
level: 2
---

# 수정 뒤에는 같은 명령으로 다시 확인합니다

<div class="command">cargo check --all-targets --locked</div>

<div class="question">컴파일 검사를 통과했다면 요구 사항대로 동작한다고 말할 수 있을까요?</div>

<!--
답은 아직 아니라고 정리합니다.
다음 편의 rustfmt와 이후의 클리피<sub>Clippy</sub>, 테스트가 서로 다른 질문에
답한다는 흐름으로 연결합니다.
-->
