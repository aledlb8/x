#!/bin/bash
set -e

ARCH=$(uname -m)
OS=$(uname -s)
URL=""

if [[ "$OS" == "Linux" ]]; then
  URL="https://github.com/aledlb8/x/releases/download/x-linux-x64"
elif [[ "$OS" == "Darwin" ]]; then
  URL="https://github.com/aledlb8/x/releases/download/x-macos-x64"
else
  echo "Unsupported OS"
  exit 1
fi

echo "Downloading CLI..."
curl -L $URL -o /usr/local/bin/x
chmod +x /usr/local/bin/x

echo "✅ Installed! Run 'x --help' to get started."