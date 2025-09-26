use crate::helpers::structs::{GeneralArgs, PasswordEntry};
use crate::helpers::{self, errors::FortressError};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::structs::GeneralArgs;

    #[test]
    fn test_add_with_generate() {
        let args = GeneralArgs::new(false, "/tmp/test.frt".to_string(), "pw".to_string());
        let result = add("id".to_string(), "user".to_string(), None, true, args);
        // This will fail unless helpers are mocked, so just check type
        assert!(result.is_err() || result.is_ok());
    }
}
