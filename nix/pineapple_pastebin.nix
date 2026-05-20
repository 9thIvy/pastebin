# MIT code adapted from copyparty © ed oss@ocv.me
{ config, lib, pkgs, ... }:
let 
  pname = "pineapple_pastebin";
  cfg = config.services.pineapple_pastebin;

in
{
  options.services.pineapple_pastebin = {
    enable = lib.mkEnableOption "pineapple_pastebin service";

    package = lib.mkPackageOption pkgs "pineapple_pastebin"{
      extraDescription = ''
        Package of the application to run, exposed for overriding purposes. 
      '';
    };

    user = lib.mkOption {
      type = lib.types.str;
      default = "pineapple_pastebin";
      description = ''
        The user that pineapple_pastebin will run under.

        If changed from default, you are responsible for making sure the user exists.
      '';
    };

    group = lib.mkOption {
      type = lib.types.str;
      default = "pineapple_pastebin";
      description = ''
        The group that pineapple_pastebin will run under.

        If changed from default, you are responsible for making sure the group exists.
      '';
    };
    
  };
 config = lib.mkIf cfg.enable (
  let
    command = "${lib.getExe cfg.package} -c $
    {runtimeConfigPath}";
  in {
    systemd.services.pineapple_pastebin = {
      description = "Pineapple Pastebin";
      wantedBy = [ "multi-user.target" ];
      after = [ "network.target" ];
    };

    lib.serviceConfig = {
      type = "notify";
      ExecStart = command;
      User = cfg.user;
      Group = cfg.group;
      Restart = "on-failure";
    };

    users.groups = lib.mkIf (cfg.group == "pineapple_pastebin"){
      pineapple_pastebin = { };
    };

    users.users = lib.mkIf (cfg.user == "pineapple_pastebin") {
      pineapple_pastebin = {
        description = "Service user of pineapple_pastebin";
        group = cfg.group;
        createHome = false;
        isSystemUser = true;
      };
    };
    
  }
    
  
 );
}