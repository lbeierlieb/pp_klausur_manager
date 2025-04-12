{ pkgs }:
let
  common = {config, pkgs, ... }: {
    users.users.nixos = {
      isNormalUser  = true;
      home  = "/home/nixos";
      extraGroups  = [ "wheel" ];
      initialPassword = "nixos";
    };
    system.stateVersion = "24.11";
  };
in
pkgs.nixosTest {
  name = "test";
  nodes.control-machine = { config, pkgs, ... }: {
    imports = [ common ];
  };
  nodes.client1 = { config, pkgs, ... }: {
    imports = [ common ];
  };
  testScript = "";
}
