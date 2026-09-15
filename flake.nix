{
  description = "Rust template";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    microvm.url = "github:microvm-nix/microvm.nix";
    microvm.inputs.nixpkgs.follows = "nixpkgs";
    osmium.url = "git+ssh://gitea@gitea.chiliahedron.wtf:6022/chiliahedron/osmium.git";
    osmium.inputs.nixpkgs.follows = "nixpkgs";
    osmium.inputs.microvm.follows = "microvm";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      microvm,
      osmium,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
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
            git
            makeWrapper
            pkg-config
          ];

          postInstall = ''
            wrapProgram $out/bin/rust-template \
              --set-default SCEPTRE_GIT ${pkgs.git}/bin/git
          '';

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

        checks.repository-creation-gitea = import ./tests/repository-creation-gitea.nix {
          inherit (pkgs) lib;
          inherit microvm osmium;
          pkgs = nixpkgs.legacyPackages.${system};
          sceptre = package;
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
      }
    );
}
