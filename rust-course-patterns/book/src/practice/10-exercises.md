# 종합 실습

간단한 작업 관리 프로그램에서 명령을 읽고 작업 상태를 설명해 봅시다. 이번
실습에서는 앞에서 배운 내용 가운데 다음 네 가지만 사용합니다.

- 슬라이스 패턴으로 명령의 이름과 인자 개수를 함께 확인합니다.
- 열거형과 구조체 변형을 구조 분해합니다.
- 리터럴·범위·`@` 패턴으로 진행률을 구분합니다.
- 구체적인 패턴을 넓은 패턴보다 먼저 둡니다.

구현할 함수는 `parse_command`와 `describe_event` 두 개입니다.

## 실습 시작하기

`src/lib.rs`에는 테스트를 통과하는 참고 구현이 들어 있습니다. 그대로 테스트하면
처음부터 모두 통과하므로, 먼저 두 함수의 본문을 다음처럼 `todo!()`로 바꿉니다.

```rust
pub fn parse_command<'a>(args: &[&'a str]) -> Option<Command<'a>> {
    todo!()
}

pub fn describe_event(event: &Event) -> String {
    todo!()
}
```

매개변수 이름을 사용하지 않는다는 경고는 구현을 시작하면 사라집니다. 이 상태에서
`cargo test`를 실행해 테스트가 실패하는지 먼저 확인하세요. 그다음 함수 이름,
매개변수 타입, 반환 타입은 유지하고 본문만 직접 작성합니다.

먼저 `parse_command`를 완성하고 다음 명령으로 확인합니다.

```bash
cargo test parses_known_commands
```

그다음 `describe_event`를 완성하고 전체 테스트를 실행합니다.

```bash
cargo test
```

## 명령 해석하기

`parse_command`는 `&[&str]`을 받아 다음 규칙에 맞는 명령을 반환합니다.

| 입력 | 반환값 |
|---|---|
| `["start", job]` | `Some(Command::Start { job })` |
| `["stop"]` | `Some(Command::Stop)` |
| `["status"]` | `Some(Command::Status)` |
| 그 밖의 입력 | `None` |

슬라이스 패턴을 사용하면 명령 이름과 전체 인자 개수를 한 번에 검사할 수 있습니다.
예를 들어 `["start", job]`은 원소가 정확히 두 개이고, 첫 원소가 `"start"`인
경우에만 매칭됩니다. `job`은 입력 문자열을 빌리므로 새 `String`을 만들 필요가
없습니다.

```rust
{{#rustdoc ../../../examples/10_exercises.rs body=command}}
```

## 작업 상태 설명하기

`describe_event`는 `&Event`를 받아 다음 문자열을 만듭니다.

| 이벤트 | 반환할 문자열 |
|---|---|
| `Started { job }` | `"{job} 작업 시작"` |
| `Progress { job, percent: 100 }` | `"{job} 작업 완료"` |
| 진행률이 `0..=99` | `"{job} 작업 {percent}%"` |
| 진행률이 `101..=255` | `"{job} 작업의 잘못된 진행률: {percent}"` |
| `Failed { job }` | `"{job} 작업 실패"` |

완료를 나타내는 `100`은 일반 진행률보다 구체적이므로 먼저 처리합니다. 일반
진행률에서는 `percent @ 0..=99`로 범위를 검사하면서 같은 값에 이름도 붙일 수
있습니다. 그 뒤의 `Progress` 갈래는 남은 범위를 처리합니다.

함수는 이벤트를 참조로 받습니다. 패턴의 `job`도 `String`을 이동하지 않고
참조로 바인딩됩니다.

```rust
{{#rustdoc ../../../examples/10_exercises.rs body=event}}
```

## 마무리 확인

두 함수가 완성되면 다음 명령을 실행합니다.

```bash
cargo test
```

테스트가 실패하면 먼저 어떤 입력이 어느 패턴과 매칭되는지 위에서 아래로
확인하세요. 특히 `Progress` 갈래의 순서를 바꾸면 `100`을 완료로 처리할 수 있는지
살펴보세요.
