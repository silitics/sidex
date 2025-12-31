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
      channel = "stable";
      version = "latest";
      mold.enable = true;
    };
    nix = {
      enable = true;
    };
  };
}
