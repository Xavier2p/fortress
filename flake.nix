{
  description = "A simple password manager written in Rust";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      config.allowUnfree = true;
      overlays = [(import rust-overlay)];
    };
    rustToolchain = pkgs.rust-bin.stable.latest.default.override {
      extensions = ["rust-src" "clippy" "rustfmt"];
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
      nativeBuildInputs = [pkgs.rustToolchain];
      buildInputs = with pkgs; [
        bacon
        cargo
        cargo-audit
        cargo-tarpaulin
        rust-analyzer
        wrkflw
        jetbrains.rust-rover
      ];
      shellHook = ''
        mkdir -p ~/.rust-rover/toolchain

        ln -sfn ${rustToolchain}/lib ~/.rust-rover/toolchain
        ln -sfn ${rustToolchain}/bin ~/.rust-rover/toolchain

        export RUST_SRC_PATH="$HOME/.rust-rover/toolchain/lib/rustlib/src/rust/library"
      '';
    };
  };
}
