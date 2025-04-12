{
  description = "Flake for ip_checker";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/24.11";
  };

  outputs = { self, nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in {
      # Define the package built using Cargo
      packages.${system}.ip_checker = pkgs.rustPlatform.buildRustPackage rec {
        pname = "ip_checker";
        version = "0.1.0";

        # 'src = self' means we use the repository as the source code.
        src = self;

        # Use your existing Cargo.lock file
        cargoLock = {
          lockFile = ./Cargo.lock;
        };

        # Optional: include pkg-config and your required library dependencies
        nativeBuildInputs = [ pkgs.pkg-config ];
        buildInputs = [
          pkgs.libGL
          pkgs.libxkbcommon
          pkgs.wayland
        ];

        # Make sure the built binary finds the necessary libraries at runtime
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
          pkgs.libGL
          pkgs.libxkbcommon
          pkgs.wayland
        ];
      };

      # You can also set the default package to be your build target
      defaultPackage = packages.${system}.ip_checker;
    };
}

