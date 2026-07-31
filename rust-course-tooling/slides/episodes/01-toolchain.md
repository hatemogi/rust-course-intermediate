---
theme: default
title: Rust 툴체인과 프로젝트 환경
info: 우아한 Rust 중급 개발 도구 1편
colorSchema: dark
aspectRatio: 16/9
canvasWidth: 1100
transition: fade-out
lineNumbers: true
layout: cover
class: cover
---

<div class="episode-label">우아한 Rust 중급 · 개발 도구 01</div>

# Rust 툴체인과<br>프로젝트 환경

`rustup`, `rustc`, 카고<sub>Cargo</sub>를 구분하고 팀의 Rust 버전을 맞춥니다.

<!--
이번 편의 핵심은 명령을 많이 외우는 것이 아닙니다.
비슷해 보이는 세 도구가 각각 무엇을 바꾸는지 구분하는 데 집중합니다.
-->

---
level: 2
---

# 10편이 하나의 검증 흐름으로 이어집니다

<div class="course-overview">
  <section v-click>
    <h2>공개 영상 · 1–4편</h2>
    <ol start="1">
      <li><span>01</span> 툴체인과 프로젝트 환경</li>
      <li><span>02</span> <code>cargo check</code>와 컴파일 오류</li>
      <li><span>03</span> rustfmt와 형식 검사</li>
      <li><span>04</span> 클리피와 첫 검증 실습</li>
    </ol>
  </section>
  <section v-click>
    <h2>전체 과정 · 5–10편</h2>
    <ol start="5">
      <li><span>05</span> 편집기 자동화와 <code>cargo fix</code></li>
      <li><span>06</span> 의존성과 <code>Cargo.lock</code></li>
      <li><span>07</span> 카고 피처와 API 문서</li>
      <li><span>08</span> 단위 테스트와 통합 테스트</li>
      <li><span>09</span> 오류 테스트와 문서 테스트</li>
      <li><span>10</span> 전체 검증 흐름과 종합 실습</li>
    </ol>
  </section>
</div>

<!--
환경을 맞추는 일에서 시작해 검사와 테스트까지 이어지는 순서를 먼저 보여 줍니다.
이번 영상은 1편이며, 4편까지 마치면 컴파일·형식·린트<sub>lint</sub>를 직접 검사할 수 있다고 안내합니다.
-->

---
level: 2
---

# 세 도구는 서로 다른 일을 맡습니다

<div class="tool-flow">
  <div v-click>
    <strong>rustup</strong>
    Rust 툴체인<sub>toolchain</sub>과 컴포넌트<sub>component</sub>를 설치하고 선택합니다.
  </div>
  <div v-click>
    <strong>rustc</strong>
    Rust 소스 코드를 컴파일합니다.
  </div>
  <div v-click>
    <strong>카고</strong>
    패키지, 의존성, 빌드와 테스트 작업을 관리합니다.
  </div>
</div>

<div v-click class="takeaway">무엇을 확인하거나 바꾸려는지 먼저 정하면 명령을 고르기 쉬워집니다.</div>

---
level: 2
---

# 버전도 각각 확인해야 합니다

```bash
rustup --version
rustc --version
cargo --version
```

<div class="grid grid-cols-2 gap-10 mt-8">
  <div>
    <h3><code>rustup --version</code></h3>
    <p>rustup 프로그램 자체의 버전입니다.</p>
  </div>
  <div>
    <h3><code>rustc --version</code></h3>
    <p>현재 선택된 Rust 컴파일러 버전입니다.</p>
  </div>
</div>

<div class="takeaway">“Rust 버전”을 묻는다면 대개 <code>rustc --version</code>의 결과가 필요합니다.</div>

---
level: 2
---

# `rustup show`로 현재 선택을 확인합니다

```bash
rustup show

rustup toolchain list
rustup component list --installed
rustup target list --installed
```

- 기본 호스트와 설치된 툴체인
- 현재 디렉터리에서 선택된 툴체인
- 설치된 선택 컴포넌트
- 설치된 컴파일 타깃

<!--
여기서는 실제 터미널로 전환해 rustup show를 실행합니다.
출력 전체를 읽기보다 active toolchain과 installed targets 위치를 먼저 짚습니다.
-->

---
level: 2
---

# 설치·업데이트·기본값 변경은 범위가 다릅니다

```bash
rustup toolchain install stable
rustup update stable
rustup default stable
```

- `toolchain install`은 stable 툴체인이 없으면 설치합니다.
- `update stable`은 설치된 stable 툴체인을 최신 stable 릴리스로 업데이트합니다.
- `default stable`은 별도 지정이 없는 디렉터리의 사용자 기본값을 바꿉니다.

<div class="takeaway compact">사용자 컴퓨터의 Rust 환경이 바뀌므로 실행 전에 필요한 범위를 확인합니다.</div>

---
level: 2
---

# 기본값을 바꾸지 않고 한 번만 선택할 수 있습니다

```bash
rustc +stable --version
cargo +stable check
cargo +nightly fmt
```

- `+toolchain`은 **그 명령에만** 적용됩니다.
- nightly는 매일 업데이트되는 개발 채널입니다. 실험 기능이 꼭 필요할 때만 씁니다.
- 지정한 툴체인이 설치되어 있지 않으면 먼저 설치해야 합니다.

<div class="takeaway">잠깐 확인할 때는 변경 범위가 가장 작은 방법부터 선택합니다.</div>

---
level: 2
---

# rustfmt와 클리피는 선택 컴포넌트입니다

