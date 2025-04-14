{ pkgs, pp_klausur_manager }:
pkgs.nixosTest {
  name = "virtual exam computer pool";
  nodes.control-machine = import ./config_controlmachine.nix { inherit pp_klausur_manager; };
  nodes.client1 = import ./config_client.nix;
  nodes.client2 = import ./config_client.nix;
  nodes.client3 = import ./config_client.nix;
  testScript = ''
    control_machine.start()
    control_machine.wait_for_unit("nfs-server.service")
    start_all()
  '';
}
