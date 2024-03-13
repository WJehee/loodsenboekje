flake: { config, system, lib }:
let
    inherit (lib) types mkEnableOption mkOption mdDoc;
    inherit (flake.packages.${system}) loodsenboekje;

    cfg = config.services.loodsenboekje;
in {
    options = {
        services.loodsenboekje = {
            enable = mkEnableOption "Loodsenboekje";
        };
        dataDir = mkOption {
            type = types.str;
            default = "/var/lib/loodsenboekje";
            description = mdDoc "Path where data and logs will be stored";
        };
        package = mkOption {
            type = types.package;
            default = loodsenboekje;
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
            serviceConfig = {
                Type = "simple";
                User = "loodsenboekje";
                Group = "loodsenboekje";

                Restart = "always";
                ExecStart = "DATA_DIR=${cfg.dataDir} ${lib.getBin cfg.package}/bin/loodsenboekje";
                StateDirectory = "loodsenboekje";
            };
        };
    };
}
