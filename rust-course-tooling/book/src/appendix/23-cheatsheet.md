# 개발 도구 명령 한눈에 보기

| 목적 | 명령 |
| --- | --- |
| rustup과 활성 도구 체인 확인 | `rustup show` |
| 설치한 도구 체인 확인 | `rustup toolchain list` |
| 설치한 구성 요소 확인 | `rustup component list --installed` |
| stable에 rustfmt와 Clippy 추가 | `rustup component add --toolchain stable rustfmt clippy` |
| 한 명령에 stable 지정 | `cargo +stable check` |
| 패키지가 지원하는 가장 오래된 Rust 선언 | `Cargo.toml`의 `rust-version` |
| 코드 형식 변경 | `cargo fmt` |
| 형식만 검사 | `cargo fmt --check` |
| 컴파일러 제안 자동 적용 | `cargo fix --locked` |
| Clippy 제안 자동 적용 | `cargo clippy --fix --locked` |
| 의존성 추가 | `cargo add 패키지명` |
| 의존성 제거 | `cargo remove 패키지명` |
| 지정한 의존성 갱신 | `cargo update -p 패키지명` |
| 의존성 관계 확인 | `cargo tree` |
| 의존성이 들어온 경로 확인 | `cargo tree -i 패키지명` |
| 중복 버전 확인 | `cargo tree --duplicates` |
| 활성화된 feature 관계 확인 | `cargo tree -e features` |
| 기본 feature 없이 모든 타깃 lint | `cargo clippy --all-targets --no-default-features --locked -- -D warnings` |
| 모든 feature로 모든 타깃 lint | `cargo clippy --all-targets --all-features --locked -- -D warnings` |
| 모든 타깃 컴파일 | `cargo check --all-targets --locked` |
| 기본 feature 없이 컴파일 | `cargo check --no-default-features --locked` |
| 모든 feature로 컴파일 | `cargo check --all-features --locked` |
| 벤치마크를 제외한 테스트 실행 | `cargo test --lib --tests --examples --locked` |
| 문서 테스트 실행 | `cargo test --doc --locked` |
| 현재 패키지 API 문서 생성 | `cargo doc --no-deps --locked` |
| 오류 코드 설명 확인 | `rustc --explain E0382` |
| 벤치마크 컴파일 | `cargo bench --no-run --locked` |
| 특정 벤치마크 실행 | `cargo bench --bench search --locked` |
| 지원하는 가장 오래된 Rust에서 검사 | `make msrv` |
| 전체 강의 검증 | `make check` |

## 편집기 설정 한눈에 보기

| 편집기 | 설정 위치 | 핵심 설정 |
| --- | --- | --- |
| VS Code | `.vscode/settings.json` | `editor.formatOnSave`, `rust-analyzer.check.command` |
| Zed | `.zed/settings.json` | `format_on_save`, `formatter`, `check.command` |

두 편집기 모두 저장할 때 rustfmt를 실행하고 rust-analyzer의 검사 명령을 Clippy로
바꿀 수 있습니다. 구체적인 JSON 설정은
[VS Code와 Zed에서 rustfmt와 Clippy 사용하기](../editors/09-vscode-zed.md)에서
확인하세요.

## 문제를 만났을 때

- rustfmt가 파일을 바꿨다면 diff를 읽고 의도하지 않은 파일이 포함됐는지 확인합니다.
- Clippy lint 이름을 읽고 경고의 원인을 설명한 뒤 수정 여부를 정합니다.
- 자동 수정 뒤에는 diff를 읽고 feature별 검사와 테스트를 다시 실행합니다.
- 의존성을 갱신했다면 `Cargo.toml`과 `Cargo.lock`의 변경을 함께 확인합니다.
- 예상하지 않은 의존성은 `cargo tree -i`로 들어온 경로를 찾습니다.
- 컴파일 오류는 첫 번째 오류부터 읽고 수정한 뒤 다시 검사합니다.
- `allow`가 필요하면 가장 좁은 범위에 두고 이유를 주석으로 남깁니다.
- 공개 API의 대표 사용법은 문서 테스트로 실행되는지 확인합니다.
- 생성된 API 문서에서 항목을 찾기 쉽고 설명이 자연스럽게 이어지는지 읽어 봅니다.
- 벤치마크 결과가 흔들리면 반복 실행하고 시스템 부하와 측정 구간을 확인합니다.
- 빠른 구현을 채택하기 전에 테스트로 두 구현의 결과가 같은지 확인합니다.
