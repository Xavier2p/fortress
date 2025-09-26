mod commands;
mod crypto;
mod data;
mod helpers;

use clap::Parser;
use helpers::{GeneralArgs, cli, errors::raise};
use rpassword::prompt_password;

fn main() {
    let args: cli::Cli = cli::Cli::parse();

    // Sanitize and validate all args here

    // get password (match with stdin flag)
    let password: String = prompt_password("Enter the vault password: ").unwrap();
    println!(
        "The password has been entered: {} ({})",
        "*".repeat(password.len()),
        password
    );

    let general_args: GeneralArgs = GeneralArgs::new(args.verbose, args.file.unwrap(), password);

    let result = match args.command {
        cli::Commands::Create { force } => commands::create::create(force, general_args),
        cli::Commands::Add {
            identifier,
            username,
            password,
            generate,
        } => commands::add::add(identifier, username, password, generate),
        cli::Commands::List {} => commands::list::list(general_args),
    };

    match result {
        Ok(()) => (),
        Err(e) => raise(e),
    }
}
