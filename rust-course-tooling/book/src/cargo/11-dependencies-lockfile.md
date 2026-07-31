# 의존성과 `Cargo.lock` 관리하기

`Cargo.toml`에는 프로젝트에서 사용할 수 있는 의존성의 버전 범위를 적습니다.
`Cargo.lock`에는 카고<sub>Cargo</sub>가 그 범위 안에서 실제로 선택한 직접·간접
의존성의 정확한 버전과 출처가 기록됩니다. `Cargo.lock`은 카고가 관리하므로 직접
편집하지 않습니다.

예를 들어 `Cargo.toml`에 일반 의존성과 개발할 때만 쓰는 의존성을 다음처럼 나누어
적을 수 있습니다.

```toml
[package]
name = "dependency-example"
version = "0.1.0"
edition = "2024"

[dependencies]
serde = { version = "1", features = ["derive"] }

[dev-dependencies]
pretty_assertions = "1"
```

`version = "1"`은 1.x 버전 가운데 호환되는 버전을 카고가 선택할 수 있다는
뜻입니다. 실제로 선택된 정확한 버전은 `Cargo.lock`에서 확인합니다.

## 의존성 추가하고 제거하기

`cargo add`는 의존성 항목을 알맞은 표에 추가합니다.

```bash
cargo add serde --features derive
cargo add pretty_assertions --dev
```

첫 번째 명령은 `[dependencies]`에 `serde`를 추가하고 `derive` 피처<sub>feature</sub>를
켭니다.
두 번째 명령은 테스트와 예제에서 사용할 개발 의존성을 `[dev-dependencies]`에
추가합니다. 더 이상 필요하지 않은 의존성은 다음처럼 제거합니다.

```bash
cargo remove serde
cargo remove pretty_assertions --dev
```

명령을 실행한 뒤에는 `Cargo.toml`과 `Cargo.lock`의 diff를 함께 읽으세요. 이름이
비슷한 다른 패키지를 추가하지 않았는지, 불필요한 피처가 켜지지 않았는지
확인해야 합니다.

## 잠금 파일 업데이트하기

`cargo update`는 `Cargo.toml`이 허용하는 범위 안에서 의존성을 다시 선택하고
`Cargo.lock`을 바꿉니다.

```bash
cargo update
cargo update -p serde
```

첫 번째 명령은 업데이트할 수 있는 의존성을 모두 다시 선택합니다. 변경 범위를 줄이고
싶다면 두 번째 명령처럼 패키지를 지정합니다. 직접 의존성 하나를 업데이트해도 그
패키지가 사용하는 간접 의존성이 함께 달라질 수 있으므로 잠금 파일의 변경량과
테스트 결과를 확인해야 합니다.

`Cargo.lock`은 보통 Git에 포함합니다. 그러면 팀과 자동 검사 환경이 같은 버전의
의존성을 사용합니다. 라이브러리를 배포할 때 잠금 파일이 어떻게 사용되는지는
실행 프로그램과 차이가 있지만, 저장소에 포함할지 확신하기 어렵다면 포함하는
것이 카고의 권장 방식입니다.

## 잠금 파일과 네트워크 사용 제한하기

다음 옵션은 비슷해 보이지만 확인하는 내용이 다릅니다.

| 옵션 | 동작 |
| --- | --- |
| `--locked` | 기존 `Cargo.lock`을 바꿔야 하면 실패합니다. |
| `--offline` | 네트워크에 접근하지 않고 이미 받은 자료만 사용합니다. |
| `--frozen` | `--locked`와 `--offline`을 함께 적용합니다. |

자동 검사에서는 의존성이 뜻밖에 바뀌지 않도록 `--locked`를 자주 사용합니다.
`--offline`은 필요한 패키지를 미리 내려받지 않았다면 실패하며, 온라인에서 선택할
결과와 달라질 수도 있습니다.

`Cargo.toml`과 `Cargo.lock`의 역할은 [카고 공식 안내서](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html)에서
더 자세히 설명합니다.
