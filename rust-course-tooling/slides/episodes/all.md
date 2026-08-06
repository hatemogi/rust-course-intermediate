---
theme: default
title: 우아한 Rust 중급 개발 도구 전체 과정
info: 우아한 Rust 중급 개발 도구 10편 통합 강의
colorSchema: dark
aspectRatio: 16/9
canvasWidth: 1100
transition: fade-out
lineNumbers: true
layout: cover
class: cover
hideInToc: true
---

<div class="episode-label">우아한 Rust 중급 · 개발 도구</div>

# 개발 도구<br>전체 과정

Rust 개발에서 일상적으로 사용하는 표준 도구들을 더 정확하고 능숙하게

<!--
교재 대응: book/src/index.md > 우아한 Rust 중급: 개발 도구

이 과정은 `rustup`, `cargo check`, rustfmt, `cargo clippy`, `cargo update`, `cargo doc`,
`cargo test`를 따로 외우는 대신 하나의 개발·검증 흐름으로 연결합니다. 각 도구가
확인하는 범위를 구분하고, 제안을 적용할지는 코드의 목적과 테스트 결과를 근거로
판단하는 것이 전체 과정의 목표입니다.
-->

---
title: 전체 과정
hideInToc: true
class: course-toc
---

# 전체 과정

<Toc maxDepth="1" columns="2" />

<div class="takeaway">챕터 제목을 누르면 해당 편의 표지로 바로 이동합니다.</div>

<!--
교재 대응: book/src/index.md > 학습 목표; book/src/SUMMARY.md > 목차

툴체인과 프로젝트 환경을 맞춘 뒤 컴파일·형식·린트를 검사하고, 의존성과 피처를
추적하며, 단위·통합·문서 테스트와 API 문서로 검증 범위를 넓힙니다. 마지막에는 이
도구들을 자동 검증 명령으로 묶고 종합 실습에 적용합니다. 목차의 각 챕터는 이
학습 순서를 영상 한 편의 목표 단위로 다시 묶은 것입니다.
-->

---
src: ./01-toolchain.md#1
level: 1
routeAlias: toolchain
---

---
src: ./01-toolchain.md#3-18
level: 2
---

---
src: ./02-cargo-check.md#1
level: 1
routeAlias: cargo-check
---

---
src: ./02-cargo-check.md#2-9
level: 2
---

---
src: ./03-rustfmt.md#1
level: 1
routeAlias: rustfmt
---

---
src: ./03-rustfmt.md#2-7
level: 2
---

---
src: ./04-clippy.md#1
level: 1
routeAlias: clippy
---

---
src: ./04-clippy.md#2-9
level: 2
---

---
src: ./05-editor-fix.md#1
level: 1
routeAlias: editor-fix
---

---
src: ./05-editor-fix.md#2-9
level: 2
---

---
src: ./06-dependencies.md#1
level: 1
routeAlias: dependencies
---

---
src: ./06-dependencies.md#2-9
level: 2
---

---
src: ./07-features-docs.md#1
level: 1
routeAlias: features-docs
---

---
src: ./07-features-docs.md#2-10
level: 2
---

---
src: ./08-tests.md#1
level: 1
routeAlias: tests
---

---
src: ./08-tests.md#2-10
level: 2
---

---
src: ./09-error-doc-tests.md#1
level: 1
routeAlias: error-doc-tests
---

---
src: ./09-error-doc-tests.md#2-10
level: 2
---

---
src: ./10-workflow.md#1
level: 1
routeAlias: workflow
---

---
src: ./10-workflow.md#2-27
level: 2
---

<style>
.course-toc .slidev-toc-list-level-1 {
  counter-reset: chapter;
  margin-top: 1.5rem;
}

.course-toc .slidev-toc-list-level-1 > .slidev-toc-item {
  counter-increment: chapter;
  break-inside: avoid;
  margin-bottom: 0.45rem;
}

.course-toc .slidev-toc-list-level-1 > .slidev-toc-item > a p::before {
  color: #f28b4b;
  content: counter(chapter, decimal-leading-zero) ". ";
  font-family:
    "SFMono-Regular",
    "Cascadia Code",
    Menlo,
    monospace;
  font-weight: 800;
}
</style>
