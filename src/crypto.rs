use crate::helpers::errors::FortressError;
use crate::helpers::structs::PasswordEntry;
use aes_gcm::{
    aead::{Aead, KeyInit}, Aes256Gcm, Key,
    Nonce,
};
use argon2::{Argon2, Params};
use rand::RngCore;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct DatabaseWrapper {
    _pwcheck: String,
    entries: Vec<PasswordEntry>,
}

/// Derive a 256-bit key from password using Argon2id
fn derive_key(password: &str, salt: &[u8; 32]) -> Result<[u8; 32], FortressError> {
    let params = Params::new(65536, 3, 4, Some(32)).map_err(|_| FortressError::EncryptionFailed)?;

    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);

    let mut key = [0u8; 32];
    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|_| FortressError::EncryptionFailed)?;

    Ok(key)
}

/// Encrypt the password database
#[allow(deprecated)]
pub fn encrypt_database(
    entries: &[PasswordEntry],
    master_password: &str,
) -> Result<Vec<u8>, FortressError> {
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
    let key_bytes = derive_key(master_password, &salt_32)?;
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);

    // Create cipher and generate nonce
    let cipher = Aes256Gcm::new(key);
    let mut nonce_bytes = [0u8; 12];
    rand::rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|_| FortressError::EncryptionFailed)?;

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
) -> Result<Vec<PasswordEntry>, FortressError> {
    // Check minimum file size (32 + 12 + 16 = 60 bytes minimum)
    if encrypted_data.len() < 60 {
        return Err(FortressError::CorruptedVault);
    }

    // Extract components
    let salt: [u8; 32] = encrypted_data[0..32]
        .try_into()
        .map_err(|_| FortressError::CorruptedVault)?;
    let nonce_bytes: [u8; 12] = encrypted_data[32..44]
        .try_into()
        .map_err(|_| FortressError::CorruptedVault)?;
    let ciphertext = &encrypted_data[44..];

    // Derive key from password
    let key_bytes = derive_key(master_password, &salt)?;
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);

    // Create cipher and nonce
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Decrypt
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| FortressError::InvalidMasterPassword)?;

    // Parse JSON
    let json_str = std::str::from_utf8(&plaintext).map_err(|_| FortressError::CorruptedVault)?;

    let wrapper: DatabaseWrapper = serde_json::from_str(json_str)?;

    // Verify password check
    if wrapper._pwcheck != "valid" {
        return Err(FortressError::InvalidMasterPassword);
    }

    Ok(wrapper.entries)
}

// Example usage
#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::errors::FortressError;

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
        let encrypted =
            encrypt_database(&entries, master_password).expect("Encryption should succeed");

        // Decrypt
        let decrypted =
            decrypt_database(&encrypted, master_password).expect("Decryption should succeed");

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

        let encrypted =
            encrypt_database(&entries, "correct_password").expect("Encryption should succeed");

        let result = decrypt_database(&encrypted, "wrong_password");
        assert!(matches!(result, Err(FortressError::InvalidMasterPassword)));
    }
}
