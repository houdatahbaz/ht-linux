{
  description = "Linux System Center - TUI system monitor";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            pkg-config
            openssl
            nushell
          ];

          shellHook = ''
            echo ""
            echo "╭─────────────────────────────────────╮"
            echo "│   Linux System Center Dev Shell     │"
            echo "├─────────────────────────────────────┤"
            echo "│  Available commands:                │"
            echo "│    cargo run     - Run the app      │"
            echo "│    cargo build   - Build the app    │"
            echo "│    cargo test    - Run tests        │"
            echo "│    cargo check   - Check for errors │"
            echo "╰─────────────────────────────────────╯"
            echo ""
            exec ${pkgs.nushell}/bin/nu
          '';
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "ht-linux";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };
      }
    );
}
