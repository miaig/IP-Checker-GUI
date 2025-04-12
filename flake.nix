{
  description = "A simple GTK4 GUI to check your IP address.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        ipChecker = pkgs.rustPlatform.buildRustPackage {
          pname = "ipChecker";
          version = "0.1.0";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [
            gtk4
            glib
            cairo
            pango
            gdk-pixbuf
          ];
        };
      in {
        packages.ipChecker = ipChecker;
        defaultPackage = ipChecker;
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
            gtk4
            glib
            cairo
            pango
            gdk-pixbuf
            rustc
            cargo
          ];
        };
      });
}

