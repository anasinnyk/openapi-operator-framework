{ pkgs, ... }:

{
  packages = with pkgs; [
    git
    tilt
    podman
    docker
    k9s
    kubectl
    kind
    vacuum-go
  ];

  languages.rust.enable = true;

  # env.KIND_EXPERIMENTAL_PROVIDER = "podman";
}
