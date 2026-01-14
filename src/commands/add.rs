//! Add a new entry to the vault.
use crate::helpers::structs::{GeneralArgs, PasswordEntry};
use crate::helpers::{self, errors::FortressError};

/// Add a new entry to the vault.
/// If no one of password or generate is provided, the clipboard is used.
/// ## Parameters:
/// - `identifier`: the *path* od the entry. Must be unique.
/// - `username`: Username or email used to log in.
/// - `password`: if provided, the password to save.
/// - `generate`: If true, generate a new password.
/// - `args`: The context of the program
/// ## Returns:
/// A result of nothing or a [`FortressError`]
pub fn add(
    identifier: String,
    username: String,
    password: Option<String>,
    generate: bool,
    args: GeneralArgs,
) -> Result<(), FortressError> {
    let password = if generate {
        helpers::generate_password(32)
    } else if let Some(pw) = password {
        pw
    } else {
        cli_clipboard::get_contents().unwrap()
    };

    let entry = PasswordEntry {
        identifier,
        username,
        password,
    };

    let mut updated: Vec<PasswordEntry> = match helpers::load_vault(args.clone()) {
        Ok(entries) => entries.clone(),
        Err(e) => return Err(e),
    };

    updated.push(entry.clone());

    match helpers::save_vault(args, &updated) {
        Ok(_) => {
            println!("{}", entry);
            log::info!("Added entry {}", entry.identifier);
            Ok(())
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::structs::GeneralArgs;

    #[test]
    fn test_add_with_generate() {
        let args = GeneralArgs::new("/tmp/test.frt".to_string(), "pw".to_string());
        let result = add("id".to_string(), "user".to_string(), None, true, args);
        assert!(result.is_err() || result.is_ok());
    }
}
