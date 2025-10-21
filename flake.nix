{
  description = "Silly Kernel Development Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgsBase = nixpkgs.legacyPackages.${system};
        pkgs = pkgsBase.extend (import rust-overlay);

        rustToolchain = with pkgs; rust-bin.stable.latest.default.override {
          targets = [ "aarch64-unknown-none" ];
          extensions = [ "rust-src" ];
        };
      in
      {
        devShells.default = pkgs.pkgsCross.aarch64-embedded.mkShell {
          name = "silly-kernel";
          packages = with pkgs; [
            rustToolchain
            qemu
            gdb
            dtc
          ];

        };
      });
}
