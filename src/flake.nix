{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    nix-bitcoin.url = "github:fort-nix/nix-bitcoin";
    blitz-api.url = "/home/admin/dev/api";
  };
  outputs = inputs @ {
    self,
    nixpkgs,
    nix-bitcoin,
    blitz-api,
    ...
  }: {
    nixosConfigurations.devsys = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        nix-bitcoin.nixosModules.default
        blitz-api.nixosModules.default
        ./configuration.vm.nix
      ];
    };

    nixosConfigurations.tbnix = nixpkgs.lib.nixosSystem {
      system = "aarch64-linux";
      modules = [
        nix-bitcoin.nixosModules.default
        blitz-api.nixosModules.default
        ./configuration.nix
      ];
    };
  };
}
