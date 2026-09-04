# 우아한 Rust 중급: 문자열 활용

문자열은 파일 경로, 사용자 입력, JSON 필드와 오류 메시지처럼 거의 모든
프로그램의 경계에 나타납니다. Rust에서는 문자열을 소유하는 `String`과 문자열의
일부를 빌리는 `&str`을 구분합니다. 이 구분을 이해하면 불필요한 복사를 피하면서도
유효하지 않은 참조가 생기지 않는 API를 만들 수 있습니다.

이 강의에서는 한글이 포함된 문자열을 예제로 사용합니다. 영문 문자열에서 우연히
맞아 보이는 바이트 단위 코드가 UTF-8 텍스트에서 왜 잘못되는지도 직접 확인합니다.

## 공개 저장소

중급 강의 자료는 공개 GitHub 저장소에 강의별 디렉터리로 나누어 두었습니다.

- 전체 저장소: <https://github.com/hatemogi/rust-course-intermediate>
- 이 강의 디렉터리: <https://github.com/hatemogi/rust-course-intermediate/tree/main/rust-course-strings>

“우아한 프로그래밍 언어 Rust 입문” 강의를 들었다고 가정합니다.
