//! Error handling and types.
use std::{fmt::Debug, io};

/// The different errors that can be raised by the program. Names are self-explanatory.
#[allow(dead_code)]
#[derive(Debug)]
pub enum FortressError {
    VaultAlreadyExists,
    VaultNotFound,
    InvalidVaultPath,
    InvalidCommand,
    DecryptionFailed,
    EncryptionFailed,
    IoError(io::Error),
    SerializationError(serde_json::Error),
    InvalidMasterPassword,
    CorruptedVault,
}

/// Treat errors as errors.
impl std::error::Error for FortressError {}

/// Display the error message.
impl std::fmt::Display for FortressError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FortressError::CorruptedVault => {
                write!(
                    f,
                    "CorruptedVault: The selected vault is corrupted or has been tampered."
                )
            }
            FortressError::InvalidMasterPassword => {
                write!(
                    f,
                    "InvalidMasterPassword: The provided master password is incorrect."
                )
            }
            FortressError::VaultAlreadyExists => {
                write!(
                    f,
                    "VaultAlreadyExists: The vault already exists at the specified path."
                )
            }
            FortressError::VaultNotFound => {
                write!(
                    f,
                    "VaultNotFound: The vault file was not found at the specified path."
                )
            }
            FortressError::InvalidVaultPath => {
                write!(f, "InvalidVaultPath: The specified vault path is invalid.")
            }
            FortressError::InvalidCommand => {
                write!(f, "InvalidCommand: The provided command is invalid.")
            }
            FortressError::DecryptionFailed => write!(
                f,
                "DecryptionFailed: Failed to decrypt the vault. Possible reasons include an incorrect master password or corrupted data."
            ),
            FortressError::EncryptionFailed => {
                write!(f, "EncryptionFailed: Failed to encrypt the vault data.")
            }
            FortressError::IoError(e) => write!(f, "IoError: {}", e),
            FortressError::SerializationError(e) => write!(f, "SerializationError: {}", e),
        }
    }
}

/// Add `io` support for errors
impl From<io::Error> for FortressError {
    fn from(error: io::Error) -> Self {
        if error.kind() == io::ErrorKind::NotFound {
            FortressError::VaultNotFound
        } else {
            FortressError::IoError(error)
        }
    }
}

/// Add `serde` support for errors
impl From<serde_json::Error> for FortressError {
    fn from(error: serde_json::Error) -> Self {
        FortressError::SerializationError(error)
    }
}

/// Print the error message and exit the program with a non-zero exit code.
pub fn raise(error: FortressError) {
    eprintln!("Error: {}", error);
    std::process::exit(1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_display_vault_already_exists() {
        let err = FortressError::VaultAlreadyExists;
        assert!(format!("{}", err).contains("VaultAlreadyExists"));
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "not found");
        let fortress_err: FortressError = io_err.into();
        matches!(fortress_err, FortressError::VaultNotFound);
    }

    #[test]
    fn test_serde_error_conversion() {
        let serde_err = serde_json::from_str::<serde_json::Value>("not_json").unwrap_err();
        let fortress_err: FortressError = serde_err.into();
        matches!(fortress_err, FortressError::SerializationError(_));
    }
}
