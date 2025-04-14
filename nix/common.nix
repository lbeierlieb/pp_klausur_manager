{ config, pkgs, ... }: {
    users.users.nixos = {
      isNormalUser = true;
      home = "/home/nixos";
      initialPassword = "nixos";
    };
    networking.firewall.enable = false;
    system.stateVersion = "24.11";
  }
