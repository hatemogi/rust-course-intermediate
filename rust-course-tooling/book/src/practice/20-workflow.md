# 포맷·린트·테스트 연결하기

다음 순서로 도구들을 활용하면 문제 범위를 빨리 좁힐 수 있습니다.

코드를 작성하는 동안에는 소스 코드를 저장할 때 rustfmt로 형식을 맞추고
클리피<sub>Clippy</sub>로 린트<sub>lint</sub>를 빠르게 확인합니다. 변경 작업을 마치면 편집기
진단 내용에만 의존하지 않고 다음 명령을 차례로 실행합니다.

1. `rustup show active-toolchain`으로 프로젝트에 선택된 툴체인<sub>toolchain</sub>을
   확인합니다.
2. `cargo fmt --check`로 코드 형식을 점검합니다.
3. 클리피를 `--no-default-features`와 `--all-features`로 각각 실행해 피처<sub>feature</sub>를
   켰을 때 들어오는 코드와 빠지는 코드를 함께 검사합니다.
4. `cargo check`를 기본 구성, `--no-default-features`와 `--all-features`로 각각
   실행해 지원하는 피처 구성을 컴파일합니다.
5. `cargo test`도 세 가지 피처 구성으로 실행해 동작을 검증합니다.
6. 문서 테스트도 `--no-default-features`와 `--all-features`로 각각 실행합니다.
7. API 문서도 두 피처 구성으로 생성해 공개 항목과 연결을 확인합니다.

`--all-features`를 실행해도 `cfg(not(feature = "..."))` 분기는 컴파일되지 않을 수
있습니다. 옵션 이름만 보고 모든 조건부 코드를 검사했다고 판단하지 말고 실제
`cfg` 조건과 프로젝트가 지원하는 조합을 확인하세요.

이 프로젝트의 `make check`는 툴체인을 바꾸지 않으며 2번부터 7번, mdBook
빌드까지 실행합니다. 툴체인 설치와 업데이트는 사용자 환경을 바꾸므로 자동 검사에
넣지 않고 프로젝트를 시작할 때 별도로 확인합니다.

```bash
make check
```

패키지가 지원하는 가장 오래된 Rust 버전은 해당 툴체인이 설치된 환경에서 별도로
확인합니다.

```bash
make msrv
```

도구들이 모든 문제를 해결하는 것은 아닙니다. rustfmt를 통과한 코드도 잘못된 결과를
낼 수 있고, 클리피 경고가 없는 코드도 요구 사항을 어길 수 있습니다.
