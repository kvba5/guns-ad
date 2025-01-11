{ pkgs, ... }: {
  channel = "stable-24.11";

  packages = [
    pkgs.pkg-config
    pkgs.gcc
    pkgs.rustup
    pkgs.openssl
    pkgs.openssl.dev
  ];

  idx = {
    extensions = [
      "rust-lang.rust-analyzer"
      "tamasfe.even-better-toml"
    ];


    workspace = {
      onCreate = {
        rustup-default = "rustup default stable";
      };
    };
  };
}
