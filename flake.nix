{
  description = "GPD LinuxControls' Nix flake";
  nixConfig =
    {
      experimental-features = [ "nix-command" "flakes" ];
      substituters = [
        # "https://mirrors.cernet.edu.cn/nix-channels/store"
        # "https://mirrors.bfsu.edu.cn/nix-channels/store"
        "https://cache.nixos.org/"
      ];
      extra-substituters = [
        "https://cryolitia.cachix.org"
      ];
      extra-trusted-public-keys = [
        "cryolitia.cachix.org-1:/RUeJIs3lEUX4X/oOco/eIcysKZEMxZNjqiMgXVItQ8="
      ];
    };
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, rust-overlay }:
    let
      systems = [
        "x86_64-linux"
        "i686-linux"
        "x86_64-darwin"
        "aarch64-linux"
        "armv6l-linux"
        "armv7l-linux"
      ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system);
    in
    {
      devShells = forAllSystems (system: (
        let
          pkgs = import nixpkgs {
            config = {
              allowUnfree = true;
              cudaSupport = false;
            };
            inherit system;
            overlays = [
              (import rust-overlay)
            ];
          };
          rust = (pkgs.rust-bin.stable.latest.rust.override {
            extensions = [ "rust-src" ];
          });
        in
        {
          default = ((pkgs.mkShell.override { stdenv = pkgs.llvmPackages.stdenv; }) {

            buildInputs = with pkgs; [
              rust
              libusb
              pkg-config
            ];

            LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
            RUST_SRC_PATH = "${rust}/lib/rustlib/src/rust";

            shellHook = ''
              rustc --version
              cargo --version
              echo ${rust}
              exec zsh
            '';

          });
        }
      ));

      legacyPackages = forAllSystems (system: (
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [
              (import rust-overlay)
            ];
          };

        in
        {
          default = pkgs.callPackage
            (
              { lib
              , rust-bin
              , makeRustPlatform
              , pkg-config
              , libusb
              }:
              let
                rustPlatform = makeRustPlatform {
                  cargo = rust-bin.stable.latest.minimal;
                  rustc = rust-bin.stable.latest.minimal;
                };
              in
              rustPlatform.buildRustPackage rec {
                pname = "gpd-linux-controls";
                version = "unstable";

                src = lib.cleanSource ./.;

                cargoLock = {
                  lockFile = ./Cargo.lock;
                };

                nativeBuildInputs = [
                  pkg-config
                ];

                buildInputs = [
                  libusb
                ];

                postInstall = ''
                  install -Dm644 42-gpd-controls.rules $out/lib/udev/rules.d/42-gpd-controls.rules

                  $out/bin/gpd-controls gen --path $out/share/man/man1/ man
                  $out/bin/gpd-controls gen --path $out/share/zsh/site-functions/ complete zsh
                  $out/bin/gpd-controls gen --path $out/share/bash-completion/completions/ complete bash
                  $out/bin/gpd-controls gen --path $out/share/fish/vendor_completions.d/ complete fish
                  # TODO: elvish https://github.com/elves/elvish/issues/1004
                '';

                meta = with lib; {
                  description = "A reverse engineered and reference implementation of GPD WinControls";
                  homepage = "https://github.com/Cryolitia/GPD-LinuxControls";
                  license = licenses.mit;
                  maintainers = with maintainers; [ Cryolitia ];
                  mainProgram = "gpd-controls";
                };
              }
            )
            { };
        }
      ));

      packages = forAllSystems (system: nixpkgs.lib.filterAttrs (_: v: nixpkgs.lib.isDerivation v) self.legacyPackages.${system});
    };
}
