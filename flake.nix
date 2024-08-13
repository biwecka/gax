{
    inputs = {
        # Get nixpkgs. Check https://status.nixos.org for newer versions.
        nixpkgs.url = "github:nixos/nixpkgs/nixos-24.05";

        # Get nixpkgs-unstable
        nixpkgs-unstable.url = "github:nixos/nixpkgs/nixos-unstable";

        # Flake-Utils
        flake-utils.url = "github:numtide/flake-utils";

        # Rust Overlay
        rust-overlay = {
            url = "github:oxalica/rust-overlay";
            inputs = {
                nixpkgs.follows = "nixpkgs";
                # flake-utils.follows = "flake-utils";
            };
        };
    };

    outputs = { self, nixpkgs, flake-utils, nixpkgs-unstable, rust-overlay }:
        flake-utils.lib.eachDefaultSystem (system:
            let
                # Define an overlay for the unstable nixpkgs channel.
                nixpkgs-unstable-overlay = final: prev: {
                    unstable = import nixpkgs-unstable {
                        inherit system;
                    };
                };

                # Define pkgs as nixpkgs with some additional overlays
                pkgs = import nixpkgs {
                    inherit system;

                    # Define overlays
                    overlays = [
                        # Rust Overlay
                        (import rust-overlay)

                        # Nixpkgs unstable overlay
                        nixpkgs-unstable-overlay
                    ];
                };

                # Load Rust toolchain from file
                rustToolchain = pkgs.pkgsBuildHost.rust-bin
                    .fromRustupToolchainFile ./rust-toolchain.toml;

                ################################################################
                # Build dependencies
                nativeBuildInputs = with pkgs; [
                    # Rust Toolchain
                    rustToolchain

                    # Additional "cargo" commands
                    unstable.cargo-watch        # auto re-compile
                    # unstable.cargo-expand       # macro expansion/inspection
                    # unstable.cargo-audit        # dependency vulnerability check
                    # unstable.cargo-udeps        # dependency inspection
                    # unstable.cargo-machete      # -- " --
                    # unstable.cargo-depgraph     # -- " --
                    # unstable.cargo-deny         # -- " --

                    # Graphviz: Graph visualization library
                    # graphviz                    # for use with "cargo-depgraph"

                    # Linker "mold" (alternative to the default 'cc')
                    unstable.mold

                    # Diesel CLI
                    # unstable.diesel-cli

                    # To find system libraries to link against (e.g. libpqxx)
                    pkg-config

                    # GNU "make" to execute makefiles
                    gnumake

                    # Tokei: Count lines of code (LoC)
                    tokei

                    # Programs for load testing
                    # oha

                    # Pre-Commit (Framework for managing pre-commit hooks)
                    # pre-commit


                    # Rerun
                    # unstable.rerun
                ];

                ################################################################
                # Runtime dependencies
                buildInputs = with pkgs;
                [
                    freetype
                    fontconfig
                ];

                ################################################################
            in
                with pkgs;
            {
                devShells.default = mkShell {
                    inherit nativeBuildInputs buildInputs;

                    LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
                        # Dependencies of "plotters"
                        freetype
                        fontconfig

                        libxkbcommon
                        libGL

                        wayland

                        xorg.libXcursor
                        xorg.libXrandr
                        xorg.libXi
                        xorg.libX11
                    ];
                };
            }
        );
}
