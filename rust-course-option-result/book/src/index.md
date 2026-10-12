# 우아한 Rust 중급: Option과 Result

<p class="course-cover">
  <img src="images/cover-ko.png" alt="우아한 Rust 중급: Option과 Result 한국어 커버">
</p>

Rust에서는 값이 없을 수 있을 때 `Option<T>`를, 작업이 실패할 수 있을 때
`Result<T, E>`를 사용합니다. 함수를 호출하는 쪽에서는 반환된 값을 보고
값이 없는 경우와 작업이 실패한 경우를 구분해 처리할 수 있습니다.

이 강의에서는 먼저 `Option`을 다루는 법을 익힌 뒤 `Result`로 실패 이유를
표현하고 오류 타입을 만드는 법을 배웁니다. 마지막에는 `ok_or`, `.ok()`, `transpose`, `collect`, `?`로
두 타입을 함께 사용하는 방법을 살펴봅니다.

교재에 나오는 모든 코드는 `examples` 디렉터리에 있으며, 직접 실행해 볼 수 있습니다.

```bash
cargo run --example 01_mental_model
cargo run --example 15_transpose
cargo test
```

## 공개 저장소

중급 강의 자료는 공개 GitHub 저장소에 강의별 디렉터리로 나누어 두었습니다.

- 전체 저장소: <https://github.com/hatemogi/rust-course-intermediate>
- 이 강의 디렉터리: <https://github.com/hatemogi/rust-course-intermediate/tree/main/rust-course-option-result>

이 강의는 “우아한 프로그래밍 언어 Rust 입문” 강의를 들은 분을 대상으로 합니다.
