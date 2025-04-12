{ pkgs }:
pkgs.nixosTest {
  name = "test";
  nodes.control-machine = { config, pkgs, ... }: {
    system.stateVersion = "24.11";
  };
  nodes.client1 = { config, pkgs, ... }: {
    system.stateVersion = "24.11";
  };
  testScript = "";
}
