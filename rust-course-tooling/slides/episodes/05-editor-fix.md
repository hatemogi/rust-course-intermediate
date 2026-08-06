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

<!--
교재 대응: book/src/cargo/10-cargo-fix.md > cargo fix와 자동 수정 검토하기

컴파일러와 클리피(Clippy)의 진단에는 도구가 안전하게 적용할 수 있다고 판단한 수정안이 포함되기도 합니다. cargo fix는 rustc가 제안한 수정안을 소스
파일에 적용합니다. cargo fix는 내부에서 cargo check를 실행하며, 기본적으로 모든 카고(Cargo) 타깃을 대상으로 삼습니다. 실행 결과는 소스 파일을
직접 바꾸므로 먼저 커밋하거나 작업 트리를 깨끗하게 만들어 두는 편이 좋습니다. 변경 중인 파일이 있으면 카고가 실행을 거부하므로 무심코 기존 작업과 자동 수정을 섞는
일을 막을 수 있습니다. 클리피가 제안한 수정안을 적용하려면 다음 명령을 사용합니다.
-->

---
level: 2
---

# VS Code와 Zed에서 rustfmt와 클리피 사용하기

<div class="tool-flow">
  <div><strong>저장할 때</strong>rustfmt로 현재 파일의 형식을 맞춥니다.</div>
  <div><strong>작성하는 동안</strong>rust-analyzer가 클리피<sub>Clippy</sub> 진단을 보여 줍니다.</div>
  <div><strong>변경을 마칠 때</strong>터미널에서 저장소 전체 명령을 실행합니다.</div>
</div>

<div class="takeaway">편집기는 피드백을 앞당기지만 프로젝트 전체 검사를 대신하지 않습니다.</div>

<!--
교재 대응: book/src/editors/09-vscode-zed.md > VS Code와 Zed에서 rustfmt와 클리피 사용하기

터미널에서 cargo fmt와 cargo clippy를 실행하는 방법을 알았다면 편집기에서도 같은 도구를 사용할 수 있습니다. 저장할 때 rustfmt로 파일을
정리하고, rust-analyzer가 cargo check 대신 cargo clippy를 실행하게 설정하면 코드를 작성하는 동안 문제를 더 일찍 발견할 수 있습니다.
먼저 프로젝트가 사용하는 툴체인(toolchain)에 필요한 컴포넌트(component)가 있는지 확인합니다. 편집기에서는 카고(Cargo) 프로젝트의 루트, 즉
Cargo.toml이 있는 디렉터리를 여세요. 하위 파일 하나만 열면 rust-analyzer가 프로젝트 전체의 타깃과 설정을 제대로 찾지 못할 수 있습니다.
-->

---
level: 2
---

# VS Code와 Zed에서 rustfmt와 클리피 사용하기

```bash
rustup component add rustfmt clippy
```

- 편집기에서는 `Cargo.toml`이 있는 프로젝트 루트를 엽니다.
- 파일 하나만 열면 rust-analyzer가 전체 타깃과 설정을 찾지 못할 수 있습니다.
- 프로젝트가 고정한 툴체인<sub>toolchain</sub>에
  컴포넌트<sub>component</sub>가 설치됐는지 확인합니다.

<!--
교재 대응: book/src/editors/09-vscode-zed.md > VS Code와 Zed에서 rustfmt와 클리피 사용하기

터미널에서 cargo fmt와 cargo clippy를 실행하는 방법을 알았다면 편집기에서도 같은 도구를 사용할 수 있습니다. 저장할 때 rustfmt로 파일을
정리하고, rust-analyzer가 cargo check 대신 cargo clippy를 실행하게 설정하면 코드를 작성하는 동안 문제를 더 일찍 발견할 수 있습니다.
먼저 프로젝트가 사용하는 툴체인(toolchain)에 필요한 컴포넌트(component)가 있는지 확인합니다. 편집기에서는 카고(Cargo) 프로젝트의 루트, 즉
Cargo.toml이 있는 디렉터리를 여세요. 하위 파일 하나만 열면 rust-analyzer가 프로젝트 전체의 타깃과 설정을 제대로 찾지 못할 수 있습니다.
-->

