# `Option`과 `Result` 퀴즈와 풀이

각 문제에서 가장 알맞은 답을 하나 고르세요. 열두 문제를 모두 푼 뒤
[정답과 풀이](#정답과-풀이)를 확인하세요.

## 문제

### 1. 값이 없는 경우

검색 결과가 없어도 정상적으로 처리하려고 합니다. 검색한 사용자 정보가 있거나
없을 수 있음을 나타내기에 가장 알맞은 타입은 무엇인가요?

- ① `Result<User, String>`
- ② `Option<User>`
- ③ `User`
- ④ `bool`

### 2. 기본값 계산

기본값을 구하는 함수 `default_port() -> u16`을 `None`일 때만 호출하려면
어떤 메서드를 사용해야 하나요?

- ① `port.unwrap_or(default_port())`
- ② `port.ok_or(default_port())`
- ③ `port.map(|_| default_port())`
- ④ `port.unwrap_or_else(default_port)`

### 3. map과 and_then

`Option<String>` 안의 값을 `fn inspect(value: String) -> Option<usize>`에
넘기려고 합니다. 결과를 한 겹 더 감싸지 않고 `Option<usize>`로 받는 식은 무엇인가요?

- ① `value.and_then(inspect)`
- ② `value.map(|text| Some(inspect(text)))`
- ③ `value.map(inspect)`
- ④ `value.as_ref().map(inspect)`

### 4. 빌린 값 다루기

`Option<String>`의 소유권을 옮기지 않고 안의 문자열을 읽으려면
어떤 방법이 알맞나요?

- ① `value.unwrap()`
- ② `value.into_iter().next()`
- ③ `value.as_ref()`
- ④ `value.take()`

### 5. Option에서 일찍 반환하기

`Option`을 반환하는 함수 안에서 `value?`가 `None`을 만나면 어떻게 되나요?

- ① 현재 함수에서 `None`을 반환합니다.
- ② 현재 함수에서 `Err`를 반환합니다.
- ③ 다음 줄을 실행합니다.
- ④ 반드시 패닉<sub>panic</sub>이 발생합니다.

### 6. 오류 바꾸기

`Result<u16, ParseIntError>`에서 성공한 값은 그대로 두고, 오류만
`ConfigError::InvalidPort`에 담으려면 어떤 메서드를 사용해야 하나요?

- ① `map(ConfigError::InvalidPort)`
- ② `ok()`
- ③ `map_err(ConfigError::InvalidPort)`
- ④ `unwrap_or(0)`

### 7. 오류 전파

`load_port`와 현재 함수가 같은 오류 타입의 `Result`를 반환합니다.
`let port = load_port()?;`는 어떻게 동작하나요?

- ① 실패하면 `port`에 기본값을 넣습니다.
- ② 실패하면 로그를 자동으로 출력합니다.
- ③ 성공과 실패를 모두 `Option`으로 바꿉니다.
- ④ 성공하면 값을 꺼내고, 실패하면 현재 함수에서 오류를 반환합니다.

### 8. 반복 중단

`[Ok(1), Err("실패"), Ok(3)]`을 순서대로 꺼내는 반복자에
`collect::<Result<Vec<_>, _>>()`를 호출하면 어떤 결과가 나오나요?

- ① `Ok(vec![1, 3])`
- ② `Err("실패")`
- ③ `Ok(vec![])`
- ④ `Ok(vec![1])`

### 9. transpose

`Some(Err::<u16, &str>("실패")).transpose()`의 결과는 무엇인가요?

- ① `Ok(Some(0))`
- ② `Ok(None)`
- ③ `Err("실패")`
- ④ `Some(Err("실패"))`

### 10. 없음과 실패 구분

`optional_port`는 `input.map(str::parse::<u16>).transpose()`의 결과를 반환합니다.
입력이 `None`일 때와 `Some("rust")`일 때의 결과를 순서대로 짝지은 것은 무엇인가요?

- ① `Ok(None)`과 `Err`
- ② `Err`와 `Err`
- ③ `None`과 `None`
- ④ `Ok(None)`과 `Ok(None)`

### 11. 프로그램 경계

오류를 처리할 때 라이브러리에서 할 일과 실행 프로그램에서 할 일을
어떻게 나누는 것이 좋을까요?

- ① 라이브러리에서 오류가 날 때마다 프로그램을 종료합니다.
- ② 라이브러리는 오류를 반환하고, `main`에서 메시지와 종료 코드를 결정합니다.
- ③ 라이브러리에서 오류를 모두 기본값으로 바꿉니다.
- ④ `main`은 오류를 확인하지 않고 항상 성공을 반환합니다.

### 12. 계산 범위 초과

결제 합계가 `u32` 범위를 벗어났을 때 오류를 반환하려면 어떤 방법이 알맞나요?

- ① `total + amount`
- ② `(total + amount).into()`
- ③ `total.wrapping_add(amount)`
- ④ `total.checked_add(amount).ok_or(PaymentError::Overflow)`

## 정답과 풀이

### 1. 정답: ②

`Option<User>`는 사용자 정보가 있는 경우와 없는 경우를 모두 나타낼 수 있습니다.
`Result`는 실패 이유를 구분할 때 어울립니다. `User`만으로는 정보가 없는 경우를
나타낼 수 없고, `bool`에는 사용자 정보를 담을 수 없습니다.

### 2. 정답: ④

`unwrap_or_else`는 `None`일 때만 함수를 호출합니다. `unwrap_or`와 `ok_or`에
넘기는 인자는 메서드를 호출하기 전에 계산합니다. `map`에 넘긴 함수는 `Some`일
때 실행하므로 기본값을 구하는 용도로는 맞지 않습니다.

### 3. 정답: ①

`and_then`은 안에 든 `String`을 함수에 넘기고, 함수가 반환한 `Option`을
그대로 돌려줍니다. ②와 ③은 결과를 `Option`으로 다시 감싸므로 중첩이 생깁니다.
④는 `&String`을 넘기지만 함수는 `String`을 받으므로 인자 타입이 맞지 않습니다.

### 4. 정답: ③

`as_ref`는 안의 값을 빌려서 읽을 수 있게 합니다. `unwrap`과 `into_iter`는
원래 값의 소유권을 가져갑니다. `take`는 안의 값을 꺼내고 그 자리에 `None`을
남기므로 원래 상태를 유지하지 않습니다.

### 5. 정답: ①

`?`는 현재 함수를 바로 끝내고 `None`을 반환합니다. 값이 없는 상태를 `Err`로
자동 변환하지 않으며, 뒤의 코드를 실행하거나 패닉을 일으키지도 않습니다.

### 6. 정답: ③

`map_err`는 오류만 바꿉니다. `map`은 성공한 값에 함수를 적용하고, `ok`는 오류
내용을 버립니다. `unwrap_or`는 오류가 있으면 기본값을 반환합니다.

### 7. 정답: ④

`?`는 성공한 값을 꺼내거나, 현재 함수에서 오류를 바로 반환합니다. 기본값을
정하거나 로그를 출력하거나 `Option`으로 바꾸는 코드는 따로 작성해야 합니다.

### 8. 정답: ②

첫 번째 오류를 만나면 값을 모으는 일을 멈추고 `Err`를 반환합니다. 실패한 항목을
건너뛰지 않으며, 앞서 모은 값만 `Ok`에 담아 반환하지도 않습니다.

### 9. 정답: ③

`transpose`는 `Option<Result<T, E>>`를 `Result<Option<T>, E>`로 바꿉니다.
`Some` 안에 있던 `Err`가 그대로 반환됩니다. 오류를 `None`이나 기본값으로
바꾸지는 않습니다.

### 10. 정답: ①

설정이 없으면 `Ok(None)`이고, 설정은 있지만 숫자로 바꿀 수 없으면 `Err`입니다.
반환형은 `Result`이므로 `None`만 반환하지 않습니다. 잘못된 설정을 설정이 없는
경우와 똑같이 처리하지도 않습니다.

### 11. 정답: ②

라이브러리가 오류를 반환하면 호출한 쪽에서 복구를 시도할지, 프로그램을 끝낼지
결정할 수 있습니다. 라이브러리에서 무조건 종료하거나, 오류를 기본값으로 바꾸거나,
항상 성공했다고 처리하면 호출한 쪽에서 상황에 맞게 판단할 수 없습니다.

### 12. 정답: ④

`checked_add`는 합이 범위를 벗어나면 `None`을 반환합니다. 이를 `ok_or`로
오류로 바꿉니다. 일반 덧셈은 빌드 설정에 따라 패닉이 발생하거나 값이 범위의
처음으로 돌아갈 수 있습니다. 덧셈을 마친 뒤 타입을 바꿔도 이를 막을 수 없습니다.
`wrapping_add`도 범위를 넘으면 값이 처음으로 돌아가므로 오류를 반환하지 않습니다.
