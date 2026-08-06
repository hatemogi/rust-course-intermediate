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

<!--
교재 대응: book/src/cargo/11-dependencies-lockfile.md > 의존성과 Cargo.lock 관리하기

Cargo.toml에는 프로젝트에서 사용할 수 있는 의존성의 버전 범위를 적습니다. Cargo.lock에는 카고(Cargo)가 그 범위 안에서 실제로 선택한 직접·간접
의존성의 정확한 버전과 출처가 기록됩니다. Cargo.lock은 카고가 관리하므로 직접 편집하지 않습니다. 예를 들어 Cargo.toml에 일반 의존성과 개발할 때만
쓰는 의존성을 다음처럼 나누어 적을 수 있습니다. version = "1"은 1.x 버전 가운데 호환되는 버전을 카고가 선택할 수 있다는 뜻입니다. 실제로 선택된 정확한
버전은 Cargo.lock에서 확인합니다.
-->

---
level: 2
---

# 의존성과 `Cargo.lock` 관리하기

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

<!--
교재 대응: book/src/cargo/11-dependencies-lockfile.md > 의존성과 Cargo.lock 관리하기

Cargo.toml에는 프로젝트에서 사용할 수 있는 의존성의 버전 범위를 적습니다. Cargo.lock에는 카고(Cargo)가 그 범위 안에서 실제로 선택한 직접·간접
의존성의 정확한 버전과 출처가 기록됩니다. Cargo.lock은 카고가 관리하므로 직접 편집하지 않습니다. 예를 들어 Cargo.toml에 일반 의존성과 개발할 때만
쓰는 의존성을 다음처럼 나누어 적을 수 있습니다. version = "1"은 1.x 버전 가운데 호환되는 버전을 카고가 선택할 수 있다는 뜻입니다. 실제로 선택된 정확한
버전은 Cargo.lock에서 확인합니다.
-->

---
level: 2
---

# 의존성 추가하고 제거하기

```bash
cargo add serde --features derive
cargo add pretty_assertions --dev

cargo remove serde
cargo remove pretty_assertions --dev
```

- 이름이 비슷한 다른 패키지를 추가하지 않았는가?
- 필요하지 않은 피처<sub>feature</sub>가 켜지지 않았는가?
- 직접 의존성 때문에 어떤 간접 의존성이 들어왔는가?

<!--
교재 대응: book/src/cargo/11-dependencies-lockfile.md > 의존성 추가하고 제거하기

cargo add는 의존성 항목을 알맞은 표에 추가합니다. 첫 번째 명령은 [dependencies]에 serde를 추가하고 derive 피처(feature)를 켭니다.
두 번째 명령은 테스트와 예제에서 사용할 개발 의존성을 [dev-dependencies]에 추가합니다. 더 이상 필요하지 않은 의존성은 다음처럼 제거합니다. 명령을
실행한 뒤에는 Cargo.toml과 Cargo.lock의 diff를 함께 읽으세요. 이름이 비슷한 다른 패키지를 추가하지 않았는지, 불필요한 피처가 켜지지 않았는지
확인해야 합니다.
-->

---
level: 2
---

# 잠금 파일 업데이트하기

```bash
cargo update
cargo update -p serde
```

패키지 하나를 지정해도 그 패키지가 사용하는 간접 의존성은 함께 달라질 수
있습니다. 잠금 파일의 변경량과 전체 테스트 결과를 확인합니다.

<div class="takeaway">“버전 하나를 올렸다”가 아니라 실제로 달라진 의존성 묶음을 검토합니다.</div>

<!--
교재 대응: book/src/cargo/11-dependencies-lockfile.md > 잠금 파일 업데이트하기

cargo update는 Cargo.toml이 허용하는 범위 안에서 의존성을 다시 선택하고 Cargo.lock을 바꿉니다. 첫 번째 명령은 업데이트할 수 있는 의존성을
모두 다시 선택합니다. 변경 범위를 줄이고 싶다면 두 번째 명령처럼 패키지를 지정합니다. 직접 의존성 하나를 업데이트해도 그 패키지가 사용하는 간접 의존성이 함께 달라질
수 있으므로 잠금 파일의 변경량과 테스트 결과를 확인해야 합니다. Cargo.lock은 보통 Git에 포함합니다. 그러면 팀과 자동 검사 환경이 같은 버전의 의존성을
사용합니다. 라이브러리를 배포할 때 잠금 파일이 어떻게 사용되는지는 실행 프로그램과 차이가 있지만, 저장소에 포함할지 확신하기 어렵다면 포함하는 것이 카고의 권장
방식입니다.
-->

---
level: 2
---

# 잠금 파일과 네트워크 사용 제한하기

<table class="compare">
  <thead><tr><th>옵션</th><th>동작</th></tr></thead>
  <tbody>
    <tr><td><code>--locked</code></td><td>기존 잠금 파일을 바꿔야 하면 실패</td></tr>
    <tr><td><code>--offline</code></td><td>네트워크 없이 이미 받은 자료만 사용</td></tr>
    <tr><td><code>--frozen</code></td><td><code>--locked</code>와 <code>--offline</code>을 함께 적용</td></tr>
  </tbody>
</table>

