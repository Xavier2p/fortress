//! Copy a specific entry in the vault.
use crate::helpers::structs::GeneralArgs;
use crate::helpers::{self, errors::FortressError};

/// Copy the password of the specific entry.
/// ## Parameters:
/// - identifier: The path of the entry to copy
/// - `args`: The context of the program
/// ## Returns:
/// A result of nothing or a [`FortressError`]
pub fn copy(identifier: String, args: GeneralArgs) -> Result<(), FortressError> {
    let decrypted = helpers::load_vault(args);
    match decrypted {
        Ok(decrypted) => match decrypted.iter().find(|item| item.identifier == identifier) {
            Some(el) => {
                println!("{}", el);
                match cli_clipboard::set_contents(el.password.to_string()) {
                    Ok(_) => {
                        println!("The decoded password is in your clipboard");
                        Ok(())
                    }
                    Err(_) => Err(FortressError::Clipboard(el.password.to_string())),
                }
            }
            None => Err(FortressError::IdNotFound(identifier)),
        },
        Err(e) => Err(e),
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::helpers::structs::GeneralArgs;
//
//     #[test]
//     fn test_list_empty() {
//         let args = GeneralArgs::new(false, "/tmp/test.frt".to_string(), "pw".to_string());
//         let result = list(args);
//         assert!(result.is_err() || result.is_ok());
//     }
// }
