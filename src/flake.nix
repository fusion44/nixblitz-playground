{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    nix-bitcoin.url = "github:fort-nix/nix-bitcoin";
    # blitz-api.url = "/home/f44/dev/blitz/api/nixosify";
    # blitz-web.url = "/home/f44/dev/blitz/web/nixosify";
    # blitz-api.url = "/home/admin/dev/api";
    # blitz-web.url = "/home/admin/dev/web";
    blitz-api.url = "github:fusion44/blitz_api/nixosify";
    blitz-web.url = "github:fusion44/raspiblitz-web/nixosify";
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    nix-bitcoin,
    blitz-api,
    blitz-web,
    ...
  }: {
    nixosConfigurations.devsys = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        nix-bitcoin.nixosModules.default
        blitz-api.nixosModules.default
        blitz-web.nixosModules.default
        ./vm/configuration.nix
      ];
    };

    nixosConfigurations.tbnix = nixpkgs.lib.nixosSystem {
      system = "aarch64-linux";
      modules = [
        nix-bitcoin.nixosModules.default
        blitz-api.nixosModules.default
        blitz-web.nixosModules.default
        ./pi/configuration.nix
      ];
    };
  };
}
