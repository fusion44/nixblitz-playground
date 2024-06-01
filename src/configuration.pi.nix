{
  config,
  lib,
  pkgs,
  ...
}: {
  imports = [
    ./hardware-configuration.pi.nix
    ./apps/bitcoind.nix
    ./apps/lnd.nix
    ./apps/blitz_api.nix
    ./apps/blitz_web.nix
    ./configuration.common.nix
  ];

  boot.loader.grub.enable = false;
  boot.loader.generic-extlinux-compatible.enable = true;

  networking.hostName = "tbnix"; # Define your hostname.

  system.stateVersion = "23.11"; # Did you read the comment?
}
