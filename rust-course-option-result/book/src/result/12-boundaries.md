# 프로그램 경계에서 오류 처리하기

라이브러리에서는 호출하는 쪽에서 오류의 종류를 구분할 수 있도록 오류를 반환하세요.
로그를 남길지, 작업을 다시 시도할지, 사용자에게 어떤 메시지를 보일지, 어떤 종료
코드로 끝낼지는 `main`처럼 프로그램 전체의 실행을 맡는 곳에서 결정합니다.

```rust
{{#rustdoc ../../../examples/12_boundaries.rs item=use:std::{num::ParseIntError,process::ExitCode}..run}}
```

프로그램이 계속 실행될 수 없는 심각한 논리 오류에는 패닉<sub>panic</sub>을, 파일이 없거나
입력값이 잘못된 것처럼 예상할 수 있는 실패에는 `Result`를 사용합니다.

## 실제 종료 코드로 연결하기

```rust
{{#rustdoc_include ../../../examples/12_boundaries.rs:20:23}}
```

`main`이 `ExitCode`를 반환하면 프로그램을 실행한 셸<sub>shell</sub>에서도
성공 여부를 알 수 있습니다. 다음 두 명령의 종료 코드는 각각 0과 2입니다. 포트 설정을 읽는 예제이며 서버를 실행하지는
않습니다.

```bash
cargo run --example 12_boundaries -- 8080
cargo run --example 12_boundaries -- rust
```
