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

---
level: 2
---

# rustfmt는 코드의 배치를 맡습니다

- 줄바꿈과 들여쓰기
- 연산자와 쉼표 주변의 공백
- Rust 코드의 일관된 형식

<div class="takeaway">형식을 도구에 맡기면 코드 검토에서 동작과 설계에 집중할 수 있습니다.</div>

rustfmt는 변수 이름, 오류 처리 방식, 알고리즘이 올바른지는 판단하지 않습니다.

---
level: 2
---

# `cargo fmt`는 소스 파일을 바꿉니다

<div class="command">cargo fmt</div>

<<< ../../examples/01_formatting.rs rust {lines:true}

명령을 실행하기 전후로 `git diff`를 읽으면 어떤 파일이 어떻게 정리됐는지 확인할
수 있습니다.

---
level: 2
---

# 자동 검사에서는 고치지 않고 차이만 찾습니다

<div class="command">cargo fmt --check</div>

- 형식이 맞으면 성공합니다.
- 형식이 다르면 파일을 고치지 않고 차이를 보여 주며 실패합니다.
- CI가 실행한 뒤 저장소 내용이 몰래 달라지는 일을 막습니다.

<div class="takeaway">로컬에서는 <code>cargo fmt</code>, 자동 검사에서는 <code>cargo fmt --check</code>를 사용합니다.</div>

---
level: 2
---

# workspace 전체를 검사할 수도 있습니다

```bash
cargo fmt --all --check
```

여러 패키지를 묶은 workspace에서는 어느 패키지를 열어 작업했는지와 관계없이 전체
Rust 소스의 형식을 확인할 수 있습니다.

---
level: 2
---

# 형식 변경은 작은 diff일 때 읽기 쉽습니다

<ol class="step-list">
  <li>기능 변경을 작게 나눕니다.</li>
  <li><code>cargo fmt</code>를 자주 실행합니다.</li>
  <li><code>git diff</code>에서 동작 변경과 형식 변경을 함께 확인합니다.</li>
  <li>커밋 전 <code>cargo fmt --check</code>를 다시 실행합니다.</li>
</ol>

<div class="takeaway">생성된 코드가 커진 뒤 한꺼번에 포맷하면 검토해야 할 diff도 커집니다.</div>

---
level: 2
---

# 한 줄의 모양보다 프로젝트 규칙을 우선합니다

rustfmt가 만든 줄바꿈이 마음에 들지 않는다는 이유만으로 해당 영역을 넓게 제외하면
이후 코드가 일관되지 않게 쌓일 수 있습니다.

<div class="question">그 모양을 반드시 유지해야 하는 이유를 팀원에게 설명할 수 있나요?</div>

---
level: 2
---

# 빠른 반복과 최종 검사는 역할이 다릅니다

<div class="tool-flow">
  <div><strong>작성 중</strong><code>cargo fmt</code><br><span class="muted">파일을 정리합니다.</span></div>
  <div><strong>검토 중</strong><code>git diff</code><br><span class="muted">의도한 변경인지 읽습니다.</span></div>
  <div><strong>완료 전</strong><code>cargo fmt --check</code><br><span class="muted">차이가 남지 않았는지 확인합니다.</span></div>
</div>

---
level: 2
layout: center
class: section
---

# 형식이 맞는 코드가<br>곧 올바른 코드는 아닙니다

다음 편에서는 Clippy의 경고를 읽고 적용 여부를 판단합니다.
