---
theme: default
title: 편집기 자동화와 cargo fix 검토
info: 우아한 Rust 중급 개발 도구 5편
colorSchema: dark
aspectRatio: 16/9
canvasWidth: 1100
transition: fade-out
lineNumbers: true
layout: cover
class: cover
---

<div class="episode-label">우아한 Rust 중급 · 개발 도구 05</div>

# 편집기 자동화와<br>`cargo fix` 검토

문제를 일찍 발견하되 자동 수정의 결과는 직접 검토합니다.

---
level: 2
---

# 작성 중 피드백과 최종 검증을 나눕니다

<div class="tool-flow">
  <div><strong>저장할 때</strong>rustfmt로 현재 파일의 형식을 맞춥니다.</div>
  <div><strong>작성하는 동안</strong>rust-analyzer가 클리피<sub>Clippy</sub> 진단을 보여 줍니다.</div>
  <div><strong>변경을 마칠 때</strong>터미널에서 저장소 전체 명령을 실행합니다.</div>
</div>

<div class="takeaway">편집기는 피드백을 앞당기지만 프로젝트 전체 검사를 대신하지 않습니다.</div>

---
level: 2
---

# 컴포넌트와 프로젝트 루트를 먼저 확인합니다

```bash
rustup component add rustfmt clippy
```

- 편집기에서는 `Cargo.toml`이 있는 프로젝트 루트를 엽니다.
- 파일 하나만 열면 rust-analyzer가 전체 타깃과 설정을 찾지 못할 수 있습니다.
- 프로젝트가 고정한 툴체인<sub>toolchain</sub>에
  컴포넌트<sub>component</sub>가 설치됐는지 확인합니다.

---
level: 2
---

# VS Code는 저장 시 포맷과 검사 명령을 설정합니다

```json
{
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.formatOnSave": true
  },
  "rust-analyzer.check.command": "clippy"
}
```

프로젝트 구성원과 공유하려면 `.vscode/settings.json`에, 개인 설정이면 사용자
설정에 둡니다.

---
level: 2
---

# Zed도 같은 동작을 설정할 수 있습니다

```json
{
  "languages": {
    "Rust": {
      "format_on_save": "on",
      "formatter": "language_server"
    }
  },
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "check": { "command": "clippy" }
      }
    }
  }
}
```

---
level: 2
---

# 편집기에 경고가 없다는 사실만으로는 부족합니다

편집기 진단의 범위는 현재 연 프로젝트, 활성 피처<sub>feature</sub>, rust-analyzer 설정에 따라
달라질 수 있습니다.

```bash
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
```

<div class="takeaway">완료 기준은 편집기 화면이 아니라 저장소가 정한 명령으로 확인합니다.</div>

---
level: 2
---

# 자동 수정 전에는 작업 상태부터 확인합니다

```bash
git status --short
cargo fix --locked
```

`cargo fix`는 rustc가 적용 가능하다고 표시한 수정안을 소스 파일에 반영합니다.
작업 중인 변경과 자동 수정을 섞지 않도록 먼저 커밋하거나 깨끗한 작업 트리에서
실행하는 편이 좋습니다. 별도 타깃 옵션이 없으면 모든 카고<sub>Cargo</sub> 타깃을
검사합니다.

---
level: 2
---

# 클리피 수정도 검사 범위를 명시합니다

```bash
cargo clippy --fix --no-default-features --locked
cargo clippy --fix --all-features --locked
```

- 현재 활성화된 피처와 플랫폼의 코드만 분석됩니다.
- `--all-features`에서 빠지는 `cfg(not(feature = "..."))` 분기도 확인합니다.
- 다른 플랫폼의 `#[cfg]` 코드는 해당 target과 도구가 준비되어야 합니다.
- “적용 가능”은 프로그램의 요구 사항까지 옳다는 뜻이 아닙니다.

---
level: 2
---

# 자동 수정 뒤의 검토 과정은 줄어들지 않습니다

<ol class="step-list">
  <li><code>git diff</code>로 바뀐 파일과 줄을 읽습니다.</li>
  <li>공개 API와 오류 처리 방식이 같은지 확인합니다.</li>
  <li>rustfmt와 클리피를 다시 실행합니다.</li>
  <li>관련 테스트와 피처 구성을 검사합니다.</li>
  <li>의도와 다른 수정은 직접 고칩니다.</li>
</ol>

---
level: 2
layout: center
class: section
---

# 자동화는 판단을 없애지 않고<br>판단할 시점을 앞당깁니다

수정 전 상태와 diff, 다시 실행한 검사를 함께 남기세요.
