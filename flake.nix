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

        EXPORT_DATABASE_URL = "export DATABASE_URL='sqlite://sqlite.db'";

        loodsenboekje = craneLib.buildPackage (args // {
            pname = name;
            nativeBuildInputs = with pkgs; [
                makeWrapper
                sqlx-cli
            ];

            doCheck = false;
            cargoArtifacts = craneLib.buildDepsOnly args;

            preBuild = ''
                ${EXPORT_DATABASE_URL}
                sqlx database create
                sqlx migrate run
            '';
            buildPhaseCargoCommand = "cargo leptos build --release -vvv";
            installPhaseCommand = ''
                mkdir -p $out/bin
                cp target/release/${name} $out/bin/
                cp -r target/site $out/bin/
                wrapProgram $out/bin/${name} \
                --set LEPTOS_SITE_ROOT $out/bin/site

                touch .env
                p1=$(openssl rand -base64 32)
                p2=$(openssl rand -base64 32)
                p3=$(openssl rand -base64 32)
                echo "READ_PASSWORD=$p1" >> .env
                echo "WRITE_PASSWORD=$p2" >> .env
                echo "ADMIN_PASSWORD=$p3" >> .env
                cp .env $out/bin/
            '';
        });
    in {
        devShells.${system}.default = with pkgs; mkShell (args // {
            # Dev tools
            buildInputs = [
                cargo-watch
                rust-analyzer
                rustfmt
                clippy
                sqlx-cli
            ];
            LD_LIBRARY_PATH = lib.makeLibraryPath [ openssl ];
            shellHook = ''
                ${EXPORT_DATABASE_URL}
            '';
        });
        packages.${system}.default = loodsenboekje;
    };
}

