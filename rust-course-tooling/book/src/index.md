# 우아한 Rust 중급: 개발 도구

코드를 작성한 뒤에는 형식을 맞추고, 의심스러운 부분을 찾고, 동작을 테스트해야
합니다. 이 교재에서는 Rust 프로젝트에서 자주 사용하는
`rustup`, `cargo check`, `rustfmt`, 클리피<sub>Clippy</sub>, 의존성 관리, `cargo doc`,
`cargo test`를 하나의 개발·검증 흐름 속에서 익힙니다.

도구의 출력을 그대로 따라 하는 수준을 넘어서, 각 도구가 무엇을 확인하는지
알아봅니다. 도구의 제안을 적용할지 판단하고, 그 근거를 코드의 목적과 테스트
결과로 설명하는 것이 이 강의의 목표입니다.

## 강의 프로젝트 내려받기

이 교재에서 사용하는 예제와 실습 코드는
[GitHub 강의 저장소](https://github.com/hatemogi/rust-course-intermediate)에서
내려받을 수 있습니다.

```bash
git clone https://github.com/hatemogi/rust-course-intermediate.git
cd rust-course-intermediate/rust-course-tooling
```

이후 별도의 디렉터리를 지정하지 않고 명령을 실행한다면
`rust-course-tooling` 디렉터리를 기준으로 합니다.

## 학습 목표

이 과정을 마치면 다음 작업을 할 수 있습니다.

- rustup으로 툴체인<sub>toolchain</sub>, 컴포넌트<sub>component</sub>, 컴파일
  대상을 확인하고 관리합니다.
- `rust-toolchain.toml`로 프로젝트가 사용할 Rust 버전과 도구를 설정합니다.
- `rust-toolchain.toml`과 `Cargo.toml`의 `rust-version`이 맡는 역할을 구분합니다.
- 카고<sub>Cargo</sub> 타깃을 구분하고 `cargo check`의 검사 범위를 지정합니다.
- 컴파일 오류에서 오류 코드부터 위치, 비고, 도움말까지 순서대로 읽습니다.
- `cargo fmt`와 `cargo fmt --check`의 쓰임을 구분합니다.
- 클리피를 모든 카고 타깃에 실행하고 린트<sub>lint</sub>가 지적한 문제와 제안 이유를
  이해합니다.
- 교육용 코드나 의도적인 구현에서 린트를 제한적으로 허용하고 그 이유를 남깁니다.
- VS Code와 Zed에서 저장할 때 rustfmt를 실행하고 클리피 진단을 표시하도록
  설정합니다.
- `cargo fix`와 `cargo clippy --fix`가 바꾼 코드를 diff와 테스트로 검토합니다.
- `Cargo.toml`과 `Cargo.lock`의 역할을 구분하고 의존성을 제한적으로 업데이트합니다.
- `cargo tree`로 의존성이 들어온 경로와 활성화된 피처<sub>feature</sub>를 찾습니다.
- 기본 구성, 기본 피처를 끈 구성, 모든 피처를 켠 구성을 각각 검사합니다.
- `cargo doc`으로 생성한 API 문서에서 공개 항목을 찾고 사용 순서를 확인합니다.
- 기본 assertion을 사용하고 검사할 내용에 따라 단위 테스트와 통합 테스트를
  알맞은 위치에 둡니다.
- 오류를 반환하는 테스트와 panic하거나 평소 제외한 테스트를 실행합니다.
- 공개 API의 사용법과 허용되지 않는 사용법을 문서 테스트로 검사합니다.
- 포맷, 린트, 피처별 컴파일, 테스트, API 문서 생성을 자동 검증 명령으로
  묶습니다.

“우아한 프로그래밍 언어 Rust 입문” 강의를 들었다고 가정합니다.

목차에서 `◆`가 붙은 주제는 전체 동영상 과정에서 이어서 다룹니다.
