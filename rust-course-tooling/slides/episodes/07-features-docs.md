---
theme: default
title: Cargo feature와 API 문서
info: 우아한 Rust 중급 개발 도구 7편
colorSchema: dark
aspectRatio: 16/9
canvasWidth: 1100
transition: fade-out
lineNumbers: true
layout: cover
class: cover
---

<div class="episode-label">우아한 Rust 중급 · 개발 도구 07</div>

# Cargo feature와<br>API 문서

지원할 feature 구성을 검사하고 생성된 공개 API 문서를 읽습니다.

---
level: 2
---

# 타깃과 feature는 서로 다른 축입니다

<div class="target-feature-visual">
  <div class="feature-axis">
    <span>구성</span>
    <code>feature</code>
  </div>
  <div class="target-feature-matrix">
    <div class="target-axis"><span>컴파일 대상</span><code>target</code></div>
    <div class="matrix-corner"></div>
    <div class="matrix-head">lib</div>
    <div class="matrix-head">bin</div>
    <div class="matrix-head">example</div>
    <div class="matrix-head">test</div>
    <div class="matrix-head">bench</div>
    <div class="matrix-row-label"><code>default</code></div>
    <div class="matrix-cell target-cell">●</div>
    <div class="matrix-cell target-cell">●</div>
    <div class="matrix-cell target-cell">●</div>
    <div class="matrix-cell target-cell">●</div>
    <div class="matrix-cell target-cell">●</div>
    <div class="matrix-row-label"><code>binary-search</code></div>
    <div class="matrix-cell feature-cell">●</div>
    <div class="matrix-cell feature-cell">●</div>
    <div class="matrix-cell feature-cell">●</div>
    <div class="matrix-cell feature-cell">●</div>
    <div class="matrix-cell feature-cell">●</div>
  </div>
</div>

<div class="axis-legend">
  <span><code>--all-targets</code>는 가로로 넓힙니다.</span>
  <span><code>--all-features</code>는 세로로 넓힙니다.</span>
</div>

<div class="takeaway compact">모든 타깃을 검사해도 기본값이 아닌 feature 코드는 빠질 수 있습니다.</div>

---
level: 2
---

# 이 프로젝트는 검색 구현을 feature로 고릅니다

```toml
[features]
default = []
binary-search = []
```

```rust {1-4|6-9}{lines:true}
#[cfg(feature = "binary-search")]
fn contains(values: &[u64], target: u64) -> bool {
    values.binary_search(&target).is_ok()
}

#[cfg(not(feature = "binary-search"))]
fn contains(values: &[u64], target: u64) -> bool {
    values.contains(&target)
}
```

---
level: 2
---

# 두 구성을 직접 실행해 비교합니다

```bash
cargo run --example 07_features
cargo run --example 07_features --features binary-search
```

<div class="grid grid-cols-2 gap-10 mt-8">
  <div><h3>기본 구성</h3>선형 검색 코드가 컴파일됩니다.</div>
  <div><h3><code>binary-search</code></h3>정렬된 입력을 전제로 이진 검색 코드를 컴파일합니다.</div>
</div>

---
level: 2
---

# 검사 명령은 지원 약속을 표현합니다

```bash
cargo check
cargo check --no-default-features
cargo check --all-features
```

- 기본 구성을 항상 검사합니다.
- 기본 feature를 끈 구성을 지원한다면 따로 검사합니다.
- 모든 feature가 함께 켜질 수 있다면 `--all-features`를 검사합니다.
- `--all-features`에서 빠지는 `cfg(not(...))` 분기도 검사합니다.

<div class="takeaway compact"><code>--all-features</code>는 모든 조건부 코드를 컴파일한다는 뜻이 아닙니다.</div>

---
level: 2
---

# feature는 가능한 한 더하는 방식으로 설계합니다

feature 하나를 켰을 때 기존 공개 API의 뜻이 달라지거나 다른 feature를 꺼야만
컴파일된다면 사용하는 쪽에서 조합하기 어렵습니다.

<div class="takeaway">가능한 모든 조합보다 프로젝트가 지원한다고 약속한 조합을 먼저 정의합니다.</div>

---
level: 2
---

# 문서 테스트와 생성된 문서는 다른 것을 확인합니다

<div class="tool-flow">
  <div><strong>문서 테스트</strong>코드 블록이 컴파일되고 실행되는가?</div>
  <div><strong>문서 생성</strong>링크와 공개 항목이 올바르게 만들어지는가?</div>
  <div><strong>사람의 검토</strong>설명과 사용 순서를 이해하기 쉬운가?</div>
</div>

---
level: 2
---

# 현재 패키지의 API 문서를 만듭니다

```bash
cargo doc --no-deps --locked
cargo doc --no-deps --open
```

- 결과는 `target/doc`에 HTML로 만들어집니다.
- `--no-deps`는 현재 패키지 문서만 생성합니다.
- `--open`은 사람이 확인할 때만 브라우저를 엽니다.
- 자동 검사에서는 브라우저를 열지 않습니다.

---
level: 2
---

# crate와 공개 항목은 주석 위치가 다릅니다

```rust
//! crate 전체를 설명합니다.

/// 이 공개 함수가 무엇을 하고 어떤 조건을 요구하는지 설명합니다.
pub fn normalize_names(names: &[&str]) -> Vec<String> {
    // ...
}
```

기본 문서에는 외부에서 접근할 수 있는 공개 항목만 나타납니다. 내부 설계를
검토하려면 `--document-private-items`를 따로 사용할 수 있습니다.

---
level: 2
---

# feature에 따라 문서도 달라질 수 있습니다

```bash
cargo doc --no-deps --no-default-features --locked
cargo doc --no-deps --all-features --locked
```

<ol class="step-list">
  <li>지원하는 feature 구성별로 문서를 생성합니다.</li>
  <li>공개 항목이 예상대로 나타나는지 확인합니다.</li>
  <li>대표 사용 예제와 오류 조건을 읽습니다.</li>
  <li>feature가 API의 뜻을 예상 밖으로 바꾸지 않는지 검토합니다.</li>
</ol>

---
level: 2
layout: center
class: section
---

# 컴파일되는 구성과<br>설명할 수 있는 API를 함께 유지합니다

다음 편에서는 구현 세부와 공개 동작을 서로 다른 테스트 위치에서 확인합니다.
