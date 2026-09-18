# 패턴 문법 퀴즈와 풀이

각 문제에서 가장 알맞은 답을 하나 고르세요. 먼저 아홉 문제를 모두 푼 뒤
[정답과 풀이](#정답과-풀이)에서 판단 근거를 확인합니다.

## 문제

### 1. `_`와 이름 있는 바인딩

코드를 실행한 뒤 두 변수를 각각 전체 값으로 다시 읽을 수 있는 경우는 무엇인가요?

```rust
{{#rustdoc ../../../examples/12_quiz.rs statements=underscore}}
```

<ul class="quiz-options">
<li>① <code>kept</code>와 <code>moved</code> 모두 읽을 수 있습니다.</li>
<li>② <code>kept</code>만 읽을 수 있습니다.</li>
<li>③ <code>moved</code>만 읽을 수 있습니다.</li>
<li>④ 둘 다 읽을 수 없습니다.</li>
</ul>

### 2. 반박 가능한 패턴

타입이 맞는 값을 받는 일반 `let`에서 사용할 수 없는 패턴은 무엇인가요?
`Point`에는 `x`와 `y` 필드만 있다고 가정합니다.

<ul class="quiz-options">
<li>① <code>(x, y)</code></li>
<li>② <code>Point { x, y }</code></li>
<li>③ <code>Some(value)</code></li>
<li>④ <code>[first, second]</code>를 길이가 2인 배열에 사용</li>
</ul>

### 3. 구조 분해

이름 있는 필드를 가진 구조체 `User`에서 `name`만 꺼내고 나머지는 무시하는
패턴은 무엇인가요?

<ul class="quiz-options">
<li>① <code>User { name, .. }</code></li>
<li>② <code>User(name)</code></li>
<li>③ <code>User => name</code></li>
<li>④ <code>User::*</code></li>
</ul>

### 4. `if let`과 `let else`

값이 특정 패턴과 매칭<sub>matching</sub>되지 않을 때 함수를 일찍 끝내고,
매칭되면 이후 코드에서 바인딩을 계속 쓰려 합니다. 가장 알맞은 문법은 무엇인가요?

<ul class="quiz-options">
<li>① <code>while let</code></li>
<li>② <code>let PATTERN = value else { return; };</code></li>
<li>③ <code>for PATTERN in value</code></li>
<li>④ <code>matches!</code>만 사용</li>
</ul>

### 5. 범위의 경계

다음 코드에서 `label`에 들어가는 값은 무엇인가요?

```rust
{{#rustdoc ../../../examples/12_quiz.rs statements=boundary}}
```

<ul class="quiz-options">
<li>① <code>"가"</code></li>
<li>② <code>"나"</code></li>
<li>③ <code>"다"</code></li>
<li>④ 두 범위가 겹쳐 컴파일 오류가 납니다.</li>
</ul>

### 6. 가드가 거짓일 때

다음 코드에서 `label`에 들어가는 값은 무엇인가요?

```rust
{{#rustdoc ../../../examples/12_quiz.rs statements=guard}}
```

<ul class="quiz-options">
<li>① <code>"범위 안"</code></li>
<li>② <code>"초과"</code></li>
<li>③ <code>"없음"</code></li>
<li>④ 첫 패턴에 매칭되었으므로 가드가 거짓이면 패닉이 납니다.</li>
</ul>

### 7. 슬라이스 패턴

길이가 하나 이상인 슬라이스에서 첫 원소와 나머지를 나누는 패턴은 무엇인가요?

<ul class="quiz-options">
<li>① <code>[first, rest]</code></li>
<li>② <code>[first, rest..]</code></li>
<li>③ <code>[first, rest @ ..]</code></li>
<li>④ <code>(first, rest)</code></li>
</ul>

### 8. 참조 바인딩의 타입

다음 코드에서 `text`의 타입과 `match` 이후의 상태를 올바르게 설명한 것은 무엇인가요?

```rust
{{#rustdoc ../../../examples/12_quiz.rs statements=reference}}
```

<ul class="quiz-options">
<li>① <code>text</code>는 <code>&String</code>이며 원본 <code>message</code>를 다시 읽을 수 있습니다.</li>
<li>② <code>text</code>는 <code>String</code>이며 원본의 내부 값이 이동합니다.</li>
<li>③ <code>text</code>는 <code>&mut String</code>이며 문자열을 변경할 수 있습니다.</li>
<li>④ <code>text</code>는 <code>&&String</code>이며 참조를 두 번 분해해야 합니다.</li>
</ul>

### 9. 매칭 여부만 확인하기

`input`의 타입은 `Option<Result<u8, &str>>`입니다. 값이 `Some(Ok(1..=3))`에
맞는지만 확인하고 내부 값은 이후에 사용하지 않습니다. 가장 알맞은 코드는
무엇인가요?

<ul class="quiz-options">
<li>① <code>input.is_some()</code></li>
<li>② <code>matches!(input, Some(_))</code></li>
<li>③ <code>matches!(input, Some(Ok(1..=3)))</code></li>
<li>④ <code>matches!(input, Ok(Some(1..=3)))</code></li>
</ul>

## 정답과 풀이

### 1. 정답: ②

`_`는 내부 값에 이름을 붙이지 않으므로 `kept`의 `String`은 이동하지 않습니다.
`_text`는 사용하지 않는 이름이어도 실제 바인딩이므로 `moved`의 `String`을
이동시킵니다. 따라서 둘 다 읽을 수 있다는 ①과 `moved`만 읽을 수 있다는 ③은
틀립니다. `kept`는 그대로 사용할 수 있으므로 ④도 맞지 않습니다.

### 2. 정답: ③

일반 `let`에는 반드시 매칭되는 반박 불가능한 패턴이 필요합니다. `Some(value)`는
입력이 `None`일 수 있으므로 `if let`, `let else`, `match` 같은 제어 흐름이
필요합니다.
①은 모든 2-튜플, ②는 해당 구조체의 모든 값을 받을 수 있습니다. ④도 길이가
타입에 포함된 2원소 배열에 사용하므로 반드시 매칭됩니다.

### 3. 정답: ①

구조체 패턴의 `..`는 나열하지 않은 나머지 필드를 무시합니다. 필요한 필드만
바인딩하면 구조체에 다른 필드가 있어도 패턴을 간결하게 유지할 수 있습니다.
②는 튜플 구조체에 쓰는 형태이므로 이름 있는 필드를 가진 `User`와 매칭되지 않습니다.
③과 ④는 Rust의 구조체 패턴 문법이 아닙니다.

### 4. 정답: ②

`let else`는 패턴이 매칭되지 않으면 `else` 블록에서 현재 제어 흐름을 빠져나가게
합니다. 패턴이 매칭되면 바인딩한 이름을 이후 문장에서 계속 사용할 수 있습니다.
①은 패턴이 계속 매칭되는 동안 반복할 때 사용하고, ③은 반복 가능한 값을
순회합니다. ④는 매칭 여부만 계산하므로 내부 값을 이후 코드에 바인딩하지 않습니다.

### 5. 정답: ②

`1..10`은 끝값 10을 제외하므로 첫 갈래와 매칭되지 않습니다. `10..=99`는 10을
포함하므로 `"나"`가 됩니다. 따라서 ①은 끝값 포함 여부를 잘못 판단했습니다.
두 번째 갈래가 선택되어 ③은 실행되지 않습니다. 두 범위는 겹치지 않으며,
범위가 겹친다는 이유만으로 반드시 컴파일 오류가 나는 것도 아니므로 ④도 틀립니다.

### 6. 정답: ①

매치 가드<sub>match guard</sub>의 `3 > 5`가 거짓이므로 다음
갈래<sub>branch</sub>인 `Some(_)`를 검사해 `"범위 안"`을 선택합니다.
②는 가드를 통과해야 선택할 수 있습니다. 값이 `None`이 아니므로 ③도 아닙니다.
가드가 거짓이라고 패닉이 나지는 않으므로 ④도 틀립니다.

### 7. 정답: ③

`rest @ ..`는 나머지 슬라이스 패턴 `..`에 `rest`라는 이름을 붙입니다. 첫 원소와
나머지 구간을 모두 사용해야 할 때 알맞습니다.
①은 원소가 정확히 두 개인 슬라이스에만 매칭되고, ②는 올바른 나머지 패턴 문법이
아닙니다. ④는 슬라이스가 아니라 튜플 패턴입니다.

### 8. 정답: ①

`&Option<String>`을 `Some(text)`로 분해하므로 매치 참조 편의
문법<sub>match ergonomics</sub>에 따라 `text`는 `&String`이 됩니다.
문자열을 빌려 읽으므로 `message`의 내부 값은 이동하지 않아 ②는 틀립니다.
공유 참조에서 변경 가능한 참조를 만들지 않으므로 ③도 아닙니다. 이 패턴에서
참조가 두 겹으로 바인딩되지 않으므로 ④도 맞지 않습니다.

### 9. 정답: ③

내부 값을 이후에 사용하지 않고 매칭 여부만 필요하므로 `matches!`가 알맞습니다.
`Some(Ok(1..=3))`은 값이 있고, 작업이 성공했으며, 성공한 값이 1부터 3까지라는
세 조건을 한 패턴에 모읍니다. ①과 ②는 `Some(Err(_))`나 범위 밖의 성공값까지
`true`로 판단합니다. ④는 `Option`과 `Result`의 중첩 순서를 반대로 썼으므로
`input`의 타입과 맞지 않습니다.
