# `cargo check`와 Cargo 타깃

`cargo check`는 Rust 코드를 컴파일할 수 있는지 검사하지만 최종 실행 파일은 만들지
않습니다. 코드 작성 중에 타입 오류와 소유권 오류를 빠르게 확인할 때 사용합니다.

```bash
cargo check
```

실행 파일이 필요하면 `cargo build`, 프로그램을 바로 실행하려면 `cargo run`을
사용합니다. 세 명령 모두 컴파일 과정을 거치지만 목적이 다릅니다.

| 명령 | 주된 목적 | 실행 파일 |
| --- | --- | --- |
| `cargo check` | 빠른 컴파일 가능 여부 확인 | 만들지 않음 |
| `cargo build` | 실행하거나 배포할 파일 만들기 | 만듦 |
| `cargo run` | 실행 파일을 만든 뒤 바로 실행하기 | 만들고 실행함 |

## Cargo 타깃

한 패키지에는 여러 종류의 컴파일 대상이 들어갈 수 있습니다. Cargo에서는 이를
타깃이라고 부릅니다.

- `src/lib.rs`: 라이브러리 타깃
- `src/main.rs`, `src/bin/*.rs`: 실행 파일 타깃
- `examples/*.rs`: 예제 타깃
- `tests/*.rs`: 통합 테스트 타깃
- `benches/*.rs`: 벤치마크 타깃

기본 `cargo check`가 모든 타깃을 검사한다고 가정하면 안 됩니다. 강의 저장소에서는
예제와 테스트도 항상 컴파일되는지 확인하기 위해 다음 명령을 사용합니다.

```bash
cargo check --all-targets --locked
```

필요한 대상만 좁혀서 검사할 수도 있습니다.

```bash
cargo check --lib
cargo check --examples
cargo check --tests
cargo check --benches
```

여러 패키지를 묶은 Cargo workspace에서는 `--workspace`로 모든 구성원을 검사합니다.

```bash
cargo check --workspace --all-targets --locked
```

`--locked`는 `Cargo.lock`과 다른 의존성 해석이 필요하면 명령을 실패시킵니다. 잠금
파일을 모르는 사이에 바꾸지 않고 같은 의존성으로 검증할 때 사용합니다.

## 판단하기

- 예제 코드를 교재에 넣었지만 `cargo check --all-targets`를 실행하지 않았다면 어떤
  문제가 남을 수 있을까요?
- 컴파일 검사에 통과한 코드가 테스트까지 통과한다고 말할 수 있나요?
