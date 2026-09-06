{ pkgs, ... }:

{
  packages = with pkgs; [
    git
    tilt
    podman
    k9s
    vacuum-go
  ];

  languages.rust.enable = true;

  env.KIND_EXPERIMENTAL_PROVIDER = "podman";
}
