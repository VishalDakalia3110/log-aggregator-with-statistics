use std::path::Path;
use log_aggregator::parser::parse_log_line;

#[test]
fn parse_valid_log_line() {
    let line = "2024-01-15 10:23:45 [INFO] app: Started successfully";
    let entry = parse_log_line(line, Path::new("test.log"), 1).unwrap();

    assert_eq!(entry.component, "app");
    assert_eq!(entry.message, "Started successfully");
}

#[test]
fn reject_invalid_log_line() {
    let line = "This is not a valid log line";
    let result = parse_log_line(line, Path::new("test.log"), 1);
    assert!(result.is_err());
}
