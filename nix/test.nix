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
    environment.etc."real_task.html".source = ./../taskdescription_localtest/SRC/index.html;
    environment.etc."tmp_task.html".source = ./../taskdescription_localtest/TMP/index.html;
    systemd.tmpfiles.rules = [
      "d /nfs 0755 root root -"
      "L /nfs/task_description.html - - - - /nfs/tmp_task.html"
    ];
    systemd.services.custom-nfs-mount = {
      description = "nfs mounting";
      wantedBy = [ "multi-user.target" ];
      script = "cp /etc/real_task.html /nfs && cp /etc/tmp_task.html /nfs";
      serviceConfig.Type = "oneshot";
    };
    services.nfs.server.enable = true;
    services.nfs.server.exports = ''
      /nfs  *(rw,no_subtree_check,no_root_squash,fsid=0)
    '';
  };
  nodes.client1 = { config, pkgs, ... }: {
    imports = [ common ];
    systemd.services.my-service = {
      description = "kanata service";

      wantedBy = [ "multi-user.target" ];

      serviceConfig = {
        ExecStart = "${pkgs.lib.getExe pkgs.kanata} -c ${./../kanata.cfg} -p 0.0.0.0:5000";
        Restart = "always";
      };
    };
    services.cage = {
      enable = true;
      user = "nixos";
      program = "${pkgs.lib.getExe pkgs.firefox} file:///nfs/task_description.html http://control-machine:8080";
    };
    systemd.services.custom-nfs-mount = {
      description = "nfs mounting";
      wantedBy = [ "multi-user.target" ];
      after = [ "network-online.target" ];
      script = "${pkgs.util-linux}/bin/mount control-machine:/nfs /nfs";
      serviceConfig.Type = "oneshot";
    };
    boot.supportedFilesystems = [ "nfs" ];
    systemd.tmpfiles.rules = [
      "d /nfs 0755 root root -"
    ];
  };
  testScript = ''
    control_machine.start()
    control_machine.wait_for_unit("nfs-server.service")
    start_all()
  '';
}
