{
  perSystem =
    { config
    , self'
    , pkgs
    , ...
    }:
    {
      devShells.default = pkgs.mkShell {
        packages = [
          # code formatting
          config.treefmt.build.wrapper

          # tasks and automation
          pkgs.just
          pkgs.jq
          pkgs.nix-update

          # rust dev
          pkgs.rust-analyzer
          pkgs.cargo-watch
          pkgs.clippy

          # crane does not have this in nativeBuildInputs
          pkgs.rustc
        ] ++ config.packages.lightning-knd.nativeBuildInputs
        ++ config.packages.lightning-knd.tests.nativeBuildInputs;
        inherit (config.packages.lightning-knd) buildInputs;
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        RUST_BACKTRACE = 1;
        inherit (self'.packages.lightning-knd) nativeBuildInputs;
      };
    };
}
