with import <nixpkgs> {};
stdenv.mkDerivation {
    name = "dev-environment";
    buildInputs = [ pkg-config sqlx-cli openssl cargo-watch];
    shellHook = ''
        export DATABASE_URL="sqlite://sqlite.db"
    '';
}