`--offline`은 필요한 패키지를 미리 받지 않았다면 실패할 수 있습니다.

<!--
교재 대응: book/src/cargo/11-dependencies-lockfile.md > 잠금 파일과 네트워크 사용 제한하기

다음 옵션은 비슷해 보이지만 확인하는 내용이 다릅니다. 자동 검사에서는 의존성이 뜻밖에 바뀌지 않도록 --locked를 자주 사용합니다. --offline은 필요한
패키지를 미리 내려받지 않았다면 실패하며, 온라인에서 선택할 결과와 달라질 수도 있습니다. Cargo.toml과 Cargo.lock의 역할은 카고 공식 안내서에서 더
자세히 설명합니다.
-->

---
level: 2
---

# `cargo tree`로 의존성 관계 찾기

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

<!--
교재 대응: book/src/cargo/12-cargo-tree.md > cargo tree로 의존성 관계 찾기

의존성이 많아지면 Cargo.toml만 보고 실제 빌드에 포함되는 패키지를 일일이 알기 어려워집니다. cargo tree는 카고(Cargo)가 계산한 의존성 관계를 트리
형태로 보여 줍니다. 들여쓰기는 어떤 패키지가 다른 패키지를 가져왔는지를 나타냅니다. 같은 의존성 하위 구조가 이미 출력됐다면 카고는 (*)를 붙이고 반복해서 펼치지
않습니다.
-->

---
level: 2
---

# 특정 의존성이 들어온 경로 찾기

```bash
cargo tree -i serde
cargo tree --duplicates
```

<div class="grid grid-cols-2 gap-10 mt-8">
  <div><h3><code>-i</code></h3>어떤 상위 의존성이 그 패키지를 가져왔는지 찾습니다.</div>
  <div><h3><code>--duplicates</code></h3>여러 버전이 함께 포함된 패키지만 확인합니다.</div>
</div>

여러 버전이 항상 오류는 아니지만 빌드 시간과 실행 파일 크기를 살펴볼 근거가 됩니다.

<!--
교재 대응: book/src/cargo/12-cargo-tree.md > 특정 의존성이 들어온 경로 찾기

예상하지 않은 패키지가 보이면 관계를 거꾸로 출력합니다. 여러 버전이 함께 포함된 패키지만 확인하려면 다음 명령을 사용합니다. 같은 패키지의 여러 버전이 항상 오류인
것은 아닙니다. 하지만 빌드 시간과 실행 파일 크기가 늘어날 수 있으므로, 오래된 버전을 요구하는 상위 의존성을 업데이트할 수 있는지 살펴볼 근거가 됩니다.
-->

---
level: 2
---

# 활성화된 피처 찾기

```bash
cargo tree -e features
cargo tree -e features -i serde
```

출력이 길다면 패키지를 지정해 어느 경로에서 피처가 활성화됐는지 거꾸로
살펴봅니다.

<div class="takeaway">의존성 이름만 보지 말고 누가 가져왔고 어떤 피처를 켰는지 확인합니다.</div>

<!--
교재 대응: book/src/cargo/12-cargo-tree.md > 활성화된 피처 찾기

어떤 의존성 피처(feature)가 켜졌는지 보려면 관계 종류를 features로 지정합니다. 첫 번째 명령은 전체 피처 관계를 보여 줍니다. 출력이 길다면 두 번째
명령처럼 패키지를 지정해 어떤 경로에서 피처가 활성화됐는지 거꾸로 살펴봅니다. cargo tree는 선택한 플랫폼과 피처 구성을 기준으로 의존성 관계를 보여 줍니다.
실행하는 카고 명령과 옵션이 달라지면 실제로 컴파일되는 의존성도 달라질 수 있습니다. 명령의 옵션과 출력 해석은 카고의 cargo tree 문서에서 확인할 수 있습니다.
-->

---
level: 2
---

# 의존성과 `Cargo.lock` 관리하기

<ol class="step-list">
  <li><code>Cargo.toml</code>에서 허용 범위를 확인합니다.</li>
  <li><code>Cargo.lock</code>의 실제 변경을 읽습니다.</li>
  <li><code>cargo tree -i</code>로 들어온 경로를 찾습니다.</li>
  <li>필요한 피처만 켜졌는지 확인합니다.</li>
  <li><code>--locked</code>로 전체 검증을 다시 실행합니다.</li>
</ol>

<!--
교재 대응: book/src/cargo/11-dependencies-lockfile.md > 의존성과 Cargo.lock 관리하기

Cargo.toml에는 프로젝트에서 사용할 수 있는 의존성의 버전 범위를 적습니다. Cargo.lock에는 카고(Cargo)가 그 범위 안에서 실제로 선택한 직접·간접
의존성의 정확한 버전과 출처가 기록됩니다. Cargo.lock은 카고가 관리하므로 직접 편집하지 않습니다. 예를 들어 Cargo.toml에 일반 의존성과 개발할 때만
쓰는 의존성을 다음처럼 나누어 적을 수 있습니다. version = "1"은 1.x 버전 가운데 호환되는 버전을 카고가 선택할 수 있다는 뜻입니다. 실제로 선택된 정확한
버전은 Cargo.lock에서 확인합니다.
-->
