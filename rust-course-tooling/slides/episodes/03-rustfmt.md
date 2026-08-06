---
theme: default
title: rustfmt로 형식 검사하기
info: 우아한 Rust 중급 개발 도구 3편
colorSchema: dark
aspectRatio: 16/9
canvasWidth: 1100
transition: fade-out
lineNumbers: true
layout: cover
class: cover
---

<div class="episode-label">우아한 Rust 중급 · 개발 도구 03</div>

# rustfmt로<br>형식 검사하기

로컬 수정 명령과 자동 검사 명령을 구분합니다.

<!--
교재 대응: book/src/formatting/05-cargo-fmt.md > cargo fmt로 코드 형식 맞추기; book/src/formatting/06-fmt-check.md > 도구의 한계

rustfmt는 Rust 코드를 정해진 규칙에 따라 다시 배치합니다. 카고(Cargo) 프로젝트에서는 대개 rustfmt를 직접 호출하지 않고 다음 명령을 사용합니다.
이 명령은 프로젝트의 Rust 소스 파일을 실제로 수정합니다. 줄바꿈·들여쓰기·공백을 사람마다 다르게 정하는 대신 도구에 맡기면 코드 검토에서는 동작과 설계에 집중할 수
있습니다. rustfmt는 코드의 뜻, 변수 이름의 적절성, 오류 처리 방법을 판단하지 않습니다. 형식 검사를 통과했다는 사실은 코드가 올바르다는 뜻이 아닙니다.
-->

---
level: 2
---

# `cargo fmt`로 코드 형식 맞추기

- 줄바꿈과 들여쓰기
- 연산자와 쉼표 주변의 공백
- Rust 코드의 일관된 형식

<div class="takeaway">형식을 도구에 맡기면 코드 검토에서 동작과 설계에 집중할 수 있습니다.</div>

rustfmt는 변수 이름, 오류 처리 방식, 알고리즘이 올바른지는 판단하지 않습니다.

<!--
교재 대응: book/src/formatting/05-cargo-fmt.md > 판단하기

생성된 코드에 cargo fmt를 실행했을 때 큰 diff가 생긴다면 코드를 만든 직후 작은 단위로 실행하는 편이 검토하기 쉽습니다. 한 줄을 특별한 모양으로 유지하려고
rustfmt를 끄기 전에 그 모양이 반드시 필요한지 판단합니다.
-->

---
level: 2
---

# `cargo fmt`로 코드 형식 맞추기

<div class="command">cargo fmt</div>

<<< ../../examples/05_formatting.rs rust {lines:true}

명령을 실행하기 전후로 `git diff`를 읽으면 어떤 파일이 어떻게 정리됐는지 확인할
수 있습니다.

<!--
교재 대응: book/src/formatting/05-cargo-fmt.md > cargo fmt로 코드 형식 맞추기

rustfmt는 Rust 코드를 정해진 규칙에 따라 다시 배치합니다. 카고(Cargo) 프로젝트에서는 대개 rustfmt를 직접 호출하지 않고 다음 명령을 사용합니다.
이 명령은 프로젝트의 Rust 소스 파일을 실제로 수정합니다. 줄바꿈·들여쓰기·공백을 사람마다 다르게 정하는 대신 도구에 맡기면 코드 검토에서는 동작과 설계에 집중할 수
있습니다. 다음 예제는 형식이 맞춰진 작은 프로그램입니다.
-->

---
level: 2
---

# 형식 검사를 작업 흐름에 넣기

<div class="command">cargo fmt --check</div>

- 형식이 맞으면 성공합니다.
- 형식이 다르면 파일을 고치지 않고 차이를 보여 주며 실패합니다.
- CI가 실행한 뒤 저장소 내용이 몰래 달라지는 일을 막습니다.

<div class="takeaway">로컬에서는 <code>cargo fmt</code>, 자동 검사에서는 <code>cargo fmt --check</code>를 사용합니다.</div>

