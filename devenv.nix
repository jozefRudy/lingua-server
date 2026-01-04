{ pkgs, ... }:

{
  packages = [
    pkgs.git
  ];

  # languages.rust = {
  #   enable = true;
  #   version = "1.91.1";
  #   channel = "stable";
  #   components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" "rust-src" ];
  # };

  languages.rust = {
    enable = true;
    channel = "stable";
  };


}
