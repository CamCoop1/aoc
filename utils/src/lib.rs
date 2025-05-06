pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub mod file_reader;
pub mod harness;
