# Fortress

*A simple password safe, written in Rust.*

## Concepts

You must define a master password, which will be used to encrypt the vault.
Each time you want to use the vault, you will be asked for this password.

First, create a vault file (by default it will create the vault at `/tmp/vault.frt`):

 ```sh
 frtrs create
 ```

Then, add entries to the vault (see docs to know more about the arguments):

 ```sh
 frtrs add --identifier <identifier> --username <username> --password <password>
 ```

> [!IMPORTANT]
> If none of the password methods are provided (neither `-p` nor `-g`), the password will be the content of the *
*clipboard**

## Security Principles

- The master password is not stored
- We are using only well-known methods and libraries
- Strict checks and tests before releases
- Independent code audit

## Installation

### Quick with Nix Flakes

```sh
nix run github:/xavier2p/fortress
```

### Build

> [!Note]
> This project requires Rust 1.56+ to build.

1. Clone the repository

 ```sh
 git clone https://github.com/xavier2p/fortress && cd fortress
 ```

2. Install the binary

 ```sh
 cargo install --path .
 ```

## Usage

> [!WARNING]
> For Flake utilization, please add before each command the `--` to precise it's a command to fortress.
>
> e.g. `nix run github:/xavier2p/fortress -- --version`

 ```console
 $ frtrs --help
 A simple password safe CLI app
 
 Usage: frtrs [OPTIONS] [COMMAND]
 
 Commands:
   create  Create a new vault
   copy    Copy the password of the desired identifier
   view    View the password of the desired identifier
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

## Development

You must have Rust installed. See [rust-lang.org](https://rust-lang.org/learn/get-started/) for more information.
All technical documentation is online, see [docs](https://xavier2p.github.io/fortress).

### Run tests

```sh
cargo test
```

### See Code Coverage

```sh
cargo tarpaulin
```

### Build and run

```sh
cargo build
cargo run
```

### Format code

```sh
cargo fmt
cargo clippy
```

## What we set up to make secure software

- We have some CI checks (tests, formatting, linting) running on each PR
- We run audit on dependencies weekly
- We use protected branches on GitHub, requiring reviews and all checks before merging

You can follow the development process via the issues and [project board](https://github.com/users/Xavier2p/projects/3)
on GitHub.

## Authors

- Xavier2p
- Axxiar
- Savened
- Aurel
