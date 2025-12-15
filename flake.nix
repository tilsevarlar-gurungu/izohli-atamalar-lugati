{
  inputs.nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0";

  outputs =
    { self, ... }@inputs:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];

      forEachSupportedSystem =
        f:
        inputs.nixpkgs.lib.genAttrs supportedSystems (
          system:
          f {
            inherit system;
            pkgs = import inputs.nixpkgs {
              inherit system;
              config.allowUnfree = true;
            };
          }
        );
    in
    {
      devShells = forEachSupportedSystem (
        {
          pkgs,
          system,
        }:
        {
          default = pkgs.mkShellNoCC {
            packages = with pkgs; [
              self.formatter.${system}
              mdbook
            ];

            shellHook = ''
              echo "============================================================="
              echo "Kitob ustida ishlash uchun quyidagi buyruqni ishga tushuring:"
              echo "mdbook serve --open"
              echo "============================================================="
            '';
          };
        }
      );

      packages = forEachSupportedSystem (
        { pkgs, ... }:
        {
          default = pkgs.stdenv.mkDerivation {
            name = "izohli-atamalar-lugati";
            src = pkgs.lib.cleanSource ./.;
            nativeBuildInputs = with pkgs; [
              mdbook
            ];
            buildPhase = ''
              mdbook build -d $out
            '';
          };
        }
      );

      formatter = forEachSupportedSystem ({ pkgs, ... }: pkgs.nixfmt-rfc-style);
    };
}
