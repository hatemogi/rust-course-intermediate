fn search() {
    let numbers = [3, 7, 10, 12];
    assert_eq!(numbers.iter().find(|&&n| n % 2 == 0), Some(&10));
    assert_eq!(numbers.iter().position(|n| *n > 10), Some(3));
}

fn collect() {
    let parsed: Option<Vec<i32>> = ["10", "20", "30"]
        .into_iter()
        .map(|text| text.parse().ok())
        .collect();
    assert_eq!(parsed, Some(vec![10, 20, 30]));
}

fn stop_at_none() {
    let mut visited = Vec::new();
    let values: Option<Vec<_>> = [Some(10), None, Some(30)]
        .into_iter()
        .inspect(|value| visited.push(*value))
        .collect();
    assert_eq!(values, None);
    assert_eq!(visited, vec![Some(10), None]);
}

fn main() {
    stop_at_none();
    search();
    collect();
}
