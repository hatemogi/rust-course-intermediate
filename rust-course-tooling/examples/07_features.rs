#[cfg(feature = "binary-search")]
fn contains(values: &[u64], target: u64) -> bool {
    values.binary_search(&target).is_ok()
}

#[cfg(not(feature = "binary-search"))]
fn contains(values: &[u64], target: u64) -> bool {
    values.contains(&target)
}

fn main() {
    let values = [10, 20, 30, 40];
    assert!(contains(&values, 30));
    assert!(!contains(&values, 35));

    if cfg!(feature = "binary-search") {
        println!("이진 검색 기능으로 찾았습니다.");
    } else {
        println!("기본 선형 검색으로 찾았습니다.");
    }
}

#[cfg(test)]
mod tests {
    use super::contains;

    #[test]
    fn finds_only_an_existing_value() {
        let values = [10, 20, 30, 40];
        assert!(contains(&values, 30));
        assert!(!contains(&values, 35));
    }
}
