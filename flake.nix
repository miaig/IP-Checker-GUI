{
  description = "Flake for ip_checker";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };

      ipChecker = pkgs.rustPlatform.buildRustPackage rec {
        pname = "ip_checker";
        version = "0.1.0";

        # Use this repository as the source.
        src = self;

        cargoLock = { lockFile = ./Cargo.lock; };

        nativeBuildInputs = [ pkgs.pkg-config ];
        buildInputs = [
          pkgs.libGL
          pkgs.libxkbcommon
          pkgs.wayland
          pkgs."gdk-pixbuf"
          pkgs.graphene
          pkgs.cairo
          pkgs.pango
          pkgs.gtk4
        ];

        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
          pkgs.libGL
          pkgs.libxkbcommon
          pkgs.wayland
          pkgs."gdk-pixbuf"
          pkgs.graphene
          pkgs.cairo
          pkgs.pango
          pkgs.gtk4
        ];
      };

      devEnv = pkgs.mkShell {
        buildInputs = [
          pkgs.cargo
          pkgs.rustc
          pkgs.rust-analyzer
          pkgs.libGL
          pkgs.libxkbcommon
          pkgs.wayland
          pkgs."gdk-pixbuf"
          pkgs.graphene
          pkgs.cairo
          pkgs.pango
          pkgs.gtk4
        ];

        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
          pkgs.libGL
          pkgs.libxkbcommon
          pkgs.wayland
          pkgs."gdk-pixbuf"
          pkgs.graphene
          pkgs.cairo
          pkgs.pango
          pkgs.gtk4
        ];

        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    in {
      packages.${system} = {
        # Expose the package as "ipChecker" so it can be referenced as "ip-checker.ipChecker"
        ipChecker = ipChecker;
        # Also alias it as the default package for convenience
        default = ipChecker;
      };

      defaultPackage.${system} = ipChecker;

      devShells.${system} = {
        default = devEnv;
        dev = devEnv;
      };

      devShell.${system} = devEnv;
    };
}

