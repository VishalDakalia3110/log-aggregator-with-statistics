use std::env;
use serde_json;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

use log_aggregator::analyzer::Analyzer;
use log_aggregator::log_entry::LogEntry;

fn parse_file(path: &Path, entries: &mut Vec<LogEntry>) {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file {:?}: {}", path, e);
            return;
        }
    };

    let reader = io::BufReader::new(file);

    for (line_number, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!(
                    "Error reading line {} in {:?}: {}",
                    line_number + 1,
                    path,
                    e
                );
                continue;
            }
        };

        match LogEntry::parse(&line, path, line_number + 1) {
            Ok(entry) => entries.push(entry),
            Err(err) => {
                eprintln!(
                    "Parse error in {:?} at line {}: {}",
                    err.file, err.line_number, err.reason
                );
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cargo run <log_file | log_directory>");
        return;
    }

    let input_path = PathBuf::from(&args[1]);
    let mut entries = Vec::new();

    if input_path.is_file() {
        parse_file(&input_path, &mut entries);
    } else if input_path.is_dir() {
        let dir_entries = match fs::read_dir(&input_path) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Failed to read directory {:?}: {}", input_path, e);
                return;
            }
        };

        for entry in dir_entries {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            let path = entry.path();
            if path.is_file() {
                parse_file(&path, &mut entries);
            }
        }
    } else {
        eprintln!("Provided path is neither a file nor a directory");
        return;
    }

    if entries.is_empty() {
        println!("No valid log entries found.");
        return;
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
    println!("\n===== JSON Output =====");

    match serde_json::to_string_pretty(&stats) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Failed to serialize statistics to JSON: {}", e),
    }
}
