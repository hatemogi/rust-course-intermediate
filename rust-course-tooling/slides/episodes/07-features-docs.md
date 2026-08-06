---
theme: default
title: 카고 피처와 API 문서
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

# 카고 피처와<br>API 문서

지원할 카고<sub>Cargo</sub> 피처<sub>feature</sub> 구성을 검사하고 생성된 공개
API 문서를 읽습니다.

<!--
교재 대응: book/src/cargo/13-features.md > 카고 피처와 검사 조합; book/src/documentation/14-cargo-doc.md > cargo doc으로 API 문서 확인하기

카고(Cargo) 피처(feature)를 사용하면 조건에 따라 코드를 컴파일하거나 필요할 때만 쓰는 의존성을 활성화할 수 있습니다. 이 강의 프로젝트에는 다음 두 피처가
있습니다. default에 아무 항목도 없으므로 별도 옵션 없이 빌드하면 binary-search는 비활성화됩니다. 다음 예제는 피처 상태에 따라 서로 다른 검색 함수를
컴파일합니다. 두 구성을 각각 실행해 출력 차이를 확인할 수 있습니다. API 문서는 라이브러리를 사용하는 사람이 공개 타입과 함수를 찾아보고 사용법을 이해하도록
돕습니다. 소스 코드의 문서 주석만 읽어서는 전체 구성을 파악하기 어려우므로, HTML 문서를 만들어 실제 독자가 보는 형태로 확인합니다. cargo doc은
rustdoc을 실행해 target/doc에 HTML 문서를 만듭니다. --no-deps를 지정하면 의존성 문서는 다시 만들지 않고 현재 패키지의 문서만 생성합니다.
--locked는 이 과정에서 잠금 파일이 바뀌어야 한다면 실패하게 합니다. --open은 문서를 만든 뒤 브라우저를 여는 동작이므로 사람이 결과를 확인할 때
편리합니다. 브라우저에서 함수 설명을 찾기 쉬운지, 타입과 메서드가 자연스럽게 이어지는지, 예제가 필요한 위치에 놓였는지 살펴봅니다.
-->

---
level: 2
---

# 검사 범위 지정하기

<div class="target-feature-visual">
  <div class="feature-axis">
    <span>구성</span>
    <code>피처</code>
  </div>
  <div class="target-feature-matrix">
    <div class="target-axis"><span>컴파일 타깃</span><code>target</code></div>
    <div class="matrix-corner"></div>
    <div class="matrix-head">lib</div>
    <div class="matrix-head">bin</div>
    <div class="matrix-head">example</div>
    <div class="matrix-head">test</div>
    <div class="matrix-row-label"><code>default</code></div>
    <div class="matrix-cell target-cell">●</div>
    <div class="matrix-cell target-cell">●</div>
    <div class="matrix-cell target-cell">●</div>
    <div class="matrix-cell target-cell">●</div>
    <div class="matrix-row-label"><code>binary-search</code></div>
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

<div class="takeaway compact">모든 타깃을 검사해도 기본값이 아닌 피처 코드는 빠질 수 있습니다.</div>

<!--
교재 대응: book/src/cargo/13-features.md > 검사 범위 지정하기

피처 옵션을 지정하지 않으면 기본 피처만 활성화됩니다. --all-targets와 --all-features는 뜻이 다릅니다. 전자는 라이브러리, 실행 파일, 예제와
테스트 같은 카고 타깃을 넓게 검사합니다. 후자는 조건부 컴파일 구성을 바꿉니다. 모든 타깃을 검사해도 기본값이 아닌 피처의 코드는 빠질 수 있습니다.
--all-features도 모든 조건부 코드를 검사한다는 뜻은 아닙니다. 예제의 #[cfg(not(feature = "binary-search"))] 분기는
binary-search를 켜면 컴파일에서 빠집니다. 따라서 기본 피처를 끈 구성과 모든 피처를 켠 구성에서 클리피(Clippy)와 테스트를 각각 실행해야 두 검색
분기를 모두 확인할 수 있습니다.
-->

---
level: 2
---

# 카고 피처와 검사 조합

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

<!--
교재 대응: book/src/cargo/13-features.md > 카고 피처와 검사 조합

카고(Cargo) 피처(feature)를 사용하면 조건에 따라 코드를 컴파일하거나 필요할 때만 쓰는 의존성을 활성화할 수 있습니다. 이 강의 프로젝트에는 다음 두 피처가
있습니다. default에 아무 항목도 없으므로 별도 옵션 없이 빌드하면 binary-search는 비활성화됩니다. 다음 예제는 피처 상태에 따라 서로 다른 검색 함수를
컴파일합니다. 두 구성을 각각 실행해 출력 차이를 확인할 수 있습니다.
-->

---
level: 2
---

# 카고 피처와 검사 조합

```bash
cargo run --example 13_features
cargo run --example 13_features --features binary-search
```

