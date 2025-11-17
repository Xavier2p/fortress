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
//! frtrs add --identifier <identifier> --username <username> --password <password>
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
//!   create  Create a new vault
//!   add     Add a new entry to the vault
//!   list    List all entries in the vault
//!   help    Print this message or the help of the given subcommand(s)
//!
//! Options:
//!   -v, --verbose      Enable verbose output
//!   -f, --file <PATH>  The input file path [default: /tmp/vault.frt]
//!       --stdin        Get the master password from stdin. If not defined, will prompt for it
//!   -h, --help         Print help
//!   -V, --version      Print version
//! ```
mod commands;
mod crypto;
mod helpers;

use clap::Parser;
use helpers::structs::GeneralArgs;
use helpers::{cli, errors::raise};
use rpassword::prompt_password;

/// The main function, in which all magic happens.
fn main() {
    let args: cli::Cli = cli::Cli::parse();

    // Sanitize and validate all args here

    // get password TODO match with stdin flag
    let password: String = prompt_password("Enter the vault password: ").unwrap();

    let general_args: GeneralArgs = GeneralArgs::new(args.verbose, args.file.unwrap(), password);

    let result = match args.command {
        Some(cli::Commands::Create { force }) => commands::create::create(force, general_args),
        Some(cli::Commands::Add {
            identifier,
            username,
            password,
            generate,
        }) => commands::add::add(identifier, username, password, generate, general_args),
        Some(cli::Commands::List {}) => commands::list::list(general_args),
        None => commands::list::list(general_args),
    };

    match result {
        Ok(()) => (),
        Err(e) => raise(e),
    }
}
