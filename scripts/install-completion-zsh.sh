#!/bin/bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPLETION_DIR="${HOME}/.zsh/completions"

echo "🚀 Installing zsh completions for todo..."

mkdir -p "$COMPLETION_DIR"

"$SCRIPT_DIR/../target/release/todo" completion zsh > "$COMPLETION_DIR/_todo"

echo "✅ Completions installed for zsh"
echo "📝 Add this to your ~/.zshrc:"
echo "  fpath=(\$HOME/.zsh/completions \$fpath)"
echo "  autoload -U compinit && compinit"
echo ""
echo "Then restart your terminal or run:"
echo "  source ~/.zshrc"
