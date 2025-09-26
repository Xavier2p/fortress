use crate::helpers::{self, GeneralArgs, errors::FortressError};

pub fn list(args: GeneralArgs) -> Result<(), FortressError> {
    let decrypted = helpers::load_vault(args);
    match decrypted {
        Ok(decrypted) => {
            println!("Listing all entries in the vault:\n{:#?}", decrypted);
            Ok(())
        }
        Err(e) => Err(e),
    }
}
