//! Copy a specific entry in the vault.
use crate::helpers::structs::GeneralArgs;
use crate::helpers::{self, errors::FortressError};

/// Copy the password of the specific entry.
/// ## Parameters:
/// - `identifier`: The path of the entry to copy
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::errors::FortressError;
    use crate::helpers::structs::GeneralArgs;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn tmp_path(name: &str) -> String {
        let mut p = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        p.push(format!("fortress_test_{}_{}.enc", name, nanos));
        p.to_str().unwrap().to_string()
    }

    fn cleanup(path: &str) {
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_copy_existing_entry() {
        let path = tmp_path("copy_test");
        cleanup(&path);
        let args = GeneralArgs::new(false, path.clone(), "master".to_string());
        crate::commands::create::create(true, args.clone()).expect("create failed");
        crate::commands::add::add(
            "copy_id".to_string(),
            "copy_user".to_string(),
            Some("copy_pw".to_string()),
            false,
            args.clone(),
        )
        .expect("add failed");

        let res = copy("copy_id".to_string(), args.clone());
        assert!(res.is_ok() || matches!(res, Err(FortressError::Clipboard(_))));
        cleanup(&path);
    }
    #[test]
    fn test_copy_missing_entry() {
        let path = tmp_path("copy_missing");
        cleanup(&path);
        let args = GeneralArgs::new(false, path.clone(), "master".to_string());
        crate::commands::create::create(true, args.clone()).expect("create failed");

        let res = copy("no_id".to_string(), args.clone());
        assert!(matches!(res, Err(FortressError::IdNotFound(_))));
        cleanup(&path);
    }
}
