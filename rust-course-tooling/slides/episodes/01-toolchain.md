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

`rustup`, `rustc`, `cargo`를 구분하고 팀의 Rust 버전을 맞춥니다.

<!--
이번 편의 핵심은 명령을 많이 외우는 것이 아닙니다.
비슷해 보이는 세 도구가 각각 무엇을 바꾸는지 구분하는 데 집중합니다.
-->

<!--
교재 대응: book/src/index.md > 우아한 Rust 중급: 개발 도구

이 강의에서는 Rust의 표준 개발 도구를 정확하고 능숙하게 사용하는 방법을 배웁니다.
코드를 작성한 뒤에는 형식을 맞추고, 의심스러운 부분을 찾고, 동작을 테스트해야 합니다. 이 교재에서는 Rust 프로젝트에서 자주 사용하는 rustup, cargo
check, rustfmt, cargo clippy, cargo update, cargo doc, cargo test를 하나의 개발·검증 흐름 속에서 익힙니다. 도구의 출력을 그대로
따라 하는 수준을 넘어서, 각 도구가 무엇을 확인하는지 알아봅니다. 도구의 제안을 적용할지 판단하고, 그 근거를 코드의 목적과 테스트 결과로 설명하는 것이 이 강의의
목표입니다. “우아한 프로그래밍 언어 Rust 입문” 강의를 들었다고 가정합니다. 목차에서 ◆가 붙은 주제는 전체 동영상 과정에서 이어서 다룹니다.
-->

---
level: 2
---

# 학습 목표

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

<!--
교재 대응: book/src/index.md > 학습 목표

이 과정을 마치면 다음 작업을 할 수 있습니다. • rustup으로 툴체인(toolchain), 컴포넌트(component), 컴파일 대상을 확인하고 관리합니다. •
rust-toolchain.toml로 프로젝트가 사용할 Rust 버전과 도구를 설정합니다. • rust-toolchain.toml과 Cargo.toml의
rust-version이 맡는 역할을 구분합니다. • 카고(Cargo) 타깃을 구분하고 cargo check의 검사 범위를 지정합니다. • 컴파일 오류에서 오류 코드부터
위치, 비고, 도움말까지 순서대로 읽습니다. • cargo fmt와 cargo fmt --check의 쓰임을 구분합니다. • 클리피를 모든 카고 타깃에 실행하고
린트(lint)가 지적한 문제와 제안 이유를 이해합니다. • 교육용 코드나 의도적인 구현에서 린트를 제한적으로 허용하고 그 이유를 남깁니다. • VS Code와
Zed에서 저장할 때 rustfmt를 실행하고 클리피 진단을 표시하도록 설정합니다. • cargo fix와 cargo clippy --fix가 바꾼 코드를 diff와
테스트로 검토합니다. • Cargo.toml과 Cargo.lock의 역할을 구분하고 의존성을 제한적으로 업데이트합니다. • cargo tree로 의존성이 들어온 경로와
활성화된 피처(feature)를 찾습니다. • 기본 구성, 기본 피처를 끈 구성, 모든 피처를 켠 구성을 각각 검사합니다. • cargo doc으로 생성한 API
문서에서 공개 항목을 찾고 사용 순서를 확인합니다. • 기본 assertion을 사용하고 검사할 내용에 따라 단위 테스트와 통합 테스트를 알맞은 위치에 둡니다. •
오류를 반환하는 테스트와 panic하거나 평소 제외한 테스트를 실행합니다. • 공개 API의 사용법과 허용되지 않는 사용법을 문서…
-->

---
level: 2
---

# 강의 프로젝트 내려받기

```bash
git clone https://github.com/hatemogi/rust-course-intermediate.git
cd rust-course-intermediate/rust-course-tooling
```

<div class="takeaway"><code>rust-course-tooling</code> 디렉터리에서 교재의 명령과 예제를 실행합니다.</div>

<!--
교재 대응: book/src/index.md > 강의 프로젝트 내려받기

이 교재에서 사용하는 예제와 실습 코드는 GitHub 강의 저장소에서 내려받을 수
있습니다. 저장소를 복제한 뒤 `rust-course-tooling` 디렉터리로 이동합니다. 이후
별도의 디렉터리를 지정하지 않은 명령은 이 디렉터리를 기준으로 실행합니다.
-->

---
level: 2
---