<!--
교재 대응: book/src/formatting/06-fmt-check.md > 형식 검사를 작업 흐름에 넣기

자동 검사에서는 파일을 고치지 않고 형식이 맞는지만 확인해야 합니다. 형식이 다르면 명령이 실패하고 차이를 보여 줍니다. 로컬에서는 cargo fmt로 고친 뒤 다시
--check를 실행합니다. CI에서 자동으로 파일을 고치게 하면 검사 결과와 저장소 내용이 달라지므로 보통 --check를 사용합니다.
-->

---
level: 2
---

# 형식 검사를 작업 흐름에 넣기

<ol class="step-list">
  <li>기능 변경을 작게 나눕니다.</li>
  <li><code>cargo fmt</code>를 자주 실행합니다.</li>
  <li><code>git diff</code>에서 동작 변경과 형식 변경을 함께 확인합니다.</li>
  <li>커밋 전 <code>cargo fmt --check</code>를 다시 실행합니다.</li>
</ol>

<div class="takeaway">생성된 코드가 커진 뒤 한꺼번에 포맷하면 검토해야 할 diff도 커집니다.</div>

<!--
교재 대응: book/src/formatting/06-fmt-check.md > 형식 검사를 작업 흐름에 넣기

자동 검사에서는 파일을 고치지 않고 형식이 맞는지만 확인해야 합니다. 형식이 다르면 명령이 실패하고 차이를 보여 줍니다. 로컬에서는 cargo fmt로 고친 뒤 다시
--check를 실행합니다. CI에서 자동으로 파일을 고치게 하면 검사 결과와 저장소 내용이 달라지므로 보통 --check를 사용합니다.
-->

---
level: 2
---

# `cargo fmt`로 코드 형식 맞추기

rustfmt가 만든 줄바꿈이 마음에 들지 않는다는 이유만으로 해당 영역을 넓게 제외하면
이후 코드가 일관되지 않게 쌓일 수 있습니다.

<div class="question">그 모양을 반드시 유지해야 하는 이유를 팀원에게 설명할 수 있나요?</div>

<!--
교재 대응: book/src/formatting/05-cargo-fmt.md > cargo fmt로 코드 형식 맞추기

rustfmt는 Rust 코드를 정해진 규칙에 따라 다시 배치합니다. 카고(Cargo) 프로젝트에서는 대개 rustfmt를 직접 호출하지 않고 다음 명령을 사용합니다.
이 명령은 프로젝트의 Rust 소스 파일을 실제로 수정합니다. 줄바꿈·들여쓰기·공백을 사람마다 다르게 정하는 대신 도구에 맡기면 코드 검토에서는 동작과 설계에 집중할 수
있습니다. 다음 예제는 형식이 맞춰진 작은 프로그램입니다.
-->

---
level: 2
---

# 형식 검사를 작업 흐름에 넣기

<div class="tool-flow">
  <div><strong>작성 중</strong><code>cargo fmt</code><br><span class="muted">파일을 정리합니다.</span></div>
  <div><strong>검토 중</strong><code>git diff</code><br><span class="muted">의도한 변경인지 읽습니다.</span></div>
  <div><strong>완료 전</strong><code>cargo fmt --check</code><br><span class="muted">차이가 남지 않았는지 확인합니다.</span></div>
</div>

<!--
교재 대응: book/src/formatting/06-fmt-check.md > 형식 검사를 작업 흐름에 넣기

자동 검사에서는 파일을 고치지 않고 형식이 맞는지만 확인해야 합니다. 형식이 다르면 명령이 실패하고 차이를 보여 줍니다. 로컬에서는 cargo fmt로 고친 뒤 다시
--check를 실행합니다. CI에서 자동으로 파일을 고치게 하면 검사 결과와 저장소 내용이 달라지므로 보통 --check를 사용합니다.
-->
