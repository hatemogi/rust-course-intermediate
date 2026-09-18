fn tuple_binding() {
    let (x, y) = (10, 20);
    assert_eq!(x + y, 30);
}

fn loop_binding() {
    let pairs = [("a", 1), ("b", 2)];
    let mut labels = String::new();
    for (name, _) in pairs {
        labels.push_str(name);
    }
    assert_eq!(labels, "ab");
}

#[allow(clippy::redundant_pattern_matching)]
fn underscore_ignored() {
    let kept = Some(String::from("Rust"));
    if let Some(_) = kept {
        assert!(kept.is_some());
    }
    assert_eq!(kept, Some(String::from("Rust")));
}

fn underscore_bound() {
    let moved = Some(String::from("Rust"));
    if let Some(_text) = moved {
        assert_eq!(_text, "Rust");
    }
    // moved는 내부 String이 이동되어 여기서 다시 사용할 수 없습니다.
}

fn main() {
    tuple_binding();
    loop_binding();
    underscore_ignored();
    underscore_bound();
}
