use std::str::FromStr;
use log_aggregator::datetime::DateTime;

#[test]
fn parse_valid_datetime() {
    let dt = DateTime::from_str("2024-01-15 10:23:45").unwrap();
    assert_eq!(dt.year, 2024);
    assert_eq!(dt.month, 1);
    assert_eq!(dt.day, 15);
    assert_eq!(dt.hour, 10);
    assert_eq!(dt.minute, 23);
    assert_eq!(dt.second, 45);
}

#[test]
fn reject_invalid_month() {
    assert!(DateTime::from_str("2024-13-01 10:00:00").is_err());
}

#[test]
fn reject_invalid_hour() {
    assert!(DateTime::from_str("2024-01-01 24:00:00").is_err());
}

#[test]
fn compare_datetimes() {
    let a = DateTime::from_str("2024-01-01 09:00:00").unwrap();
    let b = DateTime::from_str("2024-01-01 10:00:00").unwrap();
    assert!(a < b);
}
