{ pkgs, pre-commit-hooks, system }:

with pkgs;

let
  pre-commit-check = pre-commit-hooks.lib.${system}.run {
    src = ./.;
    hooks = {
      cargo-check.enable = true;
      nixpkgs-fmt.enable = true;
      nix-linter.enable = true;
      rustfmt.enable = true;
    };
    # generated files
    excludes = [
    ];
  };
in
mkShell {
  buildInputs = [
    cacert
    cargo-outdated
    cargo-edit
    openssl
    pkgconfig
    pre-commit
    rustfmt
    (rust-bin.nightly."2022-04-15".default.override {
      extensions = [ "rust-src" "rust-analyzer-preview" ];
    })
  ];

  shellHook = ''
    ${pre-commit-check.shellHook}
  '';
}
