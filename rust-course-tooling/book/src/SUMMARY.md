# Summary

[들어가며](./index.md)

# Rust 도구 체인 준비

- [`rustup`, `rustc`와 Cargo의 역할](./toolchain/01-rustup-basics.md)
- [`rust-toolchain.toml`로 환경 맞추기](./toolchain/02-project-toolchain.md)

# Cargo와 컴파일

- [`cargo check`와 Cargo 타깃](./compilation/03-cargo-check.md)
- [컴파일 오류 읽기](./compilation/04-reading-errors.md)

# 코드 형식과 정적 검사

- [`cargo fmt`로 코드 형식 맞추기](./formatting/05-cargo-fmt.md)
- [형식 검사를 작업 흐름에 넣기](./formatting/06-fmt-check.md)
- [Clippy 실행하고 결과 읽기](./clippy/07-running-clippy.md)
- [Clippy 제안을 판단하는 기준](./clippy/08-judging-lints.md)
- [VS Code와 Zed에서 rustfmt와 Clippy 사용하기 ◆](./editors/09-vscode-zed.md)

# Cargo 프로젝트 관리 ◆

- [`cargo fix`와 자동 수정 검토하기](./cargo/10-cargo-fix.md)
- [의존성과 `Cargo.lock` 관리하기](./cargo/11-dependencies-lockfile.md)
- [`cargo tree`로 의존성 관계 찾기](./cargo/12-cargo-tree.md)
- [Cargo feature와 검사 조합](./cargo/13-features.md)

# API 문서 ◆

- [`cargo doc`으로 API 문서 확인하기](./documentation/14-cargo-doc.md)

# 자동 테스트 ◆

- [`cargo test`와 기본 assertion](./testing/15-cargo-test.md)
- [단위 테스트와 통합 테스트](./testing/16-unit-integration.md)
- [오류, panic과 제외한 테스트](./testing/17-errors-panic-ignore.md)
- [문서 주석과 문서 테스트](./testing/18-documentation-tests.md)

# 성능 측정 ◆

- [벤치마크 전에 확인할 것](./benchmark/19-principles.md)
- [`cargo bench`로 반복 측정하기](./benchmark/20-cargo-bench.md)

# 실습과 정리

- [첫 번째 검증 흐름 실습](./practice/09-free-workflow.md)
- [포맷, 린트, 테스트, 측정 연결하기 ◆](./practice/21-workflow.md)
- [종합 실습 ◆](./practice/22-exercises.md)

# 부록

- [개발 도구 명령 한눈에 보기 ◆](./appendix/23-cheatsheet.md)
