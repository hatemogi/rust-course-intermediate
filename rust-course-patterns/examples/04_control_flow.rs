fn double_number_or_zero(value: Option<i32>) -> i32 {
    let Some(number) = value else {
        return 0;
    };
    number * 2
}

fn if_let_pattern() {
    let port = Some(8080);
    let mut output = String::new();
    if let Some(value) = port {
        output = format!("포트: {value}");
    }
    assert_eq!(output, "포트: 8080");
}

fn while_let_pattern() {
    let mut stack = vec![1, 2, 3];
    let mut reversed = Vec::new();
    while let Some(value) = stack.pop() {
        reversed.push(value);
    }
    assert_eq!(reversed, vec![3, 2, 1]);
}

fn main() {
    if_let_pattern();
    while_let_pattern();

    assert_eq!(double_number_or_zero(Some(4)), 8);
    assert_eq!(double_number_or_zero(None), 0);
}
