use std::collections::HashMap;
use crate::log_level::LogLevel;
use crate::datetime::DateTime;
use crate::log_entry::LogEntry;


pub struct Statistics {
    pub total_entries: usize,
    pub entries_by_level: HashMap<LogLevel, usize>,
    pub entries_by_component: HashMap<String, usize>,
    pub entries_by_hour: HashMap<u8, usize>,
    pub error_count: usize,
    pub error_rate: f64,
    pub most_active_component: Option<String>,
    pub peak_hour: Option<u8>,
    pub first_entry: Option<DateTime>,
    pub last_entry: Option<DateTime>,
}

impl Statistics {
    pub fn from_entries(entries: &[LogEntry]) -> Self {
        Self {
            total_entries: entries.len(),
            entries_by_level: HashMap::new(),
            entries_by_component: HashMap::new(),
            entries_by_hour: HashMap::new(),
            error_count: 0,
            error_rate: 0.0,
            most_active_component: None,
            peak_hour: None,
            first_entry: None,
            last_entry: None,
        }
    }
}
