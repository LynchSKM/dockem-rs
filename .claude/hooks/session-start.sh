#!/bin/bash
set -euo pipefail

if [ "${CLAUDE_CODE_REMOTE:-}" != "true" ]; then
  exit 0
fi

apt-get update -qq 2>/dev/null || true
apt-get install -y -qq --no-install-recommends \
  build-essential \
  pkg-config \
  libssl-dev \
  perl \
  make \
  xz-utils \
  git-flow \
  > /dev/null 2>&1

npm install -g @go-task/cli > /dev/null 2>&1

if ! command -v brrelease &> /dev/null; then
  BRRELEASE_URL="https://github.com/kerren/brrelease/releases/download/v1.14.3/brrelease-v1.14.3-f5c6244-linux-x64.tar.xz"
  tmp_dir=$(mktemp -d)
  curl -sL "$BRRELEASE_URL" -o "$tmp_dir/brrelease.tar.xz"
  tar -xf "$tmp_dir/brrelease.tar.xz" -C "$tmp_dir"
  cp -r "$tmp_dir/brrelease" /usr/local/lib/brrelease
  ln -sf /usr/local/lib/brrelease/bin/brrelease /usr/local/bin/brrelease
  rm -rf "$tmp_dir"
fi

cargo clippy --version >/dev/null 2>&1 || rustup component add clippy

cd "$CLAUDE_PROJECT_DIR/cli"
cargo build 2>&1
