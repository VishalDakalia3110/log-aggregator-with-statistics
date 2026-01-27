use std::path::Path;
use std::str::FromStr;

use crate::datetime::DateTime;
use crate::log_level::LogLevel;
use crate::log_entry::LogEntry;
use crate::error::ParseError;

pub fn parse_log_line(
    line: &str,
    source_file: &Path,
    line_number: usize,
) -> Result<LogEntry, ParseError> {

    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Err(ParseError {
            file: source_file.to_path_buf(),
            line_number,
            content: line.to_string(),
            reason: "Empty line".to_string(),
        });
    }

    // Split timestamp and rest
    let mut parts = trimmed.splitn(3, ' ');
    let date = parts.next();
    let time = parts.next();
    let rest = parts.next();

    if date.is_none() || time.is_none() || rest.is_none() {
        return Err(ParseError {
            file: source_file.to_path_buf(),
            line_number,
            content: line.to_string(),
            reason: "Malformed log line".to_string(),
        });
    }

    let timestamp_str = format!("{} {}", date.unwrap(), time.unwrap());
    let timestamp = DateTime::from_str(&timestamp_str).map_err(|e| ParseError {
        file: source_file.to_path_buf(),
        line_number,
        content: line.to_string(),
        reason: e,
    })?;

    // [LEVEL] component: message
    let rest = rest.unwrap().trim();
    if !rest.starts_with('[') {
        return Err(ParseError {
            file: source_file.to_path_buf(),
            line_number,
            content: line.to_string(),
            reason: "Missing log level".to_string(),
        });
    }

    let closing = rest.find(']').ok_or_else(|| ParseError {
        file: source_file.to_path_buf(),
        line_number,
        content: line.to_string(),
        reason: "Unclosed log level bracket".to_string(),
    })?;

    let level_str = &rest[1..closing];
    let level = LogLevel::from_str(level_str).map_err(|e| ParseError {
        file: source_file.to_path_buf(),
        line_number,
        content: line.to_string(),
        reason: e,
    })?;

    let after_level = rest[closing + 1..].trim();
    let mut comp_msg = after_level.splitn(2, ':');

    let component = comp_msg
        .next()
        .ok_or_else(|| ParseError {
            file: source_file.to_path_buf(),
            line_number,
            content: line.to_string(),
            reason: "Missing component".to_string(),
        })?
        .trim()
        .to_string();

    let message = comp_msg
        .next()
        .ok_or_else(|| ParseError {
            file: source_file.to_path_buf(),
            line_number,
            content: line.to_string(),
            reason: "Missing message".to_string(),
        })?
        .trim()
        .to_string();

    Ok(LogEntry {
        timestamp,
        level,
        component,
        message,
        source_file: source_file.to_path_buf(),
    })
}
