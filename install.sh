#!/bin/bash
set -e

URL="https://github.com/YOUR_USERNAME/YOUR_REPO/releases/latest/download/x-cli.zip"
INSTALL_DIR="/usr/local/bin"

echo "Downloading CLI..."
curl -L $URL -o /tmp/x-cli.zip

echo "Extracting..."
unzip -o /tmp/x-cli.zip -d $INSTALL_DIR

echo "Making it executable..."
chmod +x $INSTALL_DIR/x

echo "✅ Installed! Run 'x --help' to get started."