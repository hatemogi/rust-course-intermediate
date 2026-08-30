// ANCHOR: inline
mod temperature {
    pub fn celsius_to_fahrenheit(value: f64) -> f64 {
        value * 1.8 + 32.0
    }
}
// ANCHOR_END: inline

// ANCHOR: nested
mod report {
    pub mod text {
        pub fn render_title(title: &str) -> String {
            format!("== {title} ==")
        }
    }
}
// ANCHOR_END: nested

fn main() {
    assert_eq!(temperature::celsius_to_fahrenheit(0.0), 32.0);
    assert_eq!(report::text::render_title("작업 현황"), "== 작업 현황 ==");
}
