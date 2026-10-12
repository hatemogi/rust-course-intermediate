use rust_course_option_result::{
    evaluate_division, first_email, optional_port, parse_all, parse_ports, parse_setting,
    percentage, total_payments,
};

fn main() {
    assert_eq!(percentage(1.0, 4.0), Some(25.0));
    assert_eq!(percentage(1.0, 0.0), None);
    assert_eq!(
        first_email(&["guest", "rust@example.com"]),
        Some("rust@example.com")
    );
    assert_eq!(parse_setting("PORT=8080"), Some(8080));
    assert_eq!(parse_all(&["10", "20"]), Some(vec![10, 20]));
    assert_eq!(evaluate_division("12/3").unwrap(), 4);
    assert_eq!(parse_ports("3000, 8080").unwrap(), vec![3000, 8080]);
    assert_eq!(total_payments(&[1000, 2500, 500]).unwrap(), 4000);
    assert_eq!(optional_port(Some("443")).unwrap(), Some(443));
}
