use std::collections::BTreeMap;

use rust_course_tooling::{
    binary_contains, count_normalized_names, count_words, linear_contains, normalize_names,
};

#[test]
fn trims_and_lowercases_names() {
    assert_eq!(normalize_names(&[" Ferris ", " RUST "]), ["ferris", "rust"]);
}

#[test]
fn removes_names_that_are_empty_after_trimming() {
    assert_eq!(normalize_names(&["Ferris", "", "  "]), ["ferris"]);
}

#[test]
fn preserves_the_order_of_names() {
    assert_eq!(normalize_names(&["Zed", "Amy"]), ["zed", "amy"]);
}

#[test]
fn returns_an_empty_list_for_an_empty_input() {
    assert_eq!(normalize_names(&[]), Vec::<String>::new());
}

#[test]
// 종합 실습을 구현한 뒤 `#[ignore]`를 제거하세요.
#[ignore]
fn counts_normalized_names() {
    assert_eq!(
        count_normalized_names(&[" Ferris ", "ferris", "", " RUST "]),
        BTreeMap::from([("ferris".to_owned(), 2), ("rust".to_owned(), 1),])
    );
}

#[test]
// 종합 실습을 구현한 뒤 `#[ignore]`를 제거하세요.
#[ignore]
fn returns_no_name_counts_for_an_empty_input() {
    assert!(count_normalized_names(&[]).is_empty());
}

#[test]
fn counts_words_in_word_order() {
    assert_eq!(
        count_words("rust cargo rust"),
        BTreeMap::from([("cargo", 1), ("rust", 2)])
    );
}

#[test]
fn search_implementations_match_expected_results() {
    let values = [1, 4, 8, 15, 16, 23, 42];
    for (target, expected) in [(1, true), (15, true), (42, true), (100, false)] {
        assert_eq!(linear_contains(&values, target), expected);
        assert_eq!(binary_contains(&values, target), expected);
    }
}
