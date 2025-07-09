#!/bin/zsh

watch_dirs=("note" "note-rust")
tar_name="Note-zsh.tar.gz"

echo "📡 Watching for changes in ${watch_dirs[@]}..."

while true; do
  change=$(inotifywait -r -e modify,create,delete "${watch_dirs[@]}" 2>/dev/null)
  echo "📌 Change detected: $change"
  echo "🔧 Building..."

  cargo build --release --manifest-path=note-rust/Cargo.toml || {
    echo "❌ Build failed."
    continue
  }

  echo "📦 Creating archive..."
  tar czf "$tar_name" note note-rust note.txt README.md images

  echo "✅ Archive updated: $tar_name"
done
