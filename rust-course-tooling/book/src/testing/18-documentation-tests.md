# 문서 주석과 문서 테스트

Rust의 공개 API에는 `///` 문서 주석을 붙일 수 있습니다. 모듈이나 crate 전체를
설명할 때는 `//!`를 사용합니다. 문서의 Rust 코드 블록은 `cargo test --doc`가
컴파일하고 실행하므로 사용법이 실제 코드와 달라지는 일을 줄일 수 있습니다.

## 실행되는 사용 예제

`normalize_names`의 문서에는 다음과 같은 사용 예제가 들어 있습니다.

```rust
use rust_course_tooling::normalize_names;

let names = normalize_names(&[" Ferris ", " RUST "]);
assert_eq!(names, ["ferris", "rust"]);
```

문서 테스트에서는 독자에게 중요하지 않은 준비 코드를 `#`로 시작해 문서에서 숨길
수 있습니다. 코드는 보이지 않아도 테스트할 때 함께 컴파일됩니다.

## 코드 블록 속성

- 아무 속성이 없는 Rust 블록은 컴파일하고 실행합니다.
- `no_run`은 컴파일하지만 실행하지 않습니다.
- `compile_fail`은 컴파일이 실패해야 테스트가 통과합니다.
- `ignore`는 문서 테스트에서 제외합니다.

```rust,no_run
std::fs::write("report.txt", "완료")?;
# Ok::<(), std::io::Error>(())
```

파일을 만드는 예제처럼 문법과 타입은 검사하되 문서 테스트 중 실행하고 싶지 않은
코드에는 `no_run`을 사용할 수 있습니다.

```rust,compile_fail
let message = String::from("완료");
drop(message);
println!("{message}");
```

`compile_fail`은 소유권이나 타입 제약처럼 “이 사용법은 허용되지 않는다”는 계약을
검사할 때 유용합니다. 단지 현재 우연히 컴파일되지 않는 내부 구현을 고정하는 데
사용하지 마세요.

## 실행하기

라이브러리의 문서 테스트만 실행하려면 다음 명령을 사용합니다.

```bash
cargo test --doc --locked
```

feature에 따라 문서와 공개 API가 달라진다면 지원하는 구성을 각각 검사합니다.

```bash
cargo test --doc --no-default-features --locked
cargo test --doc --all-features --locked
```

문서 테스트가 일반 단위 테스트를 대신하지는 않습니다. 문서에는 대표적인 공개
사용법을 두고, 경계 조건과 오류 조합은 단위·통합 테스트에서 자세히 검사하세요.

## 문서 만들기

다음 명령은 현재 패키지와 의존성의 API 문서를 만들고 브라우저로 엽니다.

```bash
cargo doc --open
```

CI처럼 브라우저를 열지 않는 환경에서는 `cargo doc --no-deps`로 현재 패키지의 문서가
만들어지는지만 확인할 수 있습니다.
