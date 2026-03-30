{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      pkgs = nixpkgs.legacyPackages."x86_64-linux";
    in
    {
      devShells."x86_64-linux".default = pkgs.mkShell {
        buildInputs = [
          pkgs.cargo
          pkgs.rustc
          pkgs.rustfmt
          pkgs.clippy
          pkgs.rust-analyzer
        ];

        env = {
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
      };

      packages."x86_64-linux".default = pkgs.rustPlatform.buildRustPackage {
        name = "flow_state";
        src = ./.;
        cargoHash = "sha256-udJerd8ojnqPu2GcwLnhuKTYWsh2vMqJejayjaq3IHY=";
      };
    };
}
