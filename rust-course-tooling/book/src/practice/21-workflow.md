# 포맷, 린트, 테스트, 측정 연결하기

도구는 다음 순서로 연결하면 문제를 빨리 좁힐 수 있습니다.

코드를 작성하는 동안에는 편집기의 저장 시 포맷과 Clippy 진단으로 빠르게
확인합니다. 변경을 마치면 편집기 진단에만 의존하지 않고 다음 명령을 차례로
실행합니다.

1. `rustup show active-toolchain`으로 프로젝트에 선택된 도구 체인을 확인합니다.
2. `cargo fmt --check`로 형식 차이를 찾습니다.
3. Clippy를 `--no-default-features`와 `--all-features`로 각각 실행해 feature를
   켰을 때 들어오는 코드와 빠지는 코드를 함께 검사합니다.
4. `cargo check`를 기본 구성, `--no-default-features`와 `--all-features`로 각각
   실행해 지원하는 feature 구성을 컴파일합니다.
5. `cargo test`도 세 feature 구성으로 실행해 동작을 검증합니다.
6. 문서 테스트도 `--no-default-features`와 `--all-features`로 각각 실행합니다.
7. API 문서도 두 feature 구성으로 생성해 공개 항목과 연결을 확인합니다.
8. `cargo bench --no-run --locked`로 벤치마크가 낡지 않았는지 확인합니다.
9. 성능 판단이 필요할 때만 벤치마크를 실행하고 결과를 비교합니다.

`--all-features`를 실행해도 `cfg(not(feature = "..."))` 분기는 컴파일되지 않을 수
있습니다. 옵션 이름만 보고 모든 조건부 코드를 검사했다고 판단하지 말고 실제
`cfg` 조건과 프로젝트가 지원하는 조합을 확인하세요.

이 프로젝트의 `make check`는 도구 체인을 바꾸지 않으며 2번부터 8번, mdBook
빌드까지 실행합니다. 도구 체인 설치와 갱신은 사용자 환경을 바꾸므로 자동 검사에
넣지 않고 프로젝트를 시작할 때 별도로 확인합니다.

```bash
make check
```

패키지가 지원하는 가장 오래된 Rust 버전은 해당 도구 체인이 설치된 환경에서 별도로
확인합니다.

```bash
make msrv
```

`harness = false`인 벤치마크는 `cargo test --all-targets`에서도 실행될 수 있습니다.
벤치마크는 실행 시간과 환경의 영향을 받으므로 평소 검사에서는 대상을 나눠 컴파일만
확인합니다.
실제로 측정할 때는 다음 명령을 따로 사용합니다.

```bash
make bench
```

도구 하나가 다른 도구를 대신하지는 않습니다. rustfmt를 통과한 코드도 잘못된 값을
계산할 수 있고, Clippy 경고가 없는 코드도 느릴 수 있으며, 빠른 코드도 요구 사항을
어길 수 있습니다.
