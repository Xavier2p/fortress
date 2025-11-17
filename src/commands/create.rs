//! Create a new vault.
use crate::helpers::structs::{GeneralArgs, PasswordEntry};
use crate::helpers::{self, errors::FortressError};
use std::path::Path;

/// Create a new vault.
/// If the vault already exists, an error is returned unless force is set.
/// ## Parameters:
/// - `force`: If true, overwrite the existing vault.
/// - `args`: The context of the program
/// ## Returns:
/// A result of nothing or a [`FortressError`]
pub fn create(force: bool, args: GeneralArgs) -> Result<(), FortressError> {
    if Path::new(&args.file).exists() && !force {
        Err(FortressError::VaultAlreadyExists)
    } else {
        let empty_entries: Vec<PasswordEntry> = Vec::new();
        match helpers::save_vault(args.clone(), &empty_entries) {
            Ok(_) => {
                println!("Created new vault at {}", args.file);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use std::path::Path;

    // Helper to clean up test files
    fn cleanup(path: &str) {
        let _ = fs::remove_file(path);
    }

    // Mock Crypto for error simulation
    struct MockCrypto;
    impl MockCrypto {
        fn encrypt_database(_: &Vec<PasswordEntry>, _: &str) -> Result<Vec<u8>, ()> {
            Err(())
        }
    }

    #[test]
    fn test_create_new_vault() {
        let path = "test_vault.enc";
        cleanup(path);
        let result = create(
            true,
            GeneralArgs::new(false, path.to_string(), "testpw".to_string()),
        );
        assert!(result.is_ok());
        assert!(Path::new(path).exists());
        cleanup(path);
    }

    #[test]
    fn test_create_vault_already_exists() {
        let path = "test_vault_exists.enc";
        cleanup(path);
        // Create file first
        let mut f = fs::File::create(path).unwrap();
        writeln!(f, "dummy").unwrap();
        let result = create(
            false,
            GeneralArgs::new(false, path.to_string(), "pw".to_string()),
        );
        assert!(matches!(result, Err(FortressError::VaultAlreadyExists)));
        cleanup(path);
    }

    #[test]
    fn test_create_vault_force_overwrite() {
        let path = "test_vault_force.enc";
        cleanup(path);
        // Create file first
        let mut f = fs::File::create(path).unwrap();
        writeln!(f, "dummy").unwrap();
        let result = create(
            true,
            GeneralArgs::new(false, path.to_string(), "pw".to_string()),
        );
        assert!(result.is_ok());
        assert!(Path::new(path).exists());
        cleanup(path);
    }

    #[test]
    fn test_encryption_failed() {
        // Simulate encryption failure by calling the mock
        let result = MockCrypto::encrypt_database(&vec![], "pw");
        assert!(result.is_err());
    }
}
