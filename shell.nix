with import <nixpkgs> {};
stdenv.mkDerivation {
    name = "dev-environment";
    buildInputs = [ pkg-config sqlx-cli openssl cargo-leptos ];
    LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [ openssl ];
    shellHook = ''
        export DATABASE_URL="sqlite://sqlite.db"
    '';
}
