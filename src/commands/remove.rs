//! Remove a specific entry from the vault.
use crate::helpers::structs::GeneralArgs;
use crate::helpers::{self, errors::FortressError};

/// Remove the password of the specific entry.
/// ## Parameters:
/// - `identifier`: The path of the entry to remove
/// - `args`: The context of the program
/// ## Returns:
/// A result of nothing or a [`FortressError`]
pub fn remove(identifier: String, args: GeneralArgs) -> Result<(), FortressError> {
    let decrypted = helpers::load_vault(args.clone());
    match decrypted {
        Ok(decrypted) => match decrypted.iter().find(|item| item.identifier == identifier) {
            Some(_) => {
                let updated_entries: Vec<crate::helpers::structs::PasswordEntry> = decrypted
                    .into_iter()
                    .filter(|item| item.identifier != identifier)
                    .collect();
                helpers::save_vault(args, &updated_entries)?;
                println!("Entry '{}' has been removed.", identifier);
                Ok(())
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
    fn test_remove_existing_and_missing() {
        let path = tmp_path("remove_test");
        cleanup(&path);
        let args = GeneralArgs::new(false, path.clone(), "master".to_string());
        crate::commands::create::create(true, args.clone()).expect("create failed");
        crate::commands::add::add(
            "remove_id".to_string(),
            "remove_user".to_string(),
            Some("remove_pw".to_string()),
            false,
            args.clone(),
        )
        .expect("add failed");

        let remove_res = remove("remove_id".to_string(), args.clone());
        assert!(remove_res.is_ok());

        let remove_missing_res = remove("remove_id".to_string(), args.clone());
        assert!(matches!(
            remove_missing_res,
            Err(FortressError::IdNotFound(_))
        ));

        cleanup(&path);
    }

    #[test]
    fn test_remove_from_empty_vault() {
        let path = tmp_path("remove_empty_test");
        cleanup(&path);
        let args = GeneralArgs::new(false, path.clone(), "master".to_string());
        crate::commands::create::create(true, args.clone()).expect("create failed");
        let remove_res = remove("nonexistent_id".to_string(), args.clone());
        assert!(matches!(
            remove_res,
            Err(FortressError::IdNotFound(_))
        ));
        cleanup(&path);
    }
}