# `rustup`·`rustc`·`cargo`의 역할

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
    <strong>cargo</strong>
    패키지, 의존성, 빌드와 테스트 작업을 관리합니다.
  </div>
</div>

<div v-click class="takeaway">무엇을 확인하거나 바꾸려는지 먼저 정하면 명령을 고르기 쉬워집니다.</div>

<!--
교재 대응: book/src/toolchain/01-rustup-basics.md > rustup·rustc·cargo의 역할

Rust 개발 환경에는 rustup·rustc·cargo처럼 이름이 비슷하지만 역할이 다른 도구가 함께 설치됩니다. • rustup은 Rust
툴체인(toolchain)과 컴포넌트(component)를 설치하고 선택합니다. • rustc는 Rust 소스 코드를 컴파일합니다. • cargo는 패키지, 의존성,
빌드와 테스트 작업을 관리합니다. rustup --version은 rustup 자체의 버전입니다. 현재 사용하는 Rust 컴파일러 버전은 rustc --version으로
확인하세요.
-->

---
level: 2
---

# 툴체인과 컴포넌트

<div class="grid grid-cols-2 gap-12 mt-10">
  <div v-click>
    <h2>툴체인<sub>toolchain</sub></h2>
    <p>특정 Rust 버전의 <code>rustc</code>, 카고, 표준 라이브러리와 관련 도구를 묶은 설치 단위입니다.</p>
    <p><code>stable</code> · <code>nightly</code> · <code>1.94.0</code></p>
  </div>
  <div v-click>
    <h2>컴포넌트<sub>component</sub></h2>
    <p>툴체인을 이루는 개별 항목이며 필요에 따라 추가하거나 제거할 수 있습니다.</p>
    <p><code>rustfmt</code> · <code>clippy</code> · <code>rust-docs</code></p>
  </div>
</div>

<div v-click class="takeaway compact">같은 컴포넌트도 툴체인마다 따로 설치합니다.</div>

<!--
교재 대응: book/src/toolchain/01-rustup-basics.md > 참고: 툴체인과 컴포넌트

툴체인은 특정 Rust 버전의 `rustc`, `cargo`, 표준 라이브러리와 관련 도구를 묶은
설치 단위입니다. `stable`, `nightly`, `1.94.0`처럼 툴체인을 구분할 수
있습니다. 컴포넌트는 툴체인을 이루는 개별 항목입니다. 선택 컴포넌트는 필요에 따라
추가하거나 제거할 수 있으며, 같은 컴포넌트도 툴체인마다 따로 설치합니다.
-->

---
level: 2
---

# `rustup`·`rustc`·`cargo`의 역할

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

<!--
교재 대응: book/src/toolchain/01-rustup-basics.md > rustup·rustc·cargo의 역할

Rust 개발 환경에는 rustup·rustc·cargo처럼 이름이 비슷하지만 역할이 다른 도구가 함께 설치됩니다. rustup --version은 rustup
자체의 버전입니다. 현재 사용하는 Rust 컴파일러 버전은 rustc --version으로 확인하세요. 참고: 툴체인과 컴포넌트 툴체인은 특정 Rust 버전의
rustc, cargo, 표준 라이브러리, 관련 도구를 묶은 설치 단위입니다. stable, nightly, 1.94.0처럼 툴체인을 구분할 수 있습니다. 컴포넌트는
툴체인을 이루는 개별 항목입니다. 선택 컴포넌트는 필요에 따라 추가하거나 제거할 수 있으며, 같은 컴포넌트도 툴체인마다 따로 설치합니다.
-->

---
level: 2
---

# 현재 상태 확인하기

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

<!--
교재 대응: book/src/toolchain/01-rustup-basics.md > 현재 설치 상태 확인하기

다음 명령은 기본 호스트, 설치된 툴체인, 현재 선택된 툴체인, 설치된 컴파일 대상을 보여 줍니다. 일반적인 프로젝트에서는 안정판인 stable을 사용합니다. 실험
기능이 꼭 필요한 경우에만 범위를 좁혀 nightly를 검토하세요.
-->

---
level: 2
---

# 툴체인 설치하고 업데이트하기

```bash
rustup toolchain install stable
rustup update stable
rustup default stable
```

- `toolchain install`은 stable 툴체인이 없으면 설치합니다.
- `update stable`은 설치된 stable 툴체인을 최신 stable 릴리스로 업데이트합니다.
- `default stable`은 별도 지정이 없는 디렉터리의 사용자 기본값을 바꿉니다.

