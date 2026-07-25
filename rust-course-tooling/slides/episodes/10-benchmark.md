---
theme: default
title: 벤치마크와 측정 결과 읽기
info: 우아한 Rust 중급 개발 도구 10편
colorSchema: dark
aspectRatio: 16/9
canvasWidth: 1100
transition: fade-out
lineNumbers: true
layout: cover
class: cover
---

<div class="episode-label">우아한 Rust 중급 · 개발 도구 10</div>

# 벤치마크와<br>측정 결과 읽기

반복 측정의 조건을 맞추고 숫자가 말하지 않는 전제를 확인합니다.

---
level: 2
---

# 성능보다 올바른 결과가 먼저입니다

서로 다른 답을 내는 두 구현의 시간은 공정하게 비교할 수 없습니다.

```rust
assert_eq!(
    linear_contains(&values, target),
    binary_contains(&values, target)
);
```

<div class="takeaway">테스트로 같은 요구 사항을 만족한다고 확인한 구현끼리 측정합니다.</div>

---
level: 2
---

# 공정한 측정에는 조건이 필요합니다

<ul class="step-list">
  <li><code>--release</code> 최적화가 적용된 코드끼리 비교합니다.</li>
  <li>같은 입력과 같은 결과를 사용합니다.</li>
  <li>준비 작업과 측정할 작업을 구분합니다.</li>
  <li>짧게 준비 실행한 뒤 측정합니다.</li>
  <li>한 번이 아니라 여러 번 반복합니다.</li>
  <li>입력과 결과에 <code>black_box</code>를 검토합니다.</li>
  <li>실행 순서가 결과에 미치는 영향을 확인합니다.</li>
</ul>

---
level: 2
---

# 한 번 재는 코드는 측정 구간을 배우는 출발점입니다

```rust {1-3|5-7}{lines:true}
let started = Instant::now();
let linear_result = linear_contains(black_box(&values), target);
let linear_elapsed = started.elapsed();

let started = Instant::now();
let binary_result = binary_contains(black_box(&values), target);
let binary_elapsed = started.elapsed();
```

```bash
cargo run --release --example 03_measure
```

준비한 `values`와 측정할 검색 호출을 분리하고 두 구현의 결과가 같은지 확인합니다.

---
level: 2
---

# 한 번의 숫자는 실행 환경에 쉽게 흔들립니다

<div class="benchmark-scatter">
  <div class="scatter-heading">
    <strong>같은 프로그램을 12회 실행</strong>
    <span>각 행은 서로 다른 눈금입니다.</span>
  </div>

  <div class="scatter-row">
    <div class="scatter-label"><strong>선형 검색</strong><code>7.29–8.63 µs</code></div>
    <div class="scatter-plot">
      <i style="--x: 48%; --y: 25%"></i><i style="--x: 53%; --y: 65%"></i>
      <i style="--x: 95%; --y: 35%"></i><i style="--x: 40%; --y: 72%"></i>
      <i style="--x: 26%; --y: 38%"></i><i style="--x: 28%; --y: 70%"></i>
      <i style="--x: 40%; --y: 30%"></i><i style="--x: 34%; --y: 58%"></i>
      <i style="--x: 31%; --y: 22%"></i><i style="--x: 34%; --y: 78%"></i>
      <i style="--x: 12%; --y: 48%"></i><i style="--x: 6%; --y: 66%"></i>
      <span class="axis-min">7.2</span><span class="axis-mid">8.0</span><span class="axis-max">8.7 µs</span>
    </div>
  </div>

  <div class="scatter-row">
    <div class="scatter-label"><strong>이진 검색</strong><code>0–84 ns</code></div>
    <div class="scatter-plot binary-points">
      <i style="--x: 46%; --y: 25%"></i><i style="--x: 47%; --y: 65%"></i>
      <i style="--x: 47%; --y: 35%"></i><i style="--x: 47%; --y: 75%"></i>
      <i style="--x: 47%; --y: 48%"></i><i style="--x: 93%; --y: 25%"></i>
      <i style="--x: 92%; --y: 65%"></i><i style="--x: 92%; --y: 38%"></i>
      <i style="--x: 0%; --y: 55%"></i><i style="--x: 46%; --y: 78%"></i>
      <i style="--x: 46%; --y: 42%"></i><i style="--x: 47%; --y: 18%"></i>
      <span class="axis-min">0</span><span class="axis-mid">45</span><span class="axis-max">90 ns</span>
    </div>
  </div>
</div>

<div class="visual-caption">이 환경에서 실행한 예입니다. 한 점만으로는 실행 순간의 흔들림을 알 수 없습니다.</div>

---
level: 2
---

# 벤치마크 타깃은 최적화해서 실행합니다

```toml
[[bench]]
name = "search"
harness = false
```

```bash
cargo bench --bench search --locked
cargo bench --no-run --locked
```

`--no-run`은 평소 검증에서 벤치마크 코드가 낡지 않았는지만 컴파일로 확인할 때
사용할 수 있습니다.

---
level: 2
---

# 측정기는 준비 실행과 반복 횟수를 드러냅니다

<<< ../../benches/search.rs#measure rust {1-5|7-18}{lines:true}

<div class="visual-caption">회당 시간도 반복문과 <code>black_box</code>의 영향을 포함한 학습용 근삿값입니다.</div>

---
level: 2
---

# “이진 검색이 빠르다”에는 전제가 있습니다

<div class="search-compare-visual">
  <section>
    <div class="search-label"><strong>선형 검색</strong><span>정렬되지 않아도 됩니다.</span></div>
    <div class="search-array linear-array" aria-label="선형 검색은 목표 값 55를 찾을 때 앞에서부터 여섯 칸을 확인합니다">
      <i>34</i><i>5</i><i>21</i><i>2</i><i>13</i><i class="found">55</i><i>8</i>
    </div>
    <div class="search-steps">앞에서부터 <strong>6칸</strong> 확인</div>
  </section>

  <section>
    <div class="search-label"><strong>이진 검색</strong><span>정렬된 입력이 필요합니다.</span></div>
    <div class="search-array binary-array" aria-label="이진 검색은 정렬된 배열에서 13, 34, 55를 차례로 확인합니다">
      <i>2</i><i>5</i><i>8</i><i data-step="1">13</i><i>21</i><i data-step="2">34</i><i class="found" data-step="3">55</i>
    </div>
    <div class="search-steps">범위를 절반씩 줄여 <strong>3칸</strong> 확인</div>
  </section>
</div>

<div class="sort-cost"><span>정렬되지 않은 입력</span><b>→</b><span>정렬 비용</span><b>+</b><span>이진 검색</span></div>

<div class="visual-caption">검색 전에 매번 정렬한다면 정렬 비용까지 측정에 포함합니다.</div>

---
level: 2
---

# 결과를 읽을 때 숫자 옆의 조건을 적습니다

<ol class="step-list">
  <li>입력 크기와 분포는 무엇인가?</li>
  <li>준비 비용을 포함했는가?</li>
  <li>두 구현의 결과가 같은가?</li>
  <li>반복 결과가 얼마나 흔들리는가?</li>
  <li>항상 같은 구현을 먼저 실행하지 않았는가?</li>
  <li>실제 사용 환경에서도 같은 전제가 성립하는가?</li>
</ol>

---
level: 2
layout: center
class: section
---

# 빠른 구현이 아니라<br>조건에 맞는 구현을 선택합니다

마지막 편에서는 환경, 검사, 테스트, 문서와 측정을 하나의 절차로 연결합니다.
