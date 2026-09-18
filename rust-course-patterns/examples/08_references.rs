fn borrow() {
    let name = Some(String::from("서준"));
    let length = match &name {
        Some(value) => value.chars().count(),
        None => 0,
    };
    assert_eq!(length, 2);
    assert_eq!(name.as_deref(), Some("서준"));
}

fn match_with_ref() {
    let name = Some(String::from("서준"));
    let length = match name {
        Some(ref value) => value.chars().count(),
        None => 0,
    };
    assert_eq!(length, 2);
    assert_eq!(name.as_deref(), Some("서준"));
}

// 편의 문법 없이 참조 패턴과 바인딩을 모두 명시합니다.
#[allow(clippy::match_ref_pats, clippy::needless_borrowed_reference)]
fn explicit_match() {
    let name = Some(String::from("서준"));
    let length = match &name {
        &Some(ref value) => value.chars().count(),
        &None => 0,
    };
    assert_eq!(length, 2);
    assert_eq!(name.as_deref(), Some("서준"));
}

fn explicit_reference() {
    let number = 7;
    let &copied = &number;
    let _: i32 = copied;
    assert_eq!(copied, number);
}

fn mutable() {
    let mut message = Some(String::from("Rust"));
    if let Some(text) = &mut message {
        let _: &mut String = text;
        text.push('!');
    }
    assert_eq!(message.as_deref(), Some("Rust!"));
}

fn ref_mut() {
    let mut message = Some(String::from("Rust!"));
    if let Some(ref mut text) = message {
        text.push('?');
    }
    assert_eq!(message.as_deref(), Some("Rust!?"));
}

fn copy_value() {
    let number = Some(7);
    if let Some(value) = number {
        assert_eq!(value, 7);
    }
    assert_eq!(number, Some(7));
}

fn move_value() {
    let message = Some(String::from("Rust"));
    if let Some(text) = message {
        assert_eq!(text, "Rust");
    }
    // String이 이동했으므로 message 전체를 다시 사용할 수 없습니다.
}

fn main() {
    borrow();
    match_with_ref();
    explicit_match();
    explicit_reference();
    mutable();
    ref_mut();
    copy_value();
    move_value();
}
