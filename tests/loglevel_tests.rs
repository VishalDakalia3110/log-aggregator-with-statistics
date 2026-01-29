use std::str::FromStr;
use log_aggregator::log_level::LogLevel;

#[test]
fn parse_log_levels() {
    assert_eq!(LogLevel::from_str("info").unwrap(), LogLevel::Info);
    assert_eq!(LogLevel::from_str("ERROR").unwrap(), LogLevel::Error);
}

#[test]
fn log_level_ordering() {
    assert!(LogLevel::Info < LogLevel::Error);
    assert!(LogLevel::Trace < LogLevel::Fatal);
}