<div class="grid grid-cols-2 gap-10 mt-8">
  <div><h3>기본 구성</h3>선형 검색 코드가 컴파일됩니다.</div>
  <div><h3><code>binary-search</code></h3>정렬된 입력을 전제로 이진 검색 코드를 컴파일합니다.</div>
</div>

<!--
교재 대응: book/src/cargo/13-features.md > 카고 피처와 검사 조합

카고(Cargo) 피처(feature)를 사용하면 조건에 따라 코드를 컴파일하거나 필요할 때만 쓰는 의존성을 활성화할 수 있습니다. 이 강의 프로젝트에는 다음 두 피처가
있습니다. default에 아무 항목도 없으므로 별도 옵션 없이 빌드하면 binary-search는 비활성화됩니다. 다음 예제는 피처 상태에 따라 서로 다른 검색 함수를
컴파일합니다. 두 구성을 각각 실행해 출력 차이를 확인할 수 있습니다.
-->

---
level: 2
---

# 검사 범위 지정하기

```bash
cargo check
cargo check --features binary-search
cargo check --no-default-features
cargo check --all-features
```

- 기본 구성을 항상 검사합니다.
- 특정 피처를 추가로 켤 때는 `--features`를 사용합니다.
- 기본 피처를 끈 구성을 지원한다면 따로 검사합니다.
- 모든 피처가 함께 켜질 수 있다면 `--all-features`를 검사합니다.

<div class="takeaway compact"><code>--all-features</code>는 모든 조건부 코드를 컴파일한다는 뜻이 아닙니다.</div>

<!--
교재 대응: book/src/cargo/13-features.md > 검사 범위 지정하기

• --features는 나열한 피처를 추가로 켭니다. • --all-features는 패키지에 정의된 모든 피처를 켭니다. • --no-default-features는
default에 나열한 피처를 끕니다. --all-targets와 --all-features는 뜻이 다릅니다. 전자는 라이브러리, 실행 파일, 예제와 테스트 같은 카고
타깃을 넓게 검사합니다. 후자는 조건부 컴파일 구성을 바꿉니다. 모든 타깃을 검사해도 기본값이 아닌 피처의 코드는 빠질 수 있습니다. --all-features도 모든
조건부 코드를 검사한다는 뜻은 아닙니다. 예제의 #[cfg(not(feature = "binary-search"))] 분기는 binary-search를 켜면 컴파일에서
빠집니다. 따라서 기본 피처를 끈 구성과 모든 피처를 켠 구성에서 클리피(Clippy)와 테스트를 각각 실행해야 두 검색 분기를 모두 확인할 수 있습니다.
-->

---
level: 2
---

# 어떤 조합을 검사할지 정하기

피처 하나를 켰을 때 기존 공개 API의 뜻이 달라지거나 다른 피처를 꺼야만
컴파일된다면 사용하는 쪽에서 조합하기 어렵습니다.

<div class="takeaway">가능한 모든 조합보다 프로젝트가 지원한다고 약속한 조합을 먼저 정의합니다.</div>

<!--
교재 대응: book/src/cargo/13-features.md > 어떤 조합을 검사할지 정하기

피처가 늘어나면 가능한 조합 수가 빠르게 증가합니다. 모든 조합을 무조건 검사하기보다 프로젝트가 지원한다고 약속한 구성을 정합니다. 사용자가 별도 옵션 없이 쓰는 기본
피처 구성을 검사합니다. 기본 피처를 모두 끈 구성을 지원한다면 --no-default-features로 검사합니다. 모든 피처가 함께 켜질 수 있다면
--all-features로 검사합니다. cfg(not(...))처럼 피처를 켰을 때 빠지는 코드가 있는지 확인합니다. 자주 쓰거나 서로 영향을 주는 조합을 별도로
검사합니다. 이 프로젝트의 make check는 기본 구성, 기본 피처를 끈 구성, 모든 피처를 켠 구성을 컴파일하고 테스트합니다. 클리피도 기본 피처를 끈 구성과 모든
피처를 켠 구성에서 각각 실행합니다. 피처가 다른 피처나 선택적 의존성을 켜는 방법과, 여러 경로에서 켜진 피처가 합쳐지는 방식은 카고 피처 문서에서 확인할 수
있습니다.
-->

---
level: 2
---

# `cargo doc`으로 API 문서 확인하기

<div class="tool-flow">
  <div><strong>문서 테스트</strong>코드 블록이 컴파일되고 실행되는가?</div>
  <div><strong>문서 생성</strong>링크와 공개 항목이 올바르게 만들어지는가?</div>
  <div><strong>사람의 검토</strong>설명과 사용 순서를 이해하기 쉬운가?</div>
</div>

<!--
교재 대응: book/src/documentation/14-cargo-doc.md > cargo doc으로 API 문서 확인하기