---
level: 2
---

# VS Code 설정

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

<!--
교재 대응: book/src/editors/09-vscode-zed.md > VS Code 설정

VS Code에서는 공식 rust-analyzer 확장을 설치합니다. 설정 화면에서 다음 두 항목을 찾아 바꿀 수도 있고, 프로젝트의
.vscode/settings.json에 직접 적을 수도 있습니다. • editor.formatOnSave는 Rust 파일을 저장할 때 rust-analyzer를 통해
rustfmt를 실행합니다. • rust-analyzer.check.command를 clippy로 바꾸면 rust-analyzer가 저장한 코드를 검사할 때 cargo
clippy를 사용합니다. 진단은 코드와 Problems 창에 표시됩니다. • 설정을 프로젝트 구성원과 공유하려면 .vscode/settings.json을 Git에
포함합니다. 개인 설정으로만 쓰려면 VS Code의 사용자 설정에 같은 내용을 추가합니다. 명령 팔레트에서 Format Document를 실행하면 저장하지 않고도 현재
파일의 형식을 맞출 수 있습니다. 클리피(Clippy)가 제안한 빠른 수정은 적용하기 전에 코드의 동작과 공개 API가 그대로인지 확인하세요.
-->

---
level: 2
---

# Zed 설정

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

<!--
교재 대응: book/src/editors/09-vscode-zed.md > Zed 설정

Zed에는 Rust와 rust-analyzer 지원이 들어 있습니다. 프로젝트에서만 사용할 설정은 .zed/settings.json에 다음처럼 작성합니다. •
format_on_save를 on으로 두면 파일을 저장할 때 형식을 맞춥니다. • formatter를 language_server로 지정하면 Rust 파일은
rust-analyzer를 거쳐 rustfmt로 정리됩니다. • check.command를 clippy로 지정하면 rust-analyzer의 진단에 클리피 결과가
포함됩니다. 이 설정을 바꾼 뒤 기존 창에 바로 반영되지 않으면 rust-analyzer를 다시 시작하거나 프로젝트를 다시 여세요. 모든 프로젝트에서 같은 설정을
사용하려면 Zed의 사용자 settings.json에 넣을 수 있습니다. Zed의 Rust 지원과 설정 형식은 Zed의 Rust 문서와 전체 설정 문서에서 확인할 수
있습니다.
-->

---
level: 2
---

# 편집기 진단과 전체 검증 구분하기

편집기 진단의 범위는 현재 연 프로젝트, 활성 피처<sub>feature</sub>, rust-analyzer 설정에 따라
달라질 수 있습니다.

```bash
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
```

<div class="takeaway">완료 기준은 편집기 화면이 아니라 저장소가 정한 명령으로 확인합니다.</div>

<!--
교재 대응: book/src/editors/09-vscode-zed.md > 편집기 진단과 전체 검증 구분하기

편집기가 보여 주는 진단은 코드를 작성할 때 빠르게 확인하기 위한 것입니다. 현재 열어 둔 프로젝트의 기본 타깃과 rust-analyzer 설정에 따라 검사 범위가
달라질 수 있으므로, 편집기에 경고가 없다는 사실만으로 모든 검증이 끝났다고 판단하면 안 됩니다. 변경을 마치기 전에는 터미널에서 저장소가 정한 명령을 따로 실행하세요.
이 강의 프로젝트에서는 두 명령뿐 아니라 테스트, 문서 테스트, mdBook 빌드까지 make check로 확인합니다.
-->

---
level: 2
---

# `cargo fix`와 자동 수정 검토하기

```bash
git status --short
cargo fix --locked
```

`cargo fix`는 rustc가 적용 가능하다고 표시한 수정안을 소스 파일에 반영합니다.
작업 중인 변경과 자동 수정을 섞지 않도록 먼저 커밋하거나 깨끗한 작업 트리에서
실행하는 편이 좋습니다. 별도 타깃 옵션이 없으면 모든 카고<sub>Cargo</sub> 타깃을
검사합니다.

