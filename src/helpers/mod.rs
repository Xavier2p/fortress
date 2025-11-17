//! Utility functions and structs.
use crate::crypto;
use crate::helpers::errors::FortressError;
use std::fs;
use structs::{GeneralArgs, PasswordEntry};

pub mod cli;
pub mod errors;
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
        Ok(_) => Ok(()),
        Err(e) => Err(FortressError::IoError(e)),
    }
}

/// Loads the vault from the file.
/// ## Parameters:
/// - `args`: The context of the program
/// ## Returns:
/// A result of a vector of [`PasswordEntry`] or a [`FortressError`]
pub fn load_vault(args: GeneralArgs) -> Result<Vec<PasswordEntry>, FortressError> {
    let encrypted = match fs::read(&args.file) {
        Ok(data) => data,
        Err(e) => return Err(FortressError::IoError(e)),
    };

    match crypto::decrypt_database(&encrypted, &args.password) {
        Ok(entries) => Ok(entries),
        Err(_) => Err(FortressError::DecryptionFailed),
    }
}

/// Prints a debug message if the verbose flag is set.
/// TO BE IMPLEMENTED
#[allow(dead_code)]
pub fn debug(args: &GeneralArgs, message: String) {
    if args.verbose {
        println!("[DEBUG]: {}", message);
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

    #[test]
    fn test_generate_password_length() {
        let pw = generate_password(16);
        assert_eq!(pw.len(), 16);
    }

    #[test]
    fn test_debug_output() {
        let args = GeneralArgs::new(true, "file".to_string(), "pw".to_string());
        debug(&args, "test message".to_string());
        // No assertion, just ensure no panic
    }
}
