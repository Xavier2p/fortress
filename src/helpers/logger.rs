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

#[cfg(test)]
mod tests {
    use super::*;
    use log::info;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn tmp_path(name: &str) -> String {
        let mut p = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        p.push(format!("fortress_test_log_{}_{}.log", name, nanos));
        p.to_str().unwrap().to_string()
    }

    #[test]
    fn test_init_stdout() {
        init(None).unwrap();
        info!("test_init_stdout ok");
    }

    #[test]
    fn test_init_file() {
        let path = tmp_path("file");
        let _ = fs::remove_file(&path);
        init(Some(&path)).unwrap();
        info!("hello file");
        std::thread::sleep(std::time::Duration::from_millis(50));
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.is_empty());
        let _ = fs::remove_file(&path);
    }
}
