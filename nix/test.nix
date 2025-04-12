{ pkgs, pp_klausur_manager }:
let
  common = { config, pkgs, ... }: {
    users.users.nixos = {
      isNormalUser = true;
      home = "/home/nixos";
      initialPassword = "nixos";
    };
    networking.firewall.enable = false;
    system.stateVersion = "24.11";
  };
in
pkgs.nixosTest {
  name = "test";
  nodes.control-machine = { config, pkgs, ... }: {
    imports = [ common ];
    services.cage = {
      enable = true;
      user = "nixos";
      program = "${pkgs.lib.getExe pkgs.alacritty} -o font.normal.family=\\\"\"JetBrainsMono Nerd Font\\\"\" -e ppmngr_launch";
    };
    fonts.packages = [ (pkgs.nerdfonts.override { fonts = [ "JetBrainsMono" ]; }) ];
    environment.systemPackages = [
      (pkgs.writeScriptBin "ppmngr_launch" ''
        cd /home/nixos/
        cp /etc/ppmngr_cfg.json .
        ${pkgs.lib.getExe pp_klausur_manager} test
      '')
    ];
    environment.etc."ppmngr_cfg.json".source = ./ppmngr_cfg.json;
  };
  nodes.client1 = { config, pkgs, ... }: {
    imports = [ common ];
  };
  testScript = "";
}
