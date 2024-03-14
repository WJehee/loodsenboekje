{
    description = "Loodsenboekje";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
        rust-overlay.url = "github:oxalica/rust-overlay";
        crane = {
            url = "github:ipetkov/crane";
            inputs.nixpkgs.follows = "nixpkgs";
        };
    };

    outputs = { self, nixpkgs, rust-overlay, crane }:
    let
        system = "x86_64-linux";
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
            inherit system overlays;
        };
        rust-toolchain = pkgs.rust-bin.nightly.latest.default.override {
            extensions = [ "rust-src" ];
            targets = [ "wasm32-unknown-unknown" ];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain rust-toolchain;
    in {
        devShells.${system}.default = with pkgs; mkShell {
            # TODO: make this combine with universal required build inputs
            buildInputs = [
                rust-toolchain
                cargo
                cargo-watch

                rust-analyzer
                rustfmt
                clippy

                cargo-leptos
                sqlx-cli

                openssl
                binaryen
            ];
            LD_LIBRARY_PATH = lib.makeLibraryPath [ openssl ];
            shellHook = ''
                export DATA_DIR='./.'
                export DATABASE_URL='sqlite://sqlite.db'
            '';
        };
        packages.${system}.default = pkgs.callPackage ./nix/package.nix {
            rust-toolchain = rust-toolchain;
            craneLib = craneLib;
        };
        inherit system;
        nixosModules.loodsenboekje = import ./nix/module.nix self;
    };
}