<div class="takeaway compact">사용자 컴퓨터의 Rust 환경이 바뀌므로 실행 전에 필요한 범위를 확인합니다.</div>

<!--
교재 대응: book/src/toolchain/01-rustup-basics.md > 툴체인 설치하고 업데이트하기

• toolchain install은 해당 툴체인이 없으면 설치합니다. • update stable은 현재 컴퓨터에 설치된 stable 툴체인을 stable 채널의 최신
릴리스로 바꿉니다. • default는 별도 지정이 없는 디렉터리에서 사용할 기본 툴체인을 정합니다. 이 명령들은 사용자 컴퓨터의 Rust 환경을 바꾸고 일부는 파일을
내려받습니다. 팀 프로젝트의 명령을 따라 하기 전에 변경 범위와 필요한 버전을 확인하세요.
-->

---
level: 2
---

# 한 번만 다른 툴체인 사용하기

```bash
rustc +stable --version
cargo +stable check
cargo +nightly fmt
```

- `+toolchain`은 **그 명령에만** 적용됩니다.
- nightly는 매일 업데이트되는 개발 채널입니다. 실험 기능이 꼭 필요할 때만 씁니다.
- 지정한 툴체인이 설치되어 있지 않으면 먼저 설치해야 합니다.

<div class="takeaway">잠깐 확인할 때는 변경 범위가 가장 작은 방법부터 선택합니다.</div>

<!--
교재 대응: book/src/toolchain/01-rustup-basics.md > 한 번만 다른 툴체인 사용하기

명령 이름 뒤에 +toolchain을 붙이면 기본값을 바꾸지 않고 그 명령에만 다른 툴체인을 사용할 수 있습니다. 참고: 설치되지 않은 툴체인 기본 설정에서는
nightly가 설치되어 있지 않다면 rustup이 먼저 설치한 뒤 명령을 실행합니다. 단순히 최신 버전처럼 보인다는 이유로 프로젝트 전체의 기본값을 nightly로
바꾸지 마세요. nightly는 매일 업데이트되는 Rust 개발 채널이며, stable에 아직 포함되지 않은 실험 기능이 꼭 필요할 때 주로 사용합니다.
-->

---
level: 2
---

# 컴포넌트 관리하기

```bash
rustup component add rustfmt clippy
rustup component add --toolchain stable rustfmt clippy
rustup component list --installed
```

<div class="grid grid-cols-2 gap-10 mt-8">
  <div><h3><code>rustfmt</code></h3>Rust 코드를 정해진 형식으로 자동 정리합니다.</div>
  <div><h3>클리피<sub>Clippy</sub></h3>실수 가능성이나 불필요하게 복잡한 표현을 찾는 공식 린트 도구입니다.</div>
</div>

<!--
교재 대응: book/src/toolchain/01-rustup-basics.md > 컴포넌트 관리하기

다음 두 도구는 툴체인에 추가하는 컴포넌트입니다. 특정 툴체인에 추가하려면 --toolchain을 사용합니다. 참고: rustfmt와 클리피 rustfmt는 Rust
코드를 정해진 형식으로 자동 정리합니다. 클리피(Clippy)는 코드를 분석해 실수 가능성이나 불필요하게 복잡한 표현을 알려 주는 공식 린트 도구입니다.
-->

---
level: 2
---

# 컴파일 타깃 관리하기

```bash
rustup target list --installed
rustup target add wasm32-unknown-unknown
rustup target remove wasm32-unknown-unknown
```

웹어셈블리<sub>WebAssembly</sub>나 다른 운영체제·CPU 아키텍처용 결과물을 만들 때
필요한 컴파일 타깃을 설치합니다.

<div class="takeaway compact">크로스 컴파일<sub>cross-compilation</sub>에는 대상에 따라 링커와 시스템 라이브러리도 필요합니다.</div>

<!--
교재 대응: book/src/toolchain/01-rustup-basics.md > 컴파일 타깃 관리하기

다른 운영체제나 웹어셈블리(WebAssembly)용 결과물이 필요하면 타깃을 추가합니다. 참고: 크로스 컴파일 준비 크로스 컴파일(cross-compilation)은
현재 컴퓨터가 아닌 다른 운영체제·CPU 아키텍처·실행 환경에서 사용할 결과물을 만드는 작업입니다. 타깃을 추가하는 것만으로 모든 크로스 컴파일 준비가 끝나는 것은
아닙니다. 대상에 따라 링커와 시스템 라이브러리가 별도로 필요할 수 있습니다.
-->

