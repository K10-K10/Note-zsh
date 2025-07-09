#!/bin/zsh

INSTALL_DIR="$HOME/.note-zsh"
BIN_DIR="$HOME/.local/bin"
ARCHIVE_URL="https://github.com/K10-K10/Note-zsh/releases/latest/download/Note-zsh.tar.gz"
mkdir -p "$INSTALL_DIR" "$BIN_DIR"
echo "Downloading Note-zsh..."
curl -L "$ARCHIVE_URL" | tar -xz -C "$INSTALL_DIR"
cp "$INSTALL_DIR/note-rust/target/release/note-rust" "$BIN_DIR/note"
chmod +x "$BIN_DIR/note"

ZSHRC="$HOME/.zshrc"
PATH_LINE="export PATH=\"$BIN_DIR:\$PATH\""
[[ ! -f $ZSHRC ]] && touch $ZSHRC

if ! grep -q "$PATH_LINE" "$ZSHRC"; then
  echo "" >>"$ZSHRC"
  echo "# Note-zsh PATH addition" >>"$ZSHRC"
  echo "$PATH_LINE" >>"$ZSHRC"
fi

UPDATE_LINE="[[ \$(find $INSTALL_DIR/.last_update -mtime +6 2>/dev/null) ]] && $INSTALL_DIR/update.sh && touch $INSTALL_DIR/.last_update"
if ! grep -q "update.sh" "$ZSHRC"; then
  echo "$UPDATE_LINE" >>"$ZSHRC"
fi

echo "✅ Installed! Please restart your terminal."
