#!/bin/zsh

INSTALL_DIR="$HOME/.note-zsh"
BIN_DIR="$HOME/.local/bin"
ARCHIVE_URL="https://github.com/yourname/Note-zsh/releases/latest/download/note-zsh.tar.gz"

echo "🔄 Updating Note-zsh..."
curl -L "$ARCHIVE_URL" | tar -xz -C "$INSTALL_DIR"

cp "$INSTALL_DIR/note-rust" "$BIN_DIR/"
chmod +x "$BIN_DIR/note-rust"

echo "✅ Note-zsh updated!"

