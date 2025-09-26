use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone)]
pub struct PasswordEntry {
    pub identifier: String,
    pub username: String,
    pub password: String,
}

impl fmt::Display for PasswordEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[ENTRY]: '{}': '{}' - '{}'",
            self.identifier,
            self.username,
            "*".repeat(self.password.len())
        )
    }
}

#[derive(Clone)]
pub struct GeneralArgs {
    pub verbose: bool,
    pub file: String,
    pub password: String,
}

impl GeneralArgs {
    pub fn new(verbose: bool, file: String, password: String) -> Self {
        GeneralArgs {
            verbose,
            file,
            password,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_entry_display() {
        let entry = PasswordEntry {
            identifier: "id".to_string(),
            username: "user".to_string(),
            password: "secret".to_string(),
        };
        let display = format!("{}", entry);
        assert!(display.contains("[ENTRY]:"));
        assert!(display.contains("id"));
        assert!(display.contains("user"));
        assert!(display.contains("******"));
    }

    #[test]
    fn test_general_args_new() {
        let args = GeneralArgs::new(true, "file".to_string(), "pw".to_string());
        assert!(args.verbose);
        assert_eq!(args.file, "file");
        assert_eq!(args.password, "pw");
    }
}
