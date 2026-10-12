fn optional_port(input: Option<&str>) -> Result<Option<u16>, std::num::ParseIntError> {
    input.map(str::parse).transpose()
}

fn keep_configured_ports() {
    let results: Vec<_> = [Some("443"), None, Some("rust"), Some("8080")]
        .into_iter()
        .map(optional_port)
        .filter_map(Result::transpose)
        .collect();

    assert_eq!(results.len(), 3);
    assert_eq!(results[0], Ok(443));
    assert!(results[1].is_err());
    assert_eq!(results[2], Ok(8080));
}

fn main() {
    keep_configured_ports();
    assert!(optional_port(Some("rust")).is_err());
    assert_eq!(optional_port(None), Ok(None));
    assert_eq!(optional_port(Some("443")), Ok(Some(443)));
}
