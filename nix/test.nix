{ pkgs, pp_klausur_manager }:
pkgs.nixosTest {
  name = "virtual exam computer pool";
  nodes.controlmachine = import ./config_controlmachine.nix { inherit pp_klausur_manager; };
  nodes.client1 = import ./config_client.nix;
  nodes.client2 = import ./config_client.nix;
  nodes.client3 = import ./config_client.nix;
  testScript = ''
    controlmachine.start()
    controlmachine.wait_for_unit("nfs-server.service")
    start_all()
  '';
}
