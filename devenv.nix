{ pkgs, lib, config, inputs, ... }:

{
  packages = [
    pkgs.git
    pkgs.cargo-autoinherit
    pkgs.cargo-expand
  ];
  languages.rust = {
    enable = true;
  };
}
