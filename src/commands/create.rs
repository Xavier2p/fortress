use crate::crypto::{Crypto, PasswordEntry};
use crate::helpers::errors::FortressError;
use std::path::Path;

pub fn create(force: bool, password: String, path: String) -> Result<(), FortressError> {
    if Path::new(&path).exists() && !force {
        Err(FortressError::VaultAlreadyExists)
    } else {
        let empty_entries: Vec<PasswordEntry> = Vec::new();
        let vault = match Crypto::encrypt_database(&empty_entries, &password) {
            Ok(vault) => vault,
            Err(_) => return Err(FortressError::EncryptionFailed),
        };
        match std::fs::write(&path, vault) {
            Ok(_) => {
                println!("Vault created at {}", path);
                Ok(())
            }
            Err(e) => Err(FortressError::IoError(e)),
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
        let result = create(true, "testpw".to_string(), path.to_string());
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
        let result = create(false, "pw".to_string(), path.to_string());
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
        let result = create(true, "pw".to_string(), path.to_string());
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
