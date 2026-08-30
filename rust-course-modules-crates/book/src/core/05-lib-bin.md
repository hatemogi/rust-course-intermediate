# 라이브러리와 실행 파일 나누기

같은 패키지<sub>package</sub>에 `src/lib.rs`와 `src/main.rs`가 있으면 서로 다른
크레이트<sub>crate</sub>입니다. 실행 파일은 패키지의
라이브러리<sub>library</sub>를 외부 크레이트처럼 이름으로 가져옵니다.

```rust
use sample_app::run;

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
```

입력 파싱과 업무 규칙은 라이브러리에 두고, `main`에는 명령줄 인자 수집, 출력,
종료 코드처럼 프로그램 경계의 일만 남기면 테스트하기 쉽습니다.

`examples/`의 각 파일도 별도 실행 파일 크레이트의 루트입니다. 강의 예제가
라이브러리의 공개 API만 사용하도록 만들면 외부 사용자가 실제로 호출할 수
있는지도 함께 확인할 수 있습니다.
