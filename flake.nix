{
  inputs.nixpkgs.url = "nixpkgs/nixos-unstable";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";
  inputs.rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

  outputs = { nixpkgs, rust-overlay, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        config.allowUnfree = true;

        overlays = [ (import rust-overlay) ];
      };
    in {
      devShells.${system}.default = pkgs.mkShell {
        name = "core-utils";
        packages = with pkgs; [
          (rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" ];
          })
          rust-analyzer
          zsh
        ];
      };

    };
}
