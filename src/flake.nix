{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    nix-bitcoin.url = "github:fort-nix/nix-bitcoin";
  };
  outputs = inputs @ {
    self,
    nixpkgs,
    nix-bitcoin,
    ...
  }: {
    nixosConfigurations.tbnix = nixpkgs.lib.nixosSystem {
      # NOTE: Change this to aarch64-linux if you are on ARM
      system = "aarch64-linux";
      modules = [
        nix-bitcoin.nixosModules.default
        ./configuration.nix
      ];
    };
  };
}
