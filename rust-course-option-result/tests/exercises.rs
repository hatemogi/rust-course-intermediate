use rust_course_option_result::{
    ExpressionError, PaymentError, evaluate_division, first_email, optional_port, parse_all,
    parse_ports, parse_setting, percentage, total_payments,
};

#[test]
fn calculates_percentage_safely() {
    assert_eq!(percentage(3.0, 4.0), Some(75.0));
    assert_eq!(percentage(3.0, 0.0), None);
}

#[test]
fn finds_first_email() {
    assert_eq!(
        first_email(&["none", "a@b.com", "c@d.com"]),
        Some("a@b.com")
    );
    assert_eq!(first_email(&["none"]), None);
}

#[test]
fn parses_setting_value() {
    assert_eq!(parse_setting("PORT=8080"), Some(8080));
    assert_eq!(parse_setting("PORT=invalid"), None);
    assert_eq!(parse_setting("PORT"), None);
}

#[test]
fn parses_only_when_every_item_is_valid() {
    assert_eq!(parse_all(&["1", "2", "3"]), Some(vec![1, 2, 3]));
    assert_eq!(parse_all(&["1", "x"]), None);
}

#[test]
fn evaluates_division_expression() {
    assert_eq!(evaluate_division("12 / 3"), Ok(4));
    assert_eq!(evaluate_division("12"), Err(ExpressionError::InvalidFormat));
    assert_eq!(
        evaluate_division("rust/3"),
        Err(ExpressionError::InvalidNumber)
    );
    assert_eq!(
        evaluate_division("12/0"),
        Err(ExpressionError::DivisionByZero)
    );
}

#[test]
fn parses_every_port_or_returns_first_error() {
    assert_eq!(
        parse_ports("3000, 8080, 443").unwrap(),
        vec![3000, 8080, 443]
    );
    assert!(parse_ports("3000, rust").is_err());
}

#[test]
fn totals_only_valid_payments() {
    assert_eq!(total_payments(&[100, 200, 300]), Ok(600));
    assert_eq!(
        total_payments(&[100, 0, 300]),
        Err(PaymentError::ZeroAmount)
    );
}

#[test]
fn parses_an_optional_port() {
    assert_eq!(optional_port(None), Ok(None));
    assert_eq!(optional_port(Some("443")), Ok(Some(443)));
    assert!(optional_port(Some("rust")).is_err());
}

#[test]
fn reports_arithmetic_overflow() {
    assert_eq!(
        evaluate_division("-2147483648/-1"),
        Err(ExpressionError::Overflow)
    );
    assert_eq!(total_payments(&[u32::MAX, 1]), Err(PaymentError::Overflow));
    assert_eq!(total_payments(&[u32::MAX]), Ok(u32::MAX));
    assert_eq!(total_payments(&[]), Ok(0));
}
