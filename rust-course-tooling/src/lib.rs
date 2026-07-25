//! Rust 개발 도구 강의의 종합 실습에서 사용하는 함수입니다.
//!
//! 다음 문서 테스트는 이동한 값을 다시 사용할 수 없음을 확인합니다.
//!
//! ```compile_fail
//! let message = String::from("완료");
//! drop(message);
//! println!("{message}");
//! ```

use std::collections::BTreeMap;

fn normalized_name(name: &str) -> Option<String> {
    let name = name.trim();
    if name.is_empty() {
        None
    } else {
        Some(name.to_lowercase())
    }
}

/// 이름 앞뒤의 공백을 없애고 빈 이름을 제외한 뒤 소문자로 바꿉니다.
///
/// # Examples
///
/// ```
/// use rust_course_tooling::normalize_names;
///
/// let names = normalize_names(&[" Ferris ", " RUST "]);
/// assert_eq!(names, ["ferris", "rust"]);
/// ```
pub fn normalize_names(names: &[&str]) -> Vec<String> {
    names
        .iter()
        .filter_map(|name| normalized_name(name))
        .collect()
}

/// 공백으로 나눈 단어의 등장 횟수를 대소문자를 구분해 단어순으로 반환합니다.
///
/// # Examples
///
/// ```
/// use rust_course_tooling::count_words;
///
/// let counts = count_words("rust cargo rust");
/// assert_eq!(counts["rust"], 2);
/// assert_eq!(counts["cargo"], 1);
/// ```
pub fn count_words(text: &str) -> BTreeMap<&str, usize> {
    let mut counts = BTreeMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_default() += 1;
    }
    counts
}

/// 정렬 여부와 관계없이 사용할 수 있는 선형 검색입니다.
pub fn linear_contains(values: &[u64], target: u64) -> bool {
    values.contains(&target)
}

/// 정렬된 입력에서 이진 검색을 사용합니다.
pub fn binary_contains(sorted_values: &[u64], target: u64) -> bool {
    sorted_values.binary_search(&target).is_ok()
}

#[cfg(test)]
mod tests {
    use super::normalized_name;

    #[test]
    fn normalizes_a_non_empty_name() {
        assert_eq!(normalized_name(" Ferris "), Some("ferris".to_owned()));
    }

    #[test]
    fn rejects_a_name_containing_only_whitespace() {
        assert_eq!(normalized_name(" \t "), None);
    }
}
