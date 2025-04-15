{ pp_klausur_manager }: { pkgs, ... }: {
  imports = [ ./common.nix ];
  services.cage = {
    enable = true;
    user = "nixos";
    program = "${pkgs.lib.getExe pkgs.alacritty} -o font.normal.family=\\\"\"JetBrainsMono Nerd Font\\\"\" -e ppmngr_launch";
  };
  fonts.packages = [ pkgs.nerd-fonts.jetbrains-mono ];
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
    "d /nfs 0755 nixos users -"
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
}
