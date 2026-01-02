{ pkgs, ... }:

{
  packages = [ pkgs.git ];
  languages.rust = {
    enable = true;
    version = "1.91.1";
    channel = "stable";
    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" "rust-src" ];
  };

  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running tests"
    git --version | grep --color=auto "${pkgs.git.version}"
  '';

}
