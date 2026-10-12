use std::num::ParseIntError;

fn stop_at_error() {
    let mut visited = Vec::new();
    let ports: Result<Vec<u16>, ParseIntError> = ["3000", "rust", "443"]
        .into_iter()
        .map(|text| {
            visited.push(text);
            text.parse()
        })
        .collect();
    assert!(ports.is_err());
    assert_eq!(visited, vec!["3000", "rust"]);
}

fn collect_results() {
    let results: Vec<Result<u16, ParseIntError>> = ["3000", "rust", "443", "invalid"]
        .into_iter()
        .map(str::parse)
        .collect();
    assert_eq!(results.len(), 4);
    assert_eq!(results[0], Ok(3000));
    assert!(results[1].is_err());
    assert_eq!(results[2], Ok(443));
    assert!(results[3].is_err());
}

fn separate_results() {
    let mut ports = Vec::new();
    let mut errors = Vec::new();
    for text in ["3000", "rust", "443", "invalid"] {
        match text.parse::<u16>() {
            Ok(port) => ports.push(port),
            Err(error) => errors.push((text, error)),
        }
    }
    assert_eq!(ports, vec![3000, 443]);
    assert_eq!(errors.len(), 2);
}

fn collect_successes() {
    let ports: Result<Vec<u16>, ParseIntError> = ["3000", "8080", "443"]
        .into_iter()
        .map(str::parse)
        .collect();
    assert_eq!(ports, Ok(vec![3000, 8080, 443]));
}

fn main() {
    collect_successes();
    stop_at_error();
    collect_results();
    separate_results();
}
