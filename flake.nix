{
  description = "vulkan rust dev environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      llvm = pkgs.llvmPackages;

    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell {
        packages = with pkgs; [
          rustup
          glfw
          cmake
          clang
          wayland

          llvm.clang
          llvm.libclang
        ];

        LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
          libGL
          xorg.libXrandr
          xorg.libXinerama
          xorg.libXcursor
          xorg.libXi
        ];

        buildInputs = with pkgs; [
          libxkbcommon
          wayland
          shaderc
          cloc
        ];
        shellHook = ''

          export SHELL=${pkgs.zsh}/bin/zsh
                  export LIBCLANG_PATH=${llvm.libclang.lib}/lib
          export LD_LIBRARY_PATH=${pkgs.wayland}/lib:$LD_LIBRARY_PATH
          export LD_LIBRARY_PATH=${pkgs.libxkbcommon}/lib:$LD_LIBRARY_PATH
          zsh
        '';
      };
    };
}
