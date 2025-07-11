#!/bin/zsh

set -e

cd note-rust
cargo clean
cd ../

echo "📦 Creating Note-zsh.tar.gz for GitHub release..."
tar czf Note-zsh.tar.gz \
  note \
  note-rust \
  update.sh \
  install.sh

echo "✅ Archive created: Note-zsh.tar.gz"
