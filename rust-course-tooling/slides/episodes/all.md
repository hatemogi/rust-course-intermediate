---
theme: default
title: 우아한 Rust 중급 개발 도구 전체 과정
info: 우아한 Rust 중급 개발 도구 11편 통합 강의
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

11개 챕터를 하나의 검증 흐름으로 이어서 살펴봅니다.

---
title: 전체 과정
hideInToc: true
class: course-toc
---

# 전체 과정

<Toc maxDepth="1" columns="2" />

<div class="takeaway">챕터 제목을 누르면 해당 편의 표지로 바로 이동합니다.</div>

---
src: ./01-toolchain.md#1
level: 1
routeAlias: toolchain
---

---
src: ./01-toolchain.md#3-12
level: 2
---

---
src: ./02-cargo-check.md#1
level: 1
routeAlias: cargo-check
---

---
src: ./02-cargo-check.md#2-10
level: 2
---

---
src: ./03-rustfmt.md#1
level: 1
routeAlias: rustfmt
---

---
src: ./03-rustfmt.md#2-9
level: 2
---

---
src: ./04-clippy.md#1
level: 1
routeAlias: clippy
---

---
src: ./04-clippy.md#2-11
level: 2
---

---
src: ./05-editor-fix.md#1
level: 1
routeAlias: editor-fix
---

---
src: ./05-editor-fix.md#2-11
level: 2
---

---
src: ./06-dependencies.md#1
level: 1
routeAlias: dependencies
---

---
src: ./06-dependencies.md#2-10
level: 2
---

---
src: ./07-features-docs.md#1
level: 1
routeAlias: features-docs
---

---
src: ./07-features-docs.md#2-11
level: 2
---

---
src: ./08-tests.md#1
level: 1
routeAlias: tests
---

---
src: ./08-tests.md#2-11
level: 2
---

---
src: ./09-error-doc-tests.md#1
level: 1
routeAlias: error-doc-tests
---

---
src: ./09-error-doc-tests.md#2-11
level: 2
---

---
src: ./10-benchmark.md#1
level: 1
routeAlias: benchmark
---

---
src: ./10-benchmark.md#2-10
level: 2
---

---
src: ./11-workflow.md#1
level: 1
routeAlias: workflow
---

---
src: ./11-workflow.md#2-14
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
