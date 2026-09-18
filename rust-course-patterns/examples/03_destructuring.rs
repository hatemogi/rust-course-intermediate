struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Move { x: i32, y: i32 },
    Write(String),
}

fn struct_pattern() {
    let point = Point { x: 3, y: 0 };
    let Point { x, y: 0 } = point else {
        panic!("x축 위의 점이어야 합니다");
    };
    assert_eq!(x, 3);
}

fn remaining_fields() {
    #[allow(dead_code)]
    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }

    let point = Point3D { x: 7, y: 8, z: 9 };
    let Point3D { x, .. } = point;
    assert_eq!(x, 7);
}

fn enum_pattern() {
    let message = Message::Move { x: 5, y: -2 };
    let description = match message {
        Message::Move { x, y } => format!("({x}, {y})로 이동"),
        Message::Write(text) => format!("쓰기: {text}"),
    };
    assert_eq!(description, "(5, -2)로 이동");
    assert!(matches!(
        Message::Write(String::from("Rust")),
        Message::Write(_)
    ));
}

fn main() {
    struct_pattern();
    remaining_fields();
    enum_pattern();
}
