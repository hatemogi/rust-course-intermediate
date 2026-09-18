fn relation(point: (i32, i32)) -> &'static str {
    match point {
        (x, y) if x == y => "대각선",
        (_, 0) => "x축",
        (0, _) => "y축",
        _ => "일반",
    }
}

fn digit(value: i32) -> String {
    match value {
        number @ 0..=9 => format!("한 자리 숫자 {number}"),
        _ => String::from("한 자리 숫자가 아님"),
    }
}

enum Message {
    Move { x: i32, y: i32 },
    Write(String),
}

fn describe(message: Message) -> String {
    match message {
        Message::Move {
            x: x @ 1..=9,
            y: y @ (0 | 1),
        } if x % 2 == 1 => format!("홀수 위치: ({x}, {y})"),
        Message::Move { x, y } => format!("일반 이동: ({x}, {y})"),
        Message::Write(text) => format!("쓰기: {text}"),
    }
}

fn main() {
    assert_eq!(relation((3, 3)), "대각선");
    assert_eq!(digit(7), "한 자리 숫자 7");
    assert_eq!(describe(Message::Move { x: 3, y: 1 }), "홀수 위치: (3, 1)");
    assert_eq!(describe(Message::Write(String::from("Rust"))), "쓰기: Rust");
}
