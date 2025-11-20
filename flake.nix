{
  inputs.tooling.url = "github:mox-desktop/tooling";

  outputs =
    { tooling, ... }:
    tooling.lib.mkMoxFlake {
      devShells = tooling.lib.forAllSystems (pkgs: {
        default = pkgs.mkShell.override { stdenv = pkgs.clangStdenv; } (
          pkgs.lib.fix (finalAttrs: {
            buildInputs = builtins.attrValues {
              inherit (pkgs)
                nixd
                htmlhint
                html-tidy
                html5validator
                superhtml
                vscode-css-languageserver
                css-checker
                rustToolchain
                rust-analyzer-unwrapped
                tailwindcss
                tailwindcss-language-server
                ;
            };
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath finalAttrs.buildInputs;
            RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
          })
        );
      });

      packages = tooling.lib.forAllSystems (pkgs: {
        default = pkgs.callPackage ./nix/package.nix {
          rustPlatform = pkgs.makeRustPlatform {
            cargo = pkgs.rustToolchain;
            rustc = pkgs.rustToolchain;
          };
        };
      });
    };
}
