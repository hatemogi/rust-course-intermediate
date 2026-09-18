# 우아한 Rust 중급

Rust의 타입과 표준 라이브러리를 실제 프로그램에 적용하는 방법을 배우는 한국어
강의입니다. 영상이 공개된 강의의 교재, 슬라이드, 예제와 실습 코드만 이 저장소에
차례로 공개합니다.

## 공개된 강의

### 1. 개발 도구

Rust 도구 체인과 Cargo 프로젝트를 준비하고, 포맷, lint, 테스트, 문서와
벤치마크를 반복 가능한 검증 절차로 연결합니다.

- [교재 읽기](https://hatemogi.github.io/rust-course-intermediate/tooling/)
- [슬라이드 보기](https://hatemogi.github.io/rust-course-intermediate/tooling/slides/)
- [예제와 실습 코드](./rust-course-tooling)

각 장의 예제와 종합 실습을 직접 실행하려면 `rust-course-tooling` 디렉터리의
README를 참고하세요.

### 2. 모듈과 크레이트

package, crate, module을 구분하고 코드를 여러 파일로 나누며, 공개 범위와 재공개를
이용해 크레이트의 공개 API를 구성합니다.

- [교재 읽기](https://hatemogi.github.io/rust-course-intermediate/modules-crates/)
- [예제와 실습 코드](./rust-course-modules-crates)

각 장의 예제와 종합 실습을 직접 실행하려면 `rust-course-modules-crates` 디렉터리의
README를 참고하세요.

### 3. 문자열 활용

`String`, `&str`과 슬라이스의 관계를 이해하고 UTF-8 문자열을 만들고 수정하며,
검색·분리·변환하는 방법을 배웁니다.

- [교재 읽기](https://hatemogi.github.io/rust-course-intermediate/strings/)
- [예제와 실습 코드](./rust-course-strings)

각 장의 예제와 종합 실습을 직접 실행하려면 `rust-course-strings` 디렉터리의
README를 참고하세요.

### 4. 패턴 문법

값의 구조를 확인하고 필요한 부분에 이름을 붙이는 Rust 패턴 문법을 배웁니다.
`match`, 구조 분해, 매치 가드와 참조 패턴을 예제와 실습으로 익힙니다.

- [교재 읽기](https://hatemogi.github.io/rust-course-intermediate/patterns/)
- [예제와 실습 코드](./rust-course-patterns)

각 장의 예제와 종합 실습을 직접 실행하려면 `rust-course-patterns` 디렉터리의
README를 참고하세요.
