{ config, pkgs, lib, pineapple_pastebin, ... }:
with lib;
let
  cfg = config.services.pineapple_pastebin;
in
{
  options.services.copyparty = {
    enable = mkEnableOption "pastebin";

    package = mkPackageOption pkgs "pineapple_pastebin";

    user = mkOption {
      type = types.str;
      default = "pineapple_pastebin";
      description = ''
        The user that pineapple_pastebin will run as.
      '';
    };

    group = mkOption {
      type = types.str;
      default = "pineapple_pastebin";
      description = ''
        The group that pineapple_pastebin will run as.
      '';
    };
  };

  config = mkIf cfg.enable (
    let

    # probably ai garbage
      usedPackage = if cfg.package == null then pineapple_pastebin else cfg.package;
      command = "${getExe usedPackage}";
    in {
      systemd.services.pineapple_pastebin = {
        wantedBy = [ "multi-user.target" ];

        serviceConfig = {
          Type = "notify";
          ExecStart = command;
          User = cfg.user;
          Group = cfg.group;
          After = [ "network.target" ];
        };
      };

      environment.systemPackages = lib.mkIf (cfg.package == null) [ usedPackage ];

      users.groups = lib.mkIf (cfg.group == "pineapple_pastebin" ){
        pineapple_pastebin = { };
      };

      users.users = lib.mkIf (cfg.user == "pineapple_pastebin" ){
        pineapple_pastebin = {
          description = "pineapple_pastebin user";
          group = cfg.group;
          isSystemUser = true;
        };
      };
    }
  );
}