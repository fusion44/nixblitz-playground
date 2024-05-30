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
  ];

  boot.loader.grub.enable = false;
  boot.loader.generic-extlinux-compatible.enable = true;

  nix.extraOptions = "experimental-features = nix-command flakes";

  nixpkgs.config.allowUnfree = true;
  networking.hostName = "tbnix"; # Define your hostname.
  time.timeZone = "Europe/Berlin";

  i18n.defaultLocale = "en_US.UTF-8";
  console = {
    font = "Lat2-Terminus16";
    useXkbConfig = true; # use xkb.options in tty.
  };

  users = {
    defaultUserShell = pkgs.nushell;
    users.admin = {
      isNormalUser = true;
      extraGroups = ["wheel"]; # Enable ‘sudo’ for the user.
      packages = with pkgs; [];
      openssh.authorizedKeys.keys = [
        "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIC7M6/mq5kcNEjSiUrb8syQT+Y9uY4AHdHoWITIQ463Q some.fusion@gmail.com"
      ];
    };
  };

  environment.systemPackages = with pkgs; [
    bat
    bottom
    fzf
    git
    neovim
    ripgrep
  ];

  programs = {};

  services = {
    openssh = {
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

    redis.servers."".enable = true;
  };

  networking.firewall.allowedTCPPorts = [22];

  system.stateVersion = "23.11"; # Did you read the comment?
}
