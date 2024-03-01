{
  description = "Loodsenboekje";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: let
        system = "x86_64-linux";
        pkgs = nixpkgs.legacyPackages.${system};
  in {
        devShells.${system}.default = pkgs.mkShell {
            buildInputs = with pkgs; [
                pkg-config
                openssl
                binaryen

                rustup
                cargo
                cargo generate
                cargo watch
                cargo-leptos
                sqlx-cli
            ];
            LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [ openssl ];
            shellHook = ''
                export DATABASE_URL="sqlite://sqlite.db"
            '';
        };
  };
}
