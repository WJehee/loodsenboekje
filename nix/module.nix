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
            environment.DATA_DIR = "${cfg.dataDir}";
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
