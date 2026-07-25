# Clippy 실행하고 결과 읽기

Clippy는 컴파일되는 Rust 코드에서 실수일 가능성이 있는 표현, 불필요하게 복잡한
코드와 더 자연스러운 Rust 표현을 찾는 공식 lint 도구입니다.

```bash
cargo clippy
```

라이브러리뿐 아니라 실행 파일, 예제, 테스트와 벤치마크까지 검사하려면 다음 명령을
사용합니다.

```bash
cargo clippy --all-targets --locked -- -D warnings
```

- `--all-targets`는 Cargo의 모든 타깃을 검사합니다.
- `--locked`는 `Cargo.lock`을 임의로 바꾸지 못하게 합니다.
- `-- -D warnings`는 경고를 검사 실패로 취급합니다. `--` 뒤의 옵션은 rustc와
  Clippy에 전달됩니다.

다음 예제의 `Iterator::any`는 조건에 맞는 값이 있는지를 그대로 표현합니다.

```rust
{{#include ../../../examples/02_clippy.rs}}
```

lint가 나오면 이름과 설명, 지적한 코드 범위, 제안한 수정 내용을 차례로 읽으세요.
lint 이름으로 Clippy 설명을 찾아보면 잘못 잡아낸 경우와 적용 시 주의점도 확인할 수
있습니다.
