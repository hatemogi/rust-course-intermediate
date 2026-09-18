# 우아한 Rust 중급: 패턴 문법

패턴은 값의 모양을 확인하면서 그 값의 일부에 이름을 붙이는 문법입니다. `match`의
각 갈래<sub>branch</sub>뿐 아니라 `let`, 함수 매개변수, `for`, `if let`,
`let else`에서도 패턴을 사용합니다.

패턴을 잘 쓰면 값을 꺼낸 뒤 다시 조건문으로 확인하는 단계를 줄일 수 있습니다.
반면 한 패턴에 너무 많은 조건을 넣으면 분기의 우선순위와 소유권 이동을 알아보기
어려워집니다. 이 강의에서는 패턴 문법을 익히고, 분기 조건과 소유권 이동이
잘 드러나는 코드를 작성하는 방법을 배웁니다.

먼저 패턴이 항상 매칭<sub>matching</sub>되는지 판단하는 법을 배운 뒤
튜플·구조체·열거형·슬라이스를 구조 분해<sub>destructuring</sub>합니다. 이어서
범위·OR 패턴·매치 가드·참조 바인딩을 조합하고, 마지막 실습에서는 문자열 명령과
작업 상태를 여러 형태의 패턴으로 처리해 봅니다.

## 공개 저장소

중급 강의 자료는 공개 GitHub 저장소에 강의별 디렉터리로 나누어 두었습니다.

- 전체 저장소: <https://github.com/hatemogi/rust-course-intermediate>
- 이 강의 디렉터리: <https://github.com/hatemogi/rust-course-intermediate/tree/main/rust-course-patterns>

“우아한 프로그래밍 언어 Rust 입문” 강의를 들었다고 가정합니다.
