//! *A simple password safe, written in Rust.*
//!
//! ## Concepts
//! You must define a master password, which will be used to encrypt the vault.
//! Each time you want to use the vault, this password will be asked you.
//!
//! First, create a vault file:
//! ```sh
//! frtrs create
//! ```
//!
//! Then, add entries to the vault (see docs to know more about the arguments):
//! ```sh
//! frtrs add <identifier> --username <username> --password <password>
//! ```
//!
//! ## Security Principles
//!
//! - The master password is not stored
//! - We are using only well-known methods and libraries
//! - String checks and tests before releases
//! - Independent code audit
//!
//! ## Installation
//!
//! *Note: This project requires Rust 1.56+ to build.*
//!
//! 1. Clone the repository
//!
//! ```sh
//! git clone https://github.com/xavier2p/fortress && cd fortress
//! ```
//!
//! 2. Install the binary
//!
//! ```sh
//! cargo install --path .
//! ```
//!
//! ## Usage
//!
//! ```console
//! $ frtrs --help
//! A simple password safe CLI app
//!
//! Usage: frtrs [OPTIONS] [COMMAND]
//!
//! Commands:
//! create  Create a new vault
//! copy    Copy the password of the desired identifier
//! view    View the password of the desired identifier
//! remove  Remove an entry from the vault
//! add     Add a new entry to the vault. If no one of the password methods is provided, the password will be the content of the clipboard
//! list    List all entries in the vault
//! help    Print this message or the help of the given subcommand(s)
//!
//! Options:
//! -f, --file <PATH>      The input file path [default: /tmp/vault.frt]
//! --log-file <PATH>  Path to a file to write logs to [default: /tmp/fortress.log]
//! -h, --help             Print help
//! -V, --version          Print version
//! ```
mod commands;
mod crypto;
mod helpers;

use clap::Parser;
use helpers::structs::GeneralArgs;
use helpers::{cli, errors::raise, logger};
use rpassword::prompt_password;
use std::io;
use std::io::{IsTerminal, Read, Stdin};

/// The main function, in which all magic happens.
fn main() {
    let args: cli::Cli = cli::Cli::parse();
    let _ = logger::init(args.log_file.as_deref());

    let stdin: Stdin = io::stdin();
    let mut password: String = String::new();
    if stdin.is_terminal() {
        password = prompt_password("Enter the master vault password: ").unwrap();
    } else {
        let mut handle = stdin.lock();
        let _ = handle.read_to_string(&mut password);
        password = password.trim().to_string();
    }

    let general_args: GeneralArgs = GeneralArgs::new(args.file.unwrap(), password);

    let result = match args.command {
        Some(cli::Commands::Create { force }) => commands::create::create(force, general_args),
        Some(cli::Commands::Add {
            identifier,
            username,
            password,
            generate,
        }) => commands::add::add(
            identifier,
            username.unwrap_or("<empty>".to_string()),
            password,
            generate,
            general_args,
        ),
        Some(cli::Commands::List {}) => commands::list::list(general_args),
        Some(cli::Commands::Copy { identifier }) => commands::copy::copy(identifier, general_args),
        Some(cli::Commands::View { identifier }) => commands::view::view(identifier, general_args),
        Some(cli::Commands::Remove { identifier }) => {
            commands::remove::remove(identifier, general_args)
        }
        None => commands::list::list(general_args),
    };

    match result {
        Ok(()) => (),
        Err(e) => raise(e),
    }
}

#[cfg(test)]
mod tests {
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
    fn test_create_add_list_flow() {
        let path = tmp_path("main_flow");
        cleanup(&path);

        let args = GeneralArgs::new(path.clone(), "S3cureP@ssword".to_string());

        let create_res = crate::commands::create::create(true, args.clone());
        assert!(create_res.is_ok());

        let add_res = crate::commands::add::add(
            "id1".to_string(),
            "user1".to_string(),
            Some("secretpw".to_string()),
            false,
            args.clone(),
        );
        assert!(add_res.is_ok());

        let list_res = crate::commands::list::list(args.clone());
        assert!(list_res.is_ok());

        cleanup(&path);
    }

    #[test]
    fn test_create_without_force_fails_if_exists() {
        let path = tmp_path("main_exists");
        cleanup(&path);
        let mut f = fs::File::create(&path).unwrap();
        use std::io::Write;
        writeln!(f, "dummy").unwrap();

        let args = GeneralArgs::new(path.clone(), "S3cureP@ssword".to_string());
        let res = crate::commands::create::create(false, args.clone());
        assert!(res.is_err());

        cleanup(&path);
    }
}
