# 한글과 유니코드 정규화

화면에서 똑같이 보이는 한글도 서로 다른 유니코드 스칼라 값<sub>Unicode scalar value</sub>의
배열로 저장될 수 있습니다. 한글 음절 하나를 미리 조합된
코드 포인트<sub>code point</sub> 하나로 표현할 수도 있고, 초성·중성·종성용 자모
코드 포인트로 나누어 표현할 수도 있습니다.

## 같은 모양을 서로 다르게 저장하기

다음 두 문자열은 모두 화면에서 `한`으로 보이지만 내부 표현은 다릅니다.

```rust
{{#include ../../../examples/10_hangul_normalization.rs:representations}}
```

합쳐진 문자열은 `한`을 `U+D55C` 하나로 저장합니다. 분리된 문자열은 초성 `ᄒ`
`U+1112`, 중성 `ᅡ` `U+1161`, 종성 `ᆫ` `U+11AB` 세 개로 저장합니다. Rust의
문자열 비교는 UTF-8 바이트열을 비교하므로 두 문자열을 그대로 비교하면 서로
다릅니다.

여기서 분리된 자모는 한글 음절을 조합할 때 사용하는 자모입니다. 각각 독립된
모양으로 쓰는 호환 자모<sub>Hangul Compatibility Jamo</sub> `ㅎㅏㄴ`과는 코드
포인트가 다릅니다.

## 유니코드 정규화 추가하기

Rust 표준 라이브러리에는 유니코드 정규화<sub>Unicode normalization</sub> 기능이
없습니다. 다음 명령으로 `unicode-normalization` 크레이트를 추가합니다.

```bash
cargo add unicode-normalization
```

코드에서는 `UnicodeNormalization` 트레이트<sub>trait</sub>를 가져와 사용합니다.

```rust
use unicode_normalization::UnicodeNormalization;
```

## 분리된 자모를 합치기

정규 조합 형식<sub>Normalization Form C, NFC</sub>은 정규 분해가 가능한 문자를
분해한 뒤, 다시 합칠 수 있는 코드 포인트를 합칩니다. 분리된 한글 자모열에
`nfc`를 적용하면 합쳐진 음절로 바꿀 수 있습니다.

```rust
{{#include ../../../examples/10_hangul_normalization.rs:nfc}}
```

`nfc`는 문자를 차례로 돌려주는 반복자를 반환하므로 `collect::<String>()`으로
새 문자열을 만듭니다.

## 합쳐진 한글을 자모로 나누기

정규 분해 형식<sub>Normalization Form D, NFD</sub>은 문자를 정규 분해 형태로
바꿉니다. 합쳐진 한글 음절에 `nfd`를 적용하면 초성·중성·종성용 자모로 나눌 수
있습니다.

```rust
{{#include ../../../examples/10_hangul_normalization.rs:nfd}}
```

NFD로 나눈 결과는 화면 글자 수를 세기 위한 값이 아닙니다. `chars().count()`는
합쳐진 문자열에서 1, 분리된 문자열에서 3을 반환할 수 있습니다.

## 비교하고 검색하기 전에 정규화하기

외부에서 받은 문자열의 정규화 형태가 다르면 같은 모양의 한글도 비교와 검색에
실패할 수 있습니다.

```rust
{{#include ../../../examples/10_hangul_normalization.rs:compare}}
```

입력과 저장된 값을 같은 정규화 형태로 맞춘 뒤 비교해야 합니다. 일반적인
문자열 저장·비교에는 NFC를 많이 사용하지만, 외부 규격이 특정 형태를 요구한다면
그 규격을 따라야 합니다. 정규화는 대소문자 변환이나 맞춤법 교정이 아니라,
유니코드에서 같은 문자로 정의한 여러 표현을 일정한 형태로 맞추는 작업입니다.

자세한 정규화 규칙과 API는
[`unicode-normalization` 문서](https://docs.rs/unicode-normalization/latest/unicode_normalization/)에서
확인할 수 있습니다.
