{
  description = "A simple password manager written in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          system = system;
          config.allowUnfree = true;
        };
      in {
        inherit pkgs;
        devShells.default = pkgs.mkShell {
          name = "rust";
          buildInputs = with pkgs; [
            bacon
            cargo
            cargo-audit
            gcc
            rustup
            wrkflw
          ];
        };
      }
    );
}
