use std::collections::HashMap;
use crate::log_entry::LogEntry;
use crate::log_level::LogLevel;
use crate::datetime::DateTime;

#[derive(Debug)]
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
        let mut entries_by_level = HashMap::new();
        let mut entries_by_component = HashMap::new();
        let mut entries_by_hour = HashMap::new();

        let mut error_count = 0;
        let mut first_entry: Option<DateTime> = None;
        let mut last_entry: Option<DateTime> = None;


        for entry in entries {
            // Count by log level
            *entries_by_level.entry(entry.level).or_insert(0) += 1;

            // Count by component
            *entries_by_component
                .entry(entry.component.clone())
                .or_insert(0) += 1;

            // Count by hour
            *entries_by_hour
                .entry(entry.timestamp.hour)
                .or_insert(0) += 1;

            // Error count
            if entry.level == LogLevel::Error {
                error_count += 1;
            }

            // First & last entry
            first_entry = Some(match first_entry {
                Some(existing) => existing.min(entry.timestamp),
                None => entry.timestamp,
            });

            last_entry = Some(match last_entry {
                Some(existing) => existing.max(entry.timestamp),
                None => entry.timestamp,
            });
        }

        let total_entries = entries.len();
        let error_rate = if total_entries > 0 {
            error_count as f64 / total_entries as f64
        } else {
            0.0
        };

        let most_active_component = entries_by_component
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(comp, _)| comp.clone());

        let peak_hour = entries_by_hour
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(hour, _)| *hour);

        Self {
            total_entries,
            entries_by_level,
            entries_by_component,
            entries_by_hour,
            error_count,
            error_rate,
            most_active_component,
            peak_hour,
            first_entry,
            last_entry,
        }
    }
}