---
level: 2
---

# 프로젝트 설정 파일 만들기

<<< ../../rust-toolchain.toml toml {lines:true}

<div class="grid grid-cols-3 gap-6 mt-7">
  <div><strong>channel</strong><br><span class="muted">채널 또는 정확한 버전</span></div>
  <div><strong>profile</strong><br><span class="muted">설치 구성의 크기</span></div>
  <div><strong>components</strong><br><span class="muted">프로젝트에 필요한 도구</span></div>
</div>

<div class="takeaway compact">파일을 저장소 루트에 저장하고 커밋하면 개발자와 CI가 같은 설정을 읽습니다.</div>

<!--
교재 대응: book/src/toolchain/02-project-toolchain.md > 프로젝트 설정 파일 만들기

프로젝트 저장소 루트에 rust-toolchain.toml 파일을 만들고 다음 내용을 저장합니다. 이 강의에서는 정확한 버전을 지정합니다. • channel은 사용할
Rust 채널이나 정확한 버전을 정합니다. • profile = "minimal"은 문서 같은 추가 컴포넌트(component)를 제외한 최소한의 설치 구성을
선택합니다. • components는 프로젝트 검증에 필요한 rustfmt와 클리피(Clippy)를 함께 설치합니다. 저장소에 이 파일을 커밋하면 다른 개발자나 CI가
모두 같은 툴체인 설정을 사용합니다. 크로스 컴파일 대상까지 공통으로 필요하다면 `targets = ["wasm32-unknown-unknown"]`처럼 추가할 수 있습니다.
아직 설치되지 않은 툴체인이나 컴포넌트는 처음 명령을 실행할 때 내려받을 수 있으므로 네트워크가 제한된 환경에서는 설치 상태를 미리 확인합니다.
-->

---
level: 2
---

# `stable`과 정확한 버전 가운데 고르기

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

<!--
교재 대응: book/src/toolchain/02-project-toolchain.md > stable과 정확한 버전 가운데 고르기

stable은 환경을 설치하거나 업데이트한 시점에 따라 실제 컴파일러 버전이 달라질 수 있습니다. 새 Rust 개선을 자연스럽게 따라갈 수 있지만 모든 사람이 같은
버전을 쓴다는 보장은 약합니다. 정확한 버전을 지정하면 개발자와 CI가 같은 컴파일러로 검사할 수 있습니다. 다만 컴파일러의 버그 수정이나 새 린트(lint)를
적용하려면 channel의 버전을 직접 올린 뒤 전체 검증을 실행해야 합니다. 같은 결과를 재현하기 쉬운 점과 버전을 관리하는 수고를 함께 고려해 선택하세요.
-->

---
level: 2
---

# `rust-version`과 구분하기

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

<!--
교재 대응: book/src/toolchain/02-project-toolchain.md > rust-version과 구분하기

rust-toolchain.toml은 이 저장소에서 명령을 실행할 때 rustup이 선택할 툴체인을 정합니다. 반면 Cargo.toml의 rust-version은
패키지가 지원하는 가장 오래된 Rust 버전을 카고(Cargo)에 알립니다. 두 값은 목적이 다릅니다. 개발자와 CI가 예제처럼 1.94.0으로 작업하더라도 패키지가
rust-version = "1.85"를 적었다면, 지원 범위를 지키는지 확인하려고 1.85에서도 별도 검사를 실행할 수 있습니다. rust-toolchain.toml의
버전을 올렸다고 패키지가 지원하는 가장 오래된 버전까지 저절로 바뀌지는 않습니다. MSRV는 Minimum Supported Rust Version의 약자이며, 패키지가
지원하는 가장 오래된 Rust 버전을 뜻합니다. rust-version을 적으면 그보다 오래된 카고가 분명한 오류를 내고 의존성을 선택할 때도 이 정보를 사용할 수
있습니다. 실제로 지원하지 않는 버전을 추측해 적지 말고 자동 검사에서 확인하는 버전을 기록합니다. 이 강의에서 make check는 현재 툴체인의 전체 검증을,
make msrv는 rust-version에 적은 버전의 컴파일·테스트·문서 테스트를 실행합니다. make와 Makefile은 Rust 전용 도구가 아니라 긴 검증 명령을
같은 순서로 반복하기 위한 도구입니다.
-->

