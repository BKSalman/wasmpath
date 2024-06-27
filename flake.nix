{
  description = "basic rust development evnvironment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    wit-deps.url = "github:bytecodealliance/wit-deps";
  };

  outputs = {nixpkgs, rust-overlay, wit-deps, ...}:
      let 
        system = "x86_64-linux";
        pkgs = import nixpkgs { inherit system; overlays = [ rust-overlay.overlays.default ]; };
        libPath =  with pkgs; lib.makeLibraryPath [
          libxkbcommon
          vulkan-loader
          libGL
          fontconfig
          freetype
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          wayland
        ];
      in
    with pkgs; {
      devShells.${system}.default = mkShell.override {
        stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.clangStdenv;
      } {

          packages = [
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-std" "rust-src" "rust-analyzer" ];
              targets = [ "wasm32-wasi" ];
            })
            wit-deps.packages.${system}.wit-deps
            wabt
            wasm-tools
          ];
          
          nativeBuildInputs = [
            openssl
            pkg-config
          ];
          
          buildInputs = [
            fontconfig
            freetype

            vulkan-loader
            libGL

            libxkbcommon
            wayland

            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            xorg.libX11
          ];

          LD_LIBRARY_PATH = "${libPath}";
        };

      formatter.x86_64-linux = legacyPackages.${system}.nixpkgs-fmt;
    };
}

