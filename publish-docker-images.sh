#!/usr/bin/env bash

set -euo pipefail

repo="arinono"
image="tu2i"
systems="x86_64-linux aarch64-linux"
manifest="$repo/$image"

echo "=== Building images"
for system in $(echo $systems); do
  echo "Building for $system"
  nix -L build .#packages.${system}.dockerImage
  docker load --input result

  echo "Tagging"
  drv=$(nix eval .#packages.${system}.dockerImage.imageTag | tr -d '"')
  docker tag "$image:$drv" "$repo/$image:$drv"
  docker tag "$image:$drv" "$repo/$image:$system"
  manifest="${manifest} $repo/$image:$drv $repo/$image:$system"
done

docker push --all-tags "$repo/$image"
docker manifest create $(echo $manifest)
docker manifest push "$repo/$image"

docker manifest rm "$repo/$image"
for image in $(docker images | grep "$image" | awk '{print $1":"$2}'); do
  docker rmi "$image"
done
