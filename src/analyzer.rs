use crate::log_entry::LogEntry;
use crate::error::{ParseError, AnalyzerError};
use crate::statistics::Statistics;

pub struct LogAnalyzer {
    entries: Vec<LogEntry>,
    errors: Vec<ParseError>,
}

impl LogAnalyzer {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn entries(&self) -> &[LogEntry] {
        &self.entries
    }

    pub fn parse_errors(&self) -> &[ParseError] {
        &self.errors
    }

    pub fn statistics(&self) -> Statistics {
        Statistics::from_entries(&self.entries)
    }
}
