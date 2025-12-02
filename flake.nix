{
  description = "Advent of Code xxxx in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # rust-bin.stable."1.87.0".default
            rust-bin.stable.latest.default
            cargo
            rustc
            clippy
            rustfmt
            rust-analyzer
          ];

          shellHook = ''
            echo "🎄 Advent of Code xxxx - Rust Development Environment"
            echo ""
            echo "Available commands:"
            echo "  cargo build --release          # Build optimized binary"
            echo "  cargo run --release -- --time  # Run all days"
            echo "  cargo run --release -- 5       # Run day 5"
            echo "  cargo test                     # Run all tests"
            echo "  cargo test day05               # Test specific day"
            echo "  cargo clippy                   # Lint code"
            echo "  cargo fmt                      # Format code"
            echo "  cargo watch -x check           # Watch for changes"
            echo ""
            echo "Happy coding! 🦀"
          '';
        };

        # For compatibility with older nix versions
        devShell = self.devShells.${system}.default;
      }
    );
}
