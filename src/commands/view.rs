//! View a specific entry in the vault.

use crate::helpers::structs::GeneralArgs;
use crate::helpers::{self, errors::FortressError};

/// Display the password of the specific entry.
/// ## Parameters:
/// - `identifier`: The path of the entry to display
/// - `args`: The context of the program
/// ## Returns:
/// A result of nothing or a [`FortressError`]
pub fn view(identifier: String, args: GeneralArgs) -> Result<(), FortressError> {
    let decrypted = helpers::load_vault(args);
    match decrypted {
        Ok(decrypted) => match decrypted.iter().find(|item| item.identifier == identifier) {
            Some(el) => {
                println!("{}", el);
                log::info!("Entry viewed: {}", identifier);
                println!("The decoded password is: `{}`", el.password);
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
    fn test_view_existing_and_missing() {
        let path = tmp_path("view_test");
        cleanup(&path);
        let args = GeneralArgs::new(path.clone(), "master".to_string());
        crate::commands::create::create(true, args.clone()).expect("create failed");
        crate::commands::add::add(
            "view_id".to_string(),
            "view_user".to_string(),
            Some("view_pw".to_string()),
            false,
            args.clone(),
        )
        .expect("add failed");
        let res_ok = view("view_id".to_string(), args.clone());
        assert!(res_ok.is_ok(), "view should succeed for existing id");

        let res_missing = view("no_such_id".to_string(), args.clone());
        assert!(matches!(res_missing, Err(FortressError::IdNotFound(_))));
        cleanup(&path);
    }
}
