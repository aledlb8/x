#!/bin/bash
set -e

ARCH=$(uname -m)
OS=$(uname -s)
URL=""

if [[ "$OS" == "Linux" ]]; then
  URL="https://github.com/aledlb8/x/releases/download/2.0/x-linux"
elif [[ "$OS" == "Darwin" ]]; then
  URL="https://github.com/aledlb8/x/releases/download/2.0/x-macos"
else
  echo "Unsupported OS"
  exit 1
fi

echo "Downloading CLI..."
curl -L $URL -o /usr/local/bin/x

echo "Making it executable..."
chmod +x /usr/local/bin/x

echo "Adding to PATH..."
export PATH=$PATH:/usr/local/bin
echo 'export PATH=$PATH:/usr/local/bin' >> ~/.bashrc
echo 'export PATH=$PATH:/usr/local/bin' >> ~/.zshrc

echo "✅ Installed! Run 'x --help' to get started."