#!/bin/bash

# First we need to get the platform that we're downloading the binary for
os=$(uname -s | tr '[:upper:]' '[:lower:]')
arch=$(uname -m)

if [[ "$os" == "linux" ]]; then
  if [[ "$arch" == "x86_64" ]]; then
    platform="linux-x86_64"
  elif [[ "$arch" == "aarch64" || "$arch" == "arm64" ]]; then
    platform="linux-aarch64"
  else
    platform="linux-$arch"
  fi
elif [[ "$os" == "darwin" ]]; then
  if [[ "$arch" == "arm64" ]]; then
    platform="macOS-aarch64"
  else
    platform="macOS-$arch"
  fi
else
  platform="$os-$arch"
fi

echo "Downloading for $platform"

# Now we get the latest tag so that we can pull that binary

latest_url=$(curl -Ls -o /dev/null -w %{url_effective} https://github.com/LynchSKM/dockem-rs/releases/latest)
latest_tag=$(echo $latest_url | grep -oP "[0-9]+\.[0-9]+\.[0-9]+$")

echo "Latest tag is $latest_tag"

# Now we download the binary
curl -L -o dockem "https://github.com/LynchSKM/dockem-rs/releases/download/v$latest_tag/dockem-rs-v$latest_tag-$platform"
chmod +x dockem-rs