---
level: 2
---

# 디렉터리 override

```bash
rustup override set stable
rustup override list
rustup override unset
```

- override는 사용자 rustup 설정에 저장되고 Git으로 공유되지 않습니다.
- 잠깐 로컬에서 확인한 뒤에는 불필요한 override를 제거합니다.

<div class="takeaway">팀이 함께 따라야 하는 버전은 저장소의 <code>rust-toolchain.toml</code>에 적습니다.</div>

<!--
교재 대응: book/src/toolchain/02-project-toolchain.md > 디렉터리 override

파일을 만들지 않고 현재 디렉터리에 툴체인을 지정할 수도 있습니다. override는 사용자 rustup 설정에 저장되고 Git으로 공유되지 않습니다. 잠깐 로컬에서
확인할 때는 편리하지만 팀이 함께 따라야 하는 버전은 대개 rust-toolchain.toml이 더 분명합니다. 실습 뒤에는 불필요한 override가 남지 않았는지
확인하세요.
-->

---
level: 2
---

# 선택된 툴체인 확인하기

<div class="command">rustup show active-toolchain</div>
<div class="command">rustc --version</div>
<div class="command">cargo --version</div>

설정 파일을 추가했다고 끝난 것이 아닙니다. 그 디렉터리에서 실제로 선택된 버전과
필요한 컴포넌트가 맞는지 확인해야 합니다.

<!--
교재 대응: book/src/toolchain/02-project-toolchain.md > 선택된 툴체인 확인하기

프로젝트 디렉터리에서 다음 명령을 실행합니다. 설정 파일을 추가했다고 끝내지 말고 실제 선택된 버전과 필요한 컴포넌트를 확인하세요. 버전을 올릴 때는 포맷, 클리피,
컴파일, 테스트, 문서 테스트를 모두 실행해 새 컴파일러에서 달라진 결과가 없는지 검토합니다.
-->

---
level: 2
---

# AI가 만든 설정은 지원 범위로 검토합니다

- nightly 기능을 실제로 사용하는가?
- 필요한 `rustfmt`와 클리피가 포함됐는가?
- 불필요한 컴파일 타깃을 모두에게 설치하지 않는가?
- 정확한 버전의 업데이트 책임과 시점이 정해졌는가?
- MSRV와 CI가 같은 설정을 실제로 검사하는가?

<div class="takeaway compact">설정 파일의 모양보다 프로젝트가 약속한 지원 범위를 먼저 확인합니다.</div>

<!--
교재 대응: book/src/toolchain/02-project-toolchain.md > 프로젝트에서 Rust 버전을 선택하고 확인하기; book/src/toolchain/02-project-toolchain.md > AI가 만든 설정 검토하기

`rustup default`는 사용자 컴퓨터 전체의 기본값을 바꿉니다. 프로젝트마다 필요한
Rust 버전이 다르면 저장소 루트의 `rust-toolchain.toml`에 사용할 툴체인을
설정합니다. AI가 습관적으로 nightly나 오래된 정확한 버전을 고정할 수 있으므로,
nightly 기능을 실제로 사용하는지, rustfmt와 클리피가 필요한 컴포넌트에
포함됐는지, 필요하지 않은 target을 모든 개발자에게 설치하게 하지는 않는지
확인합니다. 정확한 버전을 누가 언제 업데이트할지 정하고, `rust-version`으로
약속한 가장 오래된 버전을 자동 검사에서 확인하며, CI도 같은 파일을 읽도록
구성해야 합니다.
-->

---
level: 2
---

# 혼동하기 쉬운 두 업데이트 명령

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

<!--
교재 대응: book/src/toolchain/01-rustup-basics.md > 혼동하기 쉬운 두 업데이트 명령

첫 번째 명령은 Rust 컴파일러와 툴체인을 업데이트합니다. 두 번째 명령은 현재 프로젝트가 사용하는 크레이트(crate) 버전을 다시 선택하고 Cargo.lock을
업데이트합니다. 이름은 비슷하지만 바꾸는 대상이 완전히 다릅니다. 참고: 크레이트 크레이트는 Rust 컴파일러가 한 번에 컴파일하는 코드 단위입니다. 라이브러리나 실행
파일 형태로 만들 수 있습니다. crates.io에는 외부 라이브러리 패키지가 크레이트 단위로 공개되어 있으며, 이를 Cargo.toml의 의존성에 추가해 사용할 수
있습니다.
-->
