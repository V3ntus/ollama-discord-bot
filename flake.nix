{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk {};

        rust-toolchain = pkgs.symlinkJoin {
          name = "rust-toolchain";
          paths = [pkgs.rustc pkgs.cargo pkgs.rustPlatform.rustcSrc];
        };
      in {
        # For `nix build` & `nix run`:
        defaultPackage = naersk'.buildPackage {
          src = ./.;
        };
        # For `nix develop`:
        devShell = pkgs.mkShell {
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          nativeBuildInputs = with pkgs; [ rust-toolchain clippy openssl_3_3 pkg-config rustfmt ];
        };
      }
    );
}
