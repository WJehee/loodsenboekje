with import <nixpkgs> {};
stdenv.mkDerivation {
    name = "dev-environment";
    buildInputs = [ cargo pkg-config sqlx-cli openssl cargo-leptos binaryen ];
    LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [ openssl ];
    shellHook = ''
        export DATABASE_URL="sqlite://sqlite.db"
    '';
}
