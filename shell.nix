{ pkgs ? (
    let
      inherit (builtins) fetchTree fromJSON readFile;
      inherit ((fromJSON (readFile ./flake.lock)).nodes) nixpkgs gomod2nix;
    in
    import (fetchTree nixpkgs.locked) {
      overlays = [
        (import "${fetchTree gomod2nix.locked}/overlay.nix")
      ];
    }
  )
, mkGoEnv ? pkgs.mkGoEnv
, gomod2nix ? pkgs.gomod2nix
, pre-commit-hooks
}:

let
  goEnv = mkGoEnv { pwd = ./.; };
  pre-commit-check = pre-commit-hooks.lib.${pkgs.system}.run {
    src = ./.;
    hooks = {
      gofmt.enable = true;
      # golangci-lint = {
      #   enable = true;
      #   name = "golangci-lint";
      #   description = "Lint application";
      #   files = "\.go$";
      #   entry = "${pkgs.golangci-lint}/bin/golangci-lint run --new-from-rev HEAD --fix ./organizations/...";
      #   require_serial = true;
      #   pass_filenames = false;
      # };
      goimports = {
        enable = true;
        name = "goimports";
        description = "Format application";
        files = "\.go$";
        entry =
          let
            script = pkgs.writeShellScript "precommit-goimports" ''
              set -e
              failed=false
              for file in "$@"; do
                # redirect stderr so that violations and summaries are properly interleaved.
                if ! ${pkgs.gotools}/bin/goimports -l -d "$file" 2>&1; then
                  failed=true
                fi
              done
              if [[ $failed == "true" ]]; then
                exit 1
              fi
            '';
          in
          builtins.toString script;
      };
    };
  };
in
pkgs.mkShell {
  inherit (pre-commit-check) shellHook;

  # shellHook = with pkgs; ''
  #   export XDG_DATA_DIRS=${gsettings-desktop-schemas}/share/gsettings-schemas/${gsettings-desktop-schemas.name}:${gtk3}/share/gsettings-schemas/${gtk3.name}:$XDG_DATA_DIRS;
  #   export GIO_MODULE_DIR="${pkgs.glib-networking}/lib/gio/modules/";
  # '';

  packages = [
    goEnv
    gomod2nix
    pkgs.delve
    pkgs.go_1_22
    pkgs.go-junit-report
    pkgs.golangci-lint
    pkgs.goreleaser
    pkgs.go-task
    pkgs.gotestsum
    pkgs.gotools
    pkgs.nodejs
    pkgs.overmind
    pkgs.stdenv.cc.cc.lib
    pkgs.yarn
  ];

  buildInputs = [
    pkgs.gtk3
    pkgs.webkitgtk
  ];

  nativeBuildInputs = [
    pkgs.pkg-config
  ];

  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.stdenv.cc.cc pkgs.gtk3 pkgs.webkitgtk ];
  # PKG_CONFIG_PATH = pkgs.lib.makeLibraryPath [ pkgs.gtk3 pkgs.webkitgtk ];
}
