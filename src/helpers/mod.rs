//! Utility functions and structs.
use crate::crypto;
use crate::helpers::errors::FortressError;
use std::fs;
use std::path::Path;
use structs::{GeneralArgs, PasswordEntry};

pub mod cli;
pub mod errors;
pub mod logger;
pub mod structs;

/// Encrypts the vault and saves it to the file.
/// ## Parameters:
/// - `args`: The context of the program
/// - `entries`: The actual data
/// ## Returns:
/// A result of nothing or a [`FortressError`]
pub fn save_vault(args: GeneralArgs, entries: &[PasswordEntry]) -> Result<(), FortressError> {
    let encrypted = match crypto::encrypt_database(entries, &args.password) {
        Ok(vault) => vault,
        Err(_) => return Err(FortressError::EncryptionFailed),
    };

    match fs::write(&args.file, encrypted) {
        Ok(_) => {
            log::warn!("Vault Saved");
            Ok(())
        }
        Err(e) => Err(FortressError::IoError(e)),
    }
}

/// Loads the vault from the file.
/// ## Parameters:
/// - `args`: The context of the program
/// ## Returns:
/// A result of a vector of [`PasswordEntry`] or a [`FortressError`]
pub fn load_vault(args: GeneralArgs) -> Result<Vec<PasswordEntry>, FortressError> {
    if !Path::new(&args.file).exists() {
        return Err(FortressError::VaultNotFound);
    }

    let encrypted = match fs::read(&args.file) {
        Ok(data) => data,
        Err(e) => return Err(FortressError::IoError(e)),
    };

    match crypto::decrypt_database(&encrypted, &args.password) {
        Ok(entries) => {
            log::warn!("Vault Opened");
            Ok(entries)
        }
        Err(_) => Err(FortressError::DecryptionFailed),
    }
}

/// Generates a random password of the given length.
/// The password is copied to the clipboard.
/// ## Parameters:
/// - `length`: The length of the password
/// ## Returns:
/// A string containing the generated password.
pub fn generate_password(length: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~";
    let mut rng = rand::rng();

    let password: String = (0..length)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    match cli_clipboard::set_contents(password.to_string()) {
        Ok(_) => {
            println!("Your generated password is in your clipboard");
        }
        Err(_) => {
            println!("Error in setting clipboard, your password is: {}", password);
        }
    }
    password
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::structs::GeneralArgs;
    use crate::helpers::structs::PasswordEntry;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_generate_password_length() {
        let pw = generate_password(16);
        assert_eq!(pw.len(), 16);
    }

    fn tmp_path(name: &str) -> String {
        let mut p = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        p.push(format!("fortress_test_{}_{}.enc", name, nanos));
        p.to_str().unwrap().to_string()
    }

    fn cleanup(path: &str) {
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_save_and_load_vault_roundtrip() {
        let path = tmp_path("save_load");
        cleanup(&path);

        let args = GeneralArgs::new(path.clone(), "masterpw".to_string());

        let entries = vec![PasswordEntry {
            identifier: "id_rt".to_string(),
            username: "user_rt".to_string(),
            password: "pw_rt".to_string(),
        }];

        // Save
        let save_res = save_vault(args.clone(), &entries);
        assert!(save_res.is_ok());

        // Load
        let load_res = load_vault(args.clone());
        assert!(load_res.is_ok());
        let loaded = load_res.unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].identifier, "id_rt");
        assert_eq!(loaded[0].username, "user_rt");
        assert_eq!(loaded[0].password, "pw_rt");

        cleanup(&path);
    }
}
