{
  config,
  lib,
  pkgs,
  ...
}: {
  imports = [
    ./apps/bitcoind.nix
    ./apps/lnd.nix
    ./apps/blitz_api.nix
    ./apps/blitz_web.nix
    ./configuration.common.nix
  ];

  boot.loader.grub.enable = false;
  boot.loader.generic-extlinux-compatible.enable = true;
  boot.loader.efi.canTouchEfiVariables = true;

  networking.hostName = "tbnix_vm"; # Define your hostname.

  virtualisation.vmVariant = {
    # following configuration is added only when building VM with build-vm
    virtualisation = {
      memorySize = 2048; # Use 2048MiB memory.
      cores = 3;
      graphics = false;
    };
  };

  system.stateVersion = "23.11"; # Did you read the comment?
}
