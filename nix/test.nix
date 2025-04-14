{ pkgs, pp_klausur_manager }:
pkgs.nixosTest {
  name = "test";
  nodes.control-machine = import ./config_controlmachine.nix { inherit pp_klausur_manager; };
  nodes.client1 = import ./config_client.nix;
  testScript = ''
    control_machine.start()
    control_machine.wait_for_unit("nfs-server.service")
    start_all()
  '';
}
