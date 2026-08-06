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

<!--
교재 대응: book/src/compilation/04-reading-errors.md > 컴파일 오류 읽기; book/src/compilation/04-reading-errors.md > 첫 번째 오류부터 읽기

Rust 컴파일 오류에는 실패했다는 사실뿐 아니라 컴파일러가 확인한 위치, 관련된 값과 가능한 수정 방법이 함께 표시됩니다. AI에게 바로 수정을 맡기기 전에 오류
메시지가 알려 주는 내용을 먼저 이해합니다.
하나의 잘못된 타입이나 문법 때문에 뒤에서 여러 오류가 연달아 생길 수 있습니다. 출력된 오류 수만큼 서로 다른 문제가 있다고 단정하지 말고 첫 번째 오류부터 고친 뒤
다시 cargo check를 실행하세요. error[E....]의 오류 코드와 한 줄 설명을 읽습니다. 화살표가 가리키는 파일과 줄을 찾습니다. 밑줄과 함께 표시된
값이나 타입을 확인합니다. 비고에서 앞선 이동, 빌림이나 타입 결정 위치를 찾습니다. 도움말의 제안이 프로그램의 의도와 맞는지 판단합니다.
-->

---
level: 2
---

# `cargo check`와 카고 타깃

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

<!--
교재 대응: book/src/compilation/03-cargo-check.md > cargo check와 카고 타깃

cargo check는 Rust 코드를 컴파일할 수 있는지 검사하지만 최종 실행 파일은 만들지 않습니다. 코드 작성 중에 타입 오류와 소유권 오류를 빠르게 확인할 때
사용합니다. 실행 파일이 필요하면 cargo build, 프로그램을 바로 실행하려면 cargo run을 사용합니다. 세 명령 모두 컴파일 과정을 거치지만 목적이
다릅니다.
-->

---
level: 2
---

# 카고 타깃

