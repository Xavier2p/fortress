use crate::crypto::{Crypto, PasswordEntry};
use crate::helpers::errors::FortressError;
use std::fs;

pub mod cli;
pub mod errors;

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

pub fn save_vault(args: GeneralArgs, entries: &[PasswordEntry]) -> Result<(), FortressError> {
    let encrypted = match Crypto::encrypt_database(entries, &args.password) {
        Ok(vault) => vault,
        Err(_) => return Err(FortressError::EncryptionFailed),
    };

    match fs::write(&args.file, encrypted) {
        Ok(_) => Ok(()),
        Err(e) => Err(FortressError::IoError(e)),
    }
}

pub fn load_vault(args: GeneralArgs) -> Result<Vec<PasswordEntry>, FortressError> {
    let encrypted = match fs::read(&args.file) {
        Ok(data) => data,
        Err(e) => return Err(FortressError::IoError(e)),
    };

    match Crypto::decrypt_database(&encrypted, &args.password) {
        Ok(entries) => Ok(entries),
        Err(_) => Err(FortressError::DecryptionFailed),
    }
}

pub fn debug(args: &GeneralArgs, message: String) {
    if args.verbose {
        println!("[DEBUG]: {}", message);
    }
}
