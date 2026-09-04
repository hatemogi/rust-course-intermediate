# 문자열 API 한눈에 보기

| 목적 | API | 결과 |
|---|---|---|
| 빈 문자열 만들기 | `String::new` | `String` |
| 용량을 미리 확보해 문자열 만들기 | `String::with_capacity` | `String` |
| 소유 문자열 만들기 | `String::from`, `to_owned` | `String` |
| 문자열 덧붙이기 | `push`, `push_str` | 원본 수정 |
| 부분 문자열 모두 바꾸기 | `replace` | `String` |
| 값을 조합해 문자열 만들기 | `format!` | `String` |
| 바이트 수 | `len` | `usize` |
| 유니코드 스칼라 값 순회 | `chars` | `Iterator<Item = char>` |
| 바이트 순회 | `bytes` | `Iterator<Item = u8>` |
| 문자열의 바이트 빌리기 | `as_bytes` | `&[u8]` |
| 소유 문자열을 바이트로 변환하기 | `into_bytes` | `Vec<u8>` |
| UTF-8 바이트를 문자열로 변환하기 | `String::from_utf8` | `Result<String, FromUtf8Error>` |
| 분리된 유니코드 표현 합치기 | `nfc` | 반복자 |
| 합쳐진 유니코드 표현 분해하기 | `nfd` | 반복자 |
| 안전하게 범위 빌리기 | `get(range)` | `Option<&str>` |
| 문자열 포함 여부 확인하기 | `contains` | `bool` |
| 문자열의 시작·끝 확인하기 | `starts_with`, `ends_with` | `bool` |
| 일치하는 위치 찾기 | `find`, `rfind` | `Option<usize>` |
| 구분자로 나누기 | `split` | 반복자 |
| 한 번 나누기 | `split_once` | `Option<(&str, &str)>` |
| 공백 단위 나누기 | `split_whitespace` | 반복자 |
| 목록 연결하기 | `join` | `String` |
| 숫자를 문자열로 변환하기 | `to_string`, `format!` | `String` |
| 문자열을 값으로 변환하기 | `parse::<T>()` | `Result<T, T::Err>` |
| 문자열을 날짜로 변환하기 | `NaiveDate::parse_from_str` | `Result<NaiveDate, ParseError>` |
| 날짜를 문자열로 변환하기 | `date.format(...).to_string()` | `String` |
| 사용자 정의 타입을 문자열에서 변환하기 | `FromStr` 구현 | `Result<T, T::Err>` |
| 사용자 정의 타입을 문자열로 변환하기 | `Display` 구현 뒤 `to_string` | `String` |
