{ pkgs, ... }:
{
  packages = with pkgs; [
    just
    git

    cargo-insta
  ];

  languages = {
    rust = {
      enable = true;
      channel = "nightly";
      version = "latest";
      mold.enable = true;
    };
    javascript = {
      enable = true;
      pnpm.enable = true;
      npm.enable = true;
    };
    python = {
      enable = true;
      uv.enable = true;
    };
    nix = {
      enable = true;
    };
  };
}
