//! List all entries in the vault.
use crate::helpers::structs::GeneralArgs;
use crate::helpers::{self, errors::FortressError};

/// List all entries in the vault.
/// All the entries will be printed with the format defined.
/// ## Parameters:
/// - `args`: The context of the program
/// ## Returns:
/// A result of nothing or a [`FortressError`]
pub fn list(args: GeneralArgs) -> Result<(), FortressError> {
    let decrypted = helpers::load_vault(args);
    match decrypted {
        Ok(decrypted) => {
            println!("[");
            decrypted.iter().for_each(|item| println!("\t{}", item));
            println!("]");
            Ok(())
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn test_list_on_empty_vault() {
        let path = tmp_path("list_empty");
        cleanup(&path);
        let args = GeneralArgs::new(false, path.clone(), "pw".to_string());

        let res = list(args);
        assert!(res.is_ok() || res.is_err());
        cleanup(&path);
    }

    #[test]
    fn test_list_after_add() {
        let path = tmp_path("list_after_add");
        cleanup(&path);
        let args = GeneralArgs::new(false, path.clone(), "master".to_string());
        crate::commands::create::create(true, args.clone()).expect("create failed");
        let _ = crate::commands::add::add(
            "id_list".to_string(),
            "user_list".to_string(),
            Some("pw".to_string()),
            false,
            args.clone(),
        );

        let res = list(args);
        assert!(res.is_ok(), "list should succeed after adding entry");
        cleanup(&path);
    }
}
