---
theme: default
title: 의존성과 Cargo.lock 추적하기
info: 우아한 Rust 중급 개발 도구 6편
colorSchema: dark
aspectRatio: 16/9
canvasWidth: 1100
transition: fade-out
lineNumbers: true
layout: cover
class: cover
---

<div class="episode-label">우아한 Rust 중급 · 개발 도구 06</div>

# 의존성과<br>`Cargo.lock` 추적하기

허용한 버전과 실제 선택된 버전을 구분하고 의존성이 들어온 경로를 찾습니다.

---
level: 2
---

# 두 파일은 서로 다른 질문에 답합니다

<div class="grid grid-cols-2 gap-10 mt-10">
  <div>
    <h2><code>Cargo.toml</code></h2>
    <p>프로젝트에서 사용할 수 있는 의존성의 버전 범위를 적습니다.</p>
  </div>
  <div>
    <h2><code>Cargo.lock</code></h2>
    <p>카고<sub>Cargo</sub>가 실제로 선택한 직접·간접 의존성의 정확한 버전과 출처를 기록합니다.</p>
  </div>
</div>

<div class="takeaway compact"><code>Cargo.lock</code>은 직접 편집하지 않고 카고 명령으로 업데이트하며, 보통 Git에 포함합니다.</div>

---
level: 2
---

# 추가와 제거 뒤에는 두 파일의 diff를 읽습니다

```bash
cargo add serde --features derive
cargo add pretty_assertions --dev

cargo remove serde
cargo remove pretty_assertions --dev
```

- 이름이 비슷한 다른 패키지를 추가하지 않았는가?
- 필요하지 않은 피처<sub>feature</sub>가 켜지지 않았는가?
- 직접 의존성 때문에 어떤 간접 의존성이 들어왔는가?

---
level: 2
---

# 업데이트 범위는 가능한 한 좁힙니다

```bash
cargo update
cargo update -p serde
```

패키지 하나를 지정해도 그 패키지가 사용하는 간접 의존성은 함께 달라질 수
있습니다. 잠금 파일의 변경량과 전체 테스트 결과를 확인합니다.

<div class="takeaway">“버전 하나를 올렸다”가 아니라 실제로 달라진 의존성 묶음을 검토합니다.</div>

---
level: 2
---

# 세 옵션은 제한하는 대상이 다릅니다

<table class="compare">
  <thead><tr><th>옵션</th><th>동작</th></tr></thead>
  <tbody>
    <tr><td><code>--locked</code></td><td>기존 잠금 파일을 바꿔야 하면 실패</td></tr>
    <tr><td><code>--offline</code></td><td>네트워크 없이 이미 받은 자료만 사용</td></tr>
    <tr><td><code>--frozen</code></td><td><code>--locked</code>와 <code>--offline</code>을 함께 적용</td></tr>
  </tbody>
</table>

`--offline`은 필요한 패키지를 미리 받지 않았다면 실패할 수 있습니다.

---
level: 2
---

# `cargo tree`는 실제 의존성 관계를 펼칩니다

```bash
cargo tree
```

- 들여쓰기는 어느 패키지가 다음 패키지를 가져왔는지 보여 줍니다.
- 이미 출력한 하위 구조에는 `(*)`가 붙습니다.
- `Cargo.toml`에 직접 적지 않은 간접 의존성도 나타납니다.

<!--
강의 프로젝트는 외부 의존성이 없으므로 먼저 현재의 짧은 결과를 보여 줍니다.
그 뒤 의존성이 있는 샘플 저장소나 cargo add 직후 결과와 비교합니다.
-->

---
level: 2
---

# 예상하지 않은 패키지는 관계를 거꾸로 찾습니다

```bash
cargo tree -i serde
cargo tree --duplicates
```

<div class="grid grid-cols-2 gap-10 mt-8">
  <div><h3><code>-i</code></h3>어떤 상위 의존성이 그 패키지를 가져왔는지 찾습니다.</div>
  <div><h3><code>--duplicates</code></h3>여러 버전이 함께 포함된 패키지만 확인합니다.</div>
</div>

여러 버전이 항상 오류는 아니지만 빌드 시간과 실행 파일 크기를 살펴볼 근거가 됩니다.

---
level: 2
---

# 활성화된 피처의 경로도 추적할 수 있습니다

```bash
cargo tree -e features
cargo tree -e features -i serde
```

출력이 길다면 패키지를 지정해 어느 경로에서 피처가 활성화됐는지 거꾸로
살펴봅니다.

<div class="takeaway">의존성 이름만 보지 말고 누가 가져왔고 어떤 피처를 켰는지 확인합니다.</div>

---
level: 2
---

# 의존성 변경은 하나의 조사 흐름입니다

<ol class="step-list">
  <li><code>Cargo.toml</code>에서 허용 범위를 확인합니다.</li>
  <li><code>Cargo.lock</code>의 실제 변경을 읽습니다.</li>
  <li><code>cargo tree -i</code>로 들어온 경로를 찾습니다.</li>
  <li>필요한 피처만 켜졌는지 확인합니다.</li>
  <li><code>--locked</code>로 전체 검증을 다시 실행합니다.</li>
</ol>

---
level: 2
layout: center
class: section
---

# 잠금 파일은 결과이고<br>의존성 관계는 판단 근거입니다

다음 편에서는 카고 피처별 코드와 API 문서를 함께 검사합니다.
