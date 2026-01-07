#!/bin/bash

set -e

VERSION=${1:-latest}
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
TMP_DIR=$(mktemp -d)
ARCH=$(uname -m)
OS=$(uname -s)

echo "🚀 Installing Todo CLI..."

# Determine architecture and download URL
case "$OS" in
  Darwin)
    if [ "$ARCH" = "arm64" ]; then
      FILENAME="todo-macos-arm64"
    else
      FILENAME="todo-macos-x86_64"
    fi
    ;;
  Linux)
    if [ "$ARCH" = "aarch64" ]; then
      FILENAME="todo-linux-arm64"
    else
      FILENAME="todo-linux-x86_64"
    fi
    ;;
  *)
    echo "❌ Unsupported OS: $OS"
    exit 1
    ;;
esac

# Get latest version if not specified
if [ "$VERSION" = "latest" ]; then
  RELEASE_URL=$(curl -s https://api.github.com/repos/ikbenignace/todo-cli/releases/latest | grep "browser_download_url" | grep "$FILENAME" | head -n 1 | cut -d '"' -f 4)
else
  RELEASE_URL="https://github.com/ikbenignace/todo-cli/releases/download/$VERSION/$FILENAME"
fi

if [ -z "$RELEASE_URL" ]; then
  echo "❌ Could not find release for $FILENAME"
  exit 1
fi

echo "📥 Downloading $FILENAME..."
curl -fsSL "$RELEASE_URL" -o "$TMP_DIR/todo"

echo "📦 Installing to $INSTALL_DIR..."

# Create install directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Copy binary
cp "$TMP_DIR/todo" "$INSTALL_DIR/todo"
chmod +x "$INSTALL_DIR/todo"

# Cleanup
rm -rf "$TMP_DIR"

echo "✅ Todo CLI installed successfully!"
echo ""
echo "To use it, add $INSTALL_DIR to your PATH:"
echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
echo ""
echo "Or add this to your shell config (~/.bashrc, ~/.zshrc, etc.):"
echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
echo ""
echo "Then you can use:"
echo "  todo \"your task description\""
