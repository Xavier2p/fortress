//! Some structs used throughout the program.
use serde::{Deserialize, Serialize};
use std::fmt;

/// A single entry in the vault.
#[derive(Serialize, Deserialize, Clone)]
pub struct PasswordEntry {
    /// The identifier for the entry. Can be see as the path to the entry.
    pub identifier: String,
    /// The username for the entry.
    pub username: String,
    /// The password for the entry.
    pub password: String,
}

/// Display the entry in a readable format.
impl fmt::Display for PasswordEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}): '*****'", self.identifier, self.username)
    }
}

/// The context of the program.
#[derive(Clone)]
pub struct GeneralArgs {
    /// Enable verbose output
    #[allow(dead_code)]
    pub verbose: bool,
    /// The input file path
    pub file: String,
    /// The master password
    pub password: String,
}

/// Function to use the program context.
impl GeneralArgs {
    /// Create a new context.
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
        assert!(display.contains("id"));
        assert!(display.contains("user"));
    }

    #[test]
    fn test_general_args_new() {
        let args = GeneralArgs::new(true, "file".to_string(), "pw".to_string());
        assert!(args.verbose);
        assert_eq!(args.file, "file");
        assert_eq!(args.password, "pw");
    }
}
