use std::path::PathBuf;
use crate::datetime::DateTime;
use crate::log_level::LogLevel;

#[derive(Debug)]
pub struct LogEntry {
    pub timestamp: DateTime,
    pub level: LogLevel,
    pub component: String,
    pub message: String,
    pub source_file: PathBuf,
}
