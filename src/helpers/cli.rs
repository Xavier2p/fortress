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

    /// The input file path
    #[arg(short, long, value_name = "PATH", default_value = "/tmp/vault.frt")]
    pub file: Option<String>,

    /// Get the master password from stdin. If not defined, will prompt for it
    #[arg(long)]
    pub stdin: bool,

    /// Path to a file to write logs to
    #[arg(long, value_name = "PATH", default_value = "/tmp/fortress.log")]
    pub log_file: Option<String>,
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
        identifier: String,
    },

    /// View the password of the desired identifier
    View {
        /// The identifier of the entry
        identifier: String,
    },

    /// Remove an entry from the vault
    Remove {
        /// The identifier of the entry to remove
        identifier: String,
    },

    /// Add a new entry to the vault. If no one of the password methods is provided,
    /// the password will be the content of the clipboard.
    #[command(arg_required_else_help = true)]
    Add {
        /// The identifier for the entry
        identifier: String,

        /// The username or email address for the entry
        #[arg(short, long)]
        username: Option<String>,

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
        let cli = Cli::parse_from(["frt-rs", "add", "id", "--username", "user", "--generate"]);
        matches!(cli.command, Some(Commands::Add { .. }));
    }

    #[test]
    fn test_cli_parse_create() {
        let cli = Cli::parse_from(["frt-rs", "create", "--force"]);
        matches!(cli.command, Some(Commands::Create { .. }));
    }
}
