{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.11";
  };

  outputs =
    { self, nixpkgs }:
    let
      overlay = final: prev: {
        kibadda = (prev.kibadda or { }) // {
          work = final.pkgs.rustPlatform.buildRustPackage {
            name = "work";
            cargoHash = "sha256-rFMraTvcvgGC1W0ZF34uv10DFE29tl+yoLmIIWNXYWg=";
            src = self;

            nativeBuildInputs = [
              final.pkgs.pkg-config
            ];

            buildInputs = [
              final.pkgs.openssl
            ];
          };
        };
      };

      system = "x86_64-linux";

      pkgs = import nixpkgs {
        inherit system;
        overlays = [ overlay ];
      };
    in
    {
      packages.${system}.default = pkgs.kibadda.work;

      devShells.${system}.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        buildInputs = with pkgs; [
          rustc
          cargo
          rustfmt
          clippy
          rust-analyzer
          openssl
        ];

        RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
      };

      overlays.default = overlay;
    };
}
