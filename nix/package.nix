{ lib, pkgs, rust-toolchain, craneLib }:
let
    cargoToml = builtins.fromTOML (builtins.readFile ../Cargo.toml);
    inherit (cargoToml.package) name version;

    args = {
        # src = craneLib.cleanCargoSource (craneLib.path ./.);
        src = lib.cleanSourceWith {
            src = craneLib.path ../.;
            filter = path: type:
            (lib.hasSuffix ".sql" path) ||
            (craneLib.filterCargoSources path type)
            ;
        };
        pname = name;
        version = version;
        buildInputs = with pkgs; [
            rust-toolchain
            cargo-leptos
            binaryen
            openssl
        ];
    };
in 
craneLib.buildPackage (args // {
    name = "Loodsenboekje";
    nativeBuildInputs = with pkgs; [
        makeWrapper
        sqlx-cli
    ];
    doCheck = false;
    cargoArtifacts = craneLib.buildDepsOnly args;

    preBuild = ''
        export DATABASE_URL='sqlite://sqlite.db'
        sqlx database create
        sqlx migrate run
    '';
    buildPhaseCargoCommand = "cargo leptos build --release -vvv";
    installPhaseCommand = ''
        mkdir -p $out/bin/
        cp target/release/${name} $out/bin/
        cp -r target/site $out/bin/
        wrapProgram $out/bin/${name} \
        --set LEPTOS_SITE_ROOT $out/bin/site \
        --set LEPTOS_SITE_ADDR 0.0.0.0:1744 \
        --set READ_PASSWORD $(openssl rand -base64 32) \
        --set WRITE_PASSWORD $(openssl rand -base64 32) \
        --set ADMIN_PASSWORD $(openssl rand -base64 32) \
    '';
})

