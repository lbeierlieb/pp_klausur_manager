{ pkgs, ... }: {
  imports = [ ./common.nix ];
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
    program = "${pkgs.lib.getExe pkgs.firefox} file:///nfs/task_description.html http://controlmachine:8080";
  };
  systemd.services.custom-nfs-mount = {
    description = "nfs mounting";
    wantedBy = [ "multi-user.target" ];
    after = [ "network-online.target" ];
    requires = [ "network-online.target" ];
    script = "${pkgs.util-linux}/bin/mount controlmachine:/nfs /nfs";
    serviceConfig.Type = "oneshot";
  };
  boot.supportedFilesystems = [ "nfs" ];
  systemd.tmpfiles.rules = [
    "d /nfs 0755 root root -"
  ];
}
