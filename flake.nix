{
  description = "Chameleon devshell";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        metadata = (pkgs.lib.importTOML ./Cargo.toml).package;
      in
      with pkgs;
      {
        devShells.default = mkShell {
          name = "chameleon";
          buildInputs = [
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" ];
              targets = [ "arm-unknown-linux-gnueabihf" ];
            })
            rust-analyzer
            pkg-config
            gcc
            just
          ];
          
          shellHook = ''
            fish
          '';
        };
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = metadata.name;
          version = metadata.version;
          cargoLock.lockFile = ./Cargo.lock;
          src = pkgs.lib.cleanSource ./.;

          nativeBuildInputs = [ pkg-config ];

          buildInputs = [
            openssl            
            bzip2
          ];
          
        };
      }
    );
}
