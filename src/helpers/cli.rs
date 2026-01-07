//! CLI related structs and functions.
use clap::{Parser, Subcommand};

/// The CLI context.
#[derive(Parser)]
#[command(name = "frt-rs", version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// The command to run
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// The input file path
    #[arg(short, long, value_name = "PATH", default_value = "/tmp/vault.frt")]
    pub file: Option<String>,

    /// Get the master password from stdin. If not defined, will prompt for it
    #[arg(long)]
    pub stdin: bool,
}

/// The different commands that can be run.
#[derive(Subcommand)]
pub enum Commands {
    /// Create a new vault
    Create {
        /// Overwrite the vault if it already exists
        #[arg(short, long)]
        force: bool,
    },

    /// Copy the password of the desired identifier
    Copy {
        /// The identifier of the entry
        #[arg(short, long)]
        identifier: String,
    },

    /// View the password of the desired identifier
    View {
        /// The identifier of the entry
        #[arg(short, long)]
        identifier: String,
    },

    /// Add a new entry to the vault. If no one of the password methods are provided,
    /// the password will be the content of the clipboard.
    #[command(arg_required_else_help = true)]
    Add {
        /// The identifier for the entry
        #[arg(short, long)]
        identifier: String,

        /// The username for the entry
        #[arg(short, long)]
        username: String,

        /// Generate a new password. Mutually exclusive with 'password'
        #[arg(short, long, conflicts_with = "password")]
        generate: bool,

        /// Direct password input. Mutually exclusive with 'generate'
        #[arg(short, long, conflicts_with = "generate")]
        password: Option<String>,
    },
    /// List all entries in the vault
    List {},
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parse_add() {
        let cli = Cli::parse_from([
            "frt-rs",
            "add",
            "--identifier",
            "id",
            "--username",
            "user",
            "--generate",
        ]);
        matches!(cli.command, Some(Commands::Add { .. }));
    }

    #[test]
    fn test_cli_parse_create() {
        let cli = Cli::parse_from(["frt-rs", "create", "--force"]);
        matches!(cli.command, Some(Commands::Create { .. }));
    }
}
