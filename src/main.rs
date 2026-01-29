use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use log_aggregator::analyzer::Analyzer;
use log_aggregator::log_entry::LogEntry;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cargo run <log_file>");
        return;
    }

    let log_path = Path::new(&args[1]);
    let file = match File::open(log_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return;
        }
    };

    let reader = io::BufReader::new(file);
    let mut entries = Vec::new();

    for (line_number, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Error reading line {}: {}", line_number + 1, e);
                continue;
            }
        };

        match LogEntry::parse(&line, log_path, line_number + 1) {
            Ok(entry) => entries.push(entry),
            Err(err) => {
                eprintln!(
                    "Parse error at line {}: {}",
                    err.line_number, err.reason
                );
            }
        }
    }

    let analyzer = Analyzer::new(entries);
    let stats = analyzer.statistics();

    println!("===== Log Statistics =====");
    println!("Total entries: {}", stats.total_entries);
    println!("Error count: {}", stats.error_count);
    println!("Error rate: {:.2}%", stats.error_rate * 100.0);

    println!("\nEntries by level:");
    for (level, count) in &stats.entries_by_level {
        println!("  {:?}: {}", level, count);
    }

    println!("\nEntries by component:");
    for (component, count) in &stats.entries_by_component {
        println!("  {}: {}", component, count);
    }

    if let Some(hour) = stats.peak_hour {
        println!("\nPeak hour: {}:00", hour);
    }

    if let Some(first) = stats.first_entry {
        println!("First log entry: {}", first);
    }

    if let Some(last) = stats.last_entry {
        println!("Last log entry: {}", last);
    }
}
