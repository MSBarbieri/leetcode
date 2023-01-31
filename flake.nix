{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
    fenix,
  }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          fenix.overlays.default
        ];
      };
      naersk-lib = pkgs.callPackage naersk {};
      watch_script = pkgs.writeShellScriptBin "watch-script" ''
        #!/bin/bash
        cargo watch -x "fmt" -s "leetcode t $(\cat ./pick_test)"
     '';
      pick_script = pkgs.writeShellScriptBin "pick" ''
        #!/bin/bash
        if [[ -f ./pick_test ]] && [[ -z $1 ]]; then
          echo "already exists"
        else
          if [[ ! -z $1 ]]; then
            echo $1 > ./pick_test
            leetcode e $1
          else
            leetcode p | awk 'NR==2 {print $1}' | cut -d "[" -f 2 | cut -d "]" -f 1 > ./pick_test
          fi 
        fi
      '';
    in {
      defaultPackage = naersk-lib.buildPackage {
        src = ./.;
        buildInputs = with pkgs; [
        ];
        RUST_LOG = "trace";
      };


      devShell = with pkgs;
        mkShell {
          buildInputs = [
            (pkgs.fenix.complete.withComponents [
              "cargo"
              "clippy"
              "rust-src"
              "rustc"
              "rustfmt"
            ])
            watch_script
            pick_script
            cargo-watch
            pre-commit
          ];
          RUST_LOG = "debug";
          nativeBuildInputs = [pkgs.pkg-config];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
    });
}
