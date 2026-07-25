---
theme: default
title: Rust 도구 체인과 프로젝트 환경
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

# Rust 도구 체인과<br>프로젝트 환경

`rustup`, `rustc`, Cargo를 구분하고 팀의 Rust 버전을 맞춥니다.

<!--
이번 편의 핵심은 명령을 많이 외우는 것이 아닙니다.
비슷해 보이는 세 도구가 각각 무엇을 바꾸는지 구분하는 데 집중합니다.
-->

---
level: 2
---

# 11편이 하나의 검증 흐름으로 이어집니다

<div class="course-overview">
  <section v-click>
    <h2>공개 영상 · 1–4편</h2>
    <ol start="1">
      <li><span>01</span> 도구 체인과 프로젝트 환경</li>
      <li><span>02</span> <code>cargo check</code>와 컴파일 오류</li>
      <li><span>03</span> rustfmt와 형식 검사</li>
      <li><span>04</span> Clippy와 첫 검증 실습</li>
    </ol>
  </section>
  <section v-click>
    <h2>전체 과정 · 5–11편</h2>
    <ol start="5">
      <li><span>05</span> 편집기 자동화와 <code>cargo fix</code></li>
      <li><span>06</span> 의존성과 <code>Cargo.lock</code></li>
      <li><span>07</span> Cargo feature와 API 문서</li>
      <li><span>08</span> 단위 테스트와 통합 테스트</li>
      <li><span>09</span> 오류 테스트와 문서 테스트</li>
      <li><span>10</span> 벤치마크와 측정 결과</li>
      <li><span>11</span> 전체 검증 흐름과 종합 실습</li>
    </ol>
  </section>
</div>

<!--
환경을 맞추는 일에서 시작해 검사, 테스트와 측정까지 이어지는 순서를 먼저 보여 줍니다.
이번 영상은 1편이며, 4편까지 마치면 컴파일·형식·lint를 직접 검사할 수 있다고 안내합니다.
-->

---
level: 2
---

# 세 도구는 서로 다른 일을 맡습니다

<div class="tool-flow">
  <div v-click>
    <strong>rustup</strong>
    Rust 도구 체인과 구성 요소를 설치하고 선택합니다.
  </div>
  <div v-click>
    <strong>rustc</strong>
    Rust 소스 코드를 컴파일합니다.
  </div>
  <div v-click>
    <strong>Cargo</strong>
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

- 기본 호스트와 설치된 도구 체인
- 현재 디렉터리에서 선택된 도구 체인
- 설치된 rustfmt, Clippy 같은 구성 요소
- 설치된 컴파일 대상

<!--
여기서는 실제 터미널로 전환해 rustup show를 실행합니다.
출력 전체를 읽기보다 active toolchain과 installed targets 위치를 먼저 짚습니다.
-->

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
- nightly가 필요한 실험을 확인하더라도 사용자 기본값까지 바꿀 필요는 없습니다.
- 지정한 도구 체인이 설치되어 있지 않으면 먼저 설치해야 합니다.

<div class="takeaway">잠깐 확인할 때는 변경 범위가 가장 작은 방법부터 선택합니다.</div>

---
level: 2
---

# 팀이 함께 쓸 버전은 저장소에 선언합니다

<<< ../../examples/toolchain/rust-toolchain.toml toml {lines:true}

<div class="grid grid-cols-3 gap-6 mt-7">
  <div><strong>channel</strong><br><span class="muted">채널 또는 정확한 버전</span></div>
  <div><strong>profile</strong><br><span class="muted">설치 구성의 크기</span></div>
  <div><strong>components</strong><br><span class="muted">프로젝트에 필요한 도구</span></div>
</div>

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
      <td>설치·갱신 시점에 따라 실제 버전이 달라짐</td>
    </tr>
    <tr>
      <td><code>1.91.0</code></td>
      <td>개발자와 CI의 결과를 맞추기 쉬움</td>
      <td>버전을 직접 올리고 전체 검증해야 함</td>
    </tr>
  </tbody>
</table>

<div class="takeaway">재현성과 갱신 비용 가운데 프로젝트에 중요한 쪽을 선택합니다.</div>

---
level: 2
---

# 실행할 버전과 지원할 버전은 다른 약속입니다

<div class="grid grid-cols-2 gap-10 mt-8">
  <div>
    <h3><code>rust-toolchain.toml</code></h3>
    <p>이 저장소에서 rustup이 선택할 도구 체인을 정합니다.</p>
  </div>
  <div>
    <h3><code>Cargo.toml</code>의 <code>rust-version</code></h3>
    <p>패키지가 지원하는 가장 오래된 Rust 버전을 선언합니다.</p>
  </div>
</div>

<div class="command mt-8">make msrv</div>

<div class="takeaway">도구 체인을 올렸다고 지원하는 가장 오래된 버전까지 저절로 바뀌지는 않습니다.</div>

---
level: 2
---

# 파일보다 실제 선택 결과를 확인합니다

<div class="command">rustup show active-toolchain</div>
<div class="command">rustc --version</div>
<div class="command">cargo --version</div>

설정 파일을 추가했다고 끝난 것이 아닙니다. 그 디렉터리에서 실제로 선택된 버전과
필요한 구성 요소가 맞는지 확인해야 합니다.

---
level: 2
---

# 이름이 비슷해도 바꾸는 대상은 다릅니다

<div class="grid grid-cols-2 gap-10 mt-10">
  <div>
    <h2><code>rustup update stable</code></h2>
    <p>Rust 컴파일러와 도구 체인을 갱신합니다.</p>
  </div>
  <div>
    <h2><code>cargo update</code></h2>
    <p>프로젝트 의존성을 다시 선택하고 <code>Cargo.lock</code>을 갱신합니다.</p>
  </div>
</div>

<div class="takeaway">명령을 실행하기 전에 어느 파일과 어느 환경이 달라지는지 말해 봅니다.</div>

---
level: 2
layout: center
class: section
---

# 프로젝트 환경과 지원 범위는<br>확인 가능한 선언이어야 합니다

두 선언을 구분하고, 실제 선택된 도구 체인과 지원 범위의 검증 결과를 확인하세요.

<!--
다음 편에서는 이 환경에서 cargo check를 실행하고 컴파일 오류를 읽습니다.
-->
