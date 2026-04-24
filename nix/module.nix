flake: { config, pkgs, lib, ... }:
let
    inherit (lib) types mkEnableOption mkOption mdDoc;

    package = flake.packages.${pkgs.stdenv.hostPlatform.system}.default;

    cfg = config.services.loodsenboekje;
in {
    options.services.loodsenboekje = {
        enable = mkEnableOption "Loodsenboekje";
        dataDir = mkOption {
            type = types.str;
            default = "/var/lib/loodsenboekje";
            description = mdDoc "Path where data and logs will be stored";
        };
        package = mkOption {
            type = types.package;
            default = package;
            description = mdDoc "The package to use";
        };
        auth = {
            mode = mkOption {
                type = types.enum [ "local" "authelia" ];
                default = "local";
                description = mdDoc ''
                    Authentication mode.

                    - `local`: built-in username/password auth (default).
                    - `authelia`: trust `Remote-*` headers set by an upstream
                      reverse proxy running Authelia. The app MUST NOT be
                      directly reachable in this mode; see
                      docs/authelia-integration.md.
                '';
            };
            groups = {
                admin = mkOption {
                    type = types.str;
                    default = "loodsenboekje-admin";
                    description = mdDoc "Authelia group mapped to the Admin role.";
                };
                writer = mkOption {
                    type = types.str;
                    default = "loodsenboekje-writer";
                    description = mdDoc "Authelia group mapped to the Writer role.";
                };
                reader = mkOption {
                    type = types.str;
                    default = "loodsenboekje-reader";
                    description = mdDoc "Authelia group mapped to the Reader role.";
                };
            };
        };
    };
    config = lib.mkIf cfg.enable {
        users = {
            users.loodsenboekje = {
                description = "Loodsenboekje daemon";
                isSystemUser = true;
                group = "loodsenboekje";
            };
            groups.loodsenboekje = {};
        };
        systemd.services.loodsenboekje = {
            wantedBy = [ "multi-user.target" ];
            after = [ "network.target" ];
            description = "Loodsenboekje server";
            environment = {
                DATA_DIR = "${cfg.dataDir}";
                AUTH_MODE = cfg.auth.mode;
                AUTHELIA_GROUP_ADMIN = cfg.auth.groups.admin;
                AUTHELIA_GROUP_WRITER = cfg.auth.groups.writer;
                AUTHELIA_GROUP_READER = cfg.auth.groups.reader;
            };
            serviceConfig = {
                Type = "simple";
                User = "loodsenboekje";
                Group = "loodsenboekje";

                Restart = "always";
                ExecStart = "${lib.getBin cfg.package}/bin/loodsenboekje";
                StateDirectory = "loodsenboekje";
            };
        };
    };
}
