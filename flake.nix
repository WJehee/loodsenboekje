{
    description = "Loodsenboekje";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
        rust-overlay.url = "github:oxalica/rust-overlay";
        crane-flake = {
            url = "github:ipetkov/crane";
            inputs.nixpkgs.follows = "nixpkgs";
        };
    };

    outputs = { self, nixpkgs, rust-overlay, crane-flake }:
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
        craneLib = (crane-flake.mkLib pkgs).overrideToolchain rust-toolchain;

        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        inherit (cargoToml.package) name version;

        args = {
            src = craneLib.path ./.;
            pname = name;
            version = version;
            # Required build inputs
            buildInputs = with pkgs; [
                rust-toolchain
                cargo-leptos
                binaryen
                openssl
            ];
        };

        LOODSENBOEKJE_DATA_DIR = "/var/lib/${name}";
        DATABASE_URL = "sqlite.db";

        loodsenboekje = craneLib.buildPackage (args // {
            pname = name;
            nativeBuildInputs = with pkgs; [
                makeWrapper
                sqlx-cli
            ];

            doCheck = false;
            cargoArtifacts = craneLib.buildDepsOnly args;

            preBuild = ''
                export DATABASE_URL='sqlite://${DATABASE_URL}'
                sqlx database create
                sqlx migrate run
            '';
            buildPhaseCargoCommand = "cargo leptos build --release -vvv";
            installPhaseCommand = ''
                mkdir -p $out/
                cp target/release/${name} $out/
                cp -r target/site $out/
                wrapProgram $out/${name} \
                --set LEPTOS_SITE_ROOT $out/site \
                --set READ_PASSWORD $(openssl rand -base64 32) \
                --set WRITE_PASSWORD $(openssl rand -base64 32) \
                --set ADMIN_PASSWORD $(openssl rand -base64 32) \
                --set DATA_DIR ${LOODSENBOEKJE_DATA_DIR}
            '';
        });
    in {
        devShells.${system}.default = with pkgs; mkShell {
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
                export DATABASE_URL='sqlite://${DATABASE_URL}'
            '';
        };
        packages.${system}.default = loodsenboekje;
    };
}

