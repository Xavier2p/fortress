# Fortress

A simple password manager CLI application written in Rust

## Features

- [x] Securely store and manage passwords
- [x] Encrypt passwords using strong encryption algorithms
- [x] Command-line interface for easy access
- [ ] Cross-platform support (Windows, macOS, Linux)
- [ ] Import and export passwords in CSV format
- [x] Generate strong passwords
- [ ] Search and filter passwords
- [ ] Backup and restore password database

## Installation

1. Make sure you have Rust installed. If not, you can install it from [here](https://www.rust-lang.org/tools/install).
2. Clone the repository:
    ```bash
    git clone git@github.com:Xavier2p/fortress.git
    cd fortress
    ```
3. Build the project:
    ```bash
    cargo install --path .
    ```
4. Run the application:
    ```console
   $ frtrs --help
   A simple password manager CLI application

   Usage: frtrs [OPTIONS] <COMMAND>

   Commands:
   create  Create a new vault
   add     Add a new entry to the vault
   list    List all entries in the vault
   help    Print this message or the help of the given subcommand(s)

   Options:
   -v, --verbose      Enable verbose output
   -f, --file <PATH>  The input file path [default: /tmp/vault.frt]
   --stdin        Get the master password from stdin. If not defined, will prompt for it
   -h, --help         Print help
   -V, --version      Print version
    ```