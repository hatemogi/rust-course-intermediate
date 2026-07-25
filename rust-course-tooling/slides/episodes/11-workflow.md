---
theme: default
title: 전체 검증 흐름과 종합 실습
info: 우아한 Rust 중급 개발 도구 11편
colorSchema: dark
aspectRatio: 16/9
canvasWidth: 1100
transition: fade-out
lineNumbers: true
layout: cover
class: cover
---

<div class="episode-label">우아한 Rust 중급 · 개발 도구 11</div>

# 전체 검증 흐름과<br>종합 실습

도구를 순서대로 연결하고 선택의 근거를 기록합니다.

---
level: 2
---

# 도구마다 쌓는 증거가 다릅니다

<div class="tool-flow">
  <div><strong>정적 확인</strong>형식, 타입, 소유권과 lint</div>
  <div><strong>동작 확인</strong>단위·통합·문서 테스트</div>
  <div><strong>결과 판단</strong>API 문서와 성능 측정</div>
</div>

<div class="takeaway">앞 단계가 빠르게 실패할수록 뒤의 비싼 검사를 실행하기 전에 문제를 좁힐 수 있습니다.</div>

---
level: 2
---

# 전체 흐름은 빠른 검사부터 쌓습니다

<ol class="step-list">
  <li>선택된 도구 체인을 확인합니다.</li>
  <li>형식과 Clippy를 검사합니다.</li>
  <li>지원하는 feature 구성을 컴파일합니다.</li>
  <li>단위·통합·문서 테스트를 실행합니다.</li>
  <li>API 문서와 벤치마크 타깃을 생성합니다.</li>
  <li>성능 판단이 필요할 때만 실제로 측정합니다.</li>
</ol>

---
level: 2
---

# 저장소의 `make check`가 순서를 고정합니다

<<< ../../Makefile makefile {maxHeight:'440px', lines:true}

도구 체인 설치와 갱신은 사용자 환경을 바꾸므로 자동 검사에 넣지 않고 프로젝트를
시작할 때 별도로 확인합니다.

---
level: 2
---

# 종합 실습은 이름 정돈 코드에서 시작합니다

```rust {1-8|10-12}{lines:true}
fn normalized_name(name: &str) -> Option<String> {
    let name = name.trim();
    if name.is_empty() {
        None
    } else {
        Some(name.to_lowercase())
    }
}

pub fn normalize_names(names: &[&str]) -> Vec<String> {
    names.iter().filter_map(|name| normalized_name(name)).collect()
}
```

확인할 요구 사항:

- 앞뒤 공백을 없앱니다.
- 빈 이름을 제외합니다.
- 남은 이름을 소문자로 바꿉니다.

---
level: 2
---

# 경계 조건을 공개 동작으로 검사합니다

```rust {1-7}{lines:true}
#[test]
fn removes_names_that_are_empty_after_trimming() {
    assert_eq!(normalize_names(&["Ferris", "", "  "]), ["ferris"]);
}
```

<div class="command">cargo test --test exercises</div>

테스트 이름과 입력만 보고 어떤 요구 사항을 확인하는지 설명해 봅니다.

---
level: 2
---

# 지원하는 세 feature 구성을 실행합니다

```bash
cargo run --example 07_features
cargo run --example 07_features --features binary-search

cargo check --locked
cargo check --no-default-features --locked
cargo check --all-features --locked
```

<div class="takeaway"><code>--all-features</code>에서는 feature를 껐을 때만 들어오는 코드가 빠질 수 있습니다.</div>

---
level: 2
---

# 문서의 설명과 예제도 검증 대상입니다

```bash
cargo test --doc --no-default-features --locked
cargo test --doc --all-features --locked
cargo doc --no-deps --no-default-features --locked
cargo doc --no-deps --all-features --locked
```

생성된 문서에서 `normalize_names`를 찾아 다음을 확인합니다.

- 설명만 읽고 입력과 결과를 예상할 수 있는가?
- 대표 예제가 실제 사용 순서를 보여 주는가?
- feature별 공개 API가 예상대로 나타나는가?

---
level: 2
---

# 성능 측정 전에 구현의 결과를 맞춥니다

```rust {1-9}{lines:true}
#[test]
fn search_implementations_match_expected_results() {
    let values = [1, 4, 8, 15, 16, 23, 42];
    for (target, expected) in [(1, true), (15, true), (42, true), (100, false)] {
        assert_eq!(linear_contains(&values, target), expected);
        assert_eq!(binary_contains(&values, target), expected);
    }
}
```

```bash
cargo bench --bench search --locked
```

입력 크기를 바꿔 비교하고, 이진 검색을 위해 정렬 비용이 추가되는 상황을 함께
기록합니다.

---
level: 2
---

# AI가 줄인 코드도 같은 기준으로 검토합니다

- 빈 이름을 제외하는 동작이 유지되는가?
- 소유한 `String`을 불필요하게 여러 번 만드는가?
- Clippy 경고를 `allow`로 덮었는가?
- 자동 수정 뒤의 diff를 확인했는가?
- 기본·전체 feature에서 요구 사항을 만족하는가?
- 기존 테스트가 변경된 동작을 충분히 확인하는가?
- 두 구현이 같은 오답을 내도 통과하는 비교만 남아 있지 않은가?

---
level: 2
---

# 판단 결과에는 채택하지 않은 선택도 남깁니다

```text
- 변경하려던 동작:
- 실행한 검사와 테스트:
- 도구가 제안한 수정:
- 채택한 수정과 근거:
- 채택하지 않은 수정과 이유:
- 성능 측정 조건과 결과:
- 아직 확인하지 못한 범위:
```

<div class="takeaway">통과 여부만 적는 것보다 어떤 범위에서 무엇을 확인했는지 적는 편이 다시 판단하기 쉽습니다.</div>

---
level: 2
---

# 마지막에는 한 명령으로 반복합니다

<div class="grid grid-cols-2 gap-6">
  <div class="command">make check</div>
  <div class="command">make msrv</div>
</div>

두 명령은 현재 도구 체인의 전체 흐름과 패키지가 지원하는 가장 오래된 Rust를
나누어 확인합니다.

- 형식과 Clippy
- feature별 컴파일과 테스트
- 문서 테스트와 API 문서 생성
- 벤치마크 컴파일
- mdBook 빌드

---
level: 2
---

# 도구 하나가 다른 도구를 대신하지 않습니다

<table class="compare">
  <tbody>
    <tr><td>rustfmt를 통과함</td><td>잘못된 값을 계산할 수 있음</td></tr>
    <tr><td>Clippy 경고가 없음</td><td>경계 조건에서 실패할 수 있음</td></tr>
    <tr><td>테스트를 통과함</td><td>측정하지 않은 성능은 알 수 없음</td></tr>
    <tr><td>빠르게 실행됨</td><td>요구 사항을 어길 수 있음</td></tr>
  </tbody>
</table>

---
level: 2
layout: center
class: section
---

# 도구의 출력을 따르지 말고<br>근거를 연결해 결정하세요

환경, diff, 테스트와 측정 결과가 함께 있을 때 코드를 믿을 수 있는 범위를 설명할 수
있습니다.