<!--
교재 대응: book/src/cargo/10-cargo-fix.md > cargo fix와 자동 수정 검토하기

컴파일러와 클리피(Clippy)의 진단에는 도구가 안전하게 적용할 수 있다고 판단한 수정안이 포함되기도 합니다. cargo fix는 rustc가 제안한 수정안을 소스
파일에 적용합니다. cargo fix는 내부에서 cargo check를 실행하며, 기본적으로 모든 카고(Cargo) 타깃을 대상으로 삼습니다. 실행 결과는 소스 파일을
직접 바꾸므로 먼저 커밋하거나 작업 트리를 깨끗하게 만들어 두는 편이 좋습니다. 변경 중인 파일이 있으면 카고가 실행을 거부하므로 무심코 기존 작업과 자동 수정을 섞는
일을 막을 수 있습니다. 두 명령 모두 적용할 수 있다고 표시된 제안만 자동으로 반영합니다. 컴파일러가 프로그램의 요구 사항까지 아는 것은 아니므로 명령이 성공했다는
사실만으로 수정이 옳다고 판단하면 안 됩니다.
-->

---
level: 2
---

# `cargo fix`와 자동 수정 검토하기

```bash
cargo fix --all-features --locked
cargo clippy --fix --all-features --locked
cargo fix --target x86_64-pc-windows-gnu
```

- 현재 활성화된 피처와 플랫폼의 코드만 분석됩니다.
- `--all-features`에서 빠지는 `cfg(not(feature = "..."))` 분기도 확인합니다.
- 다른 플랫폼의 `#[cfg]` 코드는 해당 target과 도구가 준비되어야 합니다.
- “적용 가능”은 프로그램의 요구 사항까지 옳다는 뜻이 아닙니다.

<!--
교재 대응: book/src/cargo/10-cargo-fix.md > 검사되지 않는 코드 확인하기

카고는 현재 활성화된 피처(feature)와 컴파일 대상 플랫폼에 포함된 코드만 분석합니다. 다른 피처까지 고치려면 cargo fix와 cargo clippy
--fix에 --all-features를 붙여 검사 범위를 명시합니다. 다만 --all-features에서 빠지는 cfg(not(feature = "...")) 분기는
별도로 확인해야 합니다. 플랫폼에 따라 #[cfg]로 나뉜 코드는 --target으로 해당 컴파일 대상을 지정합니다. 모든 대상 플랫폼의 코드를 한 컴퓨터에서 곧바로
고칠 수 있는 것은 아니며, 필요한 표준 라이브러리와 링커가 준비되어 있는지도 확인해야 합니다. 카고 피처는 패키지의 기능을 선택해서 켜는 이름표입니다. 피처에 따라
특정 코드나 의존성이 컴파일에 포함되므로, 현재 켜지지 않은 피처의 코드는 자동 수정 대상에서도 빠집니다.
-->

---
level: 2
---

# 자동 수정 뒤에 할 일

<ol class="step-list">
  <li><code>git diff</code>로 바뀐 파일과 줄을 읽습니다.</li>
  <li>공개 API와 오류 처리 방식이 같은지 확인합니다.</li>
  <li>rustfmt와 클리피를 다시 실행합니다.</li>
  <li>관련 테스트와 피처 구성을 검사합니다.</li>
  <li>의도와 다른 수정은 직접 고칩니다.</li>
</ol>

<!--
교재 대응: book/src/cargo/10-cargo-fix.md > 자동 수정 뒤에 할 일

자동 수정은 다음 검토 과정을 줄여 주지 않습니다. git diff로 어떤 파일과 줄이 바뀌었는지 읽습니다. 공개 API와 오류 처리 방식이 달라지지 않았는지
확인합니다. cargo fmt --check와 클리피를 다시 실행합니다. 관련 테스트를 실행합니다. 의도와 다른 수정은 되돌리거나 직접 고칩니다. 자세한 동작과 제한은
카고의 cargo fix 문서와 클리피 사용법에서 확인할 수 있습니다.
-->
