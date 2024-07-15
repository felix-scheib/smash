#!/bin/sh

if [[ $# -ne 1 ]]; then
    echo "No architecture specified, using x86_64"
    ARCH="x86_64"
  else
    ARCH=$1
fi

release=$(curl --silent "https://api.github.com/repos/hermit-os/loader/releases/latest" | jq -r ".tag_name")
echo "Latest release: $release"

wget https://github.com/hermit-os/loader/releases/download/$release/hermit-loader-$ARCH