```bash
rustup component add rustfmt clippy
rustup component add --toolchain stable rustfmt clippy
rustup component list --installed
```

<div class="grid grid-cols-2 gap-10 mt-8">
  <div><h3><code>rustfmt</code></h3>Rust 코드를 정해진 형식으로 자동 정리합니다.</div>
  <div><h3>클리피<sub>Clippy</sub></h3>실수 가능성이나 불필요하게 복잡한 표현을 찾는 공식 린트 도구입니다.</div>
</div>

---
level: 2
---

# 다른 환경용 결과물은 타깃을 추가합니다

```bash
rustup target list --installed
rustup target add wasm32-unknown-unknown
rustup target remove wasm32-unknown-unknown
```

웹어셈블리<sub>WebAssembly</sub>나 다른 운영체제·CPU 아키텍처용 결과물을 만들 때
필요한 컴파일 타깃을 설치합니다.

<div class="takeaway compact">크로스 컴파일<sub>cross-compilation</sub>에는 대상에 따라 링커와 시스템 라이브러리도 필요합니다.</div>

---
level: 2
---

# 팀이 함께 쓸 버전은 저장소에 설정합니다

<<< ../../rust-toolchain.toml toml {lines:true}

<div class="grid grid-cols-3 gap-6 mt-7">
  <div><strong>channel</strong><br><span class="muted">채널 또는 정확한 버전</span></div>
  <div><strong>profile</strong><br><span class="muted">설치 구성의 크기</span></div>
  <div><strong>components</strong><br><span class="muted">프로젝트에 필요한 도구</span></div>
</div>

<div class="takeaway compact">파일을 저장소 루트에 저장하고 커밋하면 개발자와 CI가 같은 설정을 읽습니다.</div>

---
level: 2
---

# `stable`과 정확한 버전은 비용이 다릅니다

<table class="compare">
  <thead>
    <tr><th>선택</th><th>얻는 것</th><th>감수할 것</th></tr>
  </thead>
  <tbody>
    <tr>
      <td><code>stable</code></td>
      <td>새 개선을 자연스럽게 따라감</td>
      <td>설치·업데이트 시점에 따라 실제 버전이 달라짐</td>
    </tr>
    <tr>
      <td><code>1.94.0</code></td>
      <td>개발자와 CI의 결과를 맞추기 쉬움</td>
      <td>버전을 직접 올리고 전체 검증해야 함</td>
    </tr>
  </tbody>
</table>

<div class="takeaway">재현성과 업데이트 비용 가운데 프로젝트에 중요한 쪽을 선택합니다.</div>

---
level: 2
---

# 실행할 버전과 지원할 버전은 다른 약속입니다

<div class="grid grid-cols-2 gap-10 mt-8">
  <div>
    <h3><code>rust-toolchain.toml</code></h3>
    <p>이 저장소에서 rustup이 선택할 툴체인을 정합니다.</p>
  </div>
  <div>
    <h3><code>Cargo.toml</code>의 <code>rust-version</code></h3>
    <p>패키지가 지원하는 가장 오래된 Rust 버전을 적습니다.</p>
  </div>
</div>

<p><strong>MSRV</strong>는 Minimum Supported Rust Version, 즉 가장 오래된 지원 Rust 버전입니다.</p>

<div class="command mt-8">make msrv</div>

<div class="takeaway">툴체인을 올렸다고 지원하는 가장 오래된 버전까지 저절로 바뀌지는 않습니다.</div>

---
level: 2
---

# 개인 override는 팀의 버전 설정이 아닙니다

```bash
rustup override set stable
rustup override list
rustup override unset
```

- override는 사용자 rustup 설정에 저장되고 Git으로 공유되지 않습니다.
- 잠깐 로컬에서 확인한 뒤에는 불필요한 override를 제거합니다.

<div class="takeaway">팀이 함께 따라야 하는 버전은 저장소의 <code>rust-toolchain.toml</code>에 적습니다.</div>

---
level: 2
---

# 파일보다 실제 선택 결과를 확인합니다

<div class="command">rustup show active-toolchain</div>
<div class="command">rustc --version</div>
<div class="command">cargo --version</div>

설정 파일을 추가했다고 끝난 것이 아닙니다. 그 디렉터리에서 실제로 선택된 버전과
필요한 컴포넌트가 맞는지 확인해야 합니다.

---
level: 2
---

# 이름이 비슷해도 바꾸는 대상은 다릅니다

<div class="grid grid-cols-2 gap-10 mt-10">
  <div>
    <h2><code>rustup update stable</code></h2>
    <p>Rust 컴파일러와 툴체인을 업데이트합니다.</p>
  </div>
  <div>
    <h2><code>cargo update</code></h2>
    <p>프로젝트가 사용하는 크레이트<sub>crate</sub> 버전을 다시 선택하고 <code>Cargo.lock</code>을 업데이트합니다.</p>
  </div>
</div>

<div class="takeaway compact">크레이트는 Rust 컴파일 단위이며, 외부 라이브러리 패키지는 <code>crates.io</code>에서 의존성으로 추가할 수 있습니다.</div>

---
level: 2
layout: center
class: section
---

# 프로젝트 환경과 지원 범위는<br>파일에 적고 실제로 검사합니다

두 설정의 역할을 구분하고, 실제로 선택된 툴체인과 가장 오래된 지원 버전에서
검사가 통과하는지 확인하세요.

<!--
다음 편에서는 이 환경에서 cargo check를 실행하고 컴파일 오류를 읽습니다.
-->
