#!/bin/zsh

INSTALL_DIR="$HOME/.note-zsh"
BIN_DIR="$HOME/.local/bin"
ARCHIVE_URL="https://github.com/K10-K10/Note-zsh/releases/latest/download/Note-zsh.tar.gz"

mkdir -p "$INSTALL_DIR" "$BIN_DIR"

echo "Updating Note-zsh..."
curl -L "$ARCHIVE_URL" | tar -xz -C "$INSTALL_DIR"

mv "$BIN_DIR/note.txt" "$HOME"

cp "$INSTALL_DIR/note-rust/note-rust" "$BIN_DIR/note"
chmod +x "$BIN_DIR/note"

mv "$HOME/note.txt" "BIN_DIR/note.txt"

echo "Note-zsh updated! Run with 'note tui'"