API 문서는 라이브러리를 사용하는 사람이 공개 타입과 함수를 찾아보고 사용법을 이해하도록 돕습니다. 소스 코드의 문서 주석만 읽어서는 전체 구성을 파악하기 어려우므로,
HTML 문서를 만들어 실제 독자가 보는 형태로 확인합니다. cargo doc은 rustdoc을 실행해 target/doc에 HTML 문서를 만듭니다.
--no-deps를 지정하면 의존성 문서는 다시 만들지 않고 현재 패키지의 문서만 생성합니다. --locked는 이 과정에서 잠금 파일이 바뀌어야 한다면 실패하게
합니다. 브라우저에서 바로 확인하려면 다음 명령을 사용합니다.
-->

---
level: 2
---

# `cargo doc`으로 API 문서 확인하기

```bash
cargo doc --no-deps --locked
cargo doc --no-deps --open
```

- 결과는 `target/doc`에 HTML로 만들어집니다.
- `--no-deps`는 현재 패키지 문서만 생성합니다.
- `--open`은 사람이 확인할 때만 브라우저를 엽니다.
- 자동 검사에서는 브라우저를 열지 않습니다.

<!--
교재 대응: book/src/documentation/14-cargo-doc.md > cargo doc으로 API 문서 확인하기

API 문서는 라이브러리를 사용하는 사람이 공개 타입과 함수를 찾아보고 사용법을 이해하도록 돕습니다. 소스 코드의 문서 주석만 읽어서는 전체 구성을 파악하기 어려우므로,
HTML 문서를 만들어 실제 독자가 보는 형태로 확인합니다. cargo doc은 rustdoc을 실행해 target/doc에 HTML 문서를 만듭니다.
--no-deps를 지정하면 의존성 문서는 다시 만들지 않고 현재 패키지의 문서만 생성합니다. --locked는 이 과정에서 잠금 파일이 바뀌어야 한다면 실패하게
합니다. --open은 문서를 만든 뒤 브라우저를 여는 동작이므로 사람이 결과를 확인할 때 편리합니다. 브라우저에서 함수 설명을 찾기 쉬운지, 타입과 메서드가 자연스럽게
이어지는지, 예제가 필요한 위치에 놓였는지 살펴봅니다.
-->

---
level: 2
---

# 크레이트와 공개 항목 설명하기

```rust
//! 크레이트(crate) 전체를 설명합니다.

/// 이 공개 함수가 무엇을 하고 어떤 조건을 요구하는지 설명합니다.
pub fn normalize_names(names: &[&str]) -> Vec<String> {
    // ...
}
```

기본 문서에는 외부에서 접근할 수 있는 공개 항목만 나타납니다. 내부 설계를
검토하려면 `--document-private-items`를 따로 사용할 수 있습니다.

<!--
교재 대응: book/src/documentation/14-cargo-doc.md > 크레이트와 공개 항목 설명하기

크레이트(crate) 루트의 //! 주석은 크레이트 전체를 설명하고, 공개 함수나 타입 앞의 /// 주석은 해당 항목을 설명합니다. 이 강의의 src/lib.rs에서도
두 종류를 함께 사용합니다. 기본 문서에는 외부에서 접근할 수 있는 공개 항목만 나타납니다. 내부 함수까지 살펴보고 싶다면 다음 옵션을 추가합니다. 내부 구현을
문서화한다고 해서 모두 공개 API가 되는 것은 아닙니다. 이 옵션은 설계를 검토하거나 프로젝트 내부 문서를 확인할 때 사용합니다.
-->

---
level: 2
---

# 피처별 문서 확인하기

```bash
cargo doc --no-deps --no-default-features --locked
cargo doc --no-deps --all-features --locked
```

<ol class="step-list">
  <li>지원하는 피처 구성별로 문서를 생성합니다.</li>
  <li>공개 항목이 예상대로 나타나는지 확인합니다.</li>
  <li>대표 사용 예제와 오류 조건을 읽습니다.</li>
  <li>피처가 API의 뜻을 예상 밖으로 바꾸지 않는지 검토합니다.</li>
</ol>

<!--
교재 대응: book/src/documentation/14-cargo-doc.md > 피처별 문서 확인하기

피처(feature)에 따라 공개 API가 달라진다면 문서도 그 구성에 따라 달라집니다. 구성별로 문서를 열어 보면 각 피처에서 사용할 수 있어야 하는 공개 항목이
제대로 나타나는지 확인할 수 있습니다. --all-features로 만든 문서에는 피처를 껐을 때만 나타나는 공개 항목이 빠질 수 있으므로, 프로젝트가 지원하는 구성별로
문서를 생성해 살펴봅니다. 문장이 이해하기 쉬운지와 예제가 실제 사용 순서를 잘 보여 주는지도 함께 확인합니다. 명령별 대상과 옵션은 카고(Cargo)의 cargo
doc 문서와 rustdoc 안내서에서 확인할 수 있습니다.
-->
