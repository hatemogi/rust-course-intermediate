fn command(args: &[&str]) -> String {
    match args {
        ["add", name] => format!("{name} 추가"),
        ["remove", name] => format!("{name} 제거"),
        ["list"] => String::from("목록"),
        _ => String::from("알 수 없는 명령"),
    }
}

fn split_edges(values: &[i32]) -> Option<(&i32, &[i32], &i32)> {
    match values {
        [first, middle @ .., last] => Some((first, middle, last)),
        _ => None,
    }
}

fn rest_positions() {
    let values: &[i32] = &[1, 2, 3, 4];
    let first_and_rest = match values {
        [first, rest @ ..] => Some((first, rest)),
        _ => None,
    };
    let rest_and_last = match values {
        [rest @ .., last] => Some((rest, last)),
        _ => None,
    };
    let edges = match values {
        [first, middle @ .., last] => Some((first, middle, last)),
        _ => None,
    };
    assert_eq!(first_and_rest, Some((&1, &[2, 3, 4][..])));
    assert_eq!(rest_and_last, Some((&[1, 2, 3][..], &4)));
    assert_eq!(edges, Some((&1, &[2, 3][..], &4)));
}

fn lengths() {
    assert_eq!(split_edges(&[]), None);
    assert_eq!(split_edges(&[1]), None);
    assert_eq!(split_edges(&[1, 2]), Some((&1, &[][..], &2)));
    let values = [1, 2, 3, 4];
    assert_eq!(split_edges(&values), Some((&1, &[2, 3][..], &4)));
}

fn array() {
    let values = [1, 2, 3, 4];
    let [first, middle @ .., last] = &values;
    let _: &[i32; 2] = middle;
    assert_eq!((*first, *middle, *last), (1, [2, 3], 4));
}

fn main() {
    rest_positions();
    lengths();
    array();

    assert_eq!(command(&["add", "Rust"]), "Rust 추가");
    assert_eq!(command(&["remove", "Rust"]), "Rust 제거");
    assert_eq!(command(&["list"]), "목록");
    assert_eq!(command(&[]), "알 수 없는 명령");
    assert_eq!(command(&["list", "extra"]), "알 수 없는 명령");
}
