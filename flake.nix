{
  description = "A simple password manager written in Rust";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      config.allowUnfree = true;
    };
  in {
    packages.${system}.default = let
      manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
    in
      pkgs.rustPlatform.buildRustPackage rec {
        pname = manifest.name;
        version = manifest.version;
        cargoLock.lockFile = ./Cargo.lock;
        src = pkgs.lib.cleanSource ./.;

        meta = {
          description = manifest.description;
          homepage = manifest.repository;
          maintainers = manifest.authors;
          mainProgram = "frtrs";
        };
      };

    devShells.${system}.default = pkgs.mkShell {
      name = "rust";
      buildInputs = with pkgs; [
        bacon
        cargo
        cargo-audit
        cargo-tarpaulin
        clippy
        gcc
        rustc
        rustfmt
        wrkflw
      ];
    };
  };
}
