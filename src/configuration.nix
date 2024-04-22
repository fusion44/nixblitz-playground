{
  config,
  lib,
  pkgs,
  ...
}: {
  imports = [
    ./hardware-configuration.nix
    ./apps/bitcoind.nix
    ./apps/lnd.nix
  ];

  boot.loader.grub.enable = false;
  boot.loader.generic-extlinux-compatible.enable = true;

  nix.extraOptions = "experimental-features = nix-command flakes";

  networking.hostName = "tbnix"; # Define your hostname.
  time.timeZone = "Europe/Berlin";

  i18n.defaultLocale = "en_US.UTF-8";
  console = {
    font = "Lat2-Terminus16";
    useXkbConfig = true; # use xkb.options in tty.
  };

  users.users.admin = {
    isNormalUser = true;
    extraGroups = ["wheel"]; # Enable ‘sudo’ for the user.
    packages = with pkgs; [
    ];
  };

  environment.systemPackages = with pkgs; [
    neovim
    wget
  ];

  # Enable the OpenSSH daemon.
  services.openssh = {
    enable = true;
    ports = [22];
    settings = {
      PasswordAuthentication = true;
      AllowUsers = ["admin"];
      UseDns = true;
      X11Forwarding = false;
      PermitRootLogin = "prohibit-password";
    };
  };

  networking.firewall.allowedTCPPorts = [22];

  system.stateVersion = "23.11"; # Did you read the comment?
}
