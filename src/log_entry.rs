use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::datetime::DateTime;
use crate::log_level::LogLevel;
use crate::error::ParseError;

#[derive(Debug)]
pub struct LogEntry {
    pub timestamp: DateTime,
    pub level: LogLevel,
    pub component: String,
    pub message: String,
    pub source_file: PathBuf,
}

impl LogEntry {
    pub fn parse(
        line: &str,
        source_file: &Path,
        line_number: usize,
    ) -> Result<Self, ParseError> {
        // Expected format:
        // YYYY-MM-DD HH:MM:SS LEVEL component: message

        let mut parts = line.splitn(3, ' ');

        let date = parts.next().ok_or_else(|| ParseError {
            file: source_file.to_path_buf(),
            line_number,
            content: line.to_string(),
            reason: "Missing date".to_string(),
        })?;

        let time = parts.next().ok_or_else(|| ParseError {
            file: source_file.to_path_buf(),
            line_number,
            content: line.to_string(),
            reason: "Missing time".to_string(),
        })?;

        let rest = parts.next().ok_or_else(|| ParseError {
            file: source_file.to_path_buf(),
            line_number,
            content: line.to_string(),
            reason: "Missing log level and message".to_string(),
        })?;

        let timestamp =
            DateTime::from_str(&format!("{} {}", date, time)).map_err(|e| ParseError {
                file: source_file.to_path_buf(),
                line_number,
                content: line.to_string(),
                reason: e,
            })?;

        let mut rest_parts = rest.splitn(2, ' ');

        let level_str = rest_parts.next().ok_or_else(|| ParseError {
            file: source_file.to_path_buf(),
            line_number,
            content: line.to_string(),
            reason: "Missing log level".to_string(),
        })?;

        // Accept formats like [INFO], [ERROR], etc.
        let cleaned_level = level_str.trim_matches(&['[', ']'][..]);

        let level = LogLevel::from_str(cleaned_level).map_err(|e| ParseError {
            file: source_file.to_path_buf(),
            line_number,
            content: line.to_string(),
            reason: e,
        })?;


        let after_level = rest_parts.next().ok_or_else(|| ParseError {
            file: source_file.to_path_buf(),
            line_number,
            content: line.to_string(),
            reason: "Missing component and message".to_string(),
        })?;

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

        Ok(Self {
            timestamp,
            level,
            component,
            message,
            source_file: source_file.to_path_buf(),
        })
    }
}
