# 문자열 검색하기

## 문자열이 들어 있는지 확인하기

`contains`는 문자열 안에 찾는 내용이 들어 있는지 확인합니다. 문자열의 시작과
끝만 검사하려면 `starts_with`와 `ends_with`를 사용합니다.

```rust
{{#include ../../../examples/05_search.rs:contains}}
```

세 메서드는 위치가 필요하지 않고 포함 여부만 확인할 때 알맞습니다.

## 문자열의 위치 찾기

처음 일치하는 위치는 `find`, 마지막으로 일치하는 위치는 `rfind`로 찾습니다.
찾는 내용이 없으면 `None`을 반환합니다.

```rust
{{#include ../../../examples/05_search.rs:position}}
```

`find`와 `rfind`가 돌려주는 위치는 바이트 인덱스입니다. 이 값을 문자열 슬라이스의
경계로 사용할 때도 UTF-8 문자 경계를 지켜야 합니다.
