use crate::log_entry::LogEntry;
use crate::statistics::Statistics;

pub struct Analyzer {
    entries: Vec<LogEntry>,
}

impl Analyzer {
    pub fn new(entries: Vec<LogEntry>) -> Self {
        Self { entries }
    }

    pub fn statistics(&self) -> Statistics {
        Statistics::from_entries(&self.entries)
    }
}
