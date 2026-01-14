//! Error handling and types.
use std::{fmt::Debug, io};

/// The different errors that can be raised by the program. Names are self-explanatory.
#[derive(Debug)]
pub enum FortressError {
    VaultAlreadyExists,
    VaultNotFound,
    DecryptionFailed,
    EncryptionFailed,
    IoError(io::Error),
    SerializationError(serde_json::Error),
    InvalidMasterPassword,
    CorruptedVault,
    IdNotFound(String),
    Clipboard(String),
    WeakPassword,
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
            FortressError::DecryptionFailed => write!(
                f,
                "DecryptionFailed: Failed to decrypt the vault. Possible reasons include an incorrect master password or corrupted data."
            ),
            FortressError::EncryptionFailed => {
                write!(f, "EncryptionFailed: Failed to encrypt the vault data.")
            }
            FortressError::IoError(e) => write!(f, "IoError: {}", e),
            FortressError::SerializationError(e) => write!(f, "SerializationError: {}", e),
            FortressError::IdNotFound(id) => {
                write!(f, "IdNotFoundError: `{}` not found in the vault", id)
            }
            FortressError::Clipboard(pass) => write!(
                f,
                "ClipboardError: Unable to copy, the password is {}",
                pass
            ),
            FortressError::WeakPassword => write!(
                f,
                "WeakPasswordError: Your master password is not at the required strength."
            ),
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
    log::error!("Error: {}", error);
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
        let s = format!("{}", err);
        assert!(s.contains("VaultAlreadyExists"));
    }

    #[test]
    fn test_io_error_conversion_not_found() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "not found");
        let fortress_err: FortressError = io_err.into();
        assert!(matches!(fortress_err, FortressError::VaultNotFound));
    }

    #[test]
    fn test_io_error_conversion_other() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "denied");
        let fortress_err: FortressError = io_err.into();
        assert!(matches!(fortress_err, FortressError::IoError(_)));
    }

    #[test]
    fn test_serde_error_conversion() {
        let serde_err = serde_json::from_str::<serde_json::Value>("not_json").unwrap_err();
        let fortress_err: FortressError = serde_err.into();
        assert!(matches!(fortress_err, FortressError::SerializationError(_)));
    }

    #[test]
    fn test_display_various_errors() {
        let cases: Vec<(FortressError, &str)> = vec![
            (FortressError::CorruptedVault, "CorruptedVault"),
            (
                FortressError::InvalidMasterPassword,
                "InvalidMasterPassword",
            ),
            (FortressError::DecryptionFailed, "DecryptionFailed"),
            (FortressError::EncryptionFailed, "EncryptionFailed"),
            (FortressError::VaultNotFound, "VaultNotFound"),
        ];

        for (err, substr) in cases {
            let s = format!("{}", err);
            assert!(
                s.contains(substr),
                "Display for {:?} should contain '{}' but was '{}'",
                err,
                substr,
                s
            );
        }
    }

    #[test]
    fn test_display_id_not_found_and_clipboard() {
        let id = "missing_id".to_string();
        let e = FortressError::IdNotFound(id.clone());
        let s = format!("{}", e);
        assert!(s.contains(&id));

        let pw = "topsecret".to_string();
        let e2 = FortressError::Clipboard(pw.clone());
        let s2 = format!("{}", e2);
        assert!(s2.contains(&pw));
    }
}
