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

  outputs =
    { self
    , nixpkgs
    , utils
    , naersk
    , fenix
    ,
    }:
    utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          fenix.overlays.default
        ];
      };

      leetcode = pkgs.rustPlatform.buildRustPackage
        {
          pname = "leetcode";
          version = "0.3.12";

          src = pkgs.fetchgit {
            url = "https://github.com/MSBarbieri/leetcode-cli";
            rev = "refs/heads/master";
            sha256 = "sha256-dZv3HftqzelKre1Gd0rGD+gZgqF6YwHYY6DqZ4YLh/8=";
          };

          cargoSha256 = "sha256-0W116KxulsBibZt1SofwXuEY/4bazdYS1eZCicikQpA=";

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs = with pkgs;
            [
              openssl
              dbus
              sqlite
            ]
            ++ lib.optionals stdenv.isDarwin [ darwin.apple_sdk.frameworks.Security ];

          meta = with pkgs.lib; {
            description = "Leet your code in command-line.";
            homepage = "https://github.com/clearloop/leetcode-cli";
            licenses = licenses.mit;
            maintainers = with maintainers; [ congee ];
            mainProgram = "leetcode";
          };
        };

      naersk-lib = pkgs.callPackage naersk { };

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

    in
    {
      defaultPackage = naersk-lib.buildPackage {
        src = ./.;
        buildInputs = [
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
            leetcode
          ];

          RUST_LOG = "debug";
          nativeBuildInputs = [ pkgs.pkg-config ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
    });
}
