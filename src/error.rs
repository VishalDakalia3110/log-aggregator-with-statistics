use std::path::PathBuf;

#[derive(Debug)]
pub enum AnalyzerError {
    IoError {
        path: PathBuf,
        source: std::io::Error,
    },
    NoFilesFound {
        path: PathBuf,
    },
    InvalidPath {
        path: PathBuf,
        reason: String,
    },
}

#[derive(Debug)]
pub struct ParseError {
    pub file: PathBuf,
    pub line_number: usize,
    pub content: String,
    pub reason: String,
}
