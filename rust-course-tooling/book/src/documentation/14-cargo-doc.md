# `cargo doc`으로 API 문서 확인하기

문서 테스트는 코드 블록이 컴파일되고 실행되는지를 확인합니다. 하지만 함수 설명이
찾기 쉬운지, 타입과 메서드가 문서에서 자연스럽게 이어지는지는 생성된 API 문서를
직접 읽어야 알 수 있습니다.

```bash
cargo doc --no-deps --locked
```

`cargo doc`은 rustdoc을 실행해 `target/doc`에 HTML 문서를 만듭니다. `--no-deps`를
지정하면 의존성 문서는 다시 만들지 않고 현재 패키지의 문서만 생성합니다.
`--locked`는 이 과정에서 잠금 파일이 바뀌어야 한다면 실패하게 합니다.

브라우저에서 바로 확인하려면 다음 명령을 사용합니다.

```bash
cargo doc --no-deps --open
```

`--open`은 문서를 만든 뒤 브라우저를 여는 동작이므로 사람이 결과를 확인할 때
사용합니다. 자동 검사에서는 브라우저를 열지 않는 명령을 실행합니다.

## crate와 공개 항목 설명하기

crate 루트의 `//!` 주석은 crate 전체를 설명하고, 공개 함수나 타입 앞의 `///`
주석은 해당 항목을 설명합니다. 이 강의의 `src/lib.rs`에서도 두 종류를 함께
사용합니다.

```rust
{{#include ../../../src/lib.rs:1:15}}
```

기본 문서에는 외부에서 접근할 수 있는 공개 항목만 나타납니다. 내부 함수까지
살펴보고 싶다면 다음 옵션을 추가합니다.

```bash
cargo doc --no-deps --document-private-items
```

내부 구현을 문서화한다고 해서 모두 공개 API가 되는 것은 아닙니다. 이 옵션은
설계를 검토하거나 프로젝트 내부 문서를 확인할 때 사용합니다.

## feature별 문서 확인하기

feature에 따라 공개 API가 달라진다면 문서도 그 구성에 따라 달라집니다.

```bash
cargo doc --no-deps --no-default-features --locked
cargo doc --no-deps --all-features --locked
```

자동 검사에서 문서를 생성하면 잘못된 intra-doc link처럼 문서 생성 단계에서
발견되는 문제를 확인할 수 있습니다. `--all-features`에서는 feature를 껐을 때만
나타나는 공개 항목이 빠질 수 있으므로 프로젝트가 지원하는 구성별로 생성해야
합니다. 다만 문장이 이해하기 쉬운지와 예제가 실제 사용 순서를 잘 보여 주는지는
사람이 생성된 문서를 읽고 판단해야 합니다.

명령별 대상과 옵션은 [Cargo의 `cargo doc` 문서](https://doc.rust-lang.org/cargo/commands/cargo-doc.html)와
[rustdoc 안내서](https://doc.rust-lang.org/rustdoc/)에서 확인할 수 있습니다.
