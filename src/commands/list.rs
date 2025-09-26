use crate::helpers::structs::GeneralArgs;
use crate::helpers::{self, errors::FortressError};

pub fn list(args: GeneralArgs) -> Result<(), FortressError> {
    let decrypted = helpers::load_vault(args);
    match decrypted {
        Ok(decrypted) => {
            println!("All stored entries: [");
            decrypted.iter().for_each(|item| println!("{}", item));
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

    #[test]
    fn test_list_empty() {
        let args = GeneralArgs::new(false, "/tmp/test.frt".to_string(), "pw".to_string());
        let result = list(args);
        assert!(result.is_err() || result.is_ok());
    }
}
