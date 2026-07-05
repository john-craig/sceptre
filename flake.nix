{
  description = "Rust template";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        toolchain = pkgs.rust-bin.stable.latest.default;
        rustPlatform = pkgs.makeRustPlatform {
          cargo = toolchain;
          rustc = toolchain;
        };

        package = rustPlatform.buildRustPackage {
          pname = "rust-template";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          meta = with pkgs.lib; {
            description = "Rust template project packaged as a Nix flake";
            mainProgram = "rust-template";
            platforms = platforms.unix;
          };
        };

        testApp = pkgs.writeShellApplication {
          name = "rust-template-tests";
          runtimeInputs = [ toolchain ];
          text = ''
            cargo test "$@"
          '';
        };
      in
      {
        packages.default = package;
        packages.rust-template = package;

        apps.default = {
          type = "app";
          program = "${package}/bin/rust-template";
        };

        apps.rust-template-tests = {
          type = "app";
          program = "${testApp}/bin/rust-template-tests";
        };

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            cargo-watch
            clippy
            rust-analyzer
            rustfmt
            toolchain
          ];
        };

        formatter = pkgs.nixfmt-rfc-style;
      });
}
