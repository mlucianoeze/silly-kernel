{
  description = "Silly Kernel Development Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        # devShells.default = pkgs.mkShellNoCC {
        devShells.default = pkgs.pkgsCross.aarch64-embedded.mkShell {
          name = "silly-kernel";
          packages = with pkgs; [
            qemu
            gdb
            # rustc
            # cargo
            # rustfmt
            # clippy
            # rust-analyzer
          ];

        };
      });
}
