#!/bin/bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPLETION_DIR="${HOME}/.local/share/bash-completion/completions"

echo "🚀 Installing bash completions for todo..."

mkdir -p "$COMPLETION_DIR"

"$SCRIPT_DIR/../target/release/todo" completion bash > "$COMPLETION_DIR/todo"

echo "✅ Completions installed for bash"
echo "📝 Add this to your ~/.bashrc:"
echo "  export PATH=\"\$PATH:$HOME/.local/share/bash-completion/completions\""
echo ""
echo "Then restart your terminal or run:"
echo "  source ~/.bashrc"
