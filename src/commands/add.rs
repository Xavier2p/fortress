use crate::crypto::PasswordEntry;
use crate::helpers::{self, errors::FortressError, GeneralArgs};

pub fn add(
    identifier: String,
    username: String,
    password: Option<String>,
    generate: bool,
    args: GeneralArgs,
) -> Result<(), FortressError> {
    // Sanitize and validate inputs
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
            Ok(())
        }
        Err(e) => Err(e),
    }
}
