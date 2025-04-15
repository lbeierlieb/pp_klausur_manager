{ ... }: {
  virtualisation.memorySize = 2048;
  virtualisation.cores = 2;
  users.users.nixos = {
    isNormalUser = true;
    home = "/home/nixos";
    initialPassword = "nixos";
  };
  networking.firewall.enable = false;
  system.stateVersion = "24.11";
}
