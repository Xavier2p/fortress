use aes_gcm::{
    aead::{Aead, KeyInit}, Aes256Gcm, Key,
    Nonce,
};
use argon2::{Argon2, Params};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PasswordEntry {
    pub identifier: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DatabaseWrapper {
    _pwcheck: String,
    entries: Vec<PasswordEntry>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum CryptoError {
    InvalidPassword,
    CorruptedData,
    FileNotFound,
    IoError(io::Error),
    SerializationError(serde_json::Error),
    EncryptionError,
    DecryptionError,
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoError::InvalidPassword => write!(f, "Invalid password"),
            CryptoError::CorruptedData => write!(f, "Database file is corrupted or tampered"),
            CryptoError::FileNotFound => write!(f, "Database file not found"),
            CryptoError::IoError(e) => write!(f, "IO error: {}", e),
            CryptoError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            CryptoError::EncryptionError => write!(f, "Encryption failed"),
            CryptoError::DecryptionError => write!(f, "Decryption failed"),
        }
    }
}

impl std::error::Error for CryptoError {}

impl From<io::Error> for CryptoError {
    fn from(error: io::Error) -> Self {
        if error.kind() == io::ErrorKind::NotFound {
            CryptoError::FileNotFound
        } else {
            CryptoError::IoError(error)
        }
    }
}

impl From<serde_json::Error> for CryptoError {
    fn from(error: serde_json::Error) -> Self {
        CryptoError::SerializationError(error)
    }
}

pub struct PasswordManager;

impl PasswordManager {
    /// Derive a 256-bit key from password using Argon2id
    fn derive_key(password: &str, salt: &[u8; 32]) -> Result<[u8; 32], CryptoError> {
        let params =
            Params::new(65536, 3, 4, Some(32)).map_err(|_| CryptoError::EncryptionError)?;

        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);

        let mut key = [0u8; 32];
        argon2
            .hash_password_into(password.as_bytes(), salt, &mut key)
            .map_err(|_| CryptoError::EncryptionError)?;

        Ok(key)
    }

    /// Encrypt the password database
    #[allow(deprecated)]
    pub fn encrypt_database(
        entries: &[PasswordEntry],
        master_password: &str,
    ) -> Result<Vec<u8>, CryptoError> {
        // Create wrapper with password check
        let wrapper = DatabaseWrapper {
            _pwcheck: "valid".to_string(),
            entries: entries.to_vec(),
        };

        // Serialize to JSON
        let json_data = serde_json::to_string(&wrapper)?;
        let plaintext = json_data.as_bytes();

        // Generate random salt
        let mut salt_32 = [0u8; 32];
        rand::rng().fill_bytes(&mut salt_32);

        // Derive key from password
        let key_bytes = Self::derive_key(master_password, &salt_32)?;
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);

        // Create cipher and generate nonce
        let cipher = Aes256Gcm::new(key);
        let mut nonce_bytes = [0u8; 12];
        rand::rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt
        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|_| CryptoError::EncryptionError)?;

        // Build final format: [Salt: 32 bytes][Nonce: 12 bytes][Encrypted Data + Auth Tag]
        let mut result = Vec::with_capacity(32 + 12 + ciphertext.len());
        result.extend_from_slice(&salt_32);
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }

    /// Decrypt the password database
    #[allow(deprecated)]
    pub fn decrypt_database(
        encrypted_data: &[u8],
        master_password: &str,
    ) -> Result<Vec<PasswordEntry>, CryptoError> {
        // Check minimum file size (32 + 12 + 16 = 60 bytes minimum)
        if encrypted_data.len() < 60 {
            return Err(CryptoError::CorruptedData);
        }

        // Extract components
        let salt: [u8; 32] = encrypted_data[0..32]
            .try_into()
            .map_err(|_| CryptoError::CorruptedData)?;
        let nonce_bytes: [u8; 12] = encrypted_data[32..44]
            .try_into()
            .map_err(|_| CryptoError::CorruptedData)?;
        let ciphertext = &encrypted_data[44..];

        // Derive key from password
        let key_bytes = Self::derive_key(master_password, &salt)?;
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);

        // Create cipher and nonce
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Decrypt
        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| CryptoError::InvalidPassword)?;

        // Parse JSON
        let json_str = std::str::from_utf8(&plaintext).map_err(|_| CryptoError::CorruptedData)?;

        let wrapper: DatabaseWrapper = serde_json::from_str(json_str)?;

        // Verify password check
        if wrapper._pwcheck != "valid" {
            return Err(CryptoError::InvalidPassword);
        }

        Ok(wrapper.entries)
    }

    /// Load encrypted database from file
    pub fn load_database(
        file_path: &str,
        master_password: &str,
    ) -> Result<Vec<PasswordEntry>, CryptoError> {
        let encrypted_data = fs::read(file_path)?;
        Self::decrypt_database(&encrypted_data, master_password)
    }

    /// Save encrypted database to file
    pub fn save_database(
        file_path: &str,
        entries: &[PasswordEntry],
        master_password: &str,
    ) -> Result<(), CryptoError> {
        let encrypted_data = Self::encrypt_database(entries, master_password)?;
        fs::write(file_path, encrypted_data)?;
        Ok(())
    }

    /// Prompt for password securely (without echo)
    pub fn prompt_password(prompt: &str) -> Result<String, CryptoError> {
        print!("{}", prompt);
        io::stdout().flush().map_err(CryptoError::IoError)?;

        // Note: In a real implementation, you'd want to use a crate like `rpassword`
        // to hide password input. For simplicity, this reads normally.
        let mut password = String::new();
        io::stdin()
            .read_line(&mut password)
            .map_err(CryptoError::IoError)?;

        // Remove trailing newline
        if password.ends_with('\n') {
            password.pop();
            if password.ends_with('\r') {
                password.pop();
            }
        }

        Ok(password)
    }
}

// Example usage
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_cycle() {
        let entries = vec![
            PasswordEntry {
                identifier: "Gmail".to_string(),
                username: "user@gmail.com".to_string(),
                password: "super_secret_123".to_string(),
            },
            PasswordEntry {
                identifier: "GitHub".to_string(),
                username: "developer".to_string(),
                password: "github_token_456".to_string(),
            },
        ];

        let master_password = "my_master_password";

        // Encrypt
        let encrypted = PasswordManager::encrypt_database(&entries, master_password)
            .expect("Encryption should succeed");

        // Decrypt
        let decrypted = PasswordManager::decrypt_database(&encrypted, master_password)
            .expect("Decryption should succeed");

        assert_eq!(entries.len(), decrypted.len());
        assert_eq!(entries[0].identifier, decrypted[0].identifier);
        assert_eq!(entries[0].username, decrypted[0].username);
        assert_eq!(entries[0].password, decrypted[0].password);
    }

    #[test]
    fn test_wrong_password() {
        let entries = vec![PasswordEntry {
            identifier: "Test".to_string(),
            username: "test".to_string(),
            password: "test123".to_string(),
        }];

        let encrypted = PasswordManager::encrypt_database(&entries, "correct_password")
            .expect("Encryption should succeed");

        let result = PasswordManager::decrypt_database(&encrypted, "wrong_password");
        assert!(matches!(result, Err(CryptoError::InvalidPassword)));
    }
}
