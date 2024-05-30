{ config
, lib
, pkgs
, ...
}: {
  imports = [
    ./apps/bitcoind.nix
    ./apps/lnd.nix
    ./apps/blitz_api.nix
  ];

  boot.loader.grub.enable = false;
  boot.loader.generic-extlinux-compatible.enable = true;
  boot.loader.efi.canTouchEfiVariables = true;

  nix.extraOptions = "experimental-features = nix-command flakes";

  nixpkgs.config.allowUnfree = true;
  networking.hostName = "tbnix_vm"; # Define your hostname.
  time.timeZone = "Europe/Berlin";

  i18n.defaultLocale = "en_US.UTF-8";
  console = {
    font = "Lat2-Terminus16";
    useXkbConfig = true; # use xkb.options in tty.
  };

  virtualisation.vmVariant = {
    # following configuration is added only when building VM with build-vm
    virtualisation = {
      memorySize = 2048; # Use 2048MiB memory.
      cores = 3;
      graphics = false;
    };
  };

  users = {
    defaultUserShell = pkgs.nushell;
    users.admin = {
      initialPassword = "test";
      isNormalUser = true;
      extraGroups = [ "wheel" ]; # Enable ‘sudo’ for the user.
      packages = with pkgs; [ ];
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

  programs = { };

  services = {
    openssh = {
      enable = true;
      ports = [ 22 ];
      settings = {
        PasswordAuthentication = true;
        AllowUsers = [ "admin" ];
        UseDns = true;
        X11Forwarding = false;
        PermitRootLogin = "prohibit-password";
      };
    };

    redis.servers."".enable = true;
  };

  networking.firewall.allowedTCPPorts = [ 22 ];

  system.stateVersion = "23.11"; # Did you read the comment?
}
