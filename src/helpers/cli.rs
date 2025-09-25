use clap::{Parser, Subcommand};
use std::path::Path;

#[derive(Parser)]
#[command(name = "frt-rs", version, about, long_about = None)]
pub struct Cli {
    /// The command to run
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// The input file path
    #[arg(short, long, value_name = "PATH", default_value = "/tmp/vault.frt", value_parser = check_vault_path)]
    pub file: Option<String>,

    /// Get the master password from stdin. If not defined, will prompt for it
    #[arg(long)]
    pub stdin: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new vault
    Create {
        /// Overwrite the vault if it already exists
        #[arg(short, long)]
        force: bool,
    },

    /// Add a new entry to the vault
    #[command(arg_required_else_help = true)]
    Add {
        /// The identifier for the entry
        #[arg(short, long)]
        identifier: String,

        /// The username for the entry
        #[arg(short, long)]
        username: String,

        /// Generate a new password. Mutually exclusive with 'direct'
        #[arg(short, long, conflicts_with = "password")]
        generate: bool,

        /// Direct password input. Mutually exclusive with 'generate'
        #[arg(short, long, conflicts_with = "generate")]
        password: Option<String>,
    },

    /// List all entries in the vault
    List {},
}

fn check_vault_path(input: &str) -> Result<String, String> {
    if !Path::new(input).exists() {
        Err(format!("The path '{}' does not exist.", input))
    } else {
        Ok(input.to_string())
    }
}
