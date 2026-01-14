//! Minimal logging initialization using `simplelog`.
//!
//! - If `log_file` is Some(path), logs are appended to that file.
//! - Otherwise logs go to stdout via `SimpleLogger`.

use log::LevelFilter;
use simplelog::{ConfigBuilder, SimpleLogger, WriteLogger};
use std::error::Error;
use std::fs::OpenOptions;

/// Initialize logging (file or stdout).
/// - `log_file`: optional path to write logs (append mode)
pub fn init(log_file: Option<&str>) -> Result<(), Box<dyn Error>> {
    let level: LevelFilter = LevelFilter::Info;
    let config = ConfigBuilder::new().set_time_format_rfc2822().build();

    if let Some(path) = log_file {
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        WriteLogger::init(level, config, file)?;
    } else {
        SimpleLogger::init(level, config)?;
    }

    Ok(())
}
