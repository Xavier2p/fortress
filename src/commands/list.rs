use crate::crypto::{CryptoError, PasswordEntry};

pub fn list(decrypted: Result<Vec<PasswordEntry>, CryptoError>) {
    println!("Listing all entries in the vault:");
    match decrypted {
        Ok(decrypted) => println!("Decrypted:\n{:#?}", decrypted),
        Err(e) => eprintln!("Error decrypting database: {:?}", e),
    }
}
