use chrono::{DateTime, Datelike, FixedOffset, NaiveDate};

fn main() -> Result<(), chrono::ParseError> {
    // ANCHOR: string_to_date
    let date = NaiveDate::parse_from_str("2026-09-03", "%Y-%m-%d")?;

    assert_eq!(date.year(), 2026);
    assert_eq!(date.month(), 9);
    assert_eq!(date.day(), 3);
    // ANCHOR_END: string_to_date

    // ANCHOR: date_to_string
    let date = NaiveDate::from_ymd_opt(2026, 9, 3).expect("유효한 날짜");
    let korean_date = date.format("%Y년 %m월 %d일").to_string();

    assert_eq!(korean_date, "2026년 09월 03일");
    // ANCHOR_END: date_to_string

    // ANCHOR: date_time
    let source = "2026-09-03T14:30:00+09:00";
    let date_time: DateTime<FixedOffset> = DateTime::parse_from_rfc3339(source)?;

    assert_eq!(date_time.to_rfc3339(), source);
    // ANCHOR_END: date_time

    Ok(())
}
