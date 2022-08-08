{
  description = "codecrafters rust track redis";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";
    flake-utils.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, naersk }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          srcNoTarget = dir:
            builtins.filterSource
              (path: type: type != "directory" || builtins.baseNameOf path != "target")
              dir;

          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
          rustEnv = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default);
          naersk-lib = naersk.lib."${system}";

          src = srcNoTarget ./.;
          blog = naersk-lib.buildPackage {
            inherit src;
            name = "xeBlogChallenge2022";
            buildInputs = [
              pkgs.openssl
            ];
            remapPathPrefix = true;
          };

        in
        rec {
          devShell = pkgs.mkShell {
            buildInputs = with pkgs;
              [
                rustEnv
                openssl
                pkgconfig
                cargo-edit
                cargo-watch
              ];
          };
        });
}
