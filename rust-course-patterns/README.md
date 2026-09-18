# 우아한 Rust 중급: 패턴 문법

값의 구조를 확인하고 필요한 부분에 이름을 붙이는 Rust 패턴 문법을 체계적으로
배웁니다. `match`뿐 아니라 `let`, 함수 매개변수, `if let`, `let else`,
`while let`에서 패턴이 어떻게 쓰이는지 살펴봅니다.

## 이 강의의 핵심

- 항상 매칭<sub>matching</sub>되는 패턴과 매칭되지 않을 수 있는 패턴을 구분합니다.
- 튜플, 구조체, 열거형, 슬라이스를 구조 분해<sub>destructuring</sub>합니다.
- OR 패턴, 범위, 매치 가드<sub>match guard</sub>, `@` 바인딩으로 조건을 정확히
  표현합니다.
- 참조를 분해할 때 값이 이동되는지 빌려지는지 판단합니다.

[교재 시작하기](book/src/index.md) · [전체 목차](book/src/SUMMARY.md)

## 실행과 실습

`src/lib.rs`는 종합 실습의 참고 구현입니다. 교재의 요구 사항을 보고 명령 해석과
작업 상태 설명 함수의 본문을 직접 다시 작성한 뒤 참고 구현과 비교합니다.

- `make check`는 형식, 클리피<sub>Clippy</sub>, 모든 Cargo 타깃, 테스트, 예제 실행과
  교재 빌드를 검사합니다.
- `make check-examples`는 각 예제의 `main`을 실행해 단언문을 검사합니다.
- `make mdbook`은 교재를 로컬 웹 서버로 엽니다.
