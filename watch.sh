#!/bin/zsh

cd note-rust || exit 1
cargo build --release || {
  echo "Build failed"
  exit 1
}
cd ..

chmod +rx update.sh install.sh

tar czf Note-zsh.tar.gz note note-rust/target/release/note-rust note.txt update.sh install.sh

echo "Archive Note-zsh.tar.gz created."