<div class="tool-flow">
  <div><strong>라이브러리·실행 파일</strong><code>src/lib.rs</code><br><code>src/main.rs</code></div>
  <div><strong>학습·검증 코드</strong><code>examples/*.rs</code><br><code>tests/*.rs</code></div>
</div>

<div class="takeaway">기본 <code>cargo check</code>는 라이브러리와 실행 파일 타깃만 검사합니다.</div>

<!--
교재 대응: book/src/compilation/03-cargo-check.md > 카고 타깃

한 패키지에는 여러 종류의 컴파일 대상이 들어갈 수 있습니다. 카고(Cargo)에서는 이를 타깃이라고 부릅니다. • src/lib.rs: 라이브러리 타깃 •
src/main.rs, src/bin/*.rs: 실행 파일 타깃 • examples/*.rs: 예제 타깃 • tests/*.rs: 통합 테스트 타깃 기본 cargo
check는 라이브러리와 실행 파일 타깃만 검사합니다. 강의 저장소에서는 예제와 통합 테스트도 항상 컴파일되는지 확인하기 위해 다음 명령을 사용합니다.
-->

---
level: 2
---

# 카고 타깃

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

<!--
교재 대응: book/src/compilation/03-cargo-check.md > 카고 타깃

• src/lib.rs: 라이브러리 타깃 • src/main.rs, src/bin/*.rs: 실행 파일 타깃 • examples/*.rs: 예제 타깃 •
tests/*.rs: 통합 테스트 타깃 기본 cargo check는 라이브러리와 실행 파일 타깃만 검사합니다. 강의 저장소에서는 예제와 통합 테스트도 항상 컴파일되는지
확인하기 위해 다음 명령을 사용합니다. 참고: Cargo.lock Cargo.lock은 카고가 실제로 선택한 직접·간접 의존성의 정확한 버전과 출처를 기록하는
파일입니다. 카고가 이 파일을 만들고 업데이트하므로 직접 편집하지 않습니다.
-->

---
level: 2
---

# 첫 번째 오류부터 읽기

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

<!--
교재 대응: book/src/compilation/03-cargo-check.md > 판단하기; book/src/compilation/04-reading-errors.md > 첫 번째 오류부터 읽기

예제 코드를 교재에 넣었지만 `cargo check --all-targets`를 실행하지 않았다면 예제와 테스트 타깃의 컴파일 오류를 놓칠 수 있습니다. 또한 컴파일 검사를
통과한 코드가 테스트까지 통과한다고 말할 수는 없습니다.
하나의 잘못된 타입이나 문법 때문에 뒤에서 여러 오류가 연달아 생길 수 있습니다. 출력된 오류 수만큼 서로 다른 문제가 있다고 단정하지 말고 첫 번째 오류부터 고친 뒤
다시 cargo check를 실행하세요. error[E....]의 오류 코드와 한 줄 설명을 읽습니다. 화살표가 가리키는 파일과 줄을 찾습니다. 밑줄과 함께 표시된
값이나 타입을 확인합니다. 비고에서 앞선 이동, 빌림이나 타입 결정 위치를 찾습니다. 도움말의 제안이 프로그램의 의도와 맞는지 판단합니다.
-->

---
level: 2
---

# 이동한 값을 다시 사용한 예제

<<< ../../examples/compile_fail/moved_value.rs rust {2|4|6}{lines:true}

<div class="command text-sm">rustc examples/compile_fail/moved_value.rs</div>

<div v-click class="takeaway"><code>print_message</code>가 <code>String</code>을 값으로 받으면서 소유권이 이동합니다.</div>

<!--
코드를 먼저 보여 주고 어느 줄에서 이동하는지 질문합니다.
rust-course-tooling 디렉터리에서 명령을 실행합니다. 이 예제는 컴파일 실패가
목적이므로 실행 파일이 만들어지지 않고 E0382가 나오면 정상입니다.
그 다음 실제 rustc 오류로 전환해 비고와 도움말을 함께 읽습니다.
-->

<!--
교재 대응: book/src/compilation/04-reading-errors.md > 이동한 값을 다시 사용한 예제

이 예제는 컴파일에 실패하도록 작성했으므로 실행 파일이 만들어지지 않고 명령도 실패 상태로 끝납니다. 이어서 출력되는 E0382 오류를 읽는 것이 이 예제의 목적입니다.
print_message가 String을 값으로 받으므로 message의 소유권이 함수로 이동합니다. 뒤의 println!에서 같은 값을 다시 사용하면 컴파일러는
E0382를 보고합니다. 해결책은 하나가 아닙니다. print_message가 값을 소유해야 하는지 먼저 결정해야 합니다. 읽기만 하면 된다면 &str이나
&String을 받도록 바꿀 수 있고, 함수가 값을 보관해야 한다면 호출자가 이후에 사용하지 않도록 설계를 바꾸거나 명시적으로 복제할 수 있습니다. 컴파일러가
clone을 제안하더라도 복제가 요구 사항에 맞는지는 사람이 판단해야 합니다.
-->

---
level: 2
---

# 이동한 값을 다시 사용한 예제

<div class="command">rustc --explain E0382</div>

오류 코드 설명은 문법적인 규칙을 이해하는 데 도움이 됩니다. 하지만 어느 해결책이
프로그램의 요구 사항에 맞는지는 정해 주지 않습니다.

<div class="grid grid-cols-2 gap-10 mt-8">
  <div><h3>읽기만 한다면</h3><code>&str</code>이나 <code>&String</code>을 받도록 바꿉니다.</div>
  <div><h3>함수가 보관해야 한다면</h3>호출 뒤 값을 다시 쓰지 않거나 소유 구조를 다시 설계합니다.</div>
</div>

<!--
교재 대응: book/src/compilation/04-reading-errors.md > 이동한 값을 다시 사용한 예제

print_message가 String을 값으로 받으므로 message의 소유권이 함수로 이동합니다. 뒤의 println!에서 같은 값을 다시 사용하면 컴파일러는
E0382를 보고합니다. 다음 명령으로 오류 코드의 자세한 설명을 읽을 수 있습니다. 해결책은 하나가 아닙니다. print_message가 값을 소유해야 하는지 먼저
결정해야 합니다. 읽기만 하면 된다면 &str이나 &String을 받도록 바꿀 수 있고, 함수가 값을 보관해야 한다면 호출자가 이후에 사용하지 않도록 설계를 바꾸거나
명시적으로 복제할 수 있습니다. 컴파일러가 clone을 제안하더라도 복제가 요구 사항에 맞는지는 사람이 판단해야 합니다.
-->

---
level: 2
---

# 이동한 값을 다시 사용한 예제

- 복제가 실제 요구 사항인가?
- 함수가 소유권을 가져가야 하는가?
- 호출자는 이후에도 같은 값을 사용해야 하는가?
- 공개 API와 오류 처리 방식이 달라지는가?

<div class="takeaway">컴파일러가 제안한 수정도 비용과 의도를 사람이 검토해야 합니다.</div>

<!--
교재 대응: book/src/compilation/04-reading-errors.md > 이동한 값을 다시 사용한 예제

print_message가 String을 값으로 받으므로 message의 소유권이 함수로 이동합니다. 뒤의 println!에서 같은 값을 다시 사용하면 컴파일러는
E0382를 보고합니다. 다음 명령으로 오류 코드의 자세한 설명을 읽을 수 있습니다. 해결책은 하나가 아닙니다. print_message가 값을 소유해야 하는지 먼저
결정해야 합니다. 읽기만 하면 된다면 &str이나 &String을 받도록 바꿀 수 있고, 함수가 값을 보관해야 한다면 호출자가 이후에 사용하지 않도록 설계를 바꾸거나
명시적으로 복제할 수 있습니다. 컴파일러가 clone을 제안하더라도 복제가 요구 사항에 맞는지는 사람이 판단해야 합니다.
-->

---
level: 2
---

# AI에게 오류를 전달할 때

<ol class="step-list">
  <li>최소 재현 코드</li>
  <li>실행한 정확한 명령</li>
  <li>첫 번째 오류의 전체 내용</li>
  <li>프로그램이 원래 해야 하는 일</li>
</ol>

수정안에 불필요한 `clone()`, `unwrap()`, 넓은 `allow`가 들어오지 않았는지도 다시
확인합니다.

<!--
교재 대응: book/src/compilation/04-reading-errors.md > AI에게 오류를 전달할 때

오류 한 줄만 떼어 주면 이동이 시작된 위치나 관련 타입 정보가 빠질 수 있습니다. 최소 재현 코드, 실행한 명령, 첫 번째 오류의 전체 내용, 원래 의도를 함께
전달하세요. 받은 수정안은 다음 항목을 다시 확인합니다. • 불필요한 clone()으로 소유권 문제를 덮지 않았는가? • unwrap()이나 allow로 다른 오류를
숨기지 않았는가? • 공개 API나 오류 처리 방식이 달라지지 않았는가? • 수정 뒤 cargo check, 클리피(Clippy), 테스트가 모두 통과하는가?
-->
