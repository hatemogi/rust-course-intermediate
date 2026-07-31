# VS Code와 Zed에서 rustfmt와 클리피 사용하기

터미널에서 `cargo fmt`와 `cargo clippy`를 실행하는 방법을 알았다면 편집기에서도 같은
도구를 사용할 수 있습니다. 저장할 때 rustfmt로 파일을 정리하고, rust-analyzer가
`cargo check` 대신 `cargo clippy`를 실행하게 설정하면 코드를 작성하는 동안 문제를
더 일찍 발견할 수 있습니다.

먼저 프로젝트가 사용하는 툴체인<sub>toolchain</sub>에 필요한
컴포넌트<sub>component</sub>가 있는지 확인합니다.

```bash
rustup component add rustfmt clippy
```

편집기에서는 카고<sub>Cargo</sub> 프로젝트의 루트, 즉 `Cargo.toml`이 있는
디렉터리를 여세요.
하위 파일 하나만 열면 rust-analyzer가 프로젝트 전체의 타깃과 설정을 제대로 찾지
못할 수 있습니다.

## VS Code 설정

VS Code에서는 공식 `rust-analyzer` 확장을 설치합니다. 설정 화면에서 다음 두 항목을
찾아 바꿀 수도 있고, 프로젝트의 `.vscode/settings.json`에 직접 적을 수도 있습니다.

```json
{
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.formatOnSave": true
  },
  "rust-analyzer.check.command": "clippy"
}
```

- `editor.formatOnSave`는 Rust 파일을 저장할 때 rust-analyzer를 통해
  rustfmt를 실행합니다.
- `rust-analyzer.check.command`를 `clippy`로 바꾸면 rust-analyzer가 저장한 코드를
  검사할 때 `cargo clippy`를 사용합니다. 진단은 코드와 **Problems** 창에
  표시됩니다.
- 설정을 프로젝트 구성원과 공유하려면 `.vscode/settings.json`을 Git에
  포함합니다. 개인 설정으로만 쓰려면 VS Code의 사용자 설정에 같은 내용을
  추가합니다.

명령 팔레트에서 **Format Document**를 실행하면 저장하지 않고도 현재 파일의 형식을
맞출 수 있습니다. 클리피<sub>Clippy</sub>가 제안한 빠른 수정은 적용하기 전에
코드의 동작과 공개 API가 그대로인지 확인하세요.

자세한 설정 방법은 [VS Code의 공식 Rust 안내서](https://code.visualstudio.com/docs/languages/rust)에서
확인할 수 있습니다.

## Zed 설정

Zed에는 Rust와 rust-analyzer 지원이 들어 있습니다. 프로젝트에서만 사용할 설정은
`.zed/settings.json`에 다음처럼 작성합니다.

```json
{
  "languages": {
    "Rust": {
      "format_on_save": "on",
      "formatter": "language_server"
    }
  },
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "check": {
          "command": "clippy"
        }
      }
    }
  }
}
```

- `format_on_save`를 `on`으로 두면 파일을 저장할 때 형식을 맞춥니다.
- `formatter`를 `language_server`로 지정하면 Rust 파일은 rust-analyzer를 거쳐
  rustfmt로 정리됩니다.
- `check.command`를 `clippy`로 지정하면 rust-analyzer의 진단에 클리피 결과가
  포함됩니다. 이 설정을 바꾼 뒤 기존 창에 바로 반영되지 않으면 rust-analyzer를
  다시 시작하거나 프로젝트를 다시 여세요.

모든 프로젝트에서 같은 설정을 사용하려면 Zed의 사용자 `settings.json`에 넣을 수
있습니다. Zed의 Rust 지원과 설정 형식은 [Zed의 Rust 문서](https://zed.dev/docs/languages/rust)와
[전체 설정 문서](https://zed.dev/docs/reference/all-settings)에서 확인할 수 있습니다.

## 편집기 진단과 전체 검증 구분하기

편집기가 보여 주는 진단은 코드를 작성할 때 빠르게 확인하기 위한 것입니다. 현재
열어 둔 프로젝트의 기본 타깃과 rust-analyzer 설정에 따라 검사 범위가 달라질 수
있으므로, 편집기에 경고가 없다는 사실만으로 모든 검증이 끝났다고 판단하면 안
됩니다.

변경을 마치기 전에는 터미널에서 저장소가 정한 명령을 따로 실행하세요.

```bash
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
```

이 강의 프로젝트에서는 두 명령뿐 아니라 테스트, 문서 테스트, mdBook 빌드까지
`make check`로 확인합니다.
