# 우아한 Rust 중급: 개발 도구 슬라이드

이 디렉터리에는 `VIDEO_COURSE.md`의 11편 구성에 맞춘 Slidev 강의 자료가 들어
있습니다. 각 영상은 독립된 덱이므로 필요한 편만 실행하거나 PDF로 내보낼 수
있습니다.

## 준비

```bash
make install
```

## 발표 화면 열기

기본값은 11편을 모두 묶은 통합 덱입니다.

```bash
make serve
```

개별 영상을 열 때는 파일 이름에서 `.md`를 뺀 값을 지정합니다.

```bash
make serve EPISODE=06-dependencies
```

통합 덱의 목차에서 챕터 제목을 누르면 해당 편의 표지로 이동합니다. 각 편은 원본
파일을 그대로 가져오므로 개별 덱과 통합 덱의 내용을 따로 관리할 필요가 없습니다.

발표 화면에서 `P`를 누르면 발표자 모드를 열 수 있습니다.

## 4K 녹화 가이드

발표 주소에 `?guide=4k`를 붙이면 모든 영상에서 4K 녹화 가이드를 표시할 수
있습니다.

```text
http://localhost:3030/?guide=4k
```

발표 화면의 하단 컨트롤 바에 있는 `4K` 버튼으로도 가이드를 바로 켜고 끌 수
있습니다. 버튼이 청록색이면 가이드가 켜진 상태입니다.

가이드는 브라우저 전체가 아니라 Slidev가 실제로 그린 **슬라이드 영역**의 크기와
기기의 DPR(device pixel ratio)을 곱한 예상 픽셀 크기를 보여 줍니다. 브라우저에
주소 표시줄이나 레터박스 여백이 있어도 계산에 포함하지 않습니다. 녹색이면
슬라이드 영역이 3840×2160과 정확히 일치하고, 노란색이면 16:9 비율만 일치합니다.

바깥 실선이 녹화할 슬라이드 경계이고, 점선은 그 경계에서 5% 안쪽인 안전
여백입니다. 브라우저 크기나 확대율을 조절해 실선 안쪽의 예상 크기를
3840×2160으로 맞춘 뒤, OBS나 ScreenFlow의 녹화 영역도 이 실선에 맞추세요. 실제
촬영 전에는 주소에서 `?guide=4k`를 제거해 가이드를 숨깁니다. Slidev의
`canvasWidth`는 슬라이드 내부 좌표계이므로 4K에 맞추려고 변경하지 않습니다.

## 빌드와 내보내기

```bash
make build EPISODE=01-toolchain
make export EPISODE=01-toolchain
make check
```

PDF 내보내기에는 Slidev가 사용하는 브라우저가 추가로 필요할 수 있습니다. 화면
녹화는 발표자 모드와 OBS 또는 ScreenFlow를 함께 사용하는 방식을 권합니다.

## 영상별 파일

| 편 | 파일 |
| --- | --- |
| 전체 | `episodes/all.md` |
| 1 | `episodes/01-toolchain.md` |
| 2 | `episodes/02-cargo-check.md` |
| 3 | `episodes/03-rustfmt.md` |
| 4 | `episodes/04-clippy.md` |
| 5 | `episodes/05-editor-fix.md` |
| 6 | `episodes/06-dependencies.md` |
| 7 | `episodes/07-features-docs.md` |
| 8 | `episodes/08-tests.md` |
| 9 | `episodes/09-error-doc-tests.md` |
| 10 | `episodes/10-benchmark.md` |
| 11 | `episodes/11-workflow.md` |

슬라이드의 Rust 코드는 가능한 한 상위 디렉터리의 `examples`, `src`, `tests` 파일을
직접 가져옵니다. 예제 파일을 고치면 슬라이드에도 같은 코드가 나타납니다.
